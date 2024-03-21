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

Epilepsy Warning [^!] for flashing light.

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
* [X] Comment added code
* [X] Use ```cargo doc``` command to generate docs

```bash
$ cargo doc
Documenting <package>
...
Finished dev [unoptimized + debuginfo] target(s) in 25.82s
Generated <path>/target/thumbv7em-none-eabihf/doc/mb2_embassy_rgb/index.html

$ cargo doc --open
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
    Opening <path>/target/thumbv7em-none-eabihf/doc/mb2_embassy_rgb/index.html
```




* [X] Make sure to use ```cargo clippy```

```bash
$ cargo clippy
    Checking mb2-embassy-rgb v0.1.0 (<path>)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
```

No errors reported.

* [X] Make sure to use ```cargo fmt --check``` and ```cargo fmt --all```

```bash
$ cargo fmt --all
<user>@<domain>:<path>
$ 
```

Formatted all files without issue.

### Code

* [x] Share frame rate between UI and RGB structs
* [X] Adjust RGB delays according to the frame rate

This took a bit to wrap my head around. I played with Mutex a bit to understand how the rgb and ui modules can integrate values. At first, I could get the ui module to print the values changing, but the LED light was unaffected.  

I realized that I needed to modify the rgb module `run` function to use the `frame_time_tick` function with `get_frame_rate`.

Because the frame rate is the only adjustment when no buttons are pushed, I *temporarily* had the blue light code ignored. I noticed that the light would dim as I turned the knob counter-clockwise.

Upon reintroducing the blue light changes → Both the blue light and the frame rate would increase or decrease synonymously. I mainly found it fun to watch. But the important thing was that the knob adjusted the frame rate from 10...160 and went from obnoxiously blinking to smooth.

* [X] Add Support for Red and Green lines

To start, I did the basic thing -- to add the red and green lines such that the red, green, and blue light all change in unison with the frame rate based on the knob measurement. This mechanism simply changed the LED light from 0ff to a flickery cyan to a constant cyan.

* [X] Use the buttons in UI

The main difficulty I had with the buttons was holding down both A + B and trying to change the knob. Sometimes the buttons would not read as both A and B values. I would have to go back and change the value that changed mistakenly. Additionally, one of the buttons -- I think it was *B* was not as sensitive as *A* for whatever reason so the button would not always read as *down*.

* [X] Get Measurements of approximate min frame rate & max % on-time

The values I found to produce a decent *white*:  
[R: 15, G: 10, B: 9]  
min frame-rate &approx; 50 to 60 frames/s  

For more in-depth info on the process, [see the section Mini-Experiment below][14].

## Mini-Experiment

Given that red(&nu;) < green(&nu;) < blue(&nu;)[^1], I had a thought about testing out ratio values to attempt to stabilize all three colors such that an *optimum* [^2] white can be produced in a more controlled fashion.

Looking at the wavelengths in nanometers, I wanted to get a ratio of the wavelengths between the red, green, and blue waves. Then, replicate that ratio based on the values provided in the program.

The RGB values in nm: [^3]  
Blue (0,0,255) &approx; 440 nm  
Green (0,255,0) &approx; 510 nm  
Red (237,0,0) &approx; 710 nm [^4]

### The Rabbit Hole

I found the ratio to be too broad and thus, added additional questions:  
Should 710 nm = 16 steps = 100%?  
Should both (blue, green) be measured from red? OR  
Should it be measured in regard to red &larr; green &larr; blue?  
Would frequency rather than wavelength make a difference in the ratio values?

Doing this a couple different ways, I got different variations of *white* including a *pinkish*, *greenish*, and *bluish* hue. Alas, I was not satisfied.

So, I had a new idea. Seeing as light and color is often associated with temperature, I looked into [Planck's Law][4]. I saw this as a more suitable method because just looking at the wavelength alone did not include an approximate wavelength for white. However, there is a *temperature* in Kelvins that is attributed to white and provides a calculated wavelength at the peak.  

Don't be so impressed... I used an online calculator. Really, the only values I cared about was the wavelength and temperature. Aside from changing the units from Watts to photons pre second, all other values I left as default.

![Calculator Input][13]

### Values

* Temperature of [*white* &approx; 6545 K][13]  
* Lower boundary wavelength *(blue)* set to 0.44 &mu;m
* Upper boundary wavelength *(red)* set to 0.71 &mu;m  

### Method and Result

Wavelength of the peak = 0.560684 &mu;m  

I converted the wavelengths back to nm[^6] and calculated the percentage of red, green, and blue wavelengths relative to white.

Wavelength of the peak as *white* &approx; 561 nm  
Wavelength of *red* &approx; 710 nm  
Wavelength of *green* &approx; 510 nm  
Wavelength of *blue* &approx; 440 nm  

*red* relative to *white* &approx; 125%  
*green* relative to *white* &approx; 91%  
*blue* relative to *white* &approx; 79%  

Given that *15* was the max value for each of the colors, *white* would occur at approximately *12*.

Red at level *15*  
⇒ 15 / 1.25 &approx; 12 *(white)* &rarr; just for reference  
⇒ 12 * 0.91 &approx; 10 *(green)*  
⇒ 12 * 0.78 &approx; 9 *(blue)*

So, the approximate levels I used to generate *white* are [15, 10, 9].

*Was this scientifically sound?*  
Probably not, but I wanted to try it anyway.

## Subjectivity

There are a couple factors I noted that could indicate differences in values; particularly with the frame_rate.  

1. [Photophobia][15][^7]  
   As someone with a genetic predisposition for migraines and experience migraines and extreme sensitivity to light being the largest culprit, there is a chance that I can pick up slight differences that others may not notice.  
   I tried 30 frames per second, but I could tell it was still flickering. At 40 fps, the flickering improved, but it still bothered me. 50 was okay enough to justify a min frame_rate, but I preferred 60 fps.  

   I also noticed that, due to the sensitivity, I can see the red, green, and blue in the light, and it especially left an *imprint* of the colors after looking away. Though, I could no longer sense the blinking, I was able to pick up the colors separately because they are positioned in separate locations in the light itself which is why I could see a red, green, and blue stripe.  

   This was only difficult in the sense that I couldn't properly tell how close it was to *white*, so I attempted to use a shade. Covering it with my hand helped a bit and I could see the reflected light and figured it was close enough.

2. Camera  
   The visual that may appear on the camera can also have an effect on what the light looks like in real time, especially if using a cellphone that has software editing integrated in the camera app.

## References

[Visible Light Wavelengths][3]  
[Planck's Law of Black-body Radiation][4]  
[Planck's Law Calculator][11]  
[Color-Temperature Relationship][12]

### Doc Tools

[Mintlify Doc Writer][5][^5]  
[Rust Doc Style Guide (Repo)][6]  
[Rust Docs][7]  
[Clippy Docs][8]  
[Markdown Guide][9]  
[Markdown Symbols][10]

<!-- Footers -->
[^1]: In regard to visible light frequency and energy.
[^2]: *Optimum* here does not indicate perfection, but is trying to find values that result in testing a *good*, objective blending of red, green, and blue.
[^3]: There is not a precise consensus for the specific values in the visible light spectrum, so the approximation is based on the references used, noting that other references may indicate different values.
[^4]: Due to the subjectivity, I turned up each color one at a time to get a closer *visual* match. RGB(255,0,0) seemed too orange-red, so I increased the wavelength a scosche.
[^5]: Like all *tools*, this should not be used as a be-all solution, rather it provides a good template -- but still requires manual editing and/or verbiage modification.
[^6]: Realized this was not necessary, but it's not like it took more than a second.
[^7]: Quoted from National Library of Medicine:  
  `The term photophobia is a misnomer and not quite accurate. It comes from two Greek words: photo- “light” and phobia “fear or dread of”—hence, “fear of light.”  
  It is defined as an abnormal sensitivity to light that can induce or exacerbate pain.`
[^!]: Due to the nature of LED light and flashing, it's important to note that this project may be an issue for anyone who has epilepsy.

<!--Collection of Links-->

[1]:README.md/#wiring
[2]:imgs/hardware-drawing.jpg
[3]:https://academo.org/demos/wavelength-to-colour-relationship/
[4]: https://academo.org/demos/blackbody-radiation/
[5]:https://marketplace.visualstudio.com/items?itemName=mintlify.document
[6]:https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text
[7]:https://doc.rust-lang.org/beta/rustdoc/index.html
[8]:https://doc.rust-lang.org/clippy
[9]: https://www.markdownguide.org/
[10]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references
[11]: https://www.spectralcalc.com/blackbody_calculator/blackbody.php
[12]: https://academo.org/demos/colour-temperature-relationship/
[13]: imgs/planck-calculator-input&results.png
[14]: README.md/#mini-experiment
[15]: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC3485070/
