//mod day1; 
mod day2;

use std::fs;
use std::path::PathBuf;

fn main() {
    let data = fs::read_to_string(PathBuf::from("data/day2")).unwrap();
    let result = day2::part1_solve(&data).unwrap();

    //println!("this is the input\n{data}");
    println!("result is:\n{result}");
}
