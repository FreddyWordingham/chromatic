//! Implements the `Convert` trait for `Rgb`.

use num_traits::Float;

use crate::{
    Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
};

impl<T: Float + Send + Sync> Convert<T> for Rgb<T> {
    #[inline]
    fn to_grey(&self) -> Grey<T> {
        Grey::new((self.red + self.green + self.blue) / T::from(3.0).unwrap())
    }

    #[inline]
    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new((self.red + self.green + self.blue) / T::from(3.0).unwrap(), T::one())
    }

    #[inline]
    fn to_hsl(&self) -> Hsl<T> {
        let r = self.red();
        let g = self.green();
        let b = self.blue();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        // Calculate lightness
        let lightness = (max + min) / T::from(2.0).unwrap();

        // If max equals min, the color is a shade of gray (no hue or saturation)
        if delta.abs() < T::epsilon() {
            return Hsl::new(T::zero(), T::zero(), lightness);
        }

        // Calculate saturation
        let saturation = if lightness <= T::from(0.5).unwrap() {
            delta / (max + min)
        } else {
            delta / (T::from(2.0).unwrap() - max - min)
        };

        // Calculate hue
        let hue = if r.abs_sub(max).abs() < T::epsilon() {
            // Red is max
            let segment = (g - b) / delta;
            let shift = T::zero();
            let segment_6 = segment / T::from(6.0).unwrap();

            // If green is less than blue, add 1.0 (360 degrees)
            if g < b {
                T::from(360.0).unwrap() + shift + segment_6 * T::from(360.0).unwrap()
            } else {
                shift + segment_6 * T::from(360.0).unwrap()
            }
        } else if g.abs_sub(max).abs() < T::epsilon() {
            // Green is max
            let segment = (b - r) / delta;
            let shift = T::from(1.0 / 3.0).unwrap();
            let segment_6 = segment / T::from(6.0).unwrap();

            shift + segment_6 * T::from(360.0).unwrap()
        } else {
            // Blue is max
            let segment = (r - g) / delta;
            let shift = T::from(2.0 / 3.0).unwrap();
            let segment_6 = segment / T::from(6.0).unwrap();

            shift + segment_6 * T::from(360.0).unwrap()
        };

        // Make sure hue is in the range [0, 360)
        let mut normalized_hue = hue;
        while normalized_hue >= T::from(360.0).unwrap() {
            normalized_hue = normalized_hue - T::from(360.0).unwrap();
        }
        while normalized_hue < T::zero() {
            normalized_hue = normalized_hue + T::from(360.0).unwrap();
        }

        Hsl::new(normalized_hue, saturation, lightness)
    }

    #[inline]
    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        let hsl = self.to_hsl();
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    #[inline]
    fn to_hsv(&self) -> Hsv<T> {
        let r = self.red();
        let g = self.green();
        let b = self.blue();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let value = max;

        let zero = T::zero();
        let sixty = T::from(60.0).unwrap();
        let two = T::from(2.0).unwrap();
        let four = T::from(4.0).unwrap();
        let six = T::from(6.0).unwrap();

        let saturation = if max == zero { zero } else { delta / max };

        let hue = if delta == zero {
            zero
        } else if max == r {
            let mut h = (g - b) / delta;
            if h < zero {
                h = h + six;
            }
            h * sixty
        } else if max == g {
            ((b - r) / delta + two) * sixty
        } else {
            ((r - g) / delta + four) * sixty
        };

        Hsv::new(hue, saturation, value)
    }

    #[inline]
    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    #[inline]
    fn to_lab(&self) -> Lab<T> {
        // Convert RGB to Lab via XYZ
        self.to_xyz().to_lab()
    }

    #[inline]
    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    #[inline]
    fn to_rgb(&self) -> Self {
        *self
    }

    #[inline]
    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        RgbAlpha::new(self.red(), self.green(), self.blue(), T::one())
    }

    #[inline]
    fn to_srgb(&self) -> Srgb<T> {
        // Convert from linear RGB to gamma-encoded sRGB
        let r_srgb = Srgb::gamma_encode(self.red);
        let g_srgb = Srgb::gamma_encode(self.green);
        let b_srgb = Srgb::gamma_encode(self.blue);

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    #[inline]
    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let srgb = self.to_srgb();
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    #[inline]
    fn to_xyz(&self) -> Xyz<T> {
        // Convert linear RGB to XYZ using the standard sRGB transform matrix
        // This matrix is for D65 reference white
        let x = self.red() * T::from(0.4124564).unwrap()
            + self.green() * T::from(0.3575761).unwrap()
            + self.blue() * T::from(0.1804375).unwrap();

        let y = self.red() * T::from(0.2126729).unwrap()
            + self.green() * T::from(0.7151522).unwrap()
            + self.blue() * T::from(0.0721750).unwrap();

        let z = self.red() * T::from(0.0193339).unwrap()
            + self.green() * T::from(0.1191920).unwrap()
            + self.blue() * T::from(0.9503041).unwrap();

        Xyz::new(x, y, z)
    }

    #[inline]
    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}
