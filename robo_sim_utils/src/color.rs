#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const RED: Color = Color::new(255, 0, 0, 255);
    pub const BLUE: Color = Color::new(0, 0, 255, 255);

    pub fn from_str(color_str: &str) -> Self {
        match color_str.to_lowercase().as_str() {
            "red" => Self::RED,
            "blue" => Self::BLUE,
            _ => Self::BLACK,
        }
    }
}
