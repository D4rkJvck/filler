use {
    filler::{get_data, log, Anfield, Piece, Size},
    std::io::{self, BufRead, Result, Write},
};

fn main() -> Result<()> {
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    let mut line = String::new();

    input.read_line(&mut line)?;

    let is_p1 = line.contains("p1");
    let filler = if is_p1 { ('a', '@') } else { ('s', '$') };
    let foe = if is_p1 { ('s', '$') } else { ('a', '@') };

    loop {
        line.clear();

        if input.read_line(&mut line)? == 0 {
            break;
        }

        let params: Vec<&str> = line
            .trim()
            .split_whitespace()
            .collect();

        if params.len() < 3 || params[0] != "Anfield" {
            continue;
        };

        let size = Size::new(
            params[1]
                .parse()
                .unwrap_or_default(),
            params[2]
                .trim_end_matches(':')
                .parse()
                .unwrap_or_default(),
        );

        if size.x() == 0 || size.y() == 0 {
            continue;
        }

        let mut anfield = Anfield::new(size, filler, foe);

        if let Err(_) = anfield.update(&mut input) {
            break;
        }

        match Piece::get(&mut input) {
            Err(_) => log("Failed to get the piece"),
            Ok(_) => {
                writeln!(output, "9 12")?;
                output.flush()?;
            }
        }
    }

    Ok(())
}
