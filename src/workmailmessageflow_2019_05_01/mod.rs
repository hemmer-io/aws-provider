//! Workmailmessageflow_2019_05_01 Service
//!
//! Auto-generated service module for workmailmessageflow_2019_05_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workmailmessageflow_2019_05_01
pub struct Workmailmessageflow_2019_05_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Workmailmessageflow_2019_05_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get raw_message_content resource handler
    pub fn raw_message_content(&self) -> resources::Raw_message_content<'_> {
        resources::Raw_message_content::new(self.provider)
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
