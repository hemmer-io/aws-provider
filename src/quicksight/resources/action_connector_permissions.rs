//! Action_connector_permissions resource
//!
//! ActionConnectorPermissions resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Action_connector_permissions resource handler
pub struct Action_connector_permissions<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Action_connector_permissions<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a action_connector_permissions
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_client;

        Ok(())

    }



    /// Update a action_connector_permissions
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, revoke_permissions: Option<Vec<String>>, action_connector_id: Option<String>, grant_permissions: Option<Vec<String>>, aws_account_id: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.quicksight_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_action_connector_permissions_operations() {
        // Test action_connector_permissions CRUD operations
    }
}
