//! Mailmanager_2023_10_17 Service
//!
//! Auto-generated service module for mailmanager_2023_10_17

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mailmanager_2023_10_17
pub struct Mailmanager_2023_10_17Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mailmanager_2023_10_17Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get archive_export resource handler
    pub fn archive_export(&self) -> resources::Archive_export<'_> {
        resources::Archive_export::new(self.provider)
    }
    /// Get archive_message_content resource handler
    pub fn archive_message_content(&self) -> resources::Archive_message_content<'_> {
        resources::Archive_message_content::new(self.provider)
    }
    /// Get address_list_import_job resource handler
    pub fn address_list_import_job(&self) -> resources::Address_list_import_job<'_> {
        resources::Address_list_import_job::new(self.provider)
    }
    /// Get archive_message resource handler
    pub fn archive_message(&self) -> resources::Archive_message<'_> {
        resources::Archive_message::new(self.provider)
    }
    /// Get archive_search_results resource handler
    pub fn archive_search_results(&self) -> resources::Archive_search_results<'_> {
        resources::Archive_search_results::new(self.provider)
    }
    /// Get archive_search resource handler
    pub fn archive_search(&self) -> resources::Archive_search<'_> {
        resources::Archive_search::new(self.provider)
    }
    /// Get member_of_address_list resource handler
    pub fn member_of_address_list(&self) -> resources::Member_of_address_list<'_> {
        resources::Member_of_address_list::new(self.provider)
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
