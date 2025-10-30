//! Budgets_2016_10_20 Service
//!
//! Auto-generated service module for budgets_2016_10_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for budgets_2016_10_20
pub struct Budgets_2016_10_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Budgets_2016_10_20Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get subscriber resource handler
    pub fn subscriber(&self) -> resources::Subscriber<'_> {
        resources::Subscriber::new(self.provider)
    }
    /// Get notifications_for_budget resource handler
    pub fn notifications_for_budget(&self) -> resources::Notifications_for_budget<'_> {
        resources::Notifications_for_budget::new(self.provider)
    }
    /// Get budget_action resource handler
    pub fn budget_action(&self) -> resources::Budget_action<'_> {
        resources::Budget_action::new(self.provider)
    }
    /// Get budgets resource handler
    pub fn budgets(&self) -> resources::Budgets<'_> {
        resources::Budgets::new(self.provider)
    }
    /// Get budget_actions_for_account resource handler
    pub fn budget_actions_for_account(&self) -> resources::Budget_actions_for_account<'_> {
        resources::Budget_actions_for_account::new(self.provider)
    }
    /// Get budget_actions_for_budget resource handler
    pub fn budget_actions_for_budget(&self) -> resources::Budget_actions_for_budget<'_> {
        resources::Budget_actions_for_budget::new(self.provider)
    }
    /// Get budget_action_histories resource handler
    pub fn budget_action_histories(&self) -> resources::Budget_action_histories<'_> {
        resources::Budget_action_histories::new(self.provider)
    }
    /// Get budget resource handler
    pub fn budget(&self) -> resources::Budget<'_> {
        resources::Budget::new(self.provider)
    }
    /// Get budget_notifications_for_account resource handler
    pub fn budget_notifications_for_account(&self) -> resources::Budget_notifications_for_account<'_> {
        resources::Budget_notifications_for_account::new(self.provider)
    }
    /// Get budget_performance_history resource handler
    pub fn budget_performance_history(&self) -> resources::Budget_performance_history<'_> {
        resources::Budget_performance_history::new(self.provider)
    }
    /// Get subscribers_for_notification resource handler
    pub fn subscribers_for_notification(&self) -> resources::Subscribers_for_notification<'_> {
        resources::Subscribers_for_notification::new(self.provider)
    }
    /// Get notification resource handler
    pub fn notification(&self) -> resources::Notification<'_> {
        resources::Notification::new(self.provider)
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
