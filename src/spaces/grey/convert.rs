//! Implements the `Convert` trait for `Grey`.

use num_traits::Float;

use crate::{
    Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
};

impl<T: Float + Send + Sync> Convert<T> for Grey<T> {
    #[inline]
    fn to_grey(&self) -> Self {
        *self
    }

    #[inline]
    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new(self.grey, T::one())
    }

    #[inline]
    fn to_hsl(&self) -> Hsl<T> {
        // For greyscale, hue is undefined (0), saturation is 0, and lightness equals the grey value
        Hsl::new(T::zero(), T::zero(), self.grey)
    }

    #[inline]
    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        HslAlpha::new(T::zero(), T::zero(), self.grey, T::one())
    }

    #[inline]
    fn to_hsv(&self) -> Hsv<T> {
        // For greyscale, hue is undefined (0), saturation is 0, and value equals the grey value
        Hsv::new(T::zero(), T::zero(), self.grey)
    }

    #[inline]
    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        HsvAlpha::new(T::zero(), T::zero(), self.grey, T::one())
    }

    #[inline]
    fn to_lab(&self) -> Lab<T> {
        // Convert Grey to Lab via XYZ
        self.to_xyz().to_lab()
    }

    #[inline]
    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    #[inline]
    fn to_rgb(&self) -> Rgb<T> {
        Rgb::new(self.grey, self.grey, self.grey)
    }

    #[inline]
    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        RgbAlpha::new(self.grey, self.grey, self.grey, T::one())
    }

    #[inline]
    fn to_srgb(&self) -> Srgb<T> {
        let sg = Srgb::gamma_encode(self.grey);
        Srgb::new(sg, sg, sg)
    }

    #[inline]
    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let sg = Srgb::gamma_encode(self.grey);
        SrgbAlpha::new(sg, sg, sg, T::one())
    }

    #[inline]
    fn to_xyz(&self) -> Xyz<T> {
        // Grey in XYZ space with D65 reference white
        // For greyscale, X, Y, and Z values are proportional to the reference white
        // Y (luminance) equals grey value, and X and Z are scaled according to D65

        // Simplified approach: use the luminance (Y) value directly,
        // and scale X and Z based on D65 reference white
        let white = Xyz::<T>::d65_reference_white();

        // Scale all values by the grey value (luminance)
        let x = white.x() * self.grey();
        let y = self.grey(); // Y value is directly the grey value (luminance)
        let z = white.z() * self.grey();

        Xyz::new(x, y, z)
    }

    #[inline]
    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}
