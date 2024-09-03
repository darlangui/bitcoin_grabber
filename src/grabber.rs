use serde::Deserialize;
use rust_decimal::prelude::*;

#[derive(Deserialize, Debug)]
struct PriceResponse {
    data: CryptoRate,
}

#[derive(Deserialize, Debug)]
struct CryptoRate {
    rateUsd: String,
}

pub fn get_bitcoin_price() -> Result<Decimal, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://api.coincap.io/v2/rates/bitcoin")?;
    let body = response.json::<PriceResponse>()?;
    
    let price = match Decimal::from_str(&body.data.rateUsd) {
        Ok(price) => price,
        Err(_) => {
            println!("Failed to parse price: {}", body.data.rateUsd);
            Decimal::new(0, 1) 
        }
    };

    Ok(price)
}
