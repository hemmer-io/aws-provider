//! Workspaces_2015_04_08 Service
//!
//! Auto-generated service module for workspaces_2015_04_08

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workspaces_2015_04_08
pub struct Workspaces_2015_04_08Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Workspaces_2015_04_08Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get workspace_bundle resource handler
    pub fn workspace_bundle(&self) -> resources::Workspace_bundle<'_> {
        resources::Workspace_bundle::new(self.provider)
    }
    /// Get workspaces resource handler
    pub fn workspaces(&self) -> resources::Workspaces<'_> {
        resources::Workspaces::new(self.provider)
    }
    /// Get workspace_image_permissions resource handler
    pub fn workspace_image_permissions(&self) -> resources::Workspace_image_permissions<'_> {
        resources::Workspace_image_permissions::new(self.provider)
    }
    /// Get applications resource handler
    pub fn applications(&self) -> resources::Applications<'_> {
        resources::Applications::new(self.provider)
    }
    /// Get workspaces_connection_status resource handler
    pub fn workspaces_connection_status(&self) -> resources::Workspaces_connection_status<'_> {
        resources::Workspaces_connection_status::new(self.provider)
    }
    /// Get updated_workspace_image resource handler
    pub fn updated_workspace_image(&self) -> resources::Updated_workspace_image<'_> {
        resources::Updated_workspace_image::new(self.provider)
    }
    /// Get application_associations resource handler
    pub fn application_associations(&self) -> resources::Application_associations<'_> {
        resources::Application_associations::new(self.provider)
    }
    /// Get bundle_associations resource handler
    pub fn bundle_associations(&self) -> resources::Bundle_associations<'_> {
        resources::Bundle_associations::new(self.provider)
    }
    /// Get custom_workspace_image_import resource handler
    pub fn custom_workspace_image_import(&self) -> resources::Custom_workspace_image_import<'_> {
        resources::Custom_workspace_image_import::new(self.provider)
    }
    /// Get connect_client_add_in resource handler
    pub fn connect_client_add_in(&self) -> resources::Connect_client_add_in<'_> {
        resources::Connect_client_add_in::new(self.provider)
    }
    /// Get image_associations resource handler
    pub fn image_associations(&self) -> resources::Image_associations<'_> {
        resources::Image_associations::new(self.provider)
    }
    /// Get ip_group resource handler
    pub fn ip_group(&self) -> resources::Ip_group<'_> {
        resources::Ip_group::new(self.provider)
    }
    /// Get workspaces_pool_sessions resource handler
    pub fn workspaces_pool_sessions(&self) -> resources::Workspaces_pool_sessions<'_> {
        resources::Workspaces_pool_sessions::new(self.provider)
    }
    /// Get workspace_image_permission resource handler
    pub fn workspace_image_permission(&self) -> resources::Workspace_image_permission<'_> {
        resources::Workspace_image_permission::new(self.provider)
    }
    /// Get connection_alias resource handler
    pub fn connection_alias(&self) -> resources::Connection_alias<'_> {
        resources::Connection_alias::new(self.provider)
    }
    /// Get workspace_images resource handler
    pub fn workspace_images(&self) -> resources::Workspace_images<'_> {
        resources::Workspace_images::new(self.provider)
    }
    /// Get account_modifications resource handler
    pub fn account_modifications(&self) -> resources::Account_modifications<'_> {
        resources::Account_modifications::new(self.provider)
    }
    /// Get account_link_invitation resource handler
    pub fn account_link_invitation(&self) -> resources::Account_link_invitation<'_> {
        resources::Account_link_invitation::new(self.provider)
    }
    /// Get standby_workspaces resource handler
    pub fn standby_workspaces(&self) -> resources::Standby_workspaces<'_> {
        resources::Standby_workspaces::new(self.provider)
    }
    /// Get workspace_bundles resource handler
    pub fn workspace_bundles(&self) -> resources::Workspace_bundles<'_> {
        resources::Workspace_bundles::new(self.provider)
    }
    /// Get connection_alias_permissions resource handler
    pub fn connection_alias_permissions(&self) -> resources::Connection_alias_permissions<'_> {
        resources::Connection_alias_permissions::new(self.provider)
    }
    /// Get client_branding resource handler
    pub fn client_branding(&self) -> resources::Client_branding<'_> {
        resources::Client_branding::new(self.provider)
    }
    /// Get workspace_associations resource handler
    pub fn workspace_associations(&self) -> resources::Workspace_associations<'_> {
        resources::Workspace_associations::new(self.provider)
    }
    /// Get workspace_snapshots resource handler
    pub fn workspace_snapshots(&self) -> resources::Workspace_snapshots<'_> {
        resources::Workspace_snapshots::new(self.provider)
    }
    /// Get ip_groups resource handler
    pub fn ip_groups(&self) -> resources::Ip_groups<'_> {
        resources::Ip_groups::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get connect_client_add_ins resource handler
    pub fn connect_client_add_ins(&self) -> resources::Connect_client_add_ins<'_> {
        resources::Connect_client_add_ins::new(self.provider)
    }
    /// Get connection_alias_permission resource handler
    pub fn connection_alias_permission(&self) -> resources::Connection_alias_permission<'_> {
        resources::Connection_alias_permission::new(self.provider)
    }
    /// Get connection_aliases resource handler
    pub fn connection_aliases(&self) -> resources::Connection_aliases<'_> {
        resources::Connection_aliases::new(self.provider)
    }
    /// Get workspace_directories resource handler
    pub fn workspace_directories(&self) -> resources::Workspace_directories<'_> {
        resources::Workspace_directories::new(self.provider)
    }
    /// Get client_properties resource handler
    pub fn client_properties(&self) -> resources::Client_properties<'_> {
        resources::Client_properties::new(self.provider)
    }
    /// Get workspaces_pool resource handler
    pub fn workspaces_pool(&self) -> resources::Workspaces_pool<'_> {
        resources::Workspaces_pool::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get account_link resource handler
    pub fn account_link(&self) -> resources::Account_link<'_> {
        resources::Account_link::new(self.provider)
    }
    /// Get workspaces_pools resource handler
    pub fn workspaces_pools(&self) -> resources::Workspaces_pools<'_> {
        resources::Workspaces_pools::new(self.provider)
    }
    /// Get rules_of_ip_group resource handler
    pub fn rules_of_ip_group(&self) -> resources::Rules_of_ip_group<'_> {
        resources::Rules_of_ip_group::new(self.provider)
    }
    /// Get workspace_image resource handler
    pub fn workspace_image(&self) -> resources::Workspace_image<'_> {
        resources::Workspace_image::new(self.provider)
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
