use ggez::mint::Point2;
use crate::board::cell::Cell;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug)]
pub struct Pixel {
    pub screen_pos: Point2<f32>,
    pub board_pixel_pos:  Point2<f32>,
    pub board_pos:  Point2<f32>,
}

impl Pixel {
    pub fn new(screen_pos: (f32, f32), board_pixel_pos: (f32, f32), board_pos: (f32, f32)) -> Self{
        Self {
            screen_pos: Point2{ x: screen_pos.0, y: screen_pos.1 },
            board_pixel_pos: Point2{ x: board_pixel_pos.0, y: board_pixel_pos.1 },
            board_pos:  Point2{ x: board_pos.0, y: board_pos.1 },
        }
    }
}

/// This struct is here to regroup the logic about what part
/// of the board is show on screen.
#[derive(Clone, PartialEq, Debug)]
pub struct Camera {
    position_on_board_pixel: Point2<f32>,
    position_on_board: Point2<f32>,
    position_on_board_end: Point2<f32>,
    cell_size: f32,
    screen_size: Point2<f32>,
    zoom_ratio: f32,
    cells_pos: Vec<Pixel>
}

impl Camera {
    pub fn new(position_on_board: Point2<f32>, screen_size: Point2<f32>) -> Self {
        let cell_size = 16.0;
        let position_on_board_pixel = Point2{x: position_on_board.x * cell_size, y: position_on_board.y * cell_size };
        let position_on_board_end = Point2{x: position_on_board.x + (screen_size.x / cell_size) as f32, y: position_on_board.y + (screen_size.y / cell_size) };

        let mut c = Self {
            position_on_board,
            position_on_board_end,
            position_on_board_pixel,
            screen_size,
            zoom_ratio: 1.0,
            cell_size,
            cells_pos: vec![]
        };

        c.update_line_to_show();
        c
    }

    /// Set the position of cells to show on screen
    /// It's used on draw living cells and draw the board lines
    pub fn active_cells_to_show(&self, cells: &Box<HashSet<Cell>>) -> Vec<Pixel> {
        cells.iter().map(|cell| (cell.x as f32, cell.y as f32)).map(|(x, y)| (
            (x * self.get_cell_size(), y * self.get_cell_size()),
            (x * self.get_cell_size() - self.position_on_board_pixel.x, y * self.get_cell_size() - self.position_on_board_pixel.y),
            (x, y),
        ))
            .map(|(board_pixel_pos, screen_pos, board_pos)|
                Pixel::new(screen_pos, board_pixel_pos, board_pos)
            )
            .collect()
    }

    pub fn line_to_show(&self) -> &Vec<Pixel> {
       &self.cells_pos
    }

    pub fn update_line_to_show(&mut self) {
        let x_iterator = self.position_on_board.x as i32..(self.position_on_board.x + self.screen_size.x / self.get_cell_size()) as i32;
        let y_iterator = self.position_on_board.y as i32..(self.position_on_board.y + self.screen_size.y / self.get_cell_size()) as i32;

        self.cells_pos = y_iterator.map(|y| (self.position_on_board.x, y as f32))
            .chain(x_iterator.map(|x| (x as f32, self.position_on_board.y)))
            .map(|board_pos| (
                (board_pos.0 * self.get_cell_size(), board_pos.1 * self.get_cell_size()),
                (board_pos.0 * self.get_cell_size() - self.position_on_board_pixel.x, board_pos.1 * self.get_cell_size() - self.position_on_board_pixel.y),
                board_pos
        ))
            .map(|(board_pixel_pos, screen_pos, board_pos)|
                Pixel::new(screen_pos, board_pixel_pos, board_pos)
            )
            .collect();
    }

    pub fn board_pos_from_screen_pos(&self, (x, y): (f32, f32)) -> (f32, f32) {
        ((x + self.position_on_board_pixel.x as f32) / self.get_cell_size(),
         (y + self.position_on_board_pixel.y as f32) / self.get_cell_size())
    }

    pub fn set_zoom_ratio(&mut self, zoom_ratio: f32) -> &Self {
        self.zoom_ratio = zoom_ratio;
        self.position_on_board_pixel = Point2{
            x: self.position_on_board.x * self.get_cell_size(),
            y: self.position_on_board.y * self.get_cell_size()
        };
        self.update_line_to_show();

        self
    }

    pub fn get_zoom_ratio(&mut self) -> f32 {
        self.zoom_ratio
    }

    pub fn get_cell_size(&self) -> f32 {
        self.cell_size * self.zoom_ratio
    }

    pub fn move_pos(&mut self, to_add: Point2<f32>) {
        self.position_on_board_pixel.x -= to_add.x;
        self.position_on_board_pixel.y -= to_add.y;

        self.position_on_board.x = self.position_on_board_pixel.x / self.get_cell_size();
        self.position_on_board.y = self.position_on_board_pixel.y / self.get_cell_size();
        self.update_line_to_show();
    }
}