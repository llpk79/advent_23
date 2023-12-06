use std::cmp::min;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct FarmMap {
    source: i64,
    dest: i64,
    range: i64,
    adjustment: i64,
}

impl FarmMap {
    pub fn new() -> FarmMap {
        let source: i64 = 0;
        let dest: i64 = 0;
        let range: i64 = 0;
        let adjustment: i64 = 0;
        Self {
            source,
            dest,
            range,
            adjustment,
        }
    }
}

fn adjust_seed(map: &FarmMap, seed: i64) -> Option<i64> {
    if (map.source..map.source + map.range).contains(&seed) {
        Some(map.adjustment)
    } else {
        None
    }
}

pub fn part_2() -> Option<i64> {
    let mut answer: i64 = i64::MAX;
    let input = read_to_string("src/day_5/input.txt").expect("file exists");
    let mut lines = input.lines();
    let seeds: Vec<i64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();
    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    for i in (0..seeds.len() - 1).step_by(2) {
        seed_ranges.push(seeds[i]..seeds[i] + seeds[i + 1] - 1)
    }
    let mut map_map: HashMap<&str, Vec<FarmMap>> = HashMap::new();
    let mut name: &str = "";
    for line in lines {
        let mut farm_map = FarmMap::new();
        if line.is_empty() {
            continue;
        }
        if line.ends_with(':') {
            name = line.split_ascii_whitespace().next()?;
            continue;
        }
        let nums: Vec<i64> = line
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .collect();
        farm_map.dest = nums[0];
        farm_map.source = nums[1];
        farm_map.range = nums[2];
        farm_map.adjustment = farm_map.dest - farm_map.source;
        match map_map.contains_key(&name) {
            true => {
                let vec = map_map.get_mut(&name)?;
                vec.push(farm_map);
            }
            false => {
                map_map.insert(name, vec![farm_map]);
            }
        }
    }
    // dbg!(&map_map);
    let to_soil = map_map.get("seed-to-soil")?;
    let to_fert = map_map.get("soil-to-fertilizer")?;
    let to_water = map_map.get("fertilizer-to-water")?;
    let to_light = map_map.get("water-to-light")?;
    let to_temp = map_map.get("light-to-temperature")?;
    let to_humid = map_map.get("temperature-to-humidity")?;
    let to_local = map_map.get("humidity-to-location")?;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let mut super_seed = seed;

            for map in to_soil {
                match adjust_seed(map, super_seed) {
                    Some(num) => {
                        super_seed += num;
                        break;
                    }
                    None => continue,
                }
            }
            // println!("seed soil {super_seed}");

            for map in to_fert {
                match adjust_seed(map, super_seed) {
                    Some(num) => {
                        super_seed += num;
                        break;
                    }
                    None => continue,
                }
            }
            // println!("seed fert {super_seed}");

            for map in to_water {
                match adjust_seed(map, super_seed) {
                    Some(num) => {
                        super_seed += num;
                        break;
                    }
                    None => continue,
                }
            }
            // println!("seed water {super_seed}");

            for map in to_light {
                match adjust_seed(map, super_seed) {
                    Some(num) => {
                        super_seed += num;
                        break;
                    }
                    None => continue,
                }
            }
            // println!("seed light {super_seed}");

            for map in to_temp {
                match adjust_seed(map, super_seed) {
                    Some(num) => {
                        super_seed += num;
                        break;
                    }
                    None => continue,
                }
            }
            // println!("seed temp {super_seed}");

            for map in to_humid {
                match adjust_seed(map, super_seed) {
                    Some(num) => {
                        super_seed += num;
                        break;
                    }
                    None => continue,
                }
            }
            // println!("seed humid {super_seed}\n");

            for map in to_local {
                match adjust_seed(map, super_seed) {
                    Some(num) => {
                        super_seed += num;
                        break;
                    }
                    None => continue,
                }
            }
            answer = min(answer, super_seed);
        }
    }

    println!("answer {answer}");
    Some(answer)
}
