use crate::db::buy_share;
use crate::model::{Database, Share};
use chrono::NaiveDateTime;
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
    if !args.sell {
    } else {
    }
    if !args.sell {
        //default is buy
        let result = purchase(&args)?;
    } else {
        //todo sell
    }
    Ok(())
}

fn purchase(args: &Args) -> Result<String, Report> {
    let name = if let Some(name) = args.name.clone() {
        name
    } else {
        "".to_string()
    };
    let share = Share {
        name,
        code: args.code.clone(),
        buy_price: args.price.clone(),
        ..Default::default()
    };
    let db = Database {
        name: String::from("sharejournal.db"),
    };

    buy_share(&share, &db)?;
    let result = format!("Bought '{}' at '{}'", share.code, share.buy_price);
    return Ok(result);
}

fn parse_date(src: &str) -> Result<NaiveDateTime, Report> {
    Ok(NaiveDateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S")?)
}
mod test {
    use super::*;

    #[test]
    fn test_purchase() {
        let args = Args {
            name: None,
            price: 8.96,
            code: "TEST".to_string(),
            sell: false,
            date: Some,
        };
        let result = purchase(&args).unwrap();
        assert_eq!("Bought 'TEST' at '8.96'", result);
    }
}
