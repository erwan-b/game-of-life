use game_of_life as gl;
use gl::board::Board;
use game_of_life::board::row::STATUS;

#[test]
fn test_board_add_one_elem_row() {
    let mut b = Board::new();
    b.add_row("0");

    assert_eq!(b.get_cell(0, 0).status, STATUS::DEAD);
    assert_eq!(b.get_row(0).raw().len(), 1);
    assert_eq!(b.nb_row(), 1);
}


#[test]
fn test_board_add_two_elem_row() {
    let mut b = Board::new();
    b.add_row("01");

    assert_eq!(b.get_cell(0, 0).status, STATUS::DEAD);
    assert_eq!(b.get_cell(0, 1).status, STATUS::ALIVE);
    assert_eq!(b.get_row(0).raw().len(), 2);
    assert_eq!(b.nb_row(), 1);
}

#[test]
fn test_multipleboard_add_row() {
    let mut b = Board::new();
    b.add_row("0000000000");
    b.add_row("0000000000");

    assert_eq!(b.get_cell(0, 0).status, STATUS::DEAD);
    assert_eq!(b.get_row(0).raw().len(), 10);
    assert_eq!(b.nb_row(), 2);
}