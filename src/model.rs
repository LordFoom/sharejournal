use chrono::NaiveDateTime;


///A share, when we bought it, when we sold it, the price at either end
#[derive(Debug, Default)]
pub struct Share {
    pub name: String,
    pub code: String,
    pub buy_price: f64,
    pub buy_date: NaiveDateTime,
    pub sell_price: f64,
    pub sell_date: NaiveDateTime,
    pub create_date: NaiveDateTime
}

impl Share {

    pub fn share_to_buy(share_name: String, share_code: String, buy_price: f64, buy_date: NaiveDateTime)-> Share {
        Share{
            name: share_name,
            code: share_code,
            buy_price,
            buy_date,
            ..Default::default()
        }
    }

}

///A wrapper for our database so we can pass it around and get a connection`
pub struct Database {
    pub name: String,
}
