use advent_of_code::{Range, debug_println};
use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i128> {
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
                first: i128::from_str(f).unwrap(),
                last: i128::from_str(l).unwrap(),
            }
        })
        .collect_vec()
}

fn count_silly(range: Range) -> i128 {
    count_silly_generic(2, range).into_iter().sum()
}

fn repeat(digits: i128, times: u32) -> i128 {
    let mut result = digits;
    let exp = 10i128.pow(digits.ilog10() + 1);
    for _ in 1..times {
        result = result * exp + digits
    }
    result
}

pub fn part_two(input: &str) -> Option<i128> {
    let ranges = parse(input);
    Some(ranges.into_iter().map(count_more_silly).sum())
}

fn count_more_silly(range: Range) -> i128 {
    let max = range.last.ilog10() + 1;
    (2..=max)
        .flat_map(|d| count_silly_generic(d, range))
        .unique()
        .sum()
}

fn count_silly_generic(repeats: u32, range: Range) -> Vec<i128> {
    let first_digits = range.first.ilog10() + 1;
    let last_digits = range.last.ilog10() + 1;
    let mut ids = Vec::new();
    for ds in first_digits..=last_digits {
        if ds % repeats == 0 {
            let part_digits = ds / repeats;
            let mut p = if ds == first_digits {
                take_first_digits(part_digits, range.first)
            } else {
                10i128.pow(part_digits - 1)
            };
            let mut n = repeat(p, repeats);
            while n <= range.last {
                if range.contains(n) {
                    ids.push(n);
                }
                p += 1;
                n = repeat(p, repeats);
            }
            debug_println!(
                "{ds}/{repeats}: {f}-{l} has {count} invalid IDs, {ids:?}",
                count = ids.len(),
                f = range.first,
                l = range.last,
            );
        }
    }
    ids
}

fn take_first_digits(digits: u32, number: i128) -> i128 {
    number / 10i128.pow(number.ilog10() + 1 - digits)
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
        // 95-115 now has two invalid IDs, 99 and 111.
        assert_eq!(
            count_silly_generic(
                2,
                Range {
                    first: 95,
                    last: 115
                }
            ),
            vec![99]
        );
        assert_eq!(
            count_silly_generic(
                3,
                Range {
                    first: 95,
                    last: 115
                }
            ),
            vec![111]
        );

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_helpers() {
        assert_eq!(take_first_digits(2, 21), 21);
        assert_eq!(take_first_digits(2, 1234), 12);
        assert_eq!(take_first_digits(1, 609), 6);
    }
}
