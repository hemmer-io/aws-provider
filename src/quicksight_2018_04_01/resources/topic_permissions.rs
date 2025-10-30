//! Topic_permissions resource
//!
//! TopicPermissions resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Topic_permissions resource handler
pub struct Topic_permissions<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Topic_permissions<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a topic_permissions
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }



    /// Update a topic_permissions
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, aws_account_id: Option<String>, revoke_permissions: Option<Vec<String>>, grant_permissions: Option<Vec<String>>, topic_id: Option<String>) -> Result<()> {

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
    async fn test_topic_permissions_operations() {
        // Test topic_permissions CRUD operations
    }
}
