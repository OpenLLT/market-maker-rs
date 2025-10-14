//! Inventory skew analysis example.
//!
//! Demonstrates how inventory position affects quote placement.
//! Shows the "skewing" behavior where quotes shift to encourage
//! inventory reduction.
//!
//! Run with: `cargo run --example inventory_skew`

use market_maker_rs::prelude::*;
use market_maker_rs::strategy::avellaneda_stoikov::*;

fn main() {
    println!("=== Inventory Skew Analysis ===\n");

    // Configuration
    let config = StrategyConfig::new(
        dec!(0.1),  // risk_aversion
        dec!(1.5),  // order_intensity
        3600000,    // terminal_time
        dec!(0.01), // min_spread
    )
    .expect("Failed to create config");

    let mid_price = dec!(100.0);
    let volatility = dec!(0.2);

    // Test different inventory levels
    let inventory_levels = vec![
        ("Large Short", dec!(-50.0)),
        ("Medium Short", dec!(-20.0)),
        ("Small Short", dec!(-5.0)),
        ("Flat", Decimal::ZERO),
        ("Small Long", dec!(5.0)),
        ("Medium Long", dec!(20.0)),
        ("Large Long", dec!(50.0)),
    ];

    println!("Configuration:");
    println!("  Mid Price: ${}", mid_price);
    println!("  Risk Aversion: {}", config.risk_aversion);
    println!("  Volatility: {}%", volatility * dec!(100.0));
    println!();

    println!(
        "{:15} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "Inventory", "Quantity", "Bid", "Ask", "Bid Skew", "Ask Skew", "Spread"
    );
    println!("{}", "-".repeat(85));

    // Calculate quotes for each inventory level
    let mut results = Vec::new();

    for (label, qty) in &inventory_levels {
        let (bid, ask) = calculate_optimal_quotes(
            mid_price,
            *qty,
            config.risk_aversion,
            volatility,
            config.terminal_time,
            config.order_intensity,
        )
        .expect("Failed to calculate quotes");

        let ten_thousand = dec!(10000.0);
        let hundred = dec!(100.0);
        let bid_skew = ((bid - mid_price) / mid_price * ten_thousand).round() / hundred;
        let ask_skew = ((ask - mid_price) / mid_price * ten_thousand).round() / hundred;
        let spread = ask - bid;

        results.push((label, qty, bid, ask, bid_skew, ask_skew, spread));

        println!(
            "{:15} {:>10.1} {:>10.2} {:>10.2} {:>9.2}% {:>9.2}% {:>10.4}",
            label, qty, bid, ask, bid_skew, ask_skew, spread
        );
    }

    println!();
    println!("=== Analysis ===\n");

    // Analyze skew behavior
    println!("Reservation Price Impact:");
    for (label, qty, _bid, _ask, _, _, _) in &results {
        let reservation = calculate_reservation_price(
            mid_price,
            **qty,
            config.risk_aversion,
            volatility,
            config.terminal_time,
        )
        .expect("Failed to calculate reservation");

        let impact = reservation - mid_price;
        let hundred = dec!(100.0);
        let impact_pct = (impact / mid_price * hundred).abs();

        println!(
            "  {:15} Reservation: ${:6.2} (impact: {:+6.2}, {:5.2}%)",
            label, reservation, impact, impact_pct
        );
    }

    println!();
    println!("Skew Insights:");
    println!("  • Long positions → Lower quotes (incentivize selling)");
    println!("  • Short positions → Higher quotes (incentivize buying)");
    println!("  • Larger positions → More aggressive skew");
    println!("  • Spread remains relatively constant");
    println!();

    // Calculate skew intensity
    let flat_quotes = results
        .iter()
        .find(|(l, _, _, _, _, _, _)| **l == "Flat")
        .unwrap();
    let large_long = results
        .iter()
        .find(|(l, _, _, _, _, _, _)| **l == "Large Long")
        .unwrap();
    let large_short = results
        .iter()
        .find(|(l, _, _, _, _, _, _)| **l == "Large Short")
        .unwrap();

    let hundred = dec!(100.0);
    println!("Skew Magnitude:");
    println!("  Flat → Large Long:");
    println!(
        "    Bid shift: ${:.2} ({:.2}%)",
        large_long.2 - flat_quotes.2,
        (large_long.2 - flat_quotes.2) / flat_quotes.2 * hundred
    );
    println!(
        "    Ask shift: ${:.2} ({:.2}%)",
        large_long.3 - flat_quotes.3,
        (large_long.3 - flat_quotes.3) / flat_quotes.3 * hundred
    );
    println!();
    println!("  Flat → Large Short:");
    println!(
        "    Bid shift: ${:.2} ({:.2}%)",
        large_short.2 - flat_quotes.2,
        (large_short.2 - flat_quotes.2) / flat_quotes.2 * hundred
    );
    println!(
        "    Ask shift: ${:.2} ({:.2}%)",
        large_short.3 - flat_quotes.3,
        (large_short.3 - flat_quotes.3) / flat_quotes.3 * hundred
    );
}
