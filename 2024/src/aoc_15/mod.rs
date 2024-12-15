use crate::utils::InputMode;
use std::collections::HashSet;
use std::fs;

const TEXT_INPUT: &str = r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#;

const TEXT_INPUT_2: &str = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

#[derive(Debug, Clone)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Default, Debug)]
struct Game {
    robot: (i32, i32),
    boxes: HashSet<(i32, i32)>,
    walls: HashSet<(i32, i32)>,
}

impl Game {
    // ########
    // #..O.O.#
    // ##@.O..#
    // #...O..#
    // #.#.O..#
    // #...O..#
    // #......#
    // ########
    fn tick(&mut self, instruction: Direction) {
        let dir = match instruction {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0),
        };

        let next_pos = (self.robot.0 + dir.0, self.robot.1 + dir.1);

        if self.boxes.contains(&next_pos) {
            if self.move_boxes(&next_pos, &dir) {
                self.robot = next_pos;
            }
        } else if !self.walls.contains(&next_pos) {
            self.robot = next_pos;
        }

        // self.print(instruction)
    }

    // ..xx.#
    //   ^
    fn move_boxes(&mut self, box_pos: &(i32, i32), dir: &(i32, i32)) -> bool {
        let behind_box = (box_pos.0 + dir.0, box_pos.1 + dir.1);
        if self.walls.contains(&behind_box) {
            return false;
        }

        if self.boxes.contains(&behind_box) {
            if !self.move_boxes(&behind_box, &dir) {
                return false;
            }
        }

        self.boxes.remove(&box_pos);
        self.boxes.insert(behind_box);
        return true;
    }

    fn print(&self) {
        let mut width = 0;
        let mut height = 0;
        for (x, y) in &self.walls {
            width = width.max(*x + 1);
            height = height.max(*y + 1);
        }
        let mut grid = vec![vec!['.'; width as usize]; height as usize];
        for (x, y) in &self.walls {
            grid[*y as usize][*x as usize] = '#';
        }
        for (x, y) in &self.boxes {
            grid[*y as usize][*x as usize] = 'O';
        }
        grid[self.robot.1 as usize][self.robot.0 as usize] = '@';

        for line in grid {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

fn parse(data: &str) -> (Game, Vec<Direction>) {
    let (game_layout, instructions) = data.split_once("\n\n").unwrap();
    let mut game = Game::default();

    for (y, line) in game_layout.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '#' => {
                    game.walls.insert((x as i32, y as i32));
                }
                'O' => {
                    game.boxes.insert((x as i32, y as i32));
                }
                '@' => game.robot = (x as i32, y as i32),
                _ => {}
            }
        }
    }

    let mut inst = vec![];
    for line in instructions.lines() {
        for c in line.chars() {
            match c {
                '>' => inst.push(Direction::Right),
                'v' => inst.push(Direction::Bottom),
                '^' => inst.push(Direction::Top),
                '<' => inst.push(Direction::Left),
                _ => {}
            };
        }
    }

    (game, inst)
}

fn part1(data: String) -> i32 {
    let (mut game, instructions) = parse(&data);
    game.print();

    for inst in instructions.iter() {
        game.tick(inst.clone());
    }
    game.print();

    game.boxes.iter().map(|(x, y)| 100 * y + x).sum()
}

fn part2(data: String) -> i64 {
    0
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_15/input.txt").unwrap(),
    };
    let result = part1(data);
    println!("reuslt: {result}");
}
