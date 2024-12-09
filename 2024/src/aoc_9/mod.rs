use crate::utils::InputMode;
use std::collections::{HashMap, HashSet};
use std::fs;

const TEXT_INPUT: &str = r#"2333133121414131402"#;

const EXPECTED: &str = r#"
00...111...2...333.44.5555.6666.777.888899
009..111...2...333.44.5555.6666.777.88889.
0099.111...2...333.44.5555.6666.777.8888..
00998111...2...333.44.5555.6666.777.888...
009981118..2...333.44.5555.6666.777.88....
0099811188.2...333.44.5555.6666.777.8.....
009981118882...333.44.5555.6666.777.......
0099811188827..333.44.5555.6666.77........
00998111888277.333.44.5555.6666.7.........
009981118882777333.44.5555.6666...........
009981118882777333644.5555.666............
00998111888277733364465555.66.............
0099811188827773336446555566..............
"#;

const EXPECTED_2: &str = r#""#;

fn part1(data: String) -> usize {
    // (id, length)
    let mut blocks: Vec<(Option<usize>, usize)> = vec![];
    for (i, c) in data.trim().chars().enumerate() {
        let len = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            blocks.push((Some(i / 2), len));
        } else {
            blocks.push((None, len))
        }
    }

    let mut memory: Vec<Option<usize>> = blocks
        .iter()
        .flat_map(|block| {
            if let Some(idx) = block.0 {
                vec![Some(idx); block.1]
            } else {
                vec![None; block.1]
            }
        })
        .collect();

    // println!("blocks: {blocks:?}");
    // println!("memory: {memory:?}");
    // println!("memory len: {}", memory.len());

    // for value in &memory {
    //     print!("{}", value.map(|x| format!("{x}")).unwrap_or(".".into()));
    // }
    // println!("");

    let mut left_cursor = 0;
    let mut right_cursor = memory.len() - 1;

    while left_cursor < memory.len() && left_cursor < right_cursor {
        if memory[left_cursor].is_none() {
            while memory[right_cursor].is_none() {
                right_cursor -= 1;
            }
            memory.swap(left_cursor, right_cursor);
        }
        left_cursor += 1;
    }

    // for value in &memory {
    //     print!("{}", value.map(|x| format!("{x}")).unwrap_or(".".into()));
    // }
    // println!("");

    let result: usize = memory
        .iter()
        .enumerate()
        .map(|(i, x)| if let Some(x) = x { i * x } else { 0 })
        .sum();

    result
}

fn part2(data: String) -> usize {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_9/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
