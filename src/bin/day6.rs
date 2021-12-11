use std::fs;

struct LifeCycle {
    lanternfishes: [usize; 9],
}
impl LifeCycle {
    fn new(initial_lifecycle: Vec<u8>) -> Self {
        let mut lanternfishes = [0; 9];
        initial_lifecycle
            .into_iter()
            .for_each(|x| lanternfishes[(x as usize)] += 1);
        Self { lanternfishes }
    }

    fn simulate_day(&mut self) {
        // Returns indices of lantern fishes that needs a reset
        let next_fishes = self.lanternfishes[0];
        self.lanternfishes[0] = 0;

        let mut next_lifecyle = [0; 9];
        for (idx, fishes) in self.lanternfishes[1..].iter_mut().enumerate() {
            next_lifecyle[idx] = *fishes;
            *fishes = 0;
        }

        self.lanternfishes = next_lifecyle;
        self.lanternfishes[6] += next_fishes;
        self.lanternfishes[8] += next_fishes;
    }
}

fn solve_day6(initial: Vec<u8>, to_day: u16) -> usize {
    let mut simulation = LifeCycle::new(initial);
    (0..to_day).for_each(|_| simulation.simulate_day());
    simulation.lanternfishes.iter().sum()
}

fn main() {
    let file = fs::read_to_string("src/inputs/day6.txt").unwrap();
    let initial_lifecycle = file
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u8>>();

    println!("{}", solve_day6(initial_lifecycle.clone(), 80));
    println!("{}", solve_day6(initial_lifecycle.clone(), 256));
}
