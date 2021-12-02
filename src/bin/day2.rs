use std::fs;

pub fn gen_day1() -> Vec<Movement> {
    let file = fs::read_to_string("src/inputs/day2.txt").unwrap();
    file.lines().map(|x| Movement::from(x)).collect::<Vec<Movement>>()
}
#[derive(Debug)]
pub enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}
impl From<&str> for Movement {
    fn from(item: &str) -> Self {
        let items = item.split_whitespace().collect::<Vec<_>>();
        if items.len() != 2 {
            panic!("Oh - expected two substrs here.");
        }
        let mag: i32 = items[1].parse().unwrap();
        match items[0] {
            "forward" => Movement::Forward(mag),
            "up" => Movement::Up(mag),
            "down" => Movement::Down(mag),
            _ => panic!("Oh nooooo"),
        }
    }
}

#[derive(Debug)]
/// Lazy here, but depth, horizontal, aim
pub struct Location(i32, i32, i32);
impl Location {
    pub fn move_by(&mut self, direction: &Movement) {
        match direction {
            Movement::Forward(h) => self.1 += h,
            Movement::Up(d) => self.0 -= d,
            Movement::Down(d) => self.0 += d,
        }
    }
    pub fn aim_by(&mut self, direction: &Movement) {
        match direction {
            Movement::Forward(h) => {
                self.1 += h;
                self.0 += self.2 * h;
            },
            Movement::Up(a) => self.2 -= a,
            Movement::Down(a) => self.2 += a,
        }
    }
}

pub fn solve_day1_p1(data: &Vec<Movement>) -> i32 {
    let mut starting_position = Location(0, 0, 0);
    data.iter().for_each(|dir| starting_position.move_by(dir));
    starting_position.0 * starting_position.1
}

pub fn solve_day1_p2(data: &Vec<Movement>) -> i32 {
    let mut starting_position = Location(0, 0, 0);
    data.iter().for_each(|dir| starting_position.aim_by(dir));
    starting_position.0 * starting_position.1
}

fn main(){
    let data = gen_day1();
    println!(
        "P1: {}\nP2: {:?}",
        solve_day1_p1(&data),
        solve_day1_p2(&data)
    )
}

