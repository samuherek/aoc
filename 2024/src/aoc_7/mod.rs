use crate::utils::InputMode;
use std::collections::HashSet;
use std::fs;

const TEXT_INPUT: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

enum Exp {
    Add,
    Multiply,
}

impl Exp {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Exp::Add => left + right,
            Exp::Multiply => left * right,
        }
    }
}

// 29, 23, 12
// 29 + [23, 12]
// 29 * [23, 12]
//
// 29 + [23 + 12]
// 29 + [23 * 12]
//
// 29 * [23 + 12]
// 29 * [23 * 12]

fn is_valid(nums: &[u64], target: u64, acc: u64) -> bool {
    if nums.is_empty() {
        return target == acc;
    }

    if acc > target {
        return false;
    }

    if is_valid(&nums[1..], target, acc + nums[0]) {
        return true;
    }

    if is_valid(&nums[1..], target, acc * nums[0]) {
        return true;
    }

    return false;
}

fn part1(data: String) -> u64 {
    let equations = data
        .trim()
        .lines()
        .map(|line| {
            let (result, numbers) = line.split_once(':').unwrap();
            let result = result.parse::<u64>().unwrap();
            let numbers: Vec<_> = numbers
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            (result, numbers)
        })
        .collect::<Vec<_>>();

    let mut result = 0;

    for (eq, nums) in equations {
        if is_valid(&nums, eq, 0) {
            result += eq;
        }
    }

    result
}

fn part2(data: String) -> u32 {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_7/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
