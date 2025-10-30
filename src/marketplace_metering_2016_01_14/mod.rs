//! Marketplace_metering_2016_01_14 Service
//!
//! Auto-generated service module for marketplace_metering_2016_01_14

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for marketplace_metering_2016_01_14
pub struct Marketplace_metering_2016_01_14Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Marketplace_metering_2016_01_14Service<'a> {
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
