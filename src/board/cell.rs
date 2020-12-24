
/// Define a cell alive or dead
/// This should not be access by something else than the cell
/// [TODO] Remove the pub
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum STATUS {
    DEAD = 0,
    ALIVE = 1
}

impl STATUS {
    pub fn get_char(&self) -> char {
        if STATUS::DEAD == *self {
            '0'
        } else {
            '1'
        }

    }

    pub fn get_from_char(s: char) -> STATUS {
        if s == '0' {
            STATUS::DEAD
        } else if s == '1' {
            STATUS::ALIVE
        } else {
            panic!("Wrong format")
        }
    }

    pub fn is_alive(&self) -> bool {
       *self == STATUS::ALIVE
    }
}

/// Define a cell of the board
/// It has a position on it and a status
#[derive(Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub status: STATUS
}

impl Cell {
    pub fn new(x: i32, y: i32, status: STATUS) -> Self {
        Cell{x, y, status}
    }

    pub fn is_alive(&self) -> bool {
        self.status == STATUS::ALIVE
    }

    /// Apply the game of life rules on this cell
    pub fn apply_rules(&self, adj_live_cells: usize) -> Cell {
        if self.is_alive() && adj_live_cells > 3 || adj_live_cells < 2 {
            Cell::new(self.x, self.y, STATUS::DEAD)
        } else if self.is_alive() {
            Cell::new(self.x, self.y, STATUS::ALIVE)
        } else if !self.is_alive() && adj_live_cells == 3 {
            Cell::new(self.x, self.y, STATUS::ALIVE)
        } else {
            Cell::new(self.x, self.y, STATUS::DEAD)
        }
    }
}
