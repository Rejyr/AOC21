use itertools::Itertools;
use AOC21::{if_greater, parse_number_list};

fn main() {
    let s = include_str!("input1");
    let nums = parse_number_list(s).unwrap();
    let answer: i32 = nums.iter().tuple_windows().map(if_greater).sum();
    println!("{}", answer);
}