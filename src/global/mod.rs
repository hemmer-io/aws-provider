//! Global Service
//!
//! Auto-generated service module for global

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for global
pub struct GlobalService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> GlobalService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get custom_routing_accelerator_attributes resource handler
    pub fn custom_routing_accelerator_attributes(&self) -> resources::Custom_routing_accelerator_attributes<'_> {
        resources::Custom_routing_accelerator_attributes::new(self.provider)
    }
    /// Get custom_routing_listener resource handler
    pub fn custom_routing_listener(&self) -> resources::Custom_routing_listener<'_> {
        resources::Custom_routing_listener::new(self.provider)
    }
    /// Get endpoint_group resource handler
    pub fn endpoint_group(&self) -> resources::Endpoint_group<'_> {
        resources::Endpoint_group::new(self.provider)
    }
    /// Get listener resource handler
    pub fn listener(&self) -> resources::Listener<'_> {
        resources::Listener::new(self.provider)
    }
    /// Get custom_routing_endpoint_group resource handler
    pub fn custom_routing_endpoint_group(&self) -> resources::Custom_routing_endpoint_group<'_> {
        resources::Custom_routing_endpoint_group::new(self.provider)
    }
    /// Get accelerator resource handler
    pub fn accelerator(&self) -> resources::Accelerator<'_> {
        resources::Accelerator::new(self.provider)
    }
    /// Get cross_account_attachment resource handler
    pub fn cross_account_attachment(&self) -> resources::Cross_account_attachment<'_> {
        resources::Cross_account_attachment::new(self.provider)
    }
    /// Get accelerator_attributes resource handler
    pub fn accelerator_attributes(&self) -> resources::Accelerator_attributes<'_> {
        resources::Accelerator_attributes::new(self.provider)
    }
    /// Get custom_routing_accelerator resource handler
    pub fn custom_routing_accelerator(&self) -> resources::Custom_routing_accelerator<'_> {
        resources::Custom_routing_accelerator::new(self.provider)
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
