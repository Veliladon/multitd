pub use crate::prelude::*;
pub use rand::prelude::*;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Exit {
    Open,
    #[default]
    Closed,
}

/* #[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cell {
    pub north: Exit,
    pub east: Exit,
    pub south: Exit,
    pub west: Exit,
} */

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cell {
    pub exits: [Exit; 4],
}

#[derive(Debug, Resource)]
pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub entry: i32,
    pub exit: i32,
    pub tiles: Vec<Cell>,
}

pub trait MazeBuilder {
    fn new(width: i32, height: i32, rng: ThreadRng) -> Maze;
}

#[derive(Debug)]
pub struct MazeCorridor {
    start: (i32, i32),           // Starting position of the corridor
    path: Vec<(i32, i32)>,       // Linear path through the corridor
    children: Vec<MazeCorridor>, // Corridors branching out from the end
}

impl MazeCorridor {
    pub fn new(start: (i32, i32)) -> Self {
        MazeCorridor {
            start,
            path: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: MazeCorridor) {
        self.children.push(child);
    }
}

#[derive(Debug)]
pub struct MazeCorridorTree {
    root: MazeCorridor,
}

impl MazeCorridorTree {
    pub fn new(start_position: (i32, i32)) -> Self {
        MazeCorridorTree {
            root: MazeCorridor::new(start_position),
        }
    }

    pub fn print_routes(&self) {
        let mut route = Vec::new();
        self.print_routes_from_corridor(&self.root, &mut route);
    }

    pub fn print_routes_from_corridor(&self, corridor: &MazeCorridor, route: &mut Vec<(i32, i32)>) {
        if route.is_empty() {
            route.push(corridor.start);
        }
        route.extend(&corridor.path);

        if corridor.children.is_empty() {
            println!("{:?}", route);
        } else {
            for child in &corridor.children {
                self.print_routes_from_corridor(child, route);
            }
        }

        route.truncate(route.len() - corridor.path.len());
    }
}

// fn build_corridor_tree(maze: Maze) {}

impl Maze {
    pub fn new(width: i32, height: i32, rng: ThreadRng) -> Maze {
        ABMaze::new(width, height, rng)
    }

    pub fn idx(&self, pos: IVec2) -> usize {
        ((pos.y * (self.width as i32)) + pos.x) as usize
    }

    pub fn in_bounds(&self, pos: IVec2) -> bool {
        (pos.x >= 0)
            && (pos.x < (self.width as i32))
            && (pos.y >= 0)
            && (pos.y < (self.height as i32))
    }

    pub fn num_exits(&self, pos: IVec2) -> usize {
        let mut exit_count = 0;
        for direction in 0..4 {
            if self.tiles[self.idx(pos)].exits[direction] == Exit::Open {
                exit_count += 1;
            }
        }
        exit_count
    }
}
