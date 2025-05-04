//! Implements the `Convert` trait for `Xyz`.

use num_traits::Float;

use crate::{
    Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
};

impl<T: Float + Send + Sync> Convert<T> for Xyz<T> {
    #[inline]
    fn to_grey(&self) -> Grey<T> {
        // Use the Y component (luminance) for greyscale
        // Clamp to [0, 1] range for Grey
        Grey::new(self.y.min(T::one()))
    }

    #[inline]
    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new(self.y.min(T::one()), T::one())
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
        // Convert XYZ to HSV via linear RGB
        self.to_rgb().to_hsv()
    }

    #[inline]
    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    #[inline]
    fn to_lab(&self) -> Lab<T> {
        // Constants for the conversion
        let epsilon = T::from(0.008856).unwrap(); // Intent is 216/24389
        let kappa = T::from(903.3).unwrap(); // Intent is 24389/27

        // Get XYZ values relative to reference white (D65)
        let (x_r, y_r, z_r) = self.relative_to_white();

        // Compute f(x), f(y), f(z)
        let f_x = if x_r > epsilon {
            x_r.powf(T::from(1.0 / 3.0).unwrap())
        } else {
            (kappa * x_r + T::from(16.0).unwrap()) / T::from(116.0).unwrap()
        };

        let f_y = if y_r > epsilon {
            y_r.powf(T::from(1.0 / 3.0).unwrap())
        } else {
            (kappa * y_r + T::from(16.0).unwrap()) / T::from(116.0).unwrap()
        };

        let f_z = if z_r > epsilon {
            z_r.powf(T::from(1.0 / 3.0).unwrap())
        } else {
            (kappa * z_r + T::from(16.0).unwrap()) / T::from(116.0).unwrap()
        };

        // Compute Lab components
        let l = T::from(116.0).unwrap() * f_y - T::from(16.0).unwrap();
        let a = T::from(500.0).unwrap() * (f_x - f_y);
        let b = T::from(200.0).unwrap() * (f_y - f_z);

        Lab::new(l, a, b)
    }

    #[inline]
    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    #[inline]
    fn to_rgb(&self) -> Rgb<T> {
        // XYZ to linear RGB transformation
        // Using the inverse of the RGB to XYZ matrix
        let r =
            self.x * T::from(3.2404542).unwrap() - self.y * T::from(1.5371385).unwrap() - self.z * T::from(0.4985314).unwrap();

        let g =
            -self.x * T::from(0.9692660).unwrap() + self.y * T::from(1.8760108).unwrap() + self.z * T::from(0.0415560).unwrap();

        let b =
            self.x * T::from(0.0556434).unwrap() - self.y * T::from(0.2040259).unwrap() + self.z * T::from(1.0572252).unwrap();

        // Clamp to [0, 1] range
        let r = r.max(T::zero()).min(T::one());
        let g = g.max(T::zero()).min(T::one());
        let b = b.max(T::zero()).min(T::one());

        Rgb::new(r, g, b)
    }

    #[inline]
    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    #[inline]
    fn to_srgb(&self) -> Srgb<T> {
        // Convert XYZ to sRGB via linear RGB
        let rgb = self.to_rgb();

        // Apply gamma encoding to get sRGB
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
        self.clone()
    }

    #[inline]
    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        XyzAlpha::new(self.x(), self.y(), self.z(), T::one())
    }
}
