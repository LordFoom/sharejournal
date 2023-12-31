use std::fmt::{Display, Formatter};
use chrono::NaiveDateTime;
use color_eyre::owo_colors::OwoColorize;
use crate::format::{get_date_string, get_price_string};

///A share, when we bought it, when we sold it, the price at either end
#[derive(Debug, Default)]
pub struct Share {
    pub name: String,
    pub code: String,
    pub buy_price: Option<i32>,
    pub buy_date: Option<NaiveDateTime>,
    pub sell_price: Option<i32>,
    pub sell_date: Option<NaiveDateTime>,
    pub create_date: NaiveDateTime,
}

impl Share {
    pub fn share_to_buy(
        share_name: String,
        share_code: String,
        buy_price: i32,
        buy_date: NaiveDateTime,
    ) -> Self {
        Self {
            name: share_name,
            code: share_code,
            buy_price: Some(buy_price),
            buy_date: Some(buy_date),
            ..Default::default()
        }
    }
    pub fn share_to_sell(
        share_name: String,
        share_code: String,
        sell_price: i32,
        sell_date: NaiveDateTime,
    ) -> Self {
        Self {
            name: share_name,
            code: share_code,
            sell_price: Some(sell_price),
            sell_date: Some(sell_date),
            ..Default::default()
        }
    }

}

impl Display for Share {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.code)
        let mut str_date = String::new();
        let mut str_price = String::new();
        let mut action = String::new();
        if let Some(dt) = self.buy_date {
            str_date = get_date_string(self.buy_date);
            str_price = get_price_string(self.buy_price);
            action = "BUY".to_string()
        }else if let Some(dt) = self.sell_date{
            str_date = get_date_string(self.sell_date);
            str_price = get_price_string(self.sell_price);
            action = "SELL".to_string()
        }else{
            str_date = "UNKNOWN".to_string();
            str_price = "UNKNOWN".to_string();
            action = "UNKNOWN".to_string()
        }
        write!(f, "Share: {} :: {} => Price: {}; Date: {} ", self.code.bold().blue(), action.bold().magenta(), str_price.red(), str_date.yellow(), )
    }

}

///A wrapper for our database so we can pass it around and get a connection`
pub struct Database {
    pub name: String,
}
