use itertools::Itertools;
use AOC21::{if_greater, parse_number_list};

fn main() {
    let s = include_str!("input1");
    let nums = parse_number_list(s).unwrap();
    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&nums));
}

fn part1(nums: &Vec<i32>) -> i32 {
    nums.iter().copied().tuple_windows().map(if_greater).sum()
}

fn part2(nums: &Vec<i32>) -> i32 {
    nums.iter().tuple_windows().map(|(a, b, c)| a + b + c).tuple_windows().map(if_greater).sum()
}