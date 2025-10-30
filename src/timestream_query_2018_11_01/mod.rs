//! Timestream_query_2018_11_01 Service
//!
//! Auto-generated service module for timestream_query_2018_11_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for timestream_query_2018_11_01
pub struct Timestream_query_2018_11_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Timestream_query_2018_11_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get scheduled_query resource handler
    pub fn scheduled_query(&self) -> resources::Scheduled_query<'_> {
        resources::Scheduled_query::new(self.provider)
    }
    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
    }
    /// Get endpoints resource handler
    pub fn endpoints(&self) -> resources::Endpoints<'_> {
        resources::Endpoints::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
