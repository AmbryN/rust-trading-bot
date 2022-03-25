//mod trader;
use std::env;
use std::process;
use trading::Config;
//use trader::Trader;

// struct Prices {
//     price: f32,
//     previous_price: f32,
//     moving_avg: f32,
// }

// fn _should_buy(prices: Prices, quote_balance: f32) -> bool {
//     prices.price > prices.moving_avg
//         && prices.previous_price < prices.moving_avg
//         && quote_balance > 1.0
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = trading::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
