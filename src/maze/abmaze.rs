use std::collections::VecDeque;

use rand::prelude::*;

use super::Maze;
use crate::*;

const DIRECTIONS: [IVec2; 4] = [
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: -1 },
    IVec2 { x: -1, y: 0 },
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

    pub fn is_exit(&self, pos: IVec2) -> bool {
        {
            self.tiles[self.idx(pos)].exits[0] == Exit::Finish
                || self.tiles[self.idx(pos)].exits[1] == Exit::Finish
                || self.tiles[self.idx(pos)].exits[2] == Exit::Finish
                || self.tiles[self.idx(pos)].exits[3] == Exit::Finish
        }
    }

    pub fn new(width: i32, height: i32, mut rng: ThreadRng) -> Maze {
        let mut maze = Maze {
            width,
            height,
            tiles: vec![Cell::default(); (width * height) as usize],
            entry: 0,
            exit: 0,
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

        maze.entry = rng.random_range(0..width);
        println!("{}", maze.entry);
        let entry_index = maze.idx(IVec2::new(maze.entry, 0));
        maze.tiles[entry_index].exits[2] = Exit::Start;

        maze.exit = rng.random_range(0..width);
        println!("{}", maze.exit);
        let exit_index = maze.idx(IVec2::new(maze.exit, maze.height - 1));
        maze.tiles[exit_index].exits[0] = Exit::Finish;

        maze
    }
}

pub fn find_exit(maze: &Maze, mazegraph: &MazeGraph) -> MazeRoute {
    let size = (maze.width * maze.height) as usize;
    let mut queue = VecDeque::new();

    let entry = maze.idx(IVec2 {
        x: maze.entry,
        y: 0,
    });

    queue.push_back(entry);
    let mut cells = vec![false; size];
    let mut parents = vec![usize::MAX; size];
    let exit = maze.idx(IVec2::new(maze.exit, maze.height - 1));
    let mut nodes = Vec::new();
    cells[entry] = true;

    while let Some(cell) = queue.pop_front().filter(|&c| c != exit) {
        for neighbor in mazegraph.nodes[cell].iter().flatten() {
            if !cells[*neighbor] {
                cells[*neighbor] = true;
                parents[*neighbor] = cell;
                queue.push_back(*neighbor);
            }
        }
    }
    let mut current = exit;
    while current != usize::MAX {
        nodes.push(current);
        current = parents[current]
    }
    nodes.reverse();

    MazeRoute { nodes }
}

impl MazeGraph {
    pub fn from_maze(maze: &Maze) -> Self {
        let size = (maze.width * maze.height) as usize;
        let mut nodes = vec![[None; 4]; size]; // Layout is N/E/S/W

        for (index, tile) in maze.tiles.iter().enumerate() {
            for (dir, offset) in DIRECTIONS.iter().enumerate() {
                if tile.exits[dir] == Exit::Open {
                    nodes[index][dir] =
                        Some((index as i32 + (offset.x) + (offset.y * maze.width)) as usize)
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
