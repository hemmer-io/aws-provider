//! Chime Service
//!
//! Auto-generated service module for chime

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chime
pub struct ChimeService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ChimeService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get app_instance_user_endpoint resource handler
    pub fn app_instance_user_endpoint(&self) -> resources::App_instance_user_endpoint<'_> {
        resources::App_instance_user_endpoint::new(self.provider)
    }
    /// Get app_instance resource handler
    pub fn app_instance(&self) -> resources::App_instance<'_> {
        resources::App_instance::new(self.provider)
    }
    /// Get app_instance_bot resource handler
    pub fn app_instance_bot(&self) -> resources::App_instance_bot<'_> {
        resources::App_instance_bot::new(self.provider)
    }
    /// Get app_instance_retention_settings resource handler
    pub fn app_instance_retention_settings(&self) -> resources::App_instance_retention_settings<'_> {
        resources::App_instance_retention_settings::new(self.provider)
    }
    /// Get app_instance_admin resource handler
    pub fn app_instance_admin(&self) -> resources::App_instance_admin<'_> {
        resources::App_instance_admin::new(self.provider)
    }
    /// Get app_instance_user resource handler
    pub fn app_instance_user(&self) -> resources::App_instance_user<'_> {
        resources::App_instance_user::new(self.provider)
    }
    /// Get app_instance_user_expiration_settings resource handler
    pub fn app_instance_user_expiration_settings(&self) -> resources::App_instance_user_expiration_settings<'_> {
        resources::App_instance_user_expiration_settings::new(self.provider)
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
