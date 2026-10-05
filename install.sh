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
  *) die "The current Linux build is x86_64 only. Your machine reports $(uname -m)." ;;
esac

mkdir -p "$APP_DIR" "$BIN_DIR" "$DESKTOP_DIR" "$ICON_DIR"

install_release() {
  local release_json asset_url tmp
  release_json="$(curl -fsSL -H 'Accept: application/vnd.github+json' "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null)" || return 1
  asset_url="$(
    printf '%s' "$release_json" | python3 -c '
import json,sys
data=json.load(sys.stdin)
for asset in data.get("assets", []):
    name=asset.get("name", "")
    if name.endswith(".AppImage"):
        print(asset.get("browser_download_url", ""))
        break
'
  )"
  [[ -n "$asset_url" ]] || return 1
  tmp="$(mktemp)"
  trap 'rm -f "$tmp"' RETURN
  say "Downloading the newest GearPilot release…"
  curl -fL --progress-bar "$asset_url" -o "$tmp"
  install -m 0755 "$tmp" "$APPIMAGE"
}

build_from_source() {
  command -v git >/dev/null 2>&1 || die "git is required when no prebuilt release is available."
  command -v sudo >/dev/null 2>&1 || die "sudo is required to install build dependencies."

  if command -v apt-get >/dev/null 2>&1; then
    say "No prebuilt release found. Installing build dependencies…"
    sudo apt-get update
    sudo apt-get install -y \
      build-essential curl wget file git \
      libwebkit2gtk-4.1-dev libxdo-dev libssl-dev \
      libayatana-appindicator3-dev librsvg2-dev libudev-dev
  else
    die "No prebuilt release exists yet and automatic source builds currently support Debian/Ubuntu/Mint only."
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    say "Installing Rust…"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck disable=SC1091
    source "$HOME/.cargo/env"
  fi

  if ! command -v cargo-tauri >/dev/null 2>&1; then
    say "Installing the Tauri build tool…"
    cargo install tauri-cli --version '^2' --locked
  fi

  local src
  src="$(mktemp -d)"
  trap 'rm -rf "$src"' RETURN
  say "Building GearPilot…"
  git clone --depth 1 "https://github.com/$REPO.git" "$src/GearPilot"
  (
    cd "$src/GearPilot/src-tauri"
    cargo tauri icon ../assets/gearpilot-icon.svg
    cargo tauri build --bundles appimage
  )

  local built
  built="$(find "$src/GearPilot/src-tauri/target/release/bundle/appimage" -maxdepth 1 -type f -name '*.AppImage' | head -n 1)"
  [[ -n "$built" ]] || die "The source build finished without producing an AppImage."
  install -m 0755 "$built" "$APPIMAGE"
}

say "Installing GearPilot…"
install_release || build_from_source

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
    ;;
esac

say "Installed."
printf '\nOpen GearPilot from your app menu, or run:\n\n  gearpilot\n\n'
