use advent_of_code::Range;
use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, numbers) = parse(input);
    // for r in ranges.iter() {
    //     println!("{}-{}", r.first, r.last)
    // }
    Some(
        numbers
            .into_iter()
            .filter(|&n| is_valid(&ranges, n))
            .count(),
    )
}

fn parse(input: &str) -> (Vec<Range>, Vec<i128>) {
    let mut lines = input.lines();
    let rs = lines
        .take_while_ref(|l| !l.is_empty())
        .map(|l| {
            let (first, last) = l.split_once('-').unwrap();
            Range {
                first: i128::from_str(first).unwrap(),
                last: i128::from_str(last).unwrap(),
            }
        })
        .collect_vec();
    lines.next();
    let numbers = lines.flat_map(i128::from_str).collect_vec();
    (normalize_ranges(&rs), numbers)
}

fn is_valid(ranges: &[Range], number: i128) -> bool {
    ranges
        .iter()
        .take_while(|r| r.first <= number)
        .any(|r| r.contains(number))
}

pub fn part_two(input: &str) -> Option<i128> {
    let (ranges, _) = parse(input);
    Some(ranges.iter().map(|r| r.last - r.first + 1).sum())
}

fn normalize_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut res: Vec<Range> = Vec::new();
    for &r in ranges.iter().sorted() {
        if let Some(prev) = res.last_mut()
            && r.first <= prev.last
        {
            if r.last <= prev.last {
                continue;
            }
            prev.last = r.last;
        } else {
            res.push(r)
        }
    }
    res
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
