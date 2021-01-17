use ggez::mint::Point2;

#[derive(Clone, PartialEq, Debug)]
pub struct Pixel {
    pub screen_pos: Point2<usize>,
    pub board_pixel_pos:  Point2<usize>,
    pub board_pos:  Point2<usize>,
}

impl Pixel {
    pub fn new(screen_pos: (usize, usize), board_pixel_pos: (usize, usize), board_pos: (usize, usize)) -> Self{
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
    pub position_on_board: Point2<usize>,
    pub cell_size: f32,
    screen_size: Point2<usize>,
    zoom_ratio: f32,
    cells_pos: Vec<Pixel>
}

impl Camera {
    pub fn new(position_on_board: Point2<usize>, screen_size: Point2<usize>) -> Self {
        let cell_size = 16.0;
        let pixel_pos = Point2{x: (position_on_board.x as f32 * cell_size) as usize, y: (position_on_board.y as f32 * cell_size) as usize };

        let mut c = Self {
            position_on_board: pixel_pos,
            screen_size, zoom_ratio: 1.0,
            cell_size,
            cells_pos: vec![]
        };

        c.update_cells_to_show();
        c
    }

    /// Set the position of cells to show on screen
    /// It's used on draw living cells and draw the board lines
    fn update_cells_to_show(&mut self) {
        let x_iterator = (self.position_on_board.x, self.position_on_board.x + self.screen_size.x);
        let y_iterator = self.position_on_board.y..(self.position_on_board.y + self.screen_size.y);

        self.cells_pos = y_iterator
            .flat_map(move |a| {  (x_iterator.0..x_iterator.1).map(move |b| (a, b))  })
            .map(|pos| (pos, (pos.0 - self.position_on_board.x, pos.1  - self.position_on_board.y)))
            .filter(|((x, y), _screen_pos)| x % (self.cell_size * self.zoom_ratio) as usize == 0 && y % (self.cell_size * self.zoom_ratio) as usize == 0)
            .map(|(pos, screen_pos)|
                Pixel::new(screen_pos, pos, ((pos.0 as f32 / self.cell_size) as usize, (pos.1 as f32 / self.cell_size) as usize)))
            .collect();
    }

    /// Update the zoom ratio
    pub fn set_screen_size(&mut self, screen_size: Point2<usize>) -> &Self {
        self.screen_size = screen_size;
        self.update_cells_to_show();

        self
    }

    /// Update the zoom ratio
    pub fn set_zoom_ratio(&mut self, zoom_ratio: f32) -> &Self {
        self.zoom_ratio = zoom_ratio;
        self.update_cells_to_show();

        self
    }

    /// Set the top left pixel position on the board
    pub fn set_pos(&mut self, pos: Point2<usize>) -> &Self {
        self.position_on_board = pos;
        self.update_cells_to_show();

        self
    }

    /// Return an iterator that go throw all board cells to show
    pub fn size_shown_iter(&self) -> impl Iterator<Item = &Pixel > {
        self.cells_pos.iter()
    }
}