use crate::model::{Database, Share};
use color_eyre::Report;
use rusqlite::{params, Connection};
use tracing::info;

fn create_share_table(db: &Database) -> Result<(), Report> {
    info!("Creating sharejournal table if needed");
    let conn = db.connection();
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS sharejournal(
            id INTEGER PRIMARY KEY,
            share_code TEXT NOT NULL,
            share_name TEXT,
            buy_price INTEGER,
            buy_date DATE,
            sell_price INTEGER,
            sell_date DATE
        )",
        (),
    )?;

    Ok(())
}

pub fn buy_share(share: &Share, db: &Database) -> Result<(), Report> {
    info!("Buying share: {:?}", share);
    let conn = db.connection();
    println!("{:?}", share);
    conn.execute(
        "INSERT INTO sharejournal (share_code, share_name, buy_price, buy_date)
    VALUES (?1, ?2, ?3, ?4",
        params![&share.code, &share.name, &share.buy_price, &share.buy_date],
    )?;
    Ok(())
}

fn load_share(code: &str, db: &Database) -> Result<Option<Share>, Report> {
    let conn = db.connection();
    let sql = "SELECT * from sharejournal where share_code = ?1";
    Ok(None)
}

impl Database {
    pub fn connection(&self) -> Connection {
        Connection::open(self.name.clone()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    fn test_db() -> Database {
        Database {
            name: "TestDb.db".to_string(),
        }
    }
    #[test]
    pub fn test_create_share_table() {
        let db = test_db();
        let res = create_share_table(&db);
        assert!(res.is_ok())
    }

    #[test]
    pub fn test_buy_share() {
        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1932, 9, 20).unwrap(),
            NaiveTime::from_hms_opt(9, 8, 7).unwrap(),
        );

        let share = Share::share_to_buy(
            "Test Share Company".to_string(),
            "TST".to_string(),
            12.34,
            date,
        );
        let db = test_db();
        buy_share(&share, &db).unwrap();
    }
}
