pub mod board;
pub mod graphic_interface;

use std::fs;
use ggez::{ ContextBuilder, event };

use board::{Board};
use graphic_interface::MyGame;

pub fn create_file_from_map(file_path: &str) -> Box<Board> {
    let lines = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    Box::new(Board::new(10, lines.trim().lines().collect()))
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
