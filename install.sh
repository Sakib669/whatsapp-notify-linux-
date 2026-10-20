#!/usr/bin/env bash
set -e

echo "=== WhatsApp Linux Background Daemon Installer ==="

# Check requirements
MISSING_DEPS=()
for dep in cargo xdotool wmctrl xwininfo; do
    if ! command -v "$dep" >/dev/null 2>&1; then
        MISSING_DEPS+=("$dep")
    fi
done

if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
    echo "Error: Missing required utilities: ${MISSING_DEPS[*]}"
    echo "Please install them first (e.g. sudo apt install xdotool wmctrl x11-utils cargo)"
    exit 1
fi

echo "1. Compiling release binary in Rust..."
cargo build --release
strip target/release/whatsapp-daemon

echo "2. Installing binary to ~/.local/bin/..."
mkdir -p "$HOME/.local/bin"
cp target/release/whatsapp-daemon "$HOME/.local/bin/"
chmod +x "$HOME/.local/bin/whatsapp-daemon"

cp scripts/whatsapp-url-handler "$HOME/.local/bin/"
chmod +x "$HOME/.local/bin/whatsapp-url-handler"

echo "3. Installing desktop icon & launcher..."
mkdir -p "$HOME/.local/share/icons"
cp assets/whatsapp.png "$HOME/.local/share/icons/"
cp assets/whatsapp.svg "$HOME/.local/share/icons/"

mkdir -p "$HOME/.local/share/applications"
cp desktop/whatsapp.desktop "$HOME/.local/share/applications/"
sed -i "s|Icon=whatsapp|Icon=$HOME/.local/share/icons/whatsapp.png|g" "$HOME/.local/share/applications/whatsapp.desktop"
sed -i "s|Exec=whatsapp-url-handler|Exec=$HOME/.local/bin/whatsapp-url-handler|g" "$HOME/.local/share/applications/whatsapp.desktop"
update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true

echo "4. Setting up systemd background service..."
mkdir -p "$HOME/.config/systemd/user"
cp systemd/whatsapp-daemon.service "$HOME/.config/systemd/user/"

systemctl --user daemon-reload
systemctl --user enable --now whatsapp-daemon.service

echo ""
echo "=========================================================="
echo " SUCCESS: WhatsApp Background Daemon Installed!"
echo " - WhatsApp will now run silently in the background."
echo " - Real-time notifications are enabled even when closed."
echo " - Click the WhatsApp app icon anytime to show/hide."
echo "=========================================================="
