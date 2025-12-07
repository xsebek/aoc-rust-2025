#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use std::ascii;
use std::ascii::Char::{CapitalS, CircumflexAccent, FullStop, VerticalLine};
use std::collections::HashMap;
#[allow(unused_imports)]
use std::string::String;

use advent_of_code::{debug_println, Map2D};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    Some(trace_beams(&Map2D::new(input)).0)
}

const START: ascii::Char = CapitalS;
const SPLIT: ascii::Char = CircumflexAccent;
#[allow(unused)]
const SPACE: ascii::Char = FullStop;
#[allow(unused)]
const BEAM: ascii::Char = VerticalLine;

type Beams = HashMap<BeamPos, usize>;
type BeamPos = u32;

fn start_beam(map: &Map2D) -> Beams {
    (0..map.cols)
        .find(|&col| map.get(0, col) == Some(START))
        .map(|p| Beams::from([(p as BeamPos, 1)]))
        .expect("The beam must start at first line on 'S' character")
}

fn split_beams(map: &Map2D, level: usize, prev: Beams) -> (usize, Beams) {
    let (split, keep): (Beams, Beams) = prev
        .into_iter()
        .partition(|&(b, _)| map.get(level, b as usize) == Some(SPLIT));
    let splits = split
        .iter()
        .flat_map(|(&b, &count)| [(b-1, count), (b+1, count)]);
    (split.len(), union_counts(keep, splits))
}

fn union_counts(left: Beams, right: impl Iterator<Item=(BeamPos, usize)>) -> Beams {
    let mut result = left;
    for (b, count) in right {
        let current_count = result.get(&b).copied().unwrap_or(0);
        result.insert(b, current_count + count);
    }
    result
}

fn trace_beams(map: &Map2D) -> (usize, Beams) {
    print_level(map, 0, &Beams::new());
    let mut beams = start_beam(map);
    let mut splits = 0;
    for level in 1..map.rows {
        print_level(map, level, &beams);
        let (new_splits, new_beams) = split_beams(map, level, beams);
        splits += new_splits;
        beams = new_beams;
    }
    (splits, beams)
}

#[allow(unused)]
fn print_level(map: &Map2D, level: usize, beams: &Beams) {
    debug_println!("{}", (0..map.cols)
        .map(|col| {
            let c = map.get(level, col).unwrap();
            if c == SPACE && beams.contains_key(&(col as BeamPos)) {
                BEAM
            } else {
                c
            }
        })
        .collect::<String>())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(trace_beams(&Map2D::new(input)).1.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
