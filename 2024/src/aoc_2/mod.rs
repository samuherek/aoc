use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
}

fn part1() {
    let data = fs::read_to_string("./src/aoc_2/input.txt").unwrap();
    let data = data
        .lines()
        .map(|line| {
            let values: Vec<_> = line
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect();
            values
        })
        .filter(|x| {
            let mut direction = Direction::Up;

            for right_idx in 1..x.len() {
                let left = x[right_idx - 1];
                let right = x[right_idx];
                let diff = (left as f32 - right as f32).abs() as u32;

                if diff < 1 || diff > 3 {
                    return false;
                }

                if right_idx == 1 {
                    if left == right {
                        return false;
                    }

                    direction = if left > right {
                        Direction::Down
                    } else {
                        Direction::Up
                    };
                }

                match direction {
                    Direction::Up => {
                        if left > right {
                            return false;
                        }
                    }
                    Direction::Down => {
                        if left < right {
                            return false;
                        }
                    }
                }
            }
            return true;
        })
        .collect::<Vec<_>>();
    let result = data.len();

    println!("result: {result}");
}

fn get_direction(left: u32, rigth: u32) -> Direction {
    if left > rigth {
        Direction::Down
    } else {
        Direction::Up
    }
}

fn part2() {
    let data = fs::read_to_string("./src/aoc_2/input.txt").unwrap();
    let mut data = data
        .lines()
        .map(|line| {
            let values: Vec<_> = line
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect();
            values
        })
        .collect::<Vec<_>>();
    let mut data = vec![vec![34,35,37,39,38,40,45]];

    let mut results = 0;

    // 1,2,3,4      => ok
    // 1,'3',2,4    => ok -> just remove the item
    // '3',2,4,5    => ok -> "we assume this won't be in the input.txt or that this is wrong"
    // 1,2,'6',3    => ok -> just remove the item
    // 1,2,3,'9'    => ok -> just remove the item

    for line in data.iter_mut() {
        let mut line_dir = Direction::Up;
        println!("line {line:?}");

        let mut idx = 0;
        let mut already_skipped = false;
        let mut safe = true;

        while idx < line.len() - 1 && safe {
            let inner_idx = idx;
            idx += 1;
            let left = line[inner_idx];
            let mut right = line[inner_idx + 1];
            let mut diff = (left as f32 - right as f32).abs() as u32;

            if diff < 1
                || diff > 3
                || (inner_idx > 0 && get_direction(left, right) != line_dir) && !already_skipped
            {
                already_skipped = true;
                line.remove(inner_idx + 1);

                if idx == line.len() {
                    break;
                }

                right = line[inner_idx + 1];
                diff = (left as f32 - right as f32).abs() as u32;
            }
            // println!("comapring: {left}:{right}");

            if inner_idx == 0 {
                if left == right {
                    safe = false;
                    break;
                }
            }

            line_dir = get_direction(left, right);

            if diff >= 1 && diff <= 3 {
                match line_dir {
                    Direction::Up => {
                        if left > right {
                            safe = false;
                            break;
                        }
                    }
                    Direction::Down => {
                        if left < right {
                            safe = false;
                            break;
                        }
                    }
                }
            } else {
                safe = false;
                break;
            }
        }

        if safe {
            // println!("safe");
            results += 1;
        } else {
            println!("unsafe");
        }
    }

    println!("result: {results}");
}

pub fn solve() {
    part2();
}
