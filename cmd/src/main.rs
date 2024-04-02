use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("cmd").
        version("v0.1.0")
        .about("Test cmd program")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("input file path")
            .takes_value(true).required(true)
        ).subcommand(SubCommand::with_name("help").about("subcommand help"))
        .get_matches();
    let path = matches.value_of("FILE").unwrap();
    println!("{}", path);
}
