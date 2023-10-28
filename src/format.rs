use chrono::NaiveDateTime;

///Turn our maybe_date into a
pub fn get_date_string(maybe_date: Option<NaiveDateTime>) ->String {
    if let Some(dt) = maybe_date {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else{
        String::new()
    }
}

///Turn i32 into decimal string
pub fn get_price_string(maybe_price: Option<i32>)->String {
    if let Some(price) = maybe_price {
        //format as decimal
        let cents = price % 100;
        let non_cents: i32 = price / 100;
        return format!("{}.{}", cents, non_cents);
    }
    return "".to_string()
}

#[cfg(test)]
mod test{
    use chrono::Utc;
    use super::*;

    #[test]
    pub fn test_get_price_string(){
        let some_price = Some(133456);
        let price = get_price_string(some_price);
        assert_eq!("1334.56", price);

        let none_price = get_price_string(None);
        assert_eq!("", none_price);
    }

    #[test]
    pub fn test_get_date_string() {
        let date = NaiveDateTime::parse_from_str("2023-10-15 19:47:22", "%Y-%m-%d %H:%M:%S").unwrap();
        let some_date = Some(date);
        let str_date = get_date_string(some_date);
        assert_eq!("2023-10-15 19:47:22", str_date);

        let str_date = get_date_string(None);
        println!("str_date: {}", str_date);
        assert_eq!("", str_date);
    }
}