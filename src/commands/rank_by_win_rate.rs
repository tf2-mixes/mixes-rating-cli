use std::str::FromStr;

use mixes_db::{Class, Database, SteamID};

use super::Command;
use crate::queries;

pub struct RankByWinRate;

impl Command for RankByWinRate
{
    fn num_args(&self) -> std::ops::RangeInclusive<usize> { 2..=2 }

    fn execute(
        &self,
        ratings: &mut mixes_rating::MixesElo<mixes_db::sql_db::SQLDb>,
        args: &[&str],
    ) -> Result<(), <mixes_db::sql_db::SQLDb as mixes_db::Database>::Error>
    {
        let class = args[0];
        let num_games = args[1];

        let class = match Class::from_str(class) {
            Ok(class) => class,
            Err(_) => {
                println!("Not a valid class name");
                return Ok(());
            },
        };

        let num_games: usize = match num_games.parse() {
            Ok(ng) => ng,
            Err(_e) => {
                println!("Not a valid number of games");
                return Ok(());
            },
        };

        let users = ratings.database_mut().users()?;

        // Collect the win-rate for every user in the database.
        let mut win_rates: Vec<(String, f32, usize)> = users
            .into_iter()
            .filter_map(|user| {
                let username = match ratings.database_mut().username(user) {
                    Ok(un) => un,
                    Err(e) => {
                        println!("Error querying the database for players username");
                        return None;
                    },
                };

                let display_name = match username {
                    Some(username) => username,
                    None => user.to_id64_string(),
                };

                match queries::win_rate(ratings, user, class, num_games) {
                    Ok(Some((win_rate, num_games))) => Some((display_name, win_rate, num_games)),
                    Ok(None) => {
                        println!("No games for {}. Skipping.", display_name);
                        None
                    },
                    Err(e) => {
                        println!(
                            "Error while reading games of {}. Error: {}.\nSkipping.",
                            display_name, e
                        );
                        None
                    },
                }
            })
            .collect();

        // Sort by win-rate. This results in ascending order.
        win_rates
            .sort_unstable_by(|(_ua, wra, _nga), (_ub, wrb, _ngb)| wra.partial_cmp(wrb).unwrap());

        // Announce with best (highest) win-rate at the top
        println!("Rankings for {:?}:", class);
        for (rank, (user, win_rate, num_games)) in win_rates.into_iter().rev().enumerate() {
            println!(
                "{}. {} [{:.3}] ({} games where considered)",
                rank + 1,
                user,
                win_rate,
                num_games
            );
        }

        Ok(())
    }

    fn help(&self) -> &'static str { todo!() }
}
