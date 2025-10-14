//! Parameter sensitivity analysis example.
//!
//! Demonstrates how different strategy parameters affect quote generation:
//! - Risk aversion (gamma)
//! - Volatility (sigma)
//! - Order intensity (k)
//! - Time to terminal
//!
//! Run with: `cargo run --example parameter_sensitivity`

use market_maker_rs::prelude::*;
use market_maker_rs::strategy::avellaneda_stoikov::*;

fn main() {
    println!("=== Parameter Sensitivity Analysis ===\n");

    let base_mid_price = dec!(100.0);
    let base_inventory = Decimal::ZERO;

    // === 1. Risk Aversion Sensitivity ===
    println!("=== 1. Risk Aversion (gamma) Impact ===\n");
    println!("Higher gamma = More risk-averse = Wider spreads\n");

    let risk_aversions = vec![dec!(0.01), dec!(0.05), dec!(0.1), dec!(0.5), dec!(1.0)];
    println!(
        "{:>8} {:>10} {:>10} {:>10}",
        "Gamma", "Bid", "Ask", "Spread"
    );
    println!("{}", "-".repeat(42));

    for &gamma in &risk_aversions {
        let (bid, ask) = calculate_optimal_quotes(
            base_mid_price,
            base_inventory,
            gamma,
            dec!(0.2),
            3600000,
            dec!(1.5),
        )
        .expect("Failed to calculate");

        println!(
            "{:>8.2} {:>10.2} {:>10.2} {:>10.4}",
            gamma,
            bid,
            ask,
            ask - bid
        );
    }
    println!();

    // === 2. Volatility Sensitivity ===
    println!("=== 2. Volatility (sigma) Impact ===\n");
    println!("Higher volatility = Wider spreads (more uncertainty)\n");

    let volatilities = vec![dec!(0.05), dec!(0.1), dec!(0.2), dec!(0.4), dec!(0.8)];
    println!("{:>8} {:>10} {:>10} {:>10}", "Vol", "Bid", "Ask", "Spread");
    println!("{}", "-".repeat(42));

    for &vol in &volatilities {
        let (bid, ask) = calculate_optimal_quotes(
            base_mid_price,
            base_inventory,
            dec!(0.1),
            vol,
            3600000,
            dec!(1.5),
        )
        .expect("Failed to calculate");

        println!(
            "{:>7.0}% {:>10.2} {:>10.2} {:>10.4}",
            vol * dec!(100.0),
            bid,
            ask,
            ask - bid
        );
    }
    println!();

    // === 3. Order Intensity Sensitivity ===
    println!("=== 3. Order Intensity (k) Impact ===\n");
    println!("Higher k = More orders expected = Tighter spreads\n");

    let intensities = vec![dec!(0.1), dec!(0.5), dec!(1.0), dec!(2.0), dec!(5.0)];
    println!("{:>8} {:>10} {:>10} {:>10}", "k", "Bid", "Ask", "Spread");
    println!("{}", "-".repeat(42));

    for &k in &intensities {
        let (bid, ask) = calculate_optimal_quotes(
            base_mid_price,
            base_inventory,
            dec!(0.1),
            dec!(0.2),
            3600000,
            k,
        )
        .expect("Failed to calculate");

        println!("{:>8.1} {:>10.2} {:>10.2} {:>10.4}", k, bid, ask, ask - bid);
    }
    println!();

    // === 4. Time to Terminal Sensitivity ===
    println!("=== 4. Time to Terminal Impact ===\n");
    println!("More time = More inventory risk = Wider spreads\n");

    let times = vec![
        (60000, "1 min"),
        (300000, "5 min"),
        (900000, "15 min"),
        (1800000, "30 min"),
        (3600000, "1 hour"),
        (7200000, "2 hours"),
    ];

    println!(
        "{:>10} {:>12} {:>10} {:>10} {:>10}",
        "Time", "Milliseconds", "Bid", "Ask", "Spread"
    );
    println!("{}", "-".repeat(56));

    for &(time_ms, label) in &times {
        let (bid, ask) = calculate_optimal_quotes(
            base_mid_price,
            base_inventory,
            dec!(0.1),
            dec!(0.2),
            time_ms,
            dec!(1.5),
        )
        .expect("Failed to calculate");

        println!(
            "{:>10} {:>12} {:>10.2} {:>10.2} {:>10.4}",
            label,
            time_ms,
            bid,
            ask,
            ask - bid
        );
    }
    println!();

    // === 5. Combined Effects ===
    println!("=== 5. Combined Parameter Effects ===\n");

    println!("Scenario A: Conservative (low risk, high vol)");
    let (bid_a, ask_a) = calculate_optimal_quotes(
        base_mid_price,
        base_inventory,
        dec!(0.5),
        dec!(0.4),
        3600000,
        dec!(1.0),
    )
    .expect("Failed");
    println!(
        "  Bid: ${:.2}, Ask: ${:.2}, Spread: ${:.4}",
        bid_a,
        ask_a,
        ask_a - bid_a
    );
    println!();

    println!("Scenario B: Aggressive (high risk tolerance, low vol)");
    let (bid_b, ask_b) = calculate_optimal_quotes(
        base_mid_price,
        base_inventory,
        dec!(0.01),
        dec!(0.1),
        3600000,
        dec!(5.0),
    )
    .expect("Failed");
    println!(
        "  Bid: ${:.2}, Ask: ${:.2}, Spread: ${:.4}",
        bid_b,
        ask_b,
        ask_b - bid_b
    );
    println!();

    println!("Scenario C: Balanced");
    let (bid_c, ask_c) = calculate_optimal_quotes(
        base_mid_price,
        base_inventory,
        dec!(0.1),
        dec!(0.2),
        3600000,
        dec!(1.5),
    )
    .expect("Failed");
    println!(
        "  Bid: ${:.2}, Ask: ${:.2}, Spread: ${:.4}",
        bid_c,
        ask_c,
        ask_c - bid_c
    );
    println!();

    // === Summary ===
    println!("=== Key Insights ===\n");
    println!("• Risk Aversion (γ):");
    println!("    Higher → Wider spreads (more protection)");
    println!();
    println!("• Volatility (σ):");
    println!("    Higher → Wider spreads (more uncertainty)");
    println!();
    println!("• Order Intensity (k):");
    println!("    Higher → Tighter spreads (more liquidity expected)");
    println!();
    println!("• Time to Terminal (T-t):");
    println!("    Longer → Wider spreads (more inventory risk)");
}
