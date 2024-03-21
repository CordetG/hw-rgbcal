#![no_std]
#![no_main]

/// Crate modules to synchronize the use of the potentiometer, RGE LED, and the buttons
/// from the UI.
mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

/// Importing and using the `panic_rtt_target` crate to handle panics in the embedded
/// Rust application.
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

/// Imports for various embassy crates for handling embedded functionality
use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

/// The line `pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);` is
/// declaring a static variable named `RGB_LEVELS` of type `Mutex<ThreadModeRawMutex, [u32; 3]>` and
/// initializing it with a new mutex containing an array of three unsigned 32-bit integers `[0; 3]`.
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);

/// The constant `LEVELS` represents a specific value related to the RGB LED levels.
pub const LEVELS: u32 = 16;

/// The line `pub static FR_RATE: Mutex<ThreadModeRawMutex, u64> = Mutex::new(10);` in the Rust code
/// snippet is declaring a static variable named `FR_RATE` of type `Mutex<ThreadModeRawMutex, u64>` and
/// initializing it with a new mutex containing a single unsigned 64-bit integer value `10`.
pub static FR_RATE: Mutex<ThreadModeRawMutex, u64> = Mutex::new(10);

/// This async Rust function retrieves RGB levels by locking a shared resource.
///
/// Returns:
///
/// The function `get_rgb_levels` is returning an array of 3 unsigned 32-bit integers representing RGB
/// levels.
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

/// The function `get_frame_rate` in Rust retrieves the frame rate value by locking a mutex and
/// returning the rate.
/// 
/// Returns:
/// 
/// The function `get_frame_rate` is returning an unsigned 64-bit integer representing the frame rate.
async fn get_frame_rate() -> u64 {
    let rate = FR_RATE.lock().await;
    *rate
}

/// The function `set_rgb_levels` is an asynchronous function that takes a closure as an
/// argument to set RGB levels.
///
/// Arguments:
///
/// * `setter`: The `setter` parameter is a closure that takes a mutable reference to an array of three
/// `u32` values and modifies the values in the array. It is used to set the RGB levels in the
/// `set_rgb_levels` function.
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

/// The function `set_frame_rate` is an asynchronous function that takes a closure as input to
/// set the frame rate.
/// 
/// Arguments:
/// 
/// * `setter`: The `setter` parameter is a closure that takes a mutable reference to a `u64` value and
/// modifies it. It is used to set the frame rate value in the `set_frame_rate` function.
async fn set_frame_rate<F>(setter: F)
where
    F: FnOnce(&mut u64),
{
    let mut rate = FR_RATE.lock().await;
    setter(&mut rate);
}

/// The function initializes various components on a microcontroller board, sets up interrupts,
/// configures LED pins, reads input from a knob, and runs RGB and UI tasks concurrently before
/// panicking at the end.
///
/// Arguments:
///
/// * `_spawner`: The `_spawner` parameter in the `main` function is of type `Spawner`. This parameter
/// is typically used in embedded Rust applications to spawn tasks or threads. In this case, the
/// `Spawner` type is being used to spawn asynchronous tasks.
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();
    let board = Microbit::default();

    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // The code snippet is setting up the LED pins for the RGB LED on a microcontroller
    // board.
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 10);

    // This code snippet is configuring and initializing the SAADC (Successive Approximation
    // Analog-to-Digital Converter) module on a microcontroller board.
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );
    // setup potentiometer
    let knob = Knob::new(saadc).await;
    // setup ui
    let mut ui: Ui = Ui::new(knob, board.btn_a, board.btn_b);

    // The line `join::join(rgb.run(), ui.run()).await;` in the Rust code snippet is using the `join`
    // function from the `embassy_futures` crate to concurrently run the tasks returned by `rgb.run()`
    // and `ui.run()`.
    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
