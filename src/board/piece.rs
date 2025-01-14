use {
    super::{
        Matrix,
        Size,
    },
    std::io::{
        BufRead,
        Error,
        ErrorKind,
        Result,
    },
};

#[derive(Debug, Clone)]
pub struct Piece {
    pub grid: Matrix,
    pub size: Size,
}

impl Piece {
    pub fn get(input: &mut impl BufRead) -> Result<Self> {
        let mut line = String::new();

        while !line.contains("Piece") {
            line.clear();
            input.read_line(&mut line)?;
        }

        let params: Vec<&str> = line
            .trim()
            .split_whitespace()
            .collect();

        if params.len() < 3 || params[0] != "Piece" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid piece format",
            ));
        }

        let width = params[1]
            .parse()
            .unwrap_or_default();

        let height = params[2]
            .trim_end_matches(':')
            .parse()
            .unwrap_or_default();

        let mut grid = vec![vec!['.'; width]; height];

        for y in 0..height {
            line.clear();

            input.read_line(&mut line)?;

            for (x, c) in line
                .trim()
                .chars()
                .enumerate()
            {
                if x < width {
                    grid[y][x] = c;
                }
            }
        }

        let size = Size::new(width, height);

        Ok(Self { grid, size })
    }
}
