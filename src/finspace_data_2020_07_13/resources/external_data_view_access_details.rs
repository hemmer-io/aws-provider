//! External_data_view_access_details resource
//!
//! ExternalDataViewAccessDetails resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// External_data_view_access_details resource handler
pub struct External_data_view_access_details<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> External_data_view_access_details<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a external_data_view_access_details
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.finspace_data_2020_07_13_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_external_data_view_access_details_operations() {
        // Test external_data_view_access_details CRUD operations
    }
}
