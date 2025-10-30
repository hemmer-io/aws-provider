//! Databrew_2017_07_25 Service
//!
//! Auto-generated service module for databrew_2017_07_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for databrew_2017_07_25
pub struct Databrew_2017_07_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Databrew_2017_07_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get recipe resource handler
    pub fn recipe(&self) -> resources::Recipe<'_> {
        resources::Recipe::new(self.provider)
    }
    /// Get recipe_job resource handler
    pub fn recipe_job(&self) -> resources::Recipe_job<'_> {
        resources::Recipe_job::new(self.provider)
    }
    /// Get schedule resource handler
    pub fn schedule(&self) -> resources::Schedule<'_> {
        resources::Schedule::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get job_run resource handler
    pub fn job_run(&self) -> resources::Job_run<'_> {
        resources::Job_run::new(self.provider)
    }
    /// Get ruleset resource handler
    pub fn ruleset(&self) -> resources::Ruleset<'_> {
        resources::Ruleset::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get recipe_version resource handler
    pub fn recipe_version(&self) -> resources::Recipe_version<'_> {
        resources::Recipe_version::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get profile_job resource handler
    pub fn profile_job(&self) -> resources::Profile_job<'_> {
        resources::Profile_job::new(self.provider)
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
