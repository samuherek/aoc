use crate::utils::InputMode;
use std::collections::{HashMap, HashSet};
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

const TEXT_INPUT_3: &str = r#"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
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

#[derive(Debug, Clone)]
struct Box {
    id: usize,
    cells: ((i32, i32), (i32, i32)),
}

impl Box {
    fn edges(&self, dir: &Direction) -> Vec<(i32, i32)> {
        let mut cells = vec![];
        match dir {
            Direction::Top => {
                cells.push((self.cells.0 .0, self.cells.0 .1 - 1));
                cells.push((self.cells.1 .0, self.cells.1 .1 - 1));
            }
            Direction::Right => {
                cells.push((self.cells.1 .0 + 1, self.cells.1 .1));
            }
            Direction::Bottom => {
                cells.push((self.cells.0 .0, self.cells.0 .1 + 1));
                cells.push((self.cells.1 .0, self.cells.1 .1 + 1));
            }
            Direction::Left => {
                cells.push((self.cells.0 .0 - 1, self.cells.0 .1));
            }
        }
        cells
    }

    fn reposition(&mut self, dir: &Direction) {
        match dir {
            Direction::Top => {
                self.cells.0 = (self.cells.0 .0, self.cells.0 .1 - 1);
                self.cells.1 = (self.cells.1 .0, self.cells.1 .1 - 1);
            }
            Direction::Right => {
                self.cells.0 = self.cells.1;
                self.cells.1 = (self.cells.1 .0 + 1, self.cells.1 .1);
            }
            Direction::Bottom => {
                self.cells.0 = (self.cells.0 .0, self.cells.0 .1 + 1);
                self.cells.1 = (self.cells.1 .0, self.cells.1 .1 + 1);
            }
            Direction::Left => {
                self.cells.1 = self.cells.0;
                self.cells.0 = (self.cells.0 .0 - 1, self.cells.0 .1);
            }
        }
    }
}

#[derive(Debug)]
struct GameScaled {
    robot: (i32, i32),
    boxes_layout: HashMap<(i32, i32), usize>,
    boxes: HashMap<usize, Box>,
    walls: HashSet<(i32, i32)>,
}

impl GameScaled {
    // ##############
    // ##......##..##
    // ##..........##
    // ##....[][]@.##
    // ##....[]....##
    // ##..........##
    // ##############
    fn tick(&mut self, direction: Direction) {
        let dir = match direction {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0),
        };
        let next_pos = (self.robot.0 + dir.0, self.robot.1 + dir.1);

        if let Some(id) = self.boxes_layout.get(&next_pos) {
            if self.move_boxes(*id, &direction) {
                self.robot = next_pos;
            }
        } else if !self.walls.contains(&next_pos) {
            self.robot = next_pos;
        }
    }

    //    xx
    // [][]
    //  []
    //  ^
    //
    // [] xx
    //   []
    //  []
    //  ^
    fn can_move_block(&self, edges: &Vec<(i32, i32)>, direction: &Direction) -> bool {
        if edges.iter().any(|x| self.walls.contains(x)) {
            return false;
        }

        let boxes = edges
            .iter()
            .filter_map(|cell| self.boxes_layout.get(cell))
            .filter_map(|id| self.boxes.get(id))
            .collect::<Vec<_>>();

        if boxes.len() > 0 {
            return boxes
                .iter()
                .all(|x| self.can_move_block(&x.edges(direction), direction));
        }

        true
    }

    // ..[][].#    ..[][].#
    //   ^            ^
    //
    //    []          []
    // ..[][].#    ..[][].#
    //   ^            ^
    fn move_boxes(&mut self, box_id: usize, direction: &Direction) -> bool {
        let b = self.boxes.get(&box_id).unwrap();
        let behind_cells = b.edges(direction);

        if !self.can_move_block(&behind_cells, direction) {
            return false;
        }

        let boxes = behind_cells
            .iter()
            .filter_map(|cell| self.boxes_layout.get(cell))
            .map(|x| *x)
            .collect::<Vec<_>>();
        let mut last_b = None;
        let len = boxes.len();

        for b in boxes {
            last_b = Some(b);
            if !self.move_boxes(b, direction) {
                // panic!("have have a logic bug!");
                println!(
                    "robot: {:?} box: {:?}, direction: {direction:?}",
                    self.robot,
                    self.boxes.get(&b)
                );
                println!("boxes len: {len}");
                println!("last: {last_b:?}");
                println!("behind cells: {behind_cells:?}");
                self.print();
                return false;
            }
        }

        // remove box layouts from current.
        // loop over the box cells and increment by dir
        // input into box layouts.
        let b = self.boxes.get(&box_id).unwrap();
        self.boxes_layout.remove(&b.cells.0);
        self.boxes_layout.remove(&b.cells.1);

        let b = self.boxes.get_mut(&box_id).unwrap();
        b.reposition(direction);

        self.boxes_layout.insert(b.cells.0, b.id);
        self.boxes_layout.insert(b.cells.1, b.id);

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
        for (id, b) in &self.boxes {
            let left = b.cells.0;
            let right = b.cells.1;
            grid[left.1 as usize][left.0 as usize] = '[';
            grid[right.1 as usize][right.0 as usize] = ']';
        }

        grid[self.robot.1 as usize][self.robot.0 as usize] = '@';

        for line in grid {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

impl From<Game> for GameScaled {
    fn from(game: Game) -> Self {
        let mut g = GameScaled {
            robot: (game.robot.0 * 2, game.robot.1),
            boxes_layout: HashMap::new(),
            boxes: HashMap::new(),
            walls: HashSet::new(),
        };

        for (x, y) in game.walls {
            g.walls.insert((x * 2, y));
            g.walls.insert((x * 2 + 1, y));
        }

        for (i, (x, y)) in game.boxes.iter().enumerate() {
            let box_1 = (x * 2, *y);
            let box_2 = (x * 2 + 1, *y);
            g.boxes_layout.insert(box_1.clone(), i);
            g.boxes_layout.insert(box_2.clone(), i);
            g.boxes.insert(
                i,
                Box {
                    id: i,
                    cells: (box_1, box_2),
                },
            );
        }

        g
    }
}

fn part2(data: String) -> i32 {
    let (game, inst) = parse(&data);
    let mut game = GameScaled::from(game);

    game.print();
    for (c, i) in inst.iter().enumerate() {
        game.tick(i.clone());
    }
    game.print();

    game.boxes
        .iter()
        .map(|(_, b)| 100 * b.cells.0 .1 + b.cells.0 .1)
        .sum()
}

pub fn solve() {
    let mode = InputMode::Test;
    let data = match mode {
        InputMode::Test => TEXT_INPUT_3.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_15/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
