use std::fs;

fn part1() {
    let data = fs::read_to_string("./src/aoc_3/input.txt").unwrap();
    let mut result = 0;
    let mut cursor = 0;

    while cursor < data.len() {
        if data[cursor..].starts_with("mul(") {
            cursor += 4;

            let mut left = String::new();
            while let Some(char) = data[cursor..].chars().next() {
                if char.is_digit(10) {
                    left.push(char);
                    cursor += 1;
                } else {
                    break;
                }
            }

            if left.len() == 0 || left.len() > 3 {
                continue;
            }

            let mut right = String::new();
            if data[cursor..].chars().next() == Some(',') {
                cursor += 1;
                while let Some(char) = data[cursor..].chars().next() {
                    if char.is_digit(10) {
                        right.push(char);
                        cursor += 1;
                    } else {
                        break;
                    }
                }
            } else {
                continue;
            }

            if right.len() == 0 || right.len() > 3 {
                continue;
            }

            if data[cursor..].chars().next() != Some(')') {
                cursor += 1;
                continue;
            }

            let mult = left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
            result += mult;
        } else {
            cursor += 1;
        }
    }

    println!("result: {result}");
}

const DONT: &str = "don't()";
const DO: &str = "do()";

fn part2() {
    let data = fs::read_to_string("./src/aoc_3/input.txt").unwrap();
    let mut result = 0;
    let mut cursor = 0;
    let mut enabled = true;

    while cursor < data.len() {
        if data[cursor..].starts_with(DONT) {
            enabled = false;
            cursor += DONT.len();
        } else if data[cursor..].starts_with(DO) {
            enabled = true;
            cursor += DO.len();
        } else if data[cursor..].starts_with("mul(") {
            cursor += 4;

            let mut left = String::new();
            while let Some(char) = data[cursor..].chars().next() {
                if char.is_digit(10) {
                    left.push(char);
                    cursor += 1;
                } else {
                    break;
                }
            }
            // println!("left {left}");

            if left.len() == 0 || left.len() > 3 {
                continue;
            }

            let mut right = String::new();
            if data[cursor..].chars().next() == Some(',') {
                cursor += 1;
                while let Some(char) = data[cursor..].chars().next() {
                    if char.is_digit(10) {
                        right.push(char);
                        cursor += 1;
                    } else {
                        break;
                    }
                }
            } else {
                continue;
            }
            // println!("right: {right}");

            if right.len() == 0 || right.len() > 3 {
                continue;
            }

            if data[cursor..].chars().next() != Some(')') {
                cursor += 1;
                continue;
            }

            if enabled {
                let mult = left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
                // println!("mult: {left}x{right} = {mult}");
                result += mult;
            }
        } else {
            cursor += 1;
        }
    }

    println!("result: {result}");
}

pub fn solve() {
    part2();
}
