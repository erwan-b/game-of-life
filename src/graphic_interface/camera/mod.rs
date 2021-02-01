use ggez::mint::Point2;

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
    cell_size: f32,
    screen_size: Point2<f32>,
    zoom_ratio: f32,
    cells_pos: Vec<Pixel>
}

impl Camera {
    pub fn new(position_on_board: Point2<f32>, screen_size: Point2<f32>) -> Self {
        let cell_size = 16.0;
        let pixel_pos = Point2{x: position_on_board.x * cell_size, y: position_on_board.y * cell_size };

        let mut c = Self {
            position_on_board,
            position_on_board_pixel: pixel_pos,
            screen_size,
            zoom_ratio: 1.0,
            cell_size,
            cells_pos: vec![]
        };

        c.update_cells_to_show();
        c
    }

    /// Set the position of cells to show on screen
    /// It's used on draw living cells and draw the board lines
    fn update_cells_to_show(&mut self) {
        let x_iterator = (self.position_on_board_pixel.x as i32, (self.position_on_board_pixel.x + self.screen_size.x) as i32);
        let y_iterator = self.position_on_board_pixel.y as i32..(self.position_on_board_pixel.y + self.screen_size.y) as i32;
        let size = self.get_cell_size() as usize;

        self.cells_pos = y_iterator.step_by(size).flat_map(move |y|
                (x_iterator.0..x_iterator.1).step_by(size).map(move |x| ( x as f32,  y as f32))
            )
            .map(|pos| (
                pos,
                (pos.0 - self.position_on_board_pixel.x, pos.1  - self.position_on_board_pixel.y)
            ))
            .map(|(board_pixel_pos, screen_pos)|
                Pixel::new(screen_pos, board_pixel_pos,
                    (board_pixel_pos.0 / self.get_cell_size(),
                     board_pixel_pos.1 / self.get_cell_size())
                )
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
        self.update_cells_to_show();

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
        self.update_cells_to_show();
    }

    /// Return an iterator that go throw all board cells to show
    pub fn size_shown_iter(&self) -> impl Iterator<Item = &Pixel > {
        self.cells_pos.iter()
    }
}