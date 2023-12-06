use anyhow::Result;

const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

//Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn part1_solve(data: &String) -> Result<u32> {
    let sum = data.lines().map(|line| {
        let (game_name, game) = line.split_once(":").unwrap_or(("", ""));
        let (_, id) = game_name.split_once(" ").unwrap_or(("", "0"));
        let id = id.parse::<u32>().unwrap_or(0);

        let game_passed = game.split(";").fold(false, |_, round| {
            let mut passed = true;

            let draws = round.trim().split(",").map(|v| {
                let (num, name) = v.trim().split_once(" ").unwrap_or(("0", ""));
                return (num.parse::<u32>().unwrap_or(0), name);
            }).collect::<Vec<(u32, &str)>>();

            for draw in draws.iter() {
                match draw {
                    (num, "red") if num <= &&R => {},
                    (num, "green") if num <= &&G => {},
                    (num, "blue") if num <= &&B => {},
                    _ => {
                        passed = false;
                        break;
                    }
                };
            }

            return passed;
        });

        println!("line: {line}");
        println!("game id: {id}, game passed: {game_passed}");
    
        return match game_passed {
            true => id,
            false => 0
        };
    }).sum();

    return Ok(sum);
}
