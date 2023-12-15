use std::collections::HashSet;
use std::fs::read_to_string;

fn flip_bits(num: u32) -> HashSet<u32> {
    (0..32).map(|i| num ^ 1 << i).collect()
}

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_13/input.txt").expect("file exists");
    let lines = input.lines();
    let mut puzzles: Vec<(Vec<u32>, Vec<String>)> = Vec::new();
    let mut cols: Vec<String> = Vec::new();
    let mut rows: Vec<u32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            puzzles.push((rows.clone(), cols.clone()));
            rows.clear();
            cols.clear();
            continue;
        }
        let mut digits = line
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let ch = if c == '.' { '0' } else { '1' };
                if cols.len() > i {
                    cols[i].push(ch)
                } else {
                    cols.push(format!("{ch}").to_string());
                };
                ch
            })
            .collect::<String>();
        digits.extend((0..32 - line.len() - 1).map(|_| '0').collect::<Vec<char>>());
        digits = digits.chars().rev().collect();
        rows.push(u32::from_str_radix(&digits, 2).unwrap())
    }
    puzzles.push((rows.clone(), cols.clone()));
    let real_puzzles: Vec<(Vec<u32>, Vec<u32>)> = puzzles
        .iter()
        .map(|(row, col)| {
            let num_cols: Vec<u32> = col
                .iter()
                .map(|s| {
                    s.to_string()
                        .extend((0..32 - s.len() - 1).map(|_| '0').collect::<Vec<char>>());
                    let r = s.chars().rev().collect::<String>();
                    u32::from_str_radix(&r, 2).unwrap()
                })
                .collect();
            (row.clone(), num_cols)
        })
        .collect();

    let mut total: u32 = 0;
    for puzzle in real_puzzles {
        let mut start: usize;
        let mut i_flipped: HashSet<u32>;
        let mut one_flipped: bool = false;
        'rows: for mut i in 0..puzzle.0.len() - 1 {
            i_flipped = flip_bits(puzzle.0[i]);
            if puzzle.0[i] == puzzle.0[i + 1] || i_flipped.contains(&puzzle.0[i + 1]) {
                if !one_flipped {
                    one_flipped = i_flipped.contains(&puzzle.0[i + 1])
                };
                start = i;
                let mut j = i + 1;
                while (puzzle.0[i] == puzzle.0[j] || i_flipped.contains(&puzzle.0[j]))
                    && j < puzzle.0.len() - 1
                    && i > 0
                {
                    i -= 1;
                    j += 1;
                    i_flipped = flip_bits(puzzle.0[i]);
                    if !one_flipped {
                        one_flipped = i_flipped.contains(&puzzle.0[j])
                    };
                }
                if (puzzle.0[i] == puzzle.0[j] || i_flipped.contains(&puzzle.0[j]))
                    && (j == puzzle.0.len() - 1 || i == 0)
                    && one_flipped
                {
                    total += (start + 1) as u32 * 100;
                    break 'rows;
                }
                one_flipped = false
            }
        }
        if one_flipped {
            continue;
        };
        'cols: for mut i in 0..=puzzle.1.len() - 1 {
            i_flipped = flip_bits(puzzle.1[i]);
            if puzzle.1[i] == puzzle.1[i + 1] || i_flipped.contains(&puzzle.1[i + 1]) {
                if !one_flipped {
                    one_flipped = i_flipped.contains(&puzzle.1[i + 1])
                };
                start = i;
                let mut j = i + 1;
                while (puzzle.1[i] == puzzle.1[j] || i_flipped.contains(&puzzle.1[j]))
                    && (j < puzzle.1.len() - 1)
                    && i > 0
                {
                    i -= 1;
                    j += 1;
                    i_flipped = flip_bits(puzzle.1[i]);
                    if !one_flipped {
                        one_flipped = i_flipped.contains(&puzzle.1[j])
                    };
                }
                if (puzzle.1[i] == puzzle.1[j] || i_flipped.contains(&puzzle.1[j]))
                    && (j == puzzle.1.len() - 1 || i == 0)
                    && one_flipped
                {
                    total += (start + 1) as u32;
                    break 'cols;
                }
                one_flipped = false
            }
        }
    }
    println!("total {total}");
    Some(())
}
