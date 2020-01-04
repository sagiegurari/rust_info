//! # types
//!
//! Defines the various types and aliases.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

#[derive(Debug, Clone, PartialEq, Copy)]
/// Rust channel type
pub enum RustChannel {
    /// Rust stable channel
    Stable,
    /// Rust beta channel
    Beta,
    /// Rust nightly channel
    Nightly,
}

#[derive(Debug, Clone, PartialEq)]
/// Holds the current rust installation and setup information
pub struct RustInfo {
    /// version
    pub version: Option<String>,
    /// channel
    pub channel: Option<RustChannel>,
    /// target arch cfg value
    pub target_arch: Option<String>,
    /// target env cfg value
    pub target_env: Option<String>,
    /// target OS cfg value
    pub target_os: Option<String>,
    /// target pointer width cfg value
    pub target_pointer_width: Option<String>,
    /// target vendor cfg value
    pub target_vendor: Option<String>,
    /// target triple cfg value
    pub target_triple: Option<String>,
}

impl RustInfo {
    /// Returns new instance
    pub fn new() -> RustInfo {
        RustInfo {
            version: None,
            channel: None,
            target_arch: None,
            target_env: None,
            target_os: None,
            target_pointer_width: None,
            target_vendor: None,
            target_triple: None,
        }
    }
}
