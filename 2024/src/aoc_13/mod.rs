use crate::utils::InputMode;
use std::fs;

const TEXT_INPUT: &str = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

const EXPECTED: &str = r#"
"#;

#[derive(Debug, Default)]
struct Move(u64, u64);
impl Move {
    fn x(&self) -> u64 {
        self.0
    }
    fn y(&self) -> u64 {
        self.1
    }
}

#[derive(Debug, Default)]
struct Pos(u64, u64);
impl Pos {
    fn x(&self) -> u64 {
        self.0
    }
    fn y(&self) -> u64 {
        self.1
    }
}

#[derive(Debug, Default)]
struct Game {
    button_a: Move,
    button_b: Move,
    prize: Pos,
}

const A: u64 = 3;
const B: u64 = 1;

fn part1(data: String) -> u64 {
    let mut games = vec![];
    let mut game_buf = Game::default();

    for line in data.trim().lines() {
        if line.is_empty() {
            games.push(game_buf);
            game_buf = Game::default();
        } else if line.starts_with("Button A") {
            let (_, rest) = line.split_once(":").unwrap();
            let (x, y) = rest
                .trim()
                .split_once(",")
                .map(|(x, y)| {
                    let x = x.trim().strip_prefix("X+").unwrap();
                    let y = y.trim().strip_prefix("Y+").unwrap();
                    (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
                })
                .unwrap();
            game_buf.button_a = Move(x, y);
        } else if line.starts_with("Button B") {
            let (_, rest) = line.split_once(":").unwrap();
            let (x, y) = rest
                .trim()
                .split_once(",")
                .map(|(x, y)| {
                    let x = x.trim().strip_prefix("X+").unwrap();
                    let y = y.trim().strip_prefix("Y+").unwrap();
                    (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
                })
                .unwrap();
            game_buf.button_b = Move(x, y);
        } else if line.starts_with("Prize") {
            let (_, rest) = line.split_once(":").unwrap();
            let (x, y) = rest
                .trim()
                .split_once(",")
                .map(|(x, y)| {
                    let x = x.trim().strip_prefix("X=").unwrap();
                    let y = y.trim().strip_prefix("Y=").unwrap();
                    (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
                })
                .unwrap();
            game_buf.prize = Pos(x, y);
        }
    }

    println!("games: {games:?}");
    let mut total = 0;

    for game in games {
        let mut min_cost = u64::MAX;
        for a_count in 1..=100 {
            for b_count in 1..=100 {
                let end_x = a_count * game.button_a.x() + b_count * game.button_b.x();
                let end_y = a_count * game.button_a.y() + b_count * game.button_b.y();
                if end_x == game.prize.x() && end_y == game.prize.y() {
                    let res = a_count * A + b_count * B;
                    min_cost = min_cost.min(res);
                    println!("game won: a: {a_count} b: {b_count}");
                    println!("cost {res}");
                }
            }
        }

        if min_cost < u64::MAX {
            total += min_cost;
        }
    }

    total
}

fn part2(data: String) -> usize {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_13/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
