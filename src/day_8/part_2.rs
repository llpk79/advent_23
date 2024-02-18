use std::collections::HashMap;
use std::fs::read_to_string;

fn greatest_common_denominator(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        greatest_common_denominator(y, x % y)
    }
}

fn least_common_multiple(x: usize, y: usize) -> usize {
    x * y / greatest_common_denominator(x, y)
}

pub fn part_2() -> Option<()> {
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

    let starts: Vec<&&str> = left_map.keys().filter(|key| key.ends_with('A')).collect();
    let mut steps: Vec<usize> = Vec::new();
    let mut check: &&str;
    let mut ends: Vec<&str> = Vec::new();
    let mut step: usize = 1;

    for start in starts {
        check = start;
        'outer: loop {
            for char in pattern.chars() {
                match char {
                    'R' => check = right_map.get(*check)?,
                    'L' => check = left_map.get(*check)?,
                    _ => panic!("should not happen"),
                }
                match check.ends_with('Z') {
                    true => {
                        ends.push(check);
                        break 'outer;
                    }
                    false => {
                        step += 1;
                        continue;
                    }
                }
            }
        }
        steps.push(step);
        step = 1;
    }

    let mut finishes = steps.iter();
    let first = *finishes.next().unwrap();
    let second = *finishes.next().unwrap();
    let mut answer: usize = least_common_multiple(first, second);
    while let Some(&next) = finishes.next() {
        answer = least_common_multiple(answer, next)
    }

    println!("answer {answer:?}");
    Some(())
}
