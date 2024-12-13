use filler::{Data, View};
use std::{
    error::Error,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut view = View::new()?;

    loop {
        io::stdin().read_to_string(&mut input)?;
        let data = Data::get(&input);
        view.display(&data.anfield)?;
        io::stdout().write("9 12".as_bytes())?;
    }
}
