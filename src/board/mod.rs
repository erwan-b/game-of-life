pub mod cell;

use cell::{Cell, STATUS};

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

    pub fn get_line(&self, index: usize) -> String {
       self.rows.get(index).unwrap()
           .iter().map(|cell| if cell.status == STATUS::ALIVE {
           '1'
       } else {
           '0'
       }).collect()
    }

    pub fn add_line(&mut self, line: &str) -> &Self {
        let mut x: i32 = 0;
        let y: i32 = self.rows.len() as i32;

        let vec = line.chars()
            .map(|elem| {
                let status = Board::cell_status(elem);

                x += 1;
                Cell::new(x - 1, y, status)
            }).collect::<Vec<Cell>>();
        self.rows.push(vec);

        self
    }

    pub fn get_row(&self, x: usize) -> &Vec<Cell> {
        self.rows.get(x).unwrap()
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        match self.rows.get(y as usize) {
            None => Cell::new(y, 0, STATUS::DEAD),
            Some(vec) => match vec.get(x as usize)  {
                None => Cell::new(x, y, STATUS::DEAD),
                Some(&cell) => cell
            }
        }
    }

    pub fn get_cell_status(&self, x: i32, y: i32) -> STATUS {
        match self.rows.get(y as usize) {
            None => STATUS::DEAD,
            Some(vector) => match vector.get(x as usize) {
                None => STATUS::DEAD,
                Some(cell) => cell.status
            }
        }
    }

    fn get_adj_cells(&self, pos: &Cell) -> Vec<Cell> {
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
        vec![self.get_cell_status(pos.x - 1, pos.y - 1),
             self.get_cell_status(pos.x    , pos.y - 1),
             self.get_cell_status(pos.x + 1, pos.y - 1),

             self.get_cell_status(pos.x - 1, pos.y),
             self.get_cell_status(pos.x + 1, pos.y),

             self.get_cell_status(pos.x - 1, pos.y  + 1),
             self.get_cell_status(pos.x    , pos.y  + 1),
             self.get_cell_status(pos.x + 1, pos.y  + 1)]
    }

    fn next_status_from_pos(&self, pos: &Cell) -> STATUS {
        let adj_live_cells = self.get_adj_cells_status(pos).iter()
            .filter(|&&elem| elem == STATUS::ALIVE).count();

        println!("x: {}, y: {}", pos.x, pos.y);
        println!("{}", adj_live_cells);
        if pos.status == STATUS::ALIVE && adj_live_cells > 3 || adj_live_cells < 2 {
            STATUS::DEAD
        } else if pos.status == STATUS::ALIVE {
            STATUS::ALIVE
        } else if pos.status == STATUS::DEAD && adj_live_cells == 3 {
            STATUS::ALIVE
        } else {
            STATUS::DEAD
        }
    }

    fn apply_on_row(&self, row: &Vec<Cell>) -> Vec<Cell> {
        println!("-----");
        row.iter()
            .fold(vec![], |mut acc, cell| {
                acc.push(
                    Cell::new(cell.x, cell.y, self.next_status_from_pos(cell))
                );
                acc
            })
    }

    pub fn apply_on_all(&self) -> Box<Board> {
        let mut board = Board::new();

        self.rows.iter().for_each(|row| {
            board.add_row(self.apply_on_row(row));
        });


        Box::new(board)
    }
}
