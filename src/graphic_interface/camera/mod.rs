use ggez::mint::Point2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Camera {
    pos: Point2<usize>,
    zoom_ratio: f32,
    default_size: usize
}

impl Camera {
    pub fn new() -> Self {
        Camera{pos: Point2{ x: 0, y: 0 }, zoom_ratio: 0.0, default_size: 0 }
    }

    pub fn set_zoom_ratio(&mut self, zoom_ratio: f32) -> Self {
        self.zoom_ratio = zoom_ratio;

        *self
    }

    pub fn set_pos(&mut self, pos: Point2<usize>) -> Self {
        self.pos = pos;

        *self
    }

    pub fn get_pos_to_show(&self) {

    }

    pub fn number_of_cells(&self) -> (usize, usize) {

    }
}