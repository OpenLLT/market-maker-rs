//! Example demonstrating Display and Debug trait implementations using pretty-simple-display.
//!
//! This example shows how structs with serde feature enabled automatically get:
//! - Display implementation through DisplaySimple (compact JSON)
//! - Debug implementation through DebugPretty (pretty-printed JSON)

#[cfg(feature = "serde")]
use market_maker_rs::prelude::*;

#[cfg(feature = "serde")]
fn main() {
    println!("=== Display (Compact JSON) vs Debug (Pretty JSON) ===\n");

    // Create a quote
    let quote = Quote {
        bid_price: dec!(100.0),
        bid_size: dec!(10.0),
        ask_price: dec!(101.0),
        ask_size: dec!(10.0),
        timestamp: 1234567890,
    };

    println!("Quote Display (compact):");
    println!("{}", quote);
    println!("\nQuote Debug (pretty):");
    println!("{:?}\n", quote);

    // Create a strategy config
    let config = StrategyConfig::new(dec!(0.5), dec!(1.5), 3600000, dec!(0.01)).unwrap();
    println!("Strategy Config Display (compact):");
    println!("{}", config);
    println!("\nStrategy Config Debug (pretty):");
    println!("{:?}\n", config);

    // Create an inventory position
    let position = InventoryPosition {
        quantity: dec!(100.0),
        avg_entry_price: dec!(99.5),
        last_update: 1234567890,
    };

    println!("Inventory Position Display (compact):");
    println!("{}", position);
    println!("\nInventory Position Debug (pretty):");
    println!("{:?}\n", position);

    // Create PnL
    let pnl = PnL {
        realized: dec!(500.0),
        unrealized: dec!(150.0),
        total: dec!(650.0),
    };

    println!("PnL Display (compact):");
    println!("{}", pnl);
    println!("\nPnL Debug (pretty):");
    println!("{:?}\n", pnl);

    // Create market state
    let market_state = MarketState::new(dec!(100.5), dec!(0.25), 1234567890);
    println!("Market State Display (compact):");
    println!("{}", market_state);
    println!("\nMarket State Debug (pretty):");
    println!("{:?}", market_state);
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("This example requires the 'serde' feature to be enabled.");
    println!("Run with: cargo run --example display_example --features serde");
}
