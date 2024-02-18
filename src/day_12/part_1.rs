// use std::collections::HashMap;
// use std::fs::read_to_string;
//
// #[derive(Debug)]
// struct Record {
//     sequence: String,
//     key: Vec<u32>,
// }
//
// fn solve(
//     record: &Record,
//     map: &mut HashMap<(usize, usize, usize), i32>,
//     seq_index: usize,
//     key_index: usize,
//     block_size: usize,
// ) -> i32 {
//     if let Some(num) = map.get(&(seq_index, key_index, block_size)) {
//         return *num;
//     }
//     if seq_index == record.sequence.len() {
//         if key_index == record.key.len() && block_size == 0 {
//             return 1;
//         }
//         return 0;
//     }
//     if key_index > record.key.len() {
//         return 0;
//     }
//     let mut answer: i32 = 0;
//     match &record.sequence[seq_index..seq_index + 1] {
//         "." => {
//             if block_size == 0 {
//                 if key_index < record.key.len() {
//                     answer += solve(
//                         record,
//                         map,
//                         seq_index + 1,
//                         key_index + 1,
//                         record.key[key_index] as usize,
//                     )
//                 }
//                 answer += solve(record, map, seq_index + 1, key_index, 0)
//             }
//         }
//         "#" => {
//             if block_size > 0 {
//                 answer += solve(record, map, seq_index + 1, key_index, block_size - 1)
//             }
//         }
//         "?" => {
//             if block_size == 0 {
//                 if key_index < record.key.len() {
//                     answer += solve(
//                         record,
//                         map,
//                         seq_index + 1,
//                         key_index + 1,
//                         record.key[key_index] as usize,
//                     )
//                 };
//                 answer += solve(record, map, seq_index + 1, key_index, 0);
//             }
//             answer += solve(record, map, seq_index + 1, key_index, block_size - 1);
//         }
//         _ => panic!(),
//     }
//     map.insert((seq_index, key_index, block_size), answer);
//     answer
// }
//
// pub fn part_1() -> Option<()> {
//     let input = read_to_string("src/day_12/input.txt").expect("file exists");
//     let lines = input.lines();
//     let records: Vec<Record> = lines
//         .map(|line| {
//             let split = line.split_once(' ').expect("std fmt");
//             Record {
//                 sequence: (".".to_owned() + split.0).to_string(),
//                 key: split.1.chars().filter_map(|c| c.to_digit(10)).collect(),
//             }
//         })
//         .collect();
//
//     let mut total = 0;
//     for record in records {
//         let map: &mut HashMap<(usize, usize, usize), i32> = &mut HashMap::new();
//         let this = solve(&record, map, 0, 0, 0);
//         // println!("record {record:?} {this}");
//         total += this
//     }
//     println!("total {total}");
//
//     Some(())
// }
//
// // 7526 too high
// // 4407 too low
//
// // 7515 nope
// // *** 7110 *** wtf

use itertools::Itertools;
use std::fs::read_to_string;
fn solve(spring: &str, counts: Vec<usize>) -> usize {
    let spring = format!(".{}", spring.trim_end_matches('.'));
    let spring = spring.chars().collect_vec();

    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for count in counts {
        let mut n_dp = vec![0; spring.len() + 1];
        let mut chunk = 0;

        for (i, &c) in spring.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            if chunk >= count && spring[i - count] != '#' {
                n_dp[i + 1] += dp[i - count];
            }
        }

        dp = n_dp;
    }

    *dp.last().unwrap()
}

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_12/input.txt").expect("file exists");
    let answer = input
        .lines()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            solve(spring, counts)
        })
        .sum::<usize>();
    println!("answer {answer}");
    Some(())
}
