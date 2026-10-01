# Installing GearPilot

GearPilot is intended to install like any normal gaming utility. **End users do not need Rust, Cargo, Node, or Tauri.**

## Windows
1. Open the latest GearPilot GitHub release.
2. Download the `setup.exe` file.
3. Double-click it and finish setup.
4. Launch **GearPilot** from Start.

The prerelease CI builds are unsigned, so Windows may show a SmartScreen warning until code signing is configured.

## Linux Mint / Ubuntu / Debian
1. Download the `.deb` from the latest release.
2. Double-click it in your file manager.
3. Choose **Install**.
4. Launch GearPilot from the applications menu.

The package installs GearPilot's hidraw access rule using `TAG+="uaccess"` and reloads udev. It does not make HID devices world-writable.

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
