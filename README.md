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

### Wiring Method

* [x] Connect DragonTail to the breadboard
* [x] Connect Micro:bit V2 to the DragonTail
* [x] Connect RGB LED to the breadboard
* [x] Connect the LED wires to the pins [per the Wiring section/RGB LED](README.md/#wiring)
* [x] Connect the Potentiometer to the breadboard
* [x] Connect the Potentiometer wires [per the Wiring section/Potentiometer](README.md/#wiring)

-- *What I did and how it went* --  

I used a different potentiometer than the one provided in the class kit that did not have the additional clamps that were in the way. The pins on the potentiometer used had the ground pin and 3.3V pin on one side and the P2 pin on the opposite side. The knob also has an arrow on the top that shows the position of the rotation.

Knowing that the breadboard is connected by rows and the +/- are connected by column made the wiring simple to connect the pins. I had no issues wiring up the unit. [See drawing below][1]

Upon running ```cargo embed --release``` with the original code, everything worked as intended. Turning the knob clockwise change the RGB LED from Green to Cyan.

![Image of labeled hardware setup][1]

### Documentation

* [x] Comment RGB Calibration code
* [ ] Comment added code
* [ ] Use ```cargo doc``` command to generate docs
* [ ] Make sure to use ```cargo clippy```
* [ ] Make sure to use ```cargo fmt --check``` and ```cargo fmt --all```

### Code

* [x] Share frame rate between UI and RGB structs
* [X] Adjust RGB delays according to the frame rate

This took a bit to wrap my head around. I played with Mutex a bit to understand how the rgb and ui modules can integrate values. At first, I could get the ui module to print the values changing, but the LED light was unaffected.  

I realized that I needed to modify the rgb module `run` function to use the `frame_time_tick` function with `get_frame_rate`.

Because the frame rate is the only adjustment when no buttons are pushed, I *temporarily* had the blue light code ignored. I noticed that the light would dim as I turned the knob counter-clockwise.

Upon reintroducing the blue light changes → Both the blue light and the frame rate would increase or decrease synonymously. I mainly found it fun to watch. But the important thing was that the knob adjusted the frame rate from 10...160 and went from obnoxiously blinking to smooth.

* [X] Add Support for Red and Green lines

To start, I did the basic thing -- to add the red and green lines such that the red, green, and blue light all change in unison with the frame rate based on the knob measurement. This mechanism simply changed the LED light from 0ff to a flickery cyan to a constant cyan.

* [ ] Use the buttons in UI

* [ ] Get Measurements of approximate min frame rate & max % on-time

## References

### Doc Tools

[Mintlify Doc Writer][2]  
[Rust Doc Style Guide (Repo)][3]  
[Rust Docs][4]  
[Clippy Docs][5]

---

*NOTE:* For the tools, such as *Mintlify*, they provide a good template -- but still require manual editing and/or verbiage modification.

[1]:imgs/hardware-drawing.jpg
[2]:https://marketplace.visualstudio.com/items?itemName=mintlify.document
[3]:https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text
[4]:https://doc.rust-lang.org/beta/rustdoc/index.html
[5]:https://doc.rust-lang.org/clippy
