use clap::{App, Arg, SubCommand};
use nf_rated::{data::get_database_info, data::Db, tui};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let db_info = get_database_info()?;
    let db = Db::new(&db_info)?;
    if !db_info.db_exists {
        eprintln!(
            "The database did not exist yet and was created at: {:?}
Make sure to sync it first by running 'nf-rated sync'.",
            db_info.db_path
        );
    }

    let matches = App::new("nf-rated")
        .subcommand(
            SubCommand::with_name("sync")
                .about("syncs ratings from omdb")
                .arg(
                    Arg::with_name("limit")
                        .short("l")
                        .long("limit")
                        .value_name("limit")
                        .help("nf-rated -l <rate limit>"),
                ),
        )
        .get_matches();

    match matches.subcommand_matches("sync") {
        Some(matches) => {
            let limit = if matches.is_present("limit") {
                matches
                    .value_of("limit")
                    .unwrap()
                    .parse::<u16>()
                    .expect("Limit needs to be a number, i.e. 1000")
            } else {
                1000
            };
        }
        None => tui(db)?,
    }

    Ok(())
}
