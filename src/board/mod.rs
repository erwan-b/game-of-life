pub mod cell;

use cell::{Cell, STATUS};

pub struct Board {
    rows: Vec<Vec<Cell>>,
}

/// Define the board logic
impl Board {
    /// Construct an empty board
    pub fn new() -> Self {
        let mut rows = vec![vec![]; 10];

        rows = rows.iter().enumerate().map(|(index, elem)| {
            let mut line: Vec<Cell> = Vec::with_capacity(10);
            for i in 0..10 {
                line.push(Cell::new(index as i32, i, STATUS::DEAD))
            }
            line
        }).collect();

        Board{rows}
    }

    /// Apply the game of life rules
    fn apply_rules(status: STATUS, adj_live_cells: usize) -> STATUS {
        if status == STATUS::ALIVE && adj_live_cells > 3 || adj_live_cells < 2 {
            STATUS::DEAD
        } else if status == STATUS::ALIVE {
            STATUS::ALIVE
        } else if status == STATUS::DEAD && adj_live_cells == 3 {
            STATUS::ALIVE
        } else {
            STATUS::DEAD
        }
    }

    pub fn nb_row(&self) -> usize {
        self.rows.len()
    }

    pub fn get_line(&self, index: usize) -> String {
        self.rows.get(index).unwrap().iter()
            .map(|cell| cell.status.get_char()).collect()
    }

    pub fn add_line(&mut self, line: &str) {
        let y = self.nb_row() as i32;

        let vec = line.chars().enumerate().map(|(x, elem)| {
                Cell::new(x as i32, y, STATUS::get_from_char(elem))
            }).collect::<Vec<Cell>>();
        self.rows.push(vec);
    }

    pub fn get_row(&self, x: usize) -> &Vec<Cell> {
        self.rows.get(x).unwrap()
    }

    /// Get cell from a specific position on the board
    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.rows.get(y as usize)?.get(x as usize)
    }

    /// Get status for a specific position on the board
    pub fn get_cell_status(&self, x: i32, y: i32) -> STATUS {
        match self.get_cell(x, y) {
            None => STATUS::DEAD,
            Some(&cell) => cell.status
        }
    }

    /// Get all adjacent cells status
    fn get_adj_cells_status(&self, pos: &Cell) -> Vec<STATUS> {
        vec![self.get_cell_status(pos.x - 1, pos.y - 1),
             self.get_cell_status(pos.x    , pos.y - 1),
             self.get_cell_status(pos.x + 1, pos.y - 1),

             self.get_cell_status(pos.x - 1, pos.y),
             self.get_cell_status(pos.x + 1, pos.y),

             self.get_cell_status(pos.x - 1, pos.y  + 1),
             self.get_cell_status(pos.x    , pos.y  + 1),
             self.get_cell_status(pos.x + 1, pos.y  + 1)]
    }

    /// Apply the game of life rules on a certain position on the board
    fn apply_on_pos(&self, pos: &Cell) -> STATUS {
        let adj_live_cells = self.get_adj_cells_status(pos).iter()
            .filter(|&&elem| elem == STATUS::ALIVE).count();
        Board::apply_rules(pos.status, adj_live_cells)
    }

    /// Apply the game of life rules on a row of the board
    fn apply_on_row(&self, row: &Vec<Cell>) -> Vec<Cell> {
        row.iter().map(|cell|
            Cell::new(cell.x, cell.y, self.apply_on_pos(cell))
        ).collect()
    }

    /// Apply the game of life rules on the board
    pub fn apply_on_all(&mut self) {
        self.rows = self.rows.iter()
            .map(|row| self.apply_on_row(row))
            .collect();
    }
}
