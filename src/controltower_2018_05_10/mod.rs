//! Controltower_2018_05_10 Service
//!
//! Auto-generated service module for controltower_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for controltower_2018_05_10
pub struct Controltower_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Controltower_2018_05_10Service<'a> {
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
