use ggez::mint::Point2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Camera {
    position_on_board: Point2<usize>,
    screen_size: Point2<usize>,
    zoom_ratio: f32,
}

impl Camera {
    pub fn new(position_on_board: Point2<usize>, screen_size: Point2<usize>) -> Self {
        Camera{ position_on_board, screen_size, zoom_ratio: 1.0 }
    }

    pub fn set_screen_size(&mut self, screen_size: Point2<usize>) -> Self {
        self.screen_size = screen_size;

        *self
    }

    pub fn set_zoom_ratio(&mut self, zoom_ratio: f32) -> Self {
        self.zoom_ratio = zoom_ratio;

        *self
    }

    pub fn set_pos(&mut self, pos: Point2<usize>) -> Self {
        self.position_on_board = pos;

        *self
    }

    pub fn size_shown_iter(&self) -> impl Iterator<Item = (usize, usize) > {
        let x_iterator = (self.position_on_board.x, self.position_on_board.x + self.screen_size.x);
        let y_iterator = (self.position_on_board.y..self.position_on_board.y + self.screen_size.y);

        y_iterator.flat_map(move |a| (x_iterator.0..x_iterator.1).map(move |b| (a, b)))
    }
}