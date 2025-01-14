use super::{
    Matrix,
    Piece,
    Player,
    Position,
    Size,
};

#[derive(Debug, Clone)]
pub struct Anfield {
    pub grid:   Matrix,
    pub size:   Size,
    pub filler: Player,
    pub foe:    Player,
}

impl Anfield {
    pub fn new(size: Size, filler: Player, foe: Player) -> Self {
        Self {
            grid: vec![vec!['.'; size.width]; size.height],
            size,
            filler,
            foe,
        }
    }

    pub fn update(&mut self, board_str: &str) {
        for (y, line) in board_str.lines().enumerate() {
            if y >= self.size.height {
                break;
            }

            for (x, c) in line.chars().enumerate() {
                if x >= self.size.width {
                    break;
                }

                self.grid[y][x] = c;
            }
        }
    }

    pub fn is_valid_placement(
        &self,
        piece: &Piece,
        pos: &Position,
    ) -> bool {
        let mut overlap_count = 0;

        for piece_y in 0..piece.size.height {
            for piece_x in 0..piece.size.width {
                let board_x = pos.x + piece_x as i32;
                let board_y = pos.y + piece_y as i32;

                if piece.grid[piece_y][piece_x] == 'O' {
                    // Check boundaries
                    if !self.is_within_bounds(board_x, board_y) {
                        return false;
                    }

                    let board_cell =
                        self.grid[board_y as usize][board_x as usize];

                    if board_cell == self.foe.0 || board_cell == self.foe.1
                    {
                        return false;
                    }

                    if board_cell == self.filler.0
                        || board_cell == self.filler.1
                    {
                        overlap_count += 1;
                        if overlap_count < 1 {
                            return false;
                        }
                    }
                }
            }
        }

        overlap_count == 1
    }

    fn is_within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0
            && x < self.size.width as i32
            && y >= 0
            && y < self.size.height as i32
    }

    pub fn find_best_move(&self, piece: &Piece) -> Option<Position> {
        let possible_moves = self.get_all_valid_moves(piece);

        self.find_shortest_distance(possible_moves)
    }

    fn get_all_valid_moves(&self, piece: &Piece) -> Vec<Position> {
        let mut valid_moves = Vec::new();

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let pos = Position::new(x as i32, y as i32);

                if self.is_valid_placement(piece, &pos) {
                    valid_moves.push(pos);
                }
            }
        }

        valid_moves
    }

    pub fn get_opponent_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let cell = self.grid[y][x];
                if cell == self.foe.0 || cell == self.foe.1 {
                    positions.push(Position::new(
                        x as i32, y as i32,
                    ));
                }
            }
        }

        positions
    }

    pub fn find_shortest_distance(
        &self,
        possible_moves: Vec<Position>,
    ) -> Option<Position> {
        let opponent_positions = self.get_opponent_positions();

        if possible_moves.is_empty() || opponent_positions.is_empty() {
            return None;
        }

        let mut shortest_distance = i32::MAX;
        let mut closest_position = None;

        for player_pos in &possible_moves {
            for opponent_pos in &opponent_positions {
                let distance = player_pos.manhattan_distance(opponent_pos);

                if distance < shortest_distance {
                    shortest_distance = distance;
                    closest_position = Some(player_pos.clone());
                }
            }
        }

        closest_position
    }
}
