use crate::{Error, Result};
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct Color(pub [u8; 3]);

impl Deref for Color {
    type Target = [u8; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Color> for Rgba<u8> {
    fn from(val: Color) -> Self {
        Rgba([val[0], val[1], val[2], 255])
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Self([value[0], value[1], value[2]])
    }
}
impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Self(value)
    }
}
impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.starts_with('#') {
            let value = value.strip_prefix('#').unwrap();
            if value.len() != 6 {
                return Err(format!("'{}' starts_with # but not a color", value).into());
            }
            let r = u8::from_str_radix(&value[0..2], 16)
                .map_err(|_| format!("'{}' starts_with # but not a color", value))?;
            let g = u8::from_str_radix(&value[2..4], 16)
                .map_err(|_| format!("'{}' starts_with # but not a color", value))?;
            let b = u8::from_str_radix(&value[4..6], 16)
                .map_err(|_| format!("'{}' starts_with # but not a color", value))?;
            Ok(Self([r, g, b]))
        } else {
            Err(format!("'{}' is not starts_with #", value).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let color = Color::try_from("#FF5733").unwrap();
        assert_eq!(color.0, [255, 87, 51]);
    }

    #[test]
    fn test_color_from_invalid_hex() {
        let color = Color::try_from("#ZZZZZZ");
        assert!(color.is_err());
    }

    #[test]
    fn test_color_from_invalid_named_color() {
        let color = Color::try_from("notacolor");
        assert!(color.is_err());
    }

    #[test]
    fn test_color_into_rgba() {
        let color = Color([128, 64, 32]);
        let rgba: Rgba<u8> = color.into();
        assert_eq!(rgba.0, [128, 64, 32, 255]);
    }

    #[test]
    fn test_color_from_rgba_array() {
        let color = Color::from([128, 64, 32, 255]);
        assert_eq!(color.0, [128, 64, 32]);
    }

    #[test]
    fn test_color_deref() {
        let color = Color([10, 20, 30]);
        assert_eq!(color[0], 10);
        assert_eq!(color[1], 20);
        assert_eq!(color[2], 30);
    }
}
