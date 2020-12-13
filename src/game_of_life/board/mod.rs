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

    fn get_adj_cells(&self, pos: &Cell) -> Vec<&Cell> {
        vec![self.get(pos.x - 1, pos.y - 1),
        self.get(pos.x - 1, pos.y),
        self.get(pos.x - 1, pos.y + 1),

        self.get(pos.x, pos.y - 1),
        self.get(pos.x, pos.y + 1),

        self.get(pos.x + 1, pos.y - 1),
        self.get(pos.x + 1, pos.y),
        self.get(pos.x + 1, pos.y + 1)]
    }

    fn get_adj_cells_status(&self, pos: &Cell) -> Vec<STATUS> {
        self.get_adj_cells(pos).iter()
            .map(|&cell| cell.status)
            .collect()
    }

    fn get_status_from_pos(&self, pos: &Cell) -> STATUS {
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
}

pub fn apply_rules_on_board(board: &Board) -> Board{
    let mut new_board: Board = Board::new();

    new_board
}
