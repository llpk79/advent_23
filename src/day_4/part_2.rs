use std::fs::read_to_string;

pub fn part_2() {
    let input = read_to_string("src/day_4/input.txt").expect("file exists");
    let cards: Vec<&str> = input
        .lines()
        .map(|line| line.split(':').last().expect("std fmt"))
        .collect();
    let games: Vec<i32> = cards
        .iter()
        .map(|card| {
            let mut win_mine = card.split('|');
            let winners: Vec<&str> = win_mine
                .next()
                .expect("std fmt")
                .split_ascii_whitespace()
                .collect();
            let mine: Vec<&str> = win_mine
                .next()
                .expect("std fmt")
                .split_ascii_whitespace()
                .collect();
            mine.iter()
                .map(|num| match winners.contains(num) {
                    true => 1_i32,
                    false => 0_i32,
                })
                .sum()
        })
        .collect();
    let mut card_copies: Vec<i32> = vec![1; games.len()];
    for (i, game) in games.iter().enumerate() {
        for j in 1..=*game {
            card_copies[i + j as usize] += card_copies[i];
        }
    }
    let answer: i32 = card_copies.iter().sum();
    println!("answer {:?}", answer)
}
