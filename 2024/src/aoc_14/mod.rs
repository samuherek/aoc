use crate::utils::InputMode;
use std::fs;

const TEST_1: &str = r#"
p=2,4 v=2,-3
"#;

const TEXT_INPUT: &str = r#"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

const EXPECTED_START: &str = r#"
1.12.......
...........
...........
......11.11
1.1........
.........1.
.......1...
"#;

const EXPECTED_END: &str = r#"
......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1....
"#;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    step: (i32, i32),
}

struct Room {
    width: i32,
    height: i32,
}

// p=0,4 v=3,-3
fn parse(data: &str) -> Vec<Robot> {
    data.trim()
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let pos = p[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();
            let step = v[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();
            Robot { pos, step }
        })
        .collect()
}

fn part1(data: String) -> u64 {
    let mut robots = parse(&data);
    let room = Room {
        width: 101,
        height: 103,
    };

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            let x = (robot.pos.0 + robot.step.0).rem_euclid(room.width);
            let y = (robot.pos.1 + robot.step.1).rem_euclid(room.height);
            robot.pos = (x, y);
        }
    }

    // 101 / 2 = 50 + 1
    let h_mid_idx = room.width / 2;
    let v_mid_idx = room.height / 2;
    let mut quads = [0, 0, 0, 0];

    for robot in &robots {
        let (x, y) = robot.pos;
        if x < h_mid_idx && y < v_mid_idx {
            quads[0] += 1;
        } else if x > h_mid_idx && y < v_mid_idx {
            quads[1] += 1;
        } else if x > h_mid_idx && y > v_mid_idx {
            quads[2] += 1;
        } else if x < h_mid_idx && y > v_mid_idx {
            quads[3] += 1;
        }
    }

    quads.iter().fold(1, |acc, x| acc * x)
}

fn part2(data: String) -> i64 {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_14/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
