//! Lex_models Service
//!
//! Auto-generated service module for lex_models

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lex_models
pub struct Lex_modelsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Lex_modelsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get import resource handler
    pub fn import(&self) -> resources::Import<'_> {
        resources::Import::new(self.provider)
    }
    /// Get test_set_generation resource handler
    pub fn test_set_generation(&self) -> resources::Test_set_generation<'_> {
        resources::Test_set_generation::new(self.provider)
    }
    /// Get custom_vocabulary_metadata resource handler
    pub fn custom_vocabulary_metadata(&self) -> resources::Custom_vocabulary_metadata<'_> {
        resources::Custom_vocabulary_metadata::new(self.provider)
    }
    /// Get bot resource handler
    pub fn bot(&self) -> resources::Bot<'_> {
        resources::Bot::new(self.provider)
    }
    /// Get test_set_discrepancy_report resource handler
    pub fn test_set_discrepancy_report(&self) -> resources::Test_set_discrepancy_report<'_> {
        resources::Test_set_discrepancy_report::new(self.provider)
    }
    /// Get bot_alias resource handler
    pub fn bot_alias(&self) -> resources::Bot_alias<'_> {
        resources::Bot_alias::new(self.provider)
    }
    /// Get utterances resource handler
    pub fn utterances(&self) -> resources::Utterances<'_> {
        resources::Utterances::new(self.provider)
    }
    /// Get slot_type resource handler
    pub fn slot_type(&self) -> resources::Slot_type<'_> {
        resources::Slot_type::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get bot_resource_generation resource handler
    pub fn bot_resource_generation(&self) -> resources::Bot_resource_generation<'_> {
        resources::Bot_resource_generation::new(self.provider)
    }
    /// Get bot_version resource handler
    pub fn bot_version(&self) -> resources::Bot_version<'_> {
        resources::Bot_version::new(self.provider)
    }
    /// Get resource_policy_statement resource handler
    pub fn resource_policy_statement(&self) -> resources::Resource_policy_statement<'_> {
        resources::Resource_policy_statement::new(self.provider)
    }
    /// Get slot resource handler
    pub fn slot(&self) -> resources::Slot<'_> {
        resources::Slot::new(self.provider)
    }
    /// Get export resource handler
    pub fn export(&self) -> resources::Export<'_> {
        resources::Export::new(self.provider)
    }
    /// Get upload_url resource handler
    pub fn upload_url(&self) -> resources::Upload_url<'_> {
        resources::Upload_url::new(self.provider)
    }
    /// Get test_execution resource handler
    pub fn test_execution(&self) -> resources::Test_execution<'_> {
        resources::Test_execution::new(self.provider)
    }
    /// Get bot_locale resource handler
    pub fn bot_locale(&self) -> resources::Bot_locale<'_> {
        resources::Bot_locale::new(self.provider)
    }
    /// Get test_set resource handler
    pub fn test_set(&self) -> resources::Test_set<'_> {
        resources::Test_set::new(self.provider)
    }
    /// Get test_execution_artifacts_url resource handler
    pub fn test_execution_artifacts_url(&self) -> resources::Test_execution_artifacts_url<'_> {
        resources::Test_execution_artifacts_url::new(self.provider)
    }
    /// Get bot_recommendation resource handler
    pub fn bot_recommendation(&self) -> resources::Bot_recommendation<'_> {
        resources::Bot_recommendation::new(self.provider)
    }
    /// Get intent resource handler
    pub fn intent(&self) -> resources::Intent<'_> {
        resources::Intent::new(self.provider)
    }
    /// Get bot_replica resource handler
    pub fn bot_replica(&self) -> resources::Bot_replica<'_> {
        resources::Bot_replica::new(self.provider)
    }
    /// Get custom_vocabulary resource handler
    pub fn custom_vocabulary(&self) -> resources::Custom_vocabulary<'_> {
        resources::Custom_vocabulary::new(self.provider)
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
