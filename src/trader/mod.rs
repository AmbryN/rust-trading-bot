use binance::account::*;
use binance::api::*;
use binance::config::*;

mod model;
//mod ta;

pub struct Trader {
    account: Account,
    symbol: String,
    balances: Balances,
}

impl Trader {
    fn init(self) {
        let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
        self.account = Binance::new_with_config(
            Some("a2XZQu8hAcUHJwpYQ9ImJcx6Gm5CjZ9TwB86yWJtYkTKrAv0IToyQ21xrldqeO7S".to_string()),
            Some("RBrHnQfWZbO65rsZcIpx0aKNqBFfAwlEy1ElBHfcIzEHhJSTTPN3gcKLJ51KdjQQ".to_string()),
            &config,
        );

        self.symbol = "BTCUSDT".into();

        let account_infos = account.get_account();
        match account_infos {
            Ok(account_infos) => {
                for balance in account_infos.balances {
                    if balance.asset == symbol.get(0..3).unwrap() {
                        self.base_balance = balance.free.parse().unwrap();
                    } else if balance.asset == symbol.get(3..).unwrap() {
                        self.quote_balance = balance.free.parse().unwrap();
                    }
                }
                println!(
                    "{} - Base: {} - Quote: {}",
                    symbol, base_balance, quote_balance
                );
            }
            Err(e) => println!("Error {:?}", e),
        }
    }
}
