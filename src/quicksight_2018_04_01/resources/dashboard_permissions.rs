//! Dashboard_permissions resource
//!
//! DashboardPermissions resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Dashboard_permissions resource handler
pub struct Dashboard_permissions<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Dashboard_permissions<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a dashboard_permissions
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }



    /// Update a dashboard_permissions
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, revoke_link_permissions: Option<Vec<String>>, grant_link_permissions: Option<Vec<String>>, dashboard_id: Option<String>, revoke_permissions: Option<Vec<String>>, aws_account_id: Option<String>, grant_permissions: Option<Vec<String>>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dashboard_permissions_operations() {
        // Test dashboard_permissions CRUD operations
    }
}
