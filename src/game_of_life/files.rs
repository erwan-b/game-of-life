use crate::game_of_life::board::Board;
use std::fs;

pub fn read_file(file_path: &str) -> Board {
    let mut b = Board::new();

    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file").trim().lines()
        .fold(0, |x, line| {
            b.add_row(line, x);
            x + 1
        });

    b
}
