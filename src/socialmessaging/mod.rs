//! Socialmessaging Service
//!
//! Auto-generated service module for socialmessaging

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for socialmessaging
pub struct SocialmessagingService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SocialmessagingService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get whats_app_message_template resource handler
    pub fn whats_app_message_template(&self) -> resources::Whats_app_message_template<'_> {
        resources::Whats_app_message_template::new(self.provider)
    }
    /// Get whats_app_message_template_from_library resource handler
    pub fn whats_app_message_template_from_library(&self) -> resources::Whats_app_message_template_from_library<'_> {
        resources::Whats_app_message_template_from_library::new(self.provider)
    }
    /// Get whats_app_message_template_media resource handler
    pub fn whats_app_message_template_media(&self) -> resources::Whats_app_message_template_media<'_> {
        resources::Whats_app_message_template_media::new(self.provider)
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
