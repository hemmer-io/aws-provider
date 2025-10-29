//! Apprunner Service
//!
//! Auto-generated service module for apprunner

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apprunner
pub struct ApprunnerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ApprunnerService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get default_auto_scaling_configuration resource handler
    pub fn default_auto_scaling_configuration(&self) -> resources::Default_auto_scaling_configuration<'_> {
        resources::Default_auto_scaling_configuration::new(self.provider)
    }
    /// Get custom_domains resource handler
    pub fn custom_domains(&self) -> resources::Custom_domains<'_> {
        resources::Custom_domains::new(self.provider)
    }
    /// Get auto_scaling_configuration resource handler
    pub fn auto_scaling_configuration(&self) -> resources::Auto_scaling_configuration<'_> {
        resources::Auto_scaling_configuration::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get observability_configuration resource handler
    pub fn observability_configuration(&self) -> resources::Observability_configuration<'_> {
        resources::Observability_configuration::new(self.provider)
    }
    /// Get vpc_connector resource handler
    pub fn vpc_connector(&self) -> resources::Vpc_connector<'_> {
        resources::Vpc_connector::new(self.provider)
    }
    /// Get vpc_ingress_connection resource handler
    pub fn vpc_ingress_connection(&self) -> resources::Vpc_ingress_connection<'_> {
        resources::Vpc_ingress_connection::new(self.provider)
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
