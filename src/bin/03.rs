use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let batteries = parse(input);
    Some(batteries
        .iter()
        .map(|b| joltage(b))
        .sum())
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|l| {
            l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
        })
        .collect_vec()
}

fn joltage(battery: &[u32]) -> u32 {
    let (b1pos, b1) = joltage_part(&battery[..battery.len()-1]);
    let &b2 = battery.iter().dropping(b1pos + 1).max().unwrap();
    b1 * 10 + b2
}

pub fn part_two(input: &str) -> Option<u64> {
    let batteries = parse(input);
    Some(batteries
        .iter()
        .map(|b| joltage12(b))
        .sum())
}

fn joltage12(battery: &[u32]) -> u64 {
    joltage_n(battery, 12, 0)
}

fn joltage_n(battery: &[u32], n: usize, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        let choice_len = battery.len()-n;
        let (index, jolt) = joltage_part(&battery[..=choice_len]);
        joltage_n(&battery[index+1..], n - 1, acc * 10 + u64::from(jolt))
    }
}

fn joltage_part(battery: &[u32]) -> (usize, u32) {
    debug_assert!(battery.len() > 0);
    let b1rev = battery.into_iter().rev().position_max().unwrap();
    let b1pos = battery.len() - b1rev - 1;
    (b1pos, battery[b1pos])
}

// fn joltage_generic(battery: &[u32], take: usize, accum: i64) -> i64 {
//     if take == 0 {
//         accum
//     } else if battery.len() < take {
//         i64::MIN
//     } else {
//         max(joltage_generic(&battery[1..], take - 1, accum * 10 + i64::from(battery[0])),
//             joltage_generic(&battery[1..], take, accum))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("987654321111111"), Some(98));
        assert_eq!(part_one("811111111111119"), Some(89));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("987654321111111"), Some(987654321111));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
