//! Workdocs_2016_05_01 Service
//!
//! Auto-generated service module for workdocs_2016_05_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workdocs_2016_05_01
pub struct Workdocs_2016_05_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Workdocs_2016_05_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get notification_subscriptions resource handler
    pub fn notification_subscriptions(&self) -> resources::Notification_subscriptions<'_> {
        resources::Notification_subscriptions::new(self.provider)
    }
    /// Get labels resource handler
    pub fn labels(&self) -> resources::Labels<'_> {
        resources::Labels::new(self.provider)
    }
    /// Get resource_permissions resource handler
    pub fn resource_permissions(&self) -> resources::Resource_permissions<'_> {
        resources::Resource_permissions::new(self.provider)
    }
    /// Get folder_path resource handler
    pub fn folder_path(&self) -> resources::Folder_path<'_> {
        resources::Folder_path::new(self.provider)
    }
    /// Get activities resource handler
    pub fn activities(&self) -> resources::Activities<'_> {
        resources::Activities::new(self.provider)
    }
    /// Get comments resource handler
    pub fn comments(&self) -> resources::Comments<'_> {
        resources::Comments::new(self.provider)
    }
    /// Get root_folders resource handler
    pub fn root_folders(&self) -> resources::Root_folders<'_> {
        resources::Root_folders::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
    }
    /// Get document_version resource handler
    pub fn document_version(&self) -> resources::Document_version<'_> {
        resources::Document_version::new(self.provider)
    }
    /// Get folder_contents resource handler
    pub fn folder_contents(&self) -> resources::Folder_contents<'_> {
        resources::Folder_contents::new(self.provider)
    }
    /// Get groups resource handler
    pub fn groups(&self) -> resources::Groups<'_> {
        resources::Groups::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get notification_subscription resource handler
    pub fn notification_subscription(&self) -> resources::Notification_subscription<'_> {
        resources::Notification_subscription::new(self.provider)
    }
    /// Get document_path resource handler
    pub fn document_path(&self) -> resources::Document_path<'_> {
        resources::Document_path::new(self.provider)
    }
    /// Get resources resource handler
    pub fn resources(&self) -> resources::Resources<'_> {
        resources::Resources::new(self.provider)
    }
    /// Get document_versions resource handler
    pub fn document_versions(&self) -> resources::Document_versions<'_> {
        resources::Document_versions::new(self.provider)
    }
    /// Get custom_metadata resource handler
    pub fn custom_metadata(&self) -> resources::Custom_metadata<'_> {
        resources::Custom_metadata::new(self.provider)
    }
    /// Get users resource handler
    pub fn users(&self) -> resources::Users<'_> {
        resources::Users::new(self.provider)
    }
    /// Get current_user resource handler
    pub fn current_user(&self) -> resources::Current_user<'_> {
        resources::Current_user::new(self.provider)
    }
    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
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
