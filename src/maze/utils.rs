pub use crate::prelude::*;
pub use rand::prelude::*;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Exit {
    Open,
    #[default]
    Closed,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cell {
    pub north: Exit,
    pub east: Exit,
    pub south: Exit,
    pub west: Exit,
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
struct MazeCorridor {
    start: (usize, usize),       // Starting position of the corridor
    path: Vec<(usize, usize)>,   // Linear path through the corridor
    children: Vec<MazeCorridor>, // Corridors branching out from the end
}

impl MazeCorridor {
    fn new(start: (usize, usize)) -> Self {
        MazeCorridor {
            start,
            path: Vec::new(),
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: MazeCorridor) {
        self.children.push(child);
    }
}

#[derive(Debug)]
struct MazeCorridorTree {
    root: MazeCorridor,
}

impl MazeCorridorTree {
    fn new(start_position: (usize, usize)) -> Self {
        MazeCorridorTree {
            root: MazeCorridor::new(start_position),
        }
    }

    fn print_routes(&self) {
        let mut route = Vec::new();
        self.print_routes_from_corridor(&self.root, &mut route);
    }

    fn print_routes_from_corridor(&self, corridor: &MazeCorridor, route: &mut Vec<(usize, usize)>) {
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

fn build_corridor_tree(
    maze: &[Vec<u8>],
    corridor: &mut MazeCorridor,
    visited: &mut Vec<Vec<bool>>,
) {
    let mut current = corridor.start;
    let mut path = Vec::new();

    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    loop {
        visited[current.0][current.1] = true;

        // Count valid moves from the current position
        let mut valid_moves = Vec::new();
        for (dx, dy) in &moves {
            let nx = current.0 as isize + dx;
            let ny = current.1 as isize + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < maze.len()
                && (ny as usize) < maze[0].len()
                && maze[nx as usize][ny as usize] == 0
                && !visited[nx as usize][ny as usize]
            {
                valid_moves.push((nx as usize, ny as usize));
            }
        }

        match valid_moves.len() {
            0 => {
                // Dead-end: end the corridor
                corridor.path = path;
                break;
            }
            1 => {
                // Continue along the single valid path
                current = valid_moves[0];
                path.push(current);
            }
            _ => {
                // Split: create child corridors for each valid move
                corridor.path = path.clone();
                for next_position in valid_moves {
                    let mut child_corridor = MazeCorridor::new(next_position);
                    build_corridor_tree(maze, &mut child_corridor, visited);
                    corridor.add_child(child_corridor);
                }
                break;
            }
        }
    }
}

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

    pub fn num_valid_moves(&self, pos: IVec2) -> usize {
        todo!();
    }
}
