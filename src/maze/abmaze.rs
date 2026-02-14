use rand::prelude::*;

use super::{Maze, MazeBuilder};
use crate::*;

pub struct ABMaze;

const DIRECTIONS: [IVec2; 4] = [
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: -1 },
    IVec2 { x: -1, y: 0 },
];

impl MazeBuilder for ABMaze {
    fn new(width: i32, height: i32, mut rng: ThreadRng) -> Maze {
        let mut maze = Maze {
            width,
            height,
            tiles: vec![Cell::default(); (width * height) as usize],
            entry: 0,
            exit: 0,
        };

        /* let visited_list = vec![Visited::default(); width * height]; */
        let mut visited_count: i32 = 1;

        let mut pos = IVec2::new(
            rng.random_range(0..width) as i32,
            rng.random_range(0..height) as i32,
        );

        while visited_count < (width * height) {
            let direction = rng.random_range(0..4);
            // println!("Picked {} on the 1d4 direction", direction);
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

            /* println!(
                "Starting from  {:#?} next. Visited {} tiles",
                pos, visited_count
            ); */
        }
        // println!("{:#?}", maze);
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

/* pub fn check_visited(visited: &VecVisisted, index: usize) -> Visited {
    Visited::NotVisited
} */
