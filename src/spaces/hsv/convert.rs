//! Implements the `Convert` trait for `Hsv`.

use num_traits::Float;

use crate::{
    Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
};

impl<T: Float + Send + Sync> Convert<T> for Hsv<T> {
    #[inline]
    fn to_grey(&self) -> Grey<T> {
        Grey::new(self.value)
    }

    #[inline]
    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new(self.value, T::one())
    }

    #[inline]
    fn to_hsl(&self) -> Hsl<T> {
        // Convert HSV to HSL
        // Hue remains the same
        let hue = self.hue;

        // Calculate lightness based on value and saturation
        let lightness = self.value * (T::one() - self.saturation / T::from(2.0).unwrap());

        // Calculate saturation for HSL
        let saturation = if lightness.abs() < T::epsilon() || (lightness - T::one()).abs() < T::epsilon() {
            // If lightness is 0 or 1, saturation is 0
            T::zero()
        } else {
            let min_val = lightness * T::from(2.0).unwrap() - T::one();
            let max_val = min_val + self.saturation * (T::one() - min_val.abs());

            // Calculate HSL saturation from min and max values
            (max_val - min_val) / (T::one() - (T::from(2.0).unwrap() * lightness - T::one()).abs())
        };

        Hsl::new(hue, saturation, lightness)
    }

    #[inline]
    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        let hsl = self.to_hsl();
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    #[inline]
    fn to_hsv(&self) -> Hsv<T> {
        self.clone()
    }

    #[inline]
    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        HsvAlpha::new(self.hue, self.saturation, self.value, T::one())
    }

    #[inline]
    fn to_lab(&self) -> Lab<T> {
        // Convert HSV to Lab via XYZ
        self.to_xyz().to_lab()
    }

    #[inline]
    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    #[inline]
    fn to_rgb(&self) -> Rgb<T> {
        let h = self.hue;
        let s = self.saturation;
        let v = self.value;

        let c = v * s;
        let h_prime = h / T::from(60.0).unwrap();
        let x = c * (T::one() - ((h_prime % T::from(2.0).unwrap()) - T::one()).abs());
        let m = v - c;

        let (r, g, b) = if h < T::from(60.0).unwrap() {
            (c, x, T::zero())
        } else if h < T::from(120.0).unwrap() {
            (x, c, T::zero())
        } else if h < T::from(180.0).unwrap() {
            (T::zero(), c, x)
        } else if h < T::from(240.0).unwrap() {
            (T::zero(), x, c)
        } else if h < T::from(300.0).unwrap() {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        Rgb::new(r + m, g + m, b + m)
    }

    #[inline]
    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    #[inline]
    fn to_srgb(&self) -> Srgb<T> {
        // Convert HSV to sRGB via linear RGB
        // First convert to linear RGB
        let rgb = self.to_rgb();

        // Then convert linear RGB to sRGB
        let r_srgb = Srgb::gamma_encode(rgb.red());
        let g_srgb = Srgb::gamma_encode(rgb.green());
        let b_srgb = Srgb::gamma_encode(rgb.blue());

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    #[inline]
    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let srgb = self.to_srgb();
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    #[inline]
    fn to_xyz(&self) -> Xyz<T> {
        // Convert HSV to XYZ via linear RGB
        self.to_rgb().to_xyz()
    }

    #[inline]
    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}
