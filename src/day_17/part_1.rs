use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
enum Direction {
    N,
    W,
    S,
    E,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Pos {
    pos: (u8, u8),
    dir: Direction,
}

#[derive(Eq, Debug, Clone, Copy)]
struct Block {
    cost: i16,
    strt: i8,
    pos: Pos,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

fn look_north(pos: (u8, u8), grid: &HashMap<(u8, u8), i16>) -> Option<&i16> {
    grid.get(&(pos.0 - 1, pos.1))
}

fn look_south(pos: (u8, u8), grid: &HashMap<(u8, u8), i16>) -> Option<&i16> {
    grid.get(&(pos.0 + 1, pos.1))
}

fn look_east(pos: (u8, u8), grid: &HashMap<(u8, u8), i16>) -> Option<&i16> {
    grid.get(&(pos.0, pos.1 + 1))
}

fn look_west(pos: (u8, u8), grid: &HashMap<(u8, u8), i16>) -> Option<&i16> {
    grid.get(&(pos.0, pos.1 - 1))
}

fn filter_blocks(
    blocks: Vec<Block>,
    grid: &HashMap<(u8, u8), i16>,
    visited: &mut HashSet<(u8, u8)>,
) -> Vec<Block> {
    blocks
        .into_iter()
        .filter(|b| grid.contains_key(&b.pos.pos) && b.strt < 3)
        .collect()
}

fn look(
    block: &Block,
    grid: &HashMap<(u8, u8), i16>,
    visited: &mut HashSet<(u8, u8)>,
) -> Option<Vec<Block>> {
    match block.pos.dir {
        Direction::N => {
            let west = look_west(block.pos.pos, grid).unwrap_or(&(200));
            let east = look_east(block.pos.pos, grid).unwrap_or(&(200));
            let north = look_north(block.pos.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *west,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0, block.pos.pos.1 - 1),
                        dir: Direction::N,
                    },
                },
                Block {
                    cost: *east,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0, block.pos.pos.1 + 1),
                        dir: Direction::E,
                    },
                },
                Block {
                    cost: *north,
                    strt: block.strt + 1,
                    pos: Pos {
                        pos: (block.pos.pos.0 - 1, block.pos.pos.1),
                        dir: Direction::N,
                    },
                },
            ];
            Some(filter_blocks(blocks, grid, visited))
        }
        Direction::S => {
            let west = look_west(block.pos.pos, grid).unwrap_or(&(200));
            let east = look_east(block.pos.pos, grid).unwrap_or(&(200));
            let south = look_south(block.pos.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *west,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0, block.pos.pos.1 - 1),
                        dir: Direction::W,
                    },
                },
                Block {
                    cost: *east,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0, block.pos.pos.1 + 1),
                        dir: Direction::E,
                    },
                },
                Block {
                    cost: *south,
                    strt: block.strt + 1,
                    pos: Pos {
                        pos: (block.pos.pos.0 + 1, block.pos.pos.1),
                        dir: Direction::S,
                    },
                },
            ];
            Some(filter_blocks(blocks, grid, visited))
        }
        Direction::E => {
            let north = look_north(block.pos.pos, grid).unwrap_or(&(200));
            let south = look_south(block.pos.pos, grid).unwrap_or(&(200));
            let east = look_east(block.pos.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *north,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0 - 1, block.pos.pos.1),
                        dir: Direction::N,
                    },
                },
                Block {
                    cost: *south,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0 + 1, block.pos.pos.1),
                        dir: Direction::S,
                    },
                },
                Block {
                    cost: *east,
                    strt: block.strt + 1,
                    pos: Pos {
                        pos: (block.pos.pos.0, block.pos.pos.1 + 1),
                        dir: Direction::E,
                    },
                },
            ];
            Some(filter_blocks(blocks, grid, visited))
        }
        Direction::W => {
            let north = look_north(block.pos.pos, grid).unwrap_or(&(200));
            let south = look_south(block.pos.pos, grid).unwrap_or(&(200));
            let west = look_west(block.pos.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *north,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0 - 1, block.pos.pos.1),
                        dir: Direction::N,
                    },
                },
                Block {
                    cost: *south,
                    strt: block.strt,
                    pos: Pos {
                        pos: (block.pos.pos.0 + 1, block.pos.pos.1),
                        dir: Direction::S,
                    },
                },
                Block {
                    cost: *west,
                    strt: block.strt + 1,
                    pos: Pos {
                        pos: (block.pos.pos.0, block.pos.pos.1 - 1),
                        dir: Direction::W,
                    },
                },
            ];
            Some(filter_blocks(blocks, grid, visited))
        }
    }
}

fn travel(
    start: Block,
    path: &mut HashMap<(u8, u8), Block>,
    grid: &HashMap<(u8, u8), i16>,
) -> Option<()> {
    let mut queue = BinaryHeap::new();
    let visited: &mut HashSet<(u8, u8)> = &mut HashSet::new();
    let mut cost_map: HashMap<(u8, u8), i16> = HashMap::new();

    queue.push(start);
    while !queue.is_empty() {
        let block = queue.pop()?;
        if visited.contains(&block.pos.pos) {
            continue;
        }
        visited.insert(block.pos.pos);

        let neighbors = look(&block, grid, visited)?;
        for neighbor in neighbors {
            match cost_map.get(&neighbor.pos.pos) {
                Some(existing) => {
                    println!(
                        "\nblock {:?} st {:?}\nneigh {:?} st {:?}\nexist {:?}",
                        block.pos.pos, block.pos.dir, neighbor.pos.pos, neighbor.strt, existing
                    );
                    if (neighbor.strt + (neighbor.pos.dir == block.pos.dir) as i8) < 3 {
                        println!("beep");
                        let new_cost = &(block.cost + grid.get(&neighbor.pos.pos)?);
                        let cost = min(existing, new_cost);
                        cost_map.insert(neighbor.pos.pos, *cost);
                    }
                }
                None => {
                    cost_map
                        .entry(neighbor.pos.pos)
                        .or_insert(block.cost + grid.get(&neighbor.pos.pos)?);
                }
            };
            queue.push(Block {
                cost: *cost_map.get(&neighbor.pos.pos)?,
                strt: if neighbor.pos.dir == block.pos.dir {
                    block.strt + 1
                } else {
                    0
                },
                pos: neighbor.pos,
            });

            match path.get(&neighbor.pos.pos) {
                Some(existing) => {
                    if existing.cost > *cost_map.get(&neighbor.pos.pos)?
                        && ((neighbor.strt + (neighbor.pos.dir != block.pos.dir) as i8) < 3)
                    {
                        println!("bloop");
                        path.insert(neighbor.pos.pos, block);
                    };
                }
                None => {
                    path.entry(neighbor.pos.pos).or_insert(block);
                }
            }
        }
    }
    Some(())
}

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_17/example.txt").expect("file exists");
    let grid_size = (input.clone().split_once('\n')?.0.len() - 1) as u8;
    let mut grid: HashMap<(u8, u8), i16> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            grid.insert(
                (i as u8, j as u8),
                char.to_digit(10).expect("std fmt") as i16,
            );
        }
    }

    let start_pos = Pos {
        pos: (0, 0),
        dir: Direction::E,
    };
    let start = Block {
        cost: 0,
        strt: 0,
        pos: start_pos,
    };
    let finish = (grid_size - 1, grid_size - 1);
    let path: &mut HashMap<(u8, u8), Block> = &mut HashMap::new();
    travel(start, path, &grid);
    let mut path_vec: Vec<(u8, u8)> = Vec::new();
    let mut next = path.get(&finish)?;
    loop {
        path_vec.push(next.pos.pos);
        next = path.get(&next.pos.pos)?;
        if next.pos.pos == start_pos.pos || path_vec.contains(&next.pos.pos) {
            break;
        }
    }
    path_vec.push(finish);

    let answer: i16 = path_vec.iter().filter_map(|p| grid.get(p)).sum();

    println!("answer {answer}");
    let path_map: HashSet<&(u8, u8)> = path_vec.iter().collect();
    let mut disp = "".to_string();
    for i in 0..grid_size {
        disp.push('\n');
        for j in 0..grid_size {
            if path_map.contains(&(i, j)) {
                disp.push('#')
            } else {
                disp.push(char::from_digit(*grid.get(&(i, j)).unwrap() as u32, 10)?)
            }
        }
    }
    println!("disp\n {disp}");
    println!("end: {:?}", path.get(&finish));

    Some(())
}

// 869 too high
