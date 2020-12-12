mod game_of_life;
use clap::{Arg, App};
use game_of_life::files;


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

    files::read_file(mapfile);
}
