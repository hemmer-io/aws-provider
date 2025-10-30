//! Quick_sight_q_search_configuration resource
//!
//! QuickSightQSearchConfiguration resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Quick_sight_q_search_configuration resource handler
pub struct Quick_sight_q_search_configuration<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Quick_sight_q_search_configuration<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a quick_sight_q_search_configuration
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }



    /// Update a quick_sight_q_search_configuration
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, q_search_status: Option<String>, aws_account_id: Option<String>) -> Result<()> {

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
    async fn test_quick_sight_q_search_configuration_operations() {
        // Test quick_sight_q_search_configuration CRUD operations
    }
}
