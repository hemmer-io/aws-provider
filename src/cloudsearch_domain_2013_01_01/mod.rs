//! Cloudsearch_domain_2013_01_01 Service
//!
//! Auto-generated service module for cloudsearch_domain_2013_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudsearch_domain_2013_01_01
pub struct Cloudsearch_domain_2013_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudsearch_domain_2013_01_01Service<'a> {
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
