#![feature(ascii_char)]
#![feature(ascii_char_variants)]
extern crate core;

use ascii::Char::{CommercialAt, LineFeed};
use std::ascii;
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map2D::new(input);
    print_accessible(&map);
    Some(accessible_paper(&map))
}

struct Map2D<'a> {
    raw: &'a [ascii::Char],
    cols: usize,
    rows: usize,
}

impl Map2D<'_> {
    fn new(input: &'_ str) -> Map2D<'_> {
        let raw = input.as_ascii().unwrap();
        let mut line_pos = raw.iter().positions(|&c| c == LineFeed);
        let cols = line_pos.next().unwrap();
        debug_assert!(line_pos.all(|p| (p+1).is_multiple_of(cols+1)), "map must be rectangle");
        let rows = raw.iter().positions(|&c| c == LineFeed).count();
        Map2D {raw, cols, rows}
    }

    fn get_i(&self, row: isize, col: isize) -> Option<ascii::Char> {
        if row < 0 || col < 0 {
            None
        } else {
            self.get(row as usize, col as usize)
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<ascii::Char> {
        self.raw.get(row * (self.cols + 1) + col).copied()
    }

    fn neighbors(&self, row: usize, col: usize) -> [Option<ascii::Char>; 8] {
        Itertools::collect_array((-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&r| r != (0,0))
            .map(|(r, c)| self.get_i(row as isize + r, col as isize + c)))
            .expect("8 neighbors")
    }
}

fn is_paper(map: &Map2D, row: usize, col: usize) -> bool {
    map.get(row, col) == Some(CommercialAt)
}

fn is_paper_accessible(map: &Map2D, row: usize, col: usize) -> bool {
    let papers_near = map.neighbors(row, col)
        .into_iter()
        .filter(|&c| c == Some(CommercialAt))
        .count();
    papers_near < 4
}

fn accessible_paper(map: &Map2D) -> usize {
    (0..map.rows)
        .cartesian_product(0..map.cols)
        .filter(|&(r, c)| is_paper(map, r, c) && is_paper_accessible(map, r, c))
        .count()
}

fn print_accessible(map: &Map2D) {
    for r in 0..map.rows {
        for c in 0..map.cols {
            print!("{}",
                if !is_paper(map, r, c) {'.'}
                else if is_paper_accessible(map, r, c) {'x'}
                else {'@'}
            )
        }
        println!()
    }
}

pub fn part_two(_: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
