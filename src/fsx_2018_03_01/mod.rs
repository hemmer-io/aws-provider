//! Fsx_2018_03_01 Service
//!
//! Auto-generated service module for fsx_2018_03_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fsx_2018_03_01
pub struct Fsx_2018_03_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Fsx_2018_03_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get volume_from_backup resource handler
    pub fn volume_from_backup(&self) -> resources::Volume_from_backup<'_> {
        resources::Volume_from_backup::new(self.provider)
    }
    /// Get backups resource handler
    pub fn backups(&self) -> resources::Backups<'_> {
        resources::Backups::new(self.provider)
    }
    /// Get data_repository_task resource handler
    pub fn data_repository_task(&self) -> resources::Data_repository_task<'_> {
        resources::Data_repository_task::new(self.provider)
    }
    /// Get file_caches resource handler
    pub fn file_caches(&self) -> resources::File_caches<'_> {
        resources::File_caches::new(self.provider)
    }
    /// Get volume resource handler
    pub fn volume(&self) -> resources::Volume<'_> {
        resources::Volume::new(self.provider)
    }
    /// Get s3_access_point_attachments resource handler
    pub fn s3_access_point_attachments(&self) -> resources::S3_access_point_attachments<'_> {
        resources::S3_access_point_attachments::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get snapshots resource handler
    pub fn snapshots(&self) -> resources::Snapshots<'_> {
        resources::Snapshots::new(self.provider)
    }
    /// Get storage_virtual_machines resource handler
    pub fn storage_virtual_machines(&self) -> resources::Storage_virtual_machines<'_> {
        resources::Storage_virtual_machines::new(self.provider)
    }
    /// Get file_system resource handler
    pub fn file_system(&self) -> resources::File_system<'_> {
        resources::File_system::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get file_system_from_backup resource handler
    pub fn file_system_from_backup(&self) -> resources::File_system_from_backup<'_> {
        resources::File_system_from_backup::new(self.provider)
    }
    /// Get and_attach_s3_access_point resource handler
    pub fn and_attach_s3_access_point(&self) -> resources::And_attach_s3_access_point<'_> {
        resources::And_attach_s3_access_point::new(self.provider)
    }
    /// Get data_repository_association resource handler
    pub fn data_repository_association(&self) -> resources::Data_repository_association<'_> {
        resources::Data_repository_association::new(self.provider)
    }
    /// Get file_systems resource handler
    pub fn file_systems(&self) -> resources::File_systems<'_> {
        resources::File_systems::new(self.provider)
    }
    /// Get volumes resource handler
    pub fn volumes(&self) -> resources::Volumes<'_> {
        resources::Volumes::new(self.provider)
    }
    /// Get storage_virtual_machine resource handler
    pub fn storage_virtual_machine(&self) -> resources::Storage_virtual_machine<'_> {
        resources::Storage_virtual_machine::new(self.provider)
    }
    /// Get data_repository_tasks resource handler
    pub fn data_repository_tasks(&self) -> resources::Data_repository_tasks<'_> {
        resources::Data_repository_tasks::new(self.provider)
    }
    /// Get data_repository_associations resource handler
    pub fn data_repository_associations(&self) -> resources::Data_repository_associations<'_> {
        resources::Data_repository_associations::new(self.provider)
    }
    /// Get file_cache resource handler
    pub fn file_cache(&self) -> resources::File_cache<'_> {
        resources::File_cache::new(self.provider)
    }
    /// Get shared_vpc_configuration resource handler
    pub fn shared_vpc_configuration(&self) -> resources::Shared_vpc_configuration<'_> {
        resources::Shared_vpc_configuration::new(self.provider)
    }
    /// Get file_system_aliases resource handler
    pub fn file_system_aliases(&self) -> resources::File_system_aliases<'_> {
        resources::File_system_aliases::new(self.provider)
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
