//! Qapps_2023_11_27 Service
//!
//! Auto-generated service module for qapps_2023_11_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for qapps_2023_11_27
pub struct Qapps_2023_11_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Qapps_2023_11_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get q_app_session resource handler
    pub fn q_app_session(&self) -> resources::Q_app_session<'_> {
        resources::Q_app_session::new(self.provider)
    }
    /// Get q_app_permissions resource handler
    pub fn q_app_permissions(&self) -> resources::Q_app_permissions<'_> {
        resources::Q_app_permissions::new(self.provider)
    }
    /// Get presigned_url resource handler
    pub fn presigned_url(&self) -> resources::Presigned_url<'_> {
        resources::Presigned_url::new(self.provider)
    }
    /// Get q_app resource handler
    pub fn q_app(&self) -> resources::Q_app<'_> {
        resources::Q_app::new(self.provider)
    }
    /// Get library_item_metadata resource handler
    pub fn library_item_metadata(&self) -> resources::Library_item_metadata<'_> {
        resources::Library_item_metadata::new(self.provider)
    }
    /// Get library_item resource handler
    pub fn library_item(&self) -> resources::Library_item<'_> {
        resources::Library_item::new(self.provider)
    }
    /// Get q_app_session_metadata resource handler
    pub fn q_app_session_metadata(&self) -> resources::Q_app_session_metadata<'_> {
        resources::Q_app_session_metadata::new(self.provider)
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
