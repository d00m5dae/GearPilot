# Testing checklist

## Static checks

Run:

```bash
./scripts/check.sh
```

This checks JavaScript syntax, project JSON, DOM references, version consistency, and Rust when Cargo is available.

## Manual smoke test

1. Start browser demo mode with `?demo=1`.
2. Navigate Devices, Profiles, Input Tester, Drivers, and Settings.
3. Switch Overview / Performance / Lighting / Diagnostics.
4. Activate all three profile presets.
5. Change DPI and polling targets and verify they persist locally.
6. Change lighting effect/color and verify the preview/profile updates.
7. Export a local backup.
8. Test keyboard keys, left/right/middle mouse buttons, and wheel events.
9. Change autoscan and accent preferences.
10. Resize below 1000 px and 760 px.
11. Run the Tauri build and scan real hardware.
12. Verify diagnostics do not expose HID paths or serial numbers.

## Hardware regression checklist

- One physical keyboard with several HID interfaces is grouped as one physical device when the platform exposes a reliable parent.
- Identical serial-less devices are not intentionally merged by guesswork.
- Disconnecting the selected device recovers on the next scan.
- Unknown devices remain inspect-only.
- No normal UI action sends vendor reports in v0.5.
