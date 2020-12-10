mod files;
mod graphic_lib;

pub enum STATUS {
    ALIVE,
    DEAD
}

pub struct Cell {
    x: i32,
    y: i32,
    status: STATUS
}

impl Cell {
    pub fn new(x: i32, y: i32, status: STATUS) -> Self {
        Cell{x, y, status}
    }

}
