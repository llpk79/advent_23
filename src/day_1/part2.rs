use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

pub fn part2() {
    let spelled = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let num_map: HashMap<&str, char> = spelled
        .iter()
        .enumerate()
        .map(|(i, num)| (*num, char::from_digit(i as u32, 10).expect("char is digit")))
        .collect();
    let word_lens = [3, 4, 5];

    let mut total = 0;
    let mut str_num = "".to_string();
    let mut first: char = ' ';
    let mut last: char = ' ';
    let mut first_set: bool = false;

    let example_2 = read_to_string("src/day_1/input_1.txt").expect("file exists");

    for line in example_2.lines() {
        if line.is_empty() {
            break;
        }
        // println!("line {line}");
        let line_len = line.len();
        for i in 0..line_len {
            let char = char::from_str(&line[i..i + 1]).expect("is char");
            match char.is_ascii_digit() {
                true => {
                    if first_set {
                        last = char
                    } else {
                        first = char;
                        first_set = true
                    }
                }
                false => {
                    for word_len in word_lens {
                        if i as u32 + word_len < line_len as u32 + 1 {
                            let word = &line[i..i + word_len as usize];
                            match num_map.get(word) {
                                Some(char) => {
                                    if first_set {
                                        last = *char;
                                    } else {
                                        first = *char;
                                        first_set = true
                                    }
                                }
                                None => continue,
                            }
                        }
                    }
                }
            }
        }

        str_num.push(first);
        if last == ' ' {
            str_num.push(first)
        } else {
            str_num.push(last);
        }
        // println!("num {str_num}");
        let int: i32 = str_num.parse::<i32>().expect("Should be num.");
        total += int;

        str_num = "".to_string();
        first = ' ';
        last = ' ';
        first_set = false;
    }

    println!("total {total}")
}
