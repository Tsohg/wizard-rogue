use super::color::Color;
use image::{io::Reader as ImageReader, GenericImageView, Pixel};

pub struct GlyphSet {
    bitmap: Vec<bool>,
    pub char_width: usize,
    pub char_height: usize,
    // Actual bit length
    pub glyph_length: usize,
}

impl GlyphSet {
    pub fn index(&self, glyph: u8) -> usize {
        self.glyph_length * glyph as usize
    }

    pub fn get(&self, index: usize) -> bool {
        self.bitmap[index]
    }
}

pub fn build_glyphset(
    map_size: (usize, usize),
    char_size: (usize, usize),
    data: Vec<Color>,
) -> GlyphSet {
    let mut bitmap = vec![false; map_size.0 * map_size.1 * char_size.0 * char_size.1];
    let glyph_length = char_size.0 * char_size.1;
    let image_width = map_size.0 * char_size.0;

    // Align bits into char chunks
    for i in 0..data.len() {
        let value = data[i].0 > 128;
        // Pixel coord
        let x = i % image_width;
        let y = i / image_width;

        // Char coord
        let cx = x / char_size.0;
        let cy = y / char_size.1;
        let char_index = cx + cy * map_size.0;

        // Packed pixel index
        let within_char_x = x % char_size.0;
        let within_char_y = y % char_size.1;
        let pixel_index = within_char_y * char_size.0 + within_char_x;

        bitmap[glyph_length * char_index + pixel_index] = value;
    }

    GlyphSet {
        bitmap,
        glyph_length,
        char_width: char_size.0,
        char_height: char_size.1,
    }
}

pub fn read_glyphset_file(
    file: &str,
    map_size: (usize, usize),
    char_size: (usize, usize),
) -> GlyphSet {
    let img = ImageReader::open(file).unwrap().decode().unwrap();
    let pixels = img
        .pixels()
        .map(|(_, _, pixel)| pixel.to_rgba())
        .map(|data| (data.0[0], data.0[1], data.0[2], data.0[3]))
        .collect();
    build_glyphset(map_size, char_size, pixels)
}
