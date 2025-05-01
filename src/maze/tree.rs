use crate::*;

pub fn generate_tree(maze: Res<Maze>, commands: Commands) {
    let mut prev_position = (maze.entry, -1);
    let mut position = (maze.entry, 0);

    let tree = MazeCorridorTree::new(position, Direction::North);
    let corridor = generate_corridor(&maze, position);
}

pub fn generate_corridor(maze: &Maze, position: (i32, i32)) -> MazeCorridor {
    let corridor = MazeCorridor::new(position, Direction::North);
    let exits = maze.exits(position.into());
    let exit_count = exits.iter().flatten().count();
    println!("Exit count of {:?}: {}", position, exit_count);

    match exit_count {
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
