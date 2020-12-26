pub mod board;
pub mod graphic_interface;

use std::fs;
use ggez::{ ContextBuilder, event, conf };

use board::{Board};
use graphic_interface::MyGame;

pub fn create_file_from_map(file_path: &str) -> Box<Board> {
    let lines = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    Box::new(Board::new(500, lines.trim().lines().collect()))
}

/// Run the ggez window
/// Resizable got problems with osx
fn run_game(board: Box<Board>) {
    let mut c: conf::Conf = conf::Conf::new();

    c.window_setup = c.window_setup.title(&"game of life");
    // c.window_mode = c.window_mode.resizable(true);

    let (mut ctx, mut event_loop) = ContextBuilder::new("game_of_life", "Erwan Bernard")
        .conf(c)
        .build()
        .expect("aieee, could not create ggez context!");

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
