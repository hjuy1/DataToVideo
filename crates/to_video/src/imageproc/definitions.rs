use image::{ImageBuffer, Pixel};

/// An `ImageBuffer` containing Pixels of type P with storage `Vec<P::Subpixel>`.
/// Most operations in this library only support inputs of type `Image`, rather
/// than arbitrary `image::GenericImage`s. This is obviously less flexible, but
/// has the advantage of allowing many functions to be more performant. We may want
/// to add more flexibility later, but this should not be at the expense of performance.
/// When specialisation lands we should be able to do this by defining traits for images
/// with contiguous storage.
pub type Image<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;

/// A type to which we can clamp a value of type T.
/// Implementations are not required to handle `NaN`s gracefully.
pub trait Clamp<T> {
    /// Clamp `x` to a valid value for this type.
    fn clamp(x: T) -> Self;
}

/// Creates an implementation of Clamp<From> for type To.
macro_rules! implement_clamp {
    ($from:ty, $to:ty, $min:expr, $max:expr, $min_from:expr, $max_from:expr) => {
        impl Clamp<$from> for $to {
            fn clamp(x: $from) -> $to {
                if x < $max_from as $from {
                    if x > $min_from as $from {
                        x as $to
                    } else {
                        $min
                    }
                } else {
                    $max
                }
            }
        }
    };
}

/// Implements Clamp<T> for T, for all input types T.
macro_rules! implement_identity_clamp {
    ( $($t:ty),* ) => {
        $(
            impl Clamp<$t> for $t {
                fn clamp(x: $t) -> $t { x }
            }
        )*
    };
}

implement_clamp!(i16, u8, u8::MIN, u8::MAX, u8::MIN as i16, u8::MAX as i16);
implement_clamp!(u16, u8, u8::MIN, u8::MAX, u8::MIN as u16, u8::MAX as u16);
implement_clamp!(i32, u8, u8::MIN, u8::MAX, u8::MIN as i32, u8::MAX as i32);
implement_clamp!(u32, u8, u8::MIN, u8::MAX, u8::MIN as u32, u8::MAX as u32);
implement_clamp!(f32, u8, u8::MIN, u8::MAX, u8::MIN as f32, u8::MAX as f32);
implement_clamp!(f64, u8, u8::MIN, u8::MAX, u8::MIN as f64, u8::MAX as f64);

implement_clamp!(
    i32,
    u16,
    u16::MIN,
    u16::MAX,
    u16::MIN as i32,
    u16::MAX as i32
);
implement_clamp!(
    f32,
    u16,
    u16::MIN,
    u16::MAX,
    u16::MIN as f32,
    u16::MAX as f32
);
implement_clamp!(
    f64,
    u16,
    u16::MIN,
    u16::MAX,
    u16::MIN as f64,
    u16::MAX as f64
);

implement_clamp!(
    i32,
    i16,
    i16::MIN,
    i16::MAX,
    i16::MIN as i32,
    i16::MAX as i32
);

implement_identity_clamp!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
