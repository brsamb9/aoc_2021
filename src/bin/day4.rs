use std::{fmt::Debug, fs};
#[derive(Debug)]

struct BingoBoard {
    // 5x5
    grid: [u32; 25],
    bingo: [bool; 25],
}
impl BingoBoard {
    fn new(grid_input: &str) -> Self {
        let mut grid: [u32; 25] = [0; 25];

        grid_input.lines().enumerate().for_each(|(i, row)| {
            row.split_whitespace()
                .enumerate()
                .for_each(|(j, n)| grid[(i * 5) + j] = n.parse().unwrap())
        });

        Self {
            grid,
            bingo: [false; 25],
        }
    }
    fn sum_of_unmarked(&self) -> u32 {
        let mut counter = 0;
        for (idx, b) in self.bingo.iter().enumerate() {
            if !(*b) {
                counter += self.grid[idx];
            }
        }
        counter
    }
}

#[derive(Debug)]
struct BingoPlayers {
    players: Vec<BingoBoard>,
}
impl BingoPlayers {
    fn mark(&mut self, number: u32) -> Option<Vec<usize>> {
        let mut winners = vec![];
        for (player_idx, player) in self.players.iter_mut().enumerate() {
            if let Some(idx) = player.grid.iter().position(|&x| x == number) {
                player.bingo[idx] = true;

                // row of idx
                let row_start = (idx / 5) * 5;
                let row_end = row_start + 5;
                let row_bingo = player.bingo[row_start..row_end].iter().all(|&b| b);

                let col_start = idx % 5;
                let col_end = player.bingo.len() - 5 + col_start;
                let col_bingo = (col_start..=col_end)
                    .step_by(5)
                    .map(|i| player.bingo[i])
                    .all(|b| b);

                if row_bingo || col_bingo {
                    winners.push(player_idx);
                }
            }
        }
        match winners.len() > 0 {
            true => Some(winners),
            false => None,
        }
    }
}

fn solve_day4_p1(bingo_pulls: &Vec<u32>, players: &mut BingoPlayers) -> u32 {
    // Play the game
    for &call in bingo_pulls {
        if let Some(winner_indexes) = players.mark(call) {
            // Winning board -> sum_of_unmarked * final_call
            if winner_indexes.len() > 1 {
                unimplemented!()
            }
            return players.players[winner_indexes[0]].sum_of_unmarked() * call;
        }
    }
    panic!("No winner found!");
}

fn solve_day4_p2(bingo_pulls: &Vec<u32>, players: &mut BingoPlayers) -> u32 {
    // Play the game
    for &call in bingo_pulls {
        if let Some(winner_indexes) = players.mark(call) {
            // Winning board -> sum_of_unmarked * final_call
            if players.players.len() == 1 {
                return players.players[winner_indexes[0]].sum_of_unmarked() * call;
            } else {
                for (i_arr, winner_index) in winner_indexes.iter().enumerate() {
                    players.players.remove(winner_index - i_arr);
                }
            }
        }
    }
    println!("{:?}", players.players.len());
    panic!("No winner found!");
}

fn main() {
    let file = fs::read_to_string("src/inputs/day4.txt").unwrap();

    let mut file_iterator = file.split("\n\n");

    let bingo_pulls: Vec<u32> = file_iterator
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut players1 = BingoPlayers {
        players: file_iterator
            .clone()
            .map(BingoBoard::new)
            .collect::<Vec<BingoBoard>>(),
    };

    println!("{}", solve_day4_p1(&bingo_pulls, &mut players1));

    let mut players2 = BingoPlayers {
        players: file_iterator
            .clone()
            .map(BingoBoard::new)
            .collect::<Vec<BingoBoard>>(),
    };

    println!("{}", solve_day4_p2(&bingo_pulls, &mut players2));
}
