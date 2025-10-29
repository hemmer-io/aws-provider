//! Evs Service
//!
//! Auto-generated service module for evs

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for evs
pub struct EvsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> EvsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
