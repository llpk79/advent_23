use std::cmp::max;
use std::fs::read_to_string;

pub fn part_2() {
    let input = read_to_string("src/day_2/input.txt").expect("file exists");
    let mut answer: i32 = 0;
    for line in input.lines() {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        let game_dice = line.split(':');
        let dice = game_dice.last().expect("std fmt");
        let mut digit = "".to_string();
        for char in dice.chars() {
            if char.is_ascii_digit() {
                digit.push(char)
            };
            if char.is_ascii_alphabetic() {
                let int = match digit.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        // println!("ERROR with paring {digit}");
                        continue;
                    }
                };
                match char {
                    'r' => {
                        red = max(red, int);
                        digit = "".to_string();
                    }
                    'b' => {
                        blue = max(blue, int);
                        digit = "".to_string();
                    }
                    'g' => {
                        green = max(green, int);
                        digit = "".to_string();
                    }
                    _ => continue,
                }
            }
            if char == ',' || char == ';' {
                digit = "".to_string();
            }
        }
        println!("boop, {}", (red * blue * green));
        answer += red * blue * green;
    }
    println!("Answer {answer}")
}
