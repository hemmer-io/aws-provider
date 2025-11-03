# Fsx Service



**Resources**: 22

---

## Overview

The fsx service provides access to 22 resource types:

- [Storage_virtual_machines](#storage_virtual_machines) [R]
- [Volumes](#volumes) [R]
- [Volume](#volume) [CUD]
- [Data_repository_tasks](#data_repository_tasks) [R]
- [File_system](#file_system) [CUD]
- [Data_repository_association](#data_repository_association) [CUD]
- [Storage_virtual_machine](#storage_virtual_machine) [CUD]
- [Volume_from_backup](#volume_from_backup) [C]
- [S3_access_point_attachments](#s3_access_point_attachments) [R]
- [Snapshots](#snapshots) [R]
- [File_cache](#file_cache) [CUD]
- [Backup](#backup) [CD]
- [Data_repository_associations](#data_repository_associations) [R]
- [File_systems](#file_systems) [R]
- [Backups](#backups) [R]
- [File_system_aliases](#file_system_aliases) [R]
- [File_caches](#file_caches) [R]
- [Snapshot](#snapshot) [CUD]
- [Data_repository_task](#data_repository_task) [C]
- [And_attach_s3_access_point](#and_attach_s3_access_point) [C]
- [File_system_from_backup](#file_system_from_backup) [C]
- [Shared_vpc_configuration](#shared_vpc_configuration) [RU]

---

## Resources


### Storage_virtual_machines

StorageVirtualMachines resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `storage_virtual_machines` | Vec<String> | <p>Returned after a successful <code>DescribeStorageVirtualMachines</code> operation, describing each SVM.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access storage_virtual_machines outputs
storage_virtual_machines_id = storage_virtual_machines.id
storage_virtual_machines_next_token = storage_virtual_machines.next_token
storage_virtual_machines_storage_virtual_machines = storage_virtual_machines.storage_virtual_machines
```

---


### Volumes

Volumes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volumes` | Vec<String> | <p>Returned after a successful <code>DescribeVolumes</code> operation, describing each volume.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access volumes outputs
volumes_id = volumes.id
volumes_volumes = volumes.volumes
volumes_next_token = volumes.next_token
```

---


### Volume

Volume resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  |  |
| `name` | String | ✅ | <p>Specifies the name of the volume that you're creating.</p> |
| `ontap_configuration` | String |  | <p>Specifies the configuration to use when creating the ONTAP volume.</p> |
| `volume_type` | String | ✅ | <p>Specifies the type of volume to create; <code>ONTAP</code> and <code>OPENZFS</code> are
            the only valid volume types.</p> |
| `client_request_token` | String |  |  |
| `open_zfs_configuration` | String |  | <p>Specifies the configuration to use when creating the OpenZFS volume.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create volume
volume = provider.fsx.Volume {
    name = "value"  # <p>Specifies the name of the volume that you're creating.</p>
    volume_type = "value"  # <p>Specifies the type of volume to create; <code>ONTAP</code> and <code>OPENZFS</code> are
            the only valid volume types.</p>
}

```

---


### Data_repository_tasks

DataRepositoryTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_repository_tasks` | Vec<String> | <p>The collection of data repository task descriptions returned.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_repository_tasks outputs
data_repository_tasks_id = data_repository_tasks.id
data_repository_tasks_data_repository_tasks = data_repository_tasks.data_repository_tasks
data_repository_tasks_next_token = data_repository_tasks.next_token
```

---


### File_system

FileSystem resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `storage_type` | String |  | <p>Sets the storage class for the file system that you're creating. Valid values are
                <code>SSD</code>, <code>HDD</code>, and <code>INTELLIGENT_TIERING</code>.</p>
         <ul>
            <li>
               <p>Set to <code>SSD</code> to use solid state drive storage. SSD is supported on all Windows,
                    Lustre, ONTAP, and OpenZFS deployment types.</p>
            </li>
            <li>
               <p>Set to <code>HDD</code> to use hard disk drive storage, which is supported on 
                    <code>SINGLE_AZ_2</code> and <code>MULTI_AZ_1</code> Windows file system deployment types,
                and on <code>PERSISTENT_1</code> Lustre file system deployment types.</p>
            </li>
            <li>
               <p>Set to <code>INTELLIGENT_TIERING</code> to use fully elastic, intelligently-tiered storage.
                    Intelligent-Tiering is only available for OpenZFS file systems with the Multi-AZ deployment type
                    and for Lustre file systems with the Persistent_2 deployment type.</p>
            </li>
         </ul>
         <p>Default value is <code>SSD</code>. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/optimize-fsx-costs.html#storage-type-options"> Storage
                type options</a> in the <i>FSx for Windows File Server User
                Guide</i>, <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/using-fsx-lustre.html#lustre-storage-classes">FSx for Lustre storage classes</a>
            in the <i>FSx for Lustre User Guide</i>, and <a href="https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/performance-intelligent-tiering">Working with Intelligent-Tiering</a>
            in the <i>Amazon FSx for OpenZFS User Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>The tags to apply to the file system that's being created. The key value of the
                <code>Name</code> tag appears in the console as the file system name.</p> |
| `file_system_type` | String | ✅ | <p>The type of Amazon FSx file system to create. Valid values are
                <code>WINDOWS</code>, <code>LUSTRE</code>, <code>ONTAP</code>, and
                <code>OPENZFS</code>.</p> |
| `kms_key_id` | String |  |  |
| `subnet_ids` | Vec<String> | ✅ | <p>Specifies the IDs of the subnets that the file system will be accessible from. For
            Windows and ONTAP <code>MULTI_AZ_1</code> deployment types,provide exactly two subnet
            IDs, one for the preferred file server and one for the standby file server. You specify
            one of these subnets as the preferred subnet using the <code>WindowsConfiguration >
                PreferredSubnetID</code> or <code>OntapConfiguration > PreferredSubnetID</code>
            properties. For more information about Multi-AZ file system configuration, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/high-availability-multiAZ.html">
                Availability and durability: Single-AZ and Multi-AZ file systems</a> in the
                <i>Amazon FSx for Windows User Guide</i> and <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/high-availability-multiAZ.html">
                Availability and durability</a> in the <i>Amazon FSx for ONTAP User
                Guide</i>.</p>
         <p>For Windows <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> and all Lustre 
            deployment types, provide exactly one subnet ID.
           The file server is launched in that subnet's Availability Zone.</p> |
| `open_zfs_configuration` | String |  | <p>The OpenZFS configuration for the file system that's being created.</p> |
| `windows_configuration` | String |  | <p>The Microsoft Windows configuration for the file system that's being created.</p> |
| `client_request_token` | String |  | <p>A string of up to 63 ASCII characters that Amazon FSx uses to ensure
            idempotent creation. This string is automatically filled on your behalf when you use the
                Command Line Interface (CLI) or an Amazon Web Services SDK.</p> |
| `lustre_configuration` | String |  |  |
| `ontap_configuration` | String |  |  |
| `security_group_ids` | Vec<String> |  | <p>A list of IDs specifying the security groups to apply to all network interfaces
            created for file system access. This list isn't returned in later requests to
            describe the file system.</p>
         <important>
            <p>You must specify a security group if you are creating a Multi-AZ 
            FSx for ONTAP file system in a VPC subnet that has been shared with you.</p>
         </important> |
| `file_system_type_version` | String |  | <p>For FSx for Lustre file systems, sets the Lustre version for the file system
            that you're creating. Valid values are <code>2.10</code>, <code>2.12</code>, and
            <code>2.15</code>:</p>
         <ul>
            <li>
               <p>
                  <code>2.10</code> is supported by the Scratch and Persistent_1 Lustre 
                deployment types.</p>
            </li>
            <li>
               <p>
                  <code>2.12</code> is supported by all Lustre deployment types, except
                for <code>PERSISTENT_2</code> with a metadata configuration mode.</p>
            </li>
            <li>
               <p>
                  <code>2.15</code> is supported by all Lustre deployment types and is
                recommended for all new file systems.</p>
            </li>
         </ul>
         <p>Default value is <code>2.10</code>, except for the following deployments:</p>
         <ul>
            <li>
               <p>Default value is <code>2.12</code> when <code>DeploymentType</code> is set to 
                <code>PERSISTENT_2</code> without a metadata configuration mode.</p>
            </li>
            <li>
               <p>Default value is <code>2.15</code> when <code>DeploymentType</code> is set to 
                <code>PERSISTENT_2</code> with a metadata configuration mode.</p>
            </li>
         </ul> |
| `network_type` | String |  | <p>The network type of the Amazon FSx file system that you
            are creating. Valid values are <code>IPV4</code> (which supports
            IPv4 only) and <code>DUAL</code> (for dual-stack mode, which supports
            both IPv4 and IPv6). The default is <code>IPV4</code>. Supported
            for FSx for OpenZFS, FSx for ONTAP, and FSx for Windows File Server
            file systems.</p> |
| `storage_capacity` | i64 |  | <p>Sets the storage capacity of the file system that you're creating, in gibibytes (GiB).</p>
         <p>
            <b>FSx for Lustre file systems</b> - The amount of
            storage capacity that you can configure depends on the value that you set for
                <code>StorageType</code> and the Lustre <code>DeploymentType</code>, as
            follows:</p>
         <ul>
            <li>
               <p>For <code>SCRATCH_2</code>, <code>PERSISTENT_2</code>, and <code>PERSISTENT_1</code> deployment types 
                using SSD storage type, the valid values are 1200 GiB, 2400 GiB, and increments of 2400 GiB.</p>
            </li>
            <li>
               <p>For <code>PERSISTENT_1</code> HDD file systems, valid values are increments of 6000 GiB for 
                12 MB/s/TiB file systems and increments of 1800 GiB for 40 MB/s/TiB file systems.</p>
            </li>
            <li>
               <p>For <code>SCRATCH_1</code> deployment type, valid values are 
                1200 GiB, 2400 GiB, and increments of 3600 GiB.</p>
            </li>
         </ul>
         <p>
            <b>FSx for ONTAP file systems</b> - The amount of storage capacity 
            that you can configure depends on the value of the <code>HAPairs</code> property. The minimum value is calculated as 1,024 * <code>HAPairs</code> and the maximum is calculated as 524,288 * <code>HAPairs</code>. </p>
         <p>
            <b>FSx for OpenZFS file systems</b> - The amount of storage capacity that 
            you can configure is from 64 GiB up to 524,288 GiB (512 TiB).</p>
         <p>
            <b>FSx for Windows File Server file systems</b> - The amount
            of storage capacity that you can configure depends on the value that you set for
                <code>StorageType</code> as follows:</p>
         <ul>
            <li>
               <p>For SSD storage, valid values are 32 GiB-65,536 GiB (64 TiB).</p>
            </li>
            <li>
               <p>For HDD storage, valid values are 2000 GiB-65,536 GiB (64 TiB).</p>
            </li>
         </ul> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create file_system
file_system = provider.fsx.File_system {
    file_system_type = "value"  # <p>The type of Amazon FSx file system to create. Valid values are
                <code>WINDOWS</code>, <code>LUSTRE</code>, <code>ONTAP</code>, and
                <code>OPENZFS</code>.</p>
    subnet_ids = "value"  # <p>Specifies the IDs of the subnets that the file system will be accessible from. For
            Windows and ONTAP <code>MULTI_AZ_1</code> deployment types,provide exactly two subnet
            IDs, one for the preferred file server and one for the standby file server. You specify
            one of these subnets as the preferred subnet using the <code>WindowsConfiguration >
                PreferredSubnetID</code> or <code>OntapConfiguration > PreferredSubnetID</code>
            properties. For more information about Multi-AZ file system configuration, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/high-availability-multiAZ.html">
                Availability and durability: Single-AZ and Multi-AZ file systems</a> in the
                <i>Amazon FSx for Windows User Guide</i> and <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/high-availability-multiAZ.html">
                Availability and durability</a> in the <i>Amazon FSx for ONTAP User
                Guide</i>.</p>
         <p>For Windows <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> and all Lustre 
            deployment types, provide exactly one subnet ID.
           The file server is launched in that subnet's Availability Zone.</p>
}

```

---


### Data_repository_association

DataRepositoryAssociation resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_system_path` | String |  | <p>A path on the file system that points to a high-level directory (such
            as <code>/ns1/</code>) or subdirectory (such as <code>/ns1/subdir/</code>)
            that will be mapped 1-1 with <code>DataRepositoryPath</code>.
            The leading forward slash in the name is required. Two data repository
            associations cannot have overlapping file system paths. For example, if
            a data repository is associated with file system path <code>/ns1/</code>,
            then you cannot link another data repository with file system
            path <code>/ns1/ns2</code>.</p>
         <p>This path specifies where in your file system files will be exported
            from or imported to. This file system directory can be linked to only one
            Amazon S3 bucket, and no other S3 bucket can be linked to the directory.</p>
         <note>
            <p>If you specify only a forward slash (<code>/</code>) as the file system
            path, you can link only one data repository to the file system. You can only specify
            "/" as the file system path for the first data repository associated with a file system.</p>
         </note> |
| `batch_import_meta_data_on_create` | bool |  | <p>Set to <code>true</code> to run an import data repository task to import
            metadata from the data repository to the file system after the data repository
            association is created. Default is <code>false</code>.</p> |
| `imported_file_chunk_size` | i64 |  | <p>For files imported from a data repository, this value determines the stripe count and
            maximum amount of data per file (in MiB) stored on a single physical disk. The maximum
            number of disks that a single file can be striped across is limited by the total number
            of disks that make up the file system.</p>
         <p>The default chunk size is 1,024 MiB (1 GiB) and can go as high as 512,000 MiB (500
            GiB). Amazon S3 objects have a maximum size of 5 TB.</p> |
| `client_request_token` | String |  |  |
| `s3` | String |  | <p>The configuration for an Amazon S3 data repository linked to an
            Amazon FSx Lustre file system with a data repository association.
            The configuration defines which file events (new, changed, or
            deleted files or directories) are automatically imported from
            the linked data repository to the file system or automatically
            exported from the file system to the data repository.</p> |
| `data_repository_path` | String | ✅ | <p>The path to the Amazon S3 data repository that will be linked to the file
            system. The path can be an S3 bucket or prefix in the format
            <code>s3://bucket-name/prefix/</code> (where <code>prefix</code>
            is optional). This path specifies where in the S3 data repository
            files will be imported from or exported to.</p> |
| `file_system_id` | String | ✅ |  |
| `tags` | Vec<String> |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_repository_association
data_repository_association = provider.fsx.Data_repository_association {
    data_repository_path = "value"  # <p>The path to the Amazon S3 data repository that will be linked to the file
            system. The path can be an S3 bucket or prefix in the format
            <code>s3://bucket-name/prefix/</code> (where <code>prefix</code>
            is optional). This path specifies where in the S3 data repository
            files will be imported from or exported to.</p>
    file_system_id = "value"  # Required field
}

```

---


### Storage_virtual_machine

StorageVirtualMachine resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  |  |
| `svm_admin_password` | String |  | <p>The password to use when managing the SVM using the NetApp ONTAP CLI or REST API.
            If you do not specify a password, you can still use the file system's
            <code>fsxadmin</code> user to manage the SVM.</p> |
| `root_volume_security_style` | String |  | <p>The security style of the root volume of the SVM. Specify one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>UNIX</code> if the file system is managed by a UNIX
                administrator, the majority of users are NFS clients, and an application
                accessing the data uses a UNIX user as the service account.</p>
            </li>
            <li>
               <p>
                  <code>NTFS</code> if the file system is managed by a Microsoft Windows
                administrator, the majority of users are SMB clients, and an application
                accessing the data uses a Microsoft Windows user as the service account.</p>
            </li>
            <li>
               <p>
                  <code>MIXED</code> This is an advanced setting. For more information, see 
                <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/volume-security-style.html">Volume security style</a>
                in the Amazon FSx for NetApp ONTAP User Guide.</p>
            </li>
         </ul>
         <p></p> |
| `tags` | Vec<String> |  |  |
| `file_system_id` | String | ✅ |  |
| `name` | String | ✅ | <p>The name of the SVM.</p> |
| `active_directory_configuration` | String |  | <p>Describes the self-managed Microsoft Active Directory to which you want to join the SVM. 
      Joining an Active Directory provides user authentication and access control for SMB clients, 
      including Microsoft Windows and macOS clients accessing the file system.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create storage_virtual_machine
storage_virtual_machine = provider.fsx.Storage_virtual_machine {
    file_system_id = "value"  # Required field
    name = "value"  # <p>The name of the SVM.</p>
}

```

---


### Volume_from_backup

VolumeFromBackup resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  |  |
| `tags` | Vec<String> |  |  |
| `backup_id` | String | ✅ |  |
| `name` | String | ✅ | <p>The name of the new volume you're creating.</p> |
| `ontap_configuration` | String |  | <p>Specifies the configuration of the ONTAP volume that you are creating.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create volume_from_backup
volume_from_backup = provider.fsx.Volume_from_backup {
    backup_id = "value"  # Required field
    name = "value"  # <p>The name of the new volume you're creating.</p>
}

```

---


### S3_access_point_attachments

S3AccessPointAttachments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `s3_access_point_attachments` | Vec<String> | <p>Array of S3 access point attachments returned after a successful <code>DescribeS3AccessPointAttachments</code> operation.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access s3_access_point_attachments outputs
s3_access_point_attachments_id = s3_access_point_attachments.id
s3_access_point_attachments_s3_access_point_attachments = s3_access_point_attachments.s3_access_point_attachments
s3_access_point_attachments_next_token = s3_access_point_attachments.next_token
```

---


### Snapshots

Snapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshots` | Vec<String> | <p>An array of snapshots.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshots outputs
snapshots_id = snapshots.id
snapshots_snapshots = snapshots.snapshots
snapshots_next_token = snapshots.next_token
```

---


### File_cache

FileCache resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_cache_type` | String | ✅ | <p>The type of cache that you're creating, which
            must be <code>LUSTRE</code>.</p> |
| `tags` | Vec<String> |  |  |
| `storage_capacity` | i64 | ✅ | <p>The storage capacity of the cache in gibibytes (GiB). Valid values
            are 1200 GiB, 2400 GiB, and increments of 2400 GiB.</p> |
| `subnet_ids` | Vec<String> | ✅ |  |
| `security_group_ids` | Vec<String> |  | <p>A list of IDs specifying the security groups to apply to all network interfaces
            created for Amazon File Cache access. This list isn't returned in later requests to
            describe the cache.</p> |
| `lustre_configuration` | String |  | <p>The configuration for the Amazon File Cache resource being created.</p> |
| `file_cache_type_version` | String | ✅ | <p>Sets the Lustre version for the cache that you're creating,
            which must be <code>2.12</code>.</p> |
| `kms_key_id` | String |  | <p>Specifies the ID of the Key Management Service (KMS) key to use for encrypting data on
            an Amazon File Cache. If a <code>KmsKeyId</code> isn't specified, the Amazon FSx-managed
            KMS key for your account is used. For more information,
            see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html">Encrypt</a> in the
            <i>Key Management Service API Reference</i>.</p> |
| `copy_tags_to_data_repository_associations` | bool |  | <p>A boolean flag indicating whether tags for the cache should be copied to
            data repository associations. This value defaults to false.</p> |
| `client_request_token` | String |  | <p>An idempotency token for resource creation, in a string of up to 63
            ASCII characters. This token is automatically filled on your behalf when you use the
            Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
         <p>By using the idempotent operation, you can retry a <code>CreateFileCache</code>
            operation without the risk of creating an extra cache. This approach can be useful
            when an initial call fails in a way that makes it unclear whether a cache was created.
            Examples are if a transport level timeout occurred, or your connection was reset.
            If you use the same client request token and the initial call created a cache, the
            client receives success as long as the parameters are the same.</p> |
| `data_repository_associations` | Vec<String> |  | <p>A list of up to 8 configurations for data repository associations (DRAs) to
            be created during the cache creation. The DRAs link the cache to either an
            Amazon S3 data repository or a Network File System (NFS) data repository that supports the NFSv3 protocol.</p>
         <p>The DRA configurations must meet the following requirements:</p>
         <ul>
            <li>
               <p>All configurations on the list must be of the
                same data repository type, either all S3 or all NFS. A cache
                can't link to different data repository types at the same time.</p>
            </li>
            <li>
               <p>An NFS DRA must link to an NFS file system that
                supports the NFSv3 protocol.</p>
            </li>
         </ul>
         <p>DRA automatic import and automatic export is not supported.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create file_cache
file_cache = provider.fsx.File_cache {
    file_cache_type = "value"  # <p>The type of cache that you're creating, which
            must be <code>LUSTRE</code>.</p>
    storage_capacity = "value"  # <p>The storage capacity of the cache in gibibytes (GiB). Valid values
            are 1200 GiB, 2400 GiB, and increments of 2400 GiB.</p>
    subnet_ids = "value"  # Required field
    file_cache_type_version = "value"  # <p>Sets the Lustre version for the cache that you're creating,
            which must be <code>2.12</code>.</p>
}

```

---


### Backup

Backup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>(Optional) A string of up to 63 ASCII characters that Amazon FSx uses to
            ensure idempotent creation. This string is automatically filled on your behalf when you
            use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p> |
| `tags` | Vec<String> |  | <p>(Optional) The tags to apply to the backup at backup creation. The key value of the
                <code>Name</code> tag appears in the console as the backup name. If you have set
                <code>CopyTagsToBackups</code> to <code>true</code>, and you specify one or more
            tags using the <code>CreateBackup</code> operation, no existing file system tags are
            copied from the file system to the backup.</p> |
| `volume_id` | String |  | <p>(Optional) The ID of the FSx for ONTAP volume to back up.</p> |
| `file_system_id` | String |  | <p>The ID of the file system to back up.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup
backup = provider.fsx.Backup {
}

```

---


### Data_repository_associations

DataRepositoryAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>An array of one or more data repository association descriptions.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_repository_associations outputs
data_repository_associations_id = data_repository_associations.id
data_repository_associations_associations = data_repository_associations.associations
data_repository_associations_next_token = data_repository_associations.next_token
```

---


### File_systems

FileSystems resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_systems` | Vec<String> | <p>An array of file system descriptions.</p> |
| `next_token` | String | <p>Present if there are more file systems than returned in the response (String). You
            can use the <code>NextToken</code> value in the later request to fetch the
            descriptions. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access file_systems outputs
file_systems_id = file_systems.id
file_systems_file_systems = file_systems.file_systems
file_systems_next_token = file_systems.next_token
```

---


### Backups

Backups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A <code>NextToken</code> value is present if there are more backups than returned in
            the response. You can use the <code>NextToken</code> value in the subsequent request to
            fetch the backups. </p> |
| `backups` | Vec<String> | <p>An array of backups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access backups outputs
backups_id = backups.id
backups_next_token = backups.next_token
backups_backups = backups.backups
```

---


### File_system_aliases

FileSystemAliases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aliases` | Vec<String> | <p>An array of one or more DNS aliases currently associated with the specified file system.</p> |
| `next_token` | String | <p>Present if there are more DNS aliases than returned in the response (String). You
            can use the <code>NextToken</code> value in a later request to fetch additional
            descriptions. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access file_system_aliases outputs
file_system_aliases_id = file_system_aliases.id
file_system_aliases_aliases = file_system_aliases.aliases
file_system_aliases_next_token = file_system_aliases.next_token
```

---


### File_caches

FileCaches resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_caches` | Vec<String> | <p>The response object for the <code>DescribeFileCaches</code> operation.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access file_caches outputs
file_caches_id = file_caches.id
file_caches_file_caches = file_caches.file_caches
file_caches_next_token = file_caches.next_token
```

---


### Snapshot

Snapshot resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_id` | String | ✅ | <p>The ID of the volume that you are taking a snapshot of.</p> |
| `name` | String | ✅ | <p>The name of the snapshot. </p> |
| `client_request_token` | String |  |  |
| `tags` | Vec<String> |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot
snapshot = provider.fsx.Snapshot {
    volume_id = "value"  # <p>The ID of the volume that you are taking a snapshot of.</p>
    name = "value"  # <p>The name of the snapshot. </p>
}

```

---


### Data_repository_task

DataRepositoryTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>Specifies the type of data repository task to create.</p>
         <ul>
            <li>
               <p>
                  <code>EXPORT_TO_REPOSITORY</code> tasks export from your
                Amazon FSx for Lustre file system to a linked data repository.</p>
            </li>
            <li>
               <p>
                  <code>IMPORT_METADATA_FROM_REPOSITORY</code> tasks import metadata
                changes from a linked S3 bucket to your Amazon FSx for Lustre file system.</p>
            </li>
            <li>
               <p>
                  <code>RELEASE_DATA_FROM_FILESYSTEM</code> tasks release files in
                your Amazon FSx for Lustre file system that have been exported to a linked
                S3 bucket and that meet your specified release criteria.</p>
            </li>
            <li>
               <p>
                  <code>AUTO_RELEASE_DATA</code> tasks automatically release files from
                an Amazon File Cache resource.</p>
            </li>
         </ul> |
| `paths` | Vec<String> |  | <p>A list of paths for the data repository task to use when the task is processed.
            If a path that you provide isn't valid, the task fails. If you don't provide
            paths, the default behavior is to export all files to S3 (for export tasks), import
            all files from S3 (for import tasks), or release all exported files that meet the
            last accessed time criteria (for release tasks).</p>
         <ul>
            <li>
               <p>For export tasks, the list contains paths on the FSx for Lustre file system
                from which the files are exported to the Amazon S3 bucket. The default path is the
                file system root directory. The paths you provide need to be relative to the mount
                point of the file system. If the mount point is <code>/mnt/fsx</code> and
                <code>/mnt/fsx/path1</code> is a directory or file on the file system you want
                to export, then the path to provide is <code>path1</code>.</p>
            </li>
            <li>
               <p>For import tasks, the list contains paths in the Amazon S3 bucket
                from which POSIX metadata changes are imported to the FSx for Lustre file system.
                The path can be an S3 bucket or prefix in the format
                <code>s3://bucket-name/prefix</code> (where <code>prefix</code> is optional).</p>
            </li>
            <li>
               <p>For release tasks, the list contains directory or file paths on the
                FSx for Lustre file system from which to release exported files. If a directory is
                specified, files within the directory are released. If a file path is specified,
                only that file is released. To release all exported files in the file system,
                specify a forward slash (/) as the path.</p>
               <note>
                  <p>A file must also meet the last accessed time criteria
                    specified in  for the
                    file to be released.</p>
               </note>
            </li>
         </ul> |
| `capacity_to_release` | i64 |  | <p>Specifies the amount of data to release, in GiB, by an Amazon File Cache
            <code>AUTO_RELEASE_DATA</code> task that automatically releases files from the cache.</p> |
| `report` | String | ✅ | <p>Defines whether or not Amazon FSx provides a CompletionReport once the task has completed. 
            A CompletionReport provides a detailed  report on the files that Amazon FSx processed that meet the criteria specified by the 
            <code>Scope</code> parameter. For more information, see 
            <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/task-completion-report.html">Working with Task Completion Reports</a>.</p> |
| `file_system_id` | String | ✅ |  |
| `release_configuration` | String |  | <p>The configuration that specifies the last accessed time criteria for files
            that will be released from an Amazon FSx for Lustre file system.</p> |
| `client_request_token` | String |  |  |
| `tags` | Vec<String> |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_repository_task
data_repository_task = provider.fsx.Data_repository_task {
    type = "value"  # <p>Specifies the type of data repository task to create.</p>
         <ul>
            <li>
               <p>
                  <code>EXPORT_TO_REPOSITORY</code> tasks export from your
                Amazon FSx for Lustre file system to a linked data repository.</p>
            </li>
            <li>
               <p>
                  <code>IMPORT_METADATA_FROM_REPOSITORY</code> tasks import metadata
                changes from a linked S3 bucket to your Amazon FSx for Lustre file system.</p>
            </li>
            <li>
               <p>
                  <code>RELEASE_DATA_FROM_FILESYSTEM</code> tasks release files in
                your Amazon FSx for Lustre file system that have been exported to a linked
                S3 bucket and that meet your specified release criteria.</p>
            </li>
            <li>
               <p>
                  <code>AUTO_RELEASE_DATA</code> tasks automatically release files from
                an Amazon File Cache resource.</p>
            </li>
         </ul>
    report = "value"  # <p>Defines whether or not Amazon FSx provides a CompletionReport once the task has completed. 
            A CompletionReport provides a detailed  report on the files that Amazon FSx processed that meet the criteria specified by the 
            <code>Scope</code> parameter. For more information, see 
            <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/task-completion-report.html">Working with Task Completion Reports</a>.</p>
    file_system_id = "value"  # Required field
}

```

---


### And_attach_s3_access_point

AndAttachS3AccessPoint resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name you want to assign to this S3 access point.</p> |
| `client_request_token` | String |  |  |
| `type` | String | ✅ | <p>The type of S3 access point you want to create. Only <code>OpenZFS</code> is supported.</p> |
| `s3_access_point` | String |  | <p>Specifies the virtual private cloud (VPC) configuration if you're creating an access point that is restricted to a VPC. 
         For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/access-points-vpc.html">Creating access points restricted to a virtual private cloud</a>.</p> |
| `open_zfs_configuration` | String |  | <p>Specifies the configuration to use when creating and attaching an S3 access point to an FSx for OpenZFS volume.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create and_attach_s3_access_point
and_attach_s3_access_point = provider.fsx.And_attach_s3_access_point {
    name = "value"  # <p>The name you want to assign to this S3 access point.</p>
    type = "value"  # <p>The type of S3 access point you want to create. Only <code>OpenZFS</code> is supported.</p>
}

```

---


### File_system_from_backup

FileSystemFromBackup resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network_type` | String |  | <p>Sets the network type for the Amazon FSx for OpenZFS file system
            that you're creating from a backup.</p> |
| `client_request_token` | String |  | <p>A string of up to 63 ASCII characters that Amazon FSx uses to ensure
            idempotent creation. This string is automatically filled on your behalf when you use the
                Command Line Interface (CLI) or an Amazon Web Services SDK.</p> |
| `tags` | Vec<String> |  | <p>The tags to be applied to the file system at file system creation. The key value of
            the <code>Name</code> tag appears in the console as the file system
            name.</p> |
| `lustre_configuration` | String |  |  |
| `file_system_type_version` | String |  | <p>Sets the version for the Amazon FSx for Lustre file system that you're
            creating from a backup. Valid values are <code>2.10</code>, <code>2.12</code>,
            and <code>2.15</code>.</p>
         <p>You can enter a Lustre version that is newer than the backup's
            <code>FileSystemTypeVersion</code> setting. If you don't enter a newer Lustre version,
            it defaults to the backup's setting.</p> |
| `security_group_ids` | Vec<String> |  | <p>A list of IDs for the security groups that apply to the specified network interfaces
            created for file system access. These security groups apply to all network interfaces.
            This value isn't returned in later <code>DescribeFileSystem</code> requests.</p> |
| `storage_type` | String |  | <p>Sets the storage type for the Windows, OpenZFS, or Lustre file system that you're creating from
            a backup. Valid values are <code>SSD</code>, <code>HDD</code>, and <code>INTELLIGENT_TIERING</code>.</p>
         <ul>
            <li>
               <p>Set to <code>SSD</code> to use solid state drive storage. SSD is supported on all Windows and OpenZFS
                    deployment types.</p>
            </li>
            <li>
               <p>Set to <code>HDD</code> to use hard disk drive storage. 
                    HDD is supported on <code>SINGLE_AZ_2</code> and <code>MULTI_AZ_1</code> FSx for Windows File Server file system deployment types.</p>
            </li>
            <li>
               <p>Set to <code>INTELLIGENT_TIERING</code> to use fully elastic, intelligently-tiered storage.
                    Intelligent-Tiering is only available for OpenZFS file systems with the Multi-AZ deployment type
                    and for Lustre file systems with the Persistent_2 deployment type.</p>
            </li>
         </ul>
         <p> The default value is <code>SSD</code>. </p>
         <note>
            <p>HDD and SSD storage types have different minimum storage capacity requirements. 
            A restored file system's storage capacity is tied to the file system that was backed up. 
            You can create a file system that uses HDD storage from a backup of a file system that 
            used SSD storage if the original SSD file system had a storage capacity of at least 2000 GiB.</p>
         </note> |
| `subnet_ids` | Vec<String> | ✅ | <p>Specifies the IDs of the subnets that the file system will be accessible from. For Windows <code>MULTI_AZ_1</code> 
            file system deployment types, provide exactly two subnet IDs, one for the preferred file server 
            and one for the standby file server. You specify one of these subnets as the preferred subnet 
            using the <code>WindowsConfiguration > PreferredSubnetID</code> property.</p>
         <p>Windows <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> file system deployment
            types, Lustre file systems, and OpenZFS file systems provide exactly one subnet ID. The
            file server is launched in that subnet's Availability Zone.</p> |
| `open_zfs_configuration` | String |  | <p>The OpenZFS configuration for the file system that's being created. </p> |
| `windows_configuration` | String |  | <p>The configuration for this Microsoft Windows file system.</p> |
| `storage_capacity` | i64 |  | <p>Sets the storage capacity of the OpenZFS file system that you're creating
            from a backup, in gibibytes (GiB). Valid values are from 64 GiB up to 524,288 GiB
            (512 TiB). However, the value that you specify must be equal to or greater than the
            backup's storage capacity value. If you don't use the <code>StorageCapacity</code>
            parameter, the default is the backup's <code>StorageCapacity</code> value.</p>
         <p>If used to create a file system other than OpenZFS, you must provide a value
            that matches the backup's <code>StorageCapacity</code> value. If you provide any
            other value, Amazon FSx responds with an HTTP status code 400 Bad Request. </p> |
| `backup_id` | String | ✅ |  |
| `kms_key_id` | String |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create file_system_from_backup
file_system_from_backup = provider.fsx.File_system_from_backup {
    subnet_ids = "value"  # <p>Specifies the IDs of the subnets that the file system will be accessible from. For Windows <code>MULTI_AZ_1</code> 
            file system deployment types, provide exactly two subnet IDs, one for the preferred file server 
            and one for the standby file server. You specify one of these subnets as the preferred subnet 
            using the <code>WindowsConfiguration > PreferredSubnetID</code> property.</p>
         <p>Windows <code>SINGLE_AZ_1</code> and <code>SINGLE_AZ_2</code> file system deployment
            types, Lustre file systems, and OpenZFS file systems provide exactly one subnet ID. The
            file server is launched in that subnet's Availability Zone.</p>
    backup_id = "value"  # Required field
}

```

---


### Shared_vpc_configuration

SharedVpcConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  |  |
| `enable_fsx_route_table_updates_from_participant_accounts` | String |  | <p>Specifies whether participant accounts can create FSx for ONTAP Multi-AZ
            file systems in shared subnets. Set to <code>true</code> to enable or <code>false</code>
            to disable.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_fsx_route_table_updates_from_participant_accounts` | String | <p>Indicates whether participant accounts can create FSx for ONTAP Multi-AZ file systems in shared subnets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access shared_vpc_configuration outputs
shared_vpc_configuration_id = shared_vpc_configuration.id
shared_vpc_configuration_enable_fsx_route_table_updates_from_participant_accounts = shared_vpc_configuration.enable_fsx_route_table_updates_from_participant_accounts
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple storage_virtual_machines resources
storage_virtual_machines_0 = provider.fsx.Storage_virtual_machines {
}
storage_virtual_machines_1 = provider.fsx.Storage_virtual_machines {
}
storage_virtual_machines_2 = provider.fsx.Storage_virtual_machines {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    storage_virtual_machines = provider.fsx.Storage_virtual_machines {
    }
```

---

## Related Documentation

- [AWS Fsx Documentation](https://docs.aws.amazon.com/fsx/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
