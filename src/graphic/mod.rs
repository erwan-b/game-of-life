use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler};
use ggez::event::MouseButton;

use crate::board::Board;

pub struct MyGame<'a> {
    board: &'a Board,
}

/// The impl is here to define our graphic logic called by the `EventHandler`
/// It important to kipp it split from the the rest
impl<'a> MyGame<'a> {
    pub fn new(_ctx: &mut Context, board: &'a Board) -> Self {
        MyGame {
            board
        }
    }
}

/// Define the `EventHandler` to mange the ggez lib events
impl EventHandler for MyGame<'_> {
    /// Update the cells there.
    /// There for we call the board function that return a new one with the rules applied on all cells.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        println!("update");

        Ok(())
    }

    /// Draw the board on the screen.
    /// The defined size of the cells we be translate to showed size with de zoom ratio
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

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
