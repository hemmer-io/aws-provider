//! Timestream Service
//!
//! Auto-generated service module for timestream

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for timestream
pub struct TimestreamService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TimestreamService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
    }
    /// Get scheduled_query resource handler
    pub fn scheduled_query(&self) -> resources::Scheduled_query<'_> {
        resources::Scheduled_query::new(self.provider)
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
