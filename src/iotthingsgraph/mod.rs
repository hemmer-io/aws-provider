//! Iotthingsgraph Service
//!
//! Auto-generated service module for iotthingsgraph

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iotthingsgraph
pub struct IotthingsgraphService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IotthingsgraphService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get system_instance resource handler
    pub fn system_instance(&self) -> resources::System_instance<'_> {
        resources::System_instance::new(self.provider)
    }
    /// Get namespace_deletion_status resource handler
    pub fn namespace_deletion_status(&self) -> resources::Namespace_deletion_status<'_> {
        resources::Namespace_deletion_status::new(self.provider)
    }
    /// Get flow_template_revisions resource handler
    pub fn flow_template_revisions(&self) -> resources::Flow_template_revisions<'_> {
        resources::Flow_template_revisions::new(self.provider)
    }
    /// Get entities resource handler
    pub fn entities(&self) -> resources::Entities<'_> {
        resources::Entities::new(self.provider)
    }
    /// Get system_template resource handler
    pub fn system_template(&self) -> resources::System_template<'_> {
        resources::System_template::new(self.provider)
    }
    /// Get upload_status resource handler
    pub fn upload_status(&self) -> resources::Upload_status<'_> {
        resources::Upload_status::new(self.provider)
    }
    /// Get flow_template resource handler
    pub fn flow_template(&self) -> resources::Flow_template<'_> {
        resources::Flow_template::new(self.provider)
    }
    /// Get namespace resource handler
    pub fn namespace(&self) -> resources::Namespace<'_> {
        resources::Namespace::new(self.provider)
    }
    /// Get system_template_revisions resource handler
    pub fn system_template_revisions(&self) -> resources::System_template_revisions<'_> {
        resources::System_template_revisions::new(self.provider)
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
