use {
    filler::{
        log,
        Anfield,
        Piece,
    },
    std::io::{
        self,
        Result,
        Write,
    },
};

fn main() -> Result<()> {
    let input = io::stdin();
    let mut input = input.lock();
    let mut output = io::stdout();
    
    let mut anfield = Anfield::get(&mut input)?;

    loop {
        if let Err(_) = anfield.update(&mut input) {
            continue;
        }

        match Piece::get(&mut input) {
            Err(_) => {
                println!("0 0");
                output.flush()?;
            }
            Ok(_) => {
                println!("9 12");
                output.flush()?;
            }
        }
    }
}
