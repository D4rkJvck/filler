use {
    crate::board::{
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

pub fn get_data(
    input: &mut impl BufRead,
    str: &str,
) -> Result<(Matrix, Size)> {
    let mut line = String::new();

    while !line.contains(str) {
        line.clear();
        input.read_line(&mut line)?;
    }

    let params: Vec<&str> = line
        .trim()
        .split_whitespace()
        .collect();

    if params.len() < 3 || params[0] != str {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid Format",
        ));
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

    let mut grid = vec![vec!['.'; size.x()]; size.y()];

    for y in 0..size.y() {
        line.clear();
        input.read_line(&mut line)?;

        line.trim()
            .chars()
            .enumerate()
            .for_each(|(x, c)| {
                if x < size.x() {
                    grid[y][x] = c
                }
            })
    }

    Ok((grid, size))
}
