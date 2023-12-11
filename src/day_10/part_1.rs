use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_10/input.txt").expect("file exists");
    let lines = input.lines();
    let mut graph: HashMap<(i8, i8), Vec<(i8, i8)>> = HashMap::new();
    let mut start: (i8, i8) = (0, 0);
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (y as i8, x as i8)
            }
            match char {
                '|' => graph
                    .entry((y as i8, x as i8))
                    .or_insert(vec![(y as i8 + 1, x as i8), (y as i8 - 1, x as i8)]),
                '-' => graph
                    .entry((y as i8, x as i8))
                    .or_insert(vec![(y as i8, x as i8 - 1), (y as i8, x as i8 + 1)]),
                'L' => graph
                    .entry((y as i8, x as i8))
                    .or_insert(vec![(y as i8, x as i8 + 1), (y as i8 - 1, x as i8)]),
                'J' => graph
                    .entry((y as i8, x as i8))
                    .or_insert(vec![(y as i8, x as i8 - 1), (y as i8 - 1, x as i8)]),
                '7' => graph
                    .entry((y as i8, x as i8))
                    .or_insert(vec![(y as i8, x as i8 - 1), (y as i8 + 1, x as i8)]),
                'F' => graph
                    .entry((y as i8, x as i8))
                    .or_insert(vec![(y as i8, x as i8 + 1), (y as i8 + 1, x as i8)]),
                '.' => graph
                    .entry((y as i8, x as i8))
                    .or_insert(vec![(y as i8, x as i8), (y as i8, x as i8)]),
                _ => continue,
            };
        }
    }
    let search = zip(vec![1, 0, -1, 0], vec![0, 1, -0, -1]);
    let mut explore: Vec<(i8, i8)> = search
        .map(|(y, x)| {
            (
                if start.0 + y > -1 {
                    start.0 + y
                } else {
                    start.0
                },
                if start.1 + x > -1 {
                    start.1 + x
                } else {
                    start.1
                },
            )
        })
        .filter(|point| *point != start)
        .collect();
    // println!("explore {explore:?}");
    let mut path: Vec<(i8, i8)> = vec![start];
    while !explore.is_empty() {
        // println!("\nexplore {:?}\npath {:?}", explore, path);
        let current = explore.pop()?;
        if current == start {
            break;
        }
        // println!("current {current:?}");
        let last = *path.last().unwrap();
        let next = graph.get(&current).unwrap();
        if next.iter().any(|point| *point == last) {
            if next[0] == last {
                explore.push(next[1])
            } else {
                explore.push(next[0])
            }
            path.push(current);
        }
    }
    println!("answer {}", path.len() / 2);

    Some(())
}
