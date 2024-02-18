use std::fs::read_to_string;

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_11/example.txt").expect("file exists");
    let lines = input.lines();
    let galaxies: Vec<[usize; 2]> = lines
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, char)| if char == '#' { Some([i, j]) } else { None })
                .collect::<Vec<[usize; 2]>>()
        })
        .collect();

    let mut galaxy_rows: Vec<usize> = (0..galaxies.len())
        .filter(|n| galaxies.iter().all(|g| g[0] != *n))
        .collect();
    let mut galaxy_columns: Vec<usize> = (0..galaxies.len())
        .filter(|n| galaxies.iter().all(|g| g[1] != *n))
        .collect();
    galaxy_columns.sort();
    galaxy_rows.sort();

    let mut expanded: Vec<[usize; 2]> = Vec::new();
    for galaxy in galaxies {
        let mut new: [usize; 2] = galaxy;
        for row in &galaxy_rows {
            if galaxy[0] >= *row {
                new[0] += 1
            }
        }
        for column in &galaxy_columns {
            if galaxy[1] > *column {
                new[1] += 1
            }
        }
        expanded.push(new)
    }

    let mut answer = 0;
    for i in 0..expanded.len() - 1 {
        for j in i + 1..expanded.len() {
            answer +=
                expanded[i][1].abs_diff(expanded[j][1]) + expanded[i][0].abs_diff(expanded[j][0]);
        }
    }
    println!("answer {answer}");
    Some(())
}
