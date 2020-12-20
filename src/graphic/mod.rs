use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler};

use crate::board::Board;

pub struct MyGame<'a> {
    board: &'a Board,
}

impl<'a> MyGame<'a> {
    pub fn new(_ctx: &mut Context, board: &'a Board) -> Self {
        MyGame {
            board
        }
    }
}

impl EventHandler for MyGame<'_> {
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
