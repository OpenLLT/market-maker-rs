//! Volatility estimation example.
//!
//! This example demonstrates different methods for estimating market volatility
//! from historical price data. Volatility is a crucial input for the
//! Avellaneda-Stoikov market-making strategy.
//!
//! Run with: `cargo run --example volatility_estimation`

use market_maker_rs::dec;
use market_maker_rs::market_state::volatility::VolatilityEstimator;

fn main() {
    println!("=== Volatility Estimation Example ===\n");

    // Sample price data: 20 days of closing prices
    let prices = vec![
        dec!(100.00),
        dec!(101.20),
        dec!(100.80),
        dec!(102.50),
        dec!(103.00),
        dec!(102.20),
        dec!(101.50),
        dec!(103.80),
        dec!(104.50),
        dec!(103.90),
        dec!(105.20),
        dec!(104.80),
        dec!(106.00),
        dec!(105.50),
        dec!(107.20),
        dec!(106.80),
        dec!(108.00),
        dec!(107.50),
        dec!(109.00),
        dec!(108.50),
    ];

    println!("Historical Prices (20 days):");
    println!("  Start: ${:.2}", prices[0]);
    println!("  End:   ${:.2}", prices[prices.len() - 1]);
    println!(
        "  Return: {:.2}%",
        ((prices[prices.len() - 1] / prices[0] - dec!(1)) * dec!(100))
    );
    println!();

    // Create estimator
    let estimator = VolatilityEstimator::new();

    // === Method 1: Simple Historical Volatility ===
    println!("=== Method 1: Simple Historical Volatility ===");
    let simple_vol = estimator
        .calculate_simple(&prices)
        .expect("Failed to calculate simple volatility");

    println!(
        "Annualized Volatility: {:.4} ({:.2}%)",
        simple_vol,
        simple_vol * dec!(100)
    );
    println!("Formula: σ = sqrt(Σ(r_i - mean(r))² / (n-1)) * sqrt(252)");
    println!("  where r_i = ln(P_i / P_{{i-1}})");
    println!();

    // === Method 2: EWMA Volatility ===
    println!("=== Method 2: EWMA (Exponentially Weighted Moving Average) ===");
    let ewma_vol = estimator
        .calculate_ewma(&prices, dec!(0.94))
        .expect("Failed to calculate EWMA volatility");

    println!(
        "Annualized Volatility: {:.4} ({:.2}%)",
        ewma_vol,
        ewma_vol * dec!(100)
    );
    println!("Lambda (decay factor): 0.94 (RiskMetrics standard for daily data)");
    println!("Formula: σ²_t = λ * σ²_{{t-1}} + (1-λ) * r²_t");
    println!("  Gives more weight to recent observations");
    println!();

    // === Method 3: Parkinson's Range-Based Estimator ===
    println!("=== Method 3: Parkinson's Range-Based Estimator ===");

    // Simulate high-low data (typically would come from OHLC bars)
    let highs: Vec<_> = prices.iter().map(|&p| p * dec!(1.015)).collect();
    let lows: Vec<_> = prices.iter().map(|&p| p * dec!(0.985)).collect();

    let parkinson_vol = estimator
        .calculate_parkinson(&highs, &lows)
        .expect("Failed to calculate Parkinson volatility");

    println!(
        "Annualized Volatility: {:.4} ({:.2}%)",
        parkinson_vol,
        parkinson_vol * dec!(100)
    );
    println!("Formula: σ = sqrt(Σ(ln(H_i/L_i))² / (4*n*ln(2)))");
    println!("  More efficient than close-to-close methods");
    println!("  Uses intraday high-low range information");
    println!();

    // === Comparison ===
    println!("=== Comparison of Methods ===");
    println!("{:30} {:>10}", "Method", "Volatility");
    println!("{}", "-".repeat(42));
    println!(
        "{:30} {:>9.2}%",
        "Simple Historical",
        simple_vol * dec!(100)
    );
    println!("{:30} {:>9.2}%", "EWMA (λ=0.94)", ewma_vol * dec!(100));
    println!(
        "{:30} {:>9.2}%",
        "Parkinson Range-Based",
        parkinson_vol * dec!(100)
    );
    println!();

    // === Using with Avellaneda-Stoikov ===
    println!("=== Using Volatility in Market-Making Strategy ===");

    use market_maker_rs::strategy::avellaneda_stoikov::calculate_optimal_quotes;

    let mid_price = prices[prices.len() - 1];
    let inventory = dec!(0.0); // Flat position
    let risk_aversion = dec!(0.1);
    let time_to_terminal = 3600000; // 1 hour
    let order_intensity = dec!(1.5);

    println!("Strategy Parameters:");
    println!("  Mid Price: ${:.2}", mid_price);
    println!("  Inventory: {} units", inventory);
    println!("  Risk Aversion: {}", risk_aversion);
    println!("  Time to Terminal: 1 hour");
    println!("  Order Intensity: {}", order_intensity);
    println!();

    println!("Optimal Quotes with Different Volatility Estimates:");
    println!();

    // With simple volatility
    let (bid, ask) = calculate_optimal_quotes(
        mid_price,
        inventory,
        risk_aversion,
        simple_vol,
        time_to_terminal,
        order_intensity,
    )
    .expect("Failed to calculate quotes");

    println!("Using Simple Volatility ({:.2}%):", simple_vol * dec!(100));
    println!("  Bid: ${:.2}", bid);
    println!("  Ask: ${:.2}", ask);
    println!(
        "  Spread: ${:.4} ({:.3}%)",
        ask - bid,
        ((ask - bid) / mid_price) * dec!(100)
    );
    println!();

    // With EWMA volatility
    let (bid_ewma, ask_ewma) = calculate_optimal_quotes(
        mid_price,
        inventory,
        risk_aversion,
        ewma_vol,
        time_to_terminal,
        order_intensity,
    )
    .expect("Failed to calculate quotes");

    println!("Using EWMA Volatility ({:.2}%):", ewma_vol * dec!(100));
    println!("  Bid: ${:.2}", bid_ewma);
    println!("  Ask: ${:.2}", ask_ewma);
    println!(
        "  Spread: ${:.4} ({:.3}%)",
        ask_ewma - bid_ewma,
        ((ask_ewma - bid_ewma) / mid_price) * dec!(100)
    );
    println!();

    // === Custom Annualization Factor ===
    println!("=== Custom Annualization Factor (for hourly data) ===");

    // For hourly returns: sqrt(365 * 24) ≈ 93.91
    let hourly_estimator = VolatilityEstimator::with_annualization_factor(dec!(93.91));

    let hourly_vol = hourly_estimator
        .calculate_simple(&prices)
        .expect("Failed to calculate hourly volatility");

    println!(
        "Annualized Volatility (hourly data): {:.4} ({:.2}%)",
        hourly_vol,
        hourly_vol * dec!(100)
    );
    println!("Annualization Factor: sqrt(365*24) ≈ 93.91");
    println!();

    // === Key Insights ===
    println!("=== Key Insights ===");
    println!();
    println!("1. Simple Historical Volatility:");
    println!("   - Easy to calculate and understand");
    println!("   - Treats all observations equally");
    println!("   - Good for stable markets");
    println!();
    println!("2. EWMA Volatility:");
    println!("   - More responsive to recent changes");
    println!("   - Better for time-varying volatility");
    println!("   - Used by RiskMetrics (λ=0.94 for daily)");
    println!();
    println!("3. Parkinson's Estimator:");
    println!("   - Uses high-low range information");
    println!("   - More statistically efficient (30% more than close-to-close)");
    println!("   - Requires OHLC data");
    println!();
    println!("4. Impact on Market-Making:");
    println!("   - Higher volatility → Wider spreads");
    println!("   - Lower volatility → Tighter spreads");
    println!("   - Choose method based on market conditions");

    println!("\n✓ Volatility estimation example completed successfully!");
}
