/// Import all items from the root module of the crate
use crate::*;

/// The line `pub type Adc = saadc::Saadc<'static, 1>;` is creating a public type alias `Adc` for the
/// `Saadc` struct from the `saadc` module. The `Saadc` struct is a generic struct that takes two type
/// parameters, in this case, `'static` and `1`.
pub type Adc = saadc::Saadc<'static, 1>;

/// A struct representing a knob connected to an ADC (Analog to Digital Converter) signal
pub struct Knob(Adc);

/// The `impl Knob { ... }` block in the Rust code snippet is implementing methods for the `Knob`
/// struct.
impl Knob {
    /// The function `new` in Rust initializes a new instance of a struct with a given Adc object after
    /// calibrating it asynchronously.
    ///
    /// Arguments:
    ///
    /// * `adc`: The `adc` parameter in the `new` function is an instance of the `Adc` struct.
    ///
    /// Returns:
    ///
    /// A new instance of the struct with the provided `Adc` object after calibrating the ADC
    /// asynchronously.
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    /// This Rust function asynchronously measures a value, scales it, and calculates a result based on
    /// certain conditions.
    ///
    /// Returns:
    ///
    /// The `measure` function returns a `u32` value.
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
