//! Dashboard_links resource
//!
//! DashboardLinks resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Dashboard_links resource handler
pub struct Dashboard_links<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Dashboard_links<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }






    /// Update a dashboard_links
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, dashboard_id: Option<String>, aws_account_id: Option<String>, link_entities: Option<Vec<String>>) -> Result<()> {

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
    async fn test_dashboard_links_operations() {
        // Test dashboard_links CRUD operations
    }
}
