use {
    super::{
        utils::get_data,
        Matrix,
        Size,
    },
    std::io::{
        BufRead,
        Result,
    },
};

pub struct Piece {
    grid: Matrix,
    size: Size,
}

impl Piece {
    pub fn get(input: &mut impl BufRead) -> Result<Self> {
        let (grid, size) = get_data(input, "Piece")?;
        Ok(Self { grid, size })
    }
}
