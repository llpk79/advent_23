// use crate::day_15::part_1::part_1;
// use crate::day_15::part_2::part_2;
// use crate::day_16::part_1::part_1;
use crate::day_16::part_2::part_2;
use chrono::Utc;

pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

fn main() {
    let start = Utc::now();
    part_2();
    println!(
        "sec {:.3}",
        (Utc::now() - start).num_milliseconds() as f32 / 1000.0
    )
}
