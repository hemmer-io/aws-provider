//! Max_record_size resource
//!
//! MaxRecordSize resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Max_record_size resource handler
pub struct Max_record_size<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Max_record_size<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }






    /// Update a max_record_size
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, max_record_size_in_ki_b: Option<i64>, stream_arn: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.kinesis_2013_12_02_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_max_record_size_operations() {
        // Test max_record_size CRUD operations
    }
}
