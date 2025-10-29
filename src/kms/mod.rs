//! Kms Service
//!
//! Auto-generated service module for kms

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kms
pub struct KmsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> KmsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get key_description resource handler
    pub fn key_description(&self) -> resources::Key_description<'_> {
        resources::Key_description::new(self.provider)
    }
    /// Get custom_key_store resource handler
    pub fn custom_key_store(&self) -> resources::Custom_key_store<'_> {
        resources::Custom_key_store::new(self.provider)
    }
    /// Get key_rotation_status resource handler
    pub fn key_rotation_status(&self) -> resources::Key_rotation_status<'_> {
        resources::Key_rotation_status::new(self.provider)
    }
    /// Get imported_key_material resource handler
    pub fn imported_key_material(&self) -> resources::Imported_key_material<'_> {
        resources::Imported_key_material::new(self.provider)
    }
    /// Get key_policy resource handler
    pub fn key_policy(&self) -> resources::Key_policy<'_> {
        resources::Key_policy::new(self.provider)
    }
    /// Get custom_key_stores resource handler
    pub fn custom_key_stores(&self) -> resources::Custom_key_stores<'_> {
        resources::Custom_key_stores::new(self.provider)
    }
    /// Get grant resource handler
    pub fn grant(&self) -> resources::Grant<'_> {
        resources::Grant::new(self.provider)
    }
    /// Get parameters_for_import resource handler
    pub fn parameters_for_import(&self) -> resources::Parameters_for_import<'_> {
        resources::Parameters_for_import::new(self.provider)
    }
    /// Get public_key resource handler
    pub fn public_key(&self) -> resources::Public_key<'_> {
        resources::Public_key::new(self.provider)
    }
    /// Get key resource handler
    pub fn key(&self) -> resources::Key<'_> {
        resources::Key::new(self.provider)
    }
    /// Get alias resource handler
    pub fn alias(&self) -> resources::Alias<'_> {
        resources::Alias::new(self.provider)
    }
    /// Get primary_region resource handler
    pub fn primary_region(&self) -> resources::Primary_region<'_> {
        resources::Primary_region::new(self.provider)
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
