use ggez::mint::Point2;

#[derive(Clone, PartialEq, Debug)]
pub struct Camera {
    position_on_board: Point2<usize>,
    screen_size: Point2<usize>,
    zoom_ratio: f32,
    cell_size: f32,
    cells_pos: Vec<(usize, usize)>
}

impl Camera {
    pub fn new(position_on_board: Point2<usize>, screen_size: Point2<usize>) -> Self {
        let cell_size = 14.0;
        let mut pixel_position_on_board =  position_on_board.clone();

        pixel_position_on_board.x *= cell_size as usize;
        pixel_position_on_board.y *= cell_size as usize;

        let mut c = Camera{ position_on_board: pixel_position_on_board, screen_size, zoom_ratio: 1.0, cell_size, cells_pos: vec![] };
        c.set_zoom_ratio(1.0);
        c
    }

    pub fn set_screen_size(&mut self, screen_size: Point2<usize>) -> &Self {
        self.screen_size = screen_size;

        self
    }

    pub fn set_zoom_ratio(&mut self, zoom_ratio: f32) -> &Self {
        self.zoom_ratio = zoom_ratio;

        let x_iterator = (self.position_on_board.x, self.position_on_board.x + self.screen_size.x);
        let y_iterator = self.position_on_board.y..(self.position_on_board.y + self.screen_size.y);

        self.cells_pos = y_iterator
            .flat_map(move |a| (x_iterator.0..x_iterator.1).map(move |b| (a, b)))
            .map(|elem| elem)
            .filter(|(x, y)|
                x % (self.cell_size * self.zoom_ratio) as usize == 0
                && y % (self.cell_size * self.zoom_ratio) as usize == 0)
            .collect();

        self
    }

    pub fn set_pos(&mut self, pos: Point2<usize>) -> &Self {
        self.position_on_board = pos;

        self
    }

    /// Return an iterator that go throw all screen pixels
    pub fn size_shown_iter(&self) -> impl Iterator<Item = &(usize, usize) > {
        self.cells_pos.iter()
    }
}