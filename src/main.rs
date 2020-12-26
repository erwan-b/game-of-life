use clap::{Arg, App};
use game_of_life::load_run;

fn main() {
    let matches = App::new("Game of life")
        .version("0.1.0")
        .author("Erwan Bernard <erwan.bernard@gmail.com>")
        .about("Simple programme how run the game of life")
        .arg(Arg::with_name("file")
            .alias("file")
            .takes_value(true)
            .help("config file for the board")
        )
        .get_matches();

    let mapfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", mapfile);

    load_run(mapfile);
}
