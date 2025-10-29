//! Sesv2 Service
//!
//! Auto-generated service module for sesv2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sesv2
pub struct Sesv2Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sesv2Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get email_identity_dkim_attributes resource handler
    pub fn email_identity_dkim_attributes(&self) -> resources::Email_identity_dkim_attributes<'_> {
        resources::Email_identity_dkim_attributes::new(self.provider)
    }
    /// Get email_identity_dkim_signing_attributes resource handler
    pub fn email_identity_dkim_signing_attributes(&self) -> resources::Email_identity_dkim_signing_attributes<'_> {
        resources::Email_identity_dkim_signing_attributes::new(self.provider)
    }
    /// Get reputation_entity_customer_managed_status resource handler
    pub fn reputation_entity_customer_managed_status(&self) -> resources::Reputation_entity_customer_managed_status<'_> {
        resources::Reputation_entity_customer_managed_status::new(self.provider)
    }
    /// Get account_suppression_attributes resource handler
    pub fn account_suppression_attributes(&self) -> resources::Account_suppression_attributes<'_> {
        resources::Account_suppression_attributes::new(self.provider)
    }
    /// Get email_identity_policy resource handler
    pub fn email_identity_policy(&self) -> resources::Email_identity_policy<'_> {
        resources::Email_identity_policy::new(self.provider)
    }
    /// Get custom_verification_email_template resource handler
    pub fn custom_verification_email_template(&self) -> resources::Custom_verification_email_template<'_> {
        resources::Custom_verification_email_template::new(self.provider)
    }
    /// Get configuration_set resource handler
    pub fn configuration_set(&self) -> resources::Configuration_set<'_> {
        resources::Configuration_set::new(self.provider)
    }
    /// Get contact_list resource handler
    pub fn contact_list(&self) -> resources::Contact_list<'_> {
        resources::Contact_list::new(self.provider)
    }
    /// Get email_identity resource handler
    pub fn email_identity(&self) -> resources::Email_identity<'_> {
        resources::Email_identity::new(self.provider)
    }
    /// Get blacklist_reports resource handler
    pub fn blacklist_reports(&self) -> resources::Blacklist_reports<'_> {
        resources::Blacklist_reports::new(self.provider)
    }
    /// Get dedicated_ips resource handler
    pub fn dedicated_ips(&self) -> resources::Dedicated_ips<'_> {
        resources::Dedicated_ips::new(self.provider)
    }
    /// Get account_sending_attributes resource handler
    pub fn account_sending_attributes(&self) -> resources::Account_sending_attributes<'_> {
        resources::Account_sending_attributes::new(self.provider)
    }
    /// Get message_insights resource handler
    pub fn message_insights(&self) -> resources::Message_insights<'_> {
        resources::Message_insights::new(self.provider)
    }
    /// Get configuration_set_sending_options resource handler
    pub fn configuration_set_sending_options(&self) -> resources::Configuration_set_sending_options<'_> {
        resources::Configuration_set_sending_options::new(self.provider)
    }
    /// Get contact resource handler
    pub fn contact(&self) -> resources::Contact<'_> {
        resources::Contact::new(self.provider)
    }
    /// Get deliverability_test_report resource handler
    pub fn deliverability_test_report(&self) -> resources::Deliverability_test_report<'_> {
        resources::Deliverability_test_report::new(self.provider)
    }
    /// Get configuration_set_event_destination resource handler
    pub fn configuration_set_event_destination(&self) -> resources::Configuration_set_event_destination<'_> {
        resources::Configuration_set_event_destination::new(self.provider)
    }
    /// Get dedicated_ip resource handler
    pub fn dedicated_ip(&self) -> resources::Dedicated_ip<'_> {
        resources::Dedicated_ip::new(self.provider)
    }
    /// Get deliverability_dashboard_options resource handler
    pub fn deliverability_dashboard_options(&self) -> resources::Deliverability_dashboard_options<'_> {
        resources::Deliverability_dashboard_options::new(self.provider)
    }
    /// Get import_job resource handler
    pub fn import_job(&self) -> resources::Import_job<'_> {
        resources::Import_job::new(self.provider)
    }
    /// Get multi_region_endpoint resource handler
    pub fn multi_region_endpoint(&self) -> resources::Multi_region_endpoint<'_> {
        resources::Multi_region_endpoint::new(self.provider)
    }
    /// Get account_details resource handler
    pub fn account_details(&self) -> resources::Account_details<'_> {
        resources::Account_details::new(self.provider)
    }
    /// Get configuration_set_reputation_options resource handler
    pub fn configuration_set_reputation_options(&self) -> resources::Configuration_set_reputation_options<'_> {
        resources::Configuration_set_reputation_options::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get export_job resource handler
    pub fn export_job(&self) -> resources::Export_job<'_> {
        resources::Export_job::new(self.provider)
    }
    /// Get suppressed_destination resource handler
    pub fn suppressed_destination(&self) -> resources::Suppressed_destination<'_> {
        resources::Suppressed_destination::new(self.provider)
    }
    /// Get configuration_set_suppression_options resource handler
    pub fn configuration_set_suppression_options(&self) -> resources::Configuration_set_suppression_options<'_> {
        resources::Configuration_set_suppression_options::new(self.provider)
    }
    /// Get dedicated_ip_pool resource handler
    pub fn dedicated_ip_pool(&self) -> resources::Dedicated_ip_pool<'_> {
        resources::Dedicated_ip_pool::new(self.provider)
    }
    /// Get account_dedicated_ip_warmup_attributes resource handler
    pub fn account_dedicated_ip_warmup_attributes(&self) -> resources::Account_dedicated_ip_warmup_attributes<'_> {
        resources::Account_dedicated_ip_warmup_attributes::new(self.provider)
    }
    /// Get dedicated_ip_in_pool resource handler
    pub fn dedicated_ip_in_pool(&self) -> resources::Dedicated_ip_in_pool<'_> {
        resources::Dedicated_ip_in_pool::new(self.provider)
    }
    /// Get dedicated_ip_pool_scaling_attributes resource handler
    pub fn dedicated_ip_pool_scaling_attributes(&self) -> resources::Dedicated_ip_pool_scaling_attributes<'_> {
        resources::Dedicated_ip_pool_scaling_attributes::new(self.provider)
    }
    /// Get reputation_entity_policy resource handler
    pub fn reputation_entity_policy(&self) -> resources::Reputation_entity_policy<'_> {
        resources::Reputation_entity_policy::new(self.provider)
    }
    /// Get domain_deliverability_campaign resource handler
    pub fn domain_deliverability_campaign(&self) -> resources::Domain_deliverability_campaign<'_> {
        resources::Domain_deliverability_campaign::new(self.provider)
    }
    /// Get configuration_set_archiving_options resource handler
    pub fn configuration_set_archiving_options(&self) -> resources::Configuration_set_archiving_options<'_> {
        resources::Configuration_set_archiving_options::new(self.provider)
    }
    /// Get dedicated_ip_warmup_attributes resource handler
    pub fn dedicated_ip_warmup_attributes(&self) -> resources::Dedicated_ip_warmup_attributes<'_> {
        resources::Dedicated_ip_warmup_attributes::new(self.provider)
    }
    /// Get email_identity_feedback_attributes resource handler
    pub fn email_identity_feedback_attributes(&self) -> resources::Email_identity_feedback_attributes<'_> {
        resources::Email_identity_feedback_attributes::new(self.provider)
    }
    /// Get configuration_set_event_destinations resource handler
    pub fn configuration_set_event_destinations(&self) -> resources::Configuration_set_event_destinations<'_> {
        resources::Configuration_set_event_destinations::new(self.provider)
    }
    /// Get tenant resource handler
    pub fn tenant(&self) -> resources::Tenant<'_> {
        resources::Tenant::new(self.provider)
    }
    /// Get deliverability_dashboard_option resource handler
    pub fn deliverability_dashboard_option(&self) -> resources::Deliverability_dashboard_option<'_> {
        resources::Deliverability_dashboard_option::new(self.provider)
    }
    /// Get account_vdm_attributes resource handler
    pub fn account_vdm_attributes(&self) -> resources::Account_vdm_attributes<'_> {
        resources::Account_vdm_attributes::new(self.provider)
    }
    /// Get reputation_entity resource handler
    pub fn reputation_entity(&self) -> resources::Reputation_entity<'_> {
        resources::Reputation_entity::new(self.provider)
    }
    /// Get email_identity_mail_from_attributes resource handler
    pub fn email_identity_mail_from_attributes(&self) -> resources::Email_identity_mail_from_attributes<'_> {
        resources::Email_identity_mail_from_attributes::new(self.provider)
    }
    /// Get email_template resource handler
    pub fn email_template(&self) -> resources::Email_template<'_> {
        resources::Email_template::new(self.provider)
    }
    /// Get domain_statistics_report resource handler
    pub fn domain_statistics_report(&self) -> resources::Domain_statistics_report<'_> {
        resources::Domain_statistics_report::new(self.provider)
    }
    /// Get email_identity_configuration_set_attributes resource handler
    pub fn email_identity_configuration_set_attributes(&self) -> resources::Email_identity_configuration_set_attributes<'_> {
        resources::Email_identity_configuration_set_attributes::new(self.provider)
    }
    /// Get configuration_set_tracking_options resource handler
    pub fn configuration_set_tracking_options(&self) -> resources::Configuration_set_tracking_options<'_> {
        resources::Configuration_set_tracking_options::new(self.provider)
    }
    /// Get configuration_set_delivery_options resource handler
    pub fn configuration_set_delivery_options(&self) -> resources::Configuration_set_delivery_options<'_> {
        resources::Configuration_set_delivery_options::new(self.provider)
    }
    /// Get email_identity_policies resource handler
    pub fn email_identity_policies(&self) -> resources::Email_identity_policies<'_> {
        resources::Email_identity_policies::new(self.provider)
    }
    /// Get tenant_resource_association resource handler
    pub fn tenant_resource_association(&self) -> resources::Tenant_resource_association<'_> {
        resources::Tenant_resource_association::new(self.provider)
    }
    /// Get configuration_set_vdm_options resource handler
    pub fn configuration_set_vdm_options(&self) -> resources::Configuration_set_vdm_options<'_> {
        resources::Configuration_set_vdm_options::new(self.provider)
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
