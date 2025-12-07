#![feature(ascii_char_variants)]
#![feature(ascii_char)]

use std::string::String;
use std::ascii;
use std::ascii::Char::{CapitalS, CircumflexAccent, FullStop, VerticalLine};
use std::collections::HashSet;
use advent_of_code::{debug_println, Map2D};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    Some(trace_beams(&Map2D::new(input)))
}

const START: ascii::Char = CapitalS;
const SPLIT: ascii::Char = CircumflexAccent;
const SPACE: ascii::Char = FullStop;
const BEAM: ascii::Char = VerticalLine;

type Beams = HashSet<BeamPos>;
type BeamPos = u32;

fn start_beam(map: &Map2D) -> Beams {
    (0..map.cols)
        .find(|&col| map.get(0, col) == Some(START))
        .map(|p| HashSet::from([p as BeamPos]))
        .expect("The beam must start at first line on 'S' character")
}

fn split_beams(map: &Map2D, level: usize, prev: Beams) -> (usize, Beams) {
    let (split, keep): (Beams, Beams) = prev
        .into_iter()
        .partition(|&b| map.get(level, b as usize) == Some(SPLIT));
    let splits = split.iter().flat_map(|b| [b-1, b+1]).collect();
    (split.len(),
     keep.union(&splits).copied().collect()
    )
}

fn trace_beams(map: &Map2D) -> usize {
    print_level(map, 0, &HashSet::new());
    let mut beams = start_beam(map);
    let mut splits = 0;
    for level in 1..map.rows {
        print_level(map, level, &beams);
        let (new_splits, new_beams) = split_beams(map, level, beams);
        splits += new_splits;
        beams = new_beams;
    }
    splits
}

fn print_level(map: &Map2D, level: usize, beams: &Beams) {
    debug_println!("{}", (0..map.cols)
        .map(|col| {
            let c = map.get(level, col).unwrap();
            if c == SPACE && beams.contains(&(col as BeamPos)) {
                BEAM
            } else {
                c
            }
        })
        .collect::<String>())
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
