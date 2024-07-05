use std::fmt::Display;

use super::color::{Color, BLACK};

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    pub code: u8,
    pub fg: Color,
    pub bg: Color,
}

impl Glyph {
    pub fn new(code: u8, fg: Color, bg: Color) -> Self {
        Self { code, fg, bg }
    }
}

impl Default for Glyph {
    fn default() -> Self {
        Self {
            code: 0,
            fg: BLACK,
            bg: BLACK,
        }
    }
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code as char)
    }
}
