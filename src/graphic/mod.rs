use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

use crate::board::Board;

pub struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context, board: &Board) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}
