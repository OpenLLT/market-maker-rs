//! Common types and error definitions for the market making library.
//!
//! This module contains:
//! - Error types using `thiserror`
//! - Type aliases for domain concepts
//! - Common enums and shared data structures

/// Error types for the market making library.
pub mod error;

pub mod decimal;
/// Common type aliases for prices, quantities, and time.
pub mod primitives;
