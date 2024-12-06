use crate::utils::InputMode;
use std::collections::{HashMap, HashSet};
use std::fs;

const TEXT_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

const EXPECTED: &str = r#"
....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
"#;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, std::hash::Hash)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, std::hash::Hash)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

// impl std::hash::Hash for Guard {
//     fn hash<H>(&self, state: &mut H)
//     where
//         H: std::hash::Hasher,
//     {
//         self.x.hash(state);
//         self.y.hash(state);
//         self.direction.hash(state);
//     }
// }

impl Guard {
    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x as i32;
        self.y = y as i32;
    }

    fn position(&self) -> Point {
        Point::new(self.x, self.y)
    }

    fn eq(&self, guard: &Guard) -> bool {
        self.x == guard.x && self.y == guard.y && self.direction == guard.direction
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Down => {
                self.y += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
        }
    }

    fn setp_back(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y += 1;
            }
            Direction::Right => {
                self.x -= 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Left => {
                self.x += 1;
            }
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Default)]
struct Window {
    width: i32,
    height: i32,
}

impl Window {
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.width + 1 && y > 0 && y < self.height + 1
    }
}

fn obstacle_colition(obstacles: &HashSet<Point>, point: &Point) -> bool {
    obstacles.contains(point)
}

fn print_patrol_path(
    data: &str,
    obstacles: &HashSet<Point>,
    initial_point: &Point,
    patrol_path: &HashSet<Point>,
    sign: char,
    cycle_obstacle: &Point,
) {
    let mut res: Vec<Vec<char>> = data
        .trim()
        .lines()
        .map(|line| vec!['.'; line.len()])
        .collect();
    for o in obstacles {
        res[o.y as usize][o.x as usize] = '#';
    }
    for p in patrol_path {
        res[p.y as usize][p.x as usize] = sign;
    }
    res[initial_point.y as usize][initial_point.x as usize] = '^';
    res[cycle_obstacle.y as usize][cycle_obstacle.x as usize] = 'O';

    for line in res {
        println!("{}", line.iter().collect::<String>());
    }
}

fn part1(data: String) -> usize {
    let mut obstacles: HashSet<Point> = HashSet::new();
    let mut guard = Guard::default();
    let mut window = Window::default();

    for (y, line) in data.trim().lines().enumerate() {
        let y = y as i32;
        if window.height < y {
            window.height = y;
        }
        for (x, c) in line.trim().chars().enumerate() {
            let x = x as i32;
            if window.width < x {
                window.width = x;
            }
            match c {
                '^' => {
                    guard.set_position(x, y);
                }
                '#' => {
                    obstacles.insert(Point::new(x, y));
                }
                _ => {}
            };
        }
    }

    let mut patrol_path: HashSet<Point> = HashSet::new();
    // let initial_point = guard.position();

    while window.in_bounds(guard.x, guard.y) {
        patrol_path.insert(guard.position());
        guard.step();
        let mut tries: i32 = 4;

        while obstacle_colition(&obstacles, &guard.position()) {
            if tries < 0 {
                panic!("Guard turned more than 4 times in the same move.");
            }
            guard.setp_back();
            guard.turn();
            guard.step();
            tries -= 1;
        }
    }

    patrol_path.len()
}

fn part2(data: String) -> u32 {
    let mut obstacles: HashSet<Point> = HashSet::new();
    let mut init_guard = Guard::default();
    let mut window = Window::default();

    for (y, line) in data.trim().lines().enumerate() {
        let y = y as i32;
        if window.height < y {
            window.height = y;
        }
        for (x, c) in line.trim().chars().enumerate() {
            let x = x as i32;
            if window.width < x {
                window.width = x;
            }
            match c {
                '^' => {
                    init_guard.set_position(x, y);
                }
                '#' => {
                    obstacles.insert(Point::new(x, y));
                }
                _ => {}
            };
        }
    }

    let mut cycle_path_count = 0;

    for y in 0..=window.height {
        for x in 0..=window.width {
            if x == init_guard.x && y == init_guard.y {
                continue;
            }
            // println!("{x}:{y}");
            let mut patrol_path: HashSet<Point> = HashSet::new();
            let mut cycle_detection = HashMap::new();
            let cycle_obstacle = Point::new(x, y);
            let mut guard = init_guard.clone();
            cycle_detection.insert(guard.clone(), 1);
            patrol_path.insert(guard.position());

            if obstacles.contains(&cycle_obstacle) {
                continue;
            }

            while window.in_bounds(guard.x, guard.y) {
                guard.step();

                let cycle_entry = cycle_detection
                    .entry(guard.clone())
                    .and_modify(|x| *x += 1 as i32)
                    .or_insert(0);

                if *cycle_entry > 1 as i32 {
                    cycle_path_count += 1;
                    // print_patrol_path(
                    //     &data,
                    //     &obstacles,
                    //     &init_guard.position(),
                    //     &patrol_path,
                    //     '|',
                    //     &cycle_obstacle,
                    // );
                    // println!("");
                    break;
                }

                let mut tries: i32 = 4;

                while obstacles.contains(&guard.position()) || cycle_obstacle == guard.position() {
                    if tries < 0 {
                        panic!("Guard turned more than 4 times in the same move.");
                    }
                    guard.setp_back();
                    guard.turn();
                    guard.step();
                    tries -= 1;
                }

                patrol_path.insert(guard.position());
            }
        }
    }

    cycle_path_count
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_6/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
