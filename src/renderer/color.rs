pub type Color = (u8, u8, u8, u8);

pub const BLACK: Color = (0, 0, 0, 0xFF);
pub const WHITE: Color = (0xFF, 0xFF, 0xFF, 0xFF);
pub const RED: Color = (0xFF, 0, 0, 0xFF);
pub const GREEN: Color = (0, 0xFF, 0, 0xFF);
pub const BLUE: Color = (0, 0, 0xFF, 0xFF);

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    (r, g, b, 0xFF)
}

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    (r, g, b, a)
}
