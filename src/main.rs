mod game_of_life;
use clap::{Arg, App};
use game_of_life::files;
use game_of_life::board::{Board, apply_rules_on_board};
use game_of_life::graphic_lib::init_game;

fn infini_loop(board: &Board) {
    let mut actual_board: &Board = board;
    let mut new_board: Board;

    loop {
       new_board = apply_rules_on_board(&actual_board);
        actual_board = &new_board;
    }
}


fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file")
            .alias("file")
            .takes_value(true)
            .help("config file for the board ")
        )
        .get_matches();

    let mapfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", mapfile);

    let board = files::read_file(mapfile);
    init_game(&board);
    infini_loop(&board);
}
