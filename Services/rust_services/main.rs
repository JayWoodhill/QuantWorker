// src/main.rs

use arbitrage_models::{simulate_index_data, simulate_option_data};

fn main() {
    // Simulate option data and check for arbitrage
    let option = simulate_option_data();
    if let Some(arbitrage_opportunity) = option.check_arbitrage() {
        println!("Options Arbitrage Opportunity Detected:");
        println!("{}", arbitrage_opportunity);
    } else {
        println!("No options arbitrage opportunity detected.");
    }

    // Simulate index data and check for arbitrage
    let index = simulate_index_data();
    if let Some(arbitrage_opportunity) = index.check_arbitrage() {
        println!("\nIndex Arbitrage Opportunity Detected:");
        println!("{}", arbitrage_opportunity);
    } else {
        println!("No index arbitrage opportunity detected.");
    }
}
