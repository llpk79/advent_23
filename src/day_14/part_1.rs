use std::collections::HashSet;
use std::fs::read_to_string;

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_14/example.txt").expect("file exists");
    let lines = input.lines();
    let puz_len: isize = lines.clone().last().unwrap().len() as isize;
    let mut rollers: HashSet<(isize, isize)> = HashSet::new();
    let mut stucks: Vec<(isize, isize)> = Vec::new();
    for (i, line) in lines.enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char {
                'O' => {
                    rollers.insert((i as isize, j as isize));
                }
                '#' => stucks.push((i as isize, j as isize)),
                _ => (),
            }
        }
    }
    for y in 0..puz_len {
        stucks.push((puz_len + 1, y));
    }

    for i in 0..puz_len + 1 {
        let mut stoppers: Vec<(isize, isize)> =
            stucks.iter().copied().filter(|rock| rock.1 == i).collect();
        let mut rolled: Vec<(isize, isize)> =
            rollers.iter().copied().filter(|rock| rock.1 == i).collect();
        rolled.sort();
        rolled.reverse();
        if stoppers.is_empty() {
            stoppers.push((puz_len + 1, i));
        }
        let mut start: isize = 0;
        for stop in stoppers {
            // println!("rolled {rolled:?}, stop {stop:?}");
            while !rolled.is_empty() && rolled.last().unwrap().0 < stop.0 {
                let rock = match rolled.pop() {
                    Some(rock) => rock,
                    None => break,
                };
                // println!("\nstop {stop:?}, start {start:?}, rock {rock:?}");
                rollers.remove(&rock);
                rollers.insert((start, stop.1));
                start += 1
            }
            start = stop.0 + 1;
        }
    }

    let answer: u32 = rollers
        .iter()
        .map(|rock| (puz_len - rock.0) as u32)
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("answer {answer}");
    Some(())
}
