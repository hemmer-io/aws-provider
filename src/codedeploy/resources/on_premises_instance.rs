//! On_premises_instance resource
//!
//! OnPremisesInstance resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// On_premises_instance resource handler
pub struct On_premises_instance<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> On_premises_instance<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a on_premises_instance
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.codedeploy_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_on_premises_instance_operations() {
        // Test on_premises_instance CRUD operations
    }
}
