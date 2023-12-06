use std::fs::read_to_string;

pub fn part1() {
    let example = read_to_string("src/day_1/input_1.txt").expect("File exists");
    let mut total = 0;
    let mut num = "".to_string();
    let mut first: char = ' ';
    let mut last: char = ' ';
    let mut first_set: bool = false;
    for char in example.chars() {
        match char.is_ascii_digit() {
            true => {
                if first_set {
                    last = char
                } else {
                    first = char;
                    first_set = true
                }
            }
            false => match char == '\n' {
                true => {
                    num.push(first);
                    if last == ' ' {
                        num.push(first)
                    } else {
                        num.push(last);
                    }
                    let int: i32 = num.parse::<i32>().expect("Should be num.");
                    total += int;
                    num = "".to_string();
                    first = ' ';
                    last = ' ';
                    first_set = false;
                }
                false => continue,
            },
        }
    }
    println!("total {total}")
}
