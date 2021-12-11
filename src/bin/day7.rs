use ndarray::{Array1, Axis};
use ndarray_stats::{interpolate::Midpoint, QuantileExt};
use noisy_float::prelude::n64;
use std::fs;

fn solve_day7_p1(crab_positions: Array1<u32>) -> u32 {
    // Oh, the median actually worked :D
    let mut crab_positions = crab_positions;
    let median = crab_positions
        .quantile_axis_mut(Axis(0), n64(0.5), &Midpoint)
        .unwrap()
        .first()
        .unwrap()
        .clone();

    let cost = crab_positions
        .map(|&v| if v < median { median - v } else { v - median })
        .sum();
    cost
}

fn solve_day7_p2(crab_positions: Array1<u32>) -> u32 {
    let mean = crab_positions.mean().unwrap();
    let cost = crab_positions
        .map(|&v| {
            let diff = if v < mean { mean - v } else { v - mean };
            (1..=diff).sum()
        })
        .sum();
    cost
}

fn main() {
    let crab_positions: Array1<u32> = fs::read_to_string("src/inputs/day7.txt")
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let ans1 = solve_day7_p1(crab_positions.clone());
    let ans2 = solve_day7_p2(crab_positions.clone());
    println!("Ans1: {}; Ans2: {}", ans1, ans2);
}
