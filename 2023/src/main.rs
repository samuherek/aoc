//mod day1; 
//mod day2;
//mod day3;
//mod day4;
mod day5;

use std::fs;
use std::path::PathBuf;

fn main() {
    let data = fs::read_to_string(PathBuf::from("input")).unwrap();
    let result = day5::part1_solve(&data);

    //println!("this is the input\n{data}");
    println!("result is:\n{result}");
}
