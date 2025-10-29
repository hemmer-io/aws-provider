//! Qapps Service
//!
//! Auto-generated service module for qapps

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for qapps
pub struct QappsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> QappsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get library_item_metadata resource handler
    pub fn library_item_metadata(&self) -> resources::Library_item_metadata<'_> {
        resources::Library_item_metadata::new(self.provider)
    }
    /// Get presigned_url resource handler
    pub fn presigned_url(&self) -> resources::Presigned_url<'_> {
        resources::Presigned_url::new(self.provider)
    }
    /// Get library_item resource handler
    pub fn library_item(&self) -> resources::Library_item<'_> {
        resources::Library_item::new(self.provider)
    }
    /// Get qapp_permissions resource handler
    pub fn qapp_permissions(&self) -> resources::Qapp_permissions<'_> {
        resources::Qapp_permissions::new(self.provider)
    }
    /// Get qapp resource handler
    pub fn qapp(&self) -> resources::Qapp<'_> {
        resources::Qapp::new(self.provider)
    }
    /// Get qapp_session resource handler
    pub fn qapp_session(&self) -> resources::Qapp_session<'_> {
        resources::Qapp_session::new(self.provider)
    }
    /// Get qapp_session_metadata resource handler
    pub fn qapp_session_metadata(&self) -> resources::Qapp_session_metadata<'_> {
        resources::Qapp_session_metadata::new(self.provider)
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
