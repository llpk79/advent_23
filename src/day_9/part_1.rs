use std::fs::read_to_string;

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_9/input.txt").expect("file exists");
    let lines = input.lines();
    let readings: Vec<Vec<i64>> = lines
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // println!("readings {readings:?}");
    let mut sequences: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut reduced: Vec<i64> = Vec::new();
    for reading in readings {
        let mut temps = reading.clone();
        let mut sequence: Vec<Vec<i64>> = Vec::new();
        sequence.push(reading);
        'inner: loop {
            for i in 0..temps.len() - 1 {
                reduced.push(temps[i + 1] - temps[i])
            }
            if reduced.iter().all(|num| *num == 0) {
                sequence.push(reduced.clone());
                sequences.push(sequence.clone());
                sequence.clear();
                reduced.clear();
                break 'inner;
            } else {
                sequence.push(reduced.clone());
                temps = reduced.clone();
                reduced.clear();
            }
        }
    }

    let mut answer: i64 = 0;
    let mut this: i64 = 0;
    for mut sequence in sequences {
        sequence.reverse();
        for mut next in sequence {
            let pop = next.pop()?;
            this += pop;
        }
        answer += this;
        this = 0;
    }

    println!("answer {answer}");

    Some(())
}
