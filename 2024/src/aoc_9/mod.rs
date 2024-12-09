use crate::utils::InputMode;
use std::collections::VecDeque;
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

const EXPECTED_2: &str = r#"
00...111...2...333.44.5555.6666.777.888899
0099.111...2...333.44.5555.6666.777.8888..
0099.1117772...333.44.5555.6666.....8888..
0099.111777244.333....5555.6666.....8888..
00992111777.44.333....5555.6666.....8888..
"#;

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

#[derive(Debug)]
enum Block {
    File {
        index: usize,
        len: usize,
        pos: usize,
    },
    Space {
        len: usize,
        pos: usize,
    },
}

impl Block {
    fn new_file(index: usize, len: usize, position: usize) -> Self {
        Self::File {
            index,
            len,
            pos: position,
        }
    }

    fn new_space(len: usize, position: usize) -> Self {
        Self::Space { len, pos: position }
    }

    fn len(&self) -> usize {
        match self {
            Block::File { len, .. } => *len,
            Block::Space { len, .. } => *len,
        }
    }

    fn pos(&self) -> usize {
        match self {
            Block::File { pos, .. } => *pos,
            Block::Space { pos, .. } => *pos,
        }
    }

    fn is_space(&self) -> bool {
        matches!(self, Block::Space { .. })
    }
}

fn part2(data: String) -> usize {
    // (id, length)
    let mut blocks = vec![];
    let mut cursor = 0;
    for (i, c) in data.trim().chars().enumerate() {
        let len = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            blocks.push(Block::new_file(i / 2, len, cursor));
        } else {
            blocks.push(Block::new_space(len, cursor));
        }
        cursor += len;
    }

    let mut memory: Vec<Option<usize>> = vec![None; cursor];

    while let Some(last) = blocks.pop() {
        match last {
            Block::File { index, len, pos } => {
                let possible_space_cursor =
                    blocks.iter().position(|x| x.is_space() && len <= x.len());
                if let Some(cursor) = possible_space_cursor {
                    let space = &blocks[cursor];
                    let space_len = space.len();
                    let space_pos = space.pos();

                    blocks.splice(
                        cursor..cursor + 1,
                        vec![Block::new_file(index, len, space_pos)],
                    );

                    if len < space_len {
                        blocks.insert(
                            cursor + 1,
                            Block::new_space(space_len - len, space_pos + len),
                        );
                    }
                } else {
                    if memory[pos].is_none() {
                        for i in 0..len {
                            memory[pos + i] = Some(index);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // println!("{EXPECTED_2}");

    for value in &memory {
        print!("{}", value.map(|x| format!("{x}")).unwrap_or(".".into()));
    }
    println!("");

    let result: usize = memory
        .iter()
        .enumerate()
        .map(|(i, x)| if let Some(x) = x { i * x } else { 0 })
        .sum();

    result
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_9/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
