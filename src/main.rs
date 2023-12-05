mod day1; 

use std::fs;
use std::path::PathBuf;

fn main() {
    let data = fs::read_to_string(PathBuf::from("input")).unwrap();
    let result = day1::part1_solve(&data).unwrap();

    println!("this is the input\n{data}");
    println!("result is:\n{result}");
}
