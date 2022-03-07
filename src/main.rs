mod trader;

struct Prices {
    price: f32,
    previous_price: f32,
    moving_avg: f32,
}

fn shouldBuy(prices: Prices, quoteBalance: f32) -> bool {
    prices.price > prices.moving_avg
        && prices.previous_price < prices.moving_avg
        && quoteBalance > 1.0
}

fn main() {}
