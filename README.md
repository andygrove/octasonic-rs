# octasonic-rs

Rust library for interacting with the <a href="https://www.tindie.com/products/andygrove73/octasonic-8-x-hc-sr04-ultrasonic-breakout-board/">Octasonic 8 x HC-SR04 breakout board</a> over SPI.

This code has been tested on a Raspberry Pi 3 Model B running Raspian Jessie and Rust stable 1.16.0

You must enable SPI on the Raspberry Pi for this library to work! Use the Raspberry Pi Configuration utility to do this.

# Examples

## Blink

The blink example sends SPI commands to the octasonic board to blink the LED. This is the easiest way to verify that the board is working correctly.

```
cargo run --example blink
```

## Demo

The `demo` example shows readings from each sensor and shows how to set various parameters on the board.

```
cargo run --demo
```

## Piano

The piano example generates MIDI instructions based on sensor readings and writes them to stdout.

This output can be piped into fluidsynth to generate music.

Install fluidsynth:

```
sudo apt-get install fluidsynth
```

Compile the example using the release configuration:

```
cargo build --release --example piano
```

Run the example

```
./target/release/examples/piano | fluidsynth -a alsa -s -l /usr/share/sounds/sf2/FluidR3_GM.sf2
```
