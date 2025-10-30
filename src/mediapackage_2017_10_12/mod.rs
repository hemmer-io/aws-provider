//! Mediapackage_2017_10_12 Service
//!
//! Auto-generated service module for mediapackage_2017_10_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediapackage_2017_10_12
pub struct Mediapackage_2017_10_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mediapackage_2017_10_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get origin_endpoint resource handler
    pub fn origin_endpoint(&self) -> resources::Origin_endpoint<'_> {
        resources::Origin_endpoint::new(self.provider)
    }
    /// Get harvest_job resource handler
    pub fn harvest_job(&self) -> resources::Harvest_job<'_> {
        resources::Harvest_job::new(self.provider)
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
