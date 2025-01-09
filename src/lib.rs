mod board;

use std::{
    fs::OpenOptions,
    io::{
        Result,
        Write,
    },
};

pub use board::{
    Anfield,
    Piece,
};

pub fn log(output: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("error.log")?;

    file.write_all(format!("{}\n\n", output).as_bytes())
}
