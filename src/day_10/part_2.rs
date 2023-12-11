use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

pub fn part_2() -> Option<()> {
    let input = read_to_string("src/day_10/input.txt").expect("file exists");
    let lines = input.lines();
    let mut graph: HashMap<(i16, i16), (char, Vec<(i16, i16)>)> = HashMap::new();
    let mut start: (i16, i16) = (0, 0);
    let mut y_len: i16 = 0;
    let mut x_len: i16 = 0;
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (y as i16, x as i16)
            }
            match char {
                '|' => graph.entry((y as i16, x as i16)).or_insert((
                    '|',
                    vec![(y as i16 + 1, x as i16), (y as i16 - 1, x as i16)],
                )),
                '-' => graph.entry((y as i16, x as i16)).or_insert((
                    '-',
                    vec![(y as i16, x as i16 - 1), (y as i16, x as i16 + 1)],
                )),
                'L' => graph.entry((y as i16, x as i16)).or_insert((
                    'L',
                    vec![(y as i16, x as i16 + 1), (y as i16 - 1, x as i16)],
                )),
                'J' => graph.entry((y as i16, x as i16)).or_insert((
                    'J',
                    vec![(y as i16, x as i16 - 1), (y as i16 - 1, x as i16)],
                )),
                '7' => graph.entry((y as i16, x as i16)).or_insert((
                    '7',
                    vec![(y as i16, x as i16 - 1), (y as i16 + 1, x as i16)],
                )),
                'F' => graph.entry((y as i16, x as i16)).or_insert((
                    'F',
                    vec![(y as i16, x as i16 + 1), (y as i16 + 1, x as i16)],
                )),
                '.' => graph
                    .entry((y as i16, x as i16))
                    .or_insert(('.', vec![(y as i16, x as i16), (y as i16, x as i16)])),
                'S' => graph
                    .entry((y as i16, x as i16))
                    .or_insert(('S', vec![(y as i16, x as i16), (y as i16, x as i16)])),
                _ => panic!(),
            };
            x_len = x as i16;
        }
        y_len = y as i16;
    }
    let search = zip(vec![1, 0, -1, 0], vec![0, 1, -0, -1]);
    let mut explore: Vec<(i16, i16)> = search
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
    let mut path: Vec<(i16, i16)> = vec![start];
    while !explore.is_empty() {
        // println!("\nexplore {:?}\npath {:?}", explore, path);
        let current = explore.pop().unwrap();
        if current == start {
            break;
        }
        // println!("current {current:?}");
        let last = *path.last().unwrap();
        let next = &graph.get(&current).unwrap().1;
        if next.iter().any(|point| *point == last) {
            if next[0] == last {
                explore.push(next[1])
            } else {
                explore.push(next[0])
            }
            path.push(current);
        }
    }

    let mut last: char = ' ';
    let mut inside: bool = false;
    let mut answers: Vec<(i16, i16)> = Vec::new();
    for y in 0..y_len {
        for x in 0..x_len {
            let sym = graph.get(&(y, x)).unwrap().0;
            if path.contains(&(y, x)) {
                match sym {
                    '|' => {
                        if ['|', 'F', 'L', 'J', '7', ' '].contains(&last) {
                            last = sym;
                            inside = !inside
                        }
                    }

                    'J' => {
                        if ['F', ' '].contains(&last) {
                            last = sym;
                            inside = !inside
                        }
                    }
                    '7' => {
                        if ['L', ' '].contains(&last) {
                            last = sym;
                            inside = !inside
                        }
                    }
                    'S' => {
                        if ['F', '|', ' ', '7', 'L', 'J'].contains(&last) {
                            last = sym;
                            inside = !inside
                        }
                    }
                    'F' => last = sym,
                    'L' => last = sym,
                    _ => (),
                }
            } else if inside {
                answers.push((y, x));
            }
        }
        inside = false;
        last = ' ';
    }
    answers.sort();
    println!("answers {answers:?}");
    println!("answer {:?}", answers.len());

    Some(())
}

// 354 too low
// 393 too High
// 1103 too high
