#![feature(exact_length_collection)]

use advent_of_code::Map2D;
use std::str::FromStr;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let (_presents, regions) = parse(input);
    Some(
        regions
            .iter()
            .filter(|r| {
                let area = r.length * r.width;
                let presents_area = r.shapes.map(|s| s * 9).iter().sum();
                area >= presents_area
            })
            .count(),
    )
}

type Present<'a> = Map2D<'a>;
struct Region {
    width: u32,
    length: u32,
    shapes: [u32; 6],
}

fn parse(input: &'_ str) -> (Vec<Present<'_>>, Vec<Region>) {
    const PRESENT_SIZE: usize = 3;
    const PRESENTS_COUNT: usize = 6;
    (
        Vec::new(),
        input
            .lines()
            .skip((PRESENT_SIZE + 2) * PRESENTS_COUNT)
            .map(parse_region)
            .collect(),
    )
}

fn parse_region(line: &str) -> Region {
    let (dim, reg) = line.split_once(':').expect("DIM: SHAPES");
    let (w, l) = dim.split_once('x').expect("WxH");
    let shapes = reg
        .split_whitespace()
        .map(str::trim)
        .flat_map(u32::from_str)
        .collect_array()
        .expect("N N N N N N");
    Region {
        width: u32::from_str(w).expect("W"),
        length: u32::from_str(l).expect("L"),
        shapes,
    }
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
