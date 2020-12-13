pub mod row;

use row::{Row, Cell, STATUS};

pub struct Board {
    rows: Vec<Row>,
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

    pub fn add_row(&mut self, line: &str, x: usize) -> &Self {
        let mut y: usize = 0;

        let vec = line.chars()
            .map(|elem| {
                let status = if elem == '0' {
                   STATUS::DEAD
                } else if elem == '1' {
                     STATUS::ALIVE
                } else {
                    panic!("WRONG VALUE IN MAP")
                };

                y += 1;
                Cell::new(x, y, status)
            }).collect::<Vec<Cell>>();
        self.rows.push(Row::new(vec));

        self
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        self.rows.get(x).unwrap()
            .get(y)
    }

    pub fn apply_rules_on_pos(&mut self, pos: &Cell) {
    }
}
