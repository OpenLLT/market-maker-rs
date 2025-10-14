//! Configuration comparison example.
//!
//! Compares different strategy configurations and their impact on quotes.
//!
//! Run with: `cargo run --example config_comparison`

use market_maker_rs::prelude::*;
use market_maker_rs::strategy::avellaneda_stoikov::*;

fn main() {
    println!("=== Strategy Configuration Comparison ===\n");

    let mid_price = dec!(100.0);
    let volatility = dec!(0.2);

    // Define different configurations
    let configs = vec![
        ("Conservative", dec!(0.5), dec!(1.0), 7200000, dec!(0.02)),
        ("Moderate", dec!(0.1), dec!(1.5), 3600000, dec!(0.01)),
        ("Aggressive", dec!(0.01), dec!(3.0), 1800000, dec!(0.005)),
        ("High Frequency", dec!(0.05), dec!(5.0), 900000, dec!(0.001)),
    ];

    println!(
        "{:15} {:>8} {:>10} {:>12} {:>10} {:>10} {:>10}",
        "Strategy", "Gamma", "k", "Terminal", "Bid", "Ask", "Spread"
    );
    println!("{}", "-".repeat(85));

    for (name, gamma, k, terminal, min_spread) in &configs {
        let config = StrategyConfig::new(*gamma, *k, *terminal, *min_spread)
            .expect("Failed to create config");

        let (bid, ask) = calculate_optimal_quotes(
            mid_price,
            Decimal::ZERO,
            config.risk_aversion,
            volatility,
            config.terminal_time,
            config.order_intensity,
        )
        .expect("Failed to calculate quotes");

        let spread = ask - bid;

        println!(
            "{:15} {:>8.2} {:>10.1} {:>12} {:>10.2} {:>10.2} {:>10.4}",
            name,
            gamma,
            k,
            format!("{}min", terminal / 60000),
            bid,
            ask,
            spread
        );
    }

    println!("\n=== Analysis ===\n");

    println!("Conservative Strategy:");
    println!("  • High risk aversion (0.5) → Wider spreads");
    println!("  • Low order intensity (1.0) → Less aggressive");
    println!("  • Long terminal time (2h) → More caution");
    println!();

    println!("Aggressive Strategy:");
    println!("  • Low risk aversion (0.01) → Tighter spreads");
    println!("  • High order intensity (3.0) → More volume expected");
    println!("  • Short terminal time (30min) → Less inventory risk");
    println!();

    println!("High Frequency Strategy:");
    println!("  • Very high order intensity (5.0) → Expects lots of fills");
    println!("  • Very tight spreads");
    println!("  • Short holding period (15min)");
}
