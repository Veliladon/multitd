use std::collections::VecDeque;

use rand::prelude::*;

use super::Maze;
use crate::prelude::*;

pub const HILBERT: [usize; 36] = [
    0, 1, 2, 8, 7, 6, 12, 13, 14, 15, 9, 3, 4, 5, 11, 10, 16, 17, 23, 22, 28, 29, 35, 34, 33, 27,
    21, 20, 19, 18, 24, 25, 26, 32, 31, 30,
];

impl Maze {
    pub fn idx(&self, pos: IVec2) -> usize {
        ((pos.y * (self.width as i32)) + pos.x) as usize
    }

    pub fn in_bounds(&self, pos: IVec2) -> bool {
        (pos.x >= 0)
            && (pos.x < (self.width as i32))
            && (pos.y >= 0)
            && (pos.y < (self.height as i32))
    }

    pub fn is_exit(&self, pos: usize) -> bool {
        {
            self.tiles[pos].exits[0] == Exit::Finish
                || self.tiles[pos].exits[1] == Exit::Finish
                || self.tiles[pos].exits[2] == Exit::Finish
                || self.tiles[pos].exits[3] == Exit::Finish
        }
    }

    pub fn _new(width: i32, height: i32, mut rng: ThreadRng) -> Maze {
        let mut maze = Maze {
            width,
            height,
            tiles: vec![Cell::default(); (width * height) as usize],
            entry_index: 0,
            exit_index: 0,
        };

        let mut visited_count: i32 = 1;

        let mut pos = IVec2::new(
            rng.random_range(0..width) as i32,
            rng.random_range(0..height) as i32,
        );

        while visited_count < (width * height) {
            let direction = rng.random_range(0..4);
            info!("Picked {} on the 1d4 direction", direction);
            let new_pos = pos + DIRECTIONS[direction];
            // println!("I'm at... {:#?} and trying {:#?}", pos, new_pos);
            let new_index = maze.idx(new_pos);
            if maze.in_bounds(new_pos) {
                let index = maze.idx(pos);
                if maze.tiles[new_index].visited() == Visited::NotVisited {
                    visited_count += 1;
                    maze.tiles[new_index].exits[(direction + 2) % 4] = Exit::Open;
                    maze.tiles[index].exits[direction] = Exit::Open;
                }
                pos = new_pos;
            } else {
            }
        }

        let entry = rng.random_range(0..width);
        let entry_index = maze.idx(IVec2::new(entry, 0));
        maze.tiles[entry_index].exits[2] = Exit::Start;

        let exit = rng.random_range(0..width);

        let exit_index = ((maze.width * maze.height - 1) + exit) as usize;
        maze.tiles[exit_index].exits[0] = Exit::Finish;
        dbg!("{:?}", &maze.entry_index);

        dbg!("{:?}", &maze.exit_index);

        maze
    }

    pub fn new_hilbert(width: i32, height: i32) -> Maze {
        let mut maze = Maze {
            width,
            height,
            tiles: vec![Cell::default(); (width * height) as usize],
            entry_index: 0,
            exit_index: 30,
        };

        for pair in HILBERT.windows(2) {
            let a = pair[0];
            let b = pair[1];
            let difference = b as i32 - a as i32;

            match difference {
                -6 => {
                    maze.tiles[a].exits[2] = Exit::Open;
                    maze.tiles[b].exits[0] = Exit::Open;
                } // north

                1 => {
                    maze.tiles[a].exits[1] = Exit::Open;
                    maze.tiles[b].exits[3] = Exit::Open;
                } // east
                6 => {
                    maze.tiles[a].exits[0] = Exit::Open;
                    maze.tiles[b].exits[2] = Exit::Open;
                } // south
                -1 => {
                    maze.tiles[a].exits[3] = Exit::Open;
                    maze.tiles[b].exits[1] = Exit::Open;
                } // west
                _ => panic!(),
            }
        }

        maze.tiles[maze.entry_index].exits[2] = Exit::Start;

        maze.tiles[maze.exit_index].exits[0] = Exit::Finish;
        // dbg!("{:?", &maze);
        maze
    }
}

pub fn find_exit(maze: &Maze, mazegraph: &MazeGraph) -> MazeRoute {
    let size = (maze.width * maze.height) as usize;
    let mut queue = VecDeque::new();

    let entry = maze.entry_index;
    queue.push_back(entry);
    let mut cells = vec![false; size];
    let mut parents = vec![usize::MAX; size];
    let exit = maze.exit_index;
    let mut nodes = Vec::new();
    cells[entry] = true;

    while let Some(cell) = queue.pop_front() {
        if cell == exit {
            let mut current = exit;
            while current != usize::MAX {
                nodes.push(current);
                current = parents[current];
            }
            nodes.reverse();
            return MazeRoute {
                nodes,
                entry_direction: Direction::South,
                exit_direction: Direction::North,
            };
        }
        for neighbor in mazegraph.nodes[cell].iter().flatten() {
            if !cells[*neighbor] {
                cells[*neighbor] = true;
                parents[*neighbor] = cell;
                queue.push_back(*neighbor);
            }
        }
    }

    MazeRoute {
        nodes,
        entry_direction: Direction::South,
        exit_direction: Direction::North,
    }
}

impl MazeGraph {
    pub fn from_maze(maze: &Maze) -> Self {
        let size = (maze.width * maze.height) as usize;
        let mut nodes = vec![[None; 4]; size]; // Layout is N/E/S/W

        for (index, tile) in maze.tiles.iter().enumerate() {
            for (dir, offset) in DIRECTIONS.iter().enumerate() {
                if tile.exits[dir] == Exit::Open {
                    let neighbor_pos = IVec2::new(
                        (index as i32 % maze.width) + offset.x,
                        (index as i32 / maze.width) + offset.y,
                    );
                    if maze.in_bounds(neighbor_pos) {
                        nodes[index][dir] = Some(maze.idx(neighbor_pos));
                    }
                }
            }
        }
        MazeGraph { nodes }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Visited {
    Visited,
    NotVisited,
}

impl Cell {
    pub fn visited(&self) -> Visited {
        if self.exits[0] == Exit::Closed
            && self.exits[1] == Exit::Closed
            && self.exits[2] == Exit::Closed
            && self.exits[3] == Exit::Closed
        {
            return Visited::NotVisited;
        }
        Visited::Visited
    }
}
