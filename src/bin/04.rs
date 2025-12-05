#![feature(ascii_char)]
#![feature(ascii_char_variants)]
use std::io::Write;
extern crate core;

use std::ascii;
use std::collections::HashMap;
use std::io::stdout;
use std::iter::repeat;
use advent_of_code::Map2D;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map2D::new(input);
    print_accessible(&map);
    Some(accessible_paper(&map).count())
}

const PAPER: ascii::Char = ascii::Char::CommercialAt;
const ACCESSIBLE: ascii::Char = ascii::Char::SmallX;
const SPACE: ascii::Char = ascii::Char::FullStop;

fn is_paper(map: &Map2D, row: usize, col: usize) -> bool {
    map.get(row, col) == Some(PAPER)
}

fn is_paper_accessible(map: &Map2D, row: usize, col: usize) -> bool {
    let papers_near = map.neighbors(row, col)
        .into_iter()
        .filter(|&c| c == Some(PAPER))
        .count();
    papers_near < 4
}

fn accessible_paper(map: &Map2D) -> impl Iterator<Item=(usize, usize)> {
    map.range().filter(|&(r, c)| is_paper(map, r, c) && is_paper_accessible(map, r, c))
}

fn show_pos(map: &Map2D, row: usize, col: usize) -> ascii::Char {
    if !is_paper(map, row, col) { SPACE }
    else if is_paper_accessible(map, row, col) { ACCESSIBLE }
    else { PAPER }
}

fn print_accessible(map: &Map2D) {
    for r in 0..map.rows {
        for c in 0..map.cols {
            print!("{}", show_pos(map, r, c))
        }
        println!()
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(take_all_accessible_paper(&mut Map2D::new(input)))
}

fn take_accessible_paper(map: &mut Map2D) -> bool {
    let count_before = map.overwrite_count();
    map.set_many(HashMap::from_iter(accessible_paper(map).zip(repeat(SPACE))));
    count_before != map.overwrite_count()
}

fn take_all_accessible_paper(map: &mut Map2D) -> usize {
    while take_accessible_paper(map) {
        let mut lock = stdout().lock();
        writeln!(lock).expect("no error");
        print_accessible(map);
        println!();
    }
    map.overwrite_count()
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
        assert_eq!(result, Some(43));
    }
}
