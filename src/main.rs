use clap::{App, SubCommand};

fn main() {
    let matches = App::new("test").about("test").subcommand(SubCommand::with_name("test")).get_matches();

    match matches.subcommand() {
        ("test", Some(args)) => {
            println!("sub command")
        },
        _ => eprintln!("no subcommand"),
    }
}
