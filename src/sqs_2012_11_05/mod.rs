//! Sqs_2012_11_05 Service
//!
//! Auto-generated service module for sqs_2012_11_05

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sqs_2012_11_05
pub struct Sqs_2012_11_05Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sqs_2012_11_05Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get message_batch resource handler
    pub fn message_batch(&self) -> resources::Message_batch<'_> {
        resources::Message_batch::new(self.provider)
    }
    /// Get queue_url resource handler
    pub fn queue_url(&self) -> resources::Queue_url<'_> {
        resources::Queue_url::new(self.provider)
    }
    /// Get queue_attributes resource handler
    pub fn queue_attributes(&self) -> resources::Queue_attributes<'_> {
        resources::Queue_attributes::new(self.provider)
    }
    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
    }
    /// Get queue resource handler
    pub fn queue(&self) -> resources::Queue<'_> {
        resources::Queue::new(self.provider)
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
