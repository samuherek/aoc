use crate::utils::InputMode;
use std::collections::{HashMap, HashSet};
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

fn part2(data: String) -> u64 {
    let numbers = data
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut state = numbers
        .into_iter()
        .map(|x| (x, 1))
        .collect::<HashMap<u64, u64>>();

    for _ in 0..75 {
        let mut next_state = HashMap::new();

        for (num, count) in state.iter() {
            if *num == 0 {
                *next_state.entry(1).or_insert(0) += count;
                continue;
            }

            let digits = (*num as f64).log10().floor() as usize + 1;
            if digits % 2 == 0 {
                let half = digits / 2;
                let power = 10_u64.pow(half as u32);
                let one = num / power;
                let two = num % power;
                *next_state.entry(one).or_insert(0) += count;
                *next_state.entry(two).or_insert(0) += count;
                continue;
            }

            let next = num * 2024;
            *next_state.entry(next).or_insert(0) += count;
        }
        state = next_state;
    }

    state.iter().map(|(_, v)| v).sum()
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_11/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
