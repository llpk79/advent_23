use std::fs::read_to_string;
use std::iter::zip;

fn is_symbol(char: char) -> bool {
    let mut symbol = true;
    if char.is_ascii_digit() {
        symbol = false
    }
    if char == '.' {
        symbol = false
    }
    symbol
}

pub fn part_1() {
    let input = read_to_string("src/day_3/input.txt").expect("file exists");
    let graph: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let x_max: i32 = (graph[0].len() - 1) as i32;
    let y_max: i32 = (graph.len() - 1) as i32;
    let mut str_num = "".to_string();
    let mut answer: i32 = 0;
    let mut connected = false;
    for y in 0..=y_max {
        for x in 0..=x_max {
            let current = graph[y as usize][x as usize];
            if current.is_ascii_digit() {
                str_num.push(current);
                for (i, j) in zip([-1, 0, 1, -1, 1, -1, 0, 1], [-1, -1, -1, 0, 0, 1, 1, 1]) {
                    let check_y = if ((y + i) >= 0) && ((y + i) <= y_max) {
                        y + i
                    } else {
                        y
                    };
                    let check_x = if ((x + j) >= 0) && ((x + j) <= x_max) {
                        x + j
                    } else {
                        x
                    };
                    match is_symbol(graph[check_y as usize][check_x as usize]) {
                        true => {
                            connected = {
                                // println!("sym {}", graph[check_y as usize][check_x as usize]);
                                true
                            }
                        }
                        false => continue,
                    };
                }
            } else {
                if !str_num.is_empty() {
                    println!("num {str_num} conn {connected}");
                }
                if connected {
                    let int = match str_num.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => {
                            continue;
                        }
                    };
                    answer += int;
                }
                str_num = "".to_string();
                connected = false
            };
        }
        if !str_num.is_empty() {
            println!("num {str_num} conn {connected}");
        }
        if connected {
            let int = match str_num.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    continue;
                }
            };
            answer += int;
        }

        str_num = "".to_string();
        connected = false
    }
    println!("answer {answer}")
}
