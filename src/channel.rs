//! Provides functionality for converting between channel representation types.

/// Helper trait to convert a u8 value to a channel representation type.
pub trait Channel {
    /// Converts a u8 to the channel representation type.
    fn from_u8(value: u8) -> Self;
}

// Direct implementation for u8.
impl Channel for u8 {
    #[inline]
    fn from_u8(value: u8) -> Self {
        value
    }
}

/// Channel for integer types other than u8.
macro_rules! impl_channel_for_int {
    ($($t:ty),*) => {
        $(
            impl Channel for $t {
                #[inline]
                fn from_u8(value: u8) -> Self {
                    <$t>::from(value)
                }
            }
        )*
    };
}

/// Channel for float types.
macro_rules! impl_channel_for_float {
    ($($t:ty),*) => {
        $(
            impl Channel for $t {
                #[inline]
                fn from_u8(value: u8) -> Self {
                    <$t>::from(value) / <$t>::from(255u8)
                }
            }
        )*
    };
}

impl_channel_for_int!(u16, u32, u64, usize);
impl_channel_for_float!(f32, f64);
