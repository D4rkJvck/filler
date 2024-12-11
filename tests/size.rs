use filler::get_size;

#[test]
fn test_get_size_anfield_map00() {
    let input = "Anfield 20 15:";
    assert_eq!(get_size(input), (20, 15));
}

#[test]
fn test_get_size_piece_4_3() {
    let input = "Piece 4 3:";
    assert_eq!(get_size(input), (4, 3));
}

#[test]
fn test_get_size_anfield_map01() {
    let input = "Anfield 40 30:";
    assert_eq!(get_size(input), (40, 30));
}

#[test]
fn test_get_size_piece_4_4() {
    let input = "Piece 4 4:";
    assert_eq!(get_size(input), (4, 4));
}

#[test]
fn test_get_size_anfield_map02() {
    let input = "Anfield 99 100:";
    assert_eq!(get_size(input), (99, 100));
}

#[test]
fn test_get_size_piece_17_3() {
    let input = "Piece 17 3:";
    assert_eq!(get_size(input), (17, 3));
}
