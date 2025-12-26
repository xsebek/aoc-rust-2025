use std::collections::HashSet;
use itertools::{iterate, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, space1};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::multi::{fold_many1, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use z3::ast::Int;
use z3::{Optimize, SatResult};
use advent_of_code::debug_println;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse(input);
    Some(machines.iter()
        .map(|m| {
            let buttons = buttons_to_lights(&m.buttons);
            button_presses(&buttons)
                //.take(buttons.len() + 1)
                .position(|l| l.contains(&m.lights))
                .expect("button presses should eventually lead to light configuration")
        })
        .sum())
}

struct Machine {
    lights: u32,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

fn parse(input: &str) -> Vec<Machine> {
    many1(terminated(parse_machine, newline))
        .parse(input)
        .expect("must parse whole file")
        .1
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    (parse_lights, preceded(space1, parse_wiring), preceded(space1, parse_joltage))
        .map(|(lights, wiring, joltage)| Machine {lights, buttons: wiring, joltage})
        .parse(input)
}

fn parse_lights(input: &str) -> IResult<&str, u32> {
    fn l(i: &str) -> IResult<&str, u32> {
        fold_many1(
            alt((char('#'), char('.'))),
            || (0u8, 0),
            |(i, res), item: char| {
                let l = match item { '#' => 1, _ => 0, };
                (i + 1, res + (l << i))
            }
        ).map(|r| r.1).parse(i)
    }
    delimited(tag("["), l, tag("]")).parse(input)
}

fn parse_wiring(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(
        space1,
        delimited(char('('), separated_list1(char(','), nom::character::complete::usize), char(')'))
    ).parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(char('{'), separated_list1(char(','), nom::character::complete::u32), char('}'))
        .parse(input)
}

fn button_to_lights(button: &[usize]) -> u32 {
    button.iter().map(|&p| 1 << p).sum()
}

fn buttons_to_lights(buttons: &[Vec<usize>]) -> Vec<u32> {
    buttons.iter().map(|b| button_to_lights(b)).collect_vec()
}

fn press_buttons(buttons: &[u32], lights: &HashSet<u32>) -> HashSet<u32> {
    lights.iter()
        .flat_map(|&l| buttons.iter().map(move |&b| l ^ b))
        .collect()
}

fn button_presses(buttons: &[u32]) -> impl Iterator<Item=HashSet<u32>> {
    iterate(HashSet::from([0]), |l| press_buttons(buttons, l))
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);
    Some(machines.iter().map(solve_joltages).sum())
}

fn solve_joltages(machine: &Machine) -> u64 {
    let optimizer = Optimize::new();
    let buttons = machine.buttons.iter().map(|_| Int::fresh_const("b")).collect_vec();
    for button in buttons.iter() {
        optimizer.assert(&button.ge(0));
    }
    for (j, required_jolts) in machine.joltage.iter().copied().enumerate() {
        let sum = buttons.iter()
            .zip(machine.buttons.iter())
            .filter(|(_button_presses, button_jolts)| button_jolts.contains(&j))
            .map(|(button_presses, _button_jolts)| button_presses)
            .sum::<Int>();
        optimizer.assert(&sum.eq(required_jolts))
    }
    optimizer.minimize(&buttons.iter().sum::<Int>());

    debug_println!();
    debug_println!("Machine: {:?} {:?}", machine.buttons, machine.joltage);
    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model();
            model.map(|m| {
                let solution = buttons.iter().map(|b| m.get_const_interp(b)).collect_vec();
                let presses: u64 = solution.iter().map(|b| b.as_ref().and_then(Int::as_u64).unwrap_or(0)).sum();
                debug_println!("Solution: {presses:?} {solution:?}");
                presses
            })
        },
        _ => None,
    }.expect("Machine joltage should be solvable")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (rest, lights) = parse_lights("[.##.]").expect("lights parse");
        assert_eq!(rest.len(), 0);
        assert_eq!(lights, 6);
        let (rest, buttons) = parse_wiring("(3) (1,3) (2) (2,3) (0,2) (0,1)").expect("buttons parse");
        assert_eq!(rest.len(), 0);
        assert_eq!(buttons[0], vec![3]);
        assert_eq!(buttons[1], vec![1,3]);
        let buttons = buttons_to_lights(&buttons);
        assert_eq!(buttons, vec![8, 10, 4, 12, 5, 3]);
        let expected = HashSet::from_iter(buttons.iter().copied());
        let lights_off = HashSet::from([0]);
        assert_eq!(press_buttons(&buttons, &lights_off), expected);
        assert_eq!(button_presses(&buttons).position(|l| l.contains(&lights)), Some(2));
        // -- second example
        let (rest, lights) = parse_lights("[...#.]").expect("lights parse");
        assert_eq!(rest.len(), 0);
        assert_eq!(lights, 8); // ah, index from left
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
