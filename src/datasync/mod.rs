//! Datasync Service
//!
//! Auto-generated service module for datasync

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datasync
pub struct DatasyncService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DatasyncService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get location_efs resource handler
    pub fn location_efs(&self) -> resources::Location_efs<'_> {
        resources::Location_efs::new(self.provider)
    }
    /// Get task_execution resource handler
    pub fn task_execution(&self) -> resources::Task_execution<'_> {
        resources::Task_execution::new(self.provider)
    }
    /// Get location_azure_blob resource handler
    pub fn location_azure_blob(&self) -> resources::Location_azure_blob<'_> {
        resources::Location_azure_blob::new(self.provider)
    }
    /// Get location_object_storage resource handler
    pub fn location_object_storage(&self) -> resources::Location_object_storage<'_> {
        resources::Location_object_storage::new(self.provider)
    }
    /// Get location_fsx_lustre resource handler
    pub fn location_fsx_lustre(&self) -> resources::Location_fsx_lustre<'_> {
        resources::Location_fsx_lustre::new(self.provider)
    }
    /// Get location_nfs resource handler
    pub fn location_nfs(&self) -> resources::Location_nfs<'_> {
        resources::Location_nfs::new(self.provider)
    }
    /// Get location_smb resource handler
    pub fn location_smb(&self) -> resources::Location_smb<'_> {
        resources::Location_smb::new(self.provider)
    }
    /// Get location_fsx_open_zfs resource handler
    pub fn location_fsx_open_zfs(&self) -> resources::Location_fsx_open_zfs<'_> {
        resources::Location_fsx_open_zfs::new(self.provider)
    }
    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
    }
    /// Get location_hdfs resource handler
    pub fn location_hdfs(&self) -> resources::Location_hdfs<'_> {
        resources::Location_hdfs::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get agent resource handler
    pub fn agent(&self) -> resources::Agent<'_> {
        resources::Agent::new(self.provider)
    }
    /// Get location_s3 resource handler
    pub fn location_s3(&self) -> resources::Location_s3<'_> {
        resources::Location_s3::new(self.provider)
    }
    /// Get location_fsx_windows resource handler
    pub fn location_fsx_windows(&self) -> resources::Location_fsx_windows<'_> {
        resources::Location_fsx_windows::new(self.provider)
    }
    /// Get location_fsx_ontap resource handler
    pub fn location_fsx_ontap(&self) -> resources::Location_fsx_ontap<'_> {
        resources::Location_fsx_ontap::new(self.provider)
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
