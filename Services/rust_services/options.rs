// src/options.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionData {
    pub symbol: String,
    pub underlying_price: f64,
    pub strike_price: f64,
    pub interest_rate: f64,  // risk-free rate
    pub volatility: f64,     // implied volatility
    pub time_to_expiry: f64, // in years
    pub option_type: String, // "call" or "put"
    pub market_price: f64,   // current option price in the market
}

impl OptionData {
    /// Calculates the theoretical option price using the Black-Scholes model.
    pub fn black_scholes_price(&self) -> f64 {
        let d1 = (self.underlying_price / self.strike_price).ln()
            + (self.interest_rate + 0.5 * self.volatility.powi(2)) * self.time_to_expiry;
        let d1 = d1 / (self.volatility * self.time_to_expiry.sqrt());
        let d2 = d1 - self.volatility * self.time_to_expiry.sqrt();

        match self.option_type.as_str() {
            "call" => {
                let price = self.underlying_price * norm_cdf(d1)
                    - self.strike_price * (-self.interest_rate * self.time_to_expiry).exp() * norm_cdf(d2);
                price
            }
            "put" => {
                let price = self.strike_price * (-self.interest_rate * self.time_to_expiry).exp() * norm_cdf(-d2)
                    - self.underlying_price * norm_cdf(-d1);
                price
            }
            _ => 0.0,
        }
    }

    /// Checks for arbitrage opportunities.
    pub fn check_arbitrage(&self) -> Option<String> {
        let theoretical_price = self.black_scholes_price();
        let price_diff = theoretical_price - self.market_price;

        // Set a threshold (e.g., 1% of the theoretical price)
        let threshold = 0.01 * theoretical_price.abs();

        if price_diff.abs() > threshold {
            if price_diff > 0.0 {
                Some(format!(
                    "Option {} is underpriced by ${:.2}. Buy the option.",
                    self.symbol, price_diff
                ))
            } else {
                Some(format!(
                    "Option {} is overpriced by ${:.2}. Sell the option.",
                    self.symbol, price_diff.abs()
                ))
            }
        } else {
            None
        }
    }
}

/// Cumulative distribution function for the standard normal distribution.
fn norm_cdf(x: f64) -> f64 {
    (1.0 + (x / (1.0 + 0.2316419 * x.abs())).exp()) / 2.0
}
