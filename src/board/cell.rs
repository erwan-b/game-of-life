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
}
