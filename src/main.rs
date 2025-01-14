use {
    filler::{
        Anfield,
        Piece,
        Size,
    },
    std::io::{
        self,
        BufRead,
        Write,
    },
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    let mut line = String::new();

    input.read_line(&mut line)?;

    let is_p1 = line.contains("p1");
    let filler = if is_p1 { ('@', 'a') } else { ('$', 's') };
    let foe = if is_p1 { ('$', 's') } else { ('@', 'a') };

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
        }

        let width = params[1]
            .parse()
            .unwrap_or_default();

        let height = params[2]
            .trim_end_matches(':')
            .parse()
            .unwrap_or_default();

        if width == 0 || height == 0 {
            continue;
        }

        let size = Size::new(width, height);
        let mut anfield = Anfield::new(size, filler, foe);
        let mut anfield_str = String::new();

        for _ in 0..=anfield.size.height {
            line.clear();
            input.read_line(&mut line)?;

            if line.len() > 4 {
                anfield_str.push_str(&line[4..]);
            }
        }

        anfield.update(&anfield_str[anfield.size.width as usize + 1..]);

        if let Ok(piece) = Piece::get(&mut input) {
            if let Some(best_move) = anfield.find_best_move(&piece) {
                writeln!(
                    output,
                    "{} {}",
                    best_move.x, best_move.y
                )?;
            }
            else {
                writeln!(output, "0 0")?;
            }
            output.flush()?;
        }
    }

    Ok(())
}
