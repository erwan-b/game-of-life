pub mod board;
pub mod graphic;

use std::fs;
use crate::board::{Board, apply_rules_on_board};

pub fn create_file_from_map(file_path: &str) -> Board {
    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .trim().lines()
        .fold(Board::new(), |mut board, line| {
            board.add_row(line);
            board
        })
}

fn infini_loop(board: &Board) {
    let mut actual_board: &Board = board;
    let mut new_board: Board;

    loop {
        new_board = apply_rules_on_board(&actual_board);
        actual_board = &new_board;
    }
}

pub fn load_run(mapfile: &str) {
    let board = create_file_from_map(mapfile);
    infini_loop(&board);
}
