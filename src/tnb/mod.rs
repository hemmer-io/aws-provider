//! Tnb Service
//!
//! Auto-generated service module for tnb

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for tnb
pub struct TnbService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TnbService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get sol_network_operation resource handler
    pub fn sol_network_operation(&self) -> resources::Sol_network_operation<'_> {
        resources::Sol_network_operation::new(self.provider)
    }
    /// Get sol_network_package_content resource handler
    pub fn sol_network_package_content(&self) -> resources::Sol_network_package_content<'_> {
        resources::Sol_network_package_content::new(self.provider)
    }
    /// Get sol_function_instance resource handler
    pub fn sol_function_instance(&self) -> resources::Sol_function_instance<'_> {
        resources::Sol_function_instance::new(self.provider)
    }
    /// Get sol_function_package_content resource handler
    pub fn sol_function_package_content(&self) -> resources::Sol_function_package_content<'_> {
        resources::Sol_function_package_content::new(self.provider)
    }
    /// Get sol_function_package resource handler
    pub fn sol_function_package(&self) -> resources::Sol_function_package<'_> {
        resources::Sol_function_package::new(self.provider)
    }
    /// Get sol_network_package resource handler
    pub fn sol_network_package(&self) -> resources::Sol_network_package<'_> {
        resources::Sol_network_package::new(self.provider)
    }
    /// Get sol_function_package_descriptor resource handler
    pub fn sol_function_package_descriptor(&self) -> resources::Sol_function_package_descriptor<'_> {
        resources::Sol_function_package_descriptor::new(self.provider)
    }
    /// Get sol_network_instance resource handler
    pub fn sol_network_instance(&self) -> resources::Sol_network_instance<'_> {
        resources::Sol_network_instance::new(self.provider)
    }
    /// Get sol_network_package_descriptor resource handler
    pub fn sol_network_package_descriptor(&self) -> resources::Sol_network_package_descriptor<'_> {
        resources::Sol_network_package_descriptor::new(self.provider)
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
