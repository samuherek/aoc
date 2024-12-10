use crate::utils::InputMode;
use std::fs;

const TEXT_INPUT: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

const EXPECTED: &str = r#"
"#;

const EXPECTED_2: &str = r#"
"#;

fn part1(data: String) -> usize {
    let data: Vec<Vec<_>> = data
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut starts = vec![];
    for (y, line) in data.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if *point == 0 {
                starts.push((x, y));
            }
        }
    }

    fn in_bounds(point: &(i32, i32), data: &Vec<Vec<u32>>) -> bool {
        let (x, y) = point;
        let x = *x;
        let y = *y;
        x >= 0
            && (0..data[0].len()).contains(&x.try_into().unwrap())
            && y >= 0
            && (0..data.len()).contains(&y.try_into().unwrap())
    }

    fn increment_of_one(value: u32, point: &(i32, i32), data: &Vec<Vec<u32>>) -> bool {
        let next_val = data[point.1 as usize][point.0 as usize];
        next_val <= 9 && value + 1 == next_val
    }

    fn valid_move(prev_value: u32, point: &(i32, i32), data: &Vec<Vec<u32>>) -> bool {
        let happy_bounds = in_bounds(point, data);
        if !happy_bounds {
            return false;
        }
        let happy_increment = increment_of_one(prev_value, point, data);
        happy_increment
    }

    // is end?
    //  - the value is 9
    //  retrun from the loop
    // is valid move?
    //  - the valid move is x + 1
    //  continue in the loop
    //

    // try top
    //  valid move?
    //  true -> find_path
    //  false -> move to right
    //
    // try right
    //  valid move?
    //  true -> find_path
    //      move on
    //  false -> move to right
    //
    //

    fn find_path(point: (i32, i32), data: &Vec<Vec<u32>>, points: &mut Vec<(i32, i32)>) -> bool {
        let (x, y) = point;
        let value = data[y as usize][x as usize];

        if value == 9 {
            points.push(point);
            return true;
        }

        let top = (x, y - 1);
        let right = (x + 1, y);
        let bottom = (x, y + 1);
        let left = (x - 1, y);

        if valid_move(value, &top, data) {
            // println!("valid move top");
            find_path(top, data, points);
        }
        if valid_move(value, &right, data) {
            // println!("valid move right");
            find_path(right, data, points);
        }
        if valid_move(value, &bottom, data) {
            // println!("valid move bottom");
            find_path(bottom, data, points);
        }
        if valid_move(value, &left, data) {
            // println!("valid move left");
            find_path(left, data, points);
        }

        return false;
    }

    let result = starts
        .into_iter()
        .map(|point| {
            let mut points = vec![];
            let p = (point.0 as i32, point.1 as i32);
            find_path(p, &data, &mut points);
            let mut set = std::collections::HashSet::new();
            for p in points {
                set.insert(p);
            }
            set.len()
        })
        .sum();

    result
}

fn part2(data: String) -> usize {
    let data: Vec<Vec<_>> = data
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut starts = vec![];
    for (y, line) in data.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if *point == 0 {
                starts.push((x, y));
            }
        }
    }

    fn in_bounds(point: &(i32, i32), data: &Vec<Vec<u32>>) -> bool {
        let (x, y) = point;
        let x = *x;
        let y = *y;
        x >= 0
            && (0..data[0].len()).contains(&x.try_into().unwrap())
            && y >= 0
            && (0..data.len()).contains(&y.try_into().unwrap())
    }

    fn increment_of_one(value: u32, point: &(i32, i32), data: &Vec<Vec<u32>>) -> bool {
        let next_val = data[point.1 as usize][point.0 as usize];
        next_val <= 9 && value + 1 == next_val
    }

    fn valid_move(prev_value: u32, point: &(i32, i32), data: &Vec<Vec<u32>>) -> bool {
        let happy_bounds = in_bounds(point, data);
        if !happy_bounds {
            return false;
        }
        let happy_increment = increment_of_one(prev_value, point, data);
        happy_increment
    }

    fn find_path(point: (i32, i32), data: &Vec<Vec<u32>>, count: &mut usize) -> bool {
        let (x, y) = point;
        let value = data[y as usize][x as usize];

        if value == 9 {
            *count += 1;
            return true;
        }

        let top = (x, y - 1);
        let right = (x + 1, y);
        let bottom = (x, y + 1);
        let left = (x - 1, y);

        if valid_move(value, &top, data) {
            // println!("valid move top");
            find_path(top, data, count);
        }
        if valid_move(value, &right, data) {
            // println!("valid move right");
            find_path(right, data, count);
        }
        if valid_move(value, &bottom, data) {
            // println!("valid move bottom");
            find_path(bottom, data, count);
        }
        if valid_move(value, &left, data) {
            // println!("valid move left");
            find_path(left, data, count);
        }

        return false;
    }

    let result = starts
        .into_iter()
        .map(|point| {
            let mut count = 0;
            let p = (point.0 as i32, point.1 as i32);
            find_path(p, &data, &mut count);
            count
        })
        .sum();

    result
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_10/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
