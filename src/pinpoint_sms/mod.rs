//! Pinpoint_sms Service
//!
//! Auto-generated service module for pinpoint_sms

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pinpoint_sms
pub struct Pinpoint_smsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Pinpoint_smsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get registration resource handler
    pub fn registration(&self) -> resources::Registration<'_> {
        resources::Registration::new(self.provider)
    }
    /// Get account_limits resource handler
    pub fn account_limits(&self) -> resources::Account_limits<'_> {
        resources::Account_limits::new(self.provider)
    }
    /// Get default_message_type resource handler
    pub fn default_message_type(&self) -> resources::Default_message_type<'_> {
        resources::Default_message_type::new(self.provider)
    }
    /// Get configuration_sets resource handler
    pub fn configuration_sets(&self) -> resources::Configuration_sets<'_> {
        resources::Configuration_sets::new(self.provider)
    }
    /// Get opt_out_lists resource handler
    pub fn opt_out_lists(&self) -> resources::Opt_out_lists<'_> {
        resources::Opt_out_lists::new(self.provider)
    }
    /// Get text_message_spend_limit_override resource handler
    pub fn text_message_spend_limit_override(&self) -> resources::Text_message_spend_limit_override<'_> {
        resources::Text_message_spend_limit_override::new(self.provider)
    }
    /// Get phone_numbers resource handler
    pub fn phone_numbers(&self) -> resources::Phone_numbers<'_> {
        resources::Phone_numbers::new(self.provider)
    }
    /// Get registrations resource handler
    pub fn registrations(&self) -> resources::Registrations<'_> {
        resources::Registrations::new(self.provider)
    }
    /// Get registration_type_definitions resource handler
    pub fn registration_type_definitions(&self) -> resources::Registration_type_definitions<'_> {
        resources::Registration_type_definitions::new(self.provider)
    }
    /// Get verified_destination_numbers resource handler
    pub fn verified_destination_numbers(&self) -> resources::Verified_destination_numbers<'_> {
        resources::Verified_destination_numbers::new(self.provider)
    }
    /// Get protect_configuration_country_rule_set resource handler
    pub fn protect_configuration_country_rule_set(&self) -> resources::Protect_configuration_country_rule_set<'_> {
        resources::Protect_configuration_country_rule_set::new(self.provider)
    }
    /// Get protect_configuration_rule_set_number_override resource handler
    pub fn protect_configuration_rule_set_number_override(&self) -> resources::Protect_configuration_rule_set_number_override<'_> {
        resources::Protect_configuration_rule_set_number_override::new(self.provider)
    }
    /// Get phone_number resource handler
    pub fn phone_number(&self) -> resources::Phone_number<'_> {
        resources::Phone_number::new(self.provider)
    }
    /// Get pools resource handler
    pub fn pools(&self) -> resources::Pools<'_> {
        resources::Pools::new(self.provider)
    }
    /// Get media_message_spend_limit_override resource handler
    pub fn media_message_spend_limit_override(&self) -> resources::Media_message_spend_limit_override<'_> {
        resources::Media_message_spend_limit_override::new(self.provider)
    }
    /// Get opted_out_numbers resource handler
    pub fn opted_out_numbers(&self) -> resources::Opted_out_numbers<'_> {
        resources::Opted_out_numbers::new(self.provider)
    }
    /// Get spend_limits resource handler
    pub fn spend_limits(&self) -> resources::Spend_limits<'_> {
        resources::Spend_limits::new(self.provider)
    }
    /// Get sender_id resource handler
    pub fn sender_id(&self) -> resources::Sender_id<'_> {
        resources::Sender_id::new(self.provider)
    }
    /// Get registration_field_definitions resource handler
    pub fn registration_field_definitions(&self) -> resources::Registration_field_definitions<'_> {
        resources::Registration_field_definitions::new(self.provider)
    }
    /// Get protect_configuration resource handler
    pub fn protect_configuration(&self) -> resources::Protect_configuration<'_> {
        resources::Protect_configuration::new(self.provider)
    }
    /// Get registration_version resource handler
    pub fn registration_version(&self) -> resources::Registration_version<'_> {
        resources::Registration_version::new(self.provider)
    }
    /// Get event_destination resource handler
    pub fn event_destination(&self) -> resources::Event_destination<'_> {
        resources::Event_destination::new(self.provider)
    }
    /// Get default_sender_id resource handler
    pub fn default_sender_id(&self) -> resources::Default_sender_id<'_> {
        resources::Default_sender_id::new(self.provider)
    }
    /// Get registration_attachments resource handler
    pub fn registration_attachments(&self) -> resources::Registration_attachments<'_> {
        resources::Registration_attachments::new(self.provider)
    }
    /// Get registration_field_values resource handler
    pub fn registration_field_values(&self) -> resources::Registration_field_values<'_> {
        resources::Registration_field_values::new(self.provider)
    }
    /// Get sender_ids resource handler
    pub fn sender_ids(&self) -> resources::Sender_ids<'_> {
        resources::Sender_ids::new(self.provider)
    }
    /// Get keywords resource handler
    pub fn keywords(&self) -> resources::Keywords<'_> {
        resources::Keywords::new(self.provider)
    }
    /// Get registration_association resource handler
    pub fn registration_association(&self) -> resources::Registration_association<'_> {
        resources::Registration_association::new(self.provider)
    }
    /// Get registration_section_definitions resource handler
    pub fn registration_section_definitions(&self) -> resources::Registration_section_definitions<'_> {
        resources::Registration_section_definitions::new(self.provider)
    }
    /// Get opt_out_list resource handler
    pub fn opt_out_list(&self) -> resources::Opt_out_list<'_> {
        resources::Opt_out_list::new(self.provider)
    }
    /// Get message_feedback resource handler
    pub fn message_feedback(&self) -> resources::Message_feedback<'_> {
        resources::Message_feedback::new(self.provider)
    }
    /// Get opted_out_number resource handler
    pub fn opted_out_number(&self) -> resources::Opted_out_number<'_> {
        resources::Opted_out_number::new(self.provider)
    }
    /// Get verified_destination_number resource handler
    pub fn verified_destination_number(&self) -> resources::Verified_destination_number<'_> {
        resources::Verified_destination_number::new(self.provider)
    }
    /// Get voice_message_spend_limit_override resource handler
    pub fn voice_message_spend_limit_override(&self) -> resources::Voice_message_spend_limit_override<'_> {
        resources::Voice_message_spend_limit_override::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get registration_field_value resource handler
    pub fn registration_field_value(&self) -> resources::Registration_field_value<'_> {
        resources::Registration_field_value::new(self.provider)
    }
    /// Get configuration_set resource handler
    pub fn configuration_set(&self) -> resources::Configuration_set<'_> {
        resources::Configuration_set::new(self.provider)
    }
    /// Get pool resource handler
    pub fn pool(&self) -> resources::Pool<'_> {
        resources::Pool::new(self.provider)
    }
    /// Get account_attributes resource handler
    pub fn account_attributes(&self) -> resources::Account_attributes<'_> {
        resources::Account_attributes::new(self.provider)
    }
    /// Get registration_attachment resource handler
    pub fn registration_attachment(&self) -> resources::Registration_attachment<'_> {
        resources::Registration_attachment::new(self.provider)
    }
    /// Get protect_configurations resource handler
    pub fn protect_configurations(&self) -> resources::Protect_configurations<'_> {
        resources::Protect_configurations::new(self.provider)
    }
    /// Get keyword resource handler
    pub fn keyword(&self) -> resources::Keyword<'_> {
        resources::Keyword::new(self.provider)
    }
    /// Get account_default_protect_configuration resource handler
    pub fn account_default_protect_configuration(&self) -> resources::Account_default_protect_configuration<'_> {
        resources::Account_default_protect_configuration::new(self.provider)
    }
    /// Get registration_versions resource handler
    pub fn registration_versions(&self) -> resources::Registration_versions<'_> {
        resources::Registration_versions::new(self.provider)
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
