use std::num::ParseIntError;

pub fn parse_number_list(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.lines().map(|s| s.parse()).collect()
}

pub fn if_greater((a, b): (i32, i32)) -> i32 {
    if b > a { 1 } else { 0 }
}