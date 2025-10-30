//! Kinesis_streaming_destination resource
//!
//! KinesisStreamingDestination resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Kinesis_streaming_destination resource handler
pub struct Kinesis_streaming_destination<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Kinesis_streaming_destination<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a kinesis_streaming_destination
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.dynamodb_2012_08_10_client;

        Ok(())

    }



    /// Update a kinesis_streaming_destination
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, stream_arn: Option<String>, update_kinesis_streaming_configuration: Option<String>, table_name: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.dynamodb_2012_08_10_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kinesis_streaming_destination_operations() {
        // Test kinesis_streaming_destination CRUD operations
    }
}
