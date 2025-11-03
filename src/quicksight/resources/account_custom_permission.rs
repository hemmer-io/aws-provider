//! Account_custom_permission resource
//!
//! AccountCustomPermission resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Account_custom_permission resource handler
pub struct Account_custom_permission<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Account_custom_permission<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a account_custom_permission
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_client;

        Ok(())

    }



    /// Update a account_custom_permission
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, custom_permissions_name: Option<String>, aws_account_id: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.quicksight_client;

        Ok(())

    }



    /// Delete a account_custom_permission
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_client;

        Ok(())

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_account_custom_permission_operations() {
        // Test account_custom_permission CRUD operations
    }
}
