pub mod cell;

use cell::{Cell, STATUS};
use std::collections::VecDeque;

pub struct Board {
    default_size: usize,
    rows: Vec<Vec<Cell>>,
    history: VecDeque<Box<Vec<Cell>>>
}

/// Define the board logic
impl Board {
    /// Construct an empty board
    /// TODO make this more readable
    pub fn new(size: usize, obj: Vec<&str>) -> Self {
        let obj_b: Vec<Vec<char>> = obj.iter().map(|&s| s.chars().collect()).collect();
        let get_status = |x: i64, y: i64| -> Option<STATUS> {
            if x <= 0 && y <= 0 {
                None
            } else {
                let &c = obj_b.get(y as usize)?.get(x as usize)?;
                Some(STATUS::get_from_char(c))
            }
        };

        let mut rows = Vec::with_capacity(size);

        (0..size as i64).for_each(|y| {
            let mut line: Vec<Cell> = Vec::with_capacity(size);

            (0..size as i64).for_each(|x| {
                let res = match get_status(
                    ((size / 2) - (obj.len() / 2)) as i64 - x,
                    ((size / 2) - (obj.len() / 2)) as i64 - y
                ) {
                    None => STATUS::DEAD,
                    Some(status) => status
                };
                line.push(Cell::new(x as i32, y as i32, res))
            });

            rows.push(line);
        });

        Board{default_size: size, rows, history: VecDeque::with_capacity(1001)}
    }

    pub fn set_cell(&mut self, x: i32, y: i32, status: STATUS) -> Option<&Cell> {
        let c = self.rows.get_mut(y as usize)?.get_mut(x as usize)?;

        c.status = status;
        Some(c)
    }

    pub fn nb_row(&self) -> usize {
        self.rows.len()
    }

    pub fn get_line(&self, index: usize) -> String {
        self.rows.get(index).unwrap().iter()
            .map(|cell| cell.status.get_char()).collect()
    }

    pub fn get_row(&self, x: usize) -> &Vec<Cell> {
        self.rows.get(x).unwrap()
    }

    /// Get cell from a specific position on the board
    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.rows.get(y as usize)?.get(x as usize)
    }

    /// Get cell from a specific position on the board
    pub fn get_cell_or_dead(&self, x: i32, y: i32) -> Cell {
        match self.get_cell(x, y) {
            None => Cell::new(0, 0, STATUS::DEAD),
            Some(&cell) => cell
        }
    }

    /// Get all adjacent cells status
    /// There for we make a square around the original cell
    fn get_adj_cells(&self, pos: &Cell) -> Vec<Cell> {
        vec![self.get_cell_or_dead(pos.x - 1, pos.y - 1),
             self.get_cell_or_dead(pos.x    , pos.y - 1),
             self.get_cell_or_dead(pos.x + 1, pos.y - 1),

             self.get_cell_or_dead(pos.x - 1, pos.y),
             self.get_cell_or_dead(pos.x + 1, pos.y),

             self.get_cell_or_dead(pos.x - 1, pos.y  + 1),
             self.get_cell_or_dead(pos.x    , pos.y  + 1),
             self.get_cell_or_dead(pos.x + 1, pos.y  + 1)]
    }

    /// Apply the game of life rules on a certain position on the board
    fn apply_on_pos(&self, cell: &Cell) -> Cell {
        let adj_live_cells = self.get_adj_cells(cell).iter()
            .filter(|&&elem| elem.is_alive())
            .count();
        cell.apply_rules(adj_live_cells)
    }

    /// Apply the game of life rules on a row of the board
    fn apply_on_row(&self, row: &Vec<Cell>, register_cells: &mut Box<Vec<Cell>>) -> Vec<Cell> {
        row.iter().map(|cell| {
            let c = self.apply_on_pos(cell);
            register_cells.push(c);
            c
        }).collect()
    }

    /// Apply the game of life rules on the board
    pub fn apply_on_all(&mut self) {
        let mut register_cells  = Box::new(vec![]);
        {
            let mut copy_register = register_cells.clone();
            self.rows = self.rows.iter().map(|row| self.apply_on_row(row, &mut copy_register)).collect();
        }

        if self.history.len() >= 100 {
            self.history.pop_back();
        }
        self.history.push_front(register_cells);
    }
}
