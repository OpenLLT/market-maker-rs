//! Example demonstrating synchronous trait usage.
//!
//! This example shows how to use the `AvellanedaStoikov` trait to implement
//! a custom market-making strategy or use the default implementation.
//!
//! Run with: `cargo run --example trait_sync_example`

use market_maker_rs::prelude::*;
use market_maker_rs::strategy::interface::{AvellanedaStoikov, DefaultAvellanedaStoikov};
use market_maker_rs::types::error::MMResult;

/// Custom strategy that adds a fixed spread on top of the calculated quotes.
struct CustomStrategy {
    base_strategy: DefaultAvellanedaStoikov,
    min_additional_spread: Decimal,
}

impl CustomStrategy {
    fn new(min_additional_spread: Decimal) -> Self {
        Self {
            base_strategy: DefaultAvellanedaStoikov,
            min_additional_spread,
        }
    }
}

impl AvellanedaStoikov for CustomStrategy {
    fn calculate_reservation_price(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
    ) -> MMResult<Decimal> {
        // Delegate to base implementation
        self.base_strategy.calculate_reservation_price(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal_ms,
        )
    }

    fn calculate_optimal_spread(
        &self,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<Decimal> {
        // Get base spread and add our custom minimum
        let base_spread = self.base_strategy.calculate_optimal_spread(
            risk_aversion,
            volatility,
            time_to_terminal_ms,
            order_intensity,
        )?;

        Ok(base_spread + self.min_additional_spread)
    }

    fn calculate_optimal_quotes(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<(Decimal, Decimal)> {
        // Get reservation price
        let reservation = self.calculate_reservation_price(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal_ms,
        )?;

        // Get optimal spread with our custom addition
        let spread = self.calculate_optimal_spread(
            risk_aversion,
            volatility,
            time_to_terminal_ms,
            order_intensity,
        )?;

        let half_spread = spread / dec!(2);
        let bid = reservation - half_spread;
        let ask = reservation + half_spread;

        Ok((bid, ask))
    }
}

fn main() {
    println!("=== Synchronous Trait Usage Example ===\n");

    // Market parameters
    let mid_price = dec!(100.0);
    let inventory = dec!(10.0); // Long 10 units
    let risk_aversion = dec!(0.1);
    let volatility = dec!(0.2);
    let time_to_terminal = 3600000; // 1 hour
    let order_intensity = dec!(1.5);

    println!("Market Parameters:");
    println!("  Mid Price: ${}", mid_price);
    println!("  Inventory: {} units", inventory);
    println!("  Risk Aversion: {}", risk_aversion);
    println!("  Volatility: {}", volatility);
    println!("  Time to Terminal: {} ms", time_to_terminal);
    println!("  Order Intensity: {}", order_intensity);
    println!();

    // === Example 1: Using Default Strategy ===
    println!("=== Example 1: Default Strategy ===");
    let default_strategy = DefaultAvellanedaStoikov;

    let reservation = default_strategy
        .calculate_reservation_price(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal,
        )
        .expect("Failed to calculate reservation price");

    let spread = default_strategy
        .calculate_optimal_spread(risk_aversion, volatility, time_to_terminal, order_intensity)
        .expect("Failed to calculate spread");

    let (bid, ask) = default_strategy
        .calculate_optimal_quotes(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal,
            order_intensity,
        )
        .expect("Failed to calculate quotes");

    println!("Reservation Price: ${:.2}", reservation);
    println!("Optimal Spread: ${:.4}", spread);
    println!("Bid: ${:.2}", bid);
    println!("Ask: ${:.2}", ask);
    println!("Actual Spread: ${:.4}", ask - bid);
    println!();

    // === Example 2: Using Custom Strategy ===
    println!("=== Example 2: Custom Strategy (with additional spread) ===");
    let custom_strategy = CustomStrategy::new(dec!(0.5)); // Add $0.50 minimum spread

    let custom_spread = custom_strategy
        .calculate_optimal_spread(risk_aversion, volatility, time_to_terminal, order_intensity)
        .expect("Failed to calculate custom spread");

    let (custom_bid, custom_ask) = custom_strategy
        .calculate_optimal_quotes(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal,
            order_intensity,
        )
        .expect("Failed to calculate custom quotes");

    println!("Custom Spread: ${:.4}", custom_spread);
    println!("Custom Bid: ${:.2}", custom_bid);
    println!("Custom Ask: ${:.2}", custom_ask);
    println!("Actual Spread: ${:.4}", custom_ask - custom_bid);
    println!();

    // === Comparison ===
    println!("=== Comparison ===");
    println!("Additional Spread: ${:.4}", custom_spread - spread);
    println!("Bid Difference: ${:.2}", custom_bid - bid);
    println!("Ask Difference: ${:.2}", custom_ask - ask);

    println!("\n✓ Trait usage examples completed successfully!");
}
