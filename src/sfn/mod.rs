//! Sfn Service
//!
//! Auto-generated service module for sfn

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sfn
pub struct SfnService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SfnService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get state_machine resource handler
    pub fn state_machine(&self) -> resources::State_machine<'_> {
        resources::State_machine::new(self.provider)
    }
    /// Get state_machine_alias resource handler
    pub fn state_machine_alias(&self) -> resources::State_machine_alias<'_> {
        resources::State_machine_alias::new(self.provider)
    }
    /// Get state_machine_for_execution resource handler
    pub fn state_machine_for_execution(&self) -> resources::State_machine_for_execution<'_> {
        resources::State_machine_for_execution::new(self.provider)
    }
    /// Get activity resource handler
    pub fn activity(&self) -> resources::Activity<'_> {
        resources::Activity::new(self.provider)
    }
    /// Get state_machine_version resource handler
    pub fn state_machine_version(&self) -> resources::State_machine_version<'_> {
        resources::State_machine_version::new(self.provider)
    }
    /// Get execution_history resource handler
    pub fn execution_history(&self) -> resources::Execution_history<'_> {
        resources::Execution_history::new(self.provider)
    }
    /// Get activity_task resource handler
    pub fn activity_task(&self) -> resources::Activity_task<'_> {
        resources::Activity_task::new(self.provider)
    }
    /// Get map_run resource handler
    pub fn map_run(&self) -> resources::Map_run<'_> {
        resources::Map_run::new(self.provider)
    }
    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
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
