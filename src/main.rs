// use filler::Data;
use std::{
    // error::Error,
    io::{self, BufRead, Write},
};

fn main() {
    let mut input = String::new();
    let mut file = std::fs::File::create("output.txt").expect("Unable to create file");

    for line in io::stdin().lock().lines() {
        let line = line.unwrap_or_default();

        if line.trim().is_empty() {
            file.write_all("\nEnd of Input\n".as_bytes())
                .expect("Unable to write...");
            continue;
        }

        file.write_all(format!("{}\n", line).as_bytes())
            .expect("Unable to write data");

        input += format!("{}\n", line).as_str();

        println!("9 12")
    }
}
