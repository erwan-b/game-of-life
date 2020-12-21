mod constants;

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler};
use ggez::event::MouseButton;

use crate::board::Board;
use crate::graphic::constants::Constants;
use crate::board::cell::Cell;

/// `MyGame` describe the game graphic logic
///
pub struct MyGame {
    board: Box<Board>,
    constants: Constants,
}

/// The impl is here to define our graphic logic called by the `EventHandler`
/// It important to kipp it split from the the rest
impl MyGame {
    pub fn new(_ctx: &mut Context, board: Box<Board>) -> Self {
        MyGame {
            board,
            constants: Constants::new(15 as f32, 4 as f32, 1 as f32)
        }
    }

    pub fn next(&mut self) {
            self.board = self.board.apply_on_all();
    }
}

/// Define the `EventHandler` to mange the ggez lib events
impl EventHandler for MyGame {
    /// Update the cells there.
    /// There for we call the board function that return a new one with the rules applied on all cells.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        Ok(())
    }

    /// Draw the board on the screen.
    /// The defined size of the cells we be translate to showed size with de zoom ratio
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let (h, w) = graphics::size(_ctx);
        let cell_h = h / self.constants.cell_size;
        let cell_w = w / self.constants.cell_size;
        (0..cell_h as usize).step_by(self.constants.cell_size as usize)
            .map(|elem| {
                let rect = graphics::Rect::new(0 as f32, elem as f32, w, self.constants.cell_size)
                let params = graphics::DrawParam::new();
                graphics::draw(ctx, &graphics::Rect, params);
            });

        // Draw code here...
        graphics::present(ctx)
    }

    /// We need to track the mouse event to set a cell alive if the mouse is click on a valid cell.
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        println!("button down");
    }
}
