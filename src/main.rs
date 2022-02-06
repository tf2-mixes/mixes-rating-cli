#![feature(stdio_locked)]

use std::io::{stdin_locked, BufRead};

use mixes_db::sql_db::SQLDb;
use mixes_db::Database;
use mixes_rating::MixesElo;

use crate::commands::parse_command_and_execute;

pub mod commands;
pub mod queries;

fn main()
{
    println!("Connecting to mixes database");
    let db = SQLDb::start().expect("Unable to connect to SQL database");
    let mut ratings = MixesElo::new(db);

    println!(
        "Connected to database. Please enter a command. Type `help` to get a list of commands"
    );
    parse_command_and_execute("win_rate 76561198031286581 soldier 10", &mut ratings);
    let mut stdin = stdin_locked();
    loop {
        let mut command = String::new();
        stdin
            .read_line(&mut command)
            .expect("Unable to read line from stdin");

        if !parse_command_and_execute(&command, &mut ratings) {
            break;
        }
    }
}
