use std::fs;
#[derive(Debug, Clone, Copy)]
struct BinaryCounter(u16, u16);
impl BinaryCounter {
    fn increment(&mut self, b: char) {
        match b {
            '0' => self.0 += 1,
            '1' => self.1 += 1,
            _ => panic!("Oh.."),
        }
    }
    fn most_common(&self) -> char {
        if self.0 == self.1 {
            return '_';
        }
        if self.0 > self.1 {
            '0'
        } else {
            '1'
        }
    }
    fn least_common(&self) -> char {
        if self.0 > self.1 {
            '1'
        } else {
            '0'
        }
    }
}

type ParameterCounter = [BinaryCounter; 12];

fn solve_day3_p1(data: &ParameterCounter) -> u32 {
    let most_common = data.iter().map(|p| p.most_common()).collect::<String>();
    let gamma_rate = u32::from_str_radix(&most_common, 2).unwrap();

    // Could inverse string but meh
    let least_common = data.iter().map(|p| p.least_common()).collect::<String>();
    let epsilion_rate = u32::from_str_radix(&least_common, 2).unwrap();

    gamma_rate * epsilion_rate
}

fn grab_last_value(
    starting_common_bit: char,
    lines: &Vec<&str>,
    criterion: impl Fn(BinaryCounter) -> char,
) -> u32 {
    let mut cloned_lines: Vec<Vec<char>> = lines
        .clone()
        .into_iter()
        .filter(|&line| line.chars().nth(0).unwrap() == starting_common_bit)
        .map(|s| s.chars().collect())
        .collect();

    let mut bit_idx = 1;
    loop {
        let mut next_counter = BinaryCounter(0, 0);
        cloned_lines
            .iter()
            .for_each(|chars_vec| next_counter.increment(chars_vec[bit_idx]));
        let next_common_bit = criterion(next_counter);

        cloned_lines = cloned_lines
            .into_iter()
            .filter(|chars_vec| chars_vec[bit_idx] == next_common_bit)
            .collect::<Vec<Vec<char>>>();

        bit_idx += 1;

        if cloned_lines.len() == 1 {
            return u32::from_str_radix(
                cloned_lines[0]
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .as_str(),
                2,
            )
            .unwrap();
        }
    }
}

fn solve_day3_p2(file: &String, data: &ParameterCounter) -> u32 {
    let lines = file.lines().collect::<Vec<&str>>();

    let oxygen_closure = |c: BinaryCounter| {
        if c.0 == c.1 {
            '1'
        } else if c.0 > c.1 {
            '0'
        } else {
            '1'
        }
    };
    let oxygen = grab_last_value(data[0].most_common(), &lines, oxygen_closure);
    let c02_criterion = |c: BinaryCounter| {
        if c.0 == c.1 {
            '0'
        } else if c.0 < c.1 {
            '0'
        } else {
            '1'
        }
    };
    let c02 = grab_last_value(data[0].least_common(), &lines, c02_criterion);

    oxygen * c02
}

fn main() {
    let file = fs::read_to_string("src/inputs/day3.txt").unwrap();

    let mut counter = [BinaryCounter(0, 0); 12];
    file.lines().for_each(|s| {
        s.chars()
            .enumerate()
            .for_each(|(bit_idx, b)| counter[bit_idx].increment(b))
    });

    println!("{:?}", solve_day3_p1(&counter));
    println!("{:?}", solve_day3_p2(&file, &counter));
}
