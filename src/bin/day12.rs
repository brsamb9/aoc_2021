use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct CaveStructure {
    edges: HashMap<String, Vec<String>>,
}
impl CaveStructure {
    fn new(edges: Vec<(String, String)>) -> Self {
        let mut cave_edges = HashMap::new();
        edges.into_iter().for_each(|(from, to)| {
            // Bidirectional
            cave_edges
                .entry(from.clone())
                .or_insert(vec![])
                .push(to.clone());
            cave_edges.entry(to).or_insert(vec![]).push(from);
        });
        Self { edges: cave_edges }
    }

    fn find_paths_to_end<F: Fn(&str, &[String]) -> bool + Copy>(
        &self,
        name: &str,
        mut curr_path: Vec<String>,
        small_cave_limitation: F,
    ) -> usize {
        if name == "end" {
            return 1;
        }
        if !curr_path.is_empty() && name == "start" {
            return 0;
        }
        if !curr_path.is_empty()
            && is_small_cave(name)
            && small_cave_limitation(name, curr_path.as_slice())
        {
            return 0;
        }

        // dbg!(curr_path.clone());
        curr_path.push(name.to_owned());

        let children = self.edges.get(name).unwrap();
        children
            .iter()
            .map(|name| self.find_paths_to_end(name, curr_path.clone(), small_cave_limitation))
            .sum::<usize>()
    }
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(char::is_lowercase)
}

fn main() {
    let input = std::fs::read_to_string("src/inputs/day12.txt").unwrap();
    let edges: Vec<(String, String)> = input
        .lines()
        .map(|l| l.splitn(2, "-").map(String::from).collect_tuple().unwrap())
        .collect();

    let cave = CaveStructure::new(edges);
    let part1 =
        cave.find_paths_to_end("start", vec![], |name, arr| arr.contains(&name.to_string()));
    dbg!(part1);
    let part2 = cave.find_paths_to_end("start", vec![], |name, arr| {
        if !arr.contains(&name.to_string()) {
            return false;
        }
        let mut a: HashMap<&str, usize> = HashMap::new();
        for sv in arr
            .iter()
            .filter(|s| is_small_cave(s.as_str()) && s.as_str() != "start")
        {
            *a.entry(sv.as_str()).or_insert(0) += 1;
        }
        // dbg!(a.clone());
        let keys_has_dup = a
            .iter()
            .filter(|(k, v)| **v >= 2)
            .map(|(k, _v)| k)
            .collect::<Vec<&&str>>();

        !keys_has_dup.is_empty() && (keys_has_dup.contains(&&name) || a.get(name).is_some())
    });
    dbg!(part2);
}
