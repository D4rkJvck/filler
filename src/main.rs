use std::{error::Error, fs, io};

// use filler::Data;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // let data = Data::get_from(&input);

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open("input.txt")?;

        io::Write::write_all(&mut file, input.as_bytes())?;

        println!("0 0/n")
    }
}
