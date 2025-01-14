use {
    super::{Matrix, Piece, Player, Position, Size},
    std::io::{self, BufRead},
};

#[derive(Default)]
pub struct Anfield {
    grid: Matrix,
    size: Size,
    filler: Player,
    foe: Player,
}

impl Anfield {
    pub fn new(size: Size, filler: Player, foe: Player) -> Self {
        let grid = vec![vec!['.'; size.x()]; size.y()];

        Self {
            grid,
            size,
            filler,
            foe,
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn update(&mut self, input: &mut impl BufRead) -> io::Result<()> {
        for y in 0..=self.size.y() {
            let mut line = String::new();

            input.read_line(&mut line)?;

            if line.len() > 4 {
                line = (&line[4..]).to_string();
            };

            line.trim()
                .chars()
                .enumerate()
                .for_each(|(x, c)| self.grid[y][x] = c)
        }

        Ok(())
    }

    pub fn valid_positions_for(&self, piece: &Piece) -> Vec<Position> {
        let mut valid_positions = Vec::new();

        for y in 0..self.size.y() {
            for x in 0..self.size.x() {
                let cell = Position::new(x, y);

                if piece.is_valid_on(cell, self) {
                    valid_positions.push(cell)
                }
            }
        }

        valid_positions
    }

    pub fn foe_territory(&self) -> Vec<Position> {
        let foe_positions = vec![];
        
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &cell)| {
                        if cell == self.foe.0 || cell == self.foe.1 {
                            Some(Position::new(x, y))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    pub fn is_lost_on(&self, position: Position) -> bool {
        let cell = self.grid[position.y()][position.x()];
        cell == self.foe.0 || cell == self.foe.1
    }

    pub fn is_won_on(&self, position: Position) -> bool {
        let cell = self.grid[position.y()][position.x()];
        cell == self.filler.0 || cell == self.filler.1
    }
}
