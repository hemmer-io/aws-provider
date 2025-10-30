//! Firehose_2015_08_04 Service
//!
//! Auto-generated service module for firehose_2015_08_04

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firehose_2015_08_04
pub struct Firehose_2015_08_04Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Firehose_2015_08_04Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get delivery_stream resource handler
    pub fn delivery_stream(&self) -> resources::Delivery_stream<'_> {
        resources::Delivery_stream::new(self.provider)
    }
    /// Get record resource handler
    pub fn record(&self) -> resources::Record<'_> {
        resources::Record::new(self.provider)
    }
    /// Get record_batch resource handler
    pub fn record_batch(&self) -> resources::Record_batch<'_> {
        resources::Record_batch::new(self.provider)
    }
    /// Get destination resource handler
    pub fn destination(&self) -> resources::Destination<'_> {
        resources::Destination::new(self.provider)
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
