//! Socialmessaging_2024_01_01 Service
//!
//! Auto-generated service module for socialmessaging_2024_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for socialmessaging_2024_01_01
pub struct Socialmessaging_2024_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Socialmessaging_2024_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get whats_app_message_template resource handler
    pub fn whats_app_message_template(&self) -> resources::Whats_app_message_template<'_> {
        resources::Whats_app_message_template::new(self.provider)
    }
    /// Get whats_app_message_template_media resource handler
    pub fn whats_app_message_template_media(&self) -> resources::Whats_app_message_template_media<'_> {
        resources::Whats_app_message_template_media::new(self.provider)
    }
    /// Get whats_app_message_template_from_library resource handler
    pub fn whats_app_message_template_from_library(&self) -> resources::Whats_app_message_template_from_library<'_> {
        resources::Whats_app_message_template_from_library::new(self.provider)
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
