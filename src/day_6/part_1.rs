use std::fs::read_to_string;
use std::iter::zip;

pub fn part_1() -> Option<i64> {
    let input = read_to_string("src/day_6/input.txt").expect("file exists");
    let mut lines = input.lines();
    let times: Vec<i64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();
    let distances: Vec<i64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();
    let mut total_wins: Vec<i64> = Vec::new();
    for (time, distance) in zip(times, distances) {
        let mut sec: i64 = 1;
        let mut wins: i64 = 0;
        loop {
            let millimeters = sec * (time - sec);
            if millimeters > distance {
                wins += 1
            }
            if millimeters <= distance && wins != 0 {
                break;
            }
            sec += 1
        }
        total_wins.push(wins)
    }
    println!("answer {:?}", total_wins.iter().product::<i64>());
    Some(total_wins.iter().product())
}
