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

        self.cells_pos = y_iterator.flat_map(move |a|
                (x_iterator.0..x_iterator.1).map(move |b| (a as f32, b as f32))
            )
            .map(|pos|
                (pos,
                (pos.0 - self.position_on_board_pixel.x, pos.1  - self.position_on_board_pixel.y))
            )
            .filter(|((x, y), _screen_pos)|
                (x % (self.cell_size * self.zoom_ratio)) as i32 == 0 && (y % (self.cell_size * self.zoom_ratio)) as i32 == 0
            )
            .map(|(board_pixel_pos, screen_pos)|
                Pixel::new(screen_pos,
                           board_pixel_pos,
                    (board_pixel_pos.0 / (self.cell_size * self.zoom_ratio),
                     board_pixel_pos.1 / (self.cell_size * self.zoom_ratio))
                )
            )
            .collect();
    }

    pub fn board_pos_from_screen_pos(&self, (x, y): (f32, f32)) -> (f32, f32) {
        (x + self.position_on_board.x as f32, y + self.position_on_board.y as f32)
    }

    pub fn set_zoom_ratio(&mut self, zoom_ratio: f32) -> &Self {
        self.zoom_ratio = zoom_ratio;
        self.position_on_board_pixel = Point2{x: self.position_on_board.x * self.cell_size * self.zoom_ratio, y: self.position_on_board.y * self.cell_size * self.zoom_ratio};
        self.update_cells_to_show();

        self
    }

    pub fn get_zoom_ratio(&mut self) -> f32 {
        self.zoom_ratio
    }

    pub fn get_cell_size(&self) -> f32 {
        self.cell_size * self.zoom_ratio
    }

    /// Set the top left pixel position on the board
    pub fn set_pos(&mut self, pos: Point2<f32>) -> &Self {
        self.position_on_board = pos;
        self.update_cells_to_show();

        self
    }

    /// Return an iterator that go throw all board cells to show
    pub fn size_shown_iter(&self) -> impl Iterator<Item = &Pixel > {
        self.cells_pos.iter()
    }
}