use std::cmp::max;
use std::iter::once;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::{IResult, Parser};
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use advent_of_code::{debug_println, sorted_pairs};
#[allow(unused_imports)]
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);
    sorted_pairs(0, points.len())
        .map(|(l, r)| area(points[l], points[r]))
        .max()
}
type Point = (i64, i64);

fn parse(input: &str) -> Vec<Point> {
    many1(terminated(parse_point, newline))
        .parse(input)
        .expect("must parse whole file")
        .1
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let num = nom::character::complete::i64;
    (num, preceded(tag(","), num)).parse(input)
}

fn area(x: Point, y: Point) -> u64 {
    fn d(xi: i64, yi: i64) -> u64 {
        (xi - yi).unsigned_abs() + 1
    }
    d(x.0, y.0) * d(x.1, y.1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);
    print_dot_graph(&points);
    print_candidates_above(&points);
    print_candidates_below(&points);
    // top half circle
    let p1: Point = (94693, 50233);
    let top_y = 69349;
    let largest_above = points.iter().copied()
        .filter(|r| p1.1 < r.1 && r.1 < top_y)
        .map(|r| area(p1, r))
        .max()
        .expect("one rectangle in top half circle");
    // bottom half circle
    let p2: Point = (94693, 48547);
    let bottom_y = 3238;
    let largest_below = points.iter().copied()
        .filter(|r| p2.1 > r.1 && r.1 > bottom_y)
        .map(|r| area(p1, r))
        .max()
        .expect("one rectangle in bottom half circle");
    Some(max(largest_above, largest_below))
}

#[allow(unused)]
fn print_dot_graph(points: &[Point]) {
    for p in points.iter() {
        debug_println!("P_{}_{} [pos=\"{},{}!\"]", p.0, p.1, p.0/100, p.1/100)
    }
    for (&l, &r) in points.iter().zip(points.iter().skip(1).chain(once(&points[0]))) {
        debug_println!("P_{}_{} -- P_{}_{}", l.0, l.1, r.0, r.1)
    }
}

#[allow(unused)]
fn print_candidates_above(points: &[Point]) {
    #[cfg(debug_assertions)] {
        debug_println!("CANDIDATES ABOVE:");
        let p: Point = (94693, 50233);
        let top_y = 69349;
        let largest_above = points.iter().copied()
            .filter(|r| p.1 < r.1 && r.1 < top_y)
            .k_largest_by_key(10, |r| area(p, *r));
        for r in largest_above {
            debug_println!("P_{}_{} distance {}", r.0, r.1, area(p, r))
        }
    }
}

#[allow(unused)]
fn print_candidates_below(points: &[Point]) {
    #[cfg(debug_assertions)] {
        debug_println!("CANDIDATES BELOW:");
        let p: Point = (94693,48547);
        let bottom_y = 3238;
        let largest_above = points.iter().copied()
            .filter(|r| p.1 > r.1 && r.1 > bottom_y)
            .k_largest_by_key(10, |r| area(p, *r));
        for r in largest_above {
            debug_println!("P_{}_{} distance {}", r.0, r.1, area(p, r))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    // PART TWO HAS SPECIAL INPUT SHAPE
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
