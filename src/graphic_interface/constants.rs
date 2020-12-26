use std::time::Duration;

/// This Constants object contain all values about the drawing
/// <p> - cell_size contain the nb of pixel for the call with a 1 zoom_ratio. </p>
/// <p> - cell_size - border_size = real shown cell </p>
#[derive(Clone, Copy)]
pub struct Constants {
    pub cell_size: f32,
    pub border_size: f32,

    pub refresh_rate: Duration,
}

impl Constants {
    pub fn new(cell_size: f32, border_size: f32, refresh_rate: Duration) -> Self {
        Constants {
            cell_size,
            border_size,
            refresh_rate,
        }
    }
}
