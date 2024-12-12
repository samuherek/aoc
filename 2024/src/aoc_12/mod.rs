use crate::utils::InputMode;
use std::collections::{HashMap, HashSet};
use std::fs;

const TEXT_INPUT: &str = r#"
AAAA
BBCD
BBCC
EEEC
"#;

const TEXT_INPUT_2: &str = r#"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"#;

const TEXT_INPUT_3: &str = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

const EXPECTED: &str = r#"
"#;

const EXPECTED_2: &str = r#"
"#;

#[derive(Debug)]
struct Region {
    kind: char,
    points: Vec<(i32, i32)>,
}

impl Region {
    fn new(c: char) -> Self {
        Self {
            kind: c,
            points: vec![],
        }
    }

    fn push(&mut self, point: (i32, i32)) {
        self.points.push(point);
    }

    // 1 touch -> 4 | 0 inter
    // 2 touch -> 6 | 1 inter
    // 3 touch -> 8 | 2 inter
    // 4 touch -> 4 inter | 8 || 3 inter | 10
    // 5 touch -> 5 inter | 10 || 4 inter | 12
    // 6 touch ->

    fn perimeter(&self) -> usize {
        let len = self.points.len();

        let map = self
            .points
            .iter()
            .map(|x| (x, true))
            .collect::<HashMap<_, _>>();
        let mut touch = HashSet::new();

        for point in &self.points {
            let x = point.0;
            let y = point.1;

            let top = (x, y - 1);
            let top_touch = (x, y, top.0, top.1);
            let right = (x + 1, y);
            let right_touch = (x, y, right.0, right.1);
            let bottom = (x, y + 1);
            let bottom_touch = (x, y, bottom.0, bottom.1);
            let left = (x - 1, y);
            let left_touch = (x, y, left.0, left.1);

            if !touch.contains(&top_touch) && map.contains_key(&top) {
                touch.insert(top_touch);
            }
            if !touch.contains(&right_touch) && map.contains_key(&right) {
                touch.insert(right_touch);
            }
            if !touch.contains(&bottom_touch) && map.contains_key(&bottom) {
                touch.insert(bottom_touch);
            }
            if !touch.contains(&left_touch) && map.contains_key(&left) {
                touch.insert(left_touch);
            }
        }

        // println!("len: {len} touch: {}", touch.len());
        (len * 4) - touch.len()
    }

    fn perimeter2(&self) -> usize {
        let len = self.points.len();

        let map = self
            .points
            .iter()
            .map(|x| (x, true))
            .collect::<HashMap<_, _>>();

        

        0
    }
}

fn part1(data: String) -> usize {
    let data: Vec<Vec<char>> = data
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c).collect())
        .collect();

    // collect number of items that form an area and keep their points
    // itterate over the points and find the fence border and count it
    // fence top bottom, fence right left

    // loop over data and collect regions
    //
    // aaa
    // bbc
    // bcc
    //
    // for point 1,1

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();

    fn in_bounds(point: &(i32, i32), data: &Vec<Vec<char>>) -> bool {
        let (x, y) = point;
        let x = *x;
        let y = *y;
        x >= 0
            && (0..data[0].len()).contains(&x.try_into().unwrap())
            && y >= 0
            && (0..data.len()).contains(&y.try_into().unwrap())
    }

    fn collect_region(
        point: (i32, i32),
        curr_region: &mut Region,
        data: &Vec<Vec<char>>,
        visited: &mut HashSet<(i32, i32)>,
    ) -> bool {
        if visited.contains(&point) || !in_bounds(&point, &data) {
            return false;
        }

        let x = point.0;
        let y = point.1;
        let kind = data[y as usize][x as usize];
        if kind != curr_region.kind {
            return false;
        }

        curr_region.push(point);
        visited.insert(point);

        let top = (x, y - 1);
        let right = (x + 1, y);
        let bottom = (x, y + 1);
        let left = (x - 1, y);

        collect_region(top, curr_region, data, visited);
        collect_region(right, curr_region, data, visited);
        collect_region(bottom, curr_region, data, visited);
        collect_region(left, curr_region, data, visited);

        return true;
    }

    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if !visited.contains(&(x, y)) {
                let mut region = Region::new(*c);
                collect_region((x, y), &mut region, &data, &mut visited);
                regions.push(region);
            }
        }
    }

    // println!("regions: {regions:?}");

    // for region in regions {
    //     println!("{}", region.perimeter());
    // }

    regions.iter().map(|x| x.perimeter() * x.points.len()).sum()
}

fn part2(data: String) -> u64 {
0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_12/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
