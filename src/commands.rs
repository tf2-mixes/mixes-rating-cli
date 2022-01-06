use std::ops::RangeInclusive;
use std::str::FromStr;

use mixes_db::sql_db::SQLDb;
use mixes_db::Database;
use mixes_rating::MixesElo;

#[derive(Copy, Clone)]
pub enum Command
{
    Help,
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

    let command = match Command::from_str(command) {
        Ok(cmd) => cmd,
        Err(()) => {
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

impl Command
{
    pub fn num_args(self) -> RangeInclusive<usize>
    {
        match self {
            Self::Help => 0..=usize::MAX,
        }
    }

    pub fn execute(
        self,
        ratings: &mut MixesElo<SQLDb>,
        args: &[&str],
    ) -> Result<(), <SQLDb as Database>::Error>
    {
        todo!()
    }
}

impl FromStr for Command
{
    // Always means 'unknown command'
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s {
            "help" => Ok(Self::Help),
            o => Err(()),
        }
    }
}
