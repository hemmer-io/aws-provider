//! Kinesis_2013_12_02 Service
//!
//! Auto-generated service module for kinesis_2013_12_02

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kinesis_2013_12_02
pub struct Kinesis_2013_12_02Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Kinesis_2013_12_02Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get limits resource handler
    pub fn limits(&self) -> resources::Limits<'_> {
        resources::Limits::new(self.provider)
    }
    /// Get stream_summary resource handler
    pub fn stream_summary(&self) -> resources::Stream_summary<'_> {
        resources::Stream_summary::new(self.provider)
    }
    /// Get stream resource handler
    pub fn stream(&self) -> resources::Stream<'_> {
        resources::Stream::new(self.provider)
    }
    /// Get records resource handler
    pub fn records(&self) -> resources::Records<'_> {
        resources::Records::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get record resource handler
    pub fn record(&self) -> resources::Record<'_> {
        resources::Record::new(self.provider)
    }
    /// Get stream_consumer resource handler
    pub fn stream_consumer(&self) -> resources::Stream_consumer<'_> {
        resources::Stream_consumer::new(self.provider)
    }
    /// Get shard_iterator resource handler
    pub fn shard_iterator(&self) -> resources::Shard_iterator<'_> {
        resources::Shard_iterator::new(self.provider)
    }
    /// Get max_record_size resource handler
    pub fn max_record_size(&self) -> resources::Max_record_size<'_> {
        resources::Max_record_size::new(self.provider)
    }
    /// Get shard_count resource handler
    pub fn shard_count(&self) -> resources::Shard_count<'_> {
        resources::Shard_count::new(self.provider)
    }
    /// Get stream_mode resource handler
    pub fn stream_mode(&self) -> resources::Stream_mode<'_> {
        resources::Stream_mode::new(self.provider)
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
