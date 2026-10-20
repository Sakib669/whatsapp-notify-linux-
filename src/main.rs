use std::env;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

unsafe extern "C" {
    fn setsid() -> i32;
}

fn get_socket_path() -> PathBuf {
    if let Ok(runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        PathBuf::from(runtime_dir).join("whatsapp-daemon.sock")
    } else {
        let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
        PathBuf::from(format!("/tmp/whatsapp-{}.sock", user))
    }
}

/// Detect available Chromium-based browser (Brave, Chrome, Chromium, Thorium)
fn get_browser_binary() -> &'static str {
    let candidates = [
        "/usr/bin/brave-browser",
        "/usr/bin/brave-browser-stable",
        "/usr/bin/google-chrome",
        "/usr/bin/google-chrome-stable",
        "/usr/bin/chromium-browser",
        "/usr/bin/chromium",
        "/usr/bin/thorium-browser",
    ];
    for path in candidates {
        if PathBuf::from(path).exists() {
            return path;
        }
    }
    "brave-browser"
}

/// Find the X11 window ID for WhatsApp Web
fn find_whatsapp_window() -> Option<String> {
    // 1. Try search by classname (Chromium sets instance to web.whatsapp.com)
    if let Ok(output) = Command::new("xdotool")
        .args(["search", "--classname", "web.whatsapp.com"])
        .output()
    {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines().rev() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
    }

    // 2. Fallback: search via wmctrl by title/class
    if let Ok(output) = Command::new("wmctrl").args(["-lx"]).output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.to_lowercase().contains("web.whatsapp.com")
                    || line.to_lowercase().contains("whatsapp")
                {
                    if let Some(wid) = line.split_whitespace().next() {
                        return Some(wid.to_string());
                    }
                }
            }
        }
    }

    None
}

/// Check if the window is currently visible (viewable) or unmapped (hidden)
fn is_window_visible(wid: &str) -> bool {
    if let Ok(output) = Command::new("xwininfo").args(["-id", wid]).output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            return text.contains("Map State: IsViewable");
        }
    }
    false
}

/// Show and activate the WhatsApp window
fn show_window(wid: &str) {
    let _ = Command::new("xdotool").args(["windowmap", wid]).status();
    let _ = Command::new("xdotool")
        .args(["windowactivate", wid])
        .status();
}

/// Hide (unmap) the WhatsApp window to background
fn hide_window(wid: &str) {
    let _ = Command::new("xdotool").args(["windowunmap", wid]).status();
}

/// Spawn the WhatsApp browser process
fn spawn_whatsapp() -> Option<Child> {
    let browser_bin = get_browser_binary();

    let mut cmd = Command::new(browser_bin);
    cmd.args([
        "--app=https://web.whatsapp.com",
        "--class=web.whatsapp.com",
        "--enable-background-mode",
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null());

    unsafe {
        cmd.pre_exec(|| {
            setsid();
            Ok(())
        });
    }

    cmd.spawn().ok()
}

/// Send a command to the running daemon
fn send_command(cmd: &str) -> Result<String, String> {
    let sock_path = get_socket_path();
    let mut stream = UnixStream::connect(&sock_path)
        .map_err(|e| format!("Could not connect to daemon socket: {}", e))?;

    stream
        .write_all(cmd.as_bytes())
        .map_err(|e| format!("Failed to send command: {}", e))?;

    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);
    Ok(response)
}

fn spawn_daemon_detached(hidden: bool) {
    let exe = env::current_exe().unwrap_or_else(|_| {
        if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".local/bin/whatsapp-daemon")
        } else {
            PathBuf::from("whatsapp-daemon")
        }
    });

    let mut cmd = Command::new(exe);
    cmd.arg("daemon");
    if hidden {
        cmd.arg("--hidden");
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    unsafe {
        cmd.pre_exec(|| {
            setsid();
            Ok(())
        });
    }

    let _ = cmd.spawn();
}

/// Main daemon runner
fn run_daemon(start_hidden: bool) {
    let sock_path = get_socket_path();
    if sock_path.exists() {
        let _ = fs::remove_file(&sock_path);
    }

    let listener = match UnixListener::bind(&sock_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind UNIX socket {:?}: {}", sock_path, e);
            return;
        }
    };

    println!("WhatsApp Background Daemon started. Socket: {:?}", sock_path);

    // Initial launch if not running
    if find_whatsapp_window().is_none() {
        spawn_whatsapp();
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(10) {
            if let Some(wid) = find_whatsapp_window() {
                if start_hidden {
                    hide_window(&wid);
                } else {
                    show_window(&wid);
                }
                break;
            }
            thread::sleep(Duration::from_millis(250));
        }
    } else if let Some(wid) = find_whatsapp_window() {
        if start_hidden {
            hide_window(&wid);
        } else {
            show_window(&wid);
        }
    }

    let running = Arc::new(AtomicBool::new(true));
    let r_clone = running.clone();

    // Background thread to monitor process & revive silently if user closed via 'X'
    thread::spawn(move || {
        while r_clone.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(2));

            // Check if WhatsApp window exists
            if find_whatsapp_window().is_none() {
                // If it was closed, re-launch it silently in the background
                // so real-time notifications continue uninterrupted
                spawn_whatsapp();

                let start = Instant::now();
                while start.elapsed() < Duration::from_secs(10) {
                    if let Some(wid) = find_whatsapp_window() {
                        // Immediately unmap to keep it silent in background
                        hide_window(&wid);
                        break;
                    }
                    thread::sleep(Duration::from_millis(250));
                }
            }
        }
    });

    // Accept incoming socket commands
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    while running.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((mut socket, _)) => {
                let mut buffer = [0u8; 64];
                if let Ok(bytes_read) = socket.read(&mut buffer) {
                    let cmd = String::from_utf8_lossy(&buffer[..bytes_read])
                        .trim()
                        .to_string();

                    match cmd.as_str() {
                        "SHOW" => {
                            if let Some(wid) = find_whatsapp_window() {
                                show_window(&wid);
                                let _ = socket.write_all(b"OK: SHOWN\n");
                            } else {
                                spawn_whatsapp();
                                let _ = socket.write_all(b"OK: LAUNCHED\n");
                            }
                        }
                        "HIDE" => {
                            if let Some(wid) = find_whatsapp_window() {
                                hide_window(&wid);
                                let _ = socket.write_all(b"OK: HIDDEN\n");
                            } else {
                                let _ = socket.write_all(b"ERR: NOT_FOUND\n");
                            }
                        }
                        "TOGGLE" => {
                            if let Some(wid) = find_whatsapp_window() {
                                if is_window_visible(&wid) {
                                    hide_window(&wid);
                                    let _ = socket.write_all(b"OK: HIDDEN\n");
                                } else {
                                    show_window(&wid);
                                    let _ = socket.write_all(b"OK: SHOWN\n");
                                }
                            } else {
                                spawn_whatsapp();
                                let _ = socket.write_all(b"OK: LAUNCHED\n");
                            }
                        }
                        "STATUS" => {
                            if let Some(wid) = find_whatsapp_window() {
                                if is_window_visible(&wid) {
                                    let _ = socket.write_all(b"STATUS: VISIBLE\n");
                                } else {
                                    let _ = socket.write_all(b"STATUS: HIDDEN_BACKGROUND\n");
                                }
                            } else {
                                let _ = socket.write_all(b"STATUS: STOPPED\n");
                            }
                        }
                        "QUIT" => {
                            running.store(false, Ordering::Relaxed);
                            if let Some(wid) = find_whatsapp_window() {
                                let _ =
                                    Command::new("xdotool").args(["windowclose", &wid]).status();
                            }
                            let _ = socket.write_all(b"OK: QUITTING\n");
                            break;
                        }
                        _ => {
                            let _ = socket.write_all(b"ERR: UNKNOWN_CMD\n");
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(60));
            }
            Err(e) => {
                eprintln!("Socket accept error: {}", e);
                thread::sleep(Duration::from_millis(200));
            }
        }
    }

    if sock_path.exists() {
        let _ = fs::remove_file(&sock_path);
    }
    println!("WhatsApp Background Daemon stopped.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("toggle");

    match command {
        "daemon" => {
            let start_hidden = args.get(2).map(|s| s == "--hidden").unwrap_or(false);
            run_daemon(start_hidden);
        }
        "show" => {
            if let Err(_) = send_command("SHOW") {
                spawn_daemon_detached(false);
            }
        }
        "hide" => {
            if let Err(e) = send_command("HIDE") {
                eprintln!("{}", e);
            }
        }
        "status" => match send_command("STATUS") {
            Ok(resp) => print!("{}", resp),
            Err(e) => println!("Daemon not running: {}", e),
        },
        "quit" => {
            if let Err(e) = send_command("QUIT") {
                eprintln!("{}", e);
            } else {
                println!("WhatsApp daemon and window closed.");
            }
        }
        "toggle" | _ => {
            if let Err(_) = send_command("TOGGLE") {
                spawn_daemon_detached(false);
            }
        }
    }
}
