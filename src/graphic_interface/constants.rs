/// This Constants object contain all values about the drawing
/// - cell_size contain the nb of pixel for the call with a 1 zoom_ratio.
///     cell_size - border_size = real shown cell
#[derive(Clone, Copy)]
pub struct Constants {
    pub cell_size: f32,
    pub border_size: f32,
    pub zoom_ratio: f32,
}

impl Constants {
    pub fn new(cell_size: f32, border_size: f32, zoom_ratio: f32) -> Self {
        Constants {
            cell_size,
            border_size,
            zoom_ratio
        }
    }
}
