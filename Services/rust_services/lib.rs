// src/lib.rs

pub mod options;
pub mod index_arbitrage;

// src/lib.rs (continue)

use rand::Rng;

pub fn simulate_option_data() -> options::OptionData {
    let mut rng = rand::thread_rng();

    options::OptionData {
        symbol: "AAPL".to_string(),
        underlying_price: rng.gen_range(140.0..150.0),
        strike_price: 145.0,
        interest_rate: 0.005,
        volatility: 0.2,
        time_to_expiry: 30.0 / 365.0, // 30 days
        option_type: "call".to_string(),
        market_price: rng.gen_range(5.0..10.0),
    }
}

pub fn simulate_index_data() -> index_arbitrage::IndexData {
    let mut rng = rand::thread_rng();

    index_arbitrage::IndexData {
        index_name: "S&P 500".to_string(),
        index_price: rng.gen_range(4400.0..4500.0),
        futures_price: rng.gen_range(4400.0..4500.0),
        interest_rate: 0.005,
        dividend_yield: 0.02,
        time_to_expiry: 60.0 / 365.0, // 60 days
    }
}
