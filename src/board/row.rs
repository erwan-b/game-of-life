
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum STATUS {
    DEAD = 0,
    ALIVE = 1
}

pub struct Row {
    cells: Vec<Cell>
}

impl Row {
    pub fn new(row: Vec<Cell>) -> Self {
        Row{cells: row}
    }

    pub fn get(&self, index: usize) -> &Cell {
        self.cells.get(index).unwrap()
    }

    pub fn raw(&self) -> &Vec<Cell> {
        &self.cells
    }
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
