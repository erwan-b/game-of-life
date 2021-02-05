pub mod cell;

use cell::{Cell, STATUS};
use std::collections::{VecDeque, HashSet};
use std::ops::Add;

pub struct Board {
    rows: Vec<Vec<Cell>>,
    actual: Box<HashSet<Cell>>,
    history: VecDeque<Box<HashSet<Cell>>>,
    initial_state: Box<HashSet<Cell>>,
}

/// Define the board logic
impl Board {
    fn get_status_or_dead(x: i64, y: i64, obj_b: &Vec<Vec<char>>) -> Option<STATUS> {
        if x <= 0 && y <= 0 {
            None
        } else {
            let &c = obj_b.get(y as usize)?.get(x as usize)?;
            Some(STATUS::get_from_char(c))
        }
    }

    fn get_cell_from_char(size: usize, obj_b: Vec<Vec<char>>, (x, y): (i64, i64)) -> Cell {
        let res = match Board::get_status_or_dead(
            ((size / 2) - obj_b.len() / 2) as i64 - x,
            ((size / 2) - obj_b.len() / 2) as i64 - y,
            &obj_b
        ) {
            None => STATUS::DEAD,
            Some(status) => status
        };
        Cell::new(x as i32, y as i32, res)
    }

    /// Construct the board from a map
    pub fn new(size: usize, obj: Vec<&str>) -> Self {
        let mut actual = Box::new(HashSet::new());
        let obj_b: Vec<Vec<char>> = obj.iter().map(|&s| s.chars().collect()).collect();

        let rows = (0..size as i64).map(|y| {
            (0..size as i64).map(|x| {
                let c = Self::get_cell_from_char(size, obj_b.clone(), (x, y));
                if c.is_alive() {
                    actual.insert(c);
                }
                c
            }).collect()
        }).collect();

        Board{rows, actual: actual.clone(), initial_state: actual.clone(), history: VecDeque::with_capacity(10000)}
    }

    fn active_cell_to_string(mut cells: Box<HashSet<Cell>>) -> String {
        let res = cells.iter().filter(|&cell| cell.is_alive()).copied().collect::<HashSet<Cell>>();
        cells = Box::from(res);

        let s_x = cells.iter().min_by(|&a, &b| a.x.cmp(&b.x)).unwrap();
        let s_y = cells.iter().min_by(|&a, &b| a.y.cmp(&b.y)).unwrap();
        let b_x = cells.iter().max_by(|&a, &b| a.x.cmp(&b.x)).unwrap();
        let b_y = cells.iter().max_by(|&a, &b| a.y.cmp(&b.y)).unwrap();

        (s_y.y..(b_y.y + 1)).map(|y| {
            (s_x.x..(b_x.x + 1)).map(|x| {
                if cells.contains(&Cell::new(x, y, STATUS::ALIVE)) {
                    STATUS::ALIVE.get_char()
                } else {
                    STATUS::DEAD.get_char()
                }
            }).collect::<String>().add("\n")
        }).collect()
    }

    pub fn board_to_string(&self) -> String {
        Self::active_cell_to_string(self.actual.clone())
    }


    pub fn initial_board_to_string(&self) -> String {
        Self::active_cell_to_string(self.initial_state.clone())
    }

    pub fn set_cell(&mut self, x: i32, y: i32, status: STATUS) -> Option<&Cell> {
        let c = self.rows.get_mut(y as usize)?.get_mut(x as usize)?;

        c.status = status;

        if c.is_alive() || self.actual.contains(c) {
            self.actual.insert(*c);
        }
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

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.rows.get(y as usize)?.get(x as usize)
    }

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

    /// Calculate the active zone on the board.
    /// There are 10x10 big and we get them by iterating on the actually activate cell
    /// At the end we are adding the boarding zone and remove duplicates
    fn get_actual_interest_cell(&self) -> HashSet<Cell>{
        self.actual.iter()
            .flat_map(|cell| self.get_adj_cells(cell))
            .collect()
    }

    /// Apply the game of life rules on the board
    /// Get 10x10 interesting zones and pass the rules on it
    pub fn next(&mut self) {
        let res = self.get_actual_interest_cell().iter()
            .map(|&cell| self.apply_on_pos(&cell))
            .filter(|cell| cell.is_alive())
            .collect::<Vec<Cell>>();


        self.actual.clone().iter().for_each(|cell| { self.set_cell(cell.x, cell.y, STATUS::DEAD); });
        self.history.push_front(self.actual.clone());
        self.actual = Box::new(HashSet::new());

        res.iter().for_each(|cell| { self.set_cell(cell.x, cell.y, cell.status); });
        if self.history.len() >= 10 {
            self.history.pop_back();
        }
    }

    pub fn prev(&mut self) {
        if self.history.len() > 0 {
            self.actual.clone().iter().for_each(|&cell| { self.set_cell(cell.x,  cell.y, STATUS::DEAD); });

            self.actual = self.history.pop_front().unwrap();
            self.actual.clone().iter().for_each(|&cell| { self.set_cell(cell.x, cell.y, cell.status); })
        }
    }
}