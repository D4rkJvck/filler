mod anfield;
mod piece;

pub use {
    anfield::Anfield,
    piece::Piece,
};

pub type Matrix = Vec<Vec<char>>;
pub type Player = (char, char);

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width:  usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn fits_in(&self, other: &Self) -> bool {
        self.width < other.width && self.height < other.height
    }
}
