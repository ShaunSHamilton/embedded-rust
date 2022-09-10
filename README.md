# Embedded Rust

[Embedded Rust Programming on Raspberry Pi Zero W](https://www.freecodecamp.org/news/embedded-rust-programming-on-raspberry-pi-zero-w/)

## Crates

- [Morse Coder](./morse-coder/)

## The Short of It

### Build Bootable Image

```bash
make p=<PROJECT> # e.g. make p=hello-world
```

### Load Boot on Raspberry Pi

Copy all files made within the `<PROJECT>/boot` directory to the root of an SD card. Then, insert the SD card into the Raspberry Pi and power it on.

## Project Ideas

### Hello World

Blink an LED on and off.

### Morse Coder

Setup an LED and active buzzer to beep out morse code. Accept input from a button, and output the translation in the terminal.

### LED Galore

Setup an array of LEDs. Have them blink in beautiful sequences and colours.

### Self-Flickering LED

Setup one or more LEDs facing a LDR. When the LEDs are on, the resistance will increase causing the LEDs to dim, which lowers the resistance, etc.

### Temperature Sensor

Setup an array of LEDs as a _progress bar_. Use a thermistor to measure the temperature of the Raspberry Pi APU. The higher the temperature, the more LEDs light up.

### Raspberry Pi Strainer

Write a program to put the Raspberry Pi through its paces. Push it as hard as you can.

### LED Range Finder

Set up one permanently lit LED, and an array of LEDs as a _progress bar_. Use a LDR to measure the distance between the LED and the Raspberry Pi. The closer the distance, the more LEDs light up.

## Suggestions to Get Started with Embedded Programming

### Hardware

- [Raspberry Pi Zero W](https://www.raspberrypi.com/products/raspberry-pi-zero-w/)
- [Bojack Electronice Fun Kit](https://www.amazon.co.uk/BOJACK-Electronics-Potentiometer-tie-Points-Breadboard/dp/B09DCB5D9N/ref=sr_1_3?crid=2IYPG3ZOY65JX&keywords=breadboard&qid=1653494101&sprefix=breadboard%2Caps%2C76&sr=8-3) _not a typo_

### Software

- [Raspberry Pi Imager](https://www.raspberrypi.com/products/raspberry-pi-imager/)
- [Rust](https://www.rust-lang.org/)

### Education

- [freeCodeCamp](https://www.freecodecamp.org/)
- [Rust in Replit Interactive Course](https://www.freecodecamp.org/news/rust-in-replit/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Embedded Rust](https://www.rust-lang.org/what/embedded)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
