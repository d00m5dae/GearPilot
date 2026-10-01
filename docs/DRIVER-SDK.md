# Driver SDK design notes

The runtime driver API is not implemented yet. This document defines the direction so device support does not become a collection of one-off hacks.

## Driver responsibilities

A driver should declare:

- Vendor and product IDs it recognizes.
- Optional release/interface/usage constraints.
- Read capabilities.
- Write capabilities.
- Whether a setting is volatile or stored in onboard memory.
- Validation limits for every write.
- A safe probe operation that does not change device state.

## Capability examples

```text
read.battery
read.dpi
read.polling_rate
read.rgb
write.dpi
write.polling_rate
write.rgb
write.button_map
write.keyboard_function_row
```

## Rules

- Unknown vendor reports are never sent from the normal UI.
- Read support should land before write support when possible.
- A write must validate device identity again immediately before sending.
- Values are clamped/validated by the driver, not only by the UI.
- A failed write should leave the profile intact and report that hardware state is unknown.
- Community drivers eventually need a permission model; a manifest must not equal arbitrary native code execution.
