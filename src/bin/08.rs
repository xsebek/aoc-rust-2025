use advent_of_code::{debug_print, debug_println, sorted_pairs};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    part_one_iter(1000, input)
}

pub fn part_one_iter(iterations: usize, input: &str) -> Option<usize> {
    let points = parse(input);
    let circuits = connect_closest(iterations, &points);
    Some(n_largest_circuits_product(3, circuits))
}

type Point = (i64, i64, i64);
type Dist = i64; // skip square root
type Circuits = Vec<Option<u32>>;

fn parse(input: &str) -> Vec<Point> {
    many1(terminated(parse_point, newline))
        .parse(input)
        .expect("must parse whole file")
        .1
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let num = nom::character::complete::i64;
    (num, preceded(tag(","), num), preceded(tag(","), num)).parse(input)
}

fn dist(a: Point, b: Point) -> Dist {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn k_closest_pairs(k: usize, points: &[Point]) -> impl Iterator<Item = (usize, usize)> {
    sorted_pairs(0, points.len()).k_smallest_by_key(k, |&(l, r)| dist(points[l], points[r]))
}

fn connect_closest(iterations: usize, points: &[Point]) -> Circuits {
    let mut result = vec![None; points.len()];
    let mut ix = 1;
    for (from, to) in k_closest_pairs(iterations, points) {
        debug_print!(
            "Connecting {:?}\tto {:?}\t(dist {}) - ",
            points[from],
            points[to],
            dist(points[from], points[to]).isqrt()
        );
        connect_circuits(&mut result, &mut ix, from, to);
    }
    result
}

fn connect_circuits(circuits: &mut [Option<u32>], ix: &mut u32, from: usize, to: usize) {
    match (circuits[from], circuits[to]) {
        (None, None) => {
            debug_println!("new circuit {ix}");
            circuits[from] = Some(*ix);
            circuits[to] = Some(*ix);
            *ix += 1
        }
        (Some(c_from), Some(c_to)) => {
            if c_from != c_to {
                debug_println!("connecting circuit {c_from} to {c_to}");
                connect_two_circuits(circuits, c_from, c_to)
            } else {
                debug_println!("already connected in {c_from}");
            }
        }
        (Some(c_from), None) => {
            debug_println!("connecting from {c_from}");
            circuits[to] = Some(c_from);
        }
        (None, Some(c_to)) => {
            debug_println!("connecting to {c_to}");
            circuits[from] = Some(c_to)
        }
    }
}

fn connect_two_circuits(circuits: &mut [Option<u32>], c_from: u32, c_to: u32) {
    for c in circuits.iter_mut().filter(|p| **p == Some(c_to)) {
        *c = Some(c_from);
    }
}

fn n_largest_circuits_product(n: usize, circuits: Circuits) -> usize {
    let circuit_sizes = circuits.iter().flatten().counts();
    circuit_sizes.values().sorted().rev().take(n).product()
}

pub fn part_two(input: &str) -> Option<i64> {
    let points = parse(input);
    connect_all(&points)
}

fn closest_pairs(points: &[Point]) -> impl Iterator<Item = (usize, usize)> {
    sorted_pairs(0, points.len()).sorted_by_key(|&(l, r)| dist(points[l], points[r]))
}

fn connect_all(points: &[Point]) -> Option<i64> {
    let mut result = vec![None; points.len()];
    let mut ix = 1;
    for (from, to) in closest_pairs(points) {
        debug_print!(
            "Connecting {:?}\tto {:?}\t(dist {}) - ",
            points[from],
            points[to],
            dist(points[from], points[to]).isqrt()
        );
        connect_circuits(&mut result, &mut ix, from, to);
        if result.iter().all(|c| c.is_some()) {
            return Some(distance_from_wall(points, from, to));
        }
    }
    None
}

fn distance_from_wall(points: &[Point], from: usize, to: usize) -> i64 {
    let (x1, _, _) = points[from];
    let (x2, _, _) = points[to];
    x1 * x2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        debug_println!("TEST PART ONE");
        let result = part_one_iter(10, &advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        debug_println!("TEST PART TWO");
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(216 * 117));
    }
}
