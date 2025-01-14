use {
    super::{Anfield, Matrix, Position, Size},
    crate::get_data,
    std::io::{BufRead, Result},
};

#[derive(Debug)]
pub struct Piece {
    grid: Matrix,
    size: Size,
}

impl Piece {
    pub fn get(input: &mut impl BufRead) -> Result<Self> {
        let (grid, size) = get_data(input, "Piece")?;
        Ok(Self { grid, size })
    }

    pub fn best_position(&self, anfield: &Anfield) -> Option<Position> {
        let valid_positions = anfield.valid_positions_for(self);
        
    }

    pub fn is_valid_on(
        &self,
        position: Position,
        anfield: &Anfield,
    ) -> bool {
        let mut overlaps = 0;

        for y in 0..self.size.y() {
            for x in 0..self.size.x() {
                let cell = Position::new(
                    position.x() + x,
                    position.y() + y,
                );

                if self.grid[y][x] != 'O' {
                    continue;
                }

                if cell.fits_in(anfield.size()) || anfield.is_lost_on(cell)
                {
                    return false;
                }

                if anfield.is_won_on(cell) {
                    overlaps += 1;
                }
            }
        }

        overlaps == 1
    }
}
