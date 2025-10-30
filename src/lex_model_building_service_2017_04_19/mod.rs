//! Lex_model_building_service_2017_04_19 Service
//!
//! Auto-generated service module for lex_model_building_service_2017_04_19

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lex_model_building_service_2017_04_19
pub struct Lex_model_building_service_2017_04_19Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Lex_model_building_service_2017_04_19Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get bot_channel_associations resource handler
    pub fn bot_channel_associations(&self) -> resources::Bot_channel_associations<'_> {
        resources::Bot_channel_associations::new(self.provider)
    }
    /// Get bots resource handler
    pub fn bots(&self) -> resources::Bots<'_> {
        resources::Bots::new(self.provider)
    }
    /// Get builtin_slot_types resource handler
    pub fn builtin_slot_types(&self) -> resources::Builtin_slot_types<'_> {
        resources::Builtin_slot_types::new(self.provider)
    }
    /// Get intent resource handler
    pub fn intent(&self) -> resources::Intent<'_> {
        resources::Intent::new(self.provider)
    }
    /// Get migration resource handler
    pub fn migration(&self) -> resources::Migration<'_> {
        resources::Migration::new(self.provider)
    }
    /// Get slot_type_version resource handler
    pub fn slot_type_version(&self) -> resources::Slot_type_version<'_> {
        resources::Slot_type_version::new(self.provider)
    }
    /// Get migrations resource handler
    pub fn migrations(&self) -> resources::Migrations<'_> {
        resources::Migrations::new(self.provider)
    }
    /// Get bot_versions resource handler
    pub fn bot_versions(&self) -> resources::Bot_versions<'_> {
        resources::Bot_versions::new(self.provider)
    }
    /// Get intent_version resource handler
    pub fn intent_version(&self) -> resources::Intent_version<'_> {
        resources::Intent_version::new(self.provider)
    }
    /// Get bot resource handler
    pub fn bot(&self) -> resources::Bot<'_> {
        resources::Bot::new(self.provider)
    }
    /// Get bot_channel_association resource handler
    pub fn bot_channel_association(&self) -> resources::Bot_channel_association<'_> {
        resources::Bot_channel_association::new(self.provider)
    }
    /// Get bot_aliases resource handler
    pub fn bot_aliases(&self) -> resources::Bot_aliases<'_> {
        resources::Bot_aliases::new(self.provider)
    }
    /// Get intents resource handler
    pub fn intents(&self) -> resources::Intents<'_> {
        resources::Intents::new(self.provider)
    }
    /// Get slot_types resource handler
    pub fn slot_types(&self) -> resources::Slot_types<'_> {
        resources::Slot_types::new(self.provider)
    }
    /// Get bot_alias resource handler
    pub fn bot_alias(&self) -> resources::Bot_alias<'_> {
        resources::Bot_alias::new(self.provider)
    }
    /// Get slot_type_versions resource handler
    pub fn slot_type_versions(&self) -> resources::Slot_type_versions<'_> {
        resources::Slot_type_versions::new(self.provider)
    }
    /// Get import resource handler
    pub fn import(&self) -> resources::Import<'_> {
        resources::Import::new(self.provider)
    }
    /// Get builtin_intent resource handler
    pub fn builtin_intent(&self) -> resources::Builtin_intent<'_> {
        resources::Builtin_intent::new(self.provider)
    }
    /// Get builtin_intents resource handler
    pub fn builtin_intents(&self) -> resources::Builtin_intents<'_> {
        resources::Builtin_intents::new(self.provider)
    }
    /// Get export resource handler
    pub fn export(&self) -> resources::Export<'_> {
        resources::Export::new(self.provider)
    }
    /// Get slot_type resource handler
    pub fn slot_type(&self) -> resources::Slot_type<'_> {
        resources::Slot_type::new(self.provider)
    }
    /// Get intent_versions resource handler
    pub fn intent_versions(&self) -> resources::Intent_versions<'_> {
        resources::Intent_versions::new(self.provider)
    }
    /// Get bot_version resource handler
    pub fn bot_version(&self) -> resources::Bot_version<'_> {
        resources::Bot_version::new(self.provider)
    }
    /// Get utterances resource handler
    pub fn utterances(&self) -> resources::Utterances<'_> {
        resources::Utterances::new(self.provider)
    }
    /// Get utterances_view resource handler
    pub fn utterances_view(&self) -> resources::Utterances_view<'_> {
        resources::Utterances_view::new(self.provider)
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
