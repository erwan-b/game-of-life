use game_of_life as gl;
use gl::board::Board;
use game_of_life::board::cell::STATUS;

#[test]
fn test_board_add_one_elem_row() {
    let mut b = Board::new(10, vec!["0"]);

    assert_eq!(b.get_cell(0, 0).status, STATUS::DEAD);
    assert_eq!(b.get_row(0).len(), 10);
    assert_eq!(b.nb_row(), 10);
}


#[test]
fn test_board_add_two_elem_row() {
    let mut b = Board::new(10, vec!["01"]);

    assert_eq!(b.get_cell(0, 0).status, STATUS::DEAD);
    assert_eq!(b.get_cell(0, 1).status, STATUS::ALIVE);
    assert_eq!(b.get_row(0).len(), 10);
    assert_eq!(b.nb_row(), 10);
}

#[test]
fn test_multipleboard_add_row() {
    let mut b = Board::new(10, vec!["0000000000",  "0000000000"]);

    assert_eq!(b.get_cell(0, 0).status, STATUS::DEAD);
    assert_eq!(b.get_row(0).len(), 10);
    assert_eq!(b.nb_row(), 10);
}