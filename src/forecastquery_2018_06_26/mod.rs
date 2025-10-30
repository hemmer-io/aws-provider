//! Forecastquery_2018_06_26 Service
//!
//! Auto-generated service module for forecastquery_2018_06_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for forecastquery_2018_06_26
pub struct Forecastquery_2018_06_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Forecastquery_2018_06_26Service<'a> {
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
