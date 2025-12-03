
#[derive(Copy, Clone)]
pub enum Direction {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn delta(&self, dir: Direction) -> Point {
        match dir {
            Direction::NorthEast => Point {x: self.x - 1, y: self.y + 1},
            Direction::SouthEast => Point {x: self.x + 1, y: self.y + 1},
            Direction::NorthWest => Point {x: self.x + 1, y: self.y - 1},
            Direction::SouthWest => Point {x: self.x - 1, y: self.y - 1},
        }
    }
}

impl Direction {
    pub fn delta(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::NorthEast => (x - 1, y + 1),
            Direction::SouthEast => (x + 1, y + 1),
            Direction::SouthWest => (x + 1, y - 1),
            Direction::NorthWest => (x - 1, y - 1),
        }
    }
    pub const ALL: [Direction; 4] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];
    fn opposite(&self) -> Direction {
        match self {
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::NorthWest => Direction::SouthEast,
        }
    }
    pub fn left(&self) -> Direction {
        match self {
            Direction::NorthEast => Direction::NorthWest,
            Direction::SouthEast => Direction::NorthEast,
            Direction::SouthWest => Direction::SouthEast,
            Direction::NorthWest => Direction::SouthWest,
        }
    }
    pub fn right(&self) -> Direction {
        match self {
            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
}

