// tests/tests.rs

use arbitrage_models::options::OptionData;
use arbitrage_models::index_arbitrage::IndexData;
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_black_scholes_price() {
    let option = OptionData {
        symbol: "TEST".to_string(),
        underlying_price: 100.0,
        strike_price: 100.0,
        interest_rate: 0.01,
        volatility: 0.2,
        time_to_expiry: 0.5, // 6 months
        option_type: "call".to_string(),
        market_price: 0.0,
    };

    let price = option.black_scholes_price();
    // Expected price calculated using a standard calculator
    let expected_price = 5.876; // Approximate value
    assert_approx_eq!(price, expected_price, 0.1);
}

#[test]
fn test_option_arbitrage() {
    let option = OptionData {
        symbol: "TEST".to_string(),
        underlying_price: 100.0,
        strike_price: 100.0,
        interest_rate: 0.01,
        volatility: 0.2,
        time_to_expiry: 0.5,
        option_type: "call".to_string(),
        market_price: 3.0, // Underpriced
    };

    assert!(option.check_arbitrage().is_some());
}

#[test]
fn test_fair_value() {
    let index = IndexData {
        index_name: "TEST_INDEX".to_string(),
        index_price: 1000.0,
        futures_price: 0.0,
        interest_rate: 0.01,
        dividend_yield: 0.02,
        time_to_expiry: 0.25, // 3 months
    };

    let fair_value = index.fair_value();
    let expected_value = 1000.0 * ((0.01 - 0.02) * 0.25).exp();
    assert_approx_eq!(fair_value, expected_value, 0.1);
}

#[test]
fn test_index_arbitrage() {
    let index = IndexData {
        index_name: "TEST_INDEX".to_string(),
        index_price: 1000.0,
        futures_price: 980.0, // Underpriced
        interest_rate: 0.01,
        dividend_yield: 0.02,
        time_to_expiry: 0.25,
    };

    assert!(index.check_arbitrage().is_some());
}
