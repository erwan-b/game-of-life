use crate::game_of_life::Cell;
use crate::game_of_life::STATUS;
use std::fs;
use std::env;

fn cell_status(elem: char) -> STATUS {
   if elem == '0' {
       STATUS::DEAD
   } else if elem == '1' {
       STATUS::ALIVE
   } else {
       panic!("Wrong format")
   }
}

pub fn read_file(file_path: &str) -> Vec<Cell>{
    let mut pos = Cell::new(0, 0, STATUS::DEAD);

    fs::read_to_string(file_path).expect("Something went wrong reading the file").chars()
        .filter_map(|elem| {
            if elem == '\n' {
                pos.x = 0;
                pos.y += 1;
                return None;
            } else {
                Some(Cell::new(pos.x, pos.y, cell_status(elem)))
            }
        }).collect()
}
