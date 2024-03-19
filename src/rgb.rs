//! Implementing the RGB LED pins.
//!
//! This rgb module contains utility functions for the RGB LED light.
//!
//! # Layout
//!
//! At the top level, is a type alias RgbPins for a 3-element array of type 'Output<'static, AnyPin>' used by the Rgb
//! struct. The implementation of the Rgb struct sets the frame rate 'u64' and initializes a new Rgb object.
//! The async functions perform the step and run operations for the mutable Rgb object.


use crate::*;

/// RgbPins is a type alias `RgbPins`. This alias represents an array of 3 elements, where each element is of type
/// `Output<'static, AnyPin>`.
type RgbPins = [Output<'static, AnyPin>; 3];

/// The `Rgb` struct represents an RGB LED with pins, levels, and tick time variables.
/// 
/// Properties:
/// 
/// * `rgb`: The `rgb` property in the `Rgb` struct is of type `RgbPins`. It represents the pins
/// used for controlling the RGB color output.
/// * `levels`: The `levels` property in the `Rgb` struct is an array of 3 unsigned 32-bit integers. It
/// is used to store the levels of the RGB pins.
/// * `tick_time`: The `tick_time` property in the `Rgb` struct is of type `u64`,
/// an unsigned 64-bit integer. This property is likely used to store frame-rate information..
pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

/// The `impl Rgb` block is implementing methods associated with the `Rgb` struct. Inside this
/// block, you define functions that operate on instances of the `Rgb` struct. 
/// The `impl Rgb {...}` block contains the implementation of methods for the `Rgb` struct, such as
/// `frame_tick_time`, `new`, `step`, and `run`.
impl Rgb {
    /// The function `frame_tick_time` calculates the time interval for each frame based on the frame
    /// rate and number of levels.
    /// 
    /// Arguments:
    /// 
    /// * `frame_rate`: The `frame_rate` parameter represents the number of frames per second.
    /// 
    /// Returns:
    /// 
    /// The function `frame_tick_time` returns a `u64` value, which represents the time in microseconds
    /// for each frame tick based on the given frame rate and the constant `LEVELS`.
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    /// The function `new` initializes a struct with RGB pins and frame rate parameters.
    /// 
    /// Arguments:
    /// 
    /// * `rgb`: The `rgb` parameter is of type `RgbPins`, represents the pins used for
    /// controlling an RGB LED.
    /// * `frame_rate`: The `frame_rate` parameter represents the number of frames per second at which
    /// the RGB colors will be updated.
    /// 
    /// Returns:
    /// 
    /// A new instance of the struct with the provided RGB pins and frame rate, along with default
    /// levels and calculated tick time.
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// This async function controls the behavior of an LED based on its level and timing parameters.
    /// 
    /// Arguments:
    /// 
    /// * `led`: The `led` parameter in the `step` function is the index of the LED for which the step
    /// operation is being performed.
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// This function runs asynchronously in a loop, updating RGB levels and stepping through LED colors.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
