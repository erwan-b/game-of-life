pub mod row;

use row::{Cell, STATUS};
use ggez::graphics::apply_transformations;

pub struct Board {
    rows: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        Board{rows: vec![]}
    }

    fn cell_status(elem: char) -> STATUS {
        if elem == '0' {
            STATUS::DEAD
        } else if elem == '1' {
            STATUS::ALIVE
        } else {
            panic!("Wrong format")
        }
    }

    pub fn nb_row(&self) -> usize {
        self.rows.len()
    }

    pub fn add_row(&mut self, line: Vec<Cell>) -> &Self {
        self.rows.push(line);

        self
    }

    pub fn add_line(&mut self, line: &str) -> &Self {
        let mut y: usize = 0;
        let x: usize = self.rows.len();

        let vec = line.chars()
            .map(|elem| {
                let status = Board::cell_status(elem);

                y += 1;
                Cell::new(x, y, status)
            }).collect::<Vec<Cell>>();
        self.rows.push(vec);

        self
    }

    pub fn get_row(&self, x: usize) -> &Vec<Cell> {
        self.rows.get(x).unwrap()
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        self.rows.get(x).unwrap().get(y).unwrap()
    }

    fn get_adj_cells(&self, pos: &Cell) -> Vec<&Cell> {
        vec![self.get_cell(pos.x - 1, pos.y - 1),
        self.get_cell(pos.x - 1, pos.y),
        self.get_cell(pos.x - 1, pos.y + 1),

        self.get_cell(pos.x, pos.y - 1),
        self.get_cell(pos.x, pos.y + 1),

        self.get_cell(pos.x + 1, pos.y - 1),
        self.get_cell(pos.x + 1, pos.y),
        self.get_cell(pos.x + 1, pos.y + 1)]
    }

    fn get_adj_cells_status(&self, pos: &Cell) -> Vec<STATUS> {
        self.get_adj_cells(pos).iter()
            .map(|&cell| cell.status)
            .collect()
    }

    fn next_status_from_pos(&self, pos: &Cell) -> STATUS {
        let adj_live_cells = self.get_adj_cells_status(pos).iter()
            .filter(|&&elem| elem == STATUS::ALIVE).count();
        if pos.status == STATUS::ALIVE {
            if adj_live_cells > 3 || adj_live_cells < 2 {
                STATUS::DEAD
            } else {
                STATUS::ALIVE
            }
        } else {
            if adj_live_cells == 3 {
                STATUS::ALIVE
            } else {
                STATUS::DEAD
            }
        }
    }

    fn apply_on_row(&self, row: &Vec<Cell>) -> Vec<Cell> {
        row.iter()
            .map(|cell| Cell::new(cell.x, cell.y, self.next_status_from_pos(cell)))
            .collect()
    }

    fn apply_on_all(&self) -> Board {
        let mut board = Board::new();

        self.rows.iter()
            .map(|row| self.apply_on_row(row))
            .for_each(|elem| {
                board.add_row(elem);
            });


        board
    }
}
