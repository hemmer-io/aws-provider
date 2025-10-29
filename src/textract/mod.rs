//! Textract Service
//!
//! Auto-generated service module for textract

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for textract
pub struct TextractService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TextractService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get document_text_detection resource handler
    pub fn document_text_detection(&self) -> resources::Document_text_detection<'_> {
        resources::Document_text_detection::new(self.provider)
    }
    /// Get lending_analysis_summary resource handler
    pub fn lending_analysis_summary(&self) -> resources::Lending_analysis_summary<'_> {
        resources::Lending_analysis_summary::new(self.provider)
    }
    /// Get adapter resource handler
    pub fn adapter(&self) -> resources::Adapter<'_> {
        resources::Adapter::new(self.provider)
    }
    /// Get lending_analysis resource handler
    pub fn lending_analysis(&self) -> resources::Lending_analysis<'_> {
        resources::Lending_analysis::new(self.provider)
    }
    /// Get document_analysis resource handler
    pub fn document_analysis(&self) -> resources::Document_analysis<'_> {
        resources::Document_analysis::new(self.provider)
    }
    /// Get adapter_version resource handler
    pub fn adapter_version(&self) -> resources::Adapter_version<'_> {
        resources::Adapter_version::new(self.provider)
    }
    /// Get expense_analysis resource handler
    pub fn expense_analysis(&self) -> resources::Expense_analysis<'_> {
        resources::Expense_analysis::new(self.provider)
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
