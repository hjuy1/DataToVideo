use crate::{Error, Result};
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref, sync::LazyLock};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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
                return Err("value starts_with # but not a color".into());
            }
            let r = u8::from_str_radix(&value[0..2], 16)
                .map_err(|_| "value starts_with # but not a color")?;
            let g = u8::from_str_radix(&value[2..4], 16)
                .map_err(|_| "value starts_with # but not a color")?;
            let b = u8::from_str_radix(&value[4..6], 16)
                .map_err(|_| "value starts_with # but not a color")?;
            Ok(Self([r, g, b]))
        } else {
            COLOR.get(value).copied().ok_or("value not a color".into())
        }
    }
}

static COLOR: LazyLock<HashMap<&str, Color>> = LazyLock::new(|| {
    HashMap::from([
        ("red", Color([255, 0, 0])),
        ("blue", Color([0, 0, 255])),
        ("green", Color([0, 255, 0])),
        ("yellow", Color([255, 255, 0])),
        ("white", Color([255, 255, 255])),
        ("black", Color([0, 0, 0])),
        ("gray", Color([128, 128, 128])),
        ("cyan", Color([0, 255, 255])),
        ("magenta", Color([255, 0, 255])),
        ("orange", Color([255, 165, 0])),
        ("purple", Color([128, 0, 128])),
        ("pink", Color([255, 192, 203])),
        ("brown", Color([165, 42, 42])),
        ("lime", Color([0, 255, 0])),
        ("teal", Color([0, 128, 128])),
        ("maroon", Color([128, 0, 0])),
        ("olive", Color([128, 128, 0])),
        ("navy", Color([0, 0, 128])),
        ("aqua", Color([0, 255, 255])),
        ("silver", Color([192, 192, 192])),
        ("gold", Color([255, 215, 0])),
        ("violet", Color([238, 130, 238])),
        ("indigo", Color([75, 0, 130])),
        ("rose", Color([255, 192, 203])),
        ("crimson", Color([220, 20, 60])),
        ("coral", Color([255, 127, 80])),
        ("cadetblue", Color([95, 158, 160])),
        ("chartreuse", Color([127, 255, 0])),
        ("chocolate", Color([210, 105, 30])),
        ("coral", Color([255, 127, 80])),
        ("cornflowerblue", Color([100, 149, 237])),
        ("cornsilk", Color([255, 248, 220])),
        ("crimson", Color([220, 20, 60])),
        ("darkblue", Color([0, 0, 139])),
        ("darkcyan", Color([0, 139, 139])),
        ("darkgoldenrod", Color([184, 134, 11])),
        ("darkgray", Color([169, 169, 169])),
        ("darkgreen", Color([0, 100, 0])),
        ("darkkhaki", Color([189, 183, 107])),
        ("darkmagenta", Color([139, 0, 139])),
        ("darkolivegreen", Color([85, 107, 47])),
        ("darkorange", Color([255, 140, 0])),
        ("darkorchid", Color([153, 50, 204])),
    ])
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let color = Color::try_from("#FF5733").unwrap();
        assert_eq!(color.0, [255, 87, 51]);
    }

    #[test]
    fn test_color_from_named_color() {
        let color = Color::try_from("red").unwrap();
        assert_eq!(color.0, [255, 0, 0]);
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

    #[test]
    fn test_static_color_map() {
        assert_eq!(COLOR.get("blue").unwrap().0, [0, 0, 255]);
        assert_eq!(COLOR.get("green").unwrap().0, [0, 255, 0]);
    }
}
