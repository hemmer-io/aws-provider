//! Workmailmessageflow Service
//!
//! Auto-generated service module for workmailmessageflow

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workmailmessageflow
pub struct WorkmailmessageflowService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> WorkmailmessageflowService<'a> {
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
