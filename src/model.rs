use chrono::NaiveDateTime;

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

///A wrapper for our database so we can pass it around and get a connection`
pub struct Database {
    pub name: String,
}
