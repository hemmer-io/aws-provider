//! Mq_2017_11_27 Service
//!
//! Auto-generated service module for mq_2017_11_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mq_2017_11_27
pub struct Mq_2017_11_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mq_2017_11_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get configuration resource handler
    pub fn configuration(&self) -> resources::Configuration<'_> {
        resources::Configuration::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get broker resource handler
    pub fn broker(&self) -> resources::Broker<'_> {
        resources::Broker::new(self.provider)
    }
    /// Get configuration_revision resource handler
    pub fn configuration_revision(&self) -> resources::Configuration_revision<'_> {
        resources::Configuration_revision::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get broker_instance_options resource handler
    pub fn broker_instance_options(&self) -> resources::Broker_instance_options<'_> {
        resources::Broker_instance_options::new(self.provider)
    }
    /// Get broker_engine_types resource handler
    pub fn broker_engine_types(&self) -> resources::Broker_engine_types<'_> {
        resources::Broker_engine_types::new(self.provider)
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
