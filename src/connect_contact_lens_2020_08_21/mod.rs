//! Connect_contact_lens_2020_08_21 Service
//!
//! Auto-generated service module for connect_contact_lens_2020_08_21

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for connect_contact_lens_2020_08_21
pub struct Connect_contact_lens_2020_08_21Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Connect_contact_lens_2020_08_21Service<'a> {
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
