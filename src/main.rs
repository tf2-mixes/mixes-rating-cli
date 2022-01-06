use mixes_db::sql_db::SQLDb;
use mixes_db::Database;
use mixes_rating::MixesElo;

pub mod commands;

fn main()
{
    println!("Connecting to mixes database");
    let db = SQLDb::start().expect("Unable to connect to SQL database");
    let mut ratings = MixesElo::new(db);

    println!("Updating database..");
    ratings
        .database_mut()
        .update(0.75)
        .expect("Unable to update database");
}
