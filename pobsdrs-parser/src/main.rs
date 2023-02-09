use clap::{arg, Command};
use pobsdrs_parser::Parser;


fn cli() -> Command {
    Command::new("pobsdrs-parser")
        .about("A tool to manipulate the PlayOnBSD Database")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("check")
                .about("Check for error in the Database")
                .arg(arg!(<DATABASE> "The Database"))
                .arg_required_else_help(true),
        )
}

fn main() -> Result<(), std::io::Error> {
    let matches = cli().get_matches();
    
    match matches.subcommand() {
        Some(("check", sub_matches)) => {
            let file = sub_matches.get_one::<String>("DATABASE").expect("required");
            let parser = Parser::default();
            parser.load_from_file(&file)?;
        }
        _ => println!("Unsupported command")
    }
    Ok(())
}
