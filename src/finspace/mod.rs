//! Finspace Service
//!
//! Auto-generated service module for finspace

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for finspace
pub struct FinspaceService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> FinspaceService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get programmatic_access_credentials resource handler
    pub fn programmatic_access_credentials(&self) -> resources::Programmatic_access_credentials<'_> {
        resources::Programmatic_access_credentials::new(self.provider)
    }
    /// Get changeset resource handler
    pub fn changeset(&self) -> resources::Changeset<'_> {
        resources::Changeset::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get permission_group resource handler
    pub fn permission_group(&self) -> resources::Permission_group<'_> {
        resources::Permission_group::new(self.provider)
    }
    /// Get external_data_view_access_details resource handler
    pub fn external_data_view_access_details(&self) -> resources::External_data_view_access_details<'_> {
        resources::External_data_view_access_details::new(self.provider)
    }
    /// Get working_location resource handler
    pub fn working_location(&self) -> resources::Working_location<'_> {
        resources::Working_location::new(self.provider)
    }
    /// Get data_view resource handler
    pub fn data_view(&self) -> resources::Data_view<'_> {
        resources::Data_view::new(self.provider)
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
