use crate::Matrix;

pub fn get_size(input: &str) -> (usize, usize) {
    let input = input.trim_end_matches(':');
    let tab: Vec<usize> = input.split_whitespace().collect::<Vec<&str>>()[1..]
        .iter()
        .filter(|line| line.parse::<usize>().is_ok())
        .map(|s| s.parse().unwrap())
        .collect();

    (tab[0], tab[1])
}

pub fn matricize(lines: Vec<&str>) -> Matrix {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Matrix>()
}
