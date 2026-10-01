# UI design direction

GearPilot v0.5 uses a device-first workflow rather than a generic admin dashboard.

## Principles

1. **The peripheral is the hero.** The selected device is always obvious and its common actions are one click away.
2. **Profiles are portable.** The app stores intent separately from a manufacturer's protocol.
3. **Progressive disclosure.** VID/PID, usage pages and HID paths live under Diagnostics rather than dominating the normal interface.
4. **Never fake support.** A polished disabled control with “Driver required” is preferable to a control that silently does nothing.
5. **Quiet dark UI.** Neutral charcoal surfaces, one accent, restrained borders, no glass-card overload and no decorative gradients on every panel.
6. **Fast navigation.** Device list, tabbed device editor and command palette minimize deep settings trees.
7. **Recoverable actions.** Local configuration changes support undo/redo and backup/restore.

The layout is inspired by the usability patterns of modern peripheral configuration tools, but the visual system and components are original to GearPilot.
