# Wakey: Universal Linux Zero-Latency Instant-On Daemon

![Project Status](https://img.shields.io/badge/Status-Experimental-lightgrey)
![Platform](https://img.shields.io/badge/Platform-Linux-orange)
![Distro](https://img.shields.io/badge/Distro-Agnostic-blue)
![Language](https://img.shields.io/badge/Language-Rust-brown)

Wakey is a cross-distribution system daemon written in Rust designed to eliminate wake-up latency on Linux laptops. It provides a "Ready-on-Open" experience similar to macOS or mobile devices by maintaining the system in a smart, low-power active state instead of traditional hardware suspension.

---

## Architecture and Compatibility

Wakey is designed to be hardware-agnostic and distribution-independent (Fedora, Ubuntu, Arch, etc.). It features an adaptive hardware detection engine that scales its functionality based on the host system's capabilities:

### 1. Hardware Detection Engine
- **Universal Lid Sensing:** Automatically scans ACPI paths (/proc/acpi/button/lid/*) to find the correct lid switch for any laptop model.
- **Adaptive Backlight Control:** Interfaces with standard kernel interfaces via brightnessctl to manage any display panel.
- **Vendor-Specific Enhancements:** Automatically detects specialized hardware (such as TUXEDO Computers fan controllers via D-Bus) and utilizes vendor-specific protocols if available.

### 2. Smart Resource Management
- **Intelligent Process Throttling:** Instead of freezing processes (SIGSTOP), which causes UI instability in applications like VS Code or Firefox, Wakey manipulates CPU energy preferences (EPP) and process niceness to minimize battery drain without breaking application state.
- **Dynamic Keyboard Management:** Detects various keyboard backlight interfaces (/sys/class/leds/*) and manages power states based on user-defined profiles.



---

## Operating Principle

Wakey operates as a high-priority service that overrides standard systemd-logind behaviors. By setting the system to ignore the lid-switch event, Wakey takes full control of the power state transition:

1. **Inactive State:** Triggered on lid close. The display is disabled, keyboard LEDs are turned off, fans are set to a silent/legacy profile, and the CPU is set to a power-efficiency preference.
2. **Active State:** Triggered on lid open. Hardware is restored to full performance and user-defined brightness levels in under 200ms.

---

## Installation and Setup

### Prerequisites
- Rust Toolchain (Cargo)
- `brightnessctl` (Recommended for display management)
- `systemd` (For service management)

### Universal Installation
The included installation script configures systemd to delegate lid management to Wakey:
```bash
sudo ./scripts/install.sh
```

### Build from Source
```bash
git clone https://github.com/erogluyusuf/Wakey.git
cd Wakey
cargo build --release
```

---

## Configuration

The daemon can be customized for specific hardware requirements in `src/modules/monitor.rs`:

| Parameter | Function |
|---------|------------|
| `throttle_processes` | Manages CPU priority for heavy applications during sleep. |
| `set_cpu_epp` | Adjusts hardware energy performance preferences. |
| `set_kbd_brightness` | Defines the default brightness for keyboard LEDs on wake. |

---

## Disclaimer
Wakey maintains the system in an S0 (Active) state. Battery consumption will be higher than standard S3 (Suspend-to-RAM) modes. It is designed for users who prioritize instant availability and system stability over maximum standby duration.

## License
Distributed under the MIT License. Developed by Yusuf Eroğlu.

---

**Maintained by:** Yusuf Eroğlu  
_Universal speed and stability for the Linux mobile ecosystem._
