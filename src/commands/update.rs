use mixes_db::Database;

use super::Command;

pub struct Update;

impl Command for Update
{
    fn num_args(&self) -> std::ops::RangeInclusive<usize> { 3..=3 }

    fn execute(
        &self,
        ratings: &mut mixes_rating::MixesElo<mixes_db::sql_db::SQLDb>,
        args: &[&str],
    ) -> Result<(), <mixes_db::sql_db::SQLDb as mixes_db::Database>::Error>
    {
        let ratio = args[0];
        let min_players = args[1];
        let max_players = args[2];

        let ratio: f32 = match ratio.parse() {
            Ok(r) => {
                if r >= 0.0 && r <= 1.0 {
                    r
                }
                else {
                    println!("Ratio must be between 0.0 and 1.0");
                    return Ok(());
                }
            },
            Err(e) => {
                println!("Ratio must be a floating number. {}", e);
                return Ok(());
            },
        };

        let min_players: u8 = match min_players.parse() {
            Ok(min_p) => min_p,
            Err(e) => {
                println!("Minimum number of players must be a positive integer");
                return Ok(());
            },
        };

        let max_players: u8 = match max_players.parse() {
            Ok(max_p) => max_p,
            Err(e) => {
                println!("Maximum number of players must be a positive integer");
                return Ok(());
            },
        };

        ratings
            .database_mut()
            .update(ratio, min_players..=max_players)
    }

    fn help(&self) -> &'static str
    {
        "Download the latest logs of all mixes games from logs.tf and save it into the database. \
         Takes three arguments: The ratio of mixes players that must have been in the game between \
         0.0 and 1.0 as a floating point number. 0.0 would download all logs of all players that \
         are registered as mixes players. 1.0 would download only download games where every \
         player in the server was a mixes player. The second argument is the minimum amount of \
         players that had to be in the server for the log to be considered, the third argument is \
         the maximum number of players."
    }
}
