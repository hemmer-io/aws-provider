//! Backend_api_models resource
//!
//! BackendAPIModels resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Backend_api_models resource handler
pub struct Backend_api_models<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Backend_api_models<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a backend_api_models
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.amplifybackend_2020_08_11_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_backend_api_models_operations() {
        // Test backend_api_models CRUD operations
    }
}
