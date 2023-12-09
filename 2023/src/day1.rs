use anyhow::Result;



pub fn part1_solve(data: &String) -> Result<String> {
    let lines = data.lines().map(|line| {
        let first_idx = line.find(|c| char::is_digit(c, 10)).unwrap();
        let last_idx = line.rfind(|c| char::is_digit(c, 10)).unwrap();
        let first_char = line.chars().nth(first_idx).unwrap();
        let last_char = line.chars().nth(last_idx).unwrap();
    
        let num = format!("{first_char}{last_char}");

        return num.parse::<u32>().unwrap();
    }).collect::<Vec<u32>>();
    let sum: u32 = lines.into_iter().sum();

    return Ok(sum.to_string());
}


fn word_to_digit(word: &str) -> String {
    match word {
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        _ => "0".to_string()
    }
}

fn get_first_val(val: String, word_nums: &[&str; 9]) -> u32 {
    let mut digits: Vec<String> = vec![];
    let mut temp_start = "".to_string();

    for char in val.chars() {
        if char.is_digit(10)  {
            digits.push(char.to_string());
            break;
        }

        temp_start.push(char);

        let word_res = word_nums.iter().find(|word| temp_start.contains(*word));
        if let Some(word_res) = word_res {
            digits.push(word_to_digit(word_res));
            break;
        }
    }

    return digits.join("").parse::<u32>().unwrap_or(0);
}

fn get_second_val(val: String, word_nums: &[&str; 9]) -> u32 {
    let mut digits: Vec<String> = vec![];
    let mut temp_end = "".to_string();

    for char in val.chars().rev() {
        if char.is_digit(10)  {
            digits.push(char.to_string());
            break;
        }

        temp_end = format!("{}{}", char, temp_end);

        let word_res = word_nums.iter().find(|word| temp_end.contains(*word));
        if let Some(word_res) = word_res {
            digits.push(word_to_digit(word_res));
            break;
        }
    }

    return digits.join("").parse::<u32>().unwrap_or(0);
}

pub fn part2_solve(data: &String) -> Result<String> {
    let word_nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let lines: u32 = data.lines().map(|line| {
        let first_digit = get_first_val(line.to_string(), &word_nums);
        let second_digit = get_second_val(line.to_string(), &word_nums);
        return format!("{}{}", first_digit, second_digit).parse::<u32>().unwrap_or(0);
    }).sum();

    return Ok(lines.to_string());
}
