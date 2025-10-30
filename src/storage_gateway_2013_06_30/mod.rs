//! Storage_gateway_2013_06_30 Service
//!
//! Auto-generated service module for storage_gateway_2013_06_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for storage_gateway_2013_06_30
pub struct Storage_gateway_2013_06_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Storage_gateway_2013_06_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get upload_buffer resource handler
    pub fn upload_buffer(&self) -> resources::Upload_buffer<'_> {
        resources::Upload_buffer::new(self.provider)
    }
    /// Get tape_with_barcode resource handler
    pub fn tape_with_barcode(&self) -> resources::Tape_with_barcode<'_> {
        resources::Tape_with_barcode::new(self.provider)
    }
    /// Get vtl_device_type resource handler
    pub fn vtl_device_type(&self) -> resources::Vtl_device_type<'_> {
        resources::Vtl_device_type::new(self.provider)
    }
    /// Get smb_file_share resource handler
    pub fn smb_file_share(&self) -> resources::Smb_file_share<'_> {
        resources::Smb_file_share::new(self.provider)
    }
    /// Get snapshot_from_volume_recovery_point resource handler
    pub fn snapshot_from_volume_recovery_point(&self) -> resources::Snapshot_from_volume_recovery_point<'_> {
        resources::Snapshot_from_volume_recovery_point::new(self.provider)
    }
    /// Get gateway resource handler
    pub fn gateway(&self) -> resources::Gateway<'_> {
        resources::Gateway::new(self.provider)
    }
    /// Get gateway_information resource handler
    pub fn gateway_information(&self) -> resources::Gateway_information<'_> {
        resources::Gateway_information::new(self.provider)
    }
    /// Get working_storage resource handler
    pub fn working_storage(&self) -> resources::Working_storage<'_> {
        resources::Working_storage::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get gateway_software_now resource handler
    pub fn gateway_software_now(&self) -> resources::Gateway_software_now<'_> {
        resources::Gateway_software_now::new(self.provider)
    }
    /// Get file_share resource handler
    pub fn file_share(&self) -> resources::File_share<'_> {
        resources::File_share::new(self.provider)
    }
    /// Get automatic_tape_creation_policy resource handler
    pub fn automatic_tape_creation_policy(&self) -> resources::Automatic_tape_creation_policy<'_> {
        resources::Automatic_tape_creation_policy::new(self.provider)
    }
    /// Get tape resource handler
    pub fn tape(&self) -> resources::Tape<'_> {
        resources::Tape::new(self.provider)
    }
    /// Get tape_pool resource handler
    pub fn tape_pool(&self) -> resources::Tape_pool<'_> {
        resources::Tape_pool::new(self.provider)
    }
    /// Get tape_recovery_points resource handler
    pub fn tape_recovery_points(&self) -> resources::Tape_recovery_points<'_> {
        resources::Tape_recovery_points::new(self.provider)
    }
    /// Get smb_file_shares resource handler
    pub fn smb_file_shares(&self) -> resources::Smb_file_shares<'_> {
        resources::Smb_file_shares::new(self.provider)
    }
    /// Get availability_monitor_test resource handler
    pub fn availability_monitor_test(&self) -> resources::Availability_monitor_test<'_> {
        resources::Availability_monitor_test::new(self.provider)
    }
    /// Get smb_local_groups resource handler
    pub fn smb_local_groups(&self) -> resources::Smb_local_groups<'_> {
        resources::Smb_local_groups::new(self.provider)
    }
    /// Get cache_report resource handler
    pub fn cache_report(&self) -> resources::Cache_report<'_> {
        resources::Cache_report::new(self.provider)
    }
    /// Get bandwidth_rate_limit_schedule resource handler
    pub fn bandwidth_rate_limit_schedule(&self) -> resources::Bandwidth_rate_limit_schedule<'_> {
        resources::Bandwidth_rate_limit_schedule::new(self.provider)
    }
    /// Get nfs_file_shares resource handler
    pub fn nfs_file_shares(&self) -> resources::Nfs_file_shares<'_> {
        resources::Nfs_file_shares::new(self.provider)
    }
    /// Get smb_file_share_visibility resource handler
    pub fn smb_file_share_visibility(&self) -> resources::Smb_file_share_visibility<'_> {
        resources::Smb_file_share_visibility::new(self.provider)
    }
    /// Get volume resource handler
    pub fn volume(&self) -> resources::Volume<'_> {
        resources::Volume::new(self.provider)
    }
    /// Get cache resource handler
    pub fn cache(&self) -> resources::Cache<'_> {
        resources::Cache::new(self.provider)
    }
    /// Get tapes resource handler
    pub fn tapes(&self) -> resources::Tapes<'_> {
        resources::Tapes::new(self.provider)
    }
    /// Get cachedi_scsi_volumes resource handler
    pub fn cachedi_scsi_volumes(&self) -> resources::Cachedi_scsi_volumes<'_> {
        resources::Cachedi_scsi_volumes::new(self.provider)
    }
    /// Get nfs_file_share resource handler
    pub fn nfs_file_share(&self) -> resources::Nfs_file_share<'_> {
        resources::Nfs_file_share::new(self.provider)
    }
    /// Get storedi_scsi_volumes resource handler
    pub fn storedi_scsi_volumes(&self) -> resources::Storedi_scsi_volumes<'_> {
        resources::Storedi_scsi_volumes::new(self.provider)
    }
    /// Get tape_archives resource handler
    pub fn tape_archives(&self) -> resources::Tape_archives<'_> {
        resources::Tape_archives::new(self.provider)
    }
    /// Get snapshot_schedule resource handler
    pub fn snapshot_schedule(&self) -> resources::Snapshot_schedule<'_> {
        resources::Snapshot_schedule::new(self.provider)
    }
    /// Get chap_credentials resource handler
    pub fn chap_credentials(&self) -> resources::Chap_credentials<'_> {
        resources::Chap_credentials::new(self.provider)
    }
    /// Get storedi_scsi_volume resource handler
    pub fn storedi_scsi_volume(&self) -> resources::Storedi_scsi_volume<'_> {
        resources::Storedi_scsi_volume::new(self.provider)
    }
    /// Get maintenance_start_time resource handler
    pub fn maintenance_start_time(&self) -> resources::Maintenance_start_time<'_> {
        resources::Maintenance_start_time::new(self.provider)
    }
    /// Get vtl_devices resource handler
    pub fn vtl_devices(&self) -> resources::Vtl_devices<'_> {
        resources::Vtl_devices::new(self.provider)
    }
    /// Get smb_security_strategy resource handler
    pub fn smb_security_strategy(&self) -> resources::Smb_security_strategy<'_> {
        resources::Smb_security_strategy::new(self.provider)
    }
    /// Get file_system_association resource handler
    pub fn file_system_association(&self) -> resources::File_system_association<'_> {
        resources::File_system_association::new(self.provider)
    }
    /// Get bandwidth_rate_limit resource handler
    pub fn bandwidth_rate_limit(&self) -> resources::Bandwidth_rate_limit<'_> {
        resources::Bandwidth_rate_limit::new(self.provider)
    }
    /// Get file_system_associations resource handler
    pub fn file_system_associations(&self) -> resources::File_system_associations<'_> {
        resources::File_system_associations::new(self.provider)
    }
    /// Get cachedi_scsi_volume resource handler
    pub fn cachedi_scsi_volume(&self) -> resources::Cachedi_scsi_volume<'_> {
        resources::Cachedi_scsi_volume::new(self.provider)
    }
    /// Get smb_settings resource handler
    pub fn smb_settings(&self) -> resources::Smb_settings<'_> {
        resources::Smb_settings::new(self.provider)
    }
    /// Get tape_archive resource handler
    pub fn tape_archive(&self) -> resources::Tape_archive<'_> {
        resources::Tape_archive::new(self.provider)
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
