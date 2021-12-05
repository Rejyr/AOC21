fn main() {
    let s = include_str!("input2");
    println!("Part 1: {}", part1(s));
    println!("Part 2: {}", part2(s));
}

fn part1(s: &str) -> i32 {
    #[derive(Default)]
    struct Position {
        horizontal: i32,
        depth: i32,
    }

    let mut pos = Position::default();

    for s in s.lines() {
        let (cmd, num) = s.split_once(" ").unwrap();
        let num: i32 = num.parse().unwrap();

        match cmd {
            "forward" => pos.horizontal += num,
            "up" => pos.depth -= num,
            "down" => pos.depth += num,
            _ => panic!(),
        };
    }

    pos.horizontal * pos.depth
}

fn part2(s: &str) -> i32 {
    #[derive(Default)]
    struct Position {
        x: i32,
        depth: i32,
        aim: i32,
    }

    let mut pos = Position::default();

    for s in s.lines() {
        let (cmd, num) = s.split_once(" ").unwrap();
        let num: i32 = num.parse().unwrap();

        match cmd {
            "forward" => {
                pos.x += num;
                pos.depth += pos.aim * num;
            }
            "up" => pos.aim -= num,
            "down" => pos.aim += num,
            _ => panic!()
        }
    }

    pos.x * pos.depth
}
