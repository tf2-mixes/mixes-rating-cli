use std::str::FromStr;

use mixes_db::{Database, SteamID};

use super::Command;

#[derive(Copy, Clone)]
pub struct AddUser;

impl Command for AddUser
{
    fn num_args(&self) -> std::ops::RangeInclusive<usize> { 2..=2 }

    fn execute(
        &self,
        ratings: &mut mixes_rating::MixesElo<mixes_db::sql_db::SQLDb>,
        args: &[&str],
    ) -> Result<(), <mixes_db::sql_db::SQLDb as mixes_db::Database>::Error>
    {
        let steam_id = args[0];
        let discord_id = args[1];

        let steam_id: SteamID = match SteamID::from_str(steam_id) {
            Ok(sid) => sid,
            Err(()) => {
                println!("First argument is not a valid steam id.");
                return Ok(());
            },
        };

        let discord_id: u64 = match discord_id.parse() {
            Ok(did) => did,
            Err(e) => {
                println!("Second argument is not a valid discord id.");
                return Ok(());
            },
        };

        match ratings.database_mut().add_user(steam_id, discord_id) {
            Ok(true) => {
                println!("User added to the database");
                Ok(())
            },
            Ok(false) => {
                println!("User already exists in the database. Nothing changed.");
                Ok(())
            },
            Err(e) => {
                println!("Could not add user to the database. {}", e);
                Err(e)
            },
        }
    }

    fn help(&self) -> &'static str
    {
        "Add a user to the database. Takes two arguments: The steam_id, of which currently the \
         formats steamid3 and steamid64 are supported. The second argument is the discord id of \
         the account that should be associated with this steam id."
    }
}
