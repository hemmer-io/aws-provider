//! Rds_data_2018_08_01 Service
//!
//! Auto-generated service module for rds_data_2018_08_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rds_data_2018_08_01
pub struct Rds_data_2018_08_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Rds_data_2018_08_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
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
