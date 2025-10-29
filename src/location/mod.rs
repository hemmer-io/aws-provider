//! Location Service
//!
//! Auto-generated service module for location

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for location
pub struct LocationService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> LocationService<'a> {
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
