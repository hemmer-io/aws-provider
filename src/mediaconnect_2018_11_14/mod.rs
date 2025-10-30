//! Mediaconnect_2018_11_14 Service
//!
//! Auto-generated service module for mediaconnect_2018_11_14

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediaconnect_2018_11_14
pub struct Mediaconnect_2018_11_14Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mediaconnect_2018_11_14Service<'a> {
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
