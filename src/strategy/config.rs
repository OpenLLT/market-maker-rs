//! Strategy configuration parameters.

use crate::Decimal;
use crate::types::error::{MMError, MMResult};

#[cfg(feature = "serde")]
use pretty_simple_display::{DebugPretty, DisplaySimple};

/// Configuration parameters for the Avellaneda-Stoikov strategy.
#[derive(Clone, PartialEq)]
#[cfg_attr(not(feature = "serde"), derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, DebugPretty, DisplaySimple)
)]
pub struct StrategyConfig {
    /// Risk aversion parameter (gamma).
    ///
    /// Higher values make the strategy more conservative.
    /// Must be positive.
    pub risk_aversion: Decimal,

    /// Order intensity parameter (k).
    ///
    /// Models how frequently market orders arrive.
    /// Must be positive.
    pub order_intensity: Decimal,

    /// Terminal time (end of trading session) in milliseconds since Unix epoch.
    pub terminal_time: u64,

    /// Minimum spread constraint, in price units.
    ///
    /// Ensures quotes don't cross or get too tight.
    /// Must be non-negative.
    pub min_spread: Decimal,
}

impl StrategyConfig {
    /// Creates a new strategy configuration with validation.
    ///
    /// # Arguments
    ///
    /// * `risk_aversion` - Risk aversion parameter (gamma), must be positive
    /// * `order_intensity` - Order intensity parameter (k), must be positive
    /// * `terminal_time` - Terminal time in milliseconds
    /// * `min_spread` - Minimum spread, must be non-negative
    ///
    /// # Errors
    ///
    /// Returns `MMError::InvalidConfiguration` if parameters are invalid.
    pub fn new(
        risk_aversion: Decimal,
        order_intensity: Decimal,
        terminal_time: u64,
        min_spread: Decimal,
    ) -> MMResult<Self> {
        if risk_aversion <= Decimal::ZERO {
            return Err(MMError::InvalidConfiguration(
                "risk_aversion must be positive".to_string(),
            ));
        }

        if order_intensity <= Decimal::ZERO {
            return Err(MMError::InvalidConfiguration(
                "order_intensity must be positive".to_string(),
            ));
        }

        if min_spread < Decimal::ZERO {
            return Err(MMError::InvalidConfiguration(
                "min_spread must be non-negative".to_string(),
            ));
        }

        Ok(Self {
            risk_aversion,
            order_intensity,
            terminal_time,
            min_spread,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dec;

    #[test]
    fn test_valid_config() {
        let config = StrategyConfig::new(dec!(0.5), dec!(1.5), 1000, dec!(0.01));
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.risk_aversion, dec!(0.5));
        assert_eq!(config.order_intensity, dec!(1.5));
        assert_eq!(config.terminal_time, 1000);
        assert_eq!(config.min_spread, dec!(0.01));
    }

    #[test]
    fn test_invalid_risk_aversion_zero() {
        let config = StrategyConfig::new(Decimal::ZERO, dec!(1.5), 1000, dec!(0.01));
        assert!(config.is_err());
        assert!(matches!(
            config.unwrap_err(),
            MMError::InvalidConfiguration(_)
        ));
    }

    #[test]
    fn test_invalid_risk_aversion_negative() {
        let config = StrategyConfig::new(dec!(-0.5), dec!(1.5), 1000, dec!(0.01));
        assert!(config.is_err());
        if let Err(MMError::InvalidConfiguration(msg)) = config {
            assert!(msg.contains("risk_aversion must be positive"));
        }
    }

    #[test]
    fn test_invalid_order_intensity_zero() {
        let config = StrategyConfig::new(dec!(0.5), Decimal::ZERO, 1000, dec!(0.01));
        assert!(config.is_err());
        assert!(matches!(
            config.unwrap_err(),
            MMError::InvalidConfiguration(_)
        ));
    }

    #[test]
    fn test_invalid_order_intensity_negative() {
        let config = StrategyConfig::new(dec!(0.5), dec!(-1.5), 1000, dec!(0.01));
        assert!(config.is_err());
        if let Err(MMError::InvalidConfiguration(msg)) = config {
            assert!(msg.contains("order_intensity must be positive"));
        }
    }

    #[test]
    fn test_invalid_min_spread_negative() {
        let config = StrategyConfig::new(dec!(0.5), dec!(1.5), 1000, dec!(-0.01));
        assert!(config.is_err());
        if let Err(MMError::InvalidConfiguration(msg)) = config {
            assert!(msg.contains("min_spread must be non-negative"));
        }
    }

    #[test]
    fn test_valid_min_spread_zero() {
        let config = StrategyConfig::new(dec!(0.5), dec!(1.5), 1000, Decimal::ZERO);
        assert!(config.is_ok());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_config_display() {
        let config = StrategyConfig::new(dec!(0.5), dec!(1.5), 1000, dec!(0.01)).unwrap();
        let display_str = format!("{}", config);
        assert!(display_str.contains("risk_aversion"));
        assert!(display_str.contains("0.5"));
        assert!(display_str.contains("order_intensity"));
        assert!(display_str.contains("1.5"));
    }
}
