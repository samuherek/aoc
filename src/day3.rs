use anyhow::Result;

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
