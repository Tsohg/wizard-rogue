use std::fmt::Display;

use bevy::prelude::*;

use super::color::Color;
use super::glyph::Glyph;
use super::glyphset::GlyphSet;

#[derive(Resource)]
pub struct Terminal {
    buffer: Vec<Glyph>,
    size: (usize, usize),
    glyphset: GlyphSet,
}

impl Terminal {
    pub fn new(glyphset: GlyphSet, width: usize, height: usize) -> Self {
        Self {
            buffer: vec![Glyph::default(); width * height],
            glyphset,
            size: (width, height),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Glyph::default())
    }

    pub fn put(&mut self, x: i32, y: i32, code: u8, fg: Color, bg: Color) {
        if x < 0 || y < 0 { return }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width() || y >= self.height() { return }
        let index = x + y * self.width();
        if index < self.buffer.len() {
            self.buffer[index as usize] = Glyph::new(code, fg, bg)
        }
    }

    pub fn put_str(&mut self, x: i32, y: i32, string: &str, fg: Color, bg: Color) {
        for (i, c) in string.char_indices() {
            self.put(i as i32 + x, y, c as u8, fg, bg)
        }
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..(self.size.0 * self.size.1) {
            let x = i % self.size.0;
            write!(f, "{}", self.buffer[i])?;
            if x + 1 >= self.size.0 {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

pub fn render_terminal(terminal: &Terminal, screen: &mut [u8]) {
    let char_width = terminal.glyphset.char_width;
    let char_height = terminal.glyphset.char_height;
    let pixel_width = terminal.width() * terminal.glyphset.char_width;

    for tile_index in 0..terminal.buffer.len() {
        let glyph = terminal.buffer[tile_index];

        let x = tile_index % terminal.width();
        let y = tile_index / terminal.width();

        for j in 0..terminal.glyphset.glyph_length {
            let cx = x * char_width + j % char_width;
            let cy = y * char_height + j / char_height;
            let index = terminal.glyphset.index(glyph.code) + j;

            let color = if terminal.glyphset.get(index) {
                glyph.fg
            } else {
                glyph.bg
            };
            let pixel_index = (cx + cy * pixel_width) * 4;
            screen[pixel_index + 0] = color.0;
            screen[pixel_index + 1] = color.1;
            screen[pixel_index + 2] = color.2;
            screen[pixel_index + 3] = color.3;
        }
    }
}
