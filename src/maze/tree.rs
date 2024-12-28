use crate::*;

pub fn generate_tree(maze: Res<Maze>) {
    let mut prev_position = (maze.entry, -1);
    let mut position = (maze.entry, 0);

    let tree = MazeCorridorTree::new(position);
}

pub fn generate_corridor(maze: Maze, position: (i32, i32)) -> MazeCorridor {
    let corridor = MazeCorridor::new(position);

    match maze.num_exits(position.into()) {
        0 => {
            panic!("You're at a place with no exit?")
        }

        // Dead End
        1 => {}

        // Straight
        2 => {}

        // Branch
        3 => {}

        // Triple Branch
        4 => {}

        // Unreachable
        _ => {
            panic!("You shouldn't be here")
        }
    }

    corridor
}
