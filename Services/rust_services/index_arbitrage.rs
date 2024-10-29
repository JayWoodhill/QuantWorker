// src/index_arbitrage.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexData {
    pub index_name: String,
    pub index_price: f64,
    pub futures_price: f64,
    pub interest_rate: f64,   // risk-free rate
    pub dividend_yield: f64,  // average dividend yield
    pub time_to_expiry: f64,  // in years
}

impl IndexData {
    /// Calculates the fair value of the index futures.
    pub fn fair_value(&self) -> f64 {
        self.index_price * ((self.interest_rate - self.dividend_yield) * self.time_to_expiry).exp()
    }

    /// Checks for arbitrage opportunities.
    pub fn check_arbitrage(&self) -> Option<String> {
        let fair_value = self.fair_value();
        let price_diff = fair_value - self.futures_price;

        // Set a threshold (e.g., 0.5% of the fair value)
        let threshold = 0.005 * fair_value.abs();

        if price_diff.abs() > threshold {
            if price_diff > 0.0 {
                Some(format!(
                    "Futures are underpriced by ${:.2}. Buy futures and sell the underlying index.",
                    price_diff
                ))
            } else {
                Some(format!(
                    "Futures are overpriced by ${:.2}. Sell futures and buy the underlying index.",
                    price_diff.abs()
                ))
            }
        } else {
            None
        }
    }
}
