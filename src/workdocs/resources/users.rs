//! Users resource
//!
//! Users resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Users resource handler
pub struct Users<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Users<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a users
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.workdocs_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_users_operations() {
        // Test users CRUD operations
    }
}
