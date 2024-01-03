use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, ch| ((acc + ch as u32) * 17) % 256)
}

pub fn part_2() -> Option<()> {
    let input = read_to_string("src/day_15/input.txt").expect("file exists");
    let mut hash_map: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();
    for seq in input.split(',') {
        match seq.contains('-') {
            true => {
                let (code, _) = seq.split_once('-').expect("std fmt");
                match hash_map.get_mut(&hash(code)) {
                    Some(m) => match m.iter().find_position(|c| c.0 == code) {
                        Some(i) => {
                            m.remove(i.0);
                        }
                        None => continue,
                    },
                    None => continue,
                }
            }
            false => {
                let (code, len) = seq.split_once('=').expect("std fmt");
                let len_num = len.parse::<u32>().expect("digit");
                hash_map
                    .entry(hash(code))
                    .and_modify(|m| match m.clone().iter().find_position(|c| c.0 == code) {
                        Some(i) => {
                            m.remove(i.0);
                            m.insert(i.0, (code, len_num));
                        }
                        None => m.push((code, len_num)),
                    })
                    .or_insert(vec![(code, len_num)]);
            }
        }
    }
    let answer = hash_map.iter().fold(0, |acc, m| {
        acc + m
            .1
            .iter()
            .enumerate()
            .map(|(i, seq)| (m.0 + 1) * (i as u32 + 1) * seq.1)
            .sum::<u32>()
    });

    println!("answer {answer}");
    Some(())
}
