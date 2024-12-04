use std::fs;

const TEXT_INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

const TEST_OUTPUT: &str = r#"
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
"#;

const TEST_OUTPUT_2: &str = r#"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"#;

const WORD: &str = "XMAS";

enum InputMode {
    Test,
    Source,
}

#[derive(Debug)]
enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl Direction {
    fn top(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y < WORD.len() - 1 {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y - i][x] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }

    fn top_right(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y < WORD.len() - 1 || x + WORD.len() > data[y].len() {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y - i][x + i] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }

    fn right(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if x + WORD.len() > data[y].len() {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y][x + i] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }

    fn bottom_right(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y + WORD.len() > data.len() || x + WORD.len() > data[y].len() {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y + i][x + i] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }

    fn bottom(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y + WORD.len() > data.len() {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y + i][x] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }

    fn bottom_left(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y + WORD.len() > data.len() || x < WORD.len() - 1 {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y + i][x - i] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }

    fn left(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if x < WORD.len() - 1 {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y][x - i] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }

    fn top_left(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y < WORD.len() - 1 || x < WORD.len() - 1 {
            return false;
        }
        let mut is_found = true;
        for (i, char) in WORD.chars().enumerate() {
            if data[y - i][x - i] != char {
                is_found = false;
                break;
            }
        }
        is_found
    }
}

#[derive(Debug)]
struct Word {
    x: usize,
    y: usize,
    direction: Direction,
}

fn print_found(data: &Vec<Vec<char>>, found_words: &Vec<Word>) {
    let mut res: Vec<Vec<char>> = data.iter().map(|line| vec!['.'; line.len()]).collect();
    for word in found_words {
        match word.direction {
            Direction::Top => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y - i][word.x] = ch;
                }
            }
            Direction::TopRight => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y - i][word.x + i] = ch;
                }
            }
            Direction::Right => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y][word.x + i] = ch;
                }
            }
            Direction::BottomRight => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y + i][word.x + i] = ch;
                }
            }
            Direction::Bottom => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y + i][word.x] = ch;
                }
            }
            Direction::BottomLeft => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y + i][word.x - i] = ch;
                }
            }
            Direction::Left => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y][word.x - i] = ch;
                }
            }
            Direction::TopLeft => {
                for (i, ch) in WORD.chars().enumerate() {
                    res[word.y - i][word.x - i] = ch;
                }
            }
        }
    }

    for line in res {
        println!("{}", line.iter().collect::<String>());
    }
}

fn part1(data: String) -> usize {
    let mut found_words = vec![];
    let data = data
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if ch == &'X' {
                if Direction::top(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::Top,
                    });
                }
                if Direction::top_right(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::TopRight,
                    });
                }
                if Direction::right(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::Right,
                    });
                }
                if Direction::bottom_right(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::BottomRight,
                    });
                }
                if Direction::bottom(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::Bottom,
                    });
                }
                if Direction::bottom_left(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::BottomLeft,
                    });
                }
                if Direction::left(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::Left,
                    });
                }
                if Direction::top_left(&data, x, y) {
                    found_words.push(Word {
                        x,
                        y,
                        direction: Direction::TopLeft,
                    });
                }
            }
        }
    }

    // println!("{TEST_OUTPUT}");
    // print_found(&data, &found_words);

    found_words.len()
}

struct XWord {
    x: usize,
    y: usize,
    letters: Vec<char>,
}

fn print_found2(data: &Vec<Vec<char>>, words: &[XWord]) {
    let mut res: Vec<Vec<char>> = data.iter().map(|line| vec!['.'; line.len()]).collect();
    for word in words {
        res[word.y][word.x] = 'A';
        res[word.y - 1][word.x - 1] = word.letters[0];
        res[word.y - 1][word.x + 1] = word.letters[1];
        res[word.y + 1][word.x + 1] = word.letters[2];
        res[word.y + 1][word.x - 1] = word.letters[3];
    }
    for line in res {
        println!("{}", line.iter().collect::<String>());
    }
}

fn part2(data: String) -> usize {
    let mut found_words = vec![];
    let data = data
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if ch == &'A' {
                if y < 1 || x < 1 || y > data.len() - 2 || x > line.len() - 2 {
                    continue;
                }
                let top_left = data[y - 1][x - 1];
                let top_right = data[y - 1][x + 1];
                let bottom_right = data[y + 1][x + 1];
                let bottom_left = data[y + 1][x - 1];
                let top_left_bottom_right = top_left == 'M' && bottom_right == 'S'
                    || top_left == 'S' && bottom_right == 'M';
                let top_right_bottom_left = top_right == 'M' && bottom_left == 'S'
                    || top_right == 'S' && bottom_left == 'M';

                if top_left_bottom_right && top_right_bottom_left {
                    found_words.push(XWord {
                        x,
                        y,
                        letters: vec![top_left, top_right, bottom_right, bottom_left],
                    })
                }
            }
        }
    }

    // println!("{TEST_OUTPUT_2}");
    // print_found2(&data, &found_words);

    found_words.len()
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_4/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
