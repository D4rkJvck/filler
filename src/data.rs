use crate::{
    utils::{get_size, matricize},
    Matrix, Size,
};

#[derive(Debug, PartialEq)]
pub struct Data {
    pub player_id: u8,
    pub anfield_size: Size,
    pub anfield: Matrix,
    pub piece_size: Size,
    pub piece: Matrix,
}

impl Data {
    pub fn get_from(input: &str) -> Self {
        let lines: Vec<&str> = input.split("\n").collect();

        let player_id = if lines.first().unwrap().contains("filler") {
            1
        } else {
            2
        };

        let anfield_size = get_size(lines[2]);
        let anfield = matricize(lines[4..anfield_size.1 + 4].to_vec());

        let piece_size = get_size(lines[anfield_size.1 + 4]);
        let piece = matricize(lines[anfield_size.1 + 5..lines.len() - 1].to_vec());

        Self {
            player_id,
            anfield_size,
            anfield,
            piece_size,
            piece,
        }
    }
}
