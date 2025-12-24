//! Risk management module for position limits, exposure control, and circuit breakers.
//!
//! This module provides tools for managing risk in market making operations,
//! including position limits, notional exposure limits, order scaling, and
//! automatic trading halts via circuit breakers.
//!
//! # Overview
//!
//! Market makers must carefully manage their inventory and protect against
//! catastrophic losses. This module provides:
//!
//! - **Position Limits**: Maximum absolute position size (units)
//! - **Notional Limits**: Maximum exposure in currency terms
//! - **Order Scaling**: Automatic reduction of order sizes near limits
//! - **Circuit Breakers**: Automatic trading halts on adverse conditions
//!
//! # Example
//!
//! ```rust
//! use market_maker_rs::risk::{RiskLimits, CircuitBreaker, CircuitBreakerConfig};
//! use market_maker_rs::dec;
//!
//! // Position limits
//! let limits = RiskLimits::new(
//!     dec!(100.0),  // max 100 units position
//!     dec!(10000.0), // max $10,000 notional
//!     dec!(0.5),    // 50% scaling factor
//! ).unwrap();
//!
//! // Circuit breaker
//! let config = CircuitBreakerConfig::new(
//!     dec!(1000.0),  // max $1,000 daily loss
//!     dec!(0.05),    // max 5% volatility
//!     5,             // max 5 consecutive losses
//!     dec!(0.10),    // 10% rapid drawdown threshold
//!     300_000,       // 5 minute drawdown window
//!     60_000,        // 1 minute cooldown
//! ).unwrap();
//! let mut breaker = CircuitBreaker::new(config);
//!
//! assert!(breaker.is_trading_allowed());
//! ```

mod circuit_breaker;
mod limits;

pub use circuit_breaker::{
    CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState, TriggerReason,
};
pub use limits::RiskLimits;
