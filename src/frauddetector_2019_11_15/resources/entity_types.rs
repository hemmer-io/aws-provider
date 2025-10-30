//! Entity_types resource
//!
//! EntityTypes resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Entity_types resource handler
pub struct Entity_types<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Entity_types<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a entity_types
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.frauddetector_2019_11_15_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_entity_types_operations() {
        // Test entity_types CRUD operations
    }
}
