//! Datazone_2018_05_10 Service
//!
//! Auto-generated service module for datazone_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datazone_2018_05_10
pub struct Datazone_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Datazone_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_pool resource handler
    pub fn account_pool(&self) -> resources::Account_pool<'_> {
        resources::Account_pool::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get subscription_request_details resource handler
    pub fn subscription_request_details(&self) -> resources::Subscription_request_details<'_> {
        resources::Subscription_request_details::new(self.provider)
    }
    /// Get subscription_request resource handler
    pub fn subscription_request(&self) -> resources::Subscription_request<'_> {
        resources::Subscription_request::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get environment_credentials resource handler
    pub fn environment_credentials(&self) -> resources::Environment_credentials<'_> {
        resources::Environment_credentials::new(self.provider)
    }
    /// Get time_series_data_point resource handler
    pub fn time_series_data_point(&self) -> resources::Time_series_data_point<'_> {
        resources::Time_series_data_point::new(self.provider)
    }
    /// Get lineage_node resource handler
    pub fn lineage_node(&self) -> resources::Lineage_node<'_> {
        resources::Lineage_node::new(self.provider)
    }
    /// Get listing_change_set resource handler
    pub fn listing_change_set(&self) -> resources::Listing_change_set<'_> {
        resources::Listing_change_set::new(self.provider)
    }
    /// Get subscription_target resource handler
    pub fn subscription_target(&self) -> resources::Subscription_target<'_> {
        resources::Subscription_target::new(self.provider)
    }
    /// Get subscription_grant_status resource handler
    pub fn subscription_grant_status(&self) -> resources::Subscription_grant_status<'_> {
        resources::Subscription_grant_status::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get environment_blueprint resource handler
    pub fn environment_blueprint(&self) -> resources::Environment_blueprint<'_> {
        resources::Environment_blueprint::new(self.provider)
    }
    /// Get iam_portal_login_url resource handler
    pub fn iam_portal_login_url(&self) -> resources::Iam_portal_login_url<'_> {
        resources::Iam_portal_login_url::new(self.provider)
    }
    /// Get group_profile resource handler
    pub fn group_profile(&self) -> resources::Group_profile<'_> {
        resources::Group_profile::new(self.provider)
    }
    /// Get project_profile resource handler
    pub fn project_profile(&self) -> resources::Project_profile<'_> {
        resources::Project_profile::new(self.provider)
    }
    /// Get user_profile resource handler
    pub fn user_profile(&self) -> resources::User_profile<'_> {
        resources::User_profile::new(self.provider)
    }
    /// Get asset_filter resource handler
    pub fn asset_filter(&self) -> resources::Asset_filter<'_> {
        resources::Asset_filter::new(self.provider)
    }
    /// Get time_series_data_points resource handler
    pub fn time_series_data_points(&self) -> resources::Time_series_data_points<'_> {
        resources::Time_series_data_points::new(self.provider)
    }
    /// Get environment_action resource handler
    pub fn environment_action(&self) -> resources::Environment_action<'_> {
        resources::Environment_action::new(self.provider)
    }
    /// Get subscription_grant resource handler
    pub fn subscription_grant(&self) -> resources::Subscription_grant<'_> {
        resources::Subscription_grant::new(self.provider)
    }
    /// Get project_membership resource handler
    pub fn project_membership(&self) -> resources::Project_membership<'_> {
        resources::Project_membership::new(self.provider)
    }
    /// Get environment_profile resource handler
    pub fn environment_profile(&self) -> resources::Environment_profile<'_> {
        resources::Environment_profile::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get job_run resource handler
    pub fn job_run(&self) -> resources::Job_run<'_> {
        resources::Job_run::new(self.provider)
    }
    /// Get lineage_event resource handler
    pub fn lineage_event(&self) -> resources::Lineage_event<'_> {
        resources::Lineage_event::new(self.provider)
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
