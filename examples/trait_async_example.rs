//! Example demonstrating asynchronous trait usage.
//!
//! This example shows how to use the `AsyncAvellanedaStoikov` trait to implement
//! async market-making strategies that could integrate with external data sources.
//!
//! Run with: `cargo run --example trait_async_example`

use async_trait::async_trait;
use market_maker_rs::prelude::*;
use market_maker_rs::strategy::interface::{AsyncAvellanedaStoikov, DefaultAvellanedaStoikov};
use market_maker_rs::types::error::MMResult;

/// Async strategy that simulates fetching real-time volatility from an external API.
struct RealTimeVolatilityStrategy {
    base_strategy: DefaultAvellanedaStoikov,
}

impl RealTimeVolatilityStrategy {
    fn new() -> Self {
        Self {
            base_strategy: DefaultAvellanedaStoikov,
        }
    }

    /// Simulates an async call to fetch real-time volatility.
    async fn fetch_real_time_volatility(&self) -> Decimal {
        // In a real implementation, this would call an external API
        // For this example, we simulate with a small delay and return a value
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        dec!(0.25) // Return simulated volatility
    }
}

#[async_trait]
impl AsyncAvellanedaStoikov for RealTimeVolatilityStrategy {
    async fn calculate_reservation_price(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        _volatility: Decimal, // Ignored, we'll fetch our own
        time_to_terminal_ms: u64,
    ) -> MMResult<Decimal> {
        // Fetch real-time volatility asynchronously
        let real_volatility = self.fetch_real_time_volatility().await;

        // Delegate to base implementation with real volatility
        self.base_strategy
            .calculate_reservation_price(
                mid_price,
                inventory,
                risk_aversion,
                real_volatility,
                time_to_terminal_ms,
            )
            .await
    }

    async fn calculate_optimal_spread(
        &self,
        risk_aversion: Decimal,
        _volatility: Decimal, // Ignored, we'll fetch our own
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<Decimal> {
        // Fetch real-time volatility asynchronously
        let real_volatility = self.fetch_real_time_volatility().await;

        self.base_strategy
            .calculate_optimal_spread(
                risk_aversion,
                real_volatility,
                time_to_terminal_ms,
                order_intensity,
            )
            .await
    }

    async fn calculate_optimal_quotes(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        _volatility: Decimal, // Ignored, we'll fetch our own
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<(Decimal, Decimal)> {
        // Fetch real-time volatility asynchronously
        let real_volatility = self.fetch_real_time_volatility().await;

        self.base_strategy
            .calculate_optimal_quotes(
                mid_price,
                inventory,
                risk_aversion,
                real_volatility,
                time_to_terminal_ms,
                order_intensity,
            )
            .await
    }
}

#[tokio::main]
async fn main() {
    println!("=== Asynchronous Trait Usage Example ===\n");

    // Market parameters
    let mid_price = dec!(100.0);
    let inventory = dec!(-5.0); // Short 5 units
    let risk_aversion = dec!(0.1);
    let volatility = dec!(0.2); // This will be ignored by our custom strategy
    let time_to_terminal = 3600000; // 1 hour
    let order_intensity = dec!(1.5);

    println!("Market Parameters:");
    println!("  Mid Price: ${}", mid_price);
    println!("  Inventory: {} units", inventory);
    println!("  Risk Aversion: {}", risk_aversion);
    println!("  Static Volatility: {}", volatility);
    println!("  Time to Terminal: {} ms", time_to_terminal);
    println!("  Order Intensity: {}", order_intensity);
    println!();

    // === Example 1: Using Default Async Strategy ===
    println!("=== Example 1: Default Async Strategy ===");
    let default_strategy = DefaultAvellanedaStoikov;

    println!("Calculating quotes asynchronously...");
    let start = tokio::time::Instant::now();

    let (bid, ask) = default_strategy
        .calculate_optimal_quotes(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal,
            order_intensity,
        )
        .await
        .expect("Failed to calculate quotes");

    let elapsed = start.elapsed();
    println!("Bid: ${:.2}", bid);
    println!("Ask: ${:.2}", ask);
    println!("Spread: ${:.4}", ask - bid);
    println!("Time taken: {:?}", elapsed);
    println!();

    // === Example 2: Using Custom Async Strategy with Real-Time Volatility ===
    println!("=== Example 2: Real-Time Volatility Strategy ===");
    let rt_strategy = RealTimeVolatilityStrategy::new();

    println!("Fetching real-time volatility and calculating quotes...");
    let start = tokio::time::Instant::now();

    let (rt_bid, rt_ask) = rt_strategy
        .calculate_optimal_quotes(
            mid_price,
            inventory,
            risk_aversion,
            volatility, // This will be replaced with real-time data
            time_to_terminal,
            order_intensity,
        )
        .await
        .expect("Failed to calculate real-time quotes");

    let elapsed = start.elapsed();
    println!("Bid: ${:.2}", rt_bid);
    println!("Ask: ${:.2}", rt_ask);
    println!("Spread: ${:.4}", rt_ask - rt_bid);
    println!("Time taken: {:?} (includes simulated API call)", elapsed);
    println!();

    // === Example 3: Parallel Quote Generation ===
    println!("=== Example 3: Parallel Quote Generation for Multiple Symbols ===");

    let symbols = vec![
        ("BTC/USD", dec!(100.0)),
        ("ETH/USD", dec!(50.0)),
        ("SOL/USD", dec!(25.0)),
    ];

    println!(
        "Generating quotes for {} symbols in parallel...",
        symbols.len()
    );
    let start = tokio::time::Instant::now();

    let mut handles = vec![];

    for (symbol, symbol_mid_price) in symbols {
        let strategy = RealTimeVolatilityStrategy::new();
        let handle = tokio::spawn(async move {
            let (bid, ask) = strategy
                .calculate_optimal_quotes(
                    symbol_mid_price,
                    Decimal::ZERO,
                    dec!(0.1),
                    dec!(0.2),
                    3600000,
                    dec!(1.5),
                )
                .await
                .expect("Failed to calculate quotes");
            (symbol, bid, ask)
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let (symbol, bid, ask) = handle.await.expect("Task failed");
        println!(
            "  {}: Bid ${:.2}, Ask ${:.2}, Spread ${:.4}",
            symbol,
            bid,
            ask,
            ask - bid
        );
    }

    let elapsed = start.elapsed();
    println!("Total time: {:?} (parallel execution)", elapsed);

    println!("\n✓ Async trait usage examples completed successfully!");
}
