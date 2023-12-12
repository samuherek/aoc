use std::collections::HashMap;

use anyhow::Result;

pub fn part1_solve(data: &String) -> Result<u32> {
    let sum = data.lines().map(|line| {
        let (_, games) = line.split_once(":").unwrap_or(("", ""));

        return games.split_once("|").and_then(|(target, guess)| {
            let target_nums: Vec<u32> = target.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .filter(|x| x > &0)
                .collect();
            let guess_nums: Vec<u32> = guess.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .filter(|x| x > &0)
                .collect();
            
            let winners = target_nums.iter()
                .filter(|n| guess_nums.iter().find(|x| x == n).is_some())
                .count();
             
            // 0, 1, 2, 4, 8
            // 0, 1, 2, 3, 4
            let result = match winners {
                0 => 0,
                1 => 1,
                len => 1 << len - 1
            };

            return Some(result);
        }).unwrap_or(0);
    }).sum::<u32>();

    return Ok(sum);
}

pub fn part2_solve(data: &String) -> Result<usize> {
    let d = data.lines().enumerate().fold(HashMap::new(), |mut map, (game_idx, line)| {
        let (_, game) = line.split_once(":").unwrap_or(("", ""));
        let _ = game.split_once("|").and_then(|(target, guess)| {
            let target_nums: Vec<u32> = target.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .filter(|x| x > &0)
                .collect();
            let guess_nums: Vec<u32> = guess.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .filter(|x| x > &0)
                .collect();
            
            let curr_cards_count: usize = map.entry(game_idx + 1).or_insert(1).clone();
            let winner_count = target_nums.iter()
                .filter(|n| guess_nums.iter().find(|x| x == n).is_some())
                .count();

            for c in 1..=winner_count {
                *map.entry(game_idx + 1 + c).or_insert(1) += curr_cards_count;
            }

            // get the count num of the winners for an index
            // Add one card for from index + 1 until ..winners
            // Next itteration find winners + double it for index card count
            //
            return Some(());
        });

        return map;
    });


    return Ok(d.values().sum());
}
