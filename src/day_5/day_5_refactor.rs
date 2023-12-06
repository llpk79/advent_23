use std::fs::read_to_string;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct FarmMap {
    range: Range<i64>,
    adjustment: i64,
}

impl FarmMap {
    pub fn new(start: i64, end: i64, adjustment: i64) -> FarmMap {
        let range = start..end;
        Self { range, adjustment }
    }
}

fn reverse_adjust(map: &FarmMap, seed: i64) -> Option<i64> {
    let rev = seed - map.adjustment;
    if map.range.contains(&rev) {
        Some(rev)
    } else {
        None
    }
}

fn check_location(location: i64, map: &[Vec<FarmMap>], seed_ranges: &Vec<Range<i64>>) -> bool {
    let mut check_seed = location;
    for map_vec in map.iter().rev() {
        'inner: for map in map_vec {
            match reverse_adjust(map, check_seed) {
                Some(num) => {
                    check_seed = num;
                    break 'inner;
                }
                None => continue,
            }
        }
    }
    for range in seed_ranges {
        if range.contains(&check_seed) {
            println!("answer {location}");
            return true;
        }
    }
    false
}

pub fn part_2_refactor() -> Option<i64> {
    let input = read_to_string("src/day_5/input.txt").expect("file exists");
    let mut lines = input.lines();
    let seeds: Vec<i64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();
    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    for i in (0..seeds.len() - 1).step_by(2) {
        seed_ranges.push(seeds[i]..seeds[i] + seeds[i + 1] - 1)
    }
    let mut map_map: Vec<Vec<FarmMap>> = Vec::new();
    let mut vec = Vec::new();
    for line in lines {
        if line.is_empty() {
            if !vec.is_empty() {
                map_map.push(vec.clone());
            }
            continue;
        }
        if line.ends_with(':') {
            vec.clear();
            continue;
        }
        let nums: Vec<i64> = line
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        vec.push(FarmMap::new(nums[1], nums[1] + nums[2], nums[0] - nums[1]))
    }

    let mut location: i64 = 0;
    loop {
        if check_location(location, &map_map, &seed_ranges) {
            break;
        }
        location += 1
    }
    Some(location)
}
