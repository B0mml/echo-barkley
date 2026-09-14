# Echo Barkley

ESP32 bark detector in Rust. It reads audio from a PDM microphone, computes sound levels (RMS), and sends an alert when it detects a bark.

## Wiring

## First-time setup

You'll need `rustup` and the Espressif toolchain:

```bash
# 1. Install ESP tools 
cargo +stable install espup ldproxy espflash

# 2. Install the Xtensa toolchain
espup install

# 3. Source environment or add to path
source ~/export-esp.sh
```

> **Note for Linux:** Make sure your user has serial port access (e.g. `sudo usermod -a -G uucp $USER` on Arch, or `dialout` on Ubuntu).

## Running

Plug in the board and run:

```bash
cargo run
```

This compiles the code, flashes the ESP32, and opens the serial monitor. 

Sensitivity can be tweaked in `src/main.rs` via `BARK_THRESHOLD`.
