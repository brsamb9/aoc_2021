use std::{
    collections::HashMap,
    fs,
    iter::Rev,
    ops::{Range, RangeInclusive},
};
#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord(u16, u16);
impl Coord {
    fn new(s: &str) -> Self {
        let mut split_s = s.split(",");
        let x = split_s.next().unwrap().trim().parse().unwrap();
        let y = split_s.next().unwrap().trim().parse().unwrap();
        Self(x, y)
    }
}

fn generate_line(start_end_xy: &(Coord, Coord), ignore_diagonal_lines: bool) -> Vec<Coord> {
    // Just considering vertical / horizontal lines.
    let (xy1, xy2) = start_end_xy;

    if xy1.0 == xy2.0 {
        // Vertical Line
        let (min, max) = if xy1.1 > xy2.1 {
            (xy2.1, xy1.1)
        } else {
            (xy1.1, xy2.1)
        };
        (min..=max).map(|y| Coord(xy1.0, y)).collect()
    } else if xy1.1 == xy2.1 {
        // Horizontal Line
        let (min, max) = if xy1.0 > xy2.0 {
            (xy2.0, xy1.0)
        } else {
            (xy1.0, xy2.0)
        };
        (min..=max).map(|x| Coord(x, xy1.1)).collect()
    } else {
        match ignore_diagonal_lines {
            true => vec![],
            false => {
                // Would love to have the arguments accept iterators instead.
                let diag_line = |x: Vec<u16>, y: Vec<u16>| -> Vec<Coord> {
                    return x
                        .into_iter()
                        .zip(y.into_iter())
                        .map(|(i, j)| Coord(i, j))
                        .collect();
                };
                // Diagonals are given at exactly 45 degrees.
                if xy1.0 > xy2.0 {
                    if xy1.1 > xy2.1 {
                        // E.g. (10, 10) -> (1, 1)
                        diag_line((xy2.0..=xy1.0).collect(), (xy2.1..=xy1.1).collect())
                    } else {
                        // E.g. (10, 1) -> (1, 10)
                        diag_line((xy2.0..=xy1.0).rev().collect(), (xy1.1..=xy2.1).collect())
                    }
                } else {
                    if xy1.1 > xy2.1 {
                        // E.g. (1, 10) -> (10, 1)
                        diag_line((xy1.0..=xy2.0).collect(), (xy2.1..=xy1.1).rev().collect())
                    } else {
                        // E.g. (1, 1) -> (10, 10)
                        diag_line((xy1.0..=xy2.0).collect(), (xy1.1..=xy2.1).collect())
                    }
                }
            }
        }
    }
}

fn solve_day5_p1(datapoints: &Vec<(Coord, Coord)>, ignore_diagonal_lines: bool) -> usize {
    let mut sparse_matrix: HashMap<Coord, u16> = HashMap::new();

    datapoints.iter().for_each(|datapoint| {
        generate_line(datapoint, ignore_diagonal_lines)
            .into_iter()
            .for_each(|xy| {
                *sparse_matrix.entry(xy).or_insert(0) += 1;
            })
    });

    sparse_matrix
        .values()
        .into_iter()
        .filter(|&&x| x >= 2)
        .count()
}

fn main() {
    let file = fs::read_to_string("src/inputs/day5.txt").unwrap();
    let datapoints: Vec<(Coord, Coord)> = file
        .lines()
        .map(|line| {
            let mut data = line.split("->").map(Coord::new);
            (data.next().unwrap(), data.next().unwrap())
        })
        .collect();

    println!("p1: {:?}", solve_day5_p1(&datapoints, true));
    println!("p2: {:?}", solve_day5_p1(&datapoints, false));
}
