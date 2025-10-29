//! Translate Service
//!
//! Auto-generated service module for translate

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for translate
pub struct TranslateService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TranslateService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get text_translation_job resource handler
    pub fn text_translation_job(&self) -> resources::Text_translation_job<'_> {
        resources::Text_translation_job::new(self.provider)
    }
    /// Get terminology resource handler
    pub fn terminology(&self) -> resources::Terminology<'_> {
        resources::Terminology::new(self.provider)
    }
    /// Get parallel_data resource handler
    pub fn parallel_data(&self) -> resources::Parallel_data<'_> {
        resources::Parallel_data::new(self.provider)
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
