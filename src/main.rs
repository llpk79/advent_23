// use crate::day_6::part_1::part_1;
// use crate::day_6::part_2::part_2;
use crate::day_5::day_5_refactor::part_2_refactor;
use chrono::Utc;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;

fn main() {
    let start = Utc::now();
    part_2_refactor();
    println!(
        "sec {}",
        (Utc::now() - start).num_milliseconds() as f32 / 1000.0
    )
}
