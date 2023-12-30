use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn hash(input: &str) -> u16 {
    let mut output = 0;
    for char in input.chars() {
        output += u32::from(char);
        output *= 17;
        output %= 256;
    }
    output as u16
}

pub fn part_2() -> Option<()> {
    let input = read_to_string("src/day_15/input.txt").expect("file exists");
    let mut hash_map: HashMap<u16, Vec<(&str, u16)>> = HashMap::new();
    for seq in input.split(',') {
        match seq.contains('-') {
            true => {
                let (code, _) = seq.split_once('-').expect("std fmt");
                let code_hash = hash(code);
                match hash_map.get_mut(&code_hash) {
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
                let code_hash = hash(code);
                let len_num = len.parse::<u16>().expect("digit");
                hash_map
                    .entry(code_hash)
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
    let answer: u32 = hash_map
        .iter()
        .map(|m| {
            let mut power = 0;
            for (i, seq) in m.1.iter().enumerate() {
                power += ((m.0 + 1) * (i as u16 + 1) * seq.1) as u32
            }
            power
        })
        .sum();

    println!("answer {answer}");
    Some(())
}
