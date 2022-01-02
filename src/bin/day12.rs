use std::{collections::HashMap, hash::Hash};

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

    fn find_paths_to_end<'a>(
        &self,
        name: &str,
        mut curr_path: Vec<String>,
        small_cave_limitation: usize,
    ) -> usize {
        println!(
            "{} {:?} {}",
            name,
            curr_path,
            curr_path.iter().contains(&name.to_owned())
        );
        match name {
            "end" => 1,
            "start" if !curr_path.is_empty() => 0,
            _ if is_small_cave(name)
                && curr_path.iter().filter(|n| n.as_str() == name).count()
                    == small_cave_limitation =>
            {
                0
            }
            _ => {
                let children = self.edges.get(name).unwrap();
                curr_path.push(name.to_owned());

                children
                    .iter()
                    .map(|name| {
                        self.find_paths_to_end(name, curr_path.clone(), small_cave_limitation)
                    })
                    .sum::<usize>()
            }
        }
    }
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_lowercase())
}

fn main() {
    let input = std::fs::read_to_string("src/inputs/day12.txt").unwrap();
    let edges: Vec<(String, String)> = input
        .lines()
        .map(|l| l.splitn(2, "-").map(String::from).collect_tuple().unwrap())
        .collect();

    let cave = CaveStructure::new(edges);
    let part1 = cave.find_paths_to_end("start", vec![], 1);
    dbg!(part1);
    // let part2 = cave.find_paths_to_end("start", vec![], 2);
    // dbg!(part2);
}
