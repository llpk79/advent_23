use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_8/input.txt").expect("file exists");
    let mut lines = input.lines();
    let pattern = lines.next().expect("std fmt");
    lines.next(); // blank line
    let mut left_map: HashMap<&str, &str> = HashMap::new();
    let mut right_map: HashMap<&str, &str> = HashMap::new();
    for line in lines {
        let key = line.split_once(' ')?.0;
        let left = line.split_once('(')?.1.split_once(',')?.0;
        let right = line.split_once(", ")?.1.split_once(')')?.0;
        left_map.insert(key, left).or(Some("ok"));
        right_map.insert(key, right).or(Some("ok"));
    }
    let mut start = "AAA";
    let mut steps = 0;
    'outer: loop {
        for char in pattern.chars() {
            match char {
                'R' => start = right_map.get(start)?,
                'L' => start = left_map.get(start)?,
                _ => panic!("should not happen"),
            }
            steps += 1;
            if start == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("answer {steps}");
    Some(())
}
