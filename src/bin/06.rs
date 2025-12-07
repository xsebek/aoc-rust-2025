use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, newline, space0};
use nom::combinator::value;
use nom::multi::{many_till, many1};
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u128> {
    let (numbers, ops) = parse1(input);
    Some(
        ops.into_iter()
            .enumerate()
            .map(|(i, o)| {
                let ns = numbers.iter().map(|n| n[i]);
                match o {
                    Op::Add => ns.sum::<u128>(),
                    Op::Mul => ns.product(),
                }
            })
            .sum(),
    )
}

fn parse1(input: &str) -> (Vec<Vec<u128>>, Vec<Op>) {
    let num = delimited(space0, nom::character::complete::u128, space0);
    let op = delimited(space0, parse_op, space0);
    many_till(
        terminated(many1(num), newline),
        terminated(many1(op), newline),
    )
    .parse(input)
    .expect("must parse whole file")
    .1
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((value(Op::Add, tag("+")), value(Op::Mul, tag("*")))).parse(input)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Mul,
    Add,
}

pub fn part_two(input: &str) -> Option<u128> {
    //println!("{}", transpose(input));
    Some(
        parse2(&transpose(input))
            .into_iter()
            .map(|(ns, o)| match o {
                Op::Add => ns.iter().sum::<u128>(),
                Op::Mul => ns.iter().product(),
            })
            .sum(),
    )
}

fn transpose(input: &str) -> String {
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let longest = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut acc = Vec::with_capacity(input.len());
    for col in (0..longest).rev() {
        for row in lines.iter() {
            acc.push(row.get(col).copied().unwrap_or(' '))
        }
        acc.push('\n')
    }
    acc.into_iter().collect()
}

fn parse2(input: &str) -> Vec<(Vec<u128>, Op)> {
    let num = delimited(multispace0, nom::character::complete::u128, multispace0);
    let op = terminated(parse_op, multispace0);
    many1((many1(num), op))
        .parse(input)
        .expect("must parse whole file")
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
