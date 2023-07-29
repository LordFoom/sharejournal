use clap::Parser;
use color_eyre::Report;
use Default;
use crate::db::buy_share;
use crate::model::Share;

mod model;
mod db;

///Arguments to the our app
#[derive(Parser, Debug)]
struct Args{
    code: String,
    price: f64,
    #[arg(short, long)]
    name: Option<String>,
    ///buy or sell
    #[arg(short, long)]
    sell: bool,
}

fn main()->Result<(), Report> {
    let args = Args::parse();
    //default is to buy
    if !args.sell {

    } else {

    }
    if !args.sell  {
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
    }else{
        "".to_string()
    };
    let share = Share {
        name,
        code: args.code.clone(),
        buy_price: args.price.clone(),
        ..Default::default()
    };
    buy_share(&share, &db)?;
    let result = format!("Bought '{}' at '{}'", share.code, share.buy_price);
    return Ok(result);
}


mod test {
    use super::*;

    #[test]
    fn test_purchase(){
        let args = Args{
            name: None,
            price: 8.96,
            code: "TEST".to_string(),
            sell: false,
        };
        let result = purchase(&args).unwrap();
        assert_eq!("Bought 'TEST' at '8.96'", result);
    }
}