use std::str::FromStr;
use itertools;
use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let moves = parse(input);
    Some(count_zeros(&moves))
}

fn parse(input: &str) -> Vec<i64> {
    input.split_whitespace()
        .map(|m| {
            let (dir, dist) = m.trim().split_at(1);
            parse_dir(dir) * i64::from_str(dist).unwrap()
        })
        .collect_vec()
}

fn parse_dir(input: &str) -> i64 {
    match input {
        "L" => -1,
        "R" => 1,
        _ => panic!("Unknown direction '{input}'"),
    }
}

fn count_zeros(moves: &[i64]) -> u64 {
    let mut dial = 50;
    let mut zeros = 0;
    for &m in moves {
        dial = move_dial(m, dial);
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn move_dial(m: i64, dial: i64) -> i64 {
    i64::rem_euclid(dial + m, 100)
}

pub fn part_two(input: &str) -> Option<i64> {
    let moves = parse(input);
    Some(count_zero_passes(&moves))
}

fn count_zero_passes(moves: &[i64]) -> i64 {
    let mut dial: i64 = 50;
    let mut zeros: i64 = 0;
    //println!("Start at {dial}");
    for &m in moves {
        let z = count_zero_pass(m, dial);
        zeros += z;
        dial = move_dial(m, dial);
        //println!("{m} moves to {dial}: {z}x");
    }
    zeros
}

fn count_zero_pass(m: i64, dial: i64) -> i64 {
    let left_pass = i64::from(m < 0 && dial != 0 && -m >= dial);
    let cycles = i64::abs((m + dial) / 100);
    left_pass + cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        // The dial starts by pointing at 50.
        // The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        assert_eq!(count_zero_pass(-68, 50), 1);
        // The dial is rotated L30 to point at 52.
        assert_eq!(count_zero_pass(-30, 82), 0);
        // The dial is rotated R48 to point at 0.
        assert_eq!(count_zero_pass(48, 52), 1);
        // The dial is rotated L5 to point at 95.
        assert_eq!(count_zero_pass(-5, 0), 0);
        // The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
        assert_eq!(count_zero_pass(60, 95), 1);
        // The dial is rotated L55 to point at 0.
        assert_eq!(count_zero_pass(-55, 55), 1);
        // The dial is rotated L1 to point at 99.
        assert_eq!(count_zero_pass(-1, 0), 0);
        // The dial is rotated L99 to point at 0.
        assert_eq!(count_zero_pass(-99, 99), 1);
        // The dial is rotated R14 to point at 14.
        assert_eq!(count_zero_pass(14, 0), 0);
        // The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
        assert_eq!(count_zero_pass(-82, 14), 1);

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_examples() {
        assert_eq!(count_zero_pass(1000, 50), 10);
        assert_eq!(count_zero_pass(-100, 50), 1);
        assert_eq!(count_zero_pass(-50, 50), 1);
        assert_eq!(count_zero_pass(-49, 50), 0);
        assert_eq!(count_zero_pass(-1, 0), 0);
    }
}
