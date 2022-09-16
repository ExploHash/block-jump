use font8x8::{UnicodeFonts, BASIC_FONTS};
use crate::WIDTH;

pub fn render_text(
    buffer: &mut Vec<u32>,
    text: &str,
    start_x: &usize,
    start_y: &usize,
    color: u32,
) {
    let mut space_between: usize = 0;

    for letter in text.chars() {
        render_letter(buffer, letter, &(start_x + &space_between), start_y, color);
        space_between += 10;
    }
}

pub fn render_letter(
    buffer: &mut Vec<u32>,
    letter: char,
    start_x: &usize,
    start_y: &usize,
    color: u32,
) {
    if let Some(glyph) = BASIC_FONTS.get(letter) {
        for (y, bitmap) in glyph.iter().enumerate() {
            for x in 0..8 {
                if *bitmap & 1 << x != 0 {
                    buffer[(((start_y + y) * WIDTH) + start_x + x) as usize] = color;
                }
            }
        }
    }
}

pub fn draw_rectangle(
    buffer: &mut Vec<u32>,
    start_x: &usize,
    start_y: &usize,
    width: &usize,
    color: usize,
) {
    for y in 0..*width {
        for x in 0..*width {
            buffer[(((start_y + y - 1) * WIDTH) + start_x + x) as usize] = color as u32;
        }
    }
}

pub fn draw_background(buffer: &mut Vec<u32>) {
    let color: u32 = 0x000000;

    for i in buffer.iter_mut() {
        *i = color; // write something more funny here!
    }
}
