use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    N,
    W,
    S,
    E,
}
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
struct StartPosition {
    pos: (usize, usize),
    dir: Direction,
}

fn travel(
    grid_size: usize,
    start_pos: (usize, usize),
    direction: Direction,
    grid: &HashMap<(usize, usize), char>,
    energized: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    energized.insert(start_pos);
    match direction {
        Direction::N => {
            let mut start = start_pos.0;
            while ![Some(&'-'), Some(&'/'), Some(&'\\')].contains(&grid.get(&(start, start_pos.1)))
                && start > 0
            {
                start -= 1;
                energized.insert((start, start_pos.1));
            }
            (start, start_pos.1)
        }
        Direction::W => {
            let mut start = start_pos.1;
            while ![Some(&'|'), Some(&'/'), Some(&'\\')].contains(&grid.get(&(start_pos.0, start)))
                && start > 0
            {
                start -= 1;
                energized.insert((start_pos.0, start));
            }
            (start_pos.0, start)
        }
        Direction::S => {
            let mut start = start_pos.0;
            while ![Some(&'-'), Some(&'/'), Some(&'\\')].contains(&grid.get(&(start, start_pos.1)))
                && start < grid_size - 1
            {
                start += 1;
                energized.insert((start, start_pos.1));
            }
            (start, start_pos.1)
        }
        Direction::E => {
            let mut start = start_pos.1;
            while ![Some(&'|'), Some(&'/'), Some(&'\\')].contains(&grid.get(&(start_pos.0, start)))
                && start < grid_size - 1
            {
                start += 1;
                energized.insert((start_pos.0, start));
            }
            (start_pos.0, start)
        }
    }
}

fn energize(
    grid_size: usize,
    grid: &HashMap<(usize, usize), char>,
    start_pos: ((usize, usize), Direction),
) -> usize {
    let energized: &mut HashSet<(usize, usize)> = &mut HashSet::new();
    let visited: &mut HashSet<((usize, usize), Direction)> = &mut HashSet::new();
    let mut path: Vec<((usize, usize), Direction)> = vec![start_pos];
    while let Some((start, direction)) = path.pop() {
        if visited.iter().contains(&(start, direction)) {
            continue;
        }
        visited.insert((start, direction));
        let next = travel(grid_size, start, direction, grid, energized);
        match grid.get(&next) {
            Some('\\') => match direction {
                Direction::N => {
                    let x = if next.1 > 0 { next.1 - 1 } else { next.1 };
                    path.push(((next.0, x), Direction::W))
                }
                Direction::W => {
                    let y = if next.0 > 0 { next.0 - 1 } else { next.0 };
                    path.push(((y, next.1), Direction::N))
                }
                Direction::S => {
                    let x = if next.1 + 1 < grid_size - 1 {
                        next.1 + 1
                    } else {
                        next.1
                    };
                    path.push(((next.0, x), Direction::E))
                }
                Direction::E => {
                    let y = if next.0 + 1 < grid_size - 1 {
                        next.0 + 1
                    } else {
                        next.0
                    };
                    path.push(((y, next.1), Direction::S))
                }
            },
            Some('/') => match direction {
                Direction::N => {
                    let x = if next.1 + 1 < grid_size - 1 {
                        next.1 + 1
                    } else {
                        next.1
                    };
                    path.push(((next.0, x), Direction::E))
                }
                Direction::W => {
                    let y = if next.0 + 1 < grid_size - 1 {
                        next.0 + 1
                    } else {
                        next.0
                    };
                    path.push(((y, next.1), Direction::S))
                }
                Direction::S => {
                    let x = if next.1 > 0 { next.1 - 1 } else { next.1 };
                    path.push(((next.0, x), Direction::W))
                }
                Direction::E => {
                    let y = if next.0 > 0 { next.0 - 1 } else { next.0 };
                    path.push(((y, next.1), Direction::N))
                }
            },
            Some('-') => match direction {
                Direction::N | Direction::S => {
                    path.push((next, Direction::W));
                    path.push((next, Direction::E));
                }
                _ => {}
            },
            Some('|') => match direction {
                Direction::W | Direction::E => {
                    path.push((next, Direction::N));
                    path.push((next, Direction::S));
                }
                _ => {}
            },
            _ => {}
        }
    }
    energized.len()
}

pub fn part_2() -> Option<()> {
    let input = read_to_string("src/day_16/input.txt").expect("file exists");
    let grid_size = input.split_once('\n').unwrap().0.len() - 1;
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char != '.' {
                grid.insert((i, j), char);
            }
        }
    }
    let mut starting_positions: Vec<StartPosition> = Vec::new();
    for i in 0..grid_size {
        for j in 0..grid_size {
            if i == 0 {
                starting_positions.push(StartPosition {
                    pos: (i, j),
                    dir: Direction::S,
                })
            } else if j == 0 {
                starting_positions.push(StartPosition {
                    pos: (i, j),
                    dir: Direction::E,
                })
            } else if i == grid_size - 1 {
                starting_positions.push(StartPosition {
                    pos: (i, j),
                    dir: Direction::N,
                })
            } else if j == grid_size - 1 {
                starting_positions.push(StartPosition {
                    pos: (i, j),
                    dir: Direction::W,
                })
            } else {
                continue;
            }
        }
    }
    let mut maximum = 0;
    for starting_position in starting_positions {
        let total = energize(
            grid_size,
            &grid,
            (starting_position.pos, starting_position.dir),
        );
        maximum = max(maximum, total);
    }
    println!("answer {maximum}");

    Some(())
}
