use advent_of_code::debug_println;
use itertools::Itertools;
use petgraph::algo::{all_simple_paths, has_path_connecting};
use petgraph::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::RandomState;
use petgraph::visit::{IntoNeighbors, Reversed, Walker};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let cables = parse(input);
    debug_println!("Basic DOT format:\n{:?}\n", petgraph::dot::Dot::new(&cables));
    Some(all_simple_paths::<Vec<&str>, &CableMap, RandomState>(&cables, "you", "out", 1, None)
        .count())
}

type CableMap<'a> = DiGraphMap<&'a str, ()>;

fn parse(input: &'_ str) -> CableMap<'_> {
    CableMap::from_edges(input.lines().flat_map(parse_line))
}

fn parse_line(line: &str) -> impl IntoIterator<Item=(&str, &str)> {
    let (n, edges) = line.split_once(':').expect("NODE: EDGES");
    edges.split_whitespace()
        .filter(|s| !s.trim().is_empty())
        .map(move |e| (n, e))
}

pub fn part_two(input: &str) -> Option<usize> {
    let cables = parse(input);
    debug_println!("Basic DOT format:\n{:?}\n", petgraph::dot::Dot::new(&cables));
    let cables = filter_dac_fft_paths(cables);
    debug_println!("Basic DOT format:\n{:?}\n", petgraph::dot::Dot::new(&cables));
    Some(path_count(&cables))
}

fn filter_dac_fft_paths(mut cables: CableMap) -> CableMap {
    // from svr (the server rack)
    // that visit dac (a digital-to-analog converter) and fft (fast Fourier transform)
    // to out
    assert!(has_path_connecting(&cables, "svr", "dac", None), "svr --> dac");
    assert!(has_path_connecting(&cables, "svr", "fft", None), "svr --> fft");
    assert!(has_path_connecting(&cables, "dac", "out", None), "dac --> out");
    assert_ne!(has_path_connecting(&cables, "dac","fft", None),
               has_path_connecting(&cables, "fft","dac", None),
               "dac <-/-> fft");
    debug_println!("check");

    if has_path_connecting(&cables, "dac", "fft", None) {
        debug_println!("svr --> dac --> fft --> out");
        filter_to_path(&mut cables, "svr", "dac");
        filter_to_path(&mut cables, "dac", "fft");
        filter_to_path(&mut cables, "fft", "out");
    }
    else {
        debug_println!("svr --> fft --> dac --> out");
        filter_to_path(&mut cables, "svr", "fft");
        filter_to_path(&mut cables, "fft", "dac");
        filter_to_path(&mut cables, "dac", "out");
    }
    cables
}

fn filter_to_path<'a>(cables: &mut CableMap<'a>, from_key: &'static str, to_key: &'static str) {
    let path = path(cables, from_key, to_key);
    for &node in path.iter() {
        if node == to_key {
            continue
        }
        for edge in cables.neighbors(node).collect_vec() {
            if !path.contains(edge) {
                cables.remove_edge(node, edge);
            }
        }
    }
}

fn path<'a>(cables: &CableMap<'a>, from_key: &'static str, to_key: &'static str) -> HashSet<&'a str> {
    let r = Reversed(cables);
    let from: HashSet<&'a str> = Dfs::new(cables, from_key).iter(cables).collect();
    let to: HashSet<&'a str> = Dfs::new(r, to_key).iter(r).collect();
    intersection(&from, &to)
}

fn path_count(cables: &CableMap) -> usize {
    debug_println!("path count");
    let mut result: HashMap<&str, usize> = HashMap::from([("svr", 1)]);
    let mut queue: VecDeque<&str> = VecDeque::from_iter(cables.neighbors("svr"));
    let mut queue_s: HashSet<&str> = HashSet::from_iter(queue.iter().copied());
    let mut i: usize = 0;
    while let Some(node) = queue.pop_front() {
        queue_s.remove(node);
        i += 1;
        if i > cables.node_count().pow(2) {
            panic!("Cycle in graph!?")
        }
        let parents_sum = Reversed(cables)
            .neighbors(node)
            .flat_map(|p| result.get(p).copied())
            .sum();
        let original = result.insert(node, parents_sum);
        if Some(parents_sum) != original {
            queue.extend(cables.neighbors(node).filter(|n| !queue_s.contains(n)))
        }
    }
    result["out"]
}

fn intersection<'a>(lhs: &HashSet<&'a str>, rhs: &HashSet<&'a str>) -> HashSet<&'a str> {
    lhs.intersection(rhs).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
