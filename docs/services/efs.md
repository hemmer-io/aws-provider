# Efs Service



**Resources**: 15

---

## Overview

The efs service provides access to 15 resource types:

- [Account_preferences](#account_preferences) [CR]
- [Backup_policy](#backup_policy) [CR]
- [File_system](#file_system) [CUD]
- [File_systems](#file_systems) [R]
- [Access_point](#access_point) [CD]
- [Access_points](#access_points) [R]
- [Lifecycle_configuration](#lifecycle_configuration) [CR]
- [Replication_configuration](#replication_configuration) [CD]
- [Mount_target_security_groups](#mount_target_security_groups) [R]
- [Replication_configurations](#replication_configurations) [R]
- [Tags](#tags) [CRD]
- [Mount_target](#mount_target) [CD]
- [File_system_policy](#file_system_policy) [CRD]
- [Mount_targets](#mount_targets) [R]
- [File_system_protection](#file_system_protection) [U]

---

## Resources


### Account_preferences

AccountPreferences resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_id_type` | String | ✅ | <p>Specifies the EFS resource ID preference to set for the user's Amazon Web Services account, in the current Amazon Web Services Region, either <code>LONG_ID</code>
      (17 characters), or <code>SHORT_ID</code> (8 characters).</p>
         <note>
            <p>Starting in October, 2021, you will receive an error when setting the account preference to
          <code>SHORT_ID</code>. Contact Amazon Web Services support if you receive an error and must
        use short IDs for file system and mount target resources.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Present if there are more records than returned in the response. 
      You can use the <code>NextToken</code> in the subsequent request to fetch the additional descriptions.</p> |
| `resource_id_preference` | String | <p>Describes the resource ID preference setting for the Amazon Web Services account associated with the user making the request, in the current Amazon Web Services Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_preferences
account_preferences = provider.efs.Account_preferences {
    resource_id_type = "value"  # <p>Specifies the EFS resource ID preference to set for the user's Amazon Web Services account, in the current Amazon Web Services Region, either <code>LONG_ID</code>
      (17 characters), or <code>SHORT_ID</code> (8 characters).</p>
         <note>
            <p>Starting in October, 2021, you will receive an error when setting the account preference to
          <code>SHORT_ID</code>. Contact Amazon Web Services support if you receive an error and must
        use short IDs for file system and mount target resources.</p>
         </note>
}

# Access account_preferences outputs
account_preferences_id = account_preferences.id
account_preferences_next_token = account_preferences.next_token
account_preferences_resource_id_preference = account_preferences.resource_id_preference
```

---


### Backup_policy

BackupPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_policy` | String | ✅ | <p>The backup policy included in the <code>PutBackupPolicy</code> request.</p> |
| `file_system_id` | String | ✅ | <p>Specifies which EFS file system to update the backup policy for.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_policy` | String | <p>Describes the file system's backup policy, indicating whether automatic backups are
      turned on or off.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup_policy
backup_policy = provider.efs.Backup_policy {
    backup_policy = "value"  # <p>The backup policy included in the <code>PutBackupPolicy</code> request.</p>
    file_system_id = "value"  # <p>Specifies which EFS file system to update the backup policy for.</p>
}

# Access backup_policy outputs
backup_policy_id = backup_policy.id
backup_policy_backup_policy = backup_policy.backup_policy
```

---


### File_system

FileSystem resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `performance_mode` | String |  | <p>The performance mode of the file system. We recommend <code>generalPurpose</code>
      performance mode for all file systems. File systems using the <code>maxIO</code> performance
      mode can scale to higher levels of aggregate throughput and operations per second with a
      tradeoff of slightly higher latencies for most file operations. The performance mode
      can't be changed after the file system has been created. The <code>maxIO</code> mode is
      not supported on One Zone file systems.</p>
         <important>
            <p>Due to the higher per-operation latencies with Max I/O, we recommend using General Purpose performance mode for all file systems.</p>
         </important>
         <p>Default is <code>generalPurpose</code>.</p> |
| `creation_token` | String | ✅ | <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent
      creation.</p> |
| `availability_zone_name` | String |  | <p>For One Zone file systems, specify the Amazon Web Services
      Availability Zone in which to create the file system. Use the format <code>us-east-1a</code> to
      specify the  Availability Zone. For more information about One Zone file systems, see
      <a href="https://docs.aws.amazon.com/efs/latest/ug/availability-durability.html#file-system-type">EFS file system types</a> in the <i>Amazon EFS User Guide</i>.</p>
         <note>
            <p>One Zone file systems are not available in all Availability Zones in Amazon Web Services Regions where Amazon EFS is available.</p>
         </note> |
| `encrypted` | bool |  | <p>A Boolean value that, if true, creates an encrypted file system. When creating an
      encrypted file system, you have the option of specifying an existing Key Management Service key (KMS key).
      If you don't specify a KMS key, then the default KMS key for
      Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system.
    </p> |
| `backup` | bool |  | <p>Specifies whether automatic backups are enabled on the file system that you are creating.
      Set the value to <code>true</code> to enable automatic backups. If you are creating a
      One Zone file system, automatic backups are enabled by default. For more
      information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/awsbackup.html#automatic-backups">Automatic backups</a> in the
          <i>Amazon EFS User Guide</i>.</p>
         <p>Default is <code>false</code>. However, if you specify an <code>AvailabilityZoneName</code>, 
      the default is <code>true</code>.</p>
         <note>
            <p>Backup is not available in all Amazon Web Services Regions where Amazon EFS is available.</p>
         </note> |
| `throughput_mode` | String |  | <p>Specifies the throughput mode for the file system. The mode can be <code>bursting</code>,
        <code>provisioned</code>, or <code>elastic</code>. If you set <code>ThroughputMode</code> to
        <code>provisioned</code>, you must also set a value for
        <code>ProvisionedThroughputInMibps</code>. After you create the file system, you can
      decrease your file system's Provisioned throughput or change between the
      throughput modes, with certain time restrictions. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput">Specifying
        throughput with provisioned mode</a> in the <i>Amazon EFS User
        Guide</i>. </p>
         <p>Default is <code>bursting</code>.</p> |
| `kms_key_id` | String |  | <p>The ID of the KMS key that you want to use to protect the encrypted file
      system. This parameter is required only if you want to use a non-default KMS key. If this parameter is not specified, the default KMS key for Amazon EFS is used. You can specify a KMS key ID using the following
      formats:</p>
         <ul>
            <li>
               <p>Key ID - A unique identifier of the key, for example
            <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
            </li>
            <li>
               <p>ARN - An Amazon Resource Name (ARN) for the key, for example
            <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p>
            </li>
            <li>
               <p>Key alias - A previously created display name for a key, for example
            <code>alias/projectKey1</code>.</p>
            </li>
            <li>
               <p>Key alias ARN - An ARN for a key alias, for example
            <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p>
            </li>
         </ul>
         <p>If you use <code>KmsKeyId</code>, you must set the <a>CreateFileSystemRequest$Encrypted</a> 
      parameter to true.</p>
         <important>
            <p>EFS accepts only symmetric KMS keys. You cannot use asymmetric 
      KMS keys with Amazon EFS file systems.</p>
         </important> |
| `tags` | Vec<String> |  | <p>Use to create one or more tags associated with the file system. Each
        tag is a user-defined key-value pair. Name your file system on creation by including a
        <code>"Key":"Name","Value":"{value}"</code> key-value pair. Each key must be unique. For more 
        information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>
        in the <i>Amazon Web Services General Reference Guide</i>.</p> |
| `provisioned_throughput_in_mibps` | f64 |  | <p>The throughput, measured in mebibytes per second (MiBps), that you want to provision for a
      file system that you're creating. Required if <code>ThroughputMode</code> is set to
        <code>provisioned</code>. Valid values are 1-3414 MiBps, with the upper limit depending on
      Region. To increase this limit, contact Amazon Web ServicesSupport. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS quotas
        that you can increase</a> in the <i>Amazon EFS User
      Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create file_system
file_system = provider.efs.File_system {
    creation_token = "value"  # <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent
      creation.</p>
}

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
| `marker` | String | <p>Present if provided by caller in the request (String).</p> |
| `file_systems` | Vec<String> | <p>An array of file system descriptions.</p> |
| `next_marker` | String | <p>Present if there are more file systems than returned in the response (String). You can
      use the <code>NextMarker</code> in the subsequent request to fetch the descriptions.</p> |


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
file_systems_marker = file_systems.marker
file_systems_file_systems = file_systems.file_systems
file_systems_next_marker = file_systems.next_marker
```

---


### Access_point

AccessPoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String | ✅ | <p>A string of up to 64 ASCII characters that Amazon EFS uses to ensure idempotent
      creation.</p> |
| `file_system_id` | String | ✅ | <p>The ID of the EFS file system that the access point provides access to.</p> |
| `root_directory` | String |  | <p>Specifies the directory on the EFS file system that the access point exposes as
      the root directory of your file system to NFS clients using the access point. The clients
      using the access point can only access the root directory and below. If the
        <code>RootDirectory</code> > <code>Path</code> specified does not exist, Amazon EFS creates it and applies the <code>CreationInfo</code> settings when a client connects to an
      access point. When specifying a <code>RootDirectory</code>, you must provide the
        <code>Path</code>, and the <code>CreationInfo</code>.</p>
         <p>Amazon EFS creates a root directory only if you have provided the  CreationInfo: OwnUid, OwnGID, and permissions for the directory. 
      If  you do not provide this information, Amazon EFS does not create the root directory. If the root directory does not exist, attempts to mount 
      using the access point will fail.</p> |
| `tags` | Vec<String> |  | <p>Creates tags associated with the access point. Each tag is a key-value pair, each key must be unique. For more 
      information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>
      in the <i>Amazon Web Services General Reference Guide</i>.</p> |
| `posix_user` | String |  | <p>The operating system user and
      group applied to all file system requests made using the access point.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_point
access_point = provider.efs.Access_point {
    client_token = "value"  # <p>A string of up to 64 ASCII characters that Amazon EFS uses to ensure idempotent
      creation.</p>
    file_system_id = "value"  # <p>The ID of the EFS file system that the access point provides access to.</p>
}

```

---


### Access_points

AccessPoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_points` | Vec<String> | <p>An array of access point descriptions.</p> |
| `next_token` | String | <p>Present if there are more access points than returned in the response. 
      You can use the NextMarker in the subsequent request to fetch the additional descriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_points outputs
access_points_id = access_points.id
access_points_access_points = access_points.access_points
access_points_next_token = access_points.next_token
```

---


### Lifecycle_configuration

LifecycleConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_system_id` | String | ✅ | <p>The ID of the file system for which you are creating the
        <code>LifecycleConfiguration</code> object (String).</p> |
| `lifecycle_policies` | Vec<String> | ✅ | <p>An array of <code>LifecyclePolicy</code> objects that define the file system's
        <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object
      informs lifecycle management of the following:</p>
         <ul>
            <li>
               <p>
                  <b>
                     <code>TransitionToIA</code>
                  </b> – 
      When to move files in the file system from primary storage (Standard storage class) into the Infrequent Access 
        (IA) storage.</p>
            </li>
            <li>
               <p>
                  <b>
                     <code>TransitionToArchive</code>
                  </b> –
          When to move files in the file system from their current storage class (either IA or Standard storage) into the 
         Archive storage.</p>
               <p>File systems cannot transition into Archive storage before transitioning into IA  storage. Therefore,  
        TransitionToArchive must either not be set or must be later than TransitionToIA.</p>
               <note>
                  <p>The Archive storage class is available only for file systems that use the Elastic throughput mode 
and the General Purpose performance mode. </p>
               </note>
            </li>
            <li>
               <p>
                  <b>
                     <code>TransitionToPrimaryStorageClass</code>
                  </b> – Whether to move files in the file system back to primary storage (Standard storage class) after they are accessed in IA
        or Archive storage.</p>
            </li>
         </ul>
         <note>
            <p>When using the <code>put-lifecycle-configuration</code> CLI command or the
          <code>PutLifecycleConfiguration</code> API action, Amazon EFS requires that each
          <code>LifecyclePolicy</code> object have only a single transition. This means that in a
        request body, <code>LifecyclePolicies</code> must be structured as an array of
          <code>LifecyclePolicy</code> objects, one object for each storage transition. See the example
        requests in the following section for more information.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_policies` | Vec<String> | <p>An array of lifecycle management policies. EFS supports a maximum of one
      policy per file system.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_configuration
lifecycle_configuration = provider.efs.Lifecycle_configuration {
    file_system_id = "value"  # <p>The ID of the file system for which you are creating the
        <code>LifecycleConfiguration</code> object (String).</p>
    lifecycle_policies = "value"  # <p>An array of <code>LifecyclePolicy</code> objects that define the file system's
        <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object
      informs lifecycle management of the following:</p>
         <ul>
            <li>
               <p>
                  <b>
                     <code>TransitionToIA</code>
                  </b> – 
      When to move files in the file system from primary storage (Standard storage class) into the Infrequent Access 
        (IA) storage.</p>
            </li>
            <li>
               <p>
                  <b>
                     <code>TransitionToArchive</code>
                  </b> –
          When to move files in the file system from their current storage class (either IA or Standard storage) into the 
         Archive storage.</p>
               <p>File systems cannot transition into Archive storage before transitioning into IA  storage. Therefore,  
        TransitionToArchive must either not be set or must be later than TransitionToIA.</p>
               <note>
                  <p>The Archive storage class is available only for file systems that use the Elastic throughput mode 
and the General Purpose performance mode. </p>
               </note>
            </li>
            <li>
               <p>
                  <b>
                     <code>TransitionToPrimaryStorageClass</code>
                  </b> – Whether to move files in the file system back to primary storage (Standard storage class) after they are accessed in IA
        or Archive storage.</p>
            </li>
         </ul>
         <note>
            <p>When using the <code>put-lifecycle-configuration</code> CLI command or the
          <code>PutLifecycleConfiguration</code> API action, Amazon EFS requires that each
          <code>LifecyclePolicy</code> object have only a single transition. This means that in a
        request body, <code>LifecyclePolicies</code> must be structured as an array of
          <code>LifecyclePolicy</code> objects, one object for each storage transition. See the example
        requests in the following section for more information.</p>
         </note>
}

# Access lifecycle_configuration outputs
lifecycle_configuration_id = lifecycle_configuration.id
lifecycle_configuration_lifecycle_policies = lifecycle_configuration.lifecycle_policies
```

---


### Replication_configuration

ReplicationConfiguration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_file_system_id` | String | ✅ | <p>Specifies the Amazon EFS file system that you want to replicate. This file system cannot already be 
    a source or destination file system in another replication configuration.</p> |
| `destinations` | Vec<String> | ✅ | <p>An array of destination configuration objects. Only one destination configuration object is supported.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_configuration
replication_configuration = provider.efs.Replication_configuration {
    source_file_system_id = "value"  # <p>Specifies the Amazon EFS file system that you want to replicate. This file system cannot already be 
    a source or destination file system in another replication configuration.</p>
    destinations = "value"  # <p>An array of destination configuration objects. Only one destination configuration object is supported.</p>
}

```

---


### Mount_target_security_groups

MountTargetSecurityGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_groups` | Vec<String> | <p>An array of security groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mount_target_security_groups outputs
mount_target_security_groups_id = mount_target_security_groups.id
mount_target_security_groups_security_groups = mount_target_security_groups.security_groups
```

---


### Replication_configurations

ReplicationConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replications` | Vec<String> | <p>The collection of replication configurations that is returned.</p> |
| `next_token` | String | <p>You can use the <code>NextToken</code> from the previous response in a subsequent 
      request to fetch the additional descriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_configurations outputs
replication_configurations_id = replication_configurations.id
replication_configurations_replications = replication_configurations.replications
replication_configurations_next_token = replication_configurations.next_token
```

---


### Tags

Tags resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_system_id` | String | ✅ | <p>The ID of the file system whose tags you want to modify (String). This operation modifies
      the tags only, not the file system.</p> |
| `tags` | Vec<String> | ✅ | <p>An array of <code>Tag</code> objects to add. Each <code>Tag</code> object is a key-value
      pair. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_marker` | String | <p>If a value is present, there are more tags to return. In a subsequent request, you can
      provide the value of <code>NextMarker</code> as the value of the <code>Marker</code> parameter
      in your next request to retrieve the next set of tags.</p> |
| `tags` | Vec<String> | <p>Returns tags associated with the file system as an array of <code>Tag</code> objects.
    </p> |
| `marker` | String | <p>If the request included a <code>Marker</code>, the response returns that value in this
      field.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tags
tags = provider.efs.Tags {
    file_system_id = "value"  # <p>The ID of the file system whose tags you want to modify (String). This operation modifies
      the tags only, not the file system.</p>
    tags = "value"  # <p>An array of <code>Tag</code> objects to add. Each <code>Tag</code> object is a key-value
      pair. </p>
}

# Access tags outputs
tags_id = tags.id
tags_next_marker = tags.next_marker
tags_tags = tags.tags
tags_marker = tags.marker
```

---


### Mount_target

MountTarget resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subnet_id` | String | ✅ | <p>The ID of the subnet to add the mount target in. For One Zone file systems, use the subnet
      that is associated with the file system's Availability Zone.</p> |
| `ipv6_address` | String |  | <p>If the IP address type for the mount target is IPv6, then specify the IPv6 address within
      the address range of the specified subnet.</p> |
| `file_system_id` | String | ✅ | <p>The ID of the file system for which to create the mount target.</p> |
| `ip_address_type` | String |  | <p>Specify the type of IP address of the mount target you are creating. Options are IPv4,
      dual stack, or IPv6. If you don’t specify an IpAddressType, then IPv4 is used.</p>
         <ul>
            <li>
               <p>IPV4_ONLY – Create mount target with IPv4 only subnet or dual-stack subnet.</p>
            </li>
            <li>
               <p>DUAL_STACK – Create mount target with dual-stack subnet.</p>
            </li>
            <li>
               <p>IPV6_ONLY – Create mount target with IPv6 only subnet.</p>
            </li>
         </ul>
         <note>
            <p>Creating IPv6 mount target only ENI in dual-stack subnet is not supported.</p>
         </note> |
| `security_groups` | Vec<String> |  | <p>VPC security group IDs, of the form <code>sg-xxxxxxxx</code>. These must be for the same
      VPC as the subnet specified. The maximum number of security groups depends on account quota.
      For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/amazon-vpc-limits.html">Amazon VPC Quotas</a>
      in the <i>Amazon VPC User Guide</i> (see the <b>Security Groups</b>
      table).
    </p> |
| `ip_address` | String |  | <p>If the IP address type for the mount target is IPv4, then specify the IPv4 address within
      the address range of the specified subnet.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create mount_target
mount_target = provider.efs.Mount_target {
    subnet_id = "value"  # <p>The ID of the subnet to add the mount target in. For One Zone file systems, use the subnet
      that is associated with the file system's Availability Zone.</p>
    file_system_id = "value"  # <p>The ID of the file system for which to create the mount target.</p>
}

```

---


### File_system_policy

FileSystemPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bypass_policy_lockout_safety_check` | bool |  | <p>(Optional) A boolean that specifies whether or not to bypass the <code>FileSystemPolicy</code> lockout safety check. The lockout safety check 
      determines whether the policy in the request will lock out, or prevent, the IAM principal that is making the request from making future <code>PutFileSystemPolicy</code> requests on this file system.
      Set <code>BypassPolicyLockoutSafetyCheck</code> to <code>True</code> only when you intend to prevent 
      the IAM principal that is making the request from making subsequent <code>PutFileSystemPolicy</code> requests on this file system. 
      The default value is <code>False</code>.
    </p> |
| `policy` | String | ✅ | <p>The <code>FileSystemPolicy</code> that you're creating. Accepts a JSON formatted
      policy definition. EFS file system policies have a 20,000 character limit. To find
      out more about the elements that make up a file system policy, see <a href="https://docs.aws.amazon.com/efs/latest/ug/security_iam_service-with-iam.html#security_iam_service-with-iam-resource-based-policies">Resource-based policies within Amazon EFS</a>. </p> |
| `file_system_id` | String | ✅ | <p>The ID of the EFS file system that you want to create or update the
        <code>FileSystemPolicy</code> for.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_system_id` | String | <p>Specifies the EFS file system to which the <code>FileSystemPolicy</code>
      applies.</p> |
| `policy` | String | <p>The JSON formatted <code>FileSystemPolicy</code> for the EFS file
      system.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create file_system_policy
file_system_policy = provider.efs.File_system_policy {
    policy = "value"  # <p>The <code>FileSystemPolicy</code> that you're creating. Accepts a JSON formatted
      policy definition. EFS file system policies have a 20,000 character limit. To find
      out more about the elements that make up a file system policy, see <a href="https://docs.aws.amazon.com/efs/latest/ug/security_iam_service-with-iam.html#security_iam_service-with-iam-resource-based-policies">Resource-based policies within Amazon EFS</a>. </p>
    file_system_id = "value"  # <p>The ID of the EFS file system that you want to create or update the
        <code>FileSystemPolicy</code> for.</p>
}

# Access file_system_policy outputs
file_system_policy_id = file_system_policy.id
file_system_policy_file_system_id = file_system_policy.file_system_id
file_system_policy_policy = file_system_policy.policy
```

---


### Mount_targets

MountTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_marker` | String | <p>If a value is present, there are more mount targets to return. In a subsequent request,
      you can provide <code>Marker</code> in your request with this value to retrieve the next set
      of mount targets.</p> |
| `marker` | String | <p>If the request included the <code>Marker</code>, the response returns that value in
      this field.</p> |
| `mount_targets` | Vec<String> | <p>Returns the file system's mount targets as an array of
        <code>MountTargetDescription</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mount_targets outputs
mount_targets_id = mount_targets.id
mount_targets_next_marker = mount_targets.next_marker
mount_targets_marker = mount_targets.marker
mount_targets_mount_targets = mount_targets.mount_targets
```

---


### File_system_protection

FileSystemProtection resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replication_overwrite_protection` | String |  | <p>The status of the file system's replication overwrite protection.</p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> – The file system cannot be used as the destination file
          system in a replication configuration. The file system is writeable. Replication overwrite
          protection is <code>ENABLED</code> by default. </p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> – The file system can be used as the destination file
          system in a replication configuration. The file system is read-only and can only be
          modified by EFS replication.</p>
            </li>
            <li>
               <p>
                  <code>REPLICATING</code> – The file system is being used as the destination file
          system in a replication configuration. The file system is read-only and is only modified
          only by EFS replication.</p>
            </li>
         </ul>
         <p>If the replication configuration is deleted, the file system's replication overwrite
      protection is re-enabled and the file system becomes writeable.</p> |
| `file_system_id` | String | ✅ | <p>The ID of the file system to update. </p> |



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

# Create multiple account_preferences resources
account_preferences_0 = provider.efs.Account_preferences {
    resource_id_type = "value-0"
}
account_preferences_1 = provider.efs.Account_preferences {
    resource_id_type = "value-1"
}
account_preferences_2 = provider.efs.Account_preferences {
    resource_id_type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_preferences = provider.efs.Account_preferences {
        resource_id_type = "production-value"
    }
```

---

## Related Documentation

- [AWS Efs Documentation](https://docs.aws.amazon.com/efs/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
