mod anfield;
mod piece;

pub use {anfield::Anfield, piece::Piece};

pub type Matrix = Vec<Vec<char>>;
pub type Player = (char, char);

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector(usize, usize);
pub type Position = Vector;
pub type Size = Vector;

impl Vector {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn fits_in(&self, size: Size) -> bool {
        self.x() < size.x() && self.y() < size.y()
    }
}
