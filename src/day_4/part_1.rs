use std::fs::read_to_string;

pub fn part_1() {
    let input = read_to_string("src/day_4/input.txt").expect("file exists");
    let cards: Vec<&str> = input
        .lines()
        .map(|line| line.split(':').last().expect("std fmt"))
        .collect();
    let games: Vec<i32> = cards
        .iter()
        .map(|card| {
            let mut win_mine = card.split('|');
            let winners: Vec<&str> = win_mine.next().expect("std fmt").split(' ').collect();
            let mine: Vec<&str> = win_mine.next().expect("std fmt").split(' ').collect();
            mine.iter()
                .map(|num| {
                    if winners.contains(num) && num.parse::<i32>().unwrap_or(-1) != -1 {
                        1_i32
                    } else {
                        0_i32
                    }
                })
                .sum()
        })
        .collect();
    let answer: i32 = games
        .iter()
        .map(|games_won| {
            if *games_won > 0 {
                1 << (games_won - 1)
            } else {
                0
            }
        })
        .sum();
    println!("answer {answer}");
}
