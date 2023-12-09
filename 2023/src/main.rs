//mod day1; 
//mod day2;
//mod day3;
mod day4;

use std::fs;
use std::path::PathBuf;

fn main() {
    let data = fs::read_to_string(PathBuf::from("data/day4")).unwrap();
    let result = day4::part1_solve(&data).unwrap();

    //println!("this is the input\n{data}");
    println!("result is:\n{result}");
}
