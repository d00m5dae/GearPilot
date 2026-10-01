# Contributing to GearPilot

Thanks for helping make peripheral software less brand-locked.

## Before opening code

Run:

```bash
./scripts/check.sh
```

For Rust changes, `cargo check` must pass. Keep frontend controls honest: do not label a hardware operation as working unless the matching backend driver capability exists.

## Device support rules

1. Match the **exact device family/revision** as narrowly as practical.
2. Start read-only.
3. Document where each protocol detail came from or how it was observed on hardware you own or are authorized to test.
4. Never assume every product sharing a USB vendor ID uses the same protocol.
5. Hardware writes must be individually allowlisted and have clear error handling.
6. Do not add generic raw-HID packet consoles to normal-user builds.

## Pull requests

Keep changes focused. Include:
- what changed;
- how you tested it;
- affected OS/device models;
- screenshots for substantial UI changes;
- diagnostics with serial/path fields redacted unless truly necessary.
