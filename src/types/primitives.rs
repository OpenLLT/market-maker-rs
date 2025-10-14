//! Primitive type aliases for market making domain concepts.

use rust_decimal::Decimal;

/// Price value in the market, represented as Decimal.
pub type Price = Decimal;

/// Quantity or size of an order/position, represented as Decimal.
///
/// Positive values indicate long positions, negative values indicate short positions.
pub type Quantity = Decimal;

/// Timestamp in nanoseconds since Unix epoch.
pub type Timestamp = u64;

/// Volatility value (annualized), represented as Decimal.
pub type Volatility = Decimal;

/// Risk aversion parameter (gamma), represented as Decimal.
pub type RiskAversion = Decimal;

/// Order intensity parameter (k), represented as Decimal.
pub type OrderIntensity = Decimal;
