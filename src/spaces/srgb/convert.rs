//! Implements the `Convert` trait for `Srgb`.

use num_traits::Float;

use crate::{
    Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
};

impl<T: Float + Send + Sync> Convert<T> for Srgb<T> {
    #[inline]
    fn to_grey(&self) -> Grey<T> {
        // For perceptually correct greyscale, use the luminance formula
        // Y = 0.2126*R + 0.7152*G + 0.0722*B (same as in XYZ conversion)
        // This applies to gamma-encoded (non-linear) sRGB values
        let r_linear = Self::gamma_decode(self.red);
        let g_linear = Self::gamma_decode(self.green);
        let b_linear = Self::gamma_decode(self.blue);

        let y_linear = r_linear * T::from(0.2126729).unwrap()
            + g_linear * T::from(0.7151522).unwrap()
            + b_linear * T::from(0.0721750).unwrap();

        // Keep in linear space for Grey, as Grey is a linear space
        Grey::new(y_linear)
    }

    #[inline]
    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        // Use the luminance formula as in to_grey
        let grey = self.to_grey();
        GreyAlpha::new(grey.grey(), T::one())
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
        // Convert to linear RGB first for accurate HSV conversion
        self.to_rgb().to_hsv()
    }

    #[inline]
    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    #[inline]
    fn to_lab(&self) -> Lab<T> {
        // Convert to XYZ first, then to Lab
        self.to_xyz().to_lab()
    }

    #[inline]
    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    #[inline]
    fn to_rgb(&self) -> Rgb<T> {
        // Convert from gamma-encoded sRGB to linear RGB
        let r_linear = Self::gamma_decode(self.red);
        let g_linear = Self::gamma_decode(self.green);
        let b_linear = Self::gamma_decode(self.blue);

        Rgb::new(r_linear, g_linear, b_linear)
    }

    #[inline]
    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    #[inline]
    fn to_srgb(&self) -> Srgb<T> {
        self.clone()
    }

    #[inline]
    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        SrgbAlpha::new(self.red, self.green, self.blue, T::one())
    }

    #[inline]
    fn to_xyz(&self) -> Xyz<T> {
        // Convert to linear RGB first, then to XYZ
        self.to_rgb().to_xyz()
    }

    #[inline]
    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}
