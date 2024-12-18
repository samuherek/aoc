use crate::utils::InputMode;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

const TEXT_INPUT: &str = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

const TEXT_INPUT_2: &str = r#"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#;

fn parse(data: &str) -> Game {
    let mut game = Game {
        walls: HashSet::new(),
        person: (0, 0, Direction::Right),
        finish: (0, 0),
        width: 0,
        height: 0,
        path: vec![],
    };
    for (y, line) in data.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            game.width = game.width.max((x + 1) as i32);
            game.height = game.height.max((y + 1) as i32);
            match c {
                '#' => {
                    game.walls.insert((x as i32, y as i32));
                }
                'E' => {
                    game.finish = (x as i32, y as i32);
                }
                'S' => {
                    game.person.0 = x as i32;
                    game.person.1 = y as i32;
                }
                _ => {}
            }
        }
    }

    game
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn next_coords(&self, coord: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Top => (coord.0, coord.1 - 1),
            Direction::Right => (coord.0 + 1, coord.1),
            Direction::Bottom => (coord.0, coord.1 + 1),
            Direction::Left => (coord.0 - 1, coord.1),
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::Top => Direction::Left,
            Direction::Right => Direction::Top,
            Direction::Bottom => Direction::Right,
            Direction::Left => Direction::Bottom,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        }
    }

    fn flip(&self) -> Self {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug)]
struct Game {
    walls: HashSet<(i32, i32)>,
    person: (i32, i32, Direction),
    finish: (i32, i32),
    width: i32,
    height: i32,
    path: Vec<(i32, i32, Direction)>,
}

impl Game {
    // ###############
    // #.......#....E#
    // #.#.###.#.###.#
    // #.....#.#...#.#
    // #.###.#####.#.#
    // #.#.#.......#.#
    // #.#.#####.###.#
    // #...........#.#
    // ###.#.#####.#.#
    // #...#.....#.#.#
    // #.#.#.###.#.#.#
    // #.....#...#.#.#
    // #.###.#.#.#.#.#
    // #S..#.....#...#
    // ###############
    //
    // check top, right, bottom, Left
    //   3 -> 2
    //  /    /
    // 0  -> 2  -> 2
    //  \     \
    //    1    2
    //     \
    //      2
    fn find_path(
        &self,
    ) -> (
        Vec<(i32, i32, Direction)>,
        usize,
        HashMap<(i32, i32, Direction), usize>,
    ) {
        let mut states = HashMap::<(i32, i32, Direction), usize>::new();
        let mut path = HashMap::<(i32, i32, Direction), (i32, i32, Direction)>::new();
        let mut heap = BinaryHeap::new();

        states.insert((self.person.0, self.person.1, self.person.2.clone()), 0);
        heap.push(Node {
            x: self.person.0,
            y: self.person.1,
            cost: 0,
            dir: self.person.2.clone(),
        });

        // - pop stack with least state
        while let Some(node) = heap.pop() {
            let node_pos = node.pos();
            let curr = (node.x, node.y, node.dir.clone());
            let curr_cost = states.get(&curr);

            if curr_cost.is_none() || *curr_cost.unwrap() < node.cost {
                continue;
            }

            // - check if finish, if so, skip to the end and return result
            if self.finish == node_pos {
                let mut res = vec![];
                let mut reference = &(node.x, node.y, node.dir);
                println!("path len:: {}", path.len());
                while let Some(t) = path.get(reference) {
                    res.push(t.clone());
                    reference = t;
                }
                return (res, node.cost, states);
            }

            // - get straight. If curr state would be lower:
            //      - put new state count to visited and overite
            //      - push new state to heap for next pop
            let straight = node.dir.next_coords(node_pos);
            let s_ref = (straight.0, straight.1, node.dir.clone());
            if !self.walls.contains(&straight) {
                let next_cost = node.cost + 1;

                if &next_cost < states.get(&s_ref).unwrap_or(&usize::MAX) {
                    path.insert(s_ref.clone(), curr.clone());
                    states.insert(s_ref, next_cost);

                    heap.push(Node {
                        x: straight.0,
                        y: straight.1,
                        cost: next_cost,
                        dir: node.dir.clone(),
                    });
                }
            }

            // - get lef. If curr state would be lower:
            //      - put new state count to visited and overwrite
            //      - push new state to heap for next pop
            let left = node.dir.left();
            let left_pos = left.next_coords(node_pos);
            let l_ref = (left_pos.0, left_pos.1, left.clone());
            if !self.walls.contains(&left_pos) {
                let next_cost = node.cost + 1001;

                if &next_cost < states.get(&l_ref).unwrap_or(&usize::MAX) {
                    path.insert(l_ref.clone(), curr.clone());
                    states.insert(l_ref, next_cost);

                    heap.push(Node {
                        x: left_pos.0,
                        y: left_pos.1,
                        cost: next_cost,
                        dir: left,
                    });
                }
            }

            // - get lef. If curr state would be lower:
            //      - put new state count to visited and overwrite
            //      - push new state to heap for next pop
            let right = node.dir.right();
            let right_pos = right.next_coords(node_pos);
            let r_ref = (right_pos.0, right_pos.1, right.clone());
            if !self.walls.contains(&right_pos) {
                let next_cost = node.cost + 1001;

                if &next_cost < states.get(&r_ref).unwrap_or(&usize::MAX) {
                    path.insert(r_ref.clone(), curr.clone());
                    states.insert(r_ref, next_cost);

                    heap.push(Node {
                        x: right_pos.0,
                        y: right_pos.1,
                        cost: next_cost,
                        dir: right,
                    });
                }
            }
        }

        (vec![], 0, HashMap::new())
    }

    fn dijkstras(
        &self,
        initial: &[(i32, i32, Direction)],
    ) -> HashMap<(i32, i32, Direction), usize> {
        let mut states = HashMap::new();
        let mut heap = BinaryHeap::new();

        for state in initial {
            heap.push(Node {
                x: state.0,
                y: state.1,
                dir: state.2.clone(),
                cost: 0,
            });
            states.insert((state.0, state.1, state.2.clone()), 0);
        }

        while let Some(node) = heap.pop() {
            let curr_state = (node.x, node.y, node.dir.clone());
            // If we have found a better path before, skip
            if states.get(&curr_state).unwrap_or(&usize::MAX) < &node.cost {
                continue;
            }

            let s = node.dir.next_coords((node.x, node.y));
            let s_ref = (s.0, s.1, node.dir.clone());
            if !self.walls.contains(&s) {
                let next_cost = node.cost + 1;

                if &next_cost < states.get(&s_ref).unwrap_or(&usize::MAX) {
                    states.insert(s_ref, next_cost);

                    heap.push(Node {
                        x: s.0,
                        y: s.1,
                        cost: next_cost,
                        dir: node.dir.clone(),
                    });
                }
            }

            let l = node.dir.left();
            let l_ref = (node.x, node.y, l.clone());
            let next_cost = node.cost + 1000;

            if &next_cost < states.get(&l_ref).unwrap_or(&usize::MAX) {
                states.insert(l_ref, next_cost);

                heap.push(Node {
                    x: node.x,
                    y: node.y,
                    cost: next_cost,
                    dir: l,
                });
            }

            let r = node.dir.right();
            let r_ref = (node.x, node.y, r.clone());
            let next_cost = node.cost + 1000;

            if &next_cost < states.get(&r_ref).unwrap_or(&usize::MAX) {
                states.insert(r_ref, next_cost);

                heap.push(Node {
                    x: node.x,
                    y: node.y,
                    cost: next_cost,
                    dir: r,
                });
            }
        }

        states
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    x: i32,
    y: i32,
    cost: usize,
    dir: Direction,
}

impl Node {
    fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = vec![vec!['.'; self.width as usize]; self.height as usize];

        for (x, y) in &self.walls {
            buff[*y as usize][*x as usize] = '#';
        }
        for (x, y, dir) in &self.path {
            buff[*y as usize][*x as usize] = match dir {
                Direction::Top => '^',
                Direction::Right => '>',
                Direction::Bottom => 'v',
                Direction::Left => '<',
            };
        }
        buff[self.person.1 as usize][self.person.0 as usize] = 'S';
        buff[self.finish.1 as usize][self.finish.0 as usize] = 'E';

        for line in buff {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }

        Ok(())
    }
}

fn part1(data: String) -> usize {
    let mut game = parse(&data);
    println!("{game}");

    let (path, cost, states) = game.find_path();
    game.path = path;

    println!("{game}");
    let mut st = vec![vec!["....".to_string(); game.width as usize]; game.height as usize];
    for ((x, y, _), c) in states {
        st[y as usize][x as usize] = format!("{:0>4}", c);
    }
    for l in &st {
        println!("{}", l.join(" "));
    }
    println!("");

    cost
}

fn part2(data: String) -> usize {
    let mut game = parse(&data);
    println!("{game}");

    let start = game.dijkstras(&vec![(game.person.0, game.person.1, game.person.2.clone())]);
    let end_dirs = vec![
        (game.finish.0, game.finish.1, Direction::Top),
        (game.finish.0, game.finish.1, Direction::Right),
        (game.finish.0, game.finish.1, Direction::Bottom),
        (game.finish.0, game.finish.1, Direction::Left),
    ];
    let end = game.dijkstras(&end_dirs);

    let min_finish = end_dirs
        .iter()
        .filter_map(|point| start.get(point))
        .min()
        .unwrap_or(&usize::MAX);

    let mut overlap = HashSet::new();

    for (point, cost) in &start {
        let b_cost = match end.get(&(point.0, point.1, point.2.flip())) {
            Some(v) => v,
            None => continue,
        };
        if *cost + *b_cost == *min_finish {
            overlap.insert((point.0, point.1));
        }
    }

    let mut st = vec![vec!['.'; game.width as usize]; game.height as usize];
    for (x, y) in &overlap {
        st[(*y) as usize][(*x) as usize] = 'O';
    }
    for l in &st {
        println!("{}", l.iter().collect::<String>());
    }
    println!("");

    overlap.len()
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT_2.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_16/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
