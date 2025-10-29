//! Storage Service
//!
//! Auto-generated service module for storage

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for storage
pub struct StorageService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> StorageService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get tape_recovery_points resource handler
    pub fn tape_recovery_points(&self) -> resources::Tape_recovery_points<'_> {
        resources::Tape_recovery_points::new(self.provider)
    }
    /// Get snapshot_from_volume_recovery_point resource handler
    pub fn snapshot_from_volume_recovery_point(&self) -> resources::Snapshot_from_volume_recovery_point<'_> {
        resources::Snapshot_from_volume_recovery_point::new(self.provider)
    }
    /// Get cache_report resource handler
    pub fn cache_report(&self) -> resources::Cache_report<'_> {
        resources::Cache_report::new(self.provider)
    }
    /// Get file_system_association resource handler
    pub fn file_system_association(&self) -> resources::File_system_association<'_> {
        resources::File_system_association::new(self.provider)
    }
    /// Get file_system_associations resource handler
    pub fn file_system_associations(&self) -> resources::File_system_associations<'_> {
        resources::File_system_associations::new(self.provider)
    }
    /// Get bandwidth_rate_limit_schedule resource handler
    pub fn bandwidth_rate_limit_schedule(&self) -> resources::Bandwidth_rate_limit_schedule<'_> {
        resources::Bandwidth_rate_limit_schedule::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get tape_pool resource handler
    pub fn tape_pool(&self) -> resources::Tape_pool<'_> {
        resources::Tape_pool::new(self.provider)
    }
    /// Get storedi_scsivolumes resource handler
    pub fn storedi_scsivolumes(&self) -> resources::Storedi_scsivolumes<'_> {
        resources::Storedi_scsivolumes::new(self.provider)
    }
    /// Get file_share resource handler
    pub fn file_share(&self) -> resources::File_share<'_> {
        resources::File_share::new(self.provider)
    }
    /// Get vtldevice_type resource handler
    pub fn vtldevice_type(&self) -> resources::Vtldevice_type<'_> {
        resources::Vtldevice_type::new(self.provider)
    }
    /// Get gateway resource handler
    pub fn gateway(&self) -> resources::Gateway<'_> {
        resources::Gateway::new(self.provider)
    }
    /// Get availability_monitor_test resource handler
    pub fn availability_monitor_test(&self) -> resources::Availability_monitor_test<'_> {
        resources::Availability_monitor_test::new(self.provider)
    }
    /// Get smbfile_share resource handler
    pub fn smbfile_share(&self) -> resources::Smbfile_share<'_> {
        resources::Smbfile_share::new(self.provider)
    }
    /// Get tapes resource handler
    pub fn tapes(&self) -> resources::Tapes<'_> {
        resources::Tapes::new(self.provider)
    }
    /// Get bandwidth_rate_limit resource handler
    pub fn bandwidth_rate_limit(&self) -> resources::Bandwidth_rate_limit<'_> {
        resources::Bandwidth_rate_limit::new(self.provider)
    }
    /// Get smbsecurity_strategy resource handler
    pub fn smbsecurity_strategy(&self) -> resources::Smbsecurity_strategy<'_> {
        resources::Smbsecurity_strategy::new(self.provider)
    }
    /// Get maintenance_start_time resource handler
    pub fn maintenance_start_time(&self) -> resources::Maintenance_start_time<'_> {
        resources::Maintenance_start_time::new(self.provider)
    }
    /// Get upload_buffer resource handler
    pub fn upload_buffer(&self) -> resources::Upload_buffer<'_> {
        resources::Upload_buffer::new(self.provider)
    }
    /// Get cachedi_scsivolume resource handler
    pub fn cachedi_scsivolume(&self) -> resources::Cachedi_scsivolume<'_> {
        resources::Cachedi_scsivolume::new(self.provider)
    }
    /// Get vtldevices resource handler
    pub fn vtldevices(&self) -> resources::Vtldevices<'_> {
        resources::Vtldevices::new(self.provider)
    }
    /// Get nfsfile_share resource handler
    pub fn nfsfile_share(&self) -> resources::Nfsfile_share<'_> {
        resources::Nfsfile_share::new(self.provider)
    }
    /// Get chap_credentials resource handler
    pub fn chap_credentials(&self) -> resources::Chap_credentials<'_> {
        resources::Chap_credentials::new(self.provider)
    }
    /// Get tape_archive resource handler
    pub fn tape_archive(&self) -> resources::Tape_archive<'_> {
        resources::Tape_archive::new(self.provider)
    }
    /// Get tape resource handler
    pub fn tape(&self) -> resources::Tape<'_> {
        resources::Tape::new(self.provider)
    }
    /// Get tape_archives resource handler
    pub fn tape_archives(&self) -> resources::Tape_archives<'_> {
        resources::Tape_archives::new(self.provider)
    }
    /// Get storedi_scsivolume resource handler
    pub fn storedi_scsivolume(&self) -> resources::Storedi_scsivolume<'_> {
        resources::Storedi_scsivolume::new(self.provider)
    }
    /// Get tape_with_barcode resource handler
    pub fn tape_with_barcode(&self) -> resources::Tape_with_barcode<'_> {
        resources::Tape_with_barcode::new(self.provider)
    }
    /// Get cache resource handler
    pub fn cache(&self) -> resources::Cache<'_> {
        resources::Cache::new(self.provider)
    }
    /// Get smbfile_share_visibility resource handler
    pub fn smbfile_share_visibility(&self) -> resources::Smbfile_share_visibility<'_> {
        resources::Smbfile_share_visibility::new(self.provider)
    }
    /// Get smblocal_groups resource handler
    pub fn smblocal_groups(&self) -> resources::Smblocal_groups<'_> {
        resources::Smblocal_groups::new(self.provider)
    }
    /// Get gateway_software_now resource handler
    pub fn gateway_software_now(&self) -> resources::Gateway_software_now<'_> {
        resources::Gateway_software_now::new(self.provider)
    }
    /// Get gateway_information resource handler
    pub fn gateway_information(&self) -> resources::Gateway_information<'_> {
        resources::Gateway_information::new(self.provider)
    }
    /// Get snapshot_schedule resource handler
    pub fn snapshot_schedule(&self) -> resources::Snapshot_schedule<'_> {
        resources::Snapshot_schedule::new(self.provider)
    }
    /// Get volume resource handler
    pub fn volume(&self) -> resources::Volume<'_> {
        resources::Volume::new(self.provider)
    }
    /// Get cachedi_scsivolumes resource handler
    pub fn cachedi_scsivolumes(&self) -> resources::Cachedi_scsivolumes<'_> {
        resources::Cachedi_scsivolumes::new(self.provider)
    }
    /// Get nfsfile_shares resource handler
    pub fn nfsfile_shares(&self) -> resources::Nfsfile_shares<'_> {
        resources::Nfsfile_shares::new(self.provider)
    }
    /// Get smbfile_shares resource handler
    pub fn smbfile_shares(&self) -> resources::Smbfile_shares<'_> {
        resources::Smbfile_shares::new(self.provider)
    }
    /// Get smbsettings resource handler
    pub fn smbsettings(&self) -> resources::Smbsettings<'_> {
        resources::Smbsettings::new(self.provider)
    }
    /// Get automatic_tape_creation_policy resource handler
    pub fn automatic_tape_creation_policy(&self) -> resources::Automatic_tape_creation_policy<'_> {
        resources::Automatic_tape_creation_policy::new(self.provider)
    }
    /// Get working_storage resource handler
    pub fn working_storage(&self) -> resources::Working_storage<'_> {
        resources::Working_storage::new(self.provider)
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
