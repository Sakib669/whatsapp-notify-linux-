#!/usr/bin/env bash
set -e

echo "=== Uninstalling WhatsApp Linux Background Daemon ==="

systemctl --user stop whatsapp-daemon.service 2>/dev/null || true
systemctl --user disable whatsapp-daemon.service 2>/dev/null || true
rm -f "$HOME/.config/systemd/user/whatsapp-daemon.service"
systemctl --user daemon-reload 2>/dev/null || true

rm -f "$HOME/.local/bin/whatsapp-daemon"
rm -f "$HOME/.local/bin/whatsapp-url-handler"
rm -f "$HOME/.local/share/applications/whatsapp.desktop"
rm -f "$HOME/.local/share/icons/whatsapp.png"
rm -f "$HOME/.local/share/icons/whatsapp.svg"
rm -f "$HOME/.config/autostart/whatsapp-daemon.desktop"
rm -f "/run/user/$UID/whatsapp-daemon.sock"

echo "WhatsApp Background Daemon completely uninstalled."
