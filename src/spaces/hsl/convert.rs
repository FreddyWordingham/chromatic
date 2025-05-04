//! Implements the `Convert` trait for `Hsl`.

use num_traits::Float;

use crate::{
    Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
};

impl<T: Float + Send + Sync> Convert<T> for Hsl<T> {
    #[inline]
    fn to_grey(&self) -> Grey<T> {
        Grey::new(self.lightness)
    }

    #[inline]
    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new(self.lightness, T::one())
    }

    #[inline]
    fn to_hsv(&self) -> Hsv<T> {
        // Conversion from HSL to HSV
        let v = if self.lightness + self.saturation * (T::one() - (T::from(2.0).unwrap() * self.lightness - T::one()).abs())
            < T::zero()
        {
            T::zero() // Clamp to valid range
        } else {
            self.lightness + self.saturation * (T::one() - (T::from(2.0).unwrap() * self.lightness - T::one()).abs())
        };

        let s = if v.abs() < T::epsilon() {
            T::zero() // Avoid division by zero
        } else {
            T::from(2.0).unwrap() * (T::one() - self.lightness / v)
        };

        Hsv::new(self.hue, s, v)
    }

    #[inline]
    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    #[inline]
    fn to_hsl(&self) -> Hsl<T> {
        self.clone()
    }

    #[inline]
    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        HslAlpha::new(self.hue, self.saturation, self.lightness, T::one())
    }

    #[inline]
    fn to_lab(&self) -> Lab<T> {
        // Convert HSL to Lab via RGB and XYZ
        self.to_rgb().to_lab()
    }

    #[inline]
    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    #[inline]
    fn to_rgb(&self) -> Rgb<T> {
        let lightness = self.lightness;
        let saturation = self.saturation;

        // If saturation is 0, the color is a shade of gray
        if saturation.abs() < T::epsilon() {
            return Rgb::new(lightness, lightness, lightness);
        }

        // Helper function for HSL to RGB conversion
        let hue_to_rgb = |p: T, q: T, mut t: T| -> T {
            let f6 = T::from(6.0).unwrap();
            let f2 = T::from(2.0).unwrap();
            let f3 = T::from(3.0).unwrap();

            // Normalize t to be in range [0, 1]
            if t < T::zero() {
                t = t + T::one();
            }
            if t > T::one() {
                t = t - T::one();
            }

            if t < T::one() / f6 {
                return p + (q - p) * f6 * t;
            }
            if t < T::one() / f2 {
                return q;
            }
            if t < f2 / f3 {
                return p + (q - p) * (f2 / f3 - t) * f6;
            }
            p
        };

        let q = if lightness < T::from(0.5).unwrap() {
            lightness * (T::one() + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };

        let p = T::from(2.0).unwrap() * lightness - q;
        let h = self.hue / T::from(360.0).unwrap();

        let r = hue_to_rgb(p, q, h + T::from(1.0 / 3.0).unwrap());
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - T::from(1.0 / 3.0).unwrap());

        Rgb::new(r, g, b)
    }

    #[inline]
    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    #[inline]
    fn to_srgb(&self) -> Srgb<T> {
        // Convert HSL to sRGB via linear RGB
        let rgb = self.to_rgb();
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
        // Convert HSL to XYZ via linear RGB
        self.to_rgb().to_xyz()
    }

    #[inline]
    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}
