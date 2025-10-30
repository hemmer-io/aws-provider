//! Pinpoint_email_2018_07_26 Service
//!
//! Auto-generated service module for pinpoint_email_2018_07_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pinpoint_email_2018_07_26
pub struct Pinpoint_email_2018_07_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Pinpoint_email_2018_07_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_sending_attributes resource handler
    pub fn account_sending_attributes(&self) -> resources::Account_sending_attributes<'_> {
        resources::Account_sending_attributes::new(self.provider)
    }
    /// Get configuration_set_delivery_options resource handler
    pub fn configuration_set_delivery_options(&self) -> resources::Configuration_set_delivery_options<'_> {
        resources::Configuration_set_delivery_options::new(self.provider)
    }
    /// Get configuration_set_sending_options resource handler
    pub fn configuration_set_sending_options(&self) -> resources::Configuration_set_sending_options<'_> {
        resources::Configuration_set_sending_options::new(self.provider)
    }
    /// Get email_identity resource handler
    pub fn email_identity(&self) -> resources::Email_identity<'_> {
        resources::Email_identity::new(self.provider)
    }
    /// Get deliverability_dashboard_option resource handler
    pub fn deliverability_dashboard_option(&self) -> resources::Deliverability_dashboard_option<'_> {
        resources::Deliverability_dashboard_option::new(self.provider)
    }
    /// Get email_identity_mail_from_attributes resource handler
    pub fn email_identity_mail_from_attributes(&self) -> resources::Email_identity_mail_from_attributes<'_> {
        resources::Email_identity_mail_from_attributes::new(self.provider)
    }
    /// Get domain_statistics_report resource handler
    pub fn domain_statistics_report(&self) -> resources::Domain_statistics_report<'_> {
        resources::Domain_statistics_report::new(self.provider)
    }
    /// Get email_identity_dkim_attributes resource handler
    pub fn email_identity_dkim_attributes(&self) -> resources::Email_identity_dkim_attributes<'_> {
        resources::Email_identity_dkim_attributes::new(self.provider)
    }
    /// Get configuration_set_tracking_options resource handler
    pub fn configuration_set_tracking_options(&self) -> resources::Configuration_set_tracking_options<'_> {
        resources::Configuration_set_tracking_options::new(self.provider)
    }
    /// Get configuration_set_event_destination resource handler
    pub fn configuration_set_event_destination(&self) -> resources::Configuration_set_event_destination<'_> {
        resources::Configuration_set_event_destination::new(self.provider)
    }
    /// Get domain_deliverability_campaign resource handler
    pub fn domain_deliverability_campaign(&self) -> resources::Domain_deliverability_campaign<'_> {
        resources::Domain_deliverability_campaign::new(self.provider)
    }
    /// Get dedicated_ip resource handler
    pub fn dedicated_ip(&self) -> resources::Dedicated_ip<'_> {
        resources::Dedicated_ip::new(self.provider)
    }
    /// Get deliverability_dashboard_options resource handler
    pub fn deliverability_dashboard_options(&self) -> resources::Deliverability_dashboard_options<'_> {
        resources::Deliverability_dashboard_options::new(self.provider)
    }
    /// Get dedicated_ip_warmup_attributes resource handler
    pub fn dedicated_ip_warmup_attributes(&self) -> resources::Dedicated_ip_warmup_attributes<'_> {
        resources::Dedicated_ip_warmup_attributes::new(self.provider)
    }
    /// Get account_dedicated_ip_warmup_attributes resource handler
    pub fn account_dedicated_ip_warmup_attributes(&self) -> resources::Account_dedicated_ip_warmup_attributes<'_> {
        resources::Account_dedicated_ip_warmup_attributes::new(self.provider)
    }
    /// Get configuration_set_reputation_options resource handler
    pub fn configuration_set_reputation_options(&self) -> resources::Configuration_set_reputation_options<'_> {
        resources::Configuration_set_reputation_options::new(self.provider)
    }
    /// Get deliverability_test_report resource handler
    pub fn deliverability_test_report(&self) -> resources::Deliverability_test_report<'_> {
        resources::Deliverability_test_report::new(self.provider)
    }
    /// Get dedicated_ip_in_pool resource handler
    pub fn dedicated_ip_in_pool(&self) -> resources::Dedicated_ip_in_pool<'_> {
        resources::Dedicated_ip_in_pool::new(self.provider)
    }
    /// Get email_identity_feedback_attributes resource handler
    pub fn email_identity_feedback_attributes(&self) -> resources::Email_identity_feedback_attributes<'_> {
        resources::Email_identity_feedback_attributes::new(self.provider)
    }
    /// Get dedicated_ips resource handler
    pub fn dedicated_ips(&self) -> resources::Dedicated_ips<'_> {
        resources::Dedicated_ips::new(self.provider)
    }
    /// Get configuration_set_event_destinations resource handler
    pub fn configuration_set_event_destinations(&self) -> resources::Configuration_set_event_destinations<'_> {
        resources::Configuration_set_event_destinations::new(self.provider)
    }
    /// Get blacklist_reports resource handler
    pub fn blacklist_reports(&self) -> resources::Blacklist_reports<'_> {
        resources::Blacklist_reports::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get configuration_set resource handler
    pub fn configuration_set(&self) -> resources::Configuration_set<'_> {
        resources::Configuration_set::new(self.provider)
    }
    /// Get dedicated_ip_pool resource handler
    pub fn dedicated_ip_pool(&self) -> resources::Dedicated_ip_pool<'_> {
        resources::Dedicated_ip_pool::new(self.provider)
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
