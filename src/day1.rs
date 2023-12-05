use anyhow::Result;



pub fn part1_solve(data: &String) -> Result<String> {
    let lines = data.lines().into_iter().map(|line| {
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

pub fn part2_solve(data: &String) -> Result<String> {
    let lines = data.lines().into_iter().map(|line| {
           
    });

    return Ok("".to_string());
}
