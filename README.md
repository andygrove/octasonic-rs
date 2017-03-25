# octasonic-rs

Rust library for interacting with Octasonic HC-SR04 breakout board over SPI.

This code has been tested on a Raspberry Pi 3 Model B running Raspian Jessie and Rust stable 1.16.0

You must enable SPI on the Raspberry Pi for this library to work! Use the Raspberry Pi Configuration utility to do this.

# Examples

## Blink

The blink example sends SPI commands to the octasonic board to blink the LED. This is the easiest way to verify that the board is working correctly.

```
cargo run --example blink
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
