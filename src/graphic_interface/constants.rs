use std::time::Duration;

/// This Constants object contain all values about the drawing
/// <p> - cell_size contain the nb of pixel for the call with a 1 zoom_ratio. </p>
/// <p> - cell_size - border_size = real shown cell </p>
#[derive(Clone, Copy)]
pub struct Constants {
    pub refresh_rate: Duration,
    pub turns: usize
}

impl Constants {
    pub fn new(refresh_rate: Duration) -> Self {
        Constants {
            refresh_rate,
            turns: 0,
        }
    }
}
