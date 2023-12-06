use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

fn is_symbol(char: char) -> bool {
    char == '*'
}

pub fn part_2() {
    let input = read_to_string("src/day_3/input.txt").expect("file exists");
    let graph: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let x_max: i32 = (graph[0].len() - 1) as i32;
    let y_max: i32 = (graph.len() - 1) as i32;
    let mut str_num = "".to_string();
    // let mut answer: i32 = 0;
    let mut connected = false;
    let mut gear: (i32, i32) = (-1, -1);
    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
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
                                gear = (check_y, check_x);
                                true
                            }
                        }
                        false => continue,
                    };
                }
            } else {
                if connected && !str_num.is_empty() {
                    let int = match str_num.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => {
                            continue;
                        }
                    };
                    str_num = "".to_string();
                    connected = false;
                    match gears.contains_key(&gear) {
                        true => {
                            let nums = gears.get_mut(&gear).expect("key exists");
                            nums.push(int);
                            // println!("\ninside\nnum {int}\nanswer {answer}");
                        }
                        false => {
                            let nums = vec![int];
                            match gears.insert(gear, nums) {
                                Some(_) => continue,
                                None => continue,
                            }
                        }
                    }
                }
                str_num = "".to_string();
                connected = false
            };
        }
        if connected {
            let int = match str_num.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    continue;
                }
            };
            str_num = "".to_string();
            connected = false;
            match gears.contains_key(&gear) {
                true => {
                    let nums = gears.get_mut(&gear).expect("key exists");
                    nums.push(int);
                }
                false => {
                    let nums = vec![int];
                    match gears.insert(gear, nums) {
                        Some(_) => continue,
                        None => continue,
                    }
                }
            }
        }

        str_num = "".to_string();
        connected = false
    }
    println!("gears {:?}", gears);
    let answer: i32 = gears
        .iter()
        .map(|gear| {
            if gear.1.len() == 2 {
                gear.1[0] * gear.1[1]
            } else {
                0
            }
        })
        .sum();
    println!("answer {answer}")
}
