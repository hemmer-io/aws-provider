//! Dataexchange Service
//!
//! Auto-generated service module for dataexchange

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dataexchange
pub struct DataexchangeService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DataexchangeService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get asset resource handler
    pub fn asset(&self) -> resources::Asset<'_> {
        resources::Asset::new(self.provider)
    }
    /// Get data_grant resource handler
    pub fn data_grant(&self) -> resources::Data_grant<'_> {
        resources::Data_grant::new(self.provider)
    }
    /// Get received_data_grant resource handler
    pub fn received_data_grant(&self) -> resources::Received_data_grant<'_> {
        resources::Received_data_grant::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get event_action resource handler
    pub fn event_action(&self) -> resources::Event_action<'_> {
        resources::Event_action::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
    }
    /// Get data_set resource handler
    pub fn data_set(&self) -> resources::Data_set<'_> {
        resources::Data_set::new(self.provider)
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
