//! Analysis_permissions resource
//!
//! AnalysisPermissions resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Analysis_permissions resource handler
pub struct Analysis_permissions<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Analysis_permissions<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a analysis_permissions
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }



    /// Update a analysis_permissions
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, aws_account_id: Option<String>, grant_permissions: Option<Vec<String>>, revoke_permissions: Option<Vec<String>>, analysis_id: Option<String>) -> Result<()> {

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
    async fn test_analysis_permissions_operations() {
        // Test analysis_permissions CRUD operations
    }
}
