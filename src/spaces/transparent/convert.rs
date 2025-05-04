//! Implements the `Convert` trait for transparent colour types.

/// Macro to implement the `Convert` trait for transparent colour types.
#[macro_export]
macro_rules! impl_transparent_convert {
    ($type:ty, $base:ty) => {
        impl<T: Float + Send + Sync> Convert<T> for $type {
            #[inline]
            fn to_grey(&self) -> Grey<T> {
                self.colour().to_grey()
            }

            #[inline]
            fn to_grey_alpha(&self) -> GreyAlpha<T> {
                let grey = self.colour().to_grey();
                GreyAlpha::new(grey.grey(), self.alpha())
            }

            #[inline]
            fn to_hsl(&self) -> Hsl<T> {
                self.colour().to_hsl()
            }

            #[inline]
            fn to_hsl_alpha(&self) -> HslAlpha<T> {
                let hsl = self.colour().to_hsl();
                HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), self.alpha())
            }

            #[inline]
            fn to_hsv(&self) -> Hsv<T> {
                self.colour().to_hsv()
            }

            #[inline]
            fn to_hsv_alpha(&self) -> HsvAlpha<T> {
                let hsv = self.colour().to_hsv();
                HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), self.alpha())
            }

            #[inline]
            fn to_lab(&self) -> Lab<T> {
                self.colour().to_lab()
            }

            #[inline]
            fn to_lab_alpha(&self) -> LabAlpha<T> {
                let lab = self.colour().to_lab();
                LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), self.alpha())
            }

            #[inline]
            fn to_rgb(&self) -> Rgb<T> {
                self.colour().to_rgb()
            }

            #[inline]
            fn to_rgb_alpha(&self) -> RgbAlpha<T> {
                let rgb = self.colour().to_rgb();
                RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), self.alpha())
            }

            #[inline]
            fn to_srgb(&self) -> Srgb<T> {
                self.colour().to_srgb()
            }

            #[inline]
            fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
                let srgb = self.colour().to_srgb();
                SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), self.alpha())
            }

            #[inline]
            fn to_xyz(&self) -> Xyz<T> {
                self.colour().to_xyz()
            }

            #[inline]
            fn to_xyz_alpha(&self) -> XyzAlpha<T> {
                let xyz = self.colour().to_xyz();
                XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), self.alpha())
            }
        }
    };
}
