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
* [x] Connect the LED wires to the pins [per the Wiring section/RGB LED][1]
* [x] Connect the Potentiometer to the breadboard
* [x] Connect the Potentiometer wires [per the Wiring section/Potentiometer][1]

-- *What I did and how it went* --  

I used a different potentiometer than the one provided in the class kit that did not have the additional clamps that were in the way. The pins on the potentiometer used had the ground pin and 3.3V pin on one side and the P2 pin on the opposite side. The knob also has an arrow on the top that shows the position of the rotation.

Knowing that the breadboard is connected by rows and the +/- are connected by column made the wiring simple to connect the pins. I had no issues wiring up the unit. [See drawing below][2]

Upon running ```cargo embed --release``` with the original code, everything worked as intended. Turning the knob clockwise change the RGB LED from Green to Cyan.

![Image of labeled hardware setup][2]

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

## Inspired Experiment

Given that red(&nu;) < green(&nu;) < blue(&nu;)[^1], I had a thought about testing out ratio values to attempt to stabilize all three colors such that an *optimum* [^2] white can be produced.

Looking at the wavelengths in nanometers, I wanted to get the ratio of the wavelengths between the red, green, and blue waves. Then, replicate that ratio based on the values provided in the program.

The RGB values in nm: [^3]  
Blue (0,0,255) &approx; 440nm  
Green (0,255,0) &approx; 510nm  
Red (255,0,0) &approx; 700nm

## References

[Visible Light Wavelengths][3]

### Doc Tools

[Mintlify Doc Writer][4][^4]  
[Rust Doc Style Guide (Repo)][5]  
[Rust Docs][6]  
[Clippy Docs][7]  
[Markdown Guide][8]  
[Markdown Symbols][9]

<!-- Footers -->
[^1]: In regard to visible light frequency and energy.
[^2]: *Optimum* here does not indicate perfection, but is trying to find values that result in testing a *good*, objective blending of red, green, and blue.
[^3]: There is not a precise concensus for the specific values in the visible light spectrum, so the approximation is based on the references used, noting that other references may indicate different values.
[^4]: Like all *tools*, this should not be used as a be-all solution, rather it provides a good template -- but still requires manual editing and/or verbiage modification.

<!--Collection of Links-->

[1]:README.md/#wiring
[2]:imgs/hardware-drawing.jpg
[3]:https://academo.org/demos/wavelength-to-colour-relationship/
[4]:https://marketplace.visualstudio.com/items?itemName=mintlify.document
[5]:https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text
[6]:https://doc.rust-lang.org/beta/rustdoc/index.html
[7]:https://doc.rust-lang.org/clippy
[8]: https://www.markdownguide.org/
[9]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references
