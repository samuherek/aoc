use anyhow::Result;

fn is_digit(char: &char) -> bool {
    char::is_digit(char.clone(), 10)
}

fn is_symbol(char: char) -> bool {
    !char::is_digit(char, 10) && char != '.'
}

fn is_in_row(row: Option<&&str>, cursor: &usize) -> bool {
    return row.is_some_and(|row| {
        let start = cursor.saturating_sub(1);
        let end = if cursor >= &row.len() {
            row.len().saturating_sub(1)
        } else {
            cursor + 1
        };

        return row.get(start..=end).is_some_and(|val| val.chars().any(is_symbol));
    }) ;
}

fn has_adjecent(rows: &Vec<&str>, (y_idx, x_idx): (&usize, &usize)) -> bool {
    let is_top = is_in_row(rows.get(y_idx.saturating_sub(1)), x_idx);
    let is_row = is_in_row(rows.get(*y_idx), x_idx);
    let y_idx = if y_idx >= &rows.len() {
        rows.len().saturating_sub(1)
    } else {
        y_idx + 1
    };

    let is_bottom = is_in_row(rows.get(y_idx), x_idx);

    return is_top || is_row || is_bottom;
}

pub fn part1_solve(data: &String) -> Result<u32> {
    let rows = data.lines().collect::<Vec<&str>>();

    let sum: u32 = rows.iter().enumerate().map(|(row_idx, line)| {
        let mut valid_nums: Vec<u32> = Vec::new();
        let mut temp_num = String::new();
        let mut valid_num = false;

        for (char_idx, val) in line.chars().enumerate() {
            match val {
                '0'..='9' => {
                    if !valid_num {
                        valid_num = has_adjecent(&rows, (&row_idx, &char_idx));
                    }
                    temp_num.push(val);
                },
                _ => {
                    if valid_num {
                        valid_nums.push(temp_num.parse().unwrap_or(0));
                        valid_num = false;
                    }
                    if !temp_num.is_empty() {
                        temp_num.clear();
                    }
                }
            }

        }

        if valid_num {
            valid_nums.push(temp_num.parse().unwrap_or(0));
        }

        return valid_nums;
    }).flatten().sum();

    return Ok(sum);
}


fn parse_left(val: Option<&str>) -> Option<String> {
    return val.and_then(|val| {
        let reversed = val
            .chars()
            .into_iter()
            .rev()
            .take_while(is_digit)
            .collect::<Vec<char>>();

        return Some(reversed.iter().rev().collect());
    });
}

fn parse_right(val: Option<&str>) -> Option<String> {
    return val.and_then(|val| {
        return Some(val.chars().into_iter().take_while(is_digit).collect());
    })
}

fn parse_row(row: &str, x: usize) -> Vec<u32>{
    let mut valid_nums = Vec::new();
    let mut temp_num = String::new();

    let left_x = x.saturating_sub(1);
    let left_val = row.chars().nth(left_x).unwrap_or(' ');
    let center_val = row.chars().nth(x).unwrap_or(' ');
    let right_x = if x >= row.len() {
        row.len().saturating_sub(1)
    } else {
        x + 1
    };
    let right_val = row.chars().nth(right_x).unwrap_or(' ');


    if is_digit(&left_val) {
        if let Some(num) = parse_left(row.get(..=left_x)) {
            temp_num.push_str(num.as_str());
        }
    }


    if is_digit(&center_val) {
        temp_num.push(center_val);
    } else if !temp_num.is_empty() {
        if let Ok(num) = temp_num.parse::<u32>() {
            valid_nums.push(num);
        }

        temp_num.clear();
    }

    if is_digit(&right_val) { 
        if let Some(num) = parse_right(row.get(right_x..)) {
            temp_num.push_str(num.as_str());
        }
    } else if !temp_num.is_empty() {
        if let Ok(num) = temp_num.parse::<u32>() {
            valid_nums.push(num);
        }

        temp_num.clear();
    }


    if !temp_num.is_empty() {
        if let Ok(num) = temp_num.parse::<u32>() {
            valid_nums.push(num);
        }

        temp_num.clear();
    }

    return valid_nums;
}

fn adjecent_nums(rows: &Vec<&str>, (y, x): (usize, usize)) -> Vec<u32> {
    let top_idx = y.saturating_sub(1);
    let bottom_idx = if y >= rows.len() {
        rows.len().saturating_sub(1)
    } else {
        y + 1
    };

    let top = rows.get(top_idx).and_then(|row| {
        return Some(parse_row(row, x));
    }).unwrap_or(Vec::new());

    let center = rows.get(y).and_then(|row| {
        return Some(parse_row(row, x));
    }).unwrap_or(Vec::new());

    let bottom = rows.get(bottom_idx).and_then(|row| {
        return Some(parse_row(row, x));
    }).unwrap_or(Vec::new());

    let mut result = Vec::new();
    result.extend(top);
    result.extend(center);
    result.extend(bottom);

    return result;
}

pub fn part2_solve(data: &String) -> Result<u32> {
    let rows = data.lines().collect::<Vec<&str>>();
    let mut sum = 0;

    for (row_idx, line) in rows.iter().enumerate() {
        for (char_idx, val) in line.chars().enumerate() {
            match val {
                '*' => {
                    let nums = adjecent_nums(&rows, (row_idx, char_idx));
                    if nums.len() > 1 { 
                        sum += nums.iter().fold(1, |acc, num| acc * num);
                    }
                },
                _ => {}
            }
        }
    }

    return Ok(sum);
}

