use crate::utils::InputMode;
use std::fs;

const TEXT_INPUT: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

#[derive(Debug)]
struct Rule(u32, u32);

impl Rule {
    fn new(left: &str, right: &str) -> Self {
        Self(left.parse().unwrap(), right.parse().unwrap())
    }

    fn contains(&self, value: u32) -> bool {
        self.0 == value || self.1 == value
    }

    fn before(&self) -> u32 {
        self.0
    }

    fn after(&self) -> u32 {
        self.1
    }
}

fn part1(data: String) -> u32 {
    let mut rules = vec![];
    let mut updates = vec![];
    let mut result = 0;

    for line in data.lines() {
        if line.contains("|") {
            let (left, right) = line.split_once("|").unwrap();
            rules.push(Rule::new(left, right));
        } else if line.contains(",") {
            let pages = line
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(pages);
        }
    }

    for page_order in updates {
        let curr_rules: Vec<_> = rules
            .iter()
            .filter(|x| page_order.iter().any(|y| x.contains(*y)))
            .collect();

        // 61|13
        // 61|53
        // 61|47

        // 75,47,61,53,29
        // 75
        //  - [Rule, Rule, Rule]
        //  - if
        let mut is_valid = true;
        for (page_cursor_idx, page_cursor) in page_order.iter().enumerate() {
            let page_cursor_rules = curr_rules
                .iter()
                .filter(|r| r.before() == *page_cursor)
                .collect::<Vec<_>>();

            for i in 0..page_cursor_idx {
                if page_cursor_rules.iter().any(|r| r.after() == page_order[i]) {
                    is_valid = false;
                    break;
                }
            }

            if !is_valid {
                break;
            }
        }

        if is_valid {
            if page_order.len() % 2 == 0 {
                panic!("Unexpected input");
            } else {
                let midd_idx = (page_order.len() - 1) / 2;
                result += page_order[midd_idx];
            }
        }
    }

    result
}

fn part2(data: String) -> u32 {
    let mut rules = vec![];
    let mut updates = vec![];
    let mut result = 0;

    for line in data.lines() {
        if line.contains("|") {
            let (left, right) = line.split_once("|").unwrap();
            rules.push(Rule::new(left, right));
        } else if line.contains(",") {
            let pages = line
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(pages);
        }
    }

    for page_order in updates.iter_mut() {
        let curr_rules: Vec<_> = rules
            .iter()
            .filter(|x| page_order.iter().any(|y| x.contains(*y)))
            .collect();

        // 97|13
        // 97|61
        // 97|47
        // 97|29
        // 97|53
        // 97|75
        //
        // 75|29
        // 75|53
        // 75|47
        // 75|13
        //
        // 29|13
        //

        // 75,97,47,61,53 becomes 97,75,47,61,53.
        // 61,13,29 becomes 61,29,13.
        // 97,13,75,29,47 becomes 97,75,47,29,13.
        // 97,75,13,29,47
        // 97,75,29,13,47
        // 97,75,47,13,29
        // 97,75,47,29,13
        let mut is_valid = true;
        let mut cursor = 0;
        while cursor < page_order.len() {
            let page_cursor_rules = curr_rules
                .iter()
                .filter(|r| r.before() == page_order[cursor])
                .collect::<Vec<_>>();

            for i in 0..cursor {
                if page_cursor_rules.iter().any(|r| r.after() == page_order[i]) {
                    is_valid = false;
                    page_order.swap(i, cursor);
                    cursor = i;
                    break;
                }
            }

            cursor += 1;
        }

        if !is_valid {
            // println!("invalid: {page_order:?}");
            if page_order.len() % 2 == 0 {
                panic!("Unexpected input");
            } else {
                let midd_idx = (page_order.len() - 1) / 2;
                result += page_order[midd_idx];
            }
        }
    }

    result
}

pub fn solve() {
    let mode = InputMode::Source;
    let data = match mode {
        InputMode::Test => TEXT_INPUT.to_string(),
        InputMode::Source => fs::read_to_string("./src/aoc_5/input.txt").unwrap(),
    };
    let result = part2(data);
    println!("reuslt: {result}");
}
