use std::fs;

pub fn gen_day1() -> Vec<u64> {
    let file = fs::read_to_string("src/inputs/day1.txt").unwrap();
    file.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

/// Count number of times the depth measurement increases from the previous;
pub fn solve_day1_p1(data: &Vec<u64>) -> u64 {
    data.windows(2)
        .fold(0, |acc, window| acc + (window[1] > window[0]) as u64)
}

///
pub fn solve_day1_p2(data: &Vec<u64>) -> u64 {
    let summations: Vec<u64> = data.windows(3).map(|w| w.iter().sum()).collect();
    solve_day1_p1(&summations)
}

fn main() {
    let data = gen_day1();
    println!(
        "P1: {}\nP2: {:?}",
        solve_day1_p1(&data),
        solve_day1_p2(&data)
    )
}
