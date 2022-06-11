# Embedded Rust

[Embedded Rust Programming on Raspberry Pi Zero W](https://www.freecodecamp.org/news/embedded-rust-programming-on-raspberry-pi-zero-w/)

## Crates

- [Morse Coder](./morse-coder/)

## The Short of It

### One-Time Setup

Install target:

```bash
rustup add target arm-unknown-linux-gnueabihf
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
sudo nano ~/.cargo/config
```

Configure linker:

```conf
[target.arm-unknown-linux-gnueabihf]
linker = "/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

### Cross Compile for Raspberry Pi

```bash
cargo build --release -p <crate_name> --target=arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/release/<crate_name> pi@192.168.1.199:~/<crate_name>
```
