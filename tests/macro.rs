use filler::get_closest_opponent_cell_position;

#[test]
fn test_closest_opponent_cell_player_1() {
    let matrix =
        vec![vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '@', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '$', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.',],];

    assert_eq!(get_closest_opponent_cell_position!(1, matrix),
               (9, 12))
}

#[test]
fn test_closest_opponent_cell_player_2() {
    let matrix =
        vec![vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '@', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '$',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],
             vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  '.', '.', '.', '.', '.', '.', '.',],];

    assert_eq!(get_closest_opponent_cell_position!(2, matrix),
               (4, 3));
}
