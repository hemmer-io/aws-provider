//! Serverlessapplicationrepository_2017_09_08 Service
//!
//! Auto-generated service module for serverlessapplicationrepository_2017_09_08

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for serverlessapplicationrepository_2017_09_08
pub struct Serverlessapplicationrepository_2017_09_08Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Serverlessapplicationrepository_2017_09_08Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get cloud_formation_template resource handler
    pub fn cloud_formation_template(&self) -> resources::Cloud_formation_template<'_> {
        resources::Cloud_formation_template::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get cloud_formation_change_set resource handler
    pub fn cloud_formation_change_set(&self) -> resources::Cloud_formation_change_set<'_> {
        resources::Cloud_formation_change_set::new(self.provider)
    }
    /// Get application_policy resource handler
    pub fn application_policy(&self) -> resources::Application_policy<'_> {
        resources::Application_policy::new(self.provider)
    }
    /// Get application_version resource handler
    pub fn application_version(&self) -> resources::Application_version<'_> {
        resources::Application_version::new(self.provider)
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
