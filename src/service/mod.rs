//! Service Service
//!
//! Auto-generated service module for service

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for service
pub struct ServiceService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ServiceService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get auto_management resource handler
    pub fn auto_management(&self) -> resources::Auto_management<'_> {
        resources::Auto_management::new(self.provider)
    }
    /// Get service_quota resource handler
    pub fn service_quota(&self) -> resources::Service_quota<'_> {
        resources::Service_quota::new(self.provider)
    }
    /// Get service_quota_increase_request_from_template resource handler
    pub fn service_quota_increase_request_from_template(&self) -> resources::Service_quota_increase_request_from_template<'_> {
        resources::Service_quota_increase_request_from_template::new(self.provider)
    }
    /// Get support_case resource handler
    pub fn support_case(&self) -> resources::Support_case<'_> {
        resources::Support_case::new(self.provider)
    }
    /// Get awsdefault_service_quota resource handler
    pub fn awsdefault_service_quota(&self) -> resources::Awsdefault_service_quota<'_> {
        resources::Awsdefault_service_quota::new(self.provider)
    }
    /// Get requested_service_quota_change resource handler
    pub fn requested_service_quota_change(&self) -> resources::Requested_service_quota_change<'_> {
        resources::Requested_service_quota_change::new(self.provider)
    }
    /// Get association_for_service_quota_template resource handler
    pub fn association_for_service_quota_template(&self) -> resources::Association_for_service_quota_template<'_> {
        resources::Association_for_service_quota_template::new(self.provider)
    }
    /// Get auto_management_configuration resource handler
    pub fn auto_management_configuration(&self) -> resources::Auto_management_configuration<'_> {
        resources::Auto_management_configuration::new(self.provider)
    }
    /// Get service_quota_increase_request_into_template resource handler
    pub fn service_quota_increase_request_into_template(&self) -> resources::Service_quota_increase_request_into_template<'_> {
        resources::Service_quota_increase_request_into_template::new(self.provider)
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
