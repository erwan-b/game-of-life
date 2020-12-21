#[derive(Clone, Copy, PartialEq, Debug)]
pub enum STATUS {
    DEAD = 0,
    ALIVE = 1
}

pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub status: STATUS
}

impl Cell {
    pub fn new(x: usize, y: usize, status: STATUS) -> Self {
        Cell{x, y, status}
    }
}
