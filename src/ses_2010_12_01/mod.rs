//! Ses_2010_12_01 Service
//!
//! Auto-generated service module for ses_2010_12_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ses_2010_12_01
pub struct Ses_2010_12_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ses_2010_12_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_sending_enabled resource handler
    pub fn account_sending_enabled(&self) -> resources::Account_sending_enabled<'_> {
        resources::Account_sending_enabled::new(self.provider)
    }
    /// Get send_quota resource handler
    pub fn send_quota(&self) -> resources::Send_quota<'_> {
        resources::Send_quota::new(self.provider)
    }
    /// Get configuration_set_delivery_options resource handler
    pub fn configuration_set_delivery_options(&self) -> resources::Configuration_set_delivery_options<'_> {
        resources::Configuration_set_delivery_options::new(self.provider)
    }
    /// Get send_statistics resource handler
    pub fn send_statistics(&self) -> resources::Send_statistics<'_> {
        resources::Send_statistics::new(self.provider)
    }
    /// Get identity_policy resource handler
    pub fn identity_policy(&self) -> resources::Identity_policy<'_> {
        resources::Identity_policy::new(self.provider)
    }
    /// Get identity_policies resource handler
    pub fn identity_policies(&self) -> resources::Identity_policies<'_> {
        resources::Identity_policies::new(self.provider)
    }
    /// Get verified_email_address resource handler
    pub fn verified_email_address(&self) -> resources::Verified_email_address<'_> {
        resources::Verified_email_address::new(self.provider)
    }
    /// Get receipt_filter resource handler
    pub fn receipt_filter(&self) -> resources::Receipt_filter<'_> {
        resources::Receipt_filter::new(self.provider)
    }
    /// Get template resource handler
    pub fn template(&self) -> resources::Template<'_> {
        resources::Template::new(self.provider)
    }
    /// Get receipt_rule_set resource handler
    pub fn receipt_rule_set(&self) -> resources::Receipt_rule_set<'_> {
        resources::Receipt_rule_set::new(self.provider)
    }
    /// Get identity_verification_attributes resource handler
    pub fn identity_verification_attributes(&self) -> resources::Identity_verification_attributes<'_> {
        resources::Identity_verification_attributes::new(self.provider)
    }
    /// Get configuration_set_event_destination resource handler
    pub fn configuration_set_event_destination(&self) -> resources::Configuration_set_event_destination<'_> {
        resources::Configuration_set_event_destination::new(self.provider)
    }
    /// Get identity_dkim_attributes resource handler
    pub fn identity_dkim_attributes(&self) -> resources::Identity_dkim_attributes<'_> {
        resources::Identity_dkim_attributes::new(self.provider)
    }
    /// Get custom_verification_email_template resource handler
    pub fn custom_verification_email_template(&self) -> resources::Custom_verification_email_template<'_> {
        resources::Custom_verification_email_template::new(self.provider)
    }
    /// Get active_receipt_rule_set resource handler
    pub fn active_receipt_rule_set(&self) -> resources::Active_receipt_rule_set<'_> {
        resources::Active_receipt_rule_set::new(self.provider)
    }
    /// Get configuration_set resource handler
    pub fn configuration_set(&self) -> resources::Configuration_set<'_> {
        resources::Configuration_set::new(self.provider)
    }
    /// Get configuration_set_tracking_options resource handler
    pub fn configuration_set_tracking_options(&self) -> resources::Configuration_set_tracking_options<'_> {
        resources::Configuration_set_tracking_options::new(self.provider)
    }
    /// Get configuration_set_sending_enabled resource handler
    pub fn configuration_set_sending_enabled(&self) -> resources::Configuration_set_sending_enabled<'_> {
        resources::Configuration_set_sending_enabled::new(self.provider)
    }
    /// Get receipt_rule resource handler
    pub fn receipt_rule(&self) -> resources::Receipt_rule<'_> {
        resources::Receipt_rule::new(self.provider)
    }
    /// Get identity resource handler
    pub fn identity(&self) -> resources::Identity<'_> {
        resources::Identity::new(self.provider)
    }
    /// Get configuration_set_reputation_metrics_enabled resource handler
    pub fn configuration_set_reputation_metrics_enabled(&self) -> resources::Configuration_set_reputation_metrics_enabled<'_> {
        resources::Configuration_set_reputation_metrics_enabled::new(self.provider)
    }
    /// Get identity_mail_from_domain_attributes resource handler
    pub fn identity_mail_from_domain_attributes(&self) -> resources::Identity_mail_from_domain_attributes<'_> {
        resources::Identity_mail_from_domain_attributes::new(self.provider)
    }
    /// Get identity_notification_attributes resource handler
    pub fn identity_notification_attributes(&self) -> resources::Identity_notification_attributes<'_> {
        resources::Identity_notification_attributes::new(self.provider)
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
