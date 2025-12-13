use itertools::Itertools;
use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use advent_of_code::{debug_print, debug_println};

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
    (a.0 - b.0).pow(2)
        + (a.1 - b.1).pow(2)
        + (a.2 - b.2).pow(2)
}

fn closest(points: &[Point]) -> Vec<(usize, Dist)> {
    points
        .iter()
        .enumerate()
        .map(|(i, &p)| closest_point(i, p, points))
        .collect_vec()
}

fn closest_point(index: usize, point: Point, points: &[Point]) -> (usize, Dist) {
    points.iter()
        .map(|&p| dist(point, p))
        .enumerate()
        .filter(|&(i, _)| i != index)
        .min_by(|p1, p2| p1.1.cmp(&p2.1))
        .expect("Expected nonempty points")
}

fn closest_pairs(points: &[Point]) -> impl Iterator<Item=(usize, usize)> {
    closest(points).into_iter()
        .enumerate()
        .sorted_by_key(|p| p.1.1)
        .map(|(from, (to, _))| (from, to))
}

fn connect_closest(iterations: usize, points: &[Point]) -> Circuits {
    let mut result = vec![None; points.len()];
    let mut ix = 1;
    let mut n = 0;
    for (from, to) in closest_pairs(points) {
        debug_print!("Connecting {:?} to {:?} - ", points[from], points[to]);
        let already_connected = connect_circuits(&mut result, &mut ix, from, to);
        n += usize::from(!already_connected);
        if n >= iterations {
            break
        }
    }
    result
}

fn connect_circuits(circuits: &mut [Option<u32>], ix: &mut u32, from: usize, to: usize) -> bool {
    match (circuits[from], circuits[to]) {
        (None, None) => {
            debug_println!("new circuit {ix}");
            circuits[from] = Some(*ix);
            circuits[to] =  Some(*ix);
            *ix += 1
        }
        (Some(c_from), Some(c_to)) => if c_from == c_to {
            debug_println!("already connected");
            return true
        } else {
            debug_println!("connecting circuit {c_from} to {c_to}");
            connect_two_circuits(circuits, c_from, c_to)
        },
        (Some(c_from), None) => {
            debug_println!("connecting from {c_from}");
            circuits[to] = Some(c_from);
        }
        (None, Some(c_to)) => {
            debug_println!("connecting to {c_to}");
            circuits[from] = Some(c_to)
        }
    }
    false
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

pub fn part_two(_: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_helpers() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));
        let closest_points = closest(&points);
        let last = points.len()-1;

        let d0 = dist((162,817,812), (425,690,689));
        let d1 = dist((162,817,812), (431,825,988));
        let d2 = dist((906,360,560), (805,96,715));
        let d3 = dist((431,825,988), (425,690,689));
        let d4 = dist((162,817,812), (431,825,988));

        assert_eq!(closest_points.len(), points.len());
        assert_eq!(closest_points[0].0, last);
        assert_eq!(closest_points[last].0, 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one_iter(10, &advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
