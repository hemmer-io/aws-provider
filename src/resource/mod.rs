//! Resource Service
//!
//! Auto-generated service module for resource

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for resource
pub struct ResourceService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ResourceService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get group_configuration resource handler
    pub fn group_configuration(&self) -> resources::Group_configuration<'_> {
        resources::Group_configuration::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get group_query resource handler
    pub fn group_query(&self) -> resources::Group_query<'_> {
        resources::Group_query::new(self.provider)
    }
    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
    }
    /// Get tag_sync_task resource handler
    pub fn tag_sync_task(&self) -> resources::Tag_sync_task<'_> {
        resources::Tag_sync_task::new(self.provider)
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
