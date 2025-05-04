//! XYZ colour representation.
//! The XYZ colour space is a device-independent colour space defined by the CIE (International Commission on Illumination).
//! It was created to be a standard reference space for mapping human colour perception.

use num_traits::Float;

mod colour;
mod convert;
mod fmt;

/// XYZ colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Xyz<T: Float + Send + Sync> {
    /// X component.
    x: T,
    /// Y component (luminance).
    y: T,
    /// Z component.
    z: T,
}

impl<T: Float + Send + Sync> Xyz<T> {
    /// Create a new `Xyz` instance.
    /// Note: XYZ values are theoretically unbounded, but non-negative values are enforced here for practical reasons.
    /// Typical values for D65 reference white are X ≈ 0.95, Y = 1.0, Z ≈ 1.09.
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        debug_assert!(x >= T::zero(), "X component should be non-negative.");
        debug_assert!(y >= T::zero(), "Y component should be non-negative.");
        debug_assert!(z >= T::zero(), "Z component should be non-negative.");
        Self { x, y, z }
    }

    /// Get the `x` component.
    #[inline]
    pub const fn x(&self) -> T {
        self.x
    }

    /// Get the `y` component (luminance).
    #[inline]
    pub const fn y(&self) -> T {
        self.y
    }

    /// Get the `z` component.
    #[inline]
    pub const fn z(&self) -> T {
        self.z
    }

    /// Set the `x` component.
    #[inline]
    pub fn set_x(&mut self, x: T) {
        debug_assert!(x >= T::zero(), "X component should be non-negative.");
        self.x = x;
    }

    /// Set the `y` component (luminance).
    #[inline]
    pub fn set_y(&mut self, y: T) {
        debug_assert!(y >= T::zero(), "Y component should be non-negative.");
        self.y = y;
    }

    /// Set the `z` component.
    #[inline]
    pub fn set_z(&mut self, z: T) {
        debug_assert!(z >= T::zero(), "Z component should be non-negative.");
        self.z = z;
    }

    /// Create an XYZ colour representing the D65 standard illuminant (daylight, 6504K).
    #[inline]
    pub fn d65_reference_white() -> Self {
        Self::new(T::from(0.95047).unwrap(), T::from(1.0).unwrap(), T::from(1.08883).unwrap())
    }

    /// Create an XYZ colour representing the D50 standard illuminant (horizon light, 5003K).
    #[inline]
    pub fn d50_reference_white() -> Self {
        Self::new(T::from(0.96422).unwrap(), T::from(1.0).unwrap(), T::from(0.82521).unwrap())
    }

    /// Get XYZ values relative to D65 reference white.
    /// Returns (X/Xn, Y/Yn, Z/Zn)
    #[inline]
    pub fn relative_to_white(&self) -> (T, T, T) {
        let white = Self::d65_reference_white();
        (self.x / white.x, self.y / white.y, self.z / white.z)
    }

    /// Calculate perceptual colour difference in XYZ space (simple Euclidean distance).
    /// Note: This is not an ideal colour difference metric - consider using Lab with Delta E metrics for better results.
    #[inline]
    pub fn distance(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
