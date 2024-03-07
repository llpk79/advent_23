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


#[derive(Eq, Debug, Clone, Copy)]
struct Block {
    cost: i16,
    straight: i8,
    pos: (u8, u8),
    dir: Direction,
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

fn new_direction(from: (u8, u8), to: (u8, u8)) -> Direction {
    let dif: (i8, i8) = (from.0 as i8 - to.0 as i8, from.1 as i8 - to.1 as i8);
    match dif {
        (1, 0) => Direction::N,
        (0, 1) => Direction::W,
        (-1, 0) => Direction::S,
        (0, -1) => Direction::E,
        _ => panic!(),
    }
}

fn backward(from: Direction, to: Direction) -> bool {
    match (from, to) {
        (Direction::N, Direction::S) => true,
        (Direction::W, Direction::E) => true,
        (Direction::S, Direction::S) => true,
        (Direction::E, Direction::W) => true,
        _ => false
    }
}

fn filter_blocks(
    blocks: Vec<Block>,
    grid: &HashMap<(u8, u8), i16>,
    visited: &HashSet<(u8, u8)>,
) -> Vec<Block> {
    blocks
        .into_iter()
        .filter(|b| b.straight < 3 && grid.contains_key(&b.pos) && !visited.contains(&b.pos))
        .collect()
}

fn look(
    block: Block,
    grid: &HashMap<(u8, u8), i16>,
    visited: &HashSet<(u8, u8)>,
) -> Option<Vec<Block>> {
    match block.dir {
        Direction::N => {
            let west = look_west(block.pos, grid).unwrap_or(&(200));
            let east = look_east(block.pos, grid).unwrap_or(&(200));
            let north = look_north(block.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *west,
                    straight: 0,
                    pos: (block.pos.0, block.pos.1 - 1),
                    dir: Direction::N,
                },
                Block {
                    cost: *east,
                    straight: 0,
                    pos: (block.pos.0, block.pos.1 + 1),
                    dir: Direction::E,
                },
                Block {
                    cost: *north,
                    straight: block.straight + 1,
                    pos: (block.pos.0 - 1, block.pos.1),
                    dir: Direction::N,
                },
            ];
            Some(filter_blocks(blocks, grid, visited))
        }
        Direction::S => {
            let west = look_west(block.pos, grid).unwrap_or(&(200));
            let east = look_east(block.pos, grid).unwrap_or(&(200));
            let south = look_south(block.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *west,
                    straight: 0,
                    pos: (block.pos.0, block.pos.1 - 1),
                    dir: Direction::W,
                },
                Block {
                    cost: *east,
                    straight: 0,
                    pos: (block.pos.0, block.pos.1 + 1),
                    dir: Direction::E,
                },
                Block {
                    cost: *south,
                    straight: block.straight + 1,
                    pos: (block.pos.0 + 1, block.pos.1),
                    dir: Direction::S,
                },
            ];
            Some(filter_blocks(blocks, grid, visited))
        }
        Direction::E => {
            let north = look_north(block.pos, grid).unwrap_or(&(200));
            let south = look_south(block.pos, grid).unwrap_or(&(200));
            let east = look_east(block.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *north,
                    straight: 0,
                    pos: (block.pos.0 - 1, block.pos.1),
                    dir: Direction::N,
                },
                Block {
                    cost: *south,
                    straight: 0,
                    pos: (block.pos.0 + 1, block.pos.1),
                    dir: Direction::S,
                },
                Block {
                    cost: *east,
                    straight: block.straight + 1,
                    pos: (block.pos.0, block.pos.1 + 1),
                    dir: Direction::E,
                },
            ];
            Some(filter_blocks(blocks, grid, visited))
        }
        Direction::W => {
            let north = look_north(block.pos, grid).unwrap_or(&(200));
            let south = look_south(block.pos, grid).unwrap_or(&(200));
            let west = look_west(block.pos, grid).unwrap_or(&(200));
            let blocks: Vec<Block> = vec![
                Block {
                    cost: *north,
                    straight: 0,
                    pos: (block.pos.0 - 1, block.pos.1),
                    dir: Direction::N,
                },
                Block {
                    cost: *south,
                    straight: 0,
                    pos: (block.pos.0 + 1, block.pos.1),
                    dir: Direction::S,
                },
                Block {
                    cost: *west,
                    straight: block.straight + 1,
                    pos: (block.pos.0, block.pos.1 - 1),
                    dir: Direction::W,
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
    finish: (u8, u8),
) -> Option<()> {
    let mut queue = BinaryHeap::new();
    let mut visited: HashSet<(u8, u8)> = HashSet::new();
    let mut cost_map: HashMap<(u8, u8), i16> = HashMap::new();

    // queue should not be of Blocks but of (cost, pos)
    queue.push(start);
    while !queue.is_empty() {
        let block = queue.pop()?;
        // println!("\nblock {:?}", block);
        // if block.pos == finish {
        //     break;
        // }
        // if visited.contains(&block.pos) {
        //     continue;
        // }
        visited.insert(block.pos);
        let neighbors = look(block, grid, &visited)?;
        for neighbor in neighbors {
            let existing_cost = *cost_map.get(&neighbor.pos).unwrap_or(&5000_i16);
            let new_cost = block.cost + neighbor.cost;
            match path.get(&neighbor.pos) {
                Some(existing) => {
                    if existing_cost > new_cost
                        || (existing_cost == new_cost && existing.straight - neighbor.straight > 0 && neighbor.cost < *grid.get(&existing.pos)?)
                    {
                        let cost = min(existing_cost, new_cost);
                        cost_map.insert(neighbor.pos, cost);
                        println!("\nnew {}, exist {}", new_cost, existing_cost);
                        println!("block {:?}\n\tneighbor {:?}\n\texisting {:?}", block, neighbor, existing);
                        path.insert(neighbor.pos, Block {
                            cost,
                            straight: neighbor.straight,
                            pos: block.pos,
                            dir: neighbor.dir,
                        });
                        queue.push(Block {
                            cost,
                            straight: neighbor.straight,
                            pos: neighbor.pos,
                            dir: neighbor.dir,
                        });
                    };
                }
                None => {
                    path.insert(neighbor.pos, Block {
                        cost: new_cost,
                        straight: neighbor.straight,
                        pos: block.pos,
                        dir: neighbor.dir,
                    });
                    queue.push(Block {
                        cost: new_cost,
                        straight: neighbor.straight,
                        pos: neighbor.pos,
                        dir: neighbor.dir,
                    });
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

    let start = Block {
        cost: 0,
        straight: 0,
        pos: (0, 0),
        dir: Direction::E,
    };

    let finish = (grid_size - 1, grid_size - 1);
    let path: &mut HashMap<(u8, u8), Block> = &mut HashMap::new();
    travel(start, path, &grid, finish);
    let mut path_vec: Vec<(u8, u8)> = Vec::new();
    let mut next = path.get(&finish)?;
    loop {
        path_vec.push(next.pos);
        next = path.get(&next.pos)?;
        if next.pos == start.pos || path_vec.contains(&next.pos) {
            break;
        }
    }
    path_vec.push(finish);

    let answer: i16 = path_vec.iter().filter_map(|p| grid.get(p)).sum();
    let char_map = HashMap::from([(Direction::S, 'v'), (Direction::N, '^'), (Direction::E, '>'), (Direction::W, '<')]);

    println!("answer {answer}");
    let path_map: HashSet<&(u8, u8)> = path_vec.iter().collect();
    let mut disp = "".to_string();
    for i in 0..grid_size {
        disp.push('\n');
        for j in 0..grid_size {
            if path_map.contains(&(i, j)) {
                disp.push(*char_map.get(&path.get(&(i, j))?.dir)?)
            } else {
                disp.push(char::from_digit(*grid.get(&(i, j)).unwrap() as u32, 10)?)
            }
        }
    }
    println!("disp\n {disp}");
    println!("{:?}", path.get(&(1, 2)));

    Some(())
}

// 869 too high
