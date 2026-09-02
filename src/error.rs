// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

#![allow(dead_code)]

//! Panic-free error types for the crate, built on `core::error::Error` + `exn`.
//! All fallible operations return `exn::Result<T, E>` where `E` is a concrete error.
//! Callers add context via `or_raise` at module boundaries.

use core::fmt;

/// CRC width not supported (only 5, 16, 31, 32, 64 are currently accelerated).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedWidth(pub u8);

impl fmt::Display for UnsupportedWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported CRC width: {}", self.0)
    }
}

impl core::error::Error for UnsupportedWidth {}

/// Custom CRC requested but `alloc` feature is not enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocRequired;

impl fmt::Display for AllocRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "custom CRC parameters require the 'alloc' feature")
    }
}

impl core::error::Error for AllocRequired {}

/// Crate-internal invariant violated: generic width mismatch (compile-time guarantee failed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidWidthDispatch;

impl fmt::Display for InvalidWidthDispatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid width dispatch (internal invariant violated)")
    }
}

impl core::error::Error for InvalidWidthDispatch {}

/// CRC algorithm/data length error for combine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataTooLong;

impl fmt::Display for DataTooLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "data length too large for combine")
    }
}

impl core::error::Error for DataTooLong {}

/// Generic invalid CRC configuration (e.g., unsupported combination of params).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidCrcConfiguration;

impl fmt::Display for InvalidCrcConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported CRC configuration")
    }
}

impl core::error::Error for InvalidCrcConfiguration {}

/// Custom algorithm invoked without custom parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingCustomParams;

impl fmt::Display for MissingCustomParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "custom CRC requires parameters via CrcParams::new()")
    }
}

impl core::error::Error for MissingCustomParams {}

/// Failed to lock global cache (spin::Mutex never poisons, so this is used only for API symmetry).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockPoisoned;

impl fmt::Display for LockPoisoned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lock poisoned")
    }
}

impl core::error::Error for LockPoisoned {}
