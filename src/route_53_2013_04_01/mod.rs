//! Route_53_2013_04_01 Service
//!
//! Auto-generated service module for route_53_2013_04_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for route_53_2013_04_01
pub struct Route_53_2013_04_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Route_53_2013_04_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get key_signing_key resource handler
    pub fn key_signing_key(&self) -> resources::Key_signing_key<'_> {
        resources::Key_signing_key::new(self.provider)
    }
    /// Get reusable_delegation_set_limit resource handler
    pub fn reusable_delegation_set_limit(&self) -> resources::Reusable_delegation_set_limit<'_> {
        resources::Reusable_delegation_set_limit::new(self.provider)
    }
    /// Get change resource handler
    pub fn change(&self) -> resources::Change<'_> {
        resources::Change::new(self.provider)
    }
    /// Get query_logging_config resource handler
    pub fn query_logging_config(&self) -> resources::Query_logging_config<'_> {
        resources::Query_logging_config::new(self.provider)
    }
    /// Get traffic_policy_version resource handler
    pub fn traffic_policy_version(&self) -> resources::Traffic_policy_version<'_> {
        resources::Traffic_policy_version::new(self.provider)
    }
    /// Get health_check resource handler
    pub fn health_check(&self) -> resources::Health_check<'_> {
        resources::Health_check::new(self.provider)
    }
    /// Get traffic_policy_instance_count resource handler
    pub fn traffic_policy_instance_count(&self) -> resources::Traffic_policy_instance_count<'_> {
        resources::Traffic_policy_instance_count::new(self.provider)
    }
    /// Get traffic_policy resource handler
    pub fn traffic_policy(&self) -> resources::Traffic_policy<'_> {
        resources::Traffic_policy::new(self.provider)
    }
    /// Get geo_location resource handler
    pub fn geo_location(&self) -> resources::Geo_location<'_> {
        resources::Geo_location::new(self.provider)
    }
    /// Get health_check_last_failure_reason resource handler
    pub fn health_check_last_failure_reason(&self) -> resources::Health_check_last_failure_reason<'_> {
        resources::Health_check_last_failure_reason::new(self.provider)
    }
    /// Get traffic_policy_comment resource handler
    pub fn traffic_policy_comment(&self) -> resources::Traffic_policy_comment<'_> {
        resources::Traffic_policy_comment::new(self.provider)
    }
    /// Get cidr_collection resource handler
    pub fn cidr_collection(&self) -> resources::Cidr_collection<'_> {
        resources::Cidr_collection::new(self.provider)
    }
    /// Get traffic_policy_instance resource handler
    pub fn traffic_policy_instance(&self) -> resources::Traffic_policy_instance<'_> {
        resources::Traffic_policy_instance::new(self.provider)
    }
    /// Get account_limit resource handler
    pub fn account_limit(&self) -> resources::Account_limit<'_> {
        resources::Account_limit::new(self.provider)
    }
    /// Get reusable_delegation_set resource handler
    pub fn reusable_delegation_set(&self) -> resources::Reusable_delegation_set<'_> {
        resources::Reusable_delegation_set::new(self.provider)
    }
    /// Get hosted_zone resource handler
    pub fn hosted_zone(&self) -> resources::Hosted_zone<'_> {
        resources::Hosted_zone::new(self.provider)
    }
    /// Get health_check_count resource handler
    pub fn health_check_count(&self) -> resources::Health_check_count<'_> {
        resources::Health_check_count::new(self.provider)
    }
    /// Get hosted_zone_limit resource handler
    pub fn hosted_zone_limit(&self) -> resources::Hosted_zone_limit<'_> {
        resources::Hosted_zone_limit::new(self.provider)
    }
    /// Get dnssec resource handler
    pub fn dnssec(&self) -> resources::Dnssec<'_> {
        resources::Dnssec::new(self.provider)
    }
    /// Get health_check_status resource handler
    pub fn health_check_status(&self) -> resources::Health_check_status<'_> {
        resources::Health_check_status::new(self.provider)
    }
    /// Get hosted_zone_comment resource handler
    pub fn hosted_zone_comment(&self) -> resources::Hosted_zone_comment<'_> {
        resources::Hosted_zone_comment::new(self.provider)
    }
    /// Get vpc_association_authorization resource handler
    pub fn vpc_association_authorization(&self) -> resources::Vpc_association_authorization<'_> {
        resources::Vpc_association_authorization::new(self.provider)
    }
    /// Get checker_ip_ranges resource handler
    pub fn checker_ip_ranges(&self) -> resources::Checker_ip_ranges<'_> {
        resources::Checker_ip_ranges::new(self.provider)
    }
    /// Get hosted_zone_count resource handler
    pub fn hosted_zone_count(&self) -> resources::Hosted_zone_count<'_> {
        resources::Hosted_zone_count::new(self.provider)
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
