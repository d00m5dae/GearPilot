# Security policy

GearPilot touches low-level input devices, so unsafe device writes and privilege mistakes are treated as security-sensitive.

## Please report privately when possible

Examples include:
- a way for a community driver to execute arbitrary code;
- unintended privilege escalation;
- unsafe world-writable HID permissions;
- secrets included in diagnostics or exports;
- a hardware-write path that can target an unmatched device.

Do not include private serial numbers, HID paths, tokens, or account credentials in public reports.

Until a private reporting channel is configured, open a minimal GitHub issue asking for a private contact path without publishing exploit details.
