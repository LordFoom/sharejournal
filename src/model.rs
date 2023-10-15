use std::fmt::{Display, Formatter};
use chrono::NaiveDateTime;
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
        let str_buy_date = get_date_string(self.buy_date);
        let str_sell_date = get_date_string(self.sell_date);
        let str_buy_price = get_price_string(self.buy_price);
        let str_sell_price = get_price_string(self.sell_price);
        write!(f, "{}:{}/{} - {}/{}", self.code, str_buy_date,str_sell_date, str_buy_price, str_sell_price)
    }

}

///A wrapper for our database so we can pass it around and get a connection`
pub struct Database {
    pub name: String,
}
