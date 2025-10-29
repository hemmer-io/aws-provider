//! Time_series_data_points resource
//!
//! TimeSeriesDataPoints resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Time_series_data_points resource handler
pub struct Time_series_data_points<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Time_series_data_points<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }








    /// Delete a time_series_data_points
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.datazone_client;

        Ok(())

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_time_series_data_points_operations() {
        // Test time_series_data_points CRUD operations
    }
}
