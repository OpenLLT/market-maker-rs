//! Strategy interface traits.
//!
//! This module defines the traits for implementing market-making strategies.
//! Currently supports the Avellaneda-Stoikov model in both synchronous and asynchronous forms.
//!
//! # Examples
//!
//! Using the default synchronous implementation:
//!
//! ```
//! use market_maker_rs::strategy::interface::{AvellanedaStoikov, DefaultAvellanedaStoikov};
//! use market_maker_rs::dec;
//!
//! let strategy = DefaultAvellanedaStoikov;
//! let (bid, ask) = strategy.calculate_optimal_quotes(
//!     dec!(100.0),
//!     dec!(0.0),
//!     dec!(0.1),
//!     dec!(0.2),
//!     3600000,
//!     dec!(1.5),
//! ).unwrap();
//!
//! assert!(bid < ask);
//! ```

use crate::Decimal;
use crate::types::error::MMResult;
use async_trait::async_trait;

/// Trait for implementing the Avellaneda-Stoikov market-making strategy (synchronous).
///
/// This trait defines the core operations needed for a market-making strategy based on the
/// Avellaneda-Stoikov model. Implementors must provide methods for calculating:
/// - Reservation price (adjusted mid-price based on inventory risk)
/// - Optimal spread (based on volatility and order intensity)
/// - Optimal quotes (bid/ask prices)
///
/// # Examples
///
/// ```
/// use market_maker_rs::strategy::interface::AvellanedaStoikov;
/// use market_maker_rs::{Decimal, dec};
/// use market_maker_rs::types::error::MMResult;
///
/// struct MyStrategy;
///
/// impl AvellanedaStoikov for MyStrategy {
///     fn calculate_reservation_price(
///         &self,
///         mid_price: Decimal,
///         inventory: Decimal,
///         risk_aversion: Decimal,
///         volatility: Decimal,
///         time_to_terminal_ms: u64,
///     ) -> MMResult<Decimal> {
///         // Implementation details
///         Ok(mid_price)
///     }
///
///     fn calculate_optimal_spread(
///         &self,
///         risk_aversion: Decimal,
///         volatility: Decimal,
///         time_to_terminal_ms: u64,
///         order_intensity: Decimal,
///     ) -> MMResult<Decimal> {
///         // Implementation details
///         Ok(dec!(0.1))
///     }
///
///     fn calculate_optimal_quotes(
///         &self,
///         mid_price: Decimal,
///         inventory: Decimal,
///         risk_aversion: Decimal,
///         volatility: Decimal,
///         time_to_terminal_ms: u64,
///         order_intensity: Decimal,
///     ) -> MMResult<(Decimal, Decimal)> {
///         // Implementation details
///         Ok((dec!(99.9), dec!(100.1)))
///     }
/// }
/// ```
pub trait AvellanedaStoikov {
    /// Calculates the reservation price based on current market conditions and inventory.
    ///
    /// The reservation price represents the "fair value" adjusted for inventory risk.
    /// A market maker with positive inventory (long) will have a reservation price
    /// below the mid-price, incentivizing sells.
    ///
    /// # Arguments
    ///
    /// * `mid_price` - Current mid-price of the asset
    /// * `inventory` - Current inventory position (positive = long, negative = short)
    /// * `risk_aversion` - Risk aversion parameter (gamma), must be positive
    /// * `volatility` - Volatility estimate (annualized), must be positive
    /// * `time_to_terminal_ms` - Time remaining until terminal time, in milliseconds
    ///
    /// # Returns
    ///
    /// The reservation price adjusted for inventory risk.
    ///
    /// # Errors
    ///
    /// Returns an error if parameters are invalid or calculation fails.
    fn calculate_reservation_price(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
    ) -> MMResult<Decimal>;

    /// Calculates the optimal spread based on market conditions and strategy parameters.
    ///
    /// The spread accounts for both inventory risk and adverse selection risk.
    ///
    /// # Arguments
    ///
    /// * `risk_aversion` - Risk aversion parameter (gamma), must be positive
    /// * `volatility` - Volatility estimate (annualized), must be positive
    /// * `time_to_terminal_ms` - Time remaining until terminal time, in milliseconds
    /// * `order_intensity` - Order intensity parameter (k), must be positive
    ///
    /// # Returns
    ///
    /// The optimal spread in price units.
    ///
    /// # Errors
    ///
    /// Returns an error if parameters are invalid or calculation fails.
    fn calculate_optimal_spread(
        &self,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<Decimal>;

    /// Calculates optimal bid and ask prices based on the Avellaneda-Stoikov model.
    ///
    /// This combines reservation price and spread calculations to produce bid/ask quotes.
    ///
    /// # Arguments
    ///
    /// * `mid_price` - Current mid-price
    /// * `inventory` - Current inventory position
    /// * `risk_aversion` - Risk aversion parameter (gamma)
    /// * `volatility` - Volatility estimate (annualized)
    /// * `time_to_terminal_ms` - Time to terminal in milliseconds
    /// * `order_intensity` - Order intensity parameter (k)
    ///
    /// # Returns
    ///
    /// A tuple `(bid_price, ask_price)`.
    ///
    /// # Errors
    ///
    /// Returns errors from underlying calculations if inputs are invalid.
    fn calculate_optimal_quotes(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<(Decimal, Decimal)>;
}

/// Trait for implementing the Avellaneda-Stoikov strategy with async operations.
///
/// This trait mirrors `AvellanedaStoikov` but with asynchronous method signatures,
/// allowing for non-blocking calculations that might involve I/O operations, network calls,
/// or expensive computations that should be run on a separate executor.
///
/// # Note
///
/// This trait is currently a placeholder for future async support.
/// To use it, you'll need to add an `async` feature to `Cargo.toml`.
///
/// # Examples
///
/// ```ignore
/// use market_maker_rs::strategy::interface::AsyncAvellanedaStoikov;
/// use market_maker_rs::{Decimal, dec};
/// use market_maker_rs::types::error::MMResult;
///
/// struct MyAsyncStrategy;
///
/// impl AsyncAvellanedaStoikov for MyAsyncStrategy {
///     async fn calculate_reservation_price(
///         &self,
///         mid_price: Decimal,
///         inventory: Decimal,
///         risk_aversion: Decimal,
///         volatility: Decimal,
///         time_to_terminal_ms: u64,
///     ) -> MMResult<Decimal> {
///         // Async implementation
///         Ok(mid_price)
///     }
///
///     // ... other methods
/// }
/// ```
#[async_trait]
pub trait AsyncAvellanedaStoikov {
    /// Asynchronously calculates the reservation price.
    ///
    /// See `AvellanedaStoikov::calculate_reservation_price` for details.
    async fn calculate_reservation_price(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
    ) -> MMResult<Decimal>;

    /// Asynchronously calculates the optimal spread.
    ///
    /// See `AvellanedaStoikov::calculate_optimal_spread` for details.
    async fn calculate_optimal_spread(
        &self,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<Decimal>;

    /// Asynchronously calculates optimal bid and ask prices.
    ///
    /// See `AvellanedaStoikov::calculate_optimal_quotes` for details.
    async fn calculate_optimal_quotes(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<(Decimal, Decimal)>;
}

/// Default implementation of the Avellaneda-Stoikov strategy.
///
/// This zero-sized struct provides a concrete implementation of the `AvellanedaStoikov` trait
/// using the module-level functions from `avellaneda_stoikov`.
///
/// # Examples
///
/// ```
/// use market_maker_rs::strategy::interface::{AvellanedaStoikov, DefaultAvellanedaStoikov};
/// use market_maker_rs::dec;
///
/// let strategy = DefaultAvellanedaStoikov;
/// let (bid, ask) = strategy.calculate_optimal_quotes(
///     dec!(100.0),
///     dec!(0.0),
///     dec!(0.1),
///     dec!(0.2),
///     3600000,
///     dec!(1.5),
/// ).unwrap();
///
/// assert!(bid < ask);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultAvellanedaStoikov;

impl AvellanedaStoikov for DefaultAvellanedaStoikov {
    fn calculate_reservation_price(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
    ) -> MMResult<Decimal> {
        crate::strategy::avellaneda_stoikov::calculate_reservation_price(
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
        crate::strategy::avellaneda_stoikov::calculate_optimal_spread(
            risk_aversion,
            volatility,
            time_to_terminal_ms,
            order_intensity,
        )
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
        crate::strategy::avellaneda_stoikov::calculate_optimal_quotes(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal_ms,
            order_intensity,
        )
    }
}

/// Default async implementation of the Avellaneda-Stoikov strategy.
///
/// This provides an async wrapper around the synchronous implementation,
/// allowing it to be used in async contexts.
///
/// # Examples
///
/// ```ignore
/// use market_maker_rs::strategy::interface::{AsyncAvellanedaStoikov, DefaultAvellanedaStoikov};
/// use market_maker_rs::dec;
///
/// async fn example() {
///     let strategy = DefaultAvellanedaStoikov;
///     let (bid, ask) = strategy.calculate_optimal_quotes(
///         dec!(100.0),
///         dec!(0.0),
///         dec!(0.1),
///         dec!(0.2),
///         3600000,
///         dec!(1.5),
///     ).await.unwrap();
///
///     assert!(bid < ask);
/// }
/// ```
#[async_trait]
impl AsyncAvellanedaStoikov for DefaultAvellanedaStoikov {
    async fn calculate_reservation_price(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
    ) -> MMResult<Decimal> {
        crate::strategy::avellaneda_stoikov::calculate_reservation_price(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal_ms,
        )
    }

    async fn calculate_optimal_spread(
        &self,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<Decimal> {
        crate::strategy::avellaneda_stoikov::calculate_optimal_spread(
            risk_aversion,
            volatility,
            time_to_terminal_ms,
            order_intensity,
        )
    }

    async fn calculate_optimal_quotes(
        &self,
        mid_price: Decimal,
        inventory: Decimal,
        risk_aversion: Decimal,
        volatility: Decimal,
        time_to_terminal_ms: u64,
        order_intensity: Decimal,
    ) -> MMResult<(Decimal, Decimal)> {
        crate::strategy::avellaneda_stoikov::calculate_optimal_quotes(
            mid_price,
            inventory,
            risk_aversion,
            volatility,
            time_to_terminal_ms,
            order_intensity,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dec;

    #[test]
    fn test_default_avellaneda_stoikov_sync() {
        let strategy = DefaultAvellanedaStoikov;

        // Usando el trait sync explícitamente
        let (bid, ask) = <DefaultAvellanedaStoikov as AvellanedaStoikov>::calculate_optimal_quotes(
            &strategy,
            dec!(100.0),
            dec!(0.0),
            dec!(0.1),
            dec!(0.2),
            3600000,
            dec!(1.5),
        )
        .unwrap();

        assert!(bid < ask);
        assert!(bid < dec!(100.0));
        assert!(ask > dec!(100.0));
    }

    #[test]
    fn test_default_reservation_price_sync() {
        let strategy = DefaultAvellanedaStoikov;

        let reservation =
            <DefaultAvellanedaStoikov as AvellanedaStoikov>::calculate_reservation_price(
                &strategy,
                dec!(100.0),
                dec!(10.0),
                dec!(0.1),
                dec!(0.2),
                3600000,
            )
            .unwrap();

        assert!(reservation < dec!(100.0)); // Long inventory pushes reservation down
    }

    #[test]
    fn test_default_optimal_spread_sync() {
        let strategy = DefaultAvellanedaStoikov;

        let spread = <DefaultAvellanedaStoikov as AvellanedaStoikov>::calculate_optimal_spread(
            &strategy,
            dec!(0.1),
            dec!(0.2),
            3600000,
            dec!(1.5),
        )
        .unwrap();

        assert!(spread > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_default_avellaneda_stoikov_async() {
        let strategy = DefaultAvellanedaStoikov;

        // Usando el trait async explícitamente
        let (bid, ask) =
            <DefaultAvellanedaStoikov as AsyncAvellanedaStoikov>::calculate_optimal_quotes(
                &strategy,
                dec!(100.0),
                dec!(0.0),
                dec!(0.1),
                dec!(0.2),
                3600000,
                dec!(1.5),
            )
            .await
            .unwrap();

        assert!(bid < ask);
        assert!(bid < dec!(100.0));
        assert!(ask > dec!(100.0));
    }
}
