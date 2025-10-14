//! Basic quote generation example.
//!
//! Demonstrates the fundamental functionality of calculating optimal quotes
//! using the Avellaneda-Stoikov model.
//!
//! Run with: `cargo run --example basic_quoting`

use market_maker_rs::prelude::*;
use market_maker_rs::strategy::avellaneda_stoikov::*;

fn main() {
    println!("=== Basic Quote Generation ===\n");

    // Strategy parameters
    let risk_aversion = dec!(0.1); // gamma - risk aversion parameter
    let order_intensity = dec!(1.5); // k - order arrival rate
    let time_to_terminal = 3600000; // 1 hour in milliseconds
    let volatility = dec!(0.2); // 20% annualized volatility

    // Market conditions
    let mid_price = dec!(100.0);
    let inventory = Decimal::ZERO; // Start flat

    println!("Market Conditions:");
    println!("  Mid Price: ${}", mid_price);
    println!("  Volatility: {}%", volatility * dec!(100.0));
    println!(
        "  Time to Terminal: {} ms ({} hours)",
        time_to_terminal,
        time_to_terminal / 3_600_000
    );
    println!();

    // Calculate reservation price
    let reservation_price = calculate_reservation_price(
        mid_price,
        inventory,
        risk_aversion,
        volatility,
        time_to_terminal,
    )
    .expect("Failed to calculate reservation price");

    println!("Reservation Price: ${:.2}", reservation_price);
    println!("  (Fair value adjusted for inventory risk)");
    println!();

    // Calculate optimal spread
    let spread =
        calculate_optimal_spread(risk_aversion, volatility, time_to_terminal, order_intensity)
            .expect("Failed to calculate spread");

    println!("Optimal Spread: ${:.4}", spread);
    println!();

    // Calculate bid and ask
    let (bid, ask) = calculate_optimal_quotes(
        mid_price,
        inventory,
        risk_aversion,
        volatility,
        time_to_terminal,
        order_intensity,
    )
    .expect("Failed to calculate quotes");

    let hundred = dec!(100.0);
    println!("Optimal Quotes:");
    println!(
        "  Bid: ${:.2} ({:.2}% below mid)",
        bid,
        (mid_price - bid) / mid_price * hundred
    );
    println!(
        "  Ask: ${:.2} ({:.2}% above mid)",
        ask,
        (ask - mid_price) / mid_price * hundred
    );
    println!("  Spread: ${:.2}", ask - bid);
    println!();

    // Create Quote struct
    let quote = Quote {
        bid_price: bid,
        bid_size: dec!(10.0),
        ask_price: ask,
        ask_size: dec!(10.0),
        timestamp: 0,
    };

    println!("Quote Summary:");
    println!("  Mid: ${:.2}", quote.mid_price());
    println!("  Spread: ${:.4}", quote.spread());
    println!(
        "  Spread %: {:.3}%",
        quote.spread() / quote.mid_price() * hundred
    );
}
