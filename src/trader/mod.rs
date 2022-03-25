//mod ta;
mod key;

use binance::account::*;
use binance::api::*;
use binance::config::*;
use binance::errors::Error;
use key::{API_KEY, API_SECRET};

pub struct Trader {
    account: Account,
    symbol: String,
    base_balance: f32,
    quote_balance: f32,
}

impl Trader {
    pub fn new() -> Result<Trader, Error> {
        let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
        let account: Account = Binance::new_with_config(Some(API_KEY), Some(API_SECRET), &config);

        let symbol: String = "BTCUSDT".into();

        let account_infos = account.get_account();
        match account_infos {
            Ok(account_infos) => {
                let mut base_balance: f32 = 0.;
                let mut quote_balance: f32 = 0.;
                for balance in account_infos.balances {
                    if balance.asset == symbol.get(0..3).unwrap() {
                        base_balance = balance.free.parse().unwrap();
                    } else if balance.asset == symbol.get(3..).unwrap() {
                        quote_balance = balance.free.parse().unwrap();
                    }
                }
                println!(
                    "{} - Base: {} - Quote: {}",
                    symbol, base_balance, quote_balance,
                );
                let trader = Trader {
                    account,
                    symbol,
                    base_balance,
                    quote_balance,
                };
                Ok(trader)
            }
            Err(e) => Err(e),
        }
    }
}
