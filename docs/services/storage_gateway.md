# Storage_gateway Service



**Resources**: 41

---

## Overview

The storage_gateway service provides access to 41 resource types:

- [Snapshot](#snapshot) [C]
- [Bandwidth_rate_limit_schedule](#bandwidth_rate_limit_schedule) [RU]
- [File_system_associations](#file_system_associations) [R]
- [Working_storage](#working_storage) [R]
- [Chap_credentials](#chap_credentials) [RUD]
- [Snapshot_from_volume_recovery_point](#snapshot_from_volume_recovery_point) [C]
- [Upload_buffer](#upload_buffer) [R]
- [Storedi_scsi_volumes](#storedi_scsi_volumes) [R]
- [Nfs_file_share](#nfs_file_share) [CU]
- [Bandwidth_rate_limit](#bandwidth_rate_limit) [RUD]
- [Storedi_scsi_volume](#storedi_scsi_volume) [C]
- [Tape_archive](#tape_archive) [D]
- [Tape_archives](#tape_archives) [R]
- [Gateway_software_now](#gateway_software_now) [U]
- [Smb_local_groups](#smb_local_groups) [U]
- [Volume](#volume) [D]
- [Cache_report](#cache_report) [RD]
- [Tapes](#tapes) [CR]
- [Tape_pool](#tape_pool) [CD]
- [Tape_recovery_points](#tape_recovery_points) [R]
- [Smb_file_shares](#smb_file_shares) [R]
- [Cachedi_scsi_volumes](#cachedi_scsi_volumes) [R]
- [Availability_monitor_test](#availability_monitor_test) [R]
- [Smb_security_strategy](#smb_security_strategy) [U]
- [Gateway_information](#gateway_information) [RU]
- [File_share](#file_share) [D]
- [Nfs_file_shares](#nfs_file_shares) [R]
- [Automatic_tape_creation_policy](#automatic_tape_creation_policy) [UD]
- [Smb_settings](#smb_settings) [R]
- [Tape](#tape) [D]
- [Vtl_devices](#vtl_devices) [R]
- [Cache](#cache) [R]
- [Smb_file_share_visibility](#smb_file_share_visibility) [U]
- [File_system_association](#file_system_association) [U]
- [Gateway](#gateway) [D]
- [Maintenance_start_time](#maintenance_start_time) [RU]
- [Cachedi_scsi_volume](#cachedi_scsi_volume) [C]
- [Smb_file_share](#smb_file_share) [CU]
- [Tape_with_barcode](#tape_with_barcode) [C]
- [Snapshot_schedule](#snapshot_schedule) [RUD]
- [Vtl_device_type](#vtl_device_type) [U]

---

## Resources


### Snapshot

Snapshot resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a>
         operation to return a list of gateway volumes.</p> |
| `snapshot_description` | String | ✅ | <p>Textual description of the snapshot that appears in the Amazon EC2 console, Elastic
         Block Store snapshots panel in the <b>Description</b> field, and
         in the Storage Gateway snapshot <b>Details</b> pane,
            <b>Description</b> field.</p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to a snapshot. Each tag is a key-value
         pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot
snapshot = provider.storage_gateway.Snapshot {
    volume_arn = "value"  # <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a>
         operation to return a list of gateway volumes.</p>
    snapshot_description = "value"  # <p>Textual description of the snapshot that appears in the Amazon EC2 console, Elastic
         Block Store snapshots panel in the <b>Description</b> field, and
         in the Storage Gateway snapshot <b>Details</b> pane,
            <b>Description</b> field.</p>
}

```

---


### Bandwidth_rate_limit_schedule

BandwidthRateLimitSchedule resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bandwidth_rate_limit_intervals` | Vec<String> | ✅ | <p> An array containing bandwidth rate limit schedule intervals for a gateway. When no
         bandwidth rate limit intervals have been scheduled, the array is empty. </p> |
| `gateway_arn` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bandwidth_rate_limit_intervals` | Vec<String> | <p> An array that contains the bandwidth rate limit intervals for a tape or volume gateway. </p> |
| `gateway_arn` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bandwidth_rate_limit_schedule outputs
bandwidth_rate_limit_schedule_id = bandwidth_rate_limit_schedule.id
bandwidth_rate_limit_schedule_bandwidth_rate_limit_intervals = bandwidth_rate_limit_schedule.bandwidth_rate_limit_intervals
bandwidth_rate_limit_schedule_gateway_arn = bandwidth_rate_limit_schedule.gateway_arn
```

---


### File_system_associations

FileSystemAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_system_association_info_list` | Vec<String> | <p>An array containing the <code>FileSystemAssociationInfo</code> data type of each file
         system association to be described.
         </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access file_system_associations outputs
file_system_associations_id = file_system_associations.id
file_system_associations_file_system_association_info_list = file_system_associations.file_system_association_info_list
```

---


### Working_storage

WorkingStorage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `working_storage_used_in_bytes` | i64 | <p>The total working storage in bytes in use by the gateway. If no working storage is
         configured for the gateway, this field returns 0.</p> |
| `disk_ids` | Vec<String> | <p>An array of the gateway's local disk IDs that are configured as working storage.
         Each local disk ID is specified as a string (minimum length of 1 and maximum length of
         300). If no local disks are configured as working storage, then the DiskIds array is
         empty.</p> |
| `working_storage_allocated_in_bytes` | i64 | <p>The total working storage in bytes allocated for the gateway. If no working storage is
         configured for the gateway, this field returns 0.</p> |
| `gateway_arn` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access working_storage outputs
working_storage_id = working_storage.id
working_storage_working_storage_used_in_bytes = working_storage.working_storage_used_in_bytes
working_storage_disk_ids = working_storage.disk_ids
working_storage_working_storage_allocated_in_bytes = working_storage.working_storage_allocated_in_bytes
working_storage_gateway_arn = working_storage.gateway_arn
```

---


### Chap_credentials

ChapCredentials resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secret_to_authenticate_initiator` | String | ✅ | <p>The secret key that the initiator (for example, the Windows client) must provide to
         participate in mutual CHAP with the target.</p>
         <note>
            <p>The secret key must be between 12 and 16 bytes when encoded in UTF-8.</p>
         </note> |
| `target_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return the TargetARN for specified
         VolumeARN.</p> |
| `initiator_name` | String | ✅ | <p>The iSCSI initiator that connects to the target.</p> |
| `secret_to_authenticate_target` | String |  | <p>The secret key that the target must provide to participate in mutual CHAP with the
         initiator (e.g. Windows client).</p>
         <p>Byte constraints: Minimum bytes of 12. Maximum bytes of 16.</p>
         <note>
            <p>The secret key must be between 12 and 16 bytes when encoded in UTF-8.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `chap_credentials` | Vec<String> | <p>An array of <a>ChapInfo</a> objects that represent CHAP credentials. Each
         object in the array contains CHAP credential information for one target-initiator pair. If
         no CHAP credentials are set, an empty array is returned. CHAP credential information is
         provided in a JSON object with the following fields:</p>
         <ul>
            <li>
               <p>
                  <b>InitiatorName</b>: The iSCSI initiator that connects to
               the target.</p>
            </li>
            <li>
               <p>
                  <b>SecretToAuthenticateInitiator</b>: The secret key that
               the initiator (for example, the Windows client) must provide to participate in mutual
               CHAP with the target.</p>
            </li>
            <li>
               <p>
                  <b>SecretToAuthenticateTarget</b>: The secret key that the
               target must provide to participate in mutual CHAP with the initiator (e.g. Windows
               client).</p>
            </li>
            <li>
               <p>
                  <b>TargetARN</b>: The Amazon Resource Name (ARN) of the
               storage volume.</p>
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

# Access chap_credentials outputs
chap_credentials_id = chap_credentials.id
chap_credentials_chap_credentials = chap_credentials.chap_credentials
```

---


### Snapshot_from_volume_recovery_point

SnapshotFromVolumeRecoveryPoint resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for
         specified VolumeARN.</p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to a snapshot. Each tag is a key-value
         pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |
| `snapshot_description` | String | ✅ | <p>Textual description of the snapshot that appears in the Amazon EC2 console, Elastic
         Block Store snapshots panel in the <b>Description</b> field, and
         in the Storage Gateway snapshot <b>Details</b> pane,
            <b>Description</b> field.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot_from_volume_recovery_point
snapshot_from_volume_recovery_point = provider.storage_gateway.Snapshot_from_volume_recovery_point {
    volume_arn = "value"  # <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for
         specified VolumeARN.</p>
    snapshot_description = "value"  # <p>Textual description of the snapshot that appears in the Amazon EC2 console, Elastic
         Block Store snapshots panel in the <b>Description</b> field, and
         in the Storage Gateway snapshot <b>Details</b> pane,
            <b>Description</b> field.</p>
}

```

---


### Upload_buffer

UploadBuffer resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upload_buffer_used_in_bytes` | i64 | <p>The total number of bytes being used in the gateway's upload buffer.</p> |
| `gateway_arn` | String |  |
| `upload_buffer_allocated_in_bytes` | i64 | <p>The total number of bytes allocated in the gateway's as upload buffer.</p> |
| `disk_ids` | Vec<String> | <p>An array of the gateway's local disk IDs that are configured as working storage.
         Each local disk ID is specified as a string (minimum length of 1 and maximum length of
         300). If no local disks are configured as working storage, then the DiskIds array is
         empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access upload_buffer outputs
upload_buffer_id = upload_buffer.id
upload_buffer_upload_buffer_used_in_bytes = upload_buffer.upload_buffer_used_in_bytes
upload_buffer_gateway_arn = upload_buffer.gateway_arn
upload_buffer_upload_buffer_allocated_in_bytes = upload_buffer.upload_buffer_allocated_in_bytes
upload_buffer_disk_ids = upload_buffer.disk_ids
```

---


### Storedi_scsi_volumes

StorediSCSIVolumes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `storedi_scsi_volumes` | Vec<String> | <p>Describes a single unit of output from <a>DescribeStorediSCSIVolumes</a>. The
         following fields are returned:</p>
         <ul>
            <li>
               <p>
                  <code>ChapEnabled</code>: Indicates whether mutual CHAP is enabled for the iSCSI
               target.</p>
            </li>
            <li>
               <p>
                  <code>LunNumber</code>: The logical disk number.</p>
            </li>
            <li>
               <p>
                  <code>NetworkInterfaceId</code>: The network interface ID of the stored volume that
               initiator use to map the stored volume as an iSCSI target.</p>
            </li>
            <li>
               <p>
                  <code>NetworkInterfacePort</code>: The port used to communicate with iSCSI
               targets.</p>
            </li>
            <li>
               <p>
                  <code>PreservedExistingData</code>: Indicates when the stored volume was created,
               existing data on the underlying local disk was preserved.</p>
            </li>
            <li>
               <p>
                  <code>SourceSnapshotId</code>: If the stored volume was created from a snapshot, this
               field contains the snapshot ID used, e.g. <code>snap-1122aabb</code>. Otherwise, this
               field is not included.</p>
            </li>
            <li>
               <p>
                  <code>StorediSCSIVolumes</code>: An array of StorediSCSIVolume objects where each
               object contains metadata about one stored volume.</p>
            </li>
            <li>
               <p>
                  <code>TargetARN</code>: The Amazon Resource Name (ARN) of the volume target.</p>
            </li>
            <li>
               <p>
                  <code>VolumeARN</code>: The Amazon Resource Name (ARN) of the stored volume.</p>
            </li>
            <li>
               <p>
                  <code>VolumeDiskId</code>: The disk ID of the local disk that was specified in the
                  <a>CreateStorediSCSIVolume</a> operation.</p>
            </li>
            <li>
               <p>
                  <code>VolumeId</code>: The unique identifier of the storage volume, e.g.
                  <code>vol-1122AABB</code>.</p>
            </li>
            <li>
               <p>
                  <code>VolumeiSCSIAttributes</code>: An <a>VolumeiSCSIAttributes</a> object
               that represents a collection of iSCSI attributes for one stored volume.</p>
            </li>
            <li>
               <p>
                  <code>VolumeProgress</code>: Represents the percentage complete if the volume is
               restoring or bootstrapping that represents the percent of data transferred. This
               field does not appear in the response if the stored volume is not restoring or
               bootstrapping.</p>
            </li>
            <li>
               <p>
                  <code>VolumeSizeInBytes</code>: The size of the volume in bytes.</p>
            </li>
            <li>
               <p>
                  <code>VolumeStatus</code>: One of the <code>VolumeStatus</code> values that indicates
               the state of the volume.</p>
            </li>
            <li>
               <p>
                  <code>VolumeType</code>: One of the enumeration values describing the type of the
               volume. Currently, only <code>STORED</code> volumes are supported.</p>
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

# Access storedi_scsi_volumes outputs
storedi_scsi_volumes_id = storedi_scsi_volumes.id
storedi_scsi_volumes_storedi_scsi_volumes = storedi_scsi_volumes.storedi_scsi_volumes
```

---


### Nfs_file_share

NFSFileShare resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requester_pays` | bool |  | <p>A value that sets who pays the cost of the request and the cost associated with data
         download from the S3 bucket. If this value is set to <code>true</code>, the requester pays
         the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays
         the cost of storing data.</p>
         <note>
            <p>
               <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file
            share, so make sure that the configuration on the file share is the same as the S3
            bucket configuration.</p>
         </note>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `client_token` | String | ✅ | <p>A unique string value that you supply that is used by S3 File Gateway to ensure
         idempotent file share creation.</p> |
| `vpc_endpoint_dns_name` | String |  | <p>Specifies the DNS name for the VPC endpoint that the NFS file share uses to connect to
            Amazon S3.</p>
         <note>
            <p>This parameter is required for NFS file shares that connect to Amazon S3
            through a VPC endpoint, a VPC access point, or an access point alias that points to a
            VPC access point.</p>
         </note> |
| `cache_attributes` | String |  | <p>Specifies refresh cache information for the file share.</p> |
| `bucket_region` | String |  | <p>Specifies the Region of the S3 bucket where the NFS file share stores files.</p>
         <note>
            <p>This parameter is required for NFS file shares that connect to Amazon S3
            through a VPC endpoint, a VPC access point, or an access point alias that points to a
            VPC access point.</p>
         </note> |
| `location_arn` | String | ✅ | <p>A custom ARN for the backend storage used for storing data for file shares. It includes
         a resource ARN with an optional prefix concatenation. The prefix must end with a forward
         slash (/).</p>
         <note>
            <p>You can specify LocationARN as a bucket ARN, access point ARN or access point alias,
            as shown in the following examples.</p>
            <p>Bucket ARN:</p>
            <p>
               <code>arn:aws:s3:::amzn-s3-demo-bucket/prefix/</code>
            </p>
            <p>Access point ARN:</p>
            <p>
               <code>arn:aws:s3:region:account-id:accesspoint/access-point-name/prefix/</code>
            </p>
            <p>If you specify an access point, the bucket policy must be configured to delegate
            access control to the access point. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-policies.html#access-points-delegating-control">Delegating access control to access points</a> in the <i>Amazon S3 User Guide</i>.</p>
            <p>Access point alias:</p>
            <p>
               <code>test-ap-ab123cdef4gehijklmn5opqrstuvuse1a-s3alias</code>
            </p>
         </note> |
| `object_acl` | String |  | <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket
         that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p> |
| `encryption_type` | String |  | <p>A value that specifies the type of server-side encryption that the file share will use
         for the data that it stores in Amazon S3.</p>
         <note>
            <p>We recommend using <code>EncryptionType</code> instead of <code>KMSEncrypted</code>
            to set the file share encryption method. You do not need to provide values for both
            parameters.</p>
            <p>If values for both parameters exist in the same request, then the specified
            encryption methods must not conflict. For example, if <code>EncryptionType</code> is
               <code>SseS3</code>, then <code>KMSEncrypted</code> must be <code>false</code>. If
               <code>EncryptionType</code> is <code>SseKms</code> or <code>DsseKms</code>, then
               <code>KMSEncrypted</code> must be <code>true</code>.</p>
         </note> |
| `read_only` | bool |  | <p>A value that sets the write status of a file share. Set this value to <code>true</code>
         to set the write status to read-only, otherwise set to <code>false</code>.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `kms_encrypted` | bool |  | <p>Optional. Set to <code>true</code> to use Amazon S3 server-side encryption with
         your own KMS key (SSE-KMS), or <code>false</code> to use a key managed by
            Amazon S3 (SSE-S3). To use dual-layer encryption (DSSE-KMS), set the
            <code>EncryptionType</code> parameter instead.</p>
         <note>
            <p>We recommend using <code>EncryptionType</code> instead of <code>KMSEncrypted</code>
            to set the file share encryption method. You do not need to provide values for both
            parameters.</p>
            <p>If values for both parameters exist in the same request, then the specified
            encryption methods must not conflict. For example, if <code>EncryptionType</code> is
               <code>SseS3</code>, then <code>KMSEncrypted</code> must be <code>false</code>. If
               <code>EncryptionType</code> is <code>SseKms</code> or <code>DsseKms</code>, then
               <code>KMSEncrypted</code> must be <code>true</code>.</p>
         </note>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `notification_policy` | String |  | <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls
         the number of seconds to wait after the last point in time a client wrote to a file before
         generating an <code>ObjectUploaded</code> notification. Because clients can make many small
         writes to files, it's best to set this parameter for as long as possible to avoid
         generating multiple notifications for the same file in a small time period.</p>
         <note>
            <p>
               <code>SettlingTimeInSeconds</code> has no effect on the timing of the object
            uploading to Amazon S3, only the timing of the notification.</p>
            <p>This setting is not meant to specify an exact time at which the notification will be
            sent. In some cases, the gateway might require more than the specified delay time to
            generate and send notifications.</p>
         </note>
         <p>The following example sets <code>NotificationPolicy</code> on with
            <code>SettlingTimeInSeconds</code> set to 60.</p>
         <p>
            <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code>
         </p>
         <p>The following example sets <code>NotificationPolicy</code> off.</p>
         <p>
            <code>{}</code>
         </p> |
| `kms_key` | String |  | <p>Optional. The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used
         for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric
         CMKs. This value must be set if <code>KMSEncrypted</code> is <code>true</code>, or if
            <code>EncryptionType</code> is <code>SseKms</code> or <code>DsseKms</code>.</p> |
| `role` | String | ✅ | <p>The ARN of the Identity and Access Management (IAM) role that an S3 File Gateway assumes when it
         accesses the underlying storage.</p> |
| `nfs_file_share_defaults` | String |  | <p>File share default values. Optional.</p> |
| `default_storage_class` | String |  | <p>The default storage class for objects put into an Amazon S3 bucket by the S3
         File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
         <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> |
            <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code>
         </p> |
| `audit_destination_arn` | String |  | <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to the NFS file share. Each tag is a
         key-value pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |
| `guess_mime_type_enabled` | bool |  | <p>A value that enables guessing of the MIME type for uploaded objects based on file
         extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set
         to <code>false</code>. The default value is <code>true</code>.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `client_list` | Vec<String> |  | <p>The list of clients that are allowed to access the S3 File Gateway. The list must
         contain either valid IPv4/IPv6 addresses or valid CIDR blocks.</p> |
| `gateway_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the S3 File Gateway on which you want to create a file
         share.</p> |
| `squash` | String |  | <p>A value that maps a user to anonymous user.</p>
         <p>Valid values are the following:</p>
         <ul>
            <li>
               <p>
                  <code>RootSquash</code>: Only root is mapped to anonymous user.</p>
            </li>
            <li>
               <p>
                  <code>NoSquash</code>: No one is mapped to anonymous user.</p>
            </li>
            <li>
               <p>
                  <code>AllSquash</code>: Everyone is mapped to anonymous user.</p>
            </li>
         </ul> |
| `file_share_name` | String |  | <p>The name of the file share. Optional.</p>
         <note>
            <p>
               <code>FileShareName</code> must be set if an S3 prefix name is set in
               <code>LocationARN</code>, or if an access point or access point alias is used.</p>
            <p>A valid NFS file share name can only contain the following characters:
               <code>a</code>-<code>z</code>, <code>A</code>-<code>Z</code>,
               <code>0</code>-<code>9</code>, <code>-</code>, <code>.</code>, and
            <code>_</code>.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create nfs_file_share
nfs_file_share = provider.storage_gateway.Nfs_file_share {
    client_token = "value"  # <p>A unique string value that you supply that is used by S3 File Gateway to ensure
         idempotent file share creation.</p>
    location_arn = "value"  # <p>A custom ARN for the backend storage used for storing data for file shares. It includes
         a resource ARN with an optional prefix concatenation. The prefix must end with a forward
         slash (/).</p>
         <note>
            <p>You can specify LocationARN as a bucket ARN, access point ARN or access point alias,
            as shown in the following examples.</p>
            <p>Bucket ARN:</p>
            <p>
               <code>arn:aws:s3:::amzn-s3-demo-bucket/prefix/</code>
            </p>
            <p>Access point ARN:</p>
            <p>
               <code>arn:aws:s3:region:account-id:accesspoint/access-point-name/prefix/</code>
            </p>
            <p>If you specify an access point, the bucket policy must be configured to delegate
            access control to the access point. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-policies.html#access-points-delegating-control">Delegating access control to access points</a> in the <i>Amazon S3 User Guide</i>.</p>
            <p>Access point alias:</p>
            <p>
               <code>test-ap-ab123cdef4gehijklmn5opqrstuvuse1a-s3alias</code>
            </p>
         </note>
    role = "value"  # <p>The ARN of the Identity and Access Management (IAM) role that an S3 File Gateway assumes when it
         accesses the underlying storage.</p>
    gateway_arn = "value"  # <p>The Amazon Resource Name (ARN) of the S3 File Gateway on which you want to create a file
         share.</p>
}

```

---


### Bandwidth_rate_limit

BandwidthRateLimit resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_arn` | String | ✅ |  |
| `average_download_rate_limit_in_bits_per_sec` | i64 |  | <p>The average download bandwidth rate limit in bits per second.</p> |
| `average_upload_rate_limit_in_bits_per_sec` | i64 |  | <p>The average upload bandwidth rate limit in bits per second.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gateway_arn` | String |  |
| `average_upload_rate_limit_in_bits_per_sec` | i64 | <p>The average upload bandwidth rate limit in bits per second. This field does not appear
         in the response if the upload rate limit is not set.</p> |
| `average_download_rate_limit_in_bits_per_sec` | i64 | <p>The average download bandwidth rate limit in bits per second. This field does not appear
         in the response if the download rate limit is not set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bandwidth_rate_limit outputs
bandwidth_rate_limit_id = bandwidth_rate_limit.id
bandwidth_rate_limit_gateway_arn = bandwidth_rate_limit.gateway_arn
bandwidth_rate_limit_average_upload_rate_limit_in_bits_per_sec = bandwidth_rate_limit.average_upload_rate_limit_in_bits_per_sec
bandwidth_rate_limit_average_download_rate_limit_in_bits_per_sec = bandwidth_rate_limit.average_download_rate_limit_in_bits_per_sec
```

---


### Storedi_scsi_volume

StorediSCSIVolume resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network_interface_id` | String | ✅ | <p>The network interface of the gateway on which to expose the iSCSI target. Accepts IPv4
         and IPv6 addresses. Use <a>DescribeGatewayInformation</a> to get a list of the
         network interfaces available on a gateway.</p>
         <p>Valid Values: A valid IP address.</p> |
| `snapshot_id` | String |  | <p>The snapshot ID (e.g., "snap-1122aabb") of the snapshot to restore as the new stored
         volume. Specify this field if you want to create the iSCSI storage volume from a snapshot;
         otherwise, do not include this field. To list snapshots for your account use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API
            Reference</i>.</p> |
| `preserve_existing_data` | bool | ✅ | <p>Set to <code>true</code> if you want to preserve the data on the local disk. Otherwise,
         set to <code>false</code> to create an empty volume.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `target_name` | String | ✅ | <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a
         suffix for the target ARN. For example, specifying <code>TargetName</code> as
            <i>myvolume</i> results in the target ARN of
            <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>.
         The target name must be unique across all volumes on a gateway.</p>
         <p>If you don't specify a value, Storage Gateway uses the value that was previously
         used for this volume as the new target name.</p> |
| `disk_id` | String | ✅ | <p>The unique identifier for the gateway local disk that is configured as a stored volume.
         Use <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/API_ListLocalDisks.html">ListLocalDisks</a> to
         list disk IDs for a gateway.</p> |
| `kms_key` | String |  | <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This
         value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p> |
| `gateway_arn` | String | ✅ |  |
| `kms_encrypted` | bool |  | <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own
            KMS key, or <code>false</code> to use a key managed by Amazon S3.
         Optional.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to a stored volume. Each tag is a key-value
         pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create storedi_scsi_volume
storedi_scsi_volume = provider.storage_gateway.Storedi_scsi_volume {
    network_interface_id = "value"  # <p>The network interface of the gateway on which to expose the iSCSI target. Accepts IPv4
         and IPv6 addresses. Use <a>DescribeGatewayInformation</a> to get a list of the
         network interfaces available on a gateway.</p>
         <p>Valid Values: A valid IP address.</p>
    preserve_existing_data = "value"  # <p>Set to <code>true</code> if you want to preserve the data on the local disk. Otherwise,
         set to <code>false</code> to create an empty volume.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p>
    target_name = "value"  # <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a
         suffix for the target ARN. For example, specifying <code>TargetName</code> as
            <i>myvolume</i> results in the target ARN of
            <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>.
         The target name must be unique across all volumes on a gateway.</p>
         <p>If you don't specify a value, Storage Gateway uses the value that was previously
         used for this volume as the new target name.</p>
    disk_id = "value"  # <p>The unique identifier for the gateway local disk that is configured as a stored volume.
         Use <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/API_ListLocalDisks.html">ListLocalDisks</a> to
         list disk IDs for a gateway.</p>
    gateway_arn = "value"  # Required field
}

```

---


### Tape_archive

TapeArchive resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Tape_archives

TapeArchives resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tape_archives` | Vec<String> | <p>An array of virtual tape objects in the virtual tape shelf (VTS). The description
         includes of the Amazon Resource Name (ARN) of the virtual tapes. The information returned
         includes the Amazon Resource Names (ARNs) of the tapes, size of the tapes, status of the
         tapes, progress of the description, and tape barcode.</p> |
| `marker` | String | <p>An opaque string that indicates the position at which the virtual tapes that were
         fetched for description ended. Use this marker in your next request to fetch the next set
         of virtual tapes in the virtual tape shelf (VTS). If there are no more virtual tapes to
         describe, this field does not appear in the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tape_archives outputs
tape_archives_id = tape_archives.id
tape_archives_tape_archives = tape_archives.tape_archives
tape_archives_marker = tape_archives.marker
```

---


### Gateway_software_now

GatewaySoftwareNow resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_arn` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Smb_local_groups

SMBLocalGroups resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `smb_local_groups` | String | ✅ | <p>A list of Active Directory users and groups that you want to grant special permissions
         for SMB file shares on the gateway.</p> |
| `gateway_arn` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Volume

Volume resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Cache_report

CacheReport resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_report_info` | String | <p>Contains all informational fields associated with a cache report. Includes name, ARN,
         tags, status, progress, filters, start time, and end time.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_report outputs
cache_report_id = cache_report.id
cache_report_cache_report_info = cache_report.cache_report_info
```

---


### Tapes

Tapes resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_encrypted` | bool |  | <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own
            KMS key, or <code>false</code> to use a key managed by Amazon S3.
         Optional.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `kms_key` | String |  | <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This
         value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p> |
| `client_token` | String | ✅ | <p>A unique identifier that you use to retry a request. If you retry a request, use the
         same <code>ClientToken</code> you specified in the initial request.</p>
         <note>
            <p>Using the same <code>ClientToken</code> prevents creating the tape multiple
            times.</p>
         </note> |
| `num_tapes_to_create` | i64 | ✅ | <p>The number of virtual tapes that you want to create.</p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to a virtual tape. Each tag is a key-value
         pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |
| `tape_barcode_prefix` | String | ✅ | <p>A prefix that you append to the barcode of the virtual tape you are creating. This
         prefix makes the barcode unique.</p>
         <note>
            <p>The prefix must be 1-4 characters in length and must be one of the uppercase letters
            from A to Z.</p>
         </note> |
| `pool_id` | String |  | <p>The ID of the pool that you want to add your tape to for archiving. The tape in this
         pool is archived in the S3 storage class that is associated with the pool. When you use
         your backup application to eject the tape, the tape is archived directly into the storage
         class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> |
| `worm` | bool |  | <p>Set to <code>TRUE</code> if the tape you are creating is to be configured as a
         write-once-read-many (WORM) tape.</p> |
| `gateway_arn` | String | ✅ | <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the
         virtual tapes with. Use the <a>ListGateways</a> operation to return a list of
         gateways for your account and Amazon Web Services Region.</p> |
| `tape_size_in_bytes` | i64 | ✅ | <p>The size, in bytes, of the virtual tapes that you want to create.</p>
         <note>
            <p>The size must be aligned by gigabyte (1024*1024*1024 bytes).</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An opaque string that can be used as part of a subsequent <code>DescribeTapes</code>
         call to retrieve the next page of results.</p>
         <p>If a response does not contain a marker, then there are no more results to be
         retrieved.</p> |
| `tapes` | Vec<String> | <p>An array of virtual tape descriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tapes
tapes = provider.storage_gateway.Tapes {
    client_token = "value"  # <p>A unique identifier that you use to retry a request. If you retry a request, use the
         same <code>ClientToken</code> you specified in the initial request.</p>
         <note>
            <p>Using the same <code>ClientToken</code> prevents creating the tape multiple
            times.</p>
         </note>
    num_tapes_to_create = "value"  # <p>The number of virtual tapes that you want to create.</p>
    tape_barcode_prefix = "value"  # <p>A prefix that you append to the barcode of the virtual tape you are creating. This
         prefix makes the barcode unique.</p>
         <note>
            <p>The prefix must be 1-4 characters in length and must be one of the uppercase letters
            from A to Z.</p>
         </note>
    gateway_arn = "value"  # <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the
         virtual tapes with. Use the <a>ListGateways</a> operation to return a list of
         gateways for your account and Amazon Web Services Region.</p>
    tape_size_in_bytes = "value"  # <p>The size, in bytes, of the virtual tapes that you want to create.</p>
         <note>
            <p>The size must be aligned by gigabyte (1024*1024*1024 bytes).</p>
         </note>
}

# Access tapes outputs
tapes_id = tapes.id
tapes_marker = tapes.marker
tapes_tapes = tapes.tapes
```

---


### Tape_pool

TapePool resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_lock_time_in_days` | i64 |  | <p>Tape retention lock time is set in days. Tape retention lock can be enabled for up to
         100 years (36,500 days).</p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to tape pool. Each tag is a key-value
         pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |
| `pool_name` | String | ✅ | <p>The name of the new custom tape pool.</p> |
| `retention_lock_type` | String |  | <p>Tape retention lock can be configured in two modes. When configured in governance mode,
            Amazon Web Services accounts with specific IAM permissions are authorized to remove the
         tape retention lock from archived virtual tapes. When configured in compliance mode, the
         tape retention lock cannot be removed by any user, including the root Amazon Web Services account.</p> |
| `storage_class` | String | ✅ | <p>The storage class that is associated with the new custom pool. When you use your backup
         application to eject the tape, the tape is archived directly into the storage class (S3
         Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tape_pool
tape_pool = provider.storage_gateway.Tape_pool {
    pool_name = "value"  # <p>The name of the new custom tape pool.</p>
    storage_class = "value"  # <p>The storage class that is associated with the new custom pool. When you use your backup
         application to eject the tape, the tape is archived directly into the storage class (S3
         Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p>
}

```

---


### Tape_recovery_points

TapeRecoveryPoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An opaque string that indicates the position at which the virtual tape recovery points
         that were listed for description ended.</p>
         <p>Use this marker in your next request to list the next set of virtual tape recovery
         points in the list. If there are no more recovery points to describe, this field does not
         appear in the response.</p> |
| `gateway_arn` | String |  |
| `tape_recovery_point_infos` | Vec<String> | <p>An array of TapeRecoveryPointInfos that are available for the specified gateway.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tape_recovery_points outputs
tape_recovery_points_id = tape_recovery_points.id
tape_recovery_points_marker = tape_recovery_points.marker
tape_recovery_points_gateway_arn = tape_recovery_points.gateway_arn
tape_recovery_points_tape_recovery_point_infos = tape_recovery_points.tape_recovery_point_infos
```

---


### Smb_file_shares

SMBFileShares resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `smb_file_share_info_list` | Vec<String> | <p>An array containing a description for each requested file share.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access smb_file_shares outputs
smb_file_shares_id = smb_file_shares.id
smb_file_shares_smb_file_share_info_list = smb_file_shares.smb_file_share_info_list
```

---


### Cachedi_scsi_volumes

CachediSCSIVolumes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cachedi_scsi_volumes` | Vec<String> | <p>An array of objects where each object contains metadata about one cached volume.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cachedi_scsi_volumes outputs
cachedi_scsi_volumes_id = cachedi_scsi_volumes.id
cachedi_scsi_volumes_cachedi_scsi_volumes = cachedi_scsi_volumes.cachedi_scsi_volumes
```

---


### Availability_monitor_test

AvailabilityMonitorTest resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | <p>The time the high availability monitoring test was started. If a test hasn't been
         performed, the value of this field is null.</p> |
| `status` | String | <p>The status of the high availability monitoring test. If a test hasn't been
         performed, the value of this field is null.</p> |
| `gateway_arn` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access availability_monitor_test outputs
availability_monitor_test_id = availability_monitor_test.id
availability_monitor_test_start_time = availability_monitor_test.start_time
availability_monitor_test_status = availability_monitor_test.status
availability_monitor_test_gateway_arn = availability_monitor_test.gateway_arn
```

---


### Smb_security_strategy

SMBSecurityStrategy resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `smb_security_strategy` | String | ✅ | <p>Specifies the type of security strategy.</p>
         <p>
            <code>ClientSpecified</code>: If you choose this option, requests are established based
         on what is negotiated by the client. This option is recommended when you want to maximize
         compatibility across different clients in your environment. Supported only for S3 File
         Gateway.</p>
         <p>
            <code>MandatorySigning</code>: If you choose this option, File Gateway only allows
         connections from SMBv2 or SMBv3 clients that have signing enabled. This option works with
         SMB clients on Microsoft Windows Vista, Windows Server 2008 or newer.</p>
         <p>
            <code>MandatoryEncryption</code>: If you choose this option, File Gateway only allows
         connections from SMBv3 clients that have encryption enabled. This option is recommended for
         environments that handle sensitive data. This option works with SMB clients on Microsoft
         Windows 8, Windows Server 2012 or newer.</p>
         <p>
            <code>MandatoryEncryptionNoAes128</code>: If you choose this option, File Gateway only
         allows connections from SMBv3 clients that use 256-bit AES encryption algorithms. 128-bit
         algorithms are not allowed. This option is recommended for environments that handle
         sensitive data. It works with SMB clients on Microsoft Windows 8, Windows Server 2012, or
         later.</p> |
| `gateway_arn` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Gateway_information

GatewayInformation resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_timezone` | String |  | <p>A value that indicates the time zone of the gateway.</p> |
| `cloud_watch_log_group_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that you want to use
         to monitor and log events in the gateway.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/WhatIsCloudWatchLogs.html">What is Amazon CloudWatch
            Logs?</a>
         </p> |
| `gateway_arn` | String | ✅ |  |
| `gateway_name` | String |  |  |
| `gateway_capacity` | String |  | <p>Specifies the size of the gateway's metadata cache. This setting impacts gateway
         performance and hardware recommendations. For more information, see <a href="https://docs.aws.amazon.com/filegateway/latest/files3/performance-multiple-file-shares.html">Performance guidance for gateways with multiple file shares</a>
         in the <i>Amazon S3 File Gateway User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_type` | String | <p>The type of endpoint for your gateway.</p>
         <p>Valid Values: <code>STANDARD</code> | <code>FIPS</code>
         </p> |
| `software_version` | String | <p>The version number of the software running on the gateway appliance.</p> |
| `gateway_type` | String | <p>The type of the gateway.</p>
         <important>
            <p>Amazon FSx File Gateway is no longer available to new customers. Existing
            customers of FSx File Gateway can continue to use the service normally. For
            capabilities similar to FSx File Gateway, visit <a href="https://aws.amazon.com/blogs/storage/switch-your-file-share-access-from-amazon-fsx-file-gateway-to-amazon-fsx-for-windows-file-server/">this blog post</a>.</p>
         </important> |
| `ec2_instance_region` | String | <p>The Amazon Web Services Region where the Amazon EC2 instance is located.</p> |
| `last_software_update` | String | <p>The date on which the last software update was applied to the gateway. If the gateway
         has never been updated, this field does not return a value in the response. This only only
         exist and returns once it have been chosen and set by the SGW service, based on the OS
         version of the gateway VM</p> |
| `next_update_availability_date` | String | <p>The date on which an update to the gateway is available. This date is in the time zone
         of the gateway. If the gateway is not available for an update this field is not returned in
         the response.</p> |
| `gateway_capacity` | String | <p>Specifies the size of the gateway's metadata cache.</p> |
| `gateway_state` | String | <p>A value that indicates the operating state of the gateway.</p> |
| `gateway_network_interfaces` | Vec<String> | <p>A <a>NetworkInterface</a> array that contains descriptions of the gateway
         network interfaces.</p> |
| `supported_gateway_capacities` | Vec<String> | <p>A list of the metadata cache sizes that the gateway can support based on its current
         hardware specifications.</p> |
| `tags` | Vec<String> | <p>A list of up to 50 tags assigned to the gateway, sorted alphabetically by key name. Each
         tag is a key-value pair. For a gateway with more than 10 tags assigned, you can view all
         tags using the <code>ListTagsForResource</code> API operation.</p> |
| `gateway_name` | String | <p>The name you configured for your gateway.</p> |
| `gateway_arn` | String |  |
| `vpc_endpoint` | String | <p>The configuration settings for the virtual private cloud (VPC) endpoint for your
         gateway.</p> |
| `cloud_watch_log_group_arn` | String | <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that is used to
         monitor events in the gateway. This field only only exist and returns once it have been
         chosen and set by the SGW service, based on the OS version of the gateway VM</p> |
| `ec2_instance_id` | String | <p>The ID of the Amazon EC2 instance that was used to launch the gateway.</p> |
| `host_environment` | String | <p>The type of hardware or software platform on which the gateway is running.</p>
         <note>
            <p>Tape Gateway is no longer available on Snow Family devices.</p>
         </note> |
| `deprecation_date` | String | <p>Date after which this gateway will not receive software updates for new features and bug
         fixes.</p> |
| `host_environment_id` | String | <p>A unique identifier for the specific instance of the host platform running the gateway.
         This value is only available for certain host environments, and its format depends on the
         host environment type.</p> |
| `gateway_timezone` | String | <p>A value that indicates the time zone configured for the gateway.</p> |
| `gateway_id` | String | <p>The unique identifier assigned to your gateway during activation. This ID becomes part
         of the gateway Amazon Resource Name (ARN), which you use as input for other
         operations.</p> |
| `software_updates_end_date` | String | <p>Date after which this gateway will not receive software updates for new features.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access gateway_information outputs
gateway_information_id = gateway_information.id
gateway_information_endpoint_type = gateway_information.endpoint_type
gateway_information_software_version = gateway_information.software_version
gateway_information_gateway_type = gateway_information.gateway_type
gateway_information_ec2_instance_region = gateway_information.ec2_instance_region
gateway_information_last_software_update = gateway_information.last_software_update
gateway_information_next_update_availability_date = gateway_information.next_update_availability_date
gateway_information_gateway_capacity = gateway_information.gateway_capacity
gateway_information_gateway_state = gateway_information.gateway_state
gateway_information_gateway_network_interfaces = gateway_information.gateway_network_interfaces
gateway_information_supported_gateway_capacities = gateway_information.supported_gateway_capacities
gateway_information_tags = gateway_information.tags
gateway_information_gateway_name = gateway_information.gateway_name
gateway_information_gateway_arn = gateway_information.gateway_arn
gateway_information_vpc_endpoint = gateway_information.vpc_endpoint
gateway_information_cloud_watch_log_group_arn = gateway_information.cloud_watch_log_group_arn
gateway_information_ec2_instance_id = gateway_information.ec2_instance_id
gateway_information_host_environment = gateway_information.host_environment
gateway_information_deprecation_date = gateway_information.deprecation_date
gateway_information_host_environment_id = gateway_information.host_environment_id
gateway_information_gateway_timezone = gateway_information.gateway_timezone
gateway_information_gateway_id = gateway_information.gateway_id
gateway_information_software_updates_end_date = gateway_information.software_updates_end_date
```

---


### File_share

FileShare resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Nfs_file_shares

NFSFileShares resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nfs_file_share_info_list` | Vec<String> | <p>An array containing a description for each requested file share.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access nfs_file_shares outputs
nfs_file_shares_id = nfs_file_shares.id
nfs_file_shares_nfs_file_share_info_list = nfs_file_shares.nfs_file_share_info_list
```

---


### Automatic_tape_creation_policy

AutomaticTapeCreationPolicy resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_arn` | String | ✅ |  |
| `automatic_tape_creation_rules` | Vec<String> | ✅ | <p>An automatic tape creation policy consists of a list of automatic tape creation rules.
         The rules determine when and how to automatically create new tapes.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Smb_settings

SMBSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active_directory_status` | String | <p>Indicates the status of a gateway that is a member of the Active Directory
         domain.</p>
         <note>
            <p>This field is only used as part of a <code>JoinDomain</code> request. It is not
            affected by Active Directory connectivity changes that occur after the
               <code>JoinDomain</code> request succeeds.</p>
         </note>
         <ul>
            <li>
               <p>
                  <code>ACCESS_DENIED</code>: Indicates that the <code>JoinDomain</code> operation
               failed due to an authentication error.</p>
            </li>
            <li>
               <p>
                  <code>DETACHED</code>: Indicates that gateway is not joined to a domain.</p>
            </li>
            <li>
               <p>
                  <code>JOINED</code>: Indicates that the gateway has successfully joined a
               domain.</p>
            </li>
            <li>
               <p>
                  <code>JOINING</code>: Indicates that a <code>JoinDomain</code> operation is in
               progress.</p>
            </li>
            <li>
               <p>
                  <code>NETWORK_ERROR</code>: Indicates that <code>JoinDomain</code> operation
               failed due to a network or connectivity error.</p>
            </li>
            <li>
               <p>
                  <code>TIMEOUT</code>: Indicates that the <code>JoinDomain</code> operation failed
               because the operation didn't complete within the allotted time.</p>
            </li>
            <li>
               <p>
                  <code>UNKNOWN_ERROR</code>: Indicates that the <code>JoinDomain</code> operation
               failed due to another type of error.</p>
            </li>
         </ul> |
| `smb_guest_password_set` | bool | <p>This value is <code>true</code> if a password for the guest user <code>smbguest</code>
         is set, otherwise <code>false</code>. Only supported for S3 File Gateways.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `file_shares_visible` | bool | <p>The shares on this gateway appear when listing shares. Only supported for S3 File
         Gateways. </p> |
| `smb_security_strategy` | String | <p>The type of security strategy that was specified for file gateway.</p>
         <ul>
            <li>
               <p>
                  <code>ClientSpecified</code>: If you choose this option, requests are established
               based on what is negotiated by the client. This option is recommended when you want
               to maximize compatibility across different clients in your environment. Supported
               only for S3 File Gateway.</p>
            </li>
            <li>
               <p>
                  <code>MandatorySigning</code>: If you choose this option, File Gateway only allows
               connections from SMBv2 or SMBv3 clients that have signing turned on. This option
               works with SMB clients on Microsoft Windows Vista, Windows Server 2008, or later.
            </p>
            </li>
            <li>
               <p>
                  <code>MandatoryEncryption</code>: If you choose this option, File Gateway only
               allows connections from SMBv3 clients that have encryption turned on. Both 256-bit
               and 128-bit algorithms are allowed. This option is recommended for environments that
               handle sensitive data. It works with SMB clients on Microsoft Windows 8, Windows
               Server 2012, or later.</p>
            </li>
            <li>
               <p>
                  <code>MandatoryEncryptionNoAes128</code>: If you choose this option, File Gateway
               only allows connections from SMBv3 clients that use 256-bit AES encryption
               algorithms. 128-bit algorithms are not allowed. This option is recommended for
               environments that handle sensitive data. It works with SMB clients on Microsoft
               Windows 8, Windows Server 2012, or later.</p>
            </li>
         </ul> |
| `smb_local_groups` | String | <p>A list of Active Directory users and groups that have special permissions for SMB file
         shares on the gateway.</p> |
| `domain_name` | String | <p>The name of the domain that the gateway is joined to.</p> |
| `gateway_arn` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access smb_settings outputs
smb_settings_id = smb_settings.id
smb_settings_active_directory_status = smb_settings.active_directory_status
smb_settings_smb_guest_password_set = smb_settings.smb_guest_password_set
smb_settings_file_shares_visible = smb_settings.file_shares_visible
smb_settings_smb_security_strategy = smb_settings.smb_security_strategy
smb_settings_smb_local_groups = smb_settings.smb_local_groups
smb_settings_domain_name = smb_settings.domain_name
smb_settings_gateway_arn = smb_settings.gateway_arn
```

---


### Tape

Tape resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Vtl_devices

VTLDevices resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An opaque string that indicates the position at which the VTL devices that were fetched
         for description ended. Use the marker in your next request to fetch the next set of VTL
         devices in the list. If there are no more VTL devices to describe, this field does not
         appear in the response.</p> |
| `vtl_devices` | Vec<String> | <p>An array of VTL device objects composed of the Amazon Resource Name (ARN) of the VTL
         devices.</p> |
| `gateway_arn` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vtl_devices outputs
vtl_devices_id = vtl_devices.id
vtl_devices_marker = vtl_devices.marker
vtl_devices_vtl_devices = vtl_devices.vtl_devices
vtl_devices_gateway_arn = vtl_devices.gateway_arn
```

---


### Cache

Cache resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_hit_percentage` | f64 | <p>Percent of application read operations from the file shares that are served from cache.
         The sample is taken at the end of the reporting period.</p> |
| `disk_ids` | Vec<String> | <p>An array of strings that identify disks that are to be configured as working storage.
         Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs
         from the <a>ListLocalDisks</a> API.</p> |
| `cache_allocated_in_bytes` | i64 | <p>The amount of cache in bytes allocated to a gateway.</p> |
| `cache_used_percentage` | f64 | <p>Percent use of the gateway's cache storage. This metric applies only to the
         gateway-cached volume setup. The sample is taken at the end of the reporting period.</p> |
| `cache_dirty_percentage` | f64 | <p>The file share's contribution to the overall percentage of the gateway's cache
         that has not been persisted to Amazon Web Services. The sample is taken at the end of the
         reporting period.</p> |
| `cache_miss_percentage` | f64 | <p>Percent of application read operations from the file shares that are not served from
         cache. The sample is taken at the end of the reporting period.</p> |
| `gateway_arn` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache outputs
cache_id = cache.id
cache_cache_hit_percentage = cache.cache_hit_percentage
cache_disk_ids = cache.disk_ids
cache_cache_allocated_in_bytes = cache.cache_allocated_in_bytes
cache_cache_used_percentage = cache.cache_used_percentage
cache_cache_dirty_percentage = cache.cache_dirty_percentage
cache_cache_miss_percentage = cache.cache_miss_percentage
cache_gateway_arn = cache.gateway_arn
```

---


### Smb_file_share_visibility

SMBFileShareVisibility resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_arn` | String | ✅ |  |
| `file_shares_visible` | bool | ✅ | <p>The shares on this gateway appear when listing shares.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### File_system_association

FileSystemAssociation resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_system_association_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the file system association that you want to
         update.</p> |
| `user_name` | String |  | <p>The user name of the user credential that has permission to access the root share D$ of
         the Amazon FSx file system. The user account must belong to the Amazon FSx
         delegated admin user group.</p> |
| `cache_attributes` | String |  |  |
| `password` | String |  | <p>The password of the user credential.</p> |
| `audit_destination_arn` | String |  | <p>The Amazon Resource Name (ARN) of the storage used for the audit logs.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Gateway

Gateway resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Maintenance_start_time

MaintenanceStartTime resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `minute_of_hour` | i64 |  | <p>The minute component of the maintenance start time represented as
            <i>mm</i>, where <i>mm</i> is the minute (00 to 59). The
         minute of the hour is in the time zone of the gateway.</p> |
| `software_update_preferences` | String |  | <p>A set of variables indicating the software update preferences for the gateway.</p>
         <p>Includes <code>AutomaticUpdatePolicy</code> field with the following inputs:</p>
         <p>
            <code>ALL_VERSIONS</code> - Enables regular gateway maintenance updates.</p>
         <p>
            <code>EMERGENCY_VERSIONS_ONLY</code> - Disables regular gateway maintenance updates. The
         gateway will still receive emergency version updates on rare occasions if necessary to
         remedy highly critical security or durability issues. You will be notified before an
         emergency version update is applied. These updates are applied during your gateway's
         scheduled maintenance window.</p> |
| `day_of_week` | i64 |  | <p>The day of the week component of the maintenance start time week represented as an
         ordinal number from 0 to 6, where 0 represents Sunday and 6 represents Saturday.</p> |
| `gateway_arn` | String | ✅ |  |
| `day_of_month` | i64 |  | <p>The day of the month component of the maintenance start time represented as an ordinal
         number from 1 to 28, where 1 represents the first day of the month. It is not possible to
         set the maintenance schedule to start on days 29 through 31.</p> |
| `hour_of_day` | i64 |  | <p>The hour component of the maintenance start time represented as <i>hh</i>,
         where <i>hh</i> is the hour (00 to 23). The hour of the day is in the time
         zone of the gateway.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gateway_arn` | String |  |
| `minute_of_hour` | i64 | <p>The minute component of the maintenance start time represented as
            <i>mm</i>, where <i>mm</i> is the minute (0 to 59). The
         minute of the hour is in the time zone of the gateway.</p> |
| `hour_of_day` | i64 | <p>The hour component of the maintenance start time represented as <i>hh</i>,
         where <i>hh</i> is the hour (0 to 23). The hour of the day is in the time
         zone of the gateway.</p> |
| `day_of_week` | i64 | <p>An ordinal number between 0 and 6 that represents the day of the week, where 0
         represents Sunday and 6 represents Saturday. The day of week is in the time zone of the
         gateway.</p> |
| `day_of_month` | i64 | <p>The day of the month component of the maintenance start time represented as an ordinal
         number from 1 to 28, where 1 represents the first day of the month. It is not possible to
         set the maintenance schedule to start on days 29 through 31.</p> |
| `timezone` | String | <p>A value that indicates the time zone that is set for the gateway. The start time and day
         of week specified should be in the time zone of the gateway.</p> |
| `software_update_preferences` | String | <p>A set of variables indicating the software update preferences for the gateway.</p>
         <p>Includes <code>AutomaticUpdatePolicy</code> parameter with the following inputs:</p>
         <p>
            <code>ALL_VERSIONS</code> - Enables regular gateway maintenance updates.</p>
         <p>
            <code>EMERGENCY_VERSIONS_ONLY</code> - Disables regular gateway maintenance updates. The
         gateway will still receive emergency version updates on rare occasions if necessary to
         remedy highly critical security or durability issues. You will be notified before an
         emergency version update is applied. These updates are applied during your gateway's
         scheduled maintenance window.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_start_time outputs
maintenance_start_time_id = maintenance_start_time.id
maintenance_start_time_gateway_arn = maintenance_start_time.gateway_arn
maintenance_start_time_minute_of_hour = maintenance_start_time.minute_of_hour
maintenance_start_time_hour_of_day = maintenance_start_time.hour_of_day
maintenance_start_time_day_of_week = maintenance_start_time.day_of_week
maintenance_start_time_day_of_month = maintenance_start_time.day_of_month
maintenance_start_time_timezone = maintenance_start_time.timezone
maintenance_start_time_software_update_preferences = maintenance_start_time.software_update_preferences
```

---


### Cachedi_scsi_volume

CachediSCSIVolume resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_size_in_bytes` | i64 | ✅ | <p>The size of the volume in bytes.</p> |
| `kms_encrypted` | bool |  | <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own
            KMS key, or <code>false</code> to use a key managed by Amazon S3.
         Optional.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `source_volume_arn` | String |  | <p>The ARN for an existing volume. Specifying this ARN makes the new volume into an exact
         copy of the specified existing volume's latest recovery point. The
            <code>VolumeSizeInBytes</code> value for this new volume must be equal to or larger than
         the size of the existing volume, in bytes.</p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that you can assign to a cached volume. Each tag is a key-value
         pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers that you can
            represent in UTF-8 format, and the following special characters: + - = . _ : / @. The
            maximum length of a tag's key is 128 characters, and the maximum length for a
            tag's value is 256 characters.</p>
         </note> |
| `snapshot_id` | String |  | <p>The snapshot ID (e.g. "snap-1122aabb") of the snapshot to restore as the new cached
         volume. Specify this field if you want to create the iSCSI storage volume from a snapshot;
         otherwise, do not include this field. To list snapshots for your account use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API
            Reference</i>.</p> |
| `target_name` | String | ✅ | <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a
         suffix for the target ARN. For example, specifying <code>TargetName</code> as
            <i>myvolume</i> results in the target ARN of
            <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>.
         The target name must be unique across all volumes on a gateway.</p>
         <p>If you don't specify a value, Storage Gateway uses the value that was previously
         used for this volume as the new target name.</p> |
| `gateway_arn` | String | ✅ |  |
| `network_interface_id` | String | ✅ | <p>The network interface of the gateway on which to expose the iSCSI target. Accepts IPv4
         and IPv6 addresses. Use <a>DescribeGatewayInformation</a> to get a list of the
         network interfaces available on a gateway.</p>
         <p>Valid Values: A valid IP address.</p> |
| `client_token` | String | ✅ | <p>A unique identifier that you use to retry a request. If you retry a request, use the
         same <code>ClientToken</code> you specified in the initial request.</p> |
| `kms_key` | String |  | <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This
         value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cachedi_scsi_volume
cachedi_scsi_volume = provider.storage_gateway.Cachedi_scsi_volume {
    volume_size_in_bytes = "value"  # <p>The size of the volume in bytes.</p>
    target_name = "value"  # <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a
         suffix for the target ARN. For example, specifying <code>TargetName</code> as
            <i>myvolume</i> results in the target ARN of
            <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>.
         The target name must be unique across all volumes on a gateway.</p>
         <p>If you don't specify a value, Storage Gateway uses the value that was previously
         used for this volume as the new target name.</p>
    gateway_arn = "value"  # Required field
    network_interface_id = "value"  # <p>The network interface of the gateway on which to expose the iSCSI target. Accepts IPv4
         and IPv6 addresses. Use <a>DescribeGatewayInformation</a> to get a list of the
         network interfaces available on a gateway.</p>
         <p>Valid Values: A valid IP address.</p>
    client_token = "value"  # <p>A unique identifier that you use to retry a request. If you retry a request, use the
         same <code>ClientToken</code> you specified in the initial request.</p>
}

```

---


### Smb_file_share

SMBFileShare resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_share_name` | String |  | <p>The name of the file share. Optional.</p>
         <note>
            <p>
               <code>FileShareName</code> must be set if an S3 prefix name is set in
               <code>LocationARN</code>, or if an access point or access point alias is used.</p>
            <p>A valid SMB file share name cannot contain the following characters:
               <code>[</code>,<code>]</code>,<code>#</code>,<code>;</code>,<code><</code>,<code>></code>,<code>:</code>,<code>"</code>,<code>\</code>,<code>/</code>,<code>|</code>,<code>?</code>,<code>*</code>,<code>+</code>,
            or ASCII control characters <code>1-31</code>.</p>
         </note> |
| `notification_policy` | String |  | <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls
         the number of seconds to wait after the last point in time a client wrote to a file before
         generating an <code>ObjectUploaded</code> notification. Because clients can make many small
         writes to files, it's best to set this parameter for as long as possible to avoid
         generating multiple notifications for the same file in a small time period.</p>
         <note>
            <p>
               <code>SettlingTimeInSeconds</code> has no effect on the timing of the object
            uploading to Amazon S3, only the timing of the notification.</p>
            <p>This setting is not meant to specify an exact time at which the notification will be
            sent. In some cases, the gateway might require more than the specified delay time to
            generate and send notifications.</p>
         </note>
         <p>The following example sets <code>NotificationPolicy</code> on with
            <code>SettlingTimeInSeconds</code> set to 60.</p>
         <p>
            <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code>
         </p>
         <p>The following example sets <code>NotificationPolicy</code> off.</p>
         <p>
            <code>{}</code>
         </p> |
| `gateway_arn` | String | ✅ | <p>The ARN of the S3 File Gateway on which you want to create a file share.</p> |
| `kms_key` | String |  | <p>Optional. The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used
         for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric
         CMKs. This value must be set if <code>KMSEncrypted</code> is <code>true</code>, or if
            <code>EncryptionType</code> is <code>SseKms</code> or <code>DsseKms</code>.</p> |
| `authentication` | String |  | <p>The authentication method that users use to access the file share. The default is
            <code>ActiveDirectory</code>.</p>
         <p>Valid Values: <code>ActiveDirectory</code> | <code>GuestAccess</code>
         </p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to the NFS file share. Each tag is a
         key-value pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |
| `vpc_endpoint_dns_name` | String |  | <p>Specifies the DNS name for the VPC endpoint that the SMB file share uses to connect to
            Amazon S3.</p>
         <note>
            <p>This parameter is required for SMB file shares that connect to Amazon S3
            through a VPC endpoint, a VPC access point, or an access point alias that points to a
            VPC access point.</p>
         </note> |
| `default_storage_class` | String |  | <p>The default storage class for objects put into an Amazon S3 bucket by the S3
         File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
         <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> |
            <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code>
         </p> |
| `bucket_region` | String |  | <p>Specifies the Region of the S3 bucket where the SMB file share stores files.</p>
         <note>
            <p>This parameter is required for SMB file shares that connect to Amazon S3
            through a VPC endpoint, a VPC access point, or an access point alias that points to a
            VPC access point.</p>
         </note> |
| `smbacl_enabled` | bool |  | <p>Set this value to <code>true</code> to enable access control list (ACL) on the SMB file
         share. Set it to <code>false</code> to map file and directory permissions to the POSIX
         permissions.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/filegateway/latest/files3/smb-acl.html">Using Windows ACLs to limit SMB file share
            access</a> in the <i>Amazon S3 File Gateway User
         Guide</i>.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `cache_attributes` | String |  | <p>Specifies refresh cache information for the file share.</p> |
| `oplocks_enabled` | bool |  | <p>Specifies whether opportunistic locking is enabled for the SMB file share.</p>
         <note>
            <p>Enabling opportunistic locking on case-sensitive shares is not recommended for
            workloads that involve access to files with the same name in different case.</p>
         </note>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `object_acl` | String |  | <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket
         that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p> |
| `valid_user_list` | Vec<String> |  | <p>A list of users or groups in the Active Directory that are allowed to access the file
            <a href=""></a> share. A group must be prefixed with the @ character. Acceptable formats
         include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and
            <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to
            <code>ActiveDirectory</code>.</p> |
| `invalid_user_list` | Vec<String> |  | <p>A list of users or groups in the Active Directory that are not allowed to access the
         file share. A group must be prefixed with the @ character. Acceptable formats include:
            <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and
            <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to
            <code>ActiveDirectory</code>.</p> |
| `requester_pays` | bool |  | <p>A value that sets who pays the cost of the request and the cost associated with data
         download from the S3 bucket. If this value is set to <code>true</code>, the requester pays
         the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays
         the cost of storing data.</p>
         <note>
            <p>
               <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file
            share, so make sure that the configuration on the file share is the same as the S3
            bucket configuration.</p>
         </note>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `role` | String | ✅ | <p>The ARN of the Identity and Access Management (IAM) role that an S3 File Gateway assumes when it
         accesses the underlying storage.</p> |
| `encryption_type` | String |  | <p>A value that specifies the type of server-side encryption that the file share will use
         for the data that it stores in Amazon S3.</p>
         <note>
            <p>We recommend using <code>EncryptionType</code> instead of <code>KMSEncrypted</code>
            to set the file share encryption method. You do not need to provide values for both
            parameters.</p>
            <p>If values for both parameters exist in the same request, then the specified
            encryption methods must not conflict. For example, if <code>EncryptionType</code> is
               <code>SseS3</code>, then <code>KMSEncrypted</code> must be <code>false</code>. If
               <code>EncryptionType</code> is <code>SseKms</code> or <code>DsseKms</code>, then
               <code>KMSEncrypted</code> must be <code>true</code>.</p>
         </note> |
| `read_only` | bool |  | <p>A value that sets the write status of a file share. Set this value to <code>true</code>
         to set the write status to read-only, otherwise set to <code>false</code>.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `location_arn` | String | ✅ | <p>A custom ARN for the backend storage used for storing data for file shares. It includes
         a resource ARN with an optional prefix concatenation. The prefix must end with a forward
         slash (/).</p>
         <note>
            <p>You can specify LocationARN as a bucket ARN, access point ARN or access point alias,
            as shown in the following examples.</p>
            <p>Bucket ARN:</p>
            <p>
               <code>arn:aws:s3:::amzn-s3-demo-bucket/prefix/</code>
            </p>
            <p>Access point ARN:</p>
            <p>
               <code>arn:aws:s3:region:account-id:accesspoint/access-point-name/prefix/</code>
            </p>
            <p>If you specify an access point, the bucket policy must be configured to delegate
            access control to the access point. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-policies.html#access-points-delegating-control">Delegating access control to access points</a> in the <i>Amazon S3 User Guide</i>.</p>
            <p>Access point alias:</p>
            <p>
               <code>test-ap-ab123cdef4gehijklmn5opqrstuvuse1a-s3alias</code>
            </p>
         </note> |
| `client_token` | String | ✅ | <p>A unique string value that you supply that is used by S3 File Gateway to ensure
         idempotent file share creation.</p> |
| `guess_mime_type_enabled` | bool |  | <p>A value that enables guessing of the MIME type for uploaded objects based on file
         extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set
         to <code>false</code>. The default value is <code>true</code>.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `audit_destination_arn` | String |  | <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p> |
| `kms_encrypted` | bool |  | <p>Optional. Set to <code>true</code> to use Amazon S3 server-side encryption with
         your own KMS key (SSE-KMS), or <code>false</code> to use a key managed by
            Amazon S3 (SSE-S3). To use dual-layer encryption (DSSE-KMS), set the
            <code>EncryptionType</code> parameter instead.</p>
         <note>
            <p>We recommend using <code>EncryptionType</code> instead of <code>KMSEncrypted</code>
            to set the file share encryption method. You do not need to provide values for both
            parameters.</p>
            <p>If values for both parameters exist in the same request, then the specified
            encryption methods must not conflict. For example, if <code>EncryptionType</code> is
               <code>SseS3</code>, then <code>KMSEncrypted</code> must be <code>false</code>. If
               <code>EncryptionType</code> is <code>SseKms</code> or <code>DsseKms</code>, then
               <code>KMSEncrypted</code> must be <code>true</code>.</p>
         </note>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `admin_user_list` | Vec<String> |  | <p>A list of users or groups in the Active Directory that will be granted administrator
         privileges on the file share. These users can do all file operations as the super-user.
         Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>,
            <code>@group1</code>, and <code>@DOMAIN\group1</code>.</p>
         <important>
            <p>Use this option very carefully, because any user in this list can do anything they
            like on the file share, regardless of file permissions.</p>
         </important> |
| `access_based_enumeration` | bool |  | <p>The files and folders on this share will only be visible to users with read
         access.</p> |
| `case_sensitivity` | String |  | <p>The case of an object name in an Amazon S3 bucket. For
            <code>ClientSpecified</code>, the client determines the case sensitivity. For
            <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default
         value is <code>ClientSpecified</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create smb_file_share
smb_file_share = provider.storage_gateway.Smb_file_share {
    gateway_arn = "value"  # <p>The ARN of the S3 File Gateway on which you want to create a file share.</p>
    role = "value"  # <p>The ARN of the Identity and Access Management (IAM) role that an S3 File Gateway assumes when it
         accesses the underlying storage.</p>
    location_arn = "value"  # <p>A custom ARN for the backend storage used for storing data for file shares. It includes
         a resource ARN with an optional prefix concatenation. The prefix must end with a forward
         slash (/).</p>
         <note>
            <p>You can specify LocationARN as a bucket ARN, access point ARN or access point alias,
            as shown in the following examples.</p>
            <p>Bucket ARN:</p>
            <p>
               <code>arn:aws:s3:::amzn-s3-demo-bucket/prefix/</code>
            </p>
            <p>Access point ARN:</p>
            <p>
               <code>arn:aws:s3:region:account-id:accesspoint/access-point-name/prefix/</code>
            </p>
            <p>If you specify an access point, the bucket policy must be configured to delegate
            access control to the access point. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-policies.html#access-points-delegating-control">Delegating access control to access points</a> in the <i>Amazon S3 User Guide</i>.</p>
            <p>Access point alias:</p>
            <p>
               <code>test-ap-ab123cdef4gehijklmn5opqrstuvuse1a-s3alias</code>
            </p>
         </note>
    client_token = "value"  # <p>A unique string value that you supply that is used by S3 File Gateway to ensure
         idempotent file share creation.</p>
}

```

---


### Tape_with_barcode

TapeWithBarcode resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `worm` | bool |  | <p>Set to <code>TRUE</code> if the tape you are creating is to be configured as a
         write-once-read-many (WORM) tape.</p> |
| `kms_encrypted` | bool |  | <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own
            KMS key, or <code>false</code> to use a key managed by Amazon S3.
         Optional.</p>
         <p>Valid Values: <code>true</code> | <code>false</code>
         </p> |
| `tape_barcode` | String | ✅ | <p>The barcode that you want to assign to the tape.</p>
         <note>
            <p>Barcodes cannot be reused. This includes barcodes used for tapes that have been
            deleted.</p>
         </note> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to a virtual tape that has a barcode. Each
         tag is a key-value pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |
| `tape_size_in_bytes` | i64 | ✅ | <p>The size, in bytes, of the virtual tape that you want to create.</p>
         <note>
            <p>The size must be aligned by gigabyte (1024*1024*1024 bytes).</p>
         </note> |
| `kms_key` | String |  | <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This
         value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p> |
| `pool_id` | String |  | <p>The ID of the pool that you want to add your tape to for archiving. The tape in this
         pool is archived in the S3 storage class that is associated with the pool. When you use
         your backup application to eject the tape, the tape is archived directly into the storage
         class (S3 Glacier or S3 Deep Archive) that corresponds to the pool.</p> |
| `gateway_arn` | String | ✅ | <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the
         virtual tape with. Use the <a>ListGateways</a> operation to return a list of
         gateways for your account and Amazon Web Services Region.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tape_with_barcode
tape_with_barcode = provider.storage_gateway.Tape_with_barcode {
    tape_barcode = "value"  # <p>The barcode that you want to assign to the tape.</p>
         <note>
            <p>Barcodes cannot be reused. This includes barcodes used for tapes that have been
            deleted.</p>
         </note>
    tape_size_in_bytes = "value"  # <p>The size, in bytes, of the virtual tape that you want to create.</p>
         <note>
            <p>The size must be aligned by gigabyte (1024*1024*1024 bytes).</p>
         </note>
    gateway_arn = "value"  # <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the
         virtual tape with. Use the <a>ListGateways</a> operation to return a list of
         gateways for your account and Amazon Web Services Region.</p>
}

```

---


### Snapshot_schedule

SnapshotSchedule resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a>
         operation to return a list of gateway volumes.</p> |
| `start_at` | i64 | ✅ | <p>The hour of the day at which the snapshot schedule begins represented as
            <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour
         of the day is in the time zone of the gateway.</p> |
| `description` | String |  | <p>Optional description of the snapshot that overwrites the existing description.</p> |
| `tags` | Vec<String> |  | <p>A list of up to 50 tags that can be assigned to a snapshot. Each tag is a key-value
         pair.</p>
         <note>
            <p>Valid characters for key and value are letters, spaces, and numbers representable in
            UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length
            of a tag's key is 128 characters, and the maximum length for a tag's value is
            256.</p>
         </note> |
| `recurrence_in_hours` | i64 | ✅ | <p>Frequency of snapshots. Specify the number of hours between snapshots.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `timezone` | String | <p>A value that indicates the time zone of the gateway.</p> |
| `start_at` | i64 | <p>The hour of the day at which the snapshot schedule begins represented as
            <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour
         of the day is in the time zone of the gateway.</p> |
| `volume_arn` | String | <p>The Amazon Resource Name (ARN) of the volume that was specified in the request.</p> |
| `recurrence_in_hours` | i64 | <p>The number of hours between snapshots.</p> |
| `tags` | Vec<String> | <p>A list of up to 50 tags assigned to the snapshot schedule, sorted alphabetically by key
         name. Each tag is a key-value pair. For a gateway with more than 10 tags assigned, you can
         view all tags using the <code>ListTagsForResource</code> API operation.</p> |
| `description` | String | <p>The snapshot description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshot_schedule outputs
snapshot_schedule_id = snapshot_schedule.id
snapshot_schedule_timezone = snapshot_schedule.timezone
snapshot_schedule_start_at = snapshot_schedule.start_at
snapshot_schedule_volume_arn = snapshot_schedule.volume_arn
snapshot_schedule_recurrence_in_hours = snapshot_schedule.recurrence_in_hours
snapshot_schedule_tags = snapshot_schedule.tags
snapshot_schedule_description = snapshot_schedule.description
```

---


### Vtl_device_type

VTLDeviceType resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vtl_device_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the medium changer you want to select.</p> |
| `device_type` | String | ✅ | <p>The type of medium changer you want to select.</p>
         <p>Valid Values: <code>STK-L700</code> | <code>AWS-Gateway-VTL</code> |
            <code>IBM-03584L32-0402</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple snapshot resources
snapshot_0 = provider.storage_gateway.Snapshot {
    volume_arn = "value-0"
    snapshot_description = "value-0"
}
snapshot_1 = provider.storage_gateway.Snapshot {
    volume_arn = "value-1"
    snapshot_description = "value-1"
}
snapshot_2 = provider.storage_gateway.Snapshot {
    volume_arn = "value-2"
    snapshot_description = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    snapshot = provider.storage_gateway.Snapshot {
        volume_arn = "production-value"
        snapshot_description = "production-value"
    }
```

---

## Related Documentation

- [AWS Storage_gateway Documentation](https://docs.aws.amazon.com/storage_gateway/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
