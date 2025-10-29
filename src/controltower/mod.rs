//! Controltower Service
//!
//! Auto-generated service module for controltower

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for controltower
pub struct ControltowerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ControltowerService<'a> {
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
