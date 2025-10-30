//! Qbusiness_2023_11_27 Service
//!
//! Auto-generated service module for qbusiness_2023_11_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for qbusiness_2023_11_27
pub struct Qbusiness_2023_11_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Qbusiness_2023_11_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get feedback resource handler
    pub fn feedback(&self) -> resources::Feedback<'_> {
        resources::Feedback::new(self.provider)
    }
    /// Get conversation resource handler
    pub fn conversation(&self) -> resources::Conversation<'_> {
        resources::Conversation::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get document_content resource handler
    pub fn document_content(&self) -> resources::Document_content<'_> {
        resources::Document_content::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get chat_response_configuration resource handler
    pub fn chat_response_configuration(&self) -> resources::Chat_response_configuration<'_> {
        resources::Chat_response_configuration::new(self.provider)
    }
    /// Get anonymous_web_experience_url resource handler
    pub fn anonymous_web_experience_url(&self) -> resources::Anonymous_web_experience_url<'_> {
        resources::Anonymous_web_experience_url::new(self.provider)
    }
    /// Get chat_controls_configuration resource handler
    pub fn chat_controls_configuration(&self) -> resources::Chat_controls_configuration<'_> {
        resources::Chat_controls_configuration::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
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
