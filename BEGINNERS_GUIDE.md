# 📖 Beginner's Guide: WhatsApp Background Daemon for Linux

Welcome! This guide is written step-by-step for anyone new to Linux who wants WhatsApp to run smoothly in the background and receive real-time notifications just like on Windows.

---

## 🌟 What Does This Tool Do?

Normally on Linux, when you close your WhatsApp window by clicking the **`X`** button, the whole app quits. When it quits, you stop getting message notifications.

This tool solves that:
1. **Keeps WhatsApp alive silently in the background:** When you click **`X`**, it doesn't die. It stays hidden in the background.
2. **Real-time notifications:** You get notification popups with sound and message previews on your desktop even when the window is closed.
3. **Zero lag:** Clicking the green WhatsApp icon brings the window up instantly without waiting for a browser to boot.
4. **Uses almost no memory:** The background daemon uses less than **0.3 MB of RAM** and **0% CPU**.

---

## 🛠️ Step 1: Install Prerequisites (One-Time Setup)

Open your terminal (<kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>T</kbd>) and paste the command for your Linux distribution:

### For Linux Mint / Ubuntu / Debian / Pop!_OS:
```bash
sudo apt update
sudo apt install -y cargo xdotool wmctrl x11-utils
```

### For Fedora:
```bash
sudo dnf install -y cargo xdotool wmctrl xwininfo
```

### For Arch Linux / EndeavourOS / Manjaro:
```bash
sudo pacman -S --noconfirm rust xdotool wmctrl xorg-xwininfo
```

> **Browser Requirement:** Make sure you have **Brave**, **Google Chrome**, **Chromium**, or **Thorium** installed. (Brave is recommended for lowest RAM usage).

---

## 🚀 Step 2: Install the Daemon (One Command)

Run these three commands in your terminal:

```bash
git clone https://github.com/Sakib669/whatsapp-notify-linux-.git
cd whatsapp-notify-linux-
./install.sh
```

**What the installer does automatically:**
- Compiles the fast Rust binary.
- Adds the WhatsApp icon to your Start Menu and Desktop applications.
- Configures the background service (`systemd`) so it starts automatically every time your computer turns on.

---

## 📱 Step 3: First-Time Setup (Link Your Phone)

1. Open your Start Menu and click **WhatsApp** (or type `whatsapp-daemon show` in your terminal).
2. The WhatsApp window will appear.
3. Open WhatsApp on your phone ➔ Tap **Settings** (or 3 dots) ➔ **Linked Devices** ➔ **Link a Device**.
4. Scan the QR code shown on your laptop screen.
5. In WhatsApp Web, if a small prompt asks **"Allow notifications?"**, click **Allow**.

You're all set!

---

## 💡 Step 4: Daily Usage

### How to Show / Hide WhatsApp
- **Using the Taskbar:** Click the green **WhatsApp** icon on your taskbar anytime to show or hide the window.
- **Closing with `X`:** Feel free to click the **`X`** button whenever you want to clear your screen. The app will automatically stay alive silently in the background so you never miss a message!

### When a New Message Arrives
- A native desktop notification banner will pop up in the corner of your screen showing who messaged you and a preview of the text.
- Clicking the notification (or the taskbar icon) brings WhatsApp directly to the front.

---

## ⌨️ Bonus: Set a Keyboard Shortcut (<kbd>Super</kbd> + <kbd>W</kbd>)

You can set a keyboard shortcut so pressing <kbd>Windows</kbd> (Super) + <kbd>W</kbd> pops WhatsApp open from anywhere.

### In Linux Mint (Cinnamon):
1. Open the Start Menu and search for **Keyboard**.
2. Click the **Shortcuts** tab at the top.
3. In the left column, click **Custom Shortcuts**.
4. Click **Add custom shortcut** at the bottom:
   - **Name:** `Toggle WhatsApp`
   - **Command:** `whatsapp-daemon toggle`
5. Click the unassigned key field, press <kbd>Super</kbd> + <kbd>W</kbd>. Done!

### In Ubuntu (GNOME):
1. Open **Settings** ➔ **Keyboard** ➔ **View and Customize Shortcuts**.
2. Scroll to the bottom and click **Custom Shortcuts** ➔ **+**.
3. Set **Name:** `Toggle WhatsApp`, **Command:** `whatsapp-daemon toggle`, **Shortcut:** <kbd>Super</kbd> + <kbd>W</kbd>.

---

## 🔧 Useful Terminal Commands

You don't need the terminal to use WhatsApp, but these shortcuts are available if you want them:

```bash
whatsapp-daemon toggle  # Opens WhatsApp if hidden, or hides it if open
whatsapp-daemon show    # Forces the WhatsApp window to show
whatsapp-daemon hide    # Hides the WhatsApp window to the background
whatsapp-daemon status  # Tells you if WhatsApp is currently VISIBLE or HIDDEN
whatsapp-daemon quit    # Completely closes WhatsApp and stops the background service
```

---

## ❓ Frequently Asked Questions (FAQ)

#### Q: Does it use a lot of battery or RAM?
**No.** The Rust daemon uses around `250 KB` of RAM (less than a fraction of 1 MB) and `0% CPU`. When the window is hidden, your browser automatically pauses all animations and visual rendering.

#### Q: How do I update to the latest version?
```bash
cd whatsapp-notify-linux-
git pull
./install.sh
```

#### Q: How do I uninstall it?
```bash
cd whatsapp-notify-linux-
./uninstall.sh
```
Everything will be cleanly removed with zero leftover files.
