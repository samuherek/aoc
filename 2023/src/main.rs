//mod day1; 
//mod day2;
mod day3;

use std::fs;
use std::path::PathBuf;

fn main() {
    let data = fs::read_to_string(PathBuf::from("data/day3")).unwrap();
    let result = day3::part2_solve(&data).unwrap();

    //println!("this is the input\n{data}");
    println!("result is:\n{result}");
}
