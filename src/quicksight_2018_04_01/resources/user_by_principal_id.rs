//! User_by_principal_id resource
//!
//! UserByPrincipalId resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// User_by_principal_id resource handler
pub struct User_by_principal_id<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> User_by_principal_id<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }








    /// Delete a user_by_principal_id
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_by_principal_id_operations() {
        // Test user_by_principal_id CRUD operations
    }
}
