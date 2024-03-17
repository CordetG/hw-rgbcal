# rgbcal: RGB LED calibration tool

Additional Contribution by  
Cordet Gula  
Embedded Rust  
Winter 2024  

Original Contribution by  
Bart Massey 2024

## About  

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

![image of microbitv2 hardware setup](imgs/hardware-setup.jpg)

![image of labeled hardware setup](imgs/hardware-drawing.jpg)

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
just always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10…160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scan out code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

## Process

* [ ] Share frame rate between UI and RGB structs
* [ ] Adjust RGB delays according to the frame rate
* [ ] Add Support for Red and Green lines
* [ ] Use the buttons in UI
* [ ] Get Measurements of approximate min frame rate & max % on-time

## References

### Doc Tools

[Mintlify Doc Writer](https://marketplace.visualstudio.com/items?itemName=mintlify.document)  
[Clippy Docs](https://doc.rust-lang.org/clippy)
