use std::fs;

fn part1() {
    let data = fs::read_to_string("./src/aoc_1/input.txt").unwrap();
    let mut left = vec![];
    let mut right = vec![];

    for line in data.lines() {
        let (l, r) = line
            .split_once(' ')
            .map(|(x, y)| {
                (
                    x.trim().parse::<u32>().unwrap(),
                    y.trim().parse::<u32>().unwrap(),
                )
            })
            .unwrap();

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());
    let mut result = 0;

    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];
        let diff: f32 = l as f32 - r as f32;
        result += diff.abs() as u32;
    }

    println!("result {result}");
}

fn part2() {
    let data = fs::read_to_string("./src/aoc_1/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(x, y)| {
                    (
                        x.trim().parse::<u32>().unwrap(),
                        y.trim().parse::<u32>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut right_map = std::collections::HashMap::new();

    for (_, right) in &data {
        let val = right_map.entry(right).or_insert(0);
        *val += 1;
    }

    let mut result = 0;
    for (left, _) in &data {
        let times = right_map.entry(left).or_default();
        if *times != 0 {
            result += left * (*times);
        }
    }

    println!("result: {result}");
}

pub fn solve() {
    part2();
}
