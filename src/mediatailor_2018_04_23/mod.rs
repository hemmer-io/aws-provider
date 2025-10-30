//! Mediatailor_2018_04_23 Service
//!
//! Auto-generated service module for mediatailor_2018_04_23

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediatailor_2018_04_23
pub struct Mediatailor_2018_04_23Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mediatailor_2018_04_23Service<'a> {
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
