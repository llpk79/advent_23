use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn roll_east(puz_len: isize, stucks: &mut [(isize, isize)], rollers: &mut HashSet<(isize, isize)>) {
    for i in 0..puz_len {
        let mut stoppers: Vec<(isize, isize)> =
            stucks.iter().copied().filter(|rock| rock.0 == i).collect();
        let mut rolled: Vec<(isize, isize)> =
            rollers.iter().copied().filter(|rock| rock.0 == i).collect();
        if rolled.is_empty() {
            continue;
        }
        rolled.sort();
        rolled.reverse();
        stoppers.sort();
        // println!("i {i} stoppers {stoppers:?} puz {puz_len}");
        for stop in stoppers {
            while !rolled.is_empty() && rolled.last().unwrap().1 < stop.1 {
                //
                let rock = match rolled.pop() {
                    Some(rock) => rock,
                    None => break,
                };
                rollers.remove(&rock);
                let mut start = stop.1 - 1; //
                                            // println!("\nstop {stop:?}, start {start:?}, rock {rock:?}");

                while !rollers.insert((stop.0, start)) {
                    start -= 1 //
                }
            }
        }
    }
}

fn roll_south(
    puz_len: isize,
    stucks: &mut [(isize, isize)],
    rollers: &mut HashSet<(isize, isize)>,
) {
    for i in 0..puz_len {
        let stoppers: Vec<(isize, isize)> =
            stucks.iter().copied().filter(|rock| rock.1 == i).collect();
        let mut rolled: Vec<(isize, isize)> =
            rollers.iter().copied().filter(|rock| rock.1 == i).collect();
        if rolled.is_empty() {
            continue;
        }
        rolled.sort();
        rolled.reverse();
        // println!("i {i} stoppers {stoppers:?} puz {puz_len}");
        for stop in stoppers {
            while !rolled.is_empty() && rolled.last().unwrap().0 < stop.0 {
                //
                let rock = match rolled.pop() {
                    Some(rock) => rock,
                    None => break,
                };
                rollers.remove(&rock);
                let mut start = stop.0 - 1; //
                                            // println!("\nstop {stop:?}, start {start:?}, rock {rock:?}");

                while !rollers.insert((start, stop.1)) {
                    start -= 1 //
                }
            }
        }
    }
}
fn roll_west(puz_len: isize, stucks: &mut [(isize, isize)], rollers: &mut HashSet<(isize, isize)>) {
    for i in 0..puz_len + 1 {
        let mut stoppers: Vec<(isize, isize)> =
            stucks.iter().copied().filter(|rock| rock.0 == i).collect();
        let mut rolled: Vec<(isize, isize)> =
            rollers.iter().copied().filter(|rock| rock.0 == i).collect();
        if rolled.is_empty() {
            continue;
        }
        rolled.sort();
        rolled.reverse();
        stoppers.sort();
        // println!("stoppers {stoppers:?}");
        // stoppers.reverse();
        let mut start: isize = 0; //
        for stop in stoppers {
            // println!("rolled {rolled:?}, stop {stop:?}");
            while !rolled.is_empty() && rolled.last().unwrap().1 < stop.1 {
                let rock = match rolled.pop() {
                    Some(rock) => rock,
                    None => break,
                };
                // println!("\nstop {stop:?}, start {start:?}, rock {rock:?}");
                rollers.remove(&rock);

                while !rollers.insert((stop.0, start)) {
                    start += 1 //
                }
                start += 1 //
            }
            start = stop.1 + 1; //
        }
    }
}

fn roll_north(
    puz_len: isize,
    stucks: &mut [(isize, isize)],
    rollers: &mut HashSet<(isize, isize)>,
) {
    for i in 0..puz_len + 1 {
        let stoppers: Vec<(isize, isize)> =
            stucks.iter().copied().filter(|rock| rock.1 == i).collect();
        let mut rolled: Vec<(isize, isize)> =
            rollers.iter().copied().filter(|rock| rock.1 == i).collect();
        if rolled.is_empty() {
            continue;
        }
        rolled.sort();
        rolled.reverse();
        let mut start: isize = 0; //
        for stop in stoppers {
            // println!("rolled {rolled:?}, stop {stop:?}");
            while !rolled.is_empty() && rolled.last().unwrap().0 < stop.0 {
                //
                let rock = match rolled.pop() {
                    Some(rock) => rock,
                    None => break,
                };
                // println!("\nstop {stop:?}, start {start:?}, rock {rock:?}");
                rollers.remove(&rock);

                while !rollers.insert((start, stop.1)) {
                    start += 1 //
                }
                start += 1 //
            }
            start = stop.0 + 1; //
        }
    }
}

pub fn part_2() -> Option<()> {
    let input = read_to_string("src/day_14/input.txt").expect("file exists");
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
    // println!("stuck {stucks:?}");
    for y in 0..puz_len {
        stucks.push((puz_len, y));
        stucks.push((-1, y));
        stucks.push((y, -1));
        stucks.push((y, puz_len))
    }
    let mut cycle: HashMap<Vec<(isize, isize)>, usize> = HashMap::new();
    for i in 1..=1_000 {
        roll_north(puz_len, &mut stucks, &mut rollers);
        roll_west(puz_len, &mut stucks, &mut rollers);
        roll_south(puz_len, &mut stucks, &mut rollers);
        roll_east(puz_len, &mut stucks, &mut rollers);
        let roll_copy = rollers.clone();
        match cycle.insert(roll_copy.into_iter().collect::<Vec<(isize, isize)>>(), i) {
            Some(v) => {
                if 1_000_000_000 % i - v == 0 {
                    println!("beep {i}");
                    break;
                }
            }
            None => continue,
        };
        // println!("dif {dif:?}");
    }
    // println!(
    //     "rollers {:?}",
    //     rollers.iter().sorted_by(|a, b| a.1.cmp(&b.1))
    // );
    let answer: u32 = rollers
        .iter()
        .map(|rock| (puz_len - rock.0) as u32)
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("answer {answer}");
    Some(())
}

// too low
// 104644 too high

// ? 104618
