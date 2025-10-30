//! Iot_2015_05_28 Service
//!
//! Auto-generated service module for iot_2015_05_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot_2015_05_28
pub struct Iot_2015_05_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iot_2015_05_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get logging_options resource handler
    pub fn logging_options(&self) -> resources::Logging_options<'_> {
        resources::Logging_options::new(self.provider)
    }
    /// Get billing_group resource handler
    pub fn billing_group(&self) -> resources::Billing_group<'_> {
        resources::Billing_group::new(self.provider)
    }
    /// Get provisioning_template resource handler
    pub fn provisioning_template(&self) -> resources::Provisioning_template<'_> {
        resources::Provisioning_template::new(self.provider)
    }
    /// Get event_configurations resource handler
    pub fn event_configurations(&self) -> resources::Event_configurations<'_> {
        resources::Event_configurations::new(self.provider)
    }
    /// Get topic_rule resource handler
    pub fn topic_rule(&self) -> resources::Topic_rule<'_> {
        resources::Topic_rule::new(self.provider)
    }
    /// Get percentiles resource handler
    pub fn percentiles(&self) -> resources::Percentiles<'_> {
        resources::Percentiles::new(self.provider)
    }
    /// Get job_execution resource handler
    pub fn job_execution(&self) -> resources::Job_execution<'_> {
        resources::Job_execution::new(self.provider)
    }
    /// Get package_version resource handler
    pub fn package_version(&self) -> resources::Package_version<'_> {
        resources::Package_version::new(self.provider)
    }
    /// Get thing_registration_task resource handler
    pub fn thing_registration_task(&self) -> resources::Thing_registration_task<'_> {
        resources::Thing_registration_task::new(self.provider)
    }
    /// Get registration_code resource handler
    pub fn registration_code(&self) -> resources::Registration_code<'_> {
        resources::Registration_code::new(self.provider)
    }
    /// Get encryption_configuration resource handler
    pub fn encryption_configuration(&self) -> resources::Encryption_configuration<'_> {
        resources::Encryption_configuration::new(self.provider)
    }
    /// Get audit_task resource handler
    pub fn audit_task(&self) -> resources::Audit_task<'_> {
        resources::Audit_task::new(self.provider)
    }
    /// Get domain_configuration resource handler
    pub fn domain_configuration(&self) -> resources::Domain_configuration<'_> {
        resources::Domain_configuration::new(self.provider)
    }
    /// Get managed_job_template resource handler
    pub fn managed_job_template(&self) -> resources::Managed_job_template<'_> {
        resources::Managed_job_template::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get command_execution resource handler
    pub fn command_execution(&self) -> resources::Command_execution<'_> {
        resources::Command_execution::new(self.provider)
    }
    /// Get ota_update resource handler
    pub fn ota_update(&self) -> resources::Ota_update<'_> {
        resources::Ota_update::new(self.provider)
    }
    /// Get indexing_configuration resource handler
    pub fn indexing_configuration(&self) -> resources::Indexing_configuration<'_> {
        resources::Indexing_configuration::new(self.provider)
    }
    /// Get custom_metric resource handler
    pub fn custom_metric(&self) -> resources::Custom_metric<'_> {
        resources::Custom_metric::new(self.provider)
    }
    /// Get buckets_aggregation resource handler
    pub fn buckets_aggregation(&self) -> resources::Buckets_aggregation<'_> {
        resources::Buckets_aggregation::new(self.provider)
    }
    /// Get dimension resource handler
    pub fn dimension(&self) -> resources::Dimension<'_> {
        resources::Dimension::new(self.provider)
    }
    /// Get fleet_metric resource handler
    pub fn fleet_metric(&self) -> resources::Fleet_metric<'_> {
        resources::Fleet_metric::new(self.provider)
    }
    /// Get thing_groups_for_thing resource handler
    pub fn thing_groups_for_thing(&self) -> resources::Thing_groups_for_thing<'_> {
        resources::Thing_groups_for_thing::new(self.provider)
    }
    /// Get security_profile resource handler
    pub fn security_profile(&self) -> resources::Security_profile<'_> {
        resources::Security_profile::new(self.provider)
    }
    /// Get default_authorizer resource handler
    pub fn default_authorizer(&self) -> resources::Default_authorizer<'_> {
        resources::Default_authorizer::new(self.provider)
    }
    /// Get package_configuration resource handler
    pub fn package_configuration(&self) -> resources::Package_configuration<'_> {
        resources::Package_configuration::new(self.provider)
    }
    /// Get scheduled_audit resource handler
    pub fn scheduled_audit(&self) -> resources::Scheduled_audit<'_> {
        resources::Scheduled_audit::new(self.provider)
    }
    /// Get provisioning_template_version resource handler
    pub fn provisioning_template_version(&self) -> resources::Provisioning_template_version<'_> {
        resources::Provisioning_template_version::new(self.provider)
    }
    /// Get job_template resource handler
    pub fn job_template(&self) -> resources::Job_template<'_> {
        resources::Job_template::new(self.provider)
    }
    /// Get thing_type resource handler
    pub fn thing_type(&self) -> resources::Thing_type<'_> {
        resources::Thing_type::new(self.provider)
    }
    /// Get v2_logging_level resource handler
    pub fn v2_logging_level(&self) -> resources::V2_logging_level<'_> {
        resources::V2_logging_level::new(self.provider)
    }
    /// Get thing_group resource handler
    pub fn thing_group(&self) -> resources::Thing_group<'_> {
        resources::Thing_group::new(self.provider)
    }
    /// Get thing resource handler
    pub fn thing(&self) -> resources::Thing<'_> {
        resources::Thing::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get job_document resource handler
    pub fn job_document(&self) -> resources::Job_document<'_> {
        resources::Job_document::new(self.provider)
    }
    /// Get detect_mitigation_actions_task resource handler
    pub fn detect_mitigation_actions_task(&self) -> resources::Detect_mitigation_actions_task<'_> {
        resources::Detect_mitigation_actions_task::new(self.provider)
    }
    /// Get role_alias resource handler
    pub fn role_alias(&self) -> resources::Role_alias<'_> {
        resources::Role_alias::new(self.provider)
    }
    /// Get certificate_provider resource handler
    pub fn certificate_provider(&self) -> resources::Certificate_provider<'_> {
        resources::Certificate_provider::new(self.provider)
    }
    /// Get statistics resource handler
    pub fn statistics(&self) -> resources::Statistics<'_> {
        resources::Statistics::new(self.provider)
    }
    /// Get provisioning_claim resource handler
    pub fn provisioning_claim(&self) -> resources::Provisioning_claim<'_> {
        resources::Provisioning_claim::new(self.provider)
    }
    /// Get authorizer resource handler
    pub fn authorizer(&self) -> resources::Authorizer<'_> {
        resources::Authorizer::new(self.provider)
    }
    /// Get account_audit_configuration resource handler
    pub fn account_audit_configuration(&self) -> resources::Account_audit_configuration<'_> {
        resources::Account_audit_configuration::new(self.provider)
    }
    /// Get v2_logging_options resource handler
    pub fn v2_logging_options(&self) -> resources::V2_logging_options<'_> {
        resources::V2_logging_options::new(self.provider)
    }
    /// Get index resource handler
    pub fn index(&self) -> resources::Index<'_> {
        resources::Index::new(self.provider)
    }
    /// Get verification_state_on_violation resource handler
    pub fn verification_state_on_violation(&self) -> resources::Verification_state_on_violation<'_> {
        resources::Verification_state_on_violation::new(self.provider)
    }
    /// Get audit_suppression resource handler
    pub fn audit_suppression(&self) -> resources::Audit_suppression<'_> {
        resources::Audit_suppression::new(self.provider)
    }
    /// Get mitigation_action resource handler
    pub fn mitigation_action(&self) -> resources::Mitigation_action<'_> {
        resources::Mitigation_action::new(self.provider)
    }
    /// Get policy_version resource handler
    pub fn policy_version(&self) -> resources::Policy_version<'_> {
        resources::Policy_version::new(self.provider)
    }
    /// Get audit_finding resource handler
    pub fn audit_finding(&self) -> resources::Audit_finding<'_> {
        resources::Audit_finding::new(self.provider)
    }
    /// Get thing_connectivity_data resource handler
    pub fn thing_connectivity_data(&self) -> resources::Thing_connectivity_data<'_> {
        resources::Thing_connectivity_data::new(self.provider)
    }
    /// Get package resource handler
    pub fn package(&self) -> resources::Package<'_> {
        resources::Package::new(self.provider)
    }
    /// Get stream resource handler
    pub fn stream(&self) -> resources::Stream<'_> {
        resources::Stream::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get cardinality resource handler
    pub fn cardinality(&self) -> resources::Cardinality<'_> {
        resources::Cardinality::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get behavior_model_training_summaries resource handler
    pub fn behavior_model_training_summaries(&self) -> resources::Behavior_model_training_summaries<'_> {
        resources::Behavior_model_training_summaries::new(self.provider)
    }
    /// Get ca_certificate resource handler
    pub fn ca_certificate(&self) -> resources::Ca_certificate<'_> {
        resources::Ca_certificate::new(self.provider)
    }
    /// Get audit_mitigation_actions_task resource handler
    pub fn audit_mitigation_actions_task(&self) -> resources::Audit_mitigation_actions_task<'_> {
        resources::Audit_mitigation_actions_task::new(self.provider)
    }
    /// Get keys_and_certificate resource handler
    pub fn keys_and_certificate(&self) -> resources::Keys_and_certificate<'_> {
        resources::Keys_and_certificate::new(self.provider)
    }
    /// Get command resource handler
    pub fn command(&self) -> resources::Command<'_> {
        resources::Command::new(self.provider)
    }
    /// Get certificate_from_csr resource handler
    pub fn certificate_from_csr(&self) -> resources::Certificate_from_csr<'_> {
        resources::Certificate_from_csr::new(self.provider)
    }
    /// Get effective_policies resource handler
    pub fn effective_policies(&self) -> resources::Effective_policies<'_> {
        resources::Effective_policies::new(self.provider)
    }
    /// Get dynamic_thing_group resource handler
    pub fn dynamic_thing_group(&self) -> resources::Dynamic_thing_group<'_> {
        resources::Dynamic_thing_group::new(self.provider)
    }
    /// Get topic_rule_destination resource handler
    pub fn topic_rule_destination(&self) -> resources::Topic_rule_destination<'_> {
        resources::Topic_rule_destination::new(self.provider)
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
