use crate::utils::InputMode;
use std::collections::{HashMap, HashSet};
use std::fs;

const TEXT_INPUT: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

const EXPECTED: &str = r#"
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
"#;

fn part1(data: String) -> usize {
    let mut antenas = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in data.trim().lines().enumerate() {
        height = y;
        for (x, c) in line.chars().enumerate() {
            width = x;
            if c != '.' {
                let entry = antenas.entry(c).or_insert(Vec::new());
                entry.push((x, y));
            }
        }
    }

    fn get_antinodes(
        values: &[(usize, usize)],
        set: &mut HashSet<(usize, usize)>,
        width: usize,
        height: usize,
    ) {
        if values.len() <= 1 {
            return;
        }

        let anchor = values[0];
        for pair in &values[1..] {
            let distance = (
                pair.0 as i32 - anchor.0 as i32,
                pair.1 as i32 - anchor.1 as i32,
            );
            let (top_x, top_y) = (anchor.0 as i32 - distance.0, anchor.1 as i32 - distance.1);
            let (bottom_x, bottom_y) = (pair.0 as i32 + distance.0, pair.1 as i32 + distance.1);
            if top_x >= 0
                && top_y >= 0
                && (0..=width).contains(&top_x.try_into().unwrap())
                && (0..=height).contains(&top_y.try_into().unwrap())
            {
                set.insert((top_x.try_into().unwrap(), top_y.try_into().unwrap()));
            }
            if bottom_x >= 0
                && bottom_y >= 0
                && (0..=width).contains(&bottom_x.try_into().unwrap())
                && (0..=height).contains(&bottom_y.try_into().unwrap())
            {
                set.insert((bottom_x.try_into().unwrap(), bottom_y.try_into().unwrap()));
            }
        }

        get_antinodes(&values[1..], set, width, height);
    }

    let mut antinodes = HashSet::new();
    for (_, points) in antenas.iter() {
        get_antinodes(&points, &mut antinodes, width, height);
    }

    // println!("{EXPECTED}");
    let mut res = vec![vec!['.'; width + 1]; height + 1];
    for node in antinodes.iter() {
        res[node.1][node.0] = '#';
    }
    for (c, items) in antenas.iter() {
        for (x, y) in items {
            res[*y][*x] = *c;
        }
    }

    for line in res {
        println!("{}", line.iter().collect::<String>());
    }

    // println!("antenas: {antenas:?}");
    // println!("antinodes: {antinodes:?}");

    antinodes.len()
}

fn part2(data: String) -> u64 {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_8/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
