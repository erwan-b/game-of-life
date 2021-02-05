pub mod board;
pub mod graphic_interface;

use std::fs;
use ggez::{ ContextBuilder, event, conf };

use board::{Board};
use graphic_interface::MyGame;

pub const MAP_SIZE: usize = 200;

pub fn create_file_from_map(value: &String, _file_path: &str) {
    fs::write("./map/saved_map.txt", value);
}

pub fn create_map_from_file(file_path: &str) -> Box<Board> {
    let lines = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    Box::new(Board::new(MAP_SIZE, lines.trim().lines().collect()))
}

/// Run the ggez window
/// Resizable got problems with osx
/// The resizable is commit because it don't work on OSX and linux. It can be cause by ggez
fn run_game(board: Box<Board>) {
    let mut c: conf::Conf = conf::Conf::new();

    c.window_setup = c.window_setup.title(&"game of life");
    c.window_mode = c.window_mode.resizable(true);

    let (mut ctx, event_loop) = ContextBuilder::new("game_of_life", "Erwan Bernard")
        .default_conf(c)
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx, board);

    // Run!
    event::run(ctx, event_loop, my_game)
}

pub fn load_run(mapfile: &str) {
    let board = create_map_from_file(mapfile);

    run_game(board);
}
