//! Panorama_2019_07_24 Service
//!
//! Auto-generated service module for panorama_2019_07_24

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for panorama_2019_07_24
pub struct Panorama_2019_07_24Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Panorama_2019_07_24Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get job_for_devices resource handler
    pub fn job_for_devices(&self) -> resources::Job_for_devices<'_> {
        resources::Job_for_devices::new(self.provider)
    }
    /// Get application_instance resource handler
    pub fn application_instance(&self) -> resources::Application_instance<'_> {
        resources::Application_instance::new(self.provider)
    }
    /// Get package_import_job resource handler
    pub fn package_import_job(&self) -> resources::Package_import_job<'_> {
        resources::Package_import_job::new(self.provider)
    }
    /// Get device_job resource handler
    pub fn device_job(&self) -> resources::Device_job<'_> {
        resources::Device_job::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
    }
    /// Get device_metadata resource handler
    pub fn device_metadata(&self) -> resources::Device_metadata<'_> {
        resources::Device_metadata::new(self.provider)
    }
    /// Get package resource handler
    pub fn package(&self) -> resources::Package<'_> {
        resources::Package::new(self.provider)
    }
    /// Get package_version resource handler
    pub fn package_version(&self) -> resources::Package_version<'_> {
        resources::Package_version::new(self.provider)
    }
    /// Get application_instance_details resource handler
    pub fn application_instance_details(&self) -> resources::Application_instance_details<'_> {
        resources::Application_instance_details::new(self.provider)
    }
    /// Get node_from_template_job resource handler
    pub fn node_from_template_job(&self) -> resources::Node_from_template_job<'_> {
        resources::Node_from_template_job::new(self.provider)
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
