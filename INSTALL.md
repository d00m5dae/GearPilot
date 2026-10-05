# Installing GearPilot

GearPilot is intended to install like any normal gaming utility. **End users do not need Rust, Cargo, Node, or Tauri.**

## Windows
1. Open the latest GearPilot GitHub release.
2. Download the `setup.exe` file.
3. Double-click it and finish setup.
4. Launch **GearPilot** from Start.

The prerelease CI builds are unsigned, so Windows may show a SmartScreen warning until code signing is configured.

## Linux Mint / Ubuntu / Debian

### Easiest install

Run:

```bash
curl -fsSL https://raw.githubusercontent.com/d00m5dae/GearPilot/main/install.sh | bash
```

The installer downloads the newest AppImage into your user account, creates `~/.local/bin/gearpilot`, and adds a desktop launcher under `~/.local/share/applications`. Afterward you can either search for **GearPilot** in your app menu or run:

```bash
gearpilot
```

If your current shell did not already include `~/.local/bin` in `PATH`, open a new terminal once after installation.

### Debian package

You can still download the `.deb` from the latest release and install it normally. The Debian package additionally installs GearPilot's hidraw access rule using `TAG+="uaccess"` and reloads udev. It does not make HID devices world-writable.

## Other Linux desktops
Use the `.AppImage` build. Some distributions may require marking the file executable before launch.

## macOS
1. Download the `.dmg`.
2. Open it.
3. Drag GearPilot into **Applications**.
4. Open GearPilot normally.

The CI DMG is universal for Intel and Apple Silicon. Prerelease builds are unsigned until Apple signing/notarization is configured.

## Maintainer release

After committing a clean release:

```bash
./scripts/publish-release.sh 0.5.0
```

The script validates the source, creates `v0.5.0`, and pushes the tag. GitHub Actions builds the installers on their native operating systems and publishes the release.
