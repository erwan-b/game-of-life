pub mod cell;

use cell::{Cell, STATUS};
use std::collections::{VecDeque, HashSet};

pub struct Board {
    default_size: usize,
    rows: Vec<Vec<Cell>>,
    actual: Box<Vec<Cell>>,
    history: VecDeque<Box<Vec<Cell>>>
}

/// Define the board logic
impl Board {
    /// Construct an empty board
    /// TODO make this more readable
    pub fn new(size: usize, obj: Vec<&str>) -> Self {
        let mut actual = Box::new(vec![]);
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
                let cell = Cell::new(x as i32, y as i32, res);
                if res.is_alive() { actual.push(cell); }
                line.push(cell)
            });

            rows.push(line);
        });

        Board{default_size: size, rows, actual, history: VecDeque::with_capacity(1001)}
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
            .filter(|&&elem| elem.is_alive()).count();
        cell.apply_rules(adj_live_cells)
    }

    fn get_actual_interest_zone(&self) -> HashSet<(usize, usize)>{
        self.actual.iter()
            .map(|cell| (cell.x as usize / 10, cell.y as usize / 10))
            .flat_map(|elem| {
                ((elem.1 - 1)..(elem.1 + 2)).flat_map(move |a| {
                    ((elem.0 - 1)..(elem.0 + 2)).map( move |b| (b, a))
                })
            })
            .collect::<HashSet<(usize, usize)>>().iter()
            .map(|&elem| (elem.0 * 10, elem.1 * 10))
            .flat_map(|(x, y)|
                (y..(y + 10)).flat_map(move |a| (x..(x + 10)).map(move |b| (b, a)))
            ).collect()
    }

    /// Apply the game of life rules on the board
    /// Split the board in 10 x 10 square and check active one
    pub fn next(&mut self) {
        let res = self.get_actual_interest_zone().iter()
            .map(|&(x, y)| self.get_cell_or_dead(x as i32, y as i32))
            .map(|cell| self.apply_on_pos(&cell))
            .filter(|cell| cell.is_alive())
            .collect::<Vec<Cell>>();


        self.actual.clone().iter().for_each(|cell| { self.set_cell(cell.x, cell.y, STATUS::DEAD); });
        res.iter().for_each(|cell| { self.set_cell(cell.x, cell.y, cell.status); });

        if self.history.len() >= 10 {
            self.history.pop_back();
        }
        self.history.push_front(self.actual.clone());
        self.actual = Box::new(res);
    }

    pub fn prev(&mut self) {
        if self.history.len() > 0 {
            self.actual.clone().iter().for_each(|&cell| { self.set_cell(cell.x,  cell.y, STATUS::DEAD); });

            self.actual = self.history.pop_front().unwrap();
            self.actual.clone().iter().for_each(|&cell| { self.set_cell(cell.x, cell.y, cell.status); })
        }
    }
}