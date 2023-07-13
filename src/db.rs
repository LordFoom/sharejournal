use color_eyre::Report;
use rusqlite::Connection;

fn create_share_table()-> Result<(), Report>{
    let conn =  connection();
    conn.execute("
        CREATE TABLE IF NOT EXISTS sharejournal(
            id INTEGER PRIMARY KEY,
            share_code TEXT NOT NULL,
            share_name TEXT,
            purchase_price INTEGER,
            sales_price INTEGER
        )", ())?;

    Ok(())
}

fn connection()-> Connection {
    Connection::open("sharejournal.db".to_string()).unwrap()
}

mod test{
    use  super::*;
}