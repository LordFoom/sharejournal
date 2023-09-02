use crate::db::buy_share;
use crate::model::{Database, Share};
use chrono::{NaiveDateTime, Utc};
use clap::Parser;
use color_eyre::Report;
use Default;

mod db;
mod model;

///Arguments to the our app
#[derive(Parser, Debug)]
struct Args {
    code: String,
    price: f64,
    #[arg(short, long)]
    name: Option<String>,
    ///buy or sell
    #[arg(short, long)]
    sell: bool,
    #[arg(short,long, value_parser=parse_date)]
    date: Option<NaiveDateTime>,
}

fn main() -> Result<(), Report> {
    let args = Args::parse();
    //default is to buy
    let db = Database {
        name: String::from("sharejournal.db"),
    };
    if !args.sell {
        //default is buy
        let result = purchase(&args, &db)?;
    } else {
        //todo sell
    }
    Ok(())
}

fn purchase(args: &Args, db: &Database) -> Result<String, Report> {
    let name = if let Some(name) = args.name.clone() {
        name
    } else {
        "".to_string()
    };
    let buy_date = if let Some(dt) = args.date {
        dt
    }else{
        let dt = Utc::now().naive_utc();
        dt
    };
    let share = Share {
        name,
        code: args.code.clone(),
        buy_price: args.price.clone(),
        buy_date,
        ..Default::default()
    };

    buy_share(&share, &db)?;
    let result = format!("Bought '{}' at '{}'", share.code, share.buy_price);
    return Ok(result);
}

fn parse_date(src: &str) -> Result<NaiveDateTime, Report> {
    Ok(NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")?)
}
mod test {
    use crate::db::create_share_table;
    use super::*;

    #[test]
    fn test_purchase() {
        let args = Args {
            name: None,
            price: 8.96,
            code: "TEST".to_string(),
            sell: false,
            date: None,
        };
        let db = Database{
            name: "TestDb.db".to_string()
        };
        create_share_table(&db).unwrap();
        let result = purchase(&args, &db).unwrap();
        assert_eq!("Bought 'TEST' at '8.96'", result);
    }
}
