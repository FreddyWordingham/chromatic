//! Implements the `Convert` trait for `Lab`.

use num_traits::Float;

use crate::{
    Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
};

impl<T: Float + Send + Sync> Convert<T> for Lab<T> {
    #[inline]
    fn to_grey(&self) -> Grey<T> {
        // For greyscale, we should just use the L component (lightness)
        // We need to normalize from [0, 100] to [0, 1]
        let l_normalized = self.lightness / T::from(100.0).unwrap();
        Grey::new(l_normalized)
    }

    #[inline]
    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        let l_normalized = self.lightness / T::from(100.0).unwrap();
        GreyAlpha::new(l_normalized, T::one())
    }

    #[inline]
    fn to_hsl(&self) -> Hsl<T> {
        self.to_rgb().to_hsl()
    }

    #[inline]
    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        let hsl = self.to_hsl();
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    #[inline]
    fn to_hsv(&self) -> Hsv<T> {
        // Convert Lab to HSV via XYZ and RGB
        self.to_xyz().to_rgb().to_hsv()
    }

    #[inline]
    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    #[inline]
    fn to_lab(&self) -> Self {
        *self
    }

    #[inline]
    fn to_lab_alpha(&self) -> LabAlpha<T> {
        LabAlpha::new(self.lightness(), self.a_star(), self.b_star(), T::one())
    }

    #[inline]
    fn to_rgb(&self) -> Rgb<T> {
        // Convert Lab to RGB via XYZ
        self.to_xyz().to_rgb()
    }

    #[inline]
    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    #[inline]
    fn to_srgb(&self) -> Srgb<T> {
        // Convert Lab to sRGB via XYZ
        self.to_xyz().to_srgb()
    }

    #[inline]
    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let srgb = self.to_srgb();
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    #[inline]
    fn to_xyz(&self) -> Xyz<T> {
        // Constants for the conversion
        let epsilon = T::from(0.008856).unwrap(); // Intent is 216/24389
        let kappa = T::from(903.3).unwrap(); // Intent is 24389/27

        // D65 reference white
        let ref_white = Xyz::<T>::d65_reference_white();

        // Compute f_y
        let l = self.lightness;
        let f_y = (l + T::from(16.0).unwrap()) / T::from(116.0).unwrap();

        // Compute f_x and f_z using a and b
        let f_x = self.a_star / T::from(500.0).unwrap() + f_y;
        let f_z = f_y - self.b_star / T::from(200.0).unwrap();

        // Convert f values to XYZ coordinates
        let x_r = if f_x.powi(3) > epsilon {
            f_x.powi(3)
        } else {
            (f_x * T::from(116.0).unwrap() - T::from(16.0).unwrap()) / kappa
        };

        let y_r = if l > T::from(8.0).unwrap() {
            ((l + T::from(16.0).unwrap()) / T::from(116.0).unwrap()).powi(3)
        } else {
            l / kappa
        };

        let z_r = if f_z.powi(3) > epsilon {
            f_z.powi(3)
        } else {
            (f_z * T::from(116.0).unwrap() - T::from(16.0).unwrap()) / kappa
        };

        // Scale by reference white
        let x = x_r * ref_white.x();
        let y = y_r * ref_white.y();
        let z = z_r * ref_white.z();

        Xyz::new(x, y, z)
    }

    #[inline]
    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}
