use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::config::{WIDTH, HEIGHT};

pub const CHAR_WIDTH: usize = 5;
pub const CHAR_HEIGHT: usize = 6;

pub const INVALID_CHAR: [u8; 6] = [
    0b01100,
    0b01100,
    0b01100,
    0b01100,
    0b00000,
    0b01100
];

pub const DOT_CHAR: char = '•';

lazy_static! {
    static ref CHARMAP: HashMap<char, [u8; 6]> = {
        let mut m = HashMap::new();
        m.insert(DOT_CHAR, [
            0b00000,
            0b00000,
            0b01110,
            0b01110,
            0b01110,
            0b00000
        ]);
        m.insert('0', [
            0b01100,
            0b10010,
            0b10110,
            0b11010,
            0b10010,
            0b01100
        ]);
        m.insert('1', [
            0b00100,
            0b01100,
            0b00100,
            0b00100,
            0b00100,
            0b01110
        ]);
        m.insert('2', [
            0b01100,
            0b00010,
            0b00010,
            0b00100,
            0b01000,
            0b01110
        ]);
        m.insert('3', [
            0b01110,
            0b00010,
            0b00100,
            0b00010,
            0b00010,
            0b01100
        ]);
        m.insert('4', [
            0b01010,
            0b01010,
            0b01110,
            0b00010,
            0b00010,
            0b00010
        ]);
        m.insert('5', [
            0b01110,
            0b01000,
            0b01100,
            0b00010,
            0b00010,
            0b01100
        ]);
        m.insert('6', [
            0b00100,
            0b01000,
            0b01100,
            0b01010,
            0b01010,
            0b00100
        ]);
        m.insert('7', [
            0b01110,
            0b00010,
            0b00010,
            0b00100,
            0b00100,
            0b00100
        ]);
        m.insert('8', [
            0b01110,
            0b01010,
            0b01110,
            0b01010,
            0b01010,
            0b01110
        ]);
        m.insert('9', [
            0b00100,
            0b01010,
            0b01010,
            0b00110,
            0b00010,
            0b00100
        ]);
        m.insert(' ', [
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b00000
        ]);
        m.insert('`', [
            0b01000,
            0b01000,
            0b00100,
            0b00000,
            0b00000,
            0b00000
        ]);
        m.insert('~', [
            0b00000,
            0b01000,
            0b10101,
            0b00010,
            0b00000,
            0b00000
        ]);
        m.insert('#', [
            0b01010,
            0b11111,
            0b01010,
            0b01010,
            0b11111,
            0b01010
        ]);
        m.insert('-', [
            0b00000,
            0b00000,
            0b00000,
            0b01110,
            0b00000,
            0b00000
        ]);
        m.insert('=', [
            0b00000,
            0b00000,
            0b01110,
            0b00000,
            0b01110,
            0b00000
        ]);
        m.insert('+', [
            0b00000,
            0b00000,
            0b00100,
            0b01110,
            0b00100,
            0b00000
        ]);
        m.insert('_', [
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b11110
        ]);
        m.insert('!', [
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b00000,
            0b00100
        ]);
        m.insert('@', [
            0b01100,
            0b10010,
            0b10110,
            0b10110,
            0b10000,
            0b01100
        ]);
        m.insert('$', [
            0b00100,
            0b01110,
            0b01000,
            0b00010,
            0b01110,
            0b00100
        ]);
        m.insert('%', [
            0b00000,
            0b10010,
            0b00100,
            0b01000,
            0b10010,
            0b00000
        ]);
        m.insert('^', [
            0b00100,
            0b01010,
            0b00000,
            0b00000,
            0b00000,
            0b00000
        ]);
        m.insert('&', [
            0b01000,
            0b10100,
            0b01000,
            0b10101,
            0b10010,
            0b01101
        ]);
        m.insert('*', [
            0b00100,
            0b01010,
            0b00100,
            0b00000,
            0b00000,
            0b00000
        ]);
        m.insert('(', [
            0b00100,
            0b01000,
            0b01000,
            0b01000,
            0b01000,
            0b00100
        ]);
        m.insert(')', [
            0b01000,
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b01000
        ]);
        m.insert('{', [
            0b00110,
            0b00100,
            0b01100,
            0b00100,
            0b00100,
            0b00110
        ]);
        m.insert('}', [
            0b01100,
            0b00100,
            0b00110,
            0b00100,
            0b00100,
            0b01100
        ]);
        m.insert('[', [
            0b01100,
            0b01000,
            0b01000,
            0b01000,
            0b01000,
            0b01100
        ]);
        m.insert(']', [
            0b01100,
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b01100
        ]);
        m.insert('\\', [
            0b10000,
            0b10000,
            0b01000,
            0b01000,
            0b00100,
            0b00100
        ]);
        m.insert('|', [
            0b01000,
            0b01000,
            0b01000,
            0b01000,
            0b01000,
            0b01000
        ]);
        m.insert('\'', [
            0b00100,
            0b00100,
            0b01000,
            0b00000,
            0b00000,
            0b00000
        ]);
        m.insert('"', [
            0b01010,
            0b01010,
            0b10100,
            0b00000,
            0b00000,
            0b00000
        ]);
        m.insert(':', [
            0b00000,
            0b00000,
            0b01000,
            0b00000,
            0b01000,
            0b00000
        ]);
        m.insert(';', [
            0b00000,
            0b00000,
            0b00100,
            0b00000,
            0b00100,
            0b01000
        ]);
        m.insert('.', [
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b01000
        ]);
        m.insert(',', [
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b01000,
            0b01000
        ]);
        m.insert('<', [
            0b00000,
            0b00100,
            0b01000,
            0b10000,
            0b01000,
            0b00100
        ]);
        m.insert('>', [
            0b00000,
            0b01000,
            0b00100,
            0b00010,
            0b00100,
            0b01000
        ]);
        m.insert('/', [
            0b00010,
            0b00010,
            0b00100,
            0b00100,
            0b01000,
            0b01000
        ]);
        m.insert('?', [
            0b00100,
            0b01010,
            0b00010,
            0b00100,
            0b00000,
            0b00100
        ]);
        m.insert('A', [
            0b01110,
            0b01010,
            0b01110,
            0b01010,
            0b01010,
            0b01010
        ]);
        m.insert('a', [
            0b00000,
            0b00000,
            0b01110,
            0b10010,
            0b10010,
            0b01110
        ]);
        m.insert('B', [
            0b01110,
            0b01010,
            0b01111,
            0b01001,
            0b01001,
            0b01111
        ]);
        m.insert('b', [
            0b01000,
            0b01000,
            0b01110,
            0b01001,
            0b01001,
            0b01110
        ]);
        m.insert('C', [
            0b01110,
            0b01000,
            0b01000,
            0b01000,
            0b01000,
            0b01110
        ]);
        m.insert('c', [
            0b00000,
            0b00000,
            0b00110,
            0b01000,
            0b01000,
            0b01110
        ]);
        m.insert('D', [
            0b11100,
            0b10010,
            0b10010,
            0b10010,
            0b10010,
            0b11100
        ]);
        m.insert('d', [
            0b00010,
            0b00010,
            0b01110,
            0b10010,
            0b10010,
            0b01110
        ]);
        m.insert('E', [
            0b01110,
            0b01000,
            0b01110,
            0b01000,
            0b01000,
            0b01110
        ]);
        m.insert('e', [
            0b00000,
            0b01100,
            0b10010,
            0b11110,
            0b10000,
            0b01100
        ]);
        m.insert('F', [
            0b01110,
            0b01000,
            0b01110,
            0b01000,
            0b01000,
            0b01000
        ]);
        m.insert('f', [
            0b00000,
            0b00110,
            0b01000,
            0b01110,
            0b01000,
            0b01000
        ]);
        m.insert('G', [
            0b01110,
            0b01000,
            0b01000,
            0b01011,
            0b01001,
            0b01111
        ]);
        m.insert('g', [
            0b00000,
            0b00000,
            0b01110,
            0b10000,
            0b10010,
            0b01110
        ]);
        m.insert('H', [
            0b01010,
            0b01010,
            0b01110,
            0b01010,
            0b01010,
            0b01010
        ]);
        m.insert('h', [
            0b01000,
            0b01000,
            0b01100,
            0b01010,
            0b01010,
            0b01010
        ]);
        m.insert('i', [
            0b00100,
            0b00000,
            0b01100,
            0b00100,
            0b00100,
            0b00110
        ]);
        m.insert('I', [
            0b01110,
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b01110
        ]);
        m.insert('j', [
            0b00100,
            0b00000,
            0b00100,
            0b00100,
            0b00100,
            0b01100
        ]);
        m.insert('J', [
            0b00110,
            0b00010,
            0b00010,
            0b00010,
            0b00010,
            0b01100
        ]);
        m.insert('K', [
            0b01001,
            0b01010,
            0b01100,
            0b01010,
            0b01001,
            0b01001
        ]);
        m.insert('k', [
            0b00000,
            0b01010,
            0b01010,
            0b01100,
            0b01010,
            0b01010
        ]);
        m.insert('l', [
            0b00000,
            0b01100,
            0b00100,
            0b00100,
            0b00100,
            0b00110
        ]);
        m.insert('L', [
            0b01000,
            0b01000,
            0b01000,
            0b01000,
            0b01000,
            0b01110
        ]);
        m.insert('M', [
            0b11110,
            0b10101,
            0b10101,
            0b10101,
            0b10101,
            0b10101
        ]);
        m.insert('m', [
            0b00000,
            0b00000,
            0b11110,
            0b10101,
            0b10101,
            0b10101,
        ]);
        m.insert('N', [
            0b01001,
            0b01101,
            0b01101,
            0b01011,
            0b01011,
            0b01001
        ]);
        m.insert('n', [
            0b00000,
            0b00000,
            0b11100,
            0b10010,
            0b10010,
            0b10010,
        ]);
        m.insert('O', [
            0b01100,
            0b10010,
            0b10010,
            0b10010,
            0b10010,
            0b11100
        ]);
        m.insert('o', [
            0b00000,
            0b00000,
            0b01100,
            0b10010,
            0b10010,
            0b01100
        ]);
        m.insert('P', [
            0b01100,
            0b01010,
            0b01110,
            0b01000,
            0b01000,
            0b01000
        ]);
        m.insert('p', [
            0b00000,
            0b01110,
            0b01010,
            0b01110,
            0b01000,
            0b01000,
        ]);
        m.insert('Q', [
            0b01110,
            0b01010,
            0b01010,
            0b01010,
            0b01110,
            0b00010
        ]);
        m.insert('q', [
            0b00000,
            0b01110,
            0b01010,
            0b01110,
            0b00010,
            0b00010
        ]);
        m.insert('R', [
            0b01100,
            0b01010,
            0b01100,
            0b01010,
            0b01010,
            0b01010
        ]);
        m.insert('r', [
            0b00000,
            0b00000,
            0b01010,
            0b01100,
            0b01000,
            0b01000
        ]);
        m.insert('S', [
            0b00110,
            0b01000,
            0b00100,
            0b00010,
            0b00010,
            0b01100
        ]);
        m.insert('s', [
            0b00000,
            0b00000,
            0b01110,
            0b01000,
            0b00010,
            0b01110
        ]);
        m.insert('T', [
            0b01110,
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b00100
        ]);
        m.insert('t', [
            0b00000,
            0b00100,
            0b01110,
            0b00100,
            0b00100,
            0b00110
        ]);
        m.insert('U', [
            0b01010,
            0b01010,
            0b01010,
            0b01010,
            0b01010,
            0b01110
        ]);
        m.insert('u', [
            0b00000,
            0b00000,
            0b01010,
            0b01010,
            0b01010,
            0b01110
        ]);
        m.insert('V', [
            0b01010,
            0b01010,
            0b01010,
            0b01010,
            0b01010,
            0b00100
        ]);
        m.insert('v', [
            0b00000,
            0b00000,
            0b01010,
            0b01010,
            0b01010,
            0b00100
        ]);
        m.insert('W', [
            0b10001,
            0b10101,
            0b10101,
            0b10101,
            0b10101,
            0b01010
        ]);
        m.insert('w', [
            0b00000,
            0b00000,
            0b10001,
            0b10101,
            0b10101,
            0b01010
        ]);
        m.insert('X', [
            0b01010,
            0b01010,
            0b00100,
            0b01010,
            0b01010,
            0b01010
        ]);
        m.insert('x', [
            0b00000,
            0b00000,
            0b01010,
            0b00100,
            0b01010,
            0b01010
        ]);
        m.insert('Y', [
            0b01010,
            0b01010,
            0b01010,
            0b00100,
            0b00100,
            0b00100
        ]);
        m.insert('y', [
            0b00000,
            0b00000,
            0b01010,
            0b01010,
            0b00100,
            0b00100
        ]);
        m.insert('Z', [
            0b01110,
            0b00010,
            0b00100,
            0b01000,
            0b01000,
            0b01110
        ]);
        m.insert('z', [
            0b00000,
            0b00000,
            0b01110,
            0b00010,
            0b01000,
            0b01110
        ]);
        m
    };
}

pub fn draw_char_at(original_row: usize, original_col: usize, c: &char, buffer: &mut [u32; WIDTH * HEIGHT], color: u32) {
    let map = CHARMAP.get(c).unwrap_or(&INVALID_CHAR);
    for row in 0..map.len() {
        let line = map[row];
        let line = [(line >> 4) & 1, (line >> 3) & 1, (line >> 2) & 1, (line >> 1) & 1, (line >> 0) & 1];
        for col in 0..line.len() {
            let r = original_row + row;
            let c = original_col + col;
            if line[col] == 1 {
                buffer[r * WIDTH + c] = color;
            }
        }
    }
}

pub fn draw_text(row: usize, col: usize, text: &str, buffer: &mut [u32; WIDTH * HEIGHT], color: u32) {
    let mut row = row;
    let mut i = 0;
    for c in text.chars() {
        if c == '\n' {
            row += CHAR_HEIGHT + 1;
            i = 0;
        } else {
            draw_char_at(row, col + i * CHAR_WIDTH, &c, buffer, color);
            i += 1;
        }
    }
}


