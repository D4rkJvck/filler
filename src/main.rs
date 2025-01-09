use {
    filler::{
        Anfield,
        Piece,
    },
    std::io::{
        self,
        BufRead,
        Write,
    },
};

fn main() -> io::Result<()> {
    let mut line = String::new();
    let mut input = io::stdin().lock();
    let mut output = io::stdout();

    input.read_line(&mut line)?;

    let is_p1 = line.contains("p1");
    let filler = if is_p1 { ('a', '@') } else { ('s', '$') };
    let foe = if is_p1 { ('s', '$') } else { ('a', '@') };

    let mut anfield = Anfield::get(&mut input, (filler, foe))?;

    loop {
        anfield.update(&mut input)?;

        match Piece::get(&mut input) {
            Err(_) => {
                println!("0 0");
                output.flush()?;
            }
            Ok(_) => {
                
            }
        }
    }
}
