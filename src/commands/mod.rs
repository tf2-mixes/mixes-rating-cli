pub mod add_user;
pub mod help;
pub mod rank_by_win_rate;
pub mod update;
pub mod win_rate;

use std::ops::RangeInclusive;
use std::str::FromStr;

pub use add_user::AddUser;
pub use help::Help;
use mixes_db::sql_db::SQLDb;
use mixes_db::{Database, SteamID};
use mixes_rating::MixesElo;
pub use update::Update;
pub use win_rate::WinRate;

use self::rank_by_win_rate::RankByWinRate;

pub trait Command
{
    fn num_args(&self) -> RangeInclusive<usize>;

    fn execute(
        &self,
        ratings: &mut MixesElo<SQLDb>,
        args: &[&str],
    ) -> Result<(), <SQLDb as Database>::Error>;

    fn help(&self) -> &'static str;
}

fn command_from_str(s: &str) -> Option<Box<dyn Command>>
{
    match s {
        "help" => Some(Box::new(Help)),
        "add_user" => Some(Box::new(AddUser)),
        "update" => Some(Box::new(Update)),
        "win_rate" => Some(Box::new(WinRate)),
        "rank_by_win_rate" => Some(Box::new(RankByWinRate)),
        _ => None,
    }
}

/// Take the command string, seperate it into its base command and arguments and
/// execute an action according to that command on the user ratings database
/// accordingly. Returns `true` when the command system should continue running
/// and `false` if the quit command was received.
pub fn parse_command_and_execute(command: &str, ratings: &mut MixesElo<SQLDb>) -> bool
{
    let command = command.trim();
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        println!("Please enter a command. Type `quit` to exit");
        return true;
    }

    let command = parts[0];
    let args = &parts[1..];

    // Internal exit command
    if command == "quit" || command == "exit" {
        return false;
    }

    let command = match command_from_str(command) {
        Some(cmd) => cmd,
        None => {
            println!("Unknown command: {}", command);
            return true;
        },
    };

    if !command.num_args().contains(&args.len()) {
        println!(
            "Wrong number of arguments. This command accepts {:?} arguments",
            command.num_args()
        );
        return true;
    }

    if let Err(e) = command.execute(ratings, args) {
        println!("Error processing command: {:?}", e);
    }

    true
}
