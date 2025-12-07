#![feature(ascii_char)]
#![feature(ascii_char_variants)]
extern crate core;

use advent_of_code::{Map2D, debug_print, debug_println};
use std::ascii;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;

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
    let papers_near = map
        .neighbors(row, col)
        .filter(|&c| c == PAPER)
        .take(4)
        .count();
    papers_near < 4
}

fn accessible_paper(map: &Map2D) -> impl Iterator<Item = (usize, usize)> {
    map.range()
        .filter(|&(r, c)| is_paper(map, r, c) && is_paper_accessible(map, r, c))
}

#[allow(unused)]
fn show_pos(map: &Map2D, row: usize, col: usize) -> ascii::Char {
    if !is_paper(map, row, col) {
        SPACE
    } else if is_paper_accessible(map, row, col) {
        ACCESSIBLE
    } else {
        PAPER
    }
}

#[allow(unused)]
fn print_accessible(map: &Map2D) {
    for r in 0..map.rows {
        for c in 0..map.cols {
            debug_print!("{}", show_pos(map, r, c))
        }
        debug_println!()
    }
    debug_println!();
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(take_all_accessible_paper(&mut Map2D::new(input)))
}

fn take_accessible_paper(map: &mut Map2D) -> HashSet<(usize, usize)> {
    let accessible = HashMap::from_iter(accessible_paper(map).zip(repeat(SPACE)));
    map.set_many(&accessible);
    accessible
        .into_iter()
        .flat_map(|((r, c), _)| map.neighbor_pos(r, c))
        .collect()
}

// TODO: use paper BTreeSet and iteratively filter/map/intersect/union

fn accessible_paper2(
    map: &Map2D,
    indices: HashSet<(usize, usize)>,
) -> impl Iterator<Item = (usize, usize)> {
    indices
        .into_iter()
        .filter(|&(r, c)| is_paper(map, r, c) && is_paper_accessible(map, r, c))
}

fn take_accessible_paper_search_set(
    map: &mut Map2D,
    indices: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let accessible = HashMap::from_iter(accessible_paper2(map, indices).zip(repeat(SPACE)));
    map.set_many(&accessible);
    accessible
        .into_iter()
        .flat_map(|((r, c), _)| map.neighbor_pos(r, c))
        .collect()
}

fn take_all_accessible_paper(map: &mut Map2D) -> usize {
    let mut indices = take_accessible_paper(map);
    while !indices.is_empty() {
        indices = take_accessible_paper_search_set(map, indices);
        //println!("{}", indices.len());
        // print_accessible(map);
        // sleep(Duration::from_secs(1))
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
