use crate::{color::Color, slide::Position};

pub const POSITION_3_1: (Position, Position, Position) = (
    Position::new(1, 0, 520),
    Position::new(1, 520, 214),
    Position::new(1, 734, 346),
);
pub const POSITION_4_2: (Position, Position, Position, Position) = (
    Position::new(1, 0, 500),
    Position::new(1, 500, 180),
    Position::new(1, 680, 180),
    Position::new(1, 860, 220),
);
pub const COLOR_2_1: (Color, Color) = (Color([245, 160, 50]), Color([255, 225, 200]));
pub const COLOR_2_2: (Color, Color) = (Color([200, 250, 250]), Color([240, 240, 220]));
pub const COLOR_2_3: (Color, Color) = (Color([160, 100, 255]), Color([235, 235, 235]));
pub const COLOR_2_4: (Color, Color) = (Color([25, 150, 235]), Color([45, 85, 150]));
pub const COLOR_3_1: (Color, Color, Color) = (
    Color([245, 165, 50]),
    Color([255, 225, 150]),
    Color([200, 250, 250]),
);
pub const COLOR_4_1: (Color, Color, Color, Color) = (
    Color([245, 165, 50]),
    Color([255, 225, 150]),
    Color([200, 250, 250]),
    Color([240, 240, 220]),
);

pub const BLACK: Color = Color([0, 0, 0]);
pub const WHITE: Color = Color([255, 255, 255]);
pub const GRAY: Color = Color([128, 128, 128]);
pub const GOLD: Color = Color([255, 215, 0]);
pub const SILVER: Color = Color([192, 192, 192]);

pub const RED: Color = Color([255, 0, 0]);
pub const ORANGE: Color = Color([255, 165, 0]);
pub const YELLOW: Color = Color([255, 255, 0]);
pub const GREEN: Color = Color([0, 255, 0]);
pub const CYAN: Color = Color([0, 255, 255]);
pub const BLUE: Color = Color([0, 0, 255]);
pub const PURPLE: Color = Color([128, 0, 128]);

pub const VIOLET: Color = Color([238, 130, 238]);
pub const ORCHID: Color = Color([218, 112, 214]);
pub const PINK: Color = Color([255, 192, 203]);
pub const SNOW: Color = Color([255, 250, 250]);
pub const BROWN: Color = Color([165, 42, 42]);
