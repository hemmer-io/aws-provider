//! Codecommit Service
//!
//! Auto-generated service module for codecommit

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codecommit
pub struct CodecommitService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CodecommitService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get repository_name resource handler
    pub fn repository_name(&self) -> resources::Repository_name<'_> {
        resources::Repository_name::new(self.provider)
    }
    /// Get blob resource handler
    pub fn blob(&self) -> resources::Blob<'_> {
        resources::Blob::new(self.provider)
    }
    /// Get commit resource handler
    pub fn commit(&self) -> resources::Commit<'_> {
        resources::Commit::new(self.provider)
    }
    /// Get pull_request_description resource handler
    pub fn pull_request_description(&self) -> resources::Pull_request_description<'_> {
        resources::Pull_request_description::new(self.provider)
    }
    /// Get merge_options resource handler
    pub fn merge_options(&self) -> resources::Merge_options<'_> {
        resources::Merge_options::new(self.provider)
    }
    /// Get pull_request_approval_state resource handler
    pub fn pull_request_approval_state(&self) -> resources::Pull_request_approval_state<'_> {
        resources::Pull_request_approval_state::new(self.provider)
    }
    /// Get differences resource handler
    pub fn differences(&self) -> resources::Differences<'_> {
        resources::Differences::new(self.provider)
    }
    /// Get pull_request_events resource handler
    pub fn pull_request_events(&self) -> resources::Pull_request_events<'_> {
        resources::Pull_request_events::new(self.provider)
    }
    /// Get approval_rule_template_name resource handler
    pub fn approval_rule_template_name(&self) -> resources::Approval_rule_template_name<'_> {
        resources::Approval_rule_template_name::new(self.provider)
    }
    /// Get pull_request_approval_rule resource handler
    pub fn pull_request_approval_rule(&self) -> resources::Pull_request_approval_rule<'_> {
        resources::Pull_request_approval_rule::new(self.provider)
    }
    /// Get merge_commit resource handler
    pub fn merge_commit(&self) -> resources::Merge_commit<'_> {
        resources::Merge_commit::new(self.provider)
    }
    /// Get pull_request_approval_states resource handler
    pub fn pull_request_approval_states(&self) -> resources::Pull_request_approval_states<'_> {
        resources::Pull_request_approval_states::new(self.provider)
    }
    /// Get repository resource handler
    pub fn repository(&self) -> resources::Repository<'_> {
        resources::Repository::new(self.provider)
    }
    /// Get approval_rule_template_content resource handler
    pub fn approval_rule_template_content(&self) -> resources::Approval_rule_template_content<'_> {
        resources::Approval_rule_template_content::new(self.provider)
    }
    /// Get file resource handler
    pub fn file(&self) -> resources::File<'_> {
        resources::File::new(self.provider)
    }
    /// Get merge_conflicts resource handler
    pub fn merge_conflicts(&self) -> resources::Merge_conflicts<'_> {
        resources::Merge_conflicts::new(self.provider)
    }
    /// Get pull_request_status resource handler
    pub fn pull_request_status(&self) -> resources::Pull_request_status<'_> {
        resources::Pull_request_status::new(self.provider)
    }
    /// Get pull_request_title resource handler
    pub fn pull_request_title(&self) -> resources::Pull_request_title<'_> {
        resources::Pull_request_title::new(self.provider)
    }
    /// Get approval_rule_template resource handler
    pub fn approval_rule_template(&self) -> resources::Approval_rule_template<'_> {
        resources::Approval_rule_template::new(self.provider)
    }
    /// Get unreferenced_merge_commit resource handler
    pub fn unreferenced_merge_commit(&self) -> resources::Unreferenced_merge_commit<'_> {
        resources::Unreferenced_merge_commit::new(self.provider)
    }
    /// Get approval_rule_template_description resource handler
    pub fn approval_rule_template_description(&self) -> resources::Approval_rule_template_description<'_> {
        resources::Approval_rule_template_description::new(self.provider)
    }
    /// Get comment_content resource handler
    pub fn comment_content(&self) -> resources::Comment_content<'_> {
        resources::Comment_content::new(self.provider)
    }
    /// Get comment_reaction resource handler
    pub fn comment_reaction(&self) -> resources::Comment_reaction<'_> {
        resources::Comment_reaction::new(self.provider)
    }
    /// Get comments_for_pull_request resource handler
    pub fn comments_for_pull_request(&self) -> resources::Comments_for_pull_request<'_> {
        resources::Comments_for_pull_request::new(self.provider)
    }
    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get repository_triggers resource handler
    pub fn repository_triggers(&self) -> resources::Repository_triggers<'_> {
        resources::Repository_triggers::new(self.provider)
    }
    /// Get default_branch resource handler
    pub fn default_branch(&self) -> resources::Default_branch<'_> {
        resources::Default_branch::new(self.provider)
    }
    /// Get pull_request_override_state resource handler
    pub fn pull_request_override_state(&self) -> resources::Pull_request_override_state<'_> {
        resources::Pull_request_override_state::new(self.provider)
    }
    /// Get comment_reactions resource handler
    pub fn comment_reactions(&self) -> resources::Comment_reactions<'_> {
        resources::Comment_reactions::new(self.provider)
    }
    /// Get repository_description resource handler
    pub fn repository_description(&self) -> resources::Repository_description<'_> {
        resources::Repository_description::new(self.provider)
    }
    /// Get repository_encryption_key resource handler
    pub fn repository_encryption_key(&self) -> resources::Repository_encryption_key<'_> {
        resources::Repository_encryption_key::new(self.provider)
    }
    /// Get branch resource handler
    pub fn branch(&self) -> resources::Branch<'_> {
        resources::Branch::new(self.provider)
    }
    /// Get pull_request resource handler
    pub fn pull_request(&self) -> resources::Pull_request<'_> {
        resources::Pull_request::new(self.provider)
    }
    /// Get comments_for_compared_commit resource handler
    pub fn comments_for_compared_commit(&self) -> resources::Comments_for_compared_commit<'_> {
        resources::Comments_for_compared_commit::new(self.provider)
    }
    /// Get pull_request_approval_rule_content resource handler
    pub fn pull_request_approval_rule_content(&self) -> resources::Pull_request_approval_rule_content<'_> {
        resources::Pull_request_approval_rule_content::new(self.provider)
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
