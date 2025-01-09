mod anfield;
mod piece;
mod utils;

pub(self) use utils::get_data;
pub use {
    anfield::Anfield,
    piece::Piece,
};

pub type Matrix = Vec<Vec<char>>;
pub type Player = (char, char);

pub struct Vector(usize, usize);
pub type Position = Vector;
pub type Size = Vector;

impl Vector {
    pub fn new(x: usize, y: usize) -> Self { Self(x, y) }

    pub fn x(&self) -> usize { self.0 }

    pub fn y(&self) -> usize { self.1 }
}
