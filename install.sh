#!/usr/bin/env bash
set -euo pipefail

REPO="d00m5dae/GearPilot"
APP_DIR="$HOME/.local/share/gearpilot"
BIN_DIR="$HOME/.local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"
APPIMAGE="$APP_DIR/GearPilot.AppImage"
LAUNCHER="$BIN_DIR/gearpilot"
DESKTOP_FILE="$DESKTOP_DIR/gearpilot.desktop"
ICON_FILE="$ICON_DIR/gearpilot.svg"

say() { printf '\033[1;36mGearPilot\033[0m %s\n' "$*"; }
die() { printf '\033[1;31mGearPilot install failed:\033[0m %s\n' "$*" >&2; exit 1; }

command -v curl >/dev/null 2>&1 || die "curl is required."
command -v python3 >/dev/null 2>&1 || die "python3 is required."

case "$(uname -m)" in
  x86_64|amd64) ;;
  *) die "The current Linux release is x86_64 only. Your machine reports $(uname -m)." ;;
esac

say "Finding the newest Linux release…"
release_json="$(curl -fsSL -H 'Accept: application/vnd.github+json' "https://api.github.com/repos/$REPO/releases/latest")" ||   die "Could not read the latest GitHub release."

asset_url="$(
  printf '%s' "$release_json" | python3 -c '
import json,sys
data=json.load(sys.stdin)
for asset in data.get("assets", []):
    url=asset.get("browser_download_url", "")
    name=asset.get("name", "")
    if name.endswith(".AppImage"):
        print(url)
        break
'
)"

[[ -n "$asset_url" ]] || die "The latest release does not contain a Linux AppImage yet."

mkdir -p "$APP_DIR" "$BIN_DIR" "$DESKTOP_DIR" "$ICON_DIR"
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

say "Downloading GearPilot…"
curl -fL --progress-bar "$asset_url" -o "$tmp"
install -m 0755 "$tmp" "$APPIMAGE"

say "Installing launcher…"
cat > "$LAUNCHER" <<EOF
#!/usr/bin/env bash
exec "$APPIMAGE" "\$@"
EOF
chmod 0755 "$LAUNCHER"

if curl -fsSL "https://raw.githubusercontent.com/$REPO/main/assets/gearpilot-icon.svg" -o "$ICON_FILE"; then
  icon_line="Icon=$ICON_FILE"
else
  icon_line="Icon=applications-system"
fi

cat > "$DESKTOP_FILE" <<EOF
[Desktop Entry]
Type=Application
Version=1.0
Name=GearPilot
Comment=Control panel for keyboards, mice, and gaming peripherals
Exec=$LAUNCHER
$icon_line
Terminal=false
Categories=Utility;Settings;HardwareSettings;
Keywords=keyboard;mouse;peripheral;gaming;hid;
StartupNotify=true
StartupWMClass=GearPilot
EOF
chmod 0644 "$DESKTOP_FILE"

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
fi

case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *)
    profile="$HOME/.profile"
    line='export PATH="$HOME/.local/bin:$PATH"'
    if [[ ! -f "$profile" ]] || ! grep -Fqx "$line" "$profile"; then
      printf '\n%s\n' "$line" >> "$profile"
    fi
    export PATH="$BIN_DIR:$PATH"
    ;;
esac

say "Installed."
printf '\nOpen it from your app menu as “GearPilot”, or run:\n\n  gearpilot\n\n'
