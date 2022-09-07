use structopt::{StructOpt, clap::arg_enum};

use crate::converter::{CoinPrice, get_fiat_val, get_crypto_val};


#[derive(StructOpt,Debug)]
pub struct Cli {
    /// Specify the type of operation to perform
    #[structopt(short,required=true,possible_values=&Operator::variants())]
    op : Operator,

    /// secify the fiat curency eg (inr,usd)
    #[structopt(long,required=true)]
    fiat : String,

    /// specify cryptocurrency eg (eth,btc,ltc)
    #[structopt(long,required=true)]
    coin : String,

    /// specify the amount
    #[structopt(long,required=true)]
    val : f64
}

arg_enum! {
    #[derive(Debug)]
    enum Operator {
        ToFiat,
        ToCrypto
    }

}


pub fn parse_cli() {
    let cliap = Cli::from_args();
    match cliap.op {
        Operator::ToFiat => {
            match CoinPrice::from(cliap.coin, cliap.fiat,cliap.val) {
                Ok(inp) => {
                    match get_fiat_val(&inp) {
                        Ok(res) => {
                            println!("{} {} = {} {}",inp.amount,inp.base,res,inp.currency);

                        },
                        Err(e) => {
                            match e.to_string().as_str() {
                                "error decoding response body: missing field `data` at line 1 column 65" => {
                                    println!("[ERROR] the fiat or cryptocurrency type may be invalid")
                                },
                                _ => {
                                    
                                    println!("[ERROR] {}",e);
                                }
                            };

                        }
                    };

                },
                Err(e) => {
                    println!("[ERROR] {}",e);
                }
            };
        }
        Operator::ToCrypto => {
            match CoinPrice::from(cliap.coin, cliap.fiat,cliap.val) {
                Ok(inp) => {
                    match get_crypto_val(&inp) {
                        Ok(res) => {
                            println!("{} {} = {} {}",inp.amount,inp.currency,res,inp.base);

                        },
                        Err(e) => {
                            match e.to_string().as_str() {
                                "error decoding response body: missing field `data` at line 1 column 65" => {
                                    println!("[ERROR] the fiat or cryptocurrency type may be invalid")
                                },
                                _ => {
                                    
                                    println!("[ERROR] {}",e);
                                }
                            };

                        }
                    };

                },
                Err(e) => {
                    println!("[ERROR] {}",e);
                }
            };
        },
    }
}