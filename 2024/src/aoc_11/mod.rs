use crate::utils::InputMode;
use std::boxed::Box;
use std::fs;

const TEXT_INPUT: &str = r#"
125 17
"#;

const EXPECTED: &str = r#"
"#;

const EXPECTED_2: &str = r#"
"#;

fn part1(data: String) -> usize {
    let numbers = data
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut first = numbers.clone();
    let mut second = vec![];

    for _ in 0..25 {
        for num in first.iter() {
            if num == &0 {
                second.push(1);
                continue;
            }
            let text = format!("{num}");
            if text.len() % 2 == 0 {
                let (one, two) = text.split_at(text.len() / 2);
                second.push(one.parse().unwrap());
                second.push(two.parse().unwrap());
            } else {
                second.push(num * 2024);
            }
        }
        std::mem::swap(&mut first, &mut second);
        second.clear();
    }

    first.len()
}

fn part2(data: String) -> usize {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_11/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
