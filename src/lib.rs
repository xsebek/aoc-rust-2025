#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use itertools::Itertools;
use std::ascii;
use std::ascii::Char::LineFeed;
use std::collections::HashMap;

pub mod template;

// Use this file to add helper functions and additional modules.

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] print!($($arg)*));
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Range {
    pub first: i128,
    pub last: i128,
}

impl Range {
    pub fn contains(self, value: i128) -> bool {
        self.first <= value && value <= self.last
    }
}

pub struct Map2D<'a> {
    raw: &'a [ascii::Char],
    pub cols: usize,
    pub rows: usize,
    overwrite: HashMap<(usize, usize), ascii::Char>,
}

impl Map2D<'_> {
    pub fn new(input: &'_ str) -> Map2D<'_> {
        let raw = input.as_ascii().unwrap();
        let mut line_pos = raw.iter().positions(|&c| c == LineFeed);
        let cols = line_pos.next().unwrap();
        debug_assert!(
            line_pos.all(|p| (p + 1).is_multiple_of(cols + 1)),
            "map must be rectangle"
        );
        let rows = raw.iter().positions(|&c| c == LineFeed).count();
        Map2D {
            raw,
            cols,
            rows,
            overwrite: HashMap::new(),
        }
    }

    pub fn range(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.rows).cartesian_product(0..self.cols)
    }

    pub fn get_i(&self, row: isize, col: isize) -> Option<ascii::Char> {
        if row < 0 || col < 0 {
            None
        } else {
            self.get(row as usize, col as usize)
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<ascii::Char> {
        self.overwrite
            .get(&(row, col))
            .or(self.raw.get(row * (self.cols + 1) + col))
            .copied()
    }

    pub fn neighbors(&self, row: usize, col: usize) -> [Option<ascii::Char>; 8] {
        Itertools::collect_array(
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|&r| r != (0, 0))
                .map(|(r, c)| self.get_i(row as isize + r, col as isize + c)),
        )
        .expect("8 neighbors")
    }

    pub fn set(&mut self, row: usize, col: usize, value: ascii::Char) {
        self.overwrite.insert((row, col), value);
    }

    pub fn set_many(&mut self, values: HashMap<(usize, usize), ascii::Char>) {
        self.overwrite.extend(values)
    }

    pub fn overwrite_count(&self) -> usize {
        self.overwrite.len()
    }
}
