use std::fs::read_to_string;

const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

pub fn part_1() {
    let input = read_to_string("src/day_2/input.txt").expect("file exists");
    let mut answer: i32 = 0;
    for line in input.lines() {
        let mut possible = true;
        let mut game_dice = line.split(':');
        let game = &game_dice
            .next()
            .expect("std fmt")
            .split(' ')
            .last()
            .expect("num")
            .parse::<i32>()
            .expect("num");
        let dice = game_dice.last().expect("std fmt");
        println!("\ngame {game}\ndice {dice}");
        let mut digit = "".to_string();
        'inner: for char in dice.chars() {
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
                        if int > RED_MAX {
                            println!("too much {char}, {int}");
                            possible = false;
                            break 'inner;
                        }
                        digit = "".to_string();
                    }
                    'b' => {
                        if int > BLUE_MAX {
                            println!("too much {char}, {int}");
                            possible = false;
                            break 'inner;
                        }
                        digit = "".to_string();
                    }
                    'g' => {
                        if int > GREEN_MAX {
                            println!("too much {char}, {int}");
                            possible = false;
                            break 'inner;
                        }
                        digit = "".to_string();
                    }
                    _ => continue,
                }
            }
            if char == ',' || char == ';' {
                digit = "".to_string();
            }
        }
        if possible {
            println!("boop, {}", game);
            answer += game;
        }
    }
    println!("Answer {answer}")
}
