//! Snowball_2016_06_30 Service
//!
//! Auto-generated service module for snowball_2016_06_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for snowball_2016_06_30
pub struct Snowball_2016_06_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Snowball_2016_06_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get snowball_usage resource handler
    pub fn snowball_usage(&self) -> resources::Snowball_usage<'_> {
        resources::Snowball_usage::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get long_term_pricing resource handler
    pub fn long_term_pricing(&self) -> resources::Long_term_pricing<'_> {
        resources::Long_term_pricing::new(self.provider)
    }
    /// Get addresses resource handler
    pub fn addresses(&self) -> resources::Addresses<'_> {
        resources::Addresses::new(self.provider)
    }
    /// Get job_shipment_state resource handler
    pub fn job_shipment_state(&self) -> resources::Job_shipment_state<'_> {
        resources::Job_shipment_state::new(self.provider)
    }
    /// Get address resource handler
    pub fn address(&self) -> resources::Address<'_> {
        resources::Address::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get job_manifest resource handler
    pub fn job_manifest(&self) -> resources::Job_manifest<'_> {
        resources::Job_manifest::new(self.provider)
    }
    /// Get return_shipping_label resource handler
    pub fn return_shipping_label(&self) -> resources::Return_shipping_label<'_> {
        resources::Return_shipping_label::new(self.provider)
    }
    /// Get job_unlock_code resource handler
    pub fn job_unlock_code(&self) -> resources::Job_unlock_code<'_> {
        resources::Job_unlock_code::new(self.provider)
    }
    /// Get software_updates resource handler
    pub fn software_updates(&self) -> resources::Software_updates<'_> {
        resources::Software_updates::new(self.provider)
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
