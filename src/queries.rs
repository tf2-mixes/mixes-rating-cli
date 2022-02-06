use mixes_db::sql_db::SQLDb;
use mixes_db::{Class, Database, Performance, SteamID};
use mixes_rating::MixesElo;

pub fn win_rate(
    ratings: &mut MixesElo<SQLDb>,
    steam_id: SteamID,
    class: Class,
    num_games: usize,
) -> Result<Option<(f32, usize)>, <SQLDb as mixes_db::Database>::Error>
{
    let performances = ratings
        .database_mut()
        .get_class_performance(steam_id, class, num_games)?;

    // Only use the overall performances here, which are one per game and contain
    // the round wins and number of rounds for this game.
    let overall_performances = performances.into_iter().map(|(_id, ps)| {
        ps.into_iter()
            .find_map(|p| {
                if let Performance::Overall(op) = p {
                    Some(op)
                }
                else {
                    None
                }
            })
            .unwrap()
    });
    let num_games = overall_performances.len();

    // Sum all the won rounds and total rounds to two total sums
    let mut won_rounds = 0;
    let mut total_rounds = 0;
    for op in overall_performances {
        won_rounds += op.won_rounds as u64;
        total_rounds += op.num_rounds as u64;
    }

    // Calculate win-rate in case there are games on record.
    if total_rounds != 0 {
        let win_rate = won_rounds as f32 / total_rounds as f32;
        Ok(Some((win_rate, num_games)))
    }
    else {
        Ok(None)
    }
}
