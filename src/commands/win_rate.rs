use std::str::FromStr;

use mixes_db::{Class, SteamID};

use super::Command;
use crate::queries;

pub struct WinRate;

impl Command for WinRate
{
    fn num_args(&self) -> std::ops::RangeInclusive<usize> { 2..=3 }

    fn execute(
        &self,
        ratings: &mut mixes_rating::MixesElo<mixes_db::sql_db::SQLDb>,
        args: &[&str],
    ) -> Result<(), <mixes_db::sql_db::SQLDb as mixes_db::Database>::Error>
    {
        let steam_id = args[0];
        let class = args[1];
        let num_games = args[2];

        let steam_id = match SteamID::from_str(steam_id) {
            Ok(sid) => sid,
            Err(()) => {
                println!("Not a valid steam id");
                return Ok(());
            },
        };

        let class = match Class::from_str(class) {
            Ok(class) => class,
            Err(_) => {
                println!("Not a valid class name");
                return Ok(());
            },
        };

        let num_games: usize = match num_games.parse() {
            Ok(ng) => ng,
            Err(e) => {
                println!("Not a valid number of games");
                return Ok(());
            },
        };

        match queries::win_rate(ratings, steam_id, class, num_games) {
            Ok(Some((win_rate, num_games))) => println!(
                "Win rate is {} ({} games where considered)",
                win_rate, num_games
            ),
            Ok(None) => {
                println!("No win rate can be calculated since there are no games on record.")
            },
            Err(e) => println!("There was an error while trying to calculate the win-rate"),
        }

        Ok(())
    }

    fn help(&self) -> &'static str { todo!() }
}
