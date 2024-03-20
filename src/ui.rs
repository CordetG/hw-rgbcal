//! The UI state integrates external input using the buttons of the microbit v2.
//!
//! This UI module contains utility functions for the microbit input.
//!
//! # Layout
//!
//! At the top level, is the Ui struct with a knob, _button_a, _button_b, and
//! a state. The Ui implementation initializes the Ui and runs asynchronously based
//! on the input and updates the output.
//!
//! At the inner level, is a UiState struct with and array of type 'u32' levels
//! and a frame_rate of type 'u64'. The UiState implementation outputs the
//! state of the color and labels. The Default State in the UiState hierarchy
//! is implemented when there is no input.

use crate::*;

/// The `UiState` struct contains an array of three `u32` levels and a `u64` frame rate.
///
/// Properties:
///
/// * `levels`: The `levels` property in the `UiState` struct is an array of unsigned 32-bit integers
/// with a length of 3. This array is used to store information about the levels in the user interface
/// state.
/// * `frame_rate`: The `frame_rate` property in the `UiState` struct represents the frame rate of the
/// user interface. It is of type `u64`that stores the number of frames displayed per second.
struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

/// The `impl UiState` block with the `show` method is implementing functionality for the `UiState`
/// struct.
impl UiState {
    /// The function `show` prints out the names and levels of colors, as well as the frame
    /// rate.
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

/// The `impl Default for UiState` block with the `default` method is implementing the `Default` trait
/// for the `UiState` struct. This allows instances of `UiState` to be created with a default set of
/// values when no specific values are provided.
impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// The `Ui` struct contains a `knob`, two buttons, and a state for the user interface.
///
/// Properties:
///
/// * `knob`: The knob property represents input from a potentiometer.
/// * `_button_a`: The `_button_a` property is a private field of type `Button` within the `Ui` struct.
/// It is intended to be used only within the module where it is defined and not accessible outside of it.
/// * `_button_b`: The `_button_b` property is a private field of type `Button` within the `Ui` struct.
/// It is intended to be used only within the module where it is defined and not accessible outside of it.
/// * `state`: The `state` property in the `Ui` struct represents the current state of the user
/// interface. It contains information such as which state is being displayed,
/// any user input, and other relevant data needed to manage the UI's behavior and appearance.
pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

/// The `impl Ui` block is implementing functionality for the `Ui` struct.
impl Ui {
    /// The function `new` creates a new instance of a struct with a knob, two buttons, and a default UI
    /// state in Rust.
    ///
    /// Arguments:
    ///
    /// * `knob`: The `knob` parameter is of type `Knob` and is used to represent a physical knob input
    /// in the user interface.
    /// * `_button_a`: The `_button_a` parameter is of type `Button`. It is one of the input parameters
    /// for the `new` function, which takes a `Knob` parameter, two `Button` parameters (`_button_a` and
    /// `_button_b`), and returns an instance of the struct that contains
    /// * `_button_b`: The `_button_b` parameter in the `new` function is of type `Button`. It is one of
    /// the input parameters for creating a new instance of the struct or object that the function
    /// belongs to.
    ///
    /// Returns:
    ///
    /// A new instance of the struct that this function belongs to is being returned.
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    /// The async 'run' function continuously measures a knob input, updates RGB levels, and displays
    /// the state until interrupted.
    pub async fn run(&mut self) -> ! {
        self.state.levels[2] = self.knob.measure().await;
        self.state.frame_rate = self.knob.measure().await as u64;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            // get knob measurement
            let level = self.knob.measure().await;
            // update blue from knob
            if level != self.state.levels[2] {
                self.state.levels[2] = level;
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }
            // update framerate from knob
            if level as u64 != self.state.frame_rate {
                self.state.frame_rate = level as u64;
            }
            self.state.show();
            Timer::after_millis(50).await;
        }
    }
}
