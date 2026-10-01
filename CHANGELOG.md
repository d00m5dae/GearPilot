# Changelog

## 0.5.0

### Brand and platform scope
- Renamed the project from Periph to **GearPilot**.
- Replaced the old P icon with a GearPilot G icon across PNG, ICO, and ICNS bundles.
- Removed mobile/Android scaffolding; supported targets are Windows, Linux, and macOS.
- Renamed package identifiers, installer resources, udev rule, backups, profile exports, and release artifacts.
- Added migration from legacy Periph localStorage/profile data.

### Driver core
- Added backend driver IDs instead of deriving support entirely in the frontend.
- Added a runtime driver catalog command.
- Added Generic HID Inspector, Apple Magic Keyboard foundation, and Glorious Model O foundation entries.
- Added driver ID to HID scan results and diagnostic exports.
- Kept all hardware write capabilities disabled until exact device operations are verified.

### User experience
- Added one-click FPS Gaming, Everyday, and Low Power profile templates.
- Added Safe Mode preference for future write-capable drivers.
- Added runtime version/platform information in Settings.
- Expanded Driver Center with built-in driver catalog cards.
- Fixed mouse input tester right-click highlighting and added a middle-button indicator.
- Added local backup export and migration from older Periph profile/settings data.

### Repository and distribution
- Added MIT license, contribution guide, security policy, issue templates, and pull-request checklist.
- Added normal Windows NSIS/MSI, Linux DEB/AppImage, and universal macOS DMG release workflow.
- Debian packages install a `TAG+="uaccess"` hidraw rule rather than world-writable permissions.
- Added CI source validation and release-package checks.

## 0.3.0
- Introduced the device-first workspace, portable profiles, input tester, diagnostics, backup/restore, command palette, and safe read-only HID discovery.
