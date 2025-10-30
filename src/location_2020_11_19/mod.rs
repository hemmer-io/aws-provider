//! Location_2020_11_19 Service
//!
//! Auto-generated service module for location_2020_11_19

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for location_2020_11_19
pub struct Location_2020_11_19Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Location_2020_11_19Service<'a> {
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
