//! Macros for implementing transparent colour types consistently.

/// Macro to implement `Display` for transparent colour types.
#[macro_export]
macro_rules! impl_transparent_display {
    ($type:ty) => {
        impl<T: Float + Send + Sync> Display for $type {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "{}", self.colour())
            }
        }
    };
}
