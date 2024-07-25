#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => { Direction::South }
            Direction::East => { Direction::West }
            Direction::South => { Direction::North }
            Direction::West => { Direction::East }
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => { 0 }
            Direction::East => { 1 }
            Direction::South => { 2 }
            Direction::West => { 3 }
        }
    }
}