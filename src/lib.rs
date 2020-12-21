pub mod board;
pub mod graphic;

use std::fs;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

use board::{Board};
use graphic::MyGame;

pub fn create_file_from_map(file_path: &str) -> Box<Board> {
    let c: Box<Board> = Box::new(Board::new());

    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .trim().lines()
        .fold(c, |mut board, line| {
            board.add_line(line);
            board
        })
}

fn run_game(board: Box<Board>) {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx, board);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

pub fn load_run(mapfile: &str) {
    let board = create_file_from_map(mapfile);

    run_game(board);
}
