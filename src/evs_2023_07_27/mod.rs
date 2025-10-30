//! Evs_2023_07_27 Service
//!
//! Auto-generated service module for evs_2023_07_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for evs_2023_07_27
pub struct Evs_2023_07_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Evs_2023_07_27Service<'a> {
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
