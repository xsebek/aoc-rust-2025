use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
    let ranges = parse(input);
    Some(ranges.into_iter().map(count_silly).sum())
}

fn parse(input: &str) -> Vec<Range> {
    input
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|r| {
            let (f, l) = r.trim().split_once('-').unwrap();
            Range {
                first: i64::from_str(f).unwrap(),
                last: i64::from_str(l).unwrap(),
            }
        })
        .collect_vec()
}

#[derive(Debug, Copy, Clone)]
struct Range {
    first: i64,
    last: i64,
}

impl Range {
    fn contains(self, value: i64) -> bool {
        self.first <= value && value <= self.last
    }
}

fn count_silly(range: Range) -> i64 {
    let mut result = 0;
    let mut count = 0;
    let first_digits = range.first.ilog10() + 1;
    let last_digits = range.last.ilog10() + 1;
    let mut ids = Vec::new();
    for ds in first_digits..=last_digits {
        if ds % 2 == 0 {
            let part_digits = ds / 2;
            let mut p = range.first / 10i64.pow(part_digits);
            let mut n = p * 10i64.pow(part_digits) + p;
            while n <= range.last {
                if range.contains(n) {
                    result += n;
                    count += 1;
                    ids.push(n);
                }
                p += 1;
                n = p * 10i64.pow(p.ilog10() + 1) + p;
            }
        }
    }
    println!(
        "{f}-{l} has {count} invalid IDs, {ids:?}",
        f = range.first,
        l = range.last
    );
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
