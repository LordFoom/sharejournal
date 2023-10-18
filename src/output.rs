use color_eyre::Report;
use crate::db;
use crate::model::Database;

pub fn print_share(share_code: &str, db:&Database) -> Result<(), Report> {
    let share_records = db::load_share(share_code, db)?;
    share_records.iter()
        .for_each(|share| println!("{}", share));
    Ok(())

}