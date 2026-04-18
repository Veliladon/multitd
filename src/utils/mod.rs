use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn as_vec(&self) -> IVec2 {
        match self {
            Direction::North => IVec2 { x: 0, y: 1 },
            Direction::East => IVec2 { x: 1, y: 0 },
            Direction::South => IVec2 { x: 0, y: -1 },
            Direction::West => IVec2 { x: -1, y: 0 },
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

pub const DIRECTIONS: [IVec2; 4] = [
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: -1 },
    IVec2 { x: -1, y: 0 },
];
