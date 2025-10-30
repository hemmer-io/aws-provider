//! Cleanrooms_2022_02_17 Service
//!
//! Auto-generated service module for cleanrooms_2022_02_17

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cleanrooms_2022_02_17
pub struct Cleanrooms_2022_02_17Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cleanrooms_2022_02_17Service<'a> {
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
