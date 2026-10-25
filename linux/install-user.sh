#!/usr/bin/env bash
# Builds the mixer and the userspace driver and installs them for this user only (no root):
#   ~/.local/bin/ucx-mixer, ~/.local/bin/ucx-stream
#   ~/.local/share/applications/ucx-mixer.desktop   (UCX Mixer in the application menu)
#   ~/.config/systemd/user/ucx-stream.service       (not enabled: see below)
# Run it again after pulling to update them. The kernel driver is installed separately
# (README.md, Install), and needs root.
set -euo pipefail

repo=$(cd "$(dirname "$0")/.." && pwd)
bin=${XDG_BIN_HOME:-$HOME/.local/bin}
apps=${XDG_DATA_HOME:-$HOME/.local/share}/applications
units=${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user

cargo build --release --manifest-path "$repo/Cargo.toml" -p ucx-mixer-app -p ucx-stream
install -Dm755 "$repo/target/release/ucx-mixer" "$bin/ucx-mixer"
install -Dm755 "$repo/target/release/ucx-stream" "$bin/ucx-stream"

mkdir -p "$apps"
sed "s|@BIN@|$bin|" "$repo/linux/ucx-mixer.desktop" > "$apps/ucx-mixer.desktop"
mkdir -p "$units"
sed "s|@BIN@|$bin|" "$repo/linux/ucx-stream.service" > "$units/ucx-stream.service"
systemctl --user daemon-reload 2>/dev/null || true

echo "Installed ucx-mixer and ucx-stream in $bin, and UCX Mixer in the application menu."
echo "To have the userspace driver start with your session (not with the kernel driver loaded):"
echo "    systemctl --user enable --now ucx-stream"
