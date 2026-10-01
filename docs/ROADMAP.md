# GearPilot roadmap

## v0.5 — public foundation
- GearPilot rebrand and desktop-only scope.
- Normal installers for Windows, Linux, and macOS.
- Backend driver IDs and driver catalog.
- Generic HID inspection.
- Apple Magic Keyboard and Glorious Model O driver foundations.
- Portable profiles, templates, diagnostics, tester, backup/restore, and legacy migration.
- Hardware writes remain disabled.

## v0.6 — verified device reads
- Formal Rust driver trait/API.
- Exact VID/PID/revision matching where reliable.
- Read-only Magic Keyboard state discovery for tested models.
- Read-only Model O state discovery for tested revisions.
- Structured per-capability driver errors and recovery hints.
- UI gating driven entirely by driver capability metadata.

## v0.7 — first allowlisted writes
- Add settings one operation at a time only after verified reads.
- Snapshot current state before writes where the protocol allows it.
- Apply/revert flow and failure recovery.
- Distinguish onboard-device settings from software-only profiles.

## Later
- Global remapping backends for Windows/macOS/Linux.
- Focus-aware automatic profile switching.
- Unified RGB compositor across supported device drivers.
- Sandboxed community device definitions/plugins.
- Signed Windows builds and signed/notarized macOS builds.
