# GearPilot distribution

The source repository is not the product users should install. Releases are.

## End-user experience

| Platform | Normal download | User action |
|---|---|---|
| Windows | `GearPilot_*_x64-setup.exe` | Double-click setup |
| Linux Mint / Ubuntu / Debian | `GearPilot_*.deb` | Double-click package |
| Other Linux | `GearPilot_*.AppImage` | Run AppImage |
| macOS Intel + Apple Silicon | `GearPilot_*.dmg` | Drag GearPilot to Applications |

Rust, Cargo, Tauri CLI, Node, compiler packages, and WebKit development headers are build dependencies. They are **not end-user prerequisites**.

## CI release pipeline

`.github/workflows/release.yml` runs on a `v*` tag. It builds on each target OS rather than cross-compiling installers from one machine.

The Linux build uses Ubuntu 22.04 as a compatibility baseline and packages the `hidraw` udev rule into the Debian package. Windows outputs NSIS and MSI installers. macOS outputs a universal DMG that contains both Intel and Apple Silicon code.

The workflow publishes only after all platform builds succeed.

## Why NSIS is the default Windows recommendation

The NSIS installer uses `currentUser` mode, so a normal install does not require Administrator privileges. MSI remains available for users or organizations that specifically want it.

## Signing status

Development/prerelease installers are built unsigned. Before a broad public release, configure Windows code signing and Apple Developer ID signing/notarization. Never commit signing certificates or private keys to Git.
