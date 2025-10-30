//! Mgn_2020_02_26 Service
//!
//! Auto-generated service module for mgn_2020_02_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mgn_2020_02_26
pub struct Mgn_2020_02_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mgn_2020_02_26Service<'a> {
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
