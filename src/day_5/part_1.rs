use std::collections::HashMap;
use std::fs::read_to_string;

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

pub fn part_1() -> Option<i64> {
    // let answer: i64 = 0;
    let input = read_to_string("src/day_5/input.txt").expect("file exists");
    let mut lines = input.lines();
    let seeds: Vec<i64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();
    println!("seeds {seeds:?}");
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
        println!("line {line}");
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
    let mut locations: Vec<i64> = Vec::new();
    for seed in seeds {
        let mut super_seed = seed;
        println!("\nseed start {super_seed}");
        let to_soil = map_map.get("seed-to-soil")?;
        for map in to_soil {
            if seed >= map.source && seed < map.source + map.range {
                super_seed += map.adjustment;
                break;
            }
        }
        println!("seed soil {super_seed}");

        let to_fert = map_map.get("soil-to-fertilizer")?;
        for map in to_fert {
            if super_seed >= map.source && super_seed < map.source + map.range {
                super_seed += map.adjustment;
                break;
            }
        }
        println!("seed fert {super_seed}");

        let to_water = map_map.get("fertilizer-to-water")?;
        for map in to_water {
            if (map.source..map.source + map.range).contains(&super_seed) {
                super_seed += map.adjustment;
                break;
            }
        }
        println!("seed water {super_seed}");

        let to_light = map_map.get("water-to-light")?;
        for map in to_light {
            if super_seed >= map.source && super_seed < map.source + map.range {
                super_seed += map.adjustment;
                break;
            }
        }
        println!("seed light {super_seed}");

        let to_temp = map_map.get("light-to-temperature")?;
        for map in to_temp {
            if super_seed >= map.source && super_seed < map.source + map.range {
                super_seed += map.adjustment;
                break;
            }
        }
        println!("seed temp {super_seed}");

        let to_humid = map_map.get("temperature-to-humidity")?;
        for map in to_humid {
            if super_seed >= map.source && super_seed < map.source + map.range {
                super_seed += map.adjustment;
                break;
            }
        }
        println!("seed humid {super_seed}\n");

        let to_local = map_map.get("humidity-to-location")?;
        for map in to_local {
            if super_seed >= map.source && super_seed < map.source + map.range {
                super_seed += map.adjustment;
                break;
            }
        }
        locations.push(super_seed)
    }

    println!("answer {:?}", *locations.iter().min()?);
    Some(*locations.iter().min()?)
}
