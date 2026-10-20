# WhatsApp Linux Daemon 🚀

> **A native, ultra-lightweight Rust daemon that enables real-time background WhatsApp notifications on Linux—just like the native Windows app.**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20(X11)-green.svg)]()
[![RAM Usage](https://img.shields.io/badge/Daemon%20RAM-~250%20KB-brightgreen.svg)]()

> 💡 **New to Linux?** Check out our friendly [Beginner's Step-by-Step Guide](BEGINNERS_GUIDE.md) for screenshots, keyboard shortcut setup, and simple instructions!

---

## The Problem

On Linux, WhatsApp runs as a web app (`--app=https://web.whatsapp.com`). Because WhatsApp uses **End-to-End Encryption (Signal Protocol)**, decryption keys live only in active client memory. 

When you close the WhatsApp window on Linux, the browser process is killed—terminating the WebSocket connection and killing all notifications.

On Windows, the dedicated app doesn't actually quit when clicking `X`; it minimizes to a background state.

## The Solution

**`whatsapp-daemon`** is a native, ultra-fast background daemon written in pure standard-library Rust that bridges this gap:

- 🔔 **Real-Time Background Notifications:** Keeps WhatsApp connected silently in the background (unmapped). Notifications arrive with sound & preview banners even when the window is closed.
- ⚡ **Auto-Revive on Close:** If you close WhatsApp by clicking the `X` button, the daemon detects it and silently revives it in background mode within 2 seconds.
- 🪶 **Ultra-Low Resource Footprint:**
  - **Daemon RAM:** **~250 KB** *(0.25 MB!)*
  - **Daemon CPU:** **0.0%**
  - **Background WhatsApp RAM:** ~40–60 MB *(Chromium suspends layout/render pipelines for unmapped windows)*.
- 🎯 **Single-Instance Toggle:** Clicking your desktop or panel icon instantly toggles the window (show / hide) in `0.01s`.
- 🌐 **Browser Agnostic:** Automatically works with Brave, Google Chrome, Chromium, or Thorium.

---

## Installation

### Prerequisites

Ensure you have Rust and standard X11 utilities installed:

```bash
# Ubuntu / Debian / Linux Mint / Pop!_OS
sudo apt update && sudo apt install -y cargo xdotool wmctrl x11-utils

# Fedora
sudo dnf install -y cargo xdotool wmctrl xwininfo

# Arch / EndeavourOS
sudo pacman -S --noconfirm rust xdotool wmctrl xorg-xwininfo
```

### Quick Install

```bash
git clone https://github.com/Sakib669/whatsapp-notify-linux-.git
cd whatsapp-notify-linux-
./install.sh
```

The installer will:
1. Compile the optimized release binary (`cargo build --release`).
2. Install the binary to `~/.local/bin/whatsapp-daemon`.
3. Set up desktop icons and `whatsapp://` URL handler.
4. Enable and start the background user service (`systemd --user`).

---

## Usage

### Using the App
- Simply launch **WhatsApp** from your Application Menu or Taskbar.
- Click the app icon anytime to **Show** or **Hide** the window.
- When you click the **`X`** button to close WhatsApp, it will continue running silently in the background and send real-time notifications.

### CLI Commands

```bash
whatsapp-daemon toggle  # Toggles the WhatsApp window (Show / Hide)
whatsapp-daemon show    # Restores and focuses WhatsApp
whatsapp-daemon hide    # Minimizes to background
whatsapp-daemon status  # Shows if WhatsApp is VISIBLE, HIDDEN, or STOPPED
whatsapp-daemon quit    # Completely stops WhatsApp and the background daemon
```

---

## Optional: Keyboard Shortcut

You can bind a global hotkey (like <kbd>Super</kbd> + <kbd>W</kbd>) to toggle WhatsApp from anywhere:

1. Open your desktop's **Keyboard Shortcuts** settings.
2. Add a new custom shortcut:
   - **Name:** Toggle WhatsApp
   - **Command:** `whatsapp-daemon toggle`
   - **Shortcut:** <kbd>Super</kbd> + <kbd>W</kbd>

---

## Uninstall

```bash
cd whatsapp-notify-linux-
./uninstall.sh
```

---

## Documentation

- [Beginner's Step-by-Step Guide](BEGINNERS_GUIDE.md)

---

## License

MIT License. Free and open source.
