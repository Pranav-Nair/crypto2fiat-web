

use serde::{Serialize, Deserialize};
use reqwest::{Client};


#[derive(Serialize, Deserialize, Debug)]
pub struct Prices {
    pub base : String,
    pub currency : String,
    pub amount : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub data : Prices
}

pub struct CoinPrice {
    pub base : String,
    pub currency : String,
    pub amount : f64
}

impl CoinPrice {
    pub fn from_price(obj : Prices) -> Result<CoinPrice,Box<dyn std::error::Error>> {
        Ok(CoinPrice {
            base : obj.base,
            currency : obj.currency,
            amount : obj.amount.parse::<f64>()?
        })
    }

    pub fn from(coin : String,fiat : String,val : f64) -> Result<CoinPrice,Box<dyn std::error::Error>> {
        Ok(CoinPrice {
            base : coin,
            currency : fiat,
            amount : val
        })
    }
}

#[tokio::main]
pub async fn get_data(coin : &String, fiat : &String) -> Result<CoinPrice,Box<dyn std::error::Error>> {
    let coin_in = &coin.to_uppercase();
    let fiat_in = &fiat.to_uppercase();
    let api_req = format!("https://api.coinbase.com/v2/prices/{}-{}/spot",coin_in,fiat_in);
    let client = Client::new();
    let resp = client.get(&api_req).send().await?;
    let res = resp.json::<Data>().await?;
    let coinprice = CoinPrice::from_price(res.data)?;
    Ok(coinprice)
}

pub fn get_crypto_val(inputprice : &CoinPrice) -> Result<f64,Box<dyn std::error::Error>> {
    let coinprice : CoinPrice = get_data(&inputprice.base, &inputprice.currency)?;
    let res = inputprice.amount / coinprice.amount;

    Ok(res)
}

pub fn get_fiat_val(inputprice : &CoinPrice) -> Result<f64,Box<dyn std::error::Error>> {
    let coinprice : CoinPrice = get_data(&inputprice.base, &inputprice.currency)?;
    let res = inputprice.amount * coinprice.amount;

    Ok(res)
}
