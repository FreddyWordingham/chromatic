//! Trait implemented by all colour types.

use num_traits::Float;

use crate::{
    error::Result,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
};

/// Types implementing this trait can be converted to various `Colour` `crate::spaces`.
///
/// This trait provides a unified interface for converting between different colour spaces.
/// All conversions are performed with appropriate mathematical transformations and
/// gamma corrections where necessary (e.g., sRGB ↔ linear RGB conversions).
///
/// # Errors
///
/// All conversion methods may return errors in the following situations:
///
/// - **Mathematical operations fail**: Conversions involve complex mathematical transformations
///   that may fail due to numeric overflow, underflow, or invalid intermediate values
/// - **Type conversion errors**: When converting between different floating-point types
///   or when values exceed the representable range of the target type
/// - **Gamma correction failures**: sRGB conversions apply gamma encoding/decoding which
///   may fail for extreme values or when mathematical operations produce NaN/infinity
/// - **Reference white calculations**: XYZ and Lab conversions use standard illuminant
///   calculations that may fail if reference white values cannot be computed
/// - **Colour space constraints**: Some conversions may produce values outside the valid
///   range for the target colour space, requiring clamping or error reporting
/// - **Precision loss**: Conversions between colour spaces with different gamuts may
///   result in precision loss or out-of-gamut colours that cannot be accurately represented
///
/// Most errors are recoverable and indicate either invalid input values or limitations
/// in the conversion process rather than programming errors.
pub trait Convert<T: Float + Send + Sync> {
    /// Convert a colour to the `Grey` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Luminance calculation fails during conversion
    /// - The resulting grey value is outside the valid range [0, 1]
    /// - Type conversion operations fail
    fn to_grey(&self) -> Result<Grey<T>>;

    /// Convert a colour to the `GreyAlpha` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The underlying `to_grey()` conversion fails
    /// - Alpha component validation fails (should be [0, 1])
    /// - Type conversion operations fail
    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>>;

    /// Convert a colour to the `Hsl` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Intermediate RGB conversion fails
    /// - Hue calculation produces invalid values (NaN or infinite)
    /// - Saturation or lightness values fall outside [0, 1] range
    /// - Mathematical operations during HSL calculation fail
    fn to_hsl(&self) -> Result<Hsl<T>>;

    /// Convert a colour to the `HslAlpha` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The underlying `to_hsl()` conversion fails
    /// - Alpha component validation fails (should be [0, 1])
    /// - Type conversion operations fail
    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>>;

    /// Convert a colour to the `Hsv` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Intermediate RGB conversion fails
    /// - Hue calculation produces invalid values (NaN or infinite)
    /// - Saturation or value components fall outside [0, 1] range
    /// - Mathematical operations during HSV calculation fail
    fn to_hsv(&self) -> Result<Hsv<T>>;

    /// Convert a colour to the `HsvAlpha` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The underlying `to_hsv()` conversion fails
    /// - Alpha component validation fails (should be [0, 1])
    /// - Type conversion operations fail
    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>>;

    /// Convert a colour to the `Lab` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Intermediate XYZ conversion fails
    /// - Reference white (D65) calculations fail
    /// - Cube root operations produce invalid results
    /// - Lab components fall outside valid ranges (L*: [0, 100], a*/b*: [-128, 127])
    /// - Mathematical constants cannot be converted to the target type
    fn to_lab(&self) -> Result<Lab<T>>;

    /// Convert a colour to the `LabAlpha` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The underlying `to_lab()` conversion fails
    /// - Alpha component validation fails (should be [0, 1])
    /// - Type conversion operations fail
    fn to_lab_alpha(&self) -> Result<LabAlpha<T>>;

    /// Convert a colour to the `Rgb` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Colour space transformation matrix operations fail
    /// - Resulting RGB values exceed the valid range [0, 1] after clamping
    /// - Mathematical operations produce NaN or infinite values
    /// - Type conversion operations fail
    fn to_rgb(&self) -> Result<Rgb<T>>;

    /// Convert a colour to the `RgbAlpha` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The underlying `to_rgb()` conversion fails
    /// - Alpha component validation fails (should be [0, 1])
    /// - Type conversion operations fail
    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>>;

    /// Convert a colour to the `Srgb` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Intermediate linear RGB conversion fails
    /// - Gamma encoding operations fail or produce invalid values
    /// - Resulting sRGB values fall outside [0, 1] range
    /// - Mathematical operations during gamma correction fail
    fn to_srgb(&self) -> Result<Srgb<T>>;

    /// Convert a colour to the `SrgbAlpha` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The underlying `to_srgb()` conversion fails
    /// - Alpha component validation fails (should be [0, 1])
    /// - Type conversion operations fail
    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>>;

    /// Convert a colour to the `Xyz` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - RGB to XYZ transformation matrix operations fail
    /// - XYZ values fall outside the valid range [0, 1]
    /// - Mathematical operations produce NaN or infinite values
    /// - Type conversion of transformation constants fails
    fn to_xyz(&self) -> Result<Xyz<T>>;

    /// Convert a colour to the `XyzAlpha` colour space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The underlying `to_xyz()` conversion fails
    /// - Alpha component validation fails (should be [0, 1])
    /// - Type conversion operations fail
    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>>;
}
