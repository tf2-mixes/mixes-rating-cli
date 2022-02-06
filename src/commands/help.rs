use super::Command;

pub struct Help;

impl Command for Help
{
    fn num_args(&self) -> std::ops::RangeInclusive<usize> { todo!() }

    fn execute(
        &self,
        ratings: &mut mixes_rating::MixesElo<mixes_db::sql_db::SQLDb>,
        args: &[&str],
    ) -> Result<(), <mixes_db::sql_db::SQLDb as mixes_db::Database>::Error>
    {
        todo!()
    }

    fn help(&self) -> &'static str { todo!() }
}
