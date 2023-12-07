use anyhow::Result;

const DIRS: [(i32, i32); 9] = [
    (-1, -1), (-1,0), (-1,1), 
    (0, -1), (0,0), (0,1), 
    (1, -1), (1,0), (1,1), 
];

fn get_point(rows: &Vec::<Vec<char>>, (y, x): (usize, usize)) -> Option<&char> {
    match rows.get(y) {
        Some(row) => row.get(x),
        None => None
    }
}

pub fn part1_solve(data: &String) -> Result<u32> {
    let rows: Vec::<Vec<char>> = data.lines()
        .map(|l| l.chars().collect())
        .collect();

    let sum: u32 = rows.iter().enumerate().map(|(row_idx, chars)| {
        let valid_nums: Vec<u32> = vec!();
        let temp_num = String::new();

        // check if item is a digit
        // if a digit, add it to temp digit string
        // if a digit, check if anything around is a symbol
        // -- if a symbol, mark the digit as valid and add to valid nums
        // -- if not a symbol, continue check until "."

        for (char_idx, val) in chars.iter().enumerate() {
            match 
        }

        return valid_nums.iter().sum();
    }).sum();

    return Ok(0);
}
