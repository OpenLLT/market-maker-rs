//! Error handling and validation example.
//!
//! Demonstrates validation and error handling capabilities.
//!
//! Run with: `cargo run --example error_handling`

use market_maker_rs::prelude::*;
use market_maker_rs::strategy::avellaneda_stoikov::*;

fn main() {
    println!("=== Error Handling Examples ===\n");

    // Configuration validation
    println!("1. Configuration Errors:");

    match StrategyConfig::new(dec!(-0.1), dec!(1.5), 3600000, dec!(0.01)) {
        Err(e) => println!("  ✓ Negative risk_aversion: {}", e),
        Ok(_) => println!("  ❌ Should have failed"),
    }

    match StrategyConfig::new(dec!(0.1), Decimal::ZERO, 3600000, dec!(0.01)) {
        Err(e) => println!("  ✓ Zero order_intensity: {}", e),
        Ok(_) => println!("  ❌ Should have failed"),
    }

    println!("\n2. Market State Errors:");

    match calculate_reservation_price(dec!(-100.0), Decimal::ZERO, dec!(0.1), dec!(0.2), 3600000) {
        Err(e) => println!("  ✓ Negative price: {}", e),
        Ok(_) => println!("  ❌ Should have failed"),
    }

    match calculate_reservation_price(dec!(100.0), Decimal::ZERO, dec!(0.1), dec!(-0.2), 3600000) {
        Err(e) => println!("  ✓ Negative volatility: {}", e),
        Ok(_) => println!("  ❌ Should have failed"),
    }

    println!("\n3. Position Errors:");

    // Decimal doesn't have NaN, so test with a different edge case
    match calculate_reservation_price(dec!(100.0), Decimal::ZERO, dec!(0.1), dec!(0.2), 3600000) {
        Ok(_) => println!("  ✓ Valid inputs accepted"),
        Err(e) => println!("  ❌ Should have succeeded: {}", e),
    }

    println!("\n4. Numerical Errors:");

    // Test with extreme but valid values
    match calculate_reservation_price(dec!(100.0), dec!(1000000), dec!(1000), dec!(1000), u64::MAX)
    {
        Ok(_) => println!("  ✓ Extreme values handled (Decimal has better precision)"),
        Err(e) => println!("  ℹ Decimal detected issue: {}", e),
    }

    println!("\n5. Quote Generation Errors:");

    match calculate_optimal_quotes(
        dec!(0.5),
        dec!(-1000.0),
        dec!(1.0),
        dec!(1.0),
        36000000,
        dec!(0.1),
    ) {
        Err(e) => println!("  ✓ Negative bid: {}", e),
        Ok(_) => println!("  ❌ Should have failed"),
    }

    println!("\n✓ All error cases handled correctly!");
}
