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

    pub const RED: Color = Color::new(255, 0, 0, 255);
}
