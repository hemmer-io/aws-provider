//! Mediastore_data_2017_09_01 Service
//!
//! Auto-generated service module for mediastore_data_2017_09_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediastore_data_2017_09_01
pub struct Mediastore_data_2017_09_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mediastore_data_2017_09_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get object resource handler
    pub fn object(&self) -> resources::Object<'_> {
        resources::Object::new(self.provider)
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
