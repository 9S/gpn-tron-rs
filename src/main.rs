mod util;
mod protocol;
mod algo;

use std::env;
use std::error::Error;

use std::time::SystemTime;
use uuid::Uuid;
use crate::protocol::connection::{Connection};
use crate::protocol::receive::get_mappers;
use crate::protocol::receive::init_game_data;


fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;

    let username = env::var("USER")
        .unwrap_or_else(|_| Uuid::new_v4().to_string());
    let password = env::var("PASSWORD")
        .unwrap_or_else(|_| Uuid::new_v4().to_string());


    let game_data = init_game_data();
    let mappers = get_mappers(username, password);

    let mut conn = Connection::new(game_data, &mappers)?;
    loop {
        conn.process_incoming_command()?;
    }
}