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
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub const NORTH: usize = Direction::North as usize;
pub const EAST: usize = Direction::East as usize;
pub const SOUTH: usize = Direction::South as usize;
pub const WEST: usize = Direction::West as usize;

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(direction: usize) -> Result<Self, Self::Error> {
        match direction {
            NORTH => Ok(Direction::North),
            EAST => Ok(Direction::East),
            SOUTH => Ok(Direction::South),
            WEST => Ok(Direction::West),
            _ => Err(()),
        }
    }
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
    start: (i32, i32),
    entrance_direction: Direction, // Starting position of the corridor
    path: Vec<(i32, i32)>,         // Linear path through the corridor
    children: Vec<MazeCorridor>,   // Corridors branching out from the end
}

impl MazeCorridor {
    pub fn new(start: (i32, i32), entrance_direction: Direction) -> Self {
        MazeCorridor {
            start,
            entrance_direction,
            path: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: MazeCorridor) {
        self.children.push(child);
    }
}

#[derive(Debug, Resource)]
pub struct MazeCorridorTree {
    root: MazeCorridor,
}

impl MazeCorridorTree {
    pub fn new(start_position: (i32, i32), entrance_direction: Direction) -> Self {
        MazeCorridorTree {
            root: MazeCorridor::new(start_position, entrance_direction),
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

    pub fn exits(&self, pos: IVec2) -> [Option<Direction>; 4] {
        let mut exits = [None; 4];
        for direction in 0..4 {
            if self.tiles[self.idx(pos)].exits[direction] == Exit::Open {
                exits[direction] = Some(direction.try_into().unwrap());
            }
        }
        exits
    }
}
