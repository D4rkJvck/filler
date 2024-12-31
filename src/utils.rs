use crate::Matrix;

pub fn get_size(input: &str) -> (usize, usize) {
    let input = input.trim_end_matches(':');
    let tab: Vec<usize> =
        input.split_whitespace()
             .collect::<Vec<&str>>()[1..]
                                         .iter()
                                         .filter(|line| {
                                             line.parse::<usize>().is_ok()
                                         })
                                         .map(|s| s.parse().unwrap())
                                         .collect();

    (tab[0], tab[1])
}

pub fn matricize(lines: Vec<&str>) -> Matrix {
    lines.iter()
         .map(|line| {
             line.split_whitespace()
                 .last()
                 .unwrap()
                 .chars()
                 .collect::<Vec<char>>()
         })
         .collect::<Matrix>()
}

// pub fn get_closest_opponent_cell_position(matrix: &Matrix) -> (usize,
// usize) {     (0, 0)
// }

pub fn transpose(matrix: &Matrix, piece: &Matrix) -> Result<(), ()> {
    Ok(())
}

#[macro_export]
macro_rules! get_closest_opponent_cell_position {
    (1, $matrix:expr) => {
        {
            let (mut x, mut y) = (0, 0);

            'outer: for i in 0..$matrix.len() {
                for j in 0..$matrix[y].len() {
                    if $matrix[i][j] == '$' || $matrix[i][j] == 's' {
                        (x, y) = (j, i);
                        break 'outer;
                    }
                }
            }

            (x, y)
        }
    };

    (2, $matrix:expr) => {
        {
            let (mut x, mut y) = (0, 0);

            'outer: for i in (0..$matrix.len()).rev() {
                for j in (0..$matrix[y].len()).rev() {
                    if $matrix[i][j] == '@' || $matrix[i][j] == 'a' {
                        (x, y) = (j, i);
                        break 'outer
                    }
                }
            }

            (x, y)
        }
    }
}
