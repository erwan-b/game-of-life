pub enum STATUS {
    ALIVE,
    DEAD
}

pub struct Row {
    cells: Vec<Cell>,
}

impl Row {
    pub fn new(row: Vec<Cell>) -> Self {
        Row{cells: row}
    }

    pub fn get(&self, index: usize) -> &Cell {
        self.cells.get(index).unwrap()
    }
}

pub struct Cell {
    x: usize,
    y: usize,
    status: STATUS
}

impl Cell {
    pub fn new(x: usize, y: usize, status: STATUS) -> Self {
        Cell{x, y, status}
    }
}
