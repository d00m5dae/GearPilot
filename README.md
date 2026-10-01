# GearPilot

**One app for your keyboards, mice, and gaming peripherals.**

GearPilot is an open-source desktop peripheral control project for **Windows, Linux, and macOS**. The long-term goal is to replace the pile of brand-specific utilities with one clean app for device discovery, profiles, remapping, performance settings, lighting, diagnostics, and model-specific controls.

> **v0.5.0 is an early public foundation.** Device discovery, profiles, diagnostics, input testing, backups, and the driver catalog are real. Generic hardware writes are deliberately disabled until individual device protocols are tested and allowlisted.

<p align="center"><img src="assets/gearpilot-icon.svg" width="110" alt="GearPilot icon"></p>

## Why GearPilot exists

A keyboard or mouse should not become harder to configure just because you switched operating systems. GearPilot keeps the profile format and UI brand-independent, while device-specific drivers handle the weird vendor protocols underneath.

## v0.5 highlights

- Rebrand from the Periph prototype to **GearPilot** with a new package identity and source icon.
- Windows, Linux, and macOS only; mobile scaffolding was removed.
- Real backend **driver catalog** with explicit driver IDs and support levels.
- Generic HID inspection plus Apple Magic Keyboard and Glorious Model O driver foundations.
- Physical-device grouping with HID interface inspection.
- Device search/filtering, transport info, VID/PID details, and privacy-safe diagnostics.
- Device-first workspace with **Overview / Performance / Lighting / Diagnostics** tabs.
- Portable software profiles for DPI targets, polling targets, and lighting-scene intent.
- One-click **FPS Gaming**, **Everyday**, and **Low Power** profile presets.
- Local backup export plus migration of older Periph profile/settings data.
- Keyboard and mouse input tester, including right/middle click and wheel events.
- Configurable autoscan, Safe Mode, and blue/green/gold/violet accents.
- Automated GitHub Actions installers for Windows, Linux, and macOS.
- Hardware writes remain disabled until exact device protocols are verified.

## Install

Normal users should **not** install Rust, Cargo, Node, or Tauri.

| Platform | Download | Install |
| --- | --- | --- |
| Windows | `GearPilot_*_x64-setup.exe` | Double-click the setup file |
| Linux Mint / Ubuntu / Debian | `GearPilot_*.deb` | Double-click the package |
| Other Linux desktops | `GearPilot_*.AppImage` | Run the AppImage |
| macOS | `GearPilot_*.dmg` | Open it and drag GearPilot to Applications |

See [INSTALL.md](INSTALL.md) for details.

## Safety model

GearPilot does **not** expose a generic raw-HID write console. A driver must match a tested device family and explicitly advertise each supported operation before the UI can enable it. v0.5 exposes **zero hardware write capabilities**.

That is intentional: unsupported devices can still be identified, tested, profiled, and diagnosed without guessing undocumented packets that could corrupt onboard settings.

## Current driver catalog

| Driver | Level | Reads | Writes |
| --- | --- | --- | --- |
| Generic HID Inspector | Inspect only | identification, interfaces, diagnostics | none |
| Apple Magic Keyboard | Foundation | identification, interfaces, diagnostics | none |
| Glorious Model O | Foundation | identification, interfaces, diagnostics | none |

The next driver milestone is verified **read-only state discovery** for exact tested Magic Keyboard and Model O revisions, followed by one allowlisted setting at a time.

## Developer setup

On Linux Mint / Ubuntu:

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev \
  librsvg2-dev libudev-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo install tauri-cli --version '^2' --locked

./scripts/check.sh
cd src-tauri
cargo tauri dev
```

For frontend-only work, the UI includes a clearly labeled fake demo device:

```bash
cd ui
python3 -m http.server 8765
```

Then open `http://localhost:8765/?demo=1`.

## Repository layout

```text
gearpilot/
├── ui/                       # device-first frontend
├── src-tauri/                # Rust/Tauri desktop backend
├── driver-manifests/         # reference driver metadata
├── installer/linux/          # safe hidraw udev packaging
├── docs/                     # driver/UI/testing/distribution docs
├── scripts/                  # checks + release helper
└── .github/                  # CI, releases, issue/PR templates
```

## Contributing

Device support is intentionally evidence-driven. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and use the **Device support request** issue form for new hardware.

## Project status

GearPilot is pre-1.0 software. Profiles and local app data may evolve, but v0.5 includes migration from the older Periph prototype. See [ROADMAP.md](docs/ROADMAP.md) and [CHANGELOG.md](CHANGELOG.md).

## License

MIT — see [LICENSE](LICENSE).
