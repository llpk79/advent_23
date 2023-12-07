use std::fs::read_to_string;
use std::io::SeekFrom::Start;

fn new_record(sec: i64, time: i64, dist: i64) -> bool {
    sec * (time - sec) >= dist
}

pub fn part_2() -> Option<i64> {
    let input = read_to_string("src/day_6/input.txt").expect("file exists");
    let mut lines = input.lines();
    let time: i64 = lines
        .next()?
        .chars()
        .filter(|d| d.is_ascii_digit())
        .collect::<String>()
        // .fold("".to_string(), |s, c| format!("{s}{c}"))
        .parse()
        .unwrap();
    let distance: i64 = lines
        .next()?
        .chars()
        .filter(|d| d.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    println!("times {:?}\ndistances {:?}", time, distance);
    let mut low: i64 = 0;
    let mut high: i64 = time / 2;
    let mut mid: i64;
    let mut steps = 0;
    while low + 1 < high {
        mid = (low + high) / 2;
        match new_record(mid, time, distance) {
            true => high = mid,
            false => low = mid,
        }
        steps += 1
    }
    println!("low {low} high {high}");
    assert_eq!(low + 1, high);
    assert!(new_record(high, time, distance));
    assert!(!new_record(low, time, distance));
    let last = time - high + if time % 2 != 0 { 1 } else { 0 };
    let wins = last - high + 1;
    println!("binary {wins}, steps {steps}");

    // let mut sec: i64 = 1;
    // let mut wins: i64 = 0;
    // loop {
    //     let millimeters = sec * (time - sec);
    //     if traveled(sec, time, distance) {
    //         // println!("boop {wins}");
    //         wins += 1;
    //     }
    //     if millimeters <= distance && wins != 0 {
    //         break;
    //     }
    //     sec += 1
    // }
    println!("answer {wins}");
    Some(wins)
}
