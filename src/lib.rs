mod board;
mod utils;

use std::{fs::OpenOptions, io::Write};

pub use board::{Anfield, Piece, Size};
pub use utils::get_data;

pub fn log(output: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("error.log")
        .expect("Failed to open or create file");

    file.write_all(format!("{}\n\n", output).as_bytes())
        .expect("Failed to write in file");
}
