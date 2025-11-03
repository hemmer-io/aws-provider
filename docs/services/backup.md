# Backup Service



**Resources**: 31

---

## Overview

The backup service provides access to 31 resource types:

- [Framework](#framework) [CRUD]
- [Recovery_point_lifecycle](#recovery_point_lifecycle) [U]
- [Restore_testing_plan](#restore_testing_plan) [CRUD]
- [Region_settings](#region_settings) [RU]
- [Restore_testing_selection](#restore_testing_selection) [CRUD]
- [Recovery_point](#recovery_point) [RD]
- [Backup_vault](#backup_vault) [CRD]
- [Restore_job_metadata](#restore_job_metadata) [R]
- [Backup_plan](#backup_plan) [CRUD]
- [Backup_job](#backup_job) [R]
- [Protected_resource](#protected_resource) [R]
- [Recovery_point_index_details](#recovery_point_index_details) [R]
- [Report_plan](#report_plan) [CRUD]
- [Restore_access_backup_vault](#restore_access_backup_vault) [C]
- [Supported_resource_types](#supported_resource_types) [R]
- [Restore_job](#restore_job) [R]
- [Restore_validation_result](#restore_validation_result) [C]
- [Backup_plan_from_json](#backup_plan_from_json) [R]
- [Backup_vault_access_policy](#backup_vault_access_policy) [CRD]
- [Backup_vault_lock_configuration](#backup_vault_lock_configuration) [CD]
- [Restore_testing_inferred_metadata](#restore_testing_inferred_metadata) [R]
- [Backup_selection](#backup_selection) [CRD]
- [Legal_hold](#legal_hold) [CR]
- [Copy_job](#copy_job) [R]
- [Recovery_point_index_settings](#recovery_point_index_settings) [U]
- [Report_job](#report_job) [R]
- [Backup_vault_notifications](#backup_vault_notifications) [CRD]
- [Logically_air_gapped_backup_vault](#logically_air_gapped_backup_vault) [C]
- [Global_settings](#global_settings) [RU]
- [Recovery_point_restore_metadata](#recovery_point_restore_metadata) [R]
- [Backup_plan_from_template](#backup_plan_from_template) [R]

---

## Resources


### Framework

Framework resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `framework_description` | String |  | <p>An optional description of the framework with a maximum of 1,024 characters.</p> |
| `idempotency_token` | String |  | <p>A customer-chosen string that you can use to distinguish between otherwise identical
         calls to <code>CreateFrameworkInput</code>. Retrying a successful request with the same
         idempotency token results in a success message with no action taken.</p> |
| `framework_controls` | Vec<String> | ✅ | <p>The controls that make up the framework. Each control in the list has a name,
         input parameters, and scope.</p> |
| `framework_tags` | HashMap<String, String> |  | <p>The tags to assign to the framework.</p> |
| `framework_name` | String | ✅ | <p>The unique name of the framework. The name must be between 1 and 256 characters,
         starting with a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and
         underscores (_).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `framework_description` | String | <p>An optional description of the framework.</p> |
| `framework_status` | String | <p>A framework consists of one or more controls. Each control governs a resource, such as
         backup plans, backup selections, backup vaults, or recovery points. You can also turn
            Config recording on or off for each resource. The statuses are:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code> when recording is turned on for all resources governed by the
               framework.</p>
            </li>
            <li>
               <p>
                  <code>PARTIALLY_ACTIVE</code> when recording is turned off for at least one
               resource governed by the framework.</p>
            </li>
            <li>
               <p>
                  <code>INACTIVE</code> when recording is turned off for all resources governed by
               the framework.</p>
            </li>
            <li>
               <p>
                  <code>UNAVAILABLE</code> when Backup is unable to validate recording
               status at this time.</p>
            </li>
         </ul> |
| `idempotency_token` | String | <p>A customer-chosen string that you can use to distinguish between otherwise identical
         calls to <code>DescribeFrameworkOutput</code>. Retrying a successful request with the same
         idempotency token results in a success message with no action taken.</p> |
| `framework_arn` | String | <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN
         depends on the resource type.</p> |
| `framework_name` | String | <p>The unique name of a framework.</p> |
| `framework_controls` | Vec<String> | <p>The controls that make up the framework. Each control in the list has a name,
         input parameters, and scope.</p> |
| `creation_time` | String | <p>The date and time that a framework is created, in ISO 8601 representation. The value of <code>CreationTime</code> is accurate to milliseconds. For example,
         2020-07-10T15:00:00.000-08:00 represents the 10th of July 2020 at 3:00 PM 8 hours behind
         UTC.</p> |
| `deployment_status` | String | <p>The deployment status of a framework. The statuses are:</p>
         <p>
            <code>CREATE_IN_PROGRESS | UPDATE_IN_PROGRESS | DELETE_IN_PROGRESS | COMPLETED |
            FAILED</code>
         </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create framework
framework = provider.backup.Framework {
    framework_controls = "value"  # <p>The controls that make up the framework. Each control in the list has a name,
         input parameters, and scope.</p>
    framework_name = "value"  # <p>The unique name of the framework. The name must be between 1 and 256 characters,
         starting with a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and
         underscores (_).</p>
}

# Access framework outputs
framework_id = framework.id
framework_framework_description = framework.framework_description
framework_framework_status = framework.framework_status
framework_idempotency_token = framework.idempotency_token
framework_framework_arn = framework.framework_arn
framework_framework_name = framework.framework_name
framework_framework_controls = framework.framework_controls
framework_creation_time = framework.creation_time
framework_deployment_status = framework.deployment_status
```

---


### Recovery_point_lifecycle

RecoveryPointLifecycle resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_vault_name` | String | ✅ | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created.</p> |
| `lifecycle` | String |  | <p>The lifecycle defines when a protected resource is transitioned to cold storage and when
         it expires. Backup transitions and expires backups automatically according to
         the lifecycle that you define. </p>
         <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90
         days. Therefore, the “retention” setting must be 90 days greater than the “transition to
         cold after days” setting. The “transition to cold after days” setting cannot be changed
         after a backup has been transitioned to cold. </p> |
| `recovery_point_arn` | String | ✅ | <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example,
            <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |



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


### Restore_testing_plan

RestoreTestingPlan resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restore_testing_plan` | String | ✅ | <p>A restore testing plan must contain a unique <code>RestoreTestingPlanName</code> string
         you create and must contain a <code>ScheduleExpression</code> cron. You may optionally
         include a <code>StartWindowHours</code> integer and a <code>CreatorRequestId</code>
         string.</p>
         <p>The <code>RestoreTestingPlanName</code> is a unique string that is the name of the
         restore testing plan. This cannot be changed after creation, and it must consist of only
         alphanumeric characters and underscores.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to assign to the restore testing plan.</p> |
| `creator_request_id` | String |  | <p>This is a unique string that identifies the request and 
         allows failed requests to be retriedwithout the risk of running 
         the operation twice. This parameter is optional. If used, this 
         parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restore_testing_plan` | String | <p>Specifies the body of a restore testing plan. Includes 
         <code>RestoreTestingPlanName</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create restore_testing_plan
restore_testing_plan = provider.backup.Restore_testing_plan {
    restore_testing_plan = "value"  # <p>A restore testing plan must contain a unique <code>RestoreTestingPlanName</code> string
         you create and must contain a <code>ScheduleExpression</code> cron. You may optionally
         include a <code>StartWindowHours</code> integer and a <code>CreatorRequestId</code>
         string.</p>
         <p>The <code>RestoreTestingPlanName</code> is a unique string that is the name of the
         restore testing plan. This cannot be changed after creation, and it must consist of only
         alphanumeric characters and underscores.</p>
}

# Access restore_testing_plan outputs
restore_testing_plan_id = restore_testing_plan.id
restore_testing_plan_restore_testing_plan = restore_testing_plan.restore_testing_plan
```

---


### Region_settings

RegionSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_type_opt_in_preference` | HashMap<String, bool> |  | <p>Updates the list of services along with the opt-in preferences for the Region.</p>
         <p>If resource assignments are only based on tags, then service opt-in settings are applied. 
         If a resource type is explicitly assigned to a backup plan, such as Amazon S3, 
         Amazon EC2, or Amazon RDS, it will be included in the 
         backup even if the opt-in is not enabled for that particular service. 
         If both a resource type and tags are specified in a resource assignment, 
         the resource type specified in the backup plan takes priority over the 
         tag condition. Service opt-in settings are disregarded in this situation.</p> |
| `resource_type_management_preference` | HashMap<String, bool> |  | <p>Enables or disables full Backup management of backups for a resource type.
         To enable full Backup management for DynamoDB along with <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html">
            Backup's advanced DynamoDB backup features</a>, follow the
         procedure to <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html#advanced-ddb-backup-enable-cli"> enable advanced DynamoDB backup programmatically</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_type_management_preference` | HashMap<String, bool> | <p>Returns whether Backup fully manages the backups for a resource type.</p>
         <p>For the benefits of full Backup management, see <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html#full-management">Full Backup 
         management</a>.</p>
         <p>For a list of resource types and whether each supports full Backup management, 
         see the <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/backup-feature-availability.html#features-by-resource">Feature availability by resource</a> table.</p>
         <p>If <code>"DynamoDB":false</code>, you can enable full Backup management for
         DynamoDB backup by enabling <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/advanced-ddb-backup.html#advanced-ddb-backup-enable-cli">
            Backup's advanced DynamoDB backup features</a>.</p> |
| `resource_type_opt_in_preference` | HashMap<String, bool> | <p>The services along with the opt-in preferences in the Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access region_settings outputs
region_settings_id = region_settings.id
region_settings_resource_type_management_preference = region_settings.resource_type_management_preference
region_settings_resource_type_opt_in_preference = region_settings.resource_type_opt_in_preference
```

---


### Restore_testing_selection

RestoreTestingSelection resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restore_testing_plan_name` | String | ✅ | <p>Input the restore testing plan name that was returned from the 
         related CreateRestoreTestingPlan request.</p> |
| `creator_request_id` | String |  | <p>This is an optional unique string that identifies the request and allows 
         failed requests to be retried without the risk of running the operation 
         twice. If used, this parameter must contain 
         1 to 50 alphanumeric or '-_.' characters.</p> |
| `restore_testing_selection` | String | ✅ | <p>This consists of <code>RestoreTestingSelectionName</code>,
            <code>ProtectedResourceType</code>, and one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>ProtectedResourceArns</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ProtectedResourceConditions</code>
               </p>
            </li>
         </ul>
         <p>Each protected resource type can have one single value.</p>
         <p>A restore testing selection can include a wildcard value ("*") for
            <code>ProtectedResourceArns</code> along with <code>ProtectedResourceConditions</code>.
         Alternatively, you can include up to 30 specific protected resource ARNs in
            <code>ProtectedResourceArns</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restore_testing_selection` | String | <p>Unique name of the restore testing selection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create restore_testing_selection
restore_testing_selection = provider.backup.Restore_testing_selection {
    restore_testing_plan_name = "value"  # <p>Input the restore testing plan name that was returned from the 
         related CreateRestoreTestingPlan request.</p>
    restore_testing_selection = "value"  # <p>This consists of <code>RestoreTestingSelectionName</code>,
            <code>ProtectedResourceType</code>, and one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>ProtectedResourceArns</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ProtectedResourceConditions</code>
               </p>
            </li>
         </ul>
         <p>Each protected resource type can have one single value.</p>
         <p>A restore testing selection can include a wildcard value ("*") for
            <code>ProtectedResourceArns</code> along with <code>ProtectedResourceConditions</code>.
         Alternatively, you can include up to 30 specific protected resource ARNs in
            <code>ProtectedResourceArns</code>.</p>
}

# Access restore_testing_selection outputs
restore_testing_selection_id = restore_testing_selection.id
restore_testing_selection_restore_testing_selection = restore_testing_selection.restore_testing_selection
```

---


### Recovery_point

RecoveryPoint resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `calculated_lifecycle` | String | <p>A <code>CalculatedLifecycle</code> object containing <code>DeleteAt</code> and
            <code>MoveToColdStorageAt</code> timestamps.</p> |
| `resource_name` | String | <p>The name of the resource that belongs to the specified backup.</p> |
| `recovery_point_arn` | String | <p>An ARN that uniquely identifies a recovery point; for example,
            <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |
| `source_backup_vault_arn` | String | <p>An Amazon Resource Name (ARN) that uniquely identifies the source vault where the
         resource was originally backed up in; for example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>. If the recovery is
         restored to the same Amazon Web Services account or Region, this value will be
            <code>null</code>.</p> |
| `iam_role_arn` | String | <p>Specifies the IAM role ARN used to create the target recovery point; for example,
            <code>arn:aws:iam::123456789012:role/S3Access</code>.</p> |
| `is_parent` | bool | <p>This returns the boolean value that a recovery point is a parent (composite) job.</p> |
| `vault_type` | String | <p>The type of vault in which the described recovery point is stored.</p> |
| `is_encrypted` | bool | <p>A Boolean value that is returned as <code>TRUE</code> if the specified recovery point is
         encrypted, or <code>FALSE</code> if the recovery point is not encrypted.</p> |
| `resource_arn` | String | <p>An ARN that uniquely identifies a saved resource. The format of the ARN depends on the
         resource type.</p> |
| `status` | String | <p>A status code specifying the state of the recovery point. For more information, see 
         <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/applicationstackbackups.html#cfnrecoverypointstatus">
            Recovery point status</a> in the <i>Backup Developer 
               Guide</i>.</p>
         <ul>
            <li>
               <p>
                  <code>CREATING</code> status indicates that an Backup job has been 
               initiated for a resource. The backup process has started and is actively processing 
               a backup job for the associated recovery point.</p>
            </li>
            <li>
               <p>
                  <code>AVAILABLE</code> status indicates that the backup was successfully created 
               for the recovery point. The backup process has completed without any issues, and the 
               recovery point is now ready for use.</p>
            </li>
            <li>
               <p>
                  <code>PARTIAL</code> status indicates a composite recovery point has one or more 
               nested recovery points that were not in the backup.</p>
            </li>
            <li>
               <p>
                  <code>EXPIRED</code> status indicates that the recovery point has exceeded its retention
               period, but Backup lacks permission or is otherwise unable to delete it. To
               manually delete these recovery points, see <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/gs-cleanup-resources.html#cleanup-backups"> Step 3:
                  Delete the recovery points</a> in the <i>Clean up resources</i>
               section of <i>Getting started</i>.</p>
            </li>
            <li>
               <p>
                  <code>STOPPED</code> status occurs on a continuous backup where a user has taken some
               action that causes the continuous backup to be disabled. This can be caused by the removal
               of permissions, turning off versioning, turning off events being sent to EventBridge,
               or disabling the EventBridge rules that are put in place by Backup. For
               recovery points of Amazon S3, Amazon RDS, and Amazon Aurora
               resources, this status occurs when the retention period of a continuous backup rule is
               changed.</p>
               <p>To resolve <code>STOPPED</code> status, ensure that all requested permissions are in
               place and that versioning is enabled on the S3 bucket. Once these conditions are met, the
               next instance of a backup rule running will result in a new continuous recovery point being
               created. The recovery points with STOPPED status do not need to be deleted.</p>
               <p>For SAP HANA on Amazon EC2
               <code>STOPPED</code> status occurs due to user action, application misconfiguration, or
               backup failure. To ensure that future continuous backups succeed, refer to the recovery
               point status and check SAP HANA for details.</p>
            </li>
         </ul> |
| `index_status` | String | <p>This is the current status for the backup index associated with the specified recovery
         point.</p>
         <p>Statuses are: <code>PENDING</code> | <code>ACTIVE</code> | <code>FAILED</code> |
         <code>DELETING</code>
         </p>
         <p>A recovery point with an index that has the status of <code>ACTIVE</code> can be
         included in a search.</p> |
| `created_by` | String | <p>Contains identifying information about the creation of a recovery point, including the
            <code>BackupPlanArn</code>, <code>BackupPlanId</code>, <code>BackupPlanVersion</code>,
         and <code>BackupRuleId</code> of the backup plan used to create it.</p> |
| `parent_recovery_point_arn` | String | <p>This is an ARN that uniquely identifies a parent (composite) recovery point; for example, 
         <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |
| `completion_date` | String | <p>The date and time that a job to create a recovery point is completed, in Unix format and
         Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to
         milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018
         12:11:30.087 AM.</p> |
| `backup_size_in_bytes` | i64 | <p>The size, in bytes, of a backup.</p> |
| `last_restore_time` | String | <p>The date and time that a recovery point was last restored, in Unix format and
         Coordinated Universal Time (UTC). The value of <code>LastRestoreTime</code> is accurate to
         milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018
         12:11:30.087 AM.</p> |
| `composite_member_identifier` | String | <p>The identifier of a resource within a composite group, such as 
         nested (child) recovery point belonging to a composite (parent) stack. The 
         ID is transferred from the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/resources-section-structure.html#resources-section-structure-syntax">
            logical ID</a> within a stack.</p> |
| `resource_type` | String | <p>The type of Amazon Web Services resource to save as a recovery point; for example, an
            Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database.</p> |
| `status_message` | String | <p>A status message explaining the status of the recovery point.</p> |
| `lifecycle` | String | <p>The lifecycle defines when a protected resource is transitioned to cold storage and when
         it expires. Backup transitions and expires backups automatically according to
         the lifecycle that you define.</p>
         <p>Backups that are transitioned to cold storage must be stored in cold storage for a
         minimum of 90 days. Therefore, the “retention” setting must be 90 days greater than the
         “transition to cold after days” setting. The “transition to cold after days” setting cannot
         be changed after a backup has been transitioned to cold. </p>
         <p>Resource types that can transition to cold storage are listed in the <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/backup-feature-availability.html#features-by-resource">Feature 
         availability by resource</a> table. Backup ignores this expression for other resource types.</p> |
| `index_status_message` | String | <p>A string in the form of a detailed message explaining the status of a backup index
         associated with the recovery point.</p> |
| `creation_date` | String | <p>The date and time that a recovery point is created, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `initiation_date` | String | <p>The date and time when the backup job that created this recovery point was initiated, in
         Unix format and Coordinated Universal Time (UTC).</p> |
| `backup_vault_name` | String | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Region where they are
         created.</p> |
| `storage_class` | String | <p>Specifies the storage class of the recovery point. Valid values are <code>WARM</code> or
            <code>COLD</code>.</p> |
| `backup_vault_arn` | String | <p>An ARN that uniquely identifies a backup vault; for example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>.</p> |
| `encryption_key_arn` | String | <p>The server-side encryption key used to protect your backups; for example,
            <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recovery_point outputs
recovery_point_id = recovery_point.id
recovery_point_calculated_lifecycle = recovery_point.calculated_lifecycle
recovery_point_resource_name = recovery_point.resource_name
recovery_point_recovery_point_arn = recovery_point.recovery_point_arn
recovery_point_source_backup_vault_arn = recovery_point.source_backup_vault_arn
recovery_point_iam_role_arn = recovery_point.iam_role_arn
recovery_point_is_parent = recovery_point.is_parent
recovery_point_vault_type = recovery_point.vault_type
recovery_point_is_encrypted = recovery_point.is_encrypted
recovery_point_resource_arn = recovery_point.resource_arn
recovery_point_status = recovery_point.status
recovery_point_index_status = recovery_point.index_status
recovery_point_created_by = recovery_point.created_by
recovery_point_parent_recovery_point_arn = recovery_point.parent_recovery_point_arn
recovery_point_completion_date = recovery_point.completion_date
recovery_point_backup_size_in_bytes = recovery_point.backup_size_in_bytes
recovery_point_last_restore_time = recovery_point.last_restore_time
recovery_point_composite_member_identifier = recovery_point.composite_member_identifier
recovery_point_resource_type = recovery_point.resource_type
recovery_point_status_message = recovery_point.status_message
recovery_point_lifecycle = recovery_point.lifecycle
recovery_point_index_status_message = recovery_point.index_status_message
recovery_point_creation_date = recovery_point.creation_date
recovery_point_initiation_date = recovery_point.initiation_date
recovery_point_backup_vault_name = recovery_point.backup_vault_name
recovery_point_storage_class = recovery_point.storage_class
recovery_point_backup_vault_arn = recovery_point.backup_vault_arn
recovery_point_encryption_key_arn = recovery_point.encryption_key_arn
```

---


### Backup_vault

BackupVault resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_vault_tags` | HashMap<String, String> |  | <p>The tags to assign to the backup vault.</p> |
| `backup_vault_name` | String | ✅ | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created. They consist of letters, numbers, and hyphens.</p> |
| `encryption_key_arn` | String |  | <p>The server-side encryption key that is used to protect your backups; for example,
            <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> |
| `creator_request_id` | String |  | <p>A unique string that identifies the request and allows failed requests to be retried
         without the risk of running the operation twice. This parameter is optional.</p>
         <p>If used, this parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vault_type` | String | <p>The type of vault described.</p> |
| `locked` | bool | <p>A Boolean that indicates whether Backup Vault Lock is currently protecting
         the backup vault. <code>True</code> means that Vault Lock causes delete or update
         operations on the recovery points stored in the vault to fail.</p> |
| `source_backup_vault_arn` | String | <p>The ARN of the source backup vault from which this restore access backup vault was created.</p> |
| `mpa_session_arn` | String | <p>The ARN of the MPA session associated with this backup vault.</p> |
| `min_retention_days` | i64 | <p>The Backup Vault Lock setting that specifies the minimum retention period
         that the vault retains its recovery points. If this
         parameter is not specified, Vault Lock will not enforce a minimum retention period.</p>
         <p>If specified, any backup or copy job to the vault must have a lifecycle policy with a
         retention period equal to or longer than the minimum retention period. If the job's
         retention period is shorter than that minimum retention period, then the vault fails the
         backup or copy job, and you should either modify your lifecycle settings or use a different
         vault. Recovery points already stored in the vault prior to Vault Lock are not
         affected.</p> |
| `creation_date` | String | <p>The date and time that a backup vault is created, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `number_of_recovery_points` | i64 | <p>The number of recovery points that are stored in a backup vault.</p>
         <p>Recovery point count value displayed in the console can be an approximation. Use <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/API_ListRecoveryPointsByBackupVault.html">
               <code>ListRecoveryPointsByBackupVault</code>
            </a> API to obtain the exact
         count.</p> |
| `vault_state` | String | <p>The current state of the vault.-></p> |
| `backup_vault_arn` | String | <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>.</p> |
| `backup_vault_name` | String | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Region where they are
         created.</p> |
| `encryption_key_arn` | String | <p>The server-side encryption key that is used to protect your backups; for example,
            <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> |
| `creator_request_id` | String | <p>A unique string that identifies the request and allows failed requests to be retried
         without the risk of running the operation twice. This parameter is optional. If used, this
         parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p> |
| `lock_date` | String | <p>The date and time when Backup Vault Lock configuration cannot be changed or
         deleted.</p>
         <p>If you applied Vault Lock to your vault without specifying a lock date, you can change
         any of your Vault Lock settings, or delete Vault Lock from the vault entirely, at any
         time.</p>
         <p>This value is in Unix format, Coordinated Universal Time (UTC), and accurate to
         milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018
         12:11:30.087 AM.</p> |
| `latest_mpa_approval_team_update` | String | <p>Information about the latest update to the MPA approval team association for this backup vault.</p> |
| `max_retention_days` | i64 | <p>The Backup Vault Lock setting that specifies the maximum retention period
         that the vault retains its recovery points. If this parameter is not specified, Vault Lock
         does not enforce a maximum retention period on the recovery points in the vault (allowing
         indefinite storage).</p>
         <p>If specified, any backup or copy job to the vault must have a lifecycle policy with a
         retention period equal to or shorter than the maximum retention period. If the job's
         retention period is longer than that maximum retention period, then the vault fails the
         backup or copy job, and you should either modify your lifecycle settings or use a different
         vault. Recovery points already stored in the vault prior to Vault Lock are not
         affected.</p> |
| `mpa_approval_team_arn` | String | <p>The ARN of the MPA approval team associated with this backup vault.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup_vault
backup_vault = provider.backup.Backup_vault {
    backup_vault_name = "value"  # <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created. They consist of letters, numbers, and hyphens.</p>
}

# Access backup_vault outputs
backup_vault_id = backup_vault.id
backup_vault_vault_type = backup_vault.vault_type
backup_vault_locked = backup_vault.locked
backup_vault_source_backup_vault_arn = backup_vault.source_backup_vault_arn
backup_vault_mpa_session_arn = backup_vault.mpa_session_arn
backup_vault_min_retention_days = backup_vault.min_retention_days
backup_vault_creation_date = backup_vault.creation_date
backup_vault_number_of_recovery_points = backup_vault.number_of_recovery_points
backup_vault_vault_state = backup_vault.vault_state
backup_vault_backup_vault_arn = backup_vault.backup_vault_arn
backup_vault_backup_vault_name = backup_vault.backup_vault_name
backup_vault_encryption_key_arn = backup_vault.encryption_key_arn
backup_vault_creator_request_id = backup_vault.creator_request_id
backup_vault_lock_date = backup_vault.lock_date
backup_vault_latest_mpa_approval_team_update = backup_vault.latest_mpa_approval_team_update
backup_vault_max_retention_days = backup_vault.max_retention_days
backup_vault_mpa_approval_team_arn = backup_vault.mpa_approval_team_arn
```

---


### Restore_job_metadata

RestoreJobMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restore_job_id` | String | <p>This is a unique identifier of a restore job within Backup.</p> |
| `metadata` | HashMap<String, String> | <p>This contains the metadata of the specified backup job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access restore_job_metadata outputs
restore_job_metadata_id = restore_job_metadata.id
restore_job_metadata_restore_job_id = restore_job_metadata.restore_job_id
restore_job_metadata_metadata = restore_job_metadata.metadata
```

---


### Backup_plan

BackupPlan resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creator_request_id` | String |  | <p>Identifies the request and allows failed requests to be retried without the risk of
         running the operation twice. If the request includes a <code>CreatorRequestId</code> that
         matches an existing backup plan, that plan is returned. This parameter is optional.</p>
         <p>If used, this parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p> |
| `backup_plan` | String | ✅ | <p>The body of a backup plan. Includes a <code>BackupPlanName</code> and one or
         more sets of <code>Rules</code>.</p> |
| `backup_plan_tags` | HashMap<String, String> |  | <p>The tags to assign to the backup plan.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The date and time that a backup plan is created, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `version_id` | String | <p>Unique, randomly generated, Unicode, UTF-8 encoded strings that are at most 1,024 bytes
         long. Version IDs cannot be edited.</p> |
| `scheduled_runs_preview` | Vec<String> | <p>List of upcoming scheduled backup runs. Only included when <code>MaxScheduledRunsPreview</code> parameter is greater than 0. Contains up to 10 future backup executions with their scheduled times, execution types, and associated rule IDs.</p> |
| `backup_plan_id` | String | <p>Uniquely identifies a backup plan.</p> |
| `advanced_backup_settings` | Vec<String> | <p>Contains a list of <code>BackupOptions</code> for each resource type. The list is
         populated only if the advanced option is set for the backup plan.</p> |
| `backup_plan_arn` | String | <p>An Amazon Resource Name (ARN) that uniquely identifies a backup plan; for example,
            <code>arn:aws:backup:us-east-1:123456789012:plan:8F81F553-3A74-4A3F-B93D-B3360DC80C50</code>.</p> |
| `deletion_date` | String | <p>The date and time that a backup plan is deleted, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>DeletionDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `last_execution_date` | String | <p>The last time this backup plan was run. A date and time,
         in Unix format and Coordinated Universal Time (UTC). The value of
            <code>LastExecutionDate</code> is accurate to milliseconds. For example, the value
         1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p> |
| `backup_plan` | String | <p>Specifies the body of a backup plan. Includes a <code>BackupPlanName</code> and one or
         more sets of <code>Rules</code>.</p> |
| `creator_request_id` | String | <p>A unique string that identifies the request and allows failed requests to be retried
         without the risk of running the operation twice. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup_plan
backup_plan = provider.backup.Backup_plan {
    backup_plan = "value"  # <p>The body of a backup plan. Includes a <code>BackupPlanName</code> and one or
         more sets of <code>Rules</code>.</p>
}

# Access backup_plan outputs
backup_plan_id = backup_plan.id
backup_plan_creation_date = backup_plan.creation_date
backup_plan_version_id = backup_plan.version_id
backup_plan_scheduled_runs_preview = backup_plan.scheduled_runs_preview
backup_plan_backup_plan_id = backup_plan.backup_plan_id
backup_plan_advanced_backup_settings = backup_plan.advanced_backup_settings
backup_plan_backup_plan_arn = backup_plan.backup_plan_arn
backup_plan_deletion_date = backup_plan.deletion_date
backup_plan_last_execution_date = backup_plan.last_execution_date
backup_plan_backup_plan = backup_plan.backup_plan
backup_plan_creator_request_id = backup_plan.creator_request_id
```

---


### Backup_job

BackupJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | <p>Returns the account ID that owns the backup job.</p> |
| `child_jobs_in_state` | HashMap<String, i64> | <p>This returns the statistics of the included child (nested) backup jobs.</p> |
| `resource_name` | String | <p>The non-unique name of the resource that 
         belongs to the specified backup.</p> |
| `resource_arn` | String | <p>An ARN that uniquely identifies a saved resource. The format of the ARN depends on the
         resource type.</p> |
| `expected_completion_date` | String | <p>The date and time that a job to back up resources is expected to be completed, in Unix
         format and Coordinated Universal Time (UTC). The value of
            <code>ExpectedCompletionDate</code> is accurate to milliseconds. For example, the value
         1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p> |
| `backup_size_in_bytes` | i64 | <p>The size, in bytes, of a backup (recovery point).</p>
         <p>This value can render differently depending on the resource type as Backup pulls in data information from other Amazon Web Services services. For example, the 
         value returned may show a value of <code>0</code>, which may differ from the 
         anticipated value.</p>
         <p>The expected behavior for values by resource type are described as follows:</p>
         <ul>
            <li>
               <p>Amazon Aurora, Amazon DocumentDB, and Amazon Neptune do
               not have this value populate from the operation
               <code>GetBackupJobStatus</code>.</p>
            </li>
            <li>
               <p>For Amazon DynamoDB with advanced features, this value refers to the size
               of the recovery point (backup).</p>
            </li>
            <li>
               <p>Amazon EC2 and Amazon EBS show volume size (provisioned storage)
               returned as part of this value. Amazon EBS does not return backup size
               information; snapshot size will have the same value as the original resource that was
               backed up.</p>
            </li>
            <li>
               <p>For Amazon EFS, this value refers to the delta bytes transferred during a
               backup.</p>
            </li>
            <li>
               <p>Amazon FSx does not populate this value from the operation
               <code>GetBackupJobStatus</code> for FSx file systems.</p>
            </li>
            <li>
               <p>An Amazon RDS instance will show as <code>0</code>.</p>
            </li>
            <li>
               <p>For virtual machines running VMware, this value is passed to Backup
               through an asynchronous workflow, which can mean this displayed value can
               under-represent the actual backup size.</p>
            </li>
         </ul> |
| `recovery_point_arn` | String | <p>An ARN that uniquely identifies a recovery point; for example,
            <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |
| `created_by` | String | <p>Contains identifying information about the creation of a backup job, including the
            <code>BackupPlanArn</code>, <code>BackupPlanId</code>, <code>BackupPlanVersion</code>,
         and <code>BackupRuleId</code> of the backup plan that is used to create it.</p> |
| `backup_vault_arn` | String | <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>.</p> |
| `backup_job_id` | String | <p>Uniquely identifies a request to Backup to back up a resource.</p> |
| `backup_vault_name` | String | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created.</p> |
| `state` | String | <p>The current state of a backup job.</p> |
| `resource_type` | String | <p>The type of Amazon Web Services resource to be backed up; for example, an Amazon Elastic Block Store (Amazon EBS) volume or an Amazon Relational Database Service (Amazon RDS) database.</p> |
| `bytes_transferred` | i64 | <p>The size in bytes transferred to a backup vault at the time that the job status was
         queried.</p> |
| `percent_done` | String | <p>Contains an estimated percentage that is complete of a job at the time the job status
         was queried.</p> |
| `number_of_child_jobs` | i64 | <p>This returns the number of child (nested) backup jobs.</p> |
| `initiation_date` | String | <p>The date a backup job was initiated.</p> |
| `parent_job_id` | String | <p>This returns the parent (composite) resource backup job ID.</p> |
| `iam_role_arn` | String | <p>Specifies the IAM role ARN used to create the target recovery point; for example,
            <code>arn:aws:iam::123456789012:role/S3Access</code>.</p> |
| `message_category` | String | <p>The job count for the specified 
         message category.</p>
         <p>Example strings may include <code>AccessDenied</code>, <code>SUCCESS</code>,
            <code>AGGREGATE_ALL</code>, and <code>INVALIDPARAMETERS</code>. View <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/monitoring.html">Monitoring</a>
         for a list of accepted MessageCategory strings.</p> |
| `is_encrypted` | bool | <p>A boolean value indicating whether the backup is encrypted. All backups in Backup are encrypted, but this field indicates the encryption status for transparency.</p> |
| `completion_date` | String | <p>The date and time that a job to create a backup job is completed, in Unix format and
         Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate to
         milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018
         12:11:30.087 AM.</p> |
| `status_message` | String | <p>A detailed message explaining the status of the job to back up a resource.</p> |
| `start_by` | String | <p>Specifies the time in Unix format and Coordinated Universal Time (UTC) when a backup job
         must be started before it is canceled. The value is calculated by adding the start window
         to the scheduled time. So if the scheduled time were 6:00 PM and the start window is 2
         hours, the <code>StartBy</code> time would be 8:00 PM on the date specified. The value of
            <code>StartBy</code> is accurate to milliseconds. For example, the value 1516925490.087
         represents Friday, January 26, 2018 12:11:30.087 AM.</p> |
| `backup_options` | HashMap<String, String> | <p>Represents the options specified as part of backup plan or on-demand backup job.</p> |
| `vault_lock_state` | String | <p>The lock state of the backup vault. For logically air-gapped vaults, this indicates whether the vault is locked in compliance mode. Valid values include <code>LOCKED</code> and <code>UNLOCKED</code>.</p> |
| `is_parent` | bool | <p>This returns the boolean value that a backup job is a parent (composite) job.</p> |
| `encryption_key_arn` | String | <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt the backup. This can be a customer-managed key or an Amazon Web Services managed key, depending on the vault configuration.</p> |
| `recovery_point_lifecycle` | String |  |
| `vault_type` | String | <p>The type of backup vault where the recovery point is stored. Valid values are <code>BACKUP_VAULT</code> for standard backup vaults and <code>LOGICALLY_AIR_GAPPED_BACKUP_VAULT</code> for logically air-gapped vaults.</p> |
| `creation_date` | String | <p>The date and time that a backup job is created, in Unix format and Coordinated Universal
         Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For
         example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `backup_type` | String | <p>Represents the actual backup type selected for a backup job. For example, if a
         successful Windows Volume Shadow Copy Service (VSS) backup was taken,
            <code>BackupType</code> returns <code>"WindowsVSS"</code>. If <code>BackupType</code> is
         empty, then the backup type was a regular backup.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access backup_job outputs
backup_job_id = backup_job.id
backup_job_account_id = backup_job.account_id
backup_job_child_jobs_in_state = backup_job.child_jobs_in_state
backup_job_resource_name = backup_job.resource_name
backup_job_resource_arn = backup_job.resource_arn
backup_job_expected_completion_date = backup_job.expected_completion_date
backup_job_backup_size_in_bytes = backup_job.backup_size_in_bytes
backup_job_recovery_point_arn = backup_job.recovery_point_arn
backup_job_created_by = backup_job.created_by
backup_job_backup_vault_arn = backup_job.backup_vault_arn
backup_job_backup_job_id = backup_job.backup_job_id
backup_job_backup_vault_name = backup_job.backup_vault_name
backup_job_state = backup_job.state
backup_job_resource_type = backup_job.resource_type
backup_job_bytes_transferred = backup_job.bytes_transferred
backup_job_percent_done = backup_job.percent_done
backup_job_number_of_child_jobs = backup_job.number_of_child_jobs
backup_job_initiation_date = backup_job.initiation_date
backup_job_parent_job_id = backup_job.parent_job_id
backup_job_iam_role_arn = backup_job.iam_role_arn
backup_job_message_category = backup_job.message_category
backup_job_is_encrypted = backup_job.is_encrypted
backup_job_completion_date = backup_job.completion_date
backup_job_status_message = backup_job.status_message
backup_job_start_by = backup_job.start_by
backup_job_backup_options = backup_job.backup_options
backup_job_vault_lock_state = backup_job.vault_lock_state
backup_job_is_parent = backup_job.is_parent
backup_job_encryption_key_arn = backup_job.encryption_key_arn
backup_job_recovery_point_lifecycle = backup_job.recovery_point_lifecycle
backup_job_vault_type = backup_job.vault_type
backup_job_creation_date = backup_job.creation_date
backup_job_backup_type = backup_job.backup_type
```

---


### Protected_resource

ProtectedResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_restore_execution_time_minutes` | i64 | <p>The time, in minutes, that the most recent restore job took to complete.</p> |
| `latest_restore_job_creation_date` | String | <p>The creation date of the most recent restore job.</p> |
| `latest_restore_recovery_point_creation_date` | String | <p>The date the most recent recovery point was created.</p> |
| `resource_type` | String | <p>The type of Amazon Web Services resource saved as a recovery point; for example, an
            Amazon EBS volume or an Amazon RDS database.</p> |
| `resource_name` | String | <p>The name of the resource that belongs to the specified backup.</p> |
| `last_backup_time` | String | <p>The date and time that a resource was last backed up, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>LastBackupTime</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `resource_arn` | String | <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the
         resource type.</p> |
| `last_backup_vault_arn` | String | <p>The ARN (Amazon Resource Name) of the backup vault 
         that contains the most recent backup recovery point.</p> |
| `last_recovery_point_arn` | String | <p>The ARN (Amazon Resource Name) of the most recent 
         recovery point.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access protected_resource outputs
protected_resource_id = protected_resource.id
protected_resource_latest_restore_execution_time_minutes = protected_resource.latest_restore_execution_time_minutes
protected_resource_latest_restore_job_creation_date = protected_resource.latest_restore_job_creation_date
protected_resource_latest_restore_recovery_point_creation_date = protected_resource.latest_restore_recovery_point_creation_date
protected_resource_resource_type = protected_resource.resource_type
protected_resource_resource_name = protected_resource.resource_name
protected_resource_last_backup_time = protected_resource.last_backup_time
protected_resource_resource_arn = protected_resource.resource_arn
protected_resource_last_backup_vault_arn = protected_resource.last_backup_vault_arn
protected_resource_last_recovery_point_arn = protected_resource.last_recovery_point_arn
```

---


### Recovery_point_index_details

RecoveryPointIndexDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_status_message` | String | <p>A detailed message explaining the status of a backup index associated 
         with the recovery point.</p> |
| `recovery_point_arn` | String | <p>An ARN that uniquely identifies a recovery point; for example,
         <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |
| `source_resource_arn` | String | <p>A string of the  Amazon Resource Name (ARN) that uniquely identifies 
         the source resource.</p> |
| `index_creation_date` | String | <p>The date and time that a backup index was created, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `total_items_indexed` | i64 | <p>Count of items within the backup index associated with the 
         recovery point.</p> |
| `index_status` | String | <p>This is the current status for the backup index associated 
         with the specified recovery point.</p>
         <p>Statuses are: <code>PENDING</code> | <code>ACTIVE</code> | <code>FAILED</code> | <code>DELETING</code>
         </p>
         <p>A recovery point with an index that has the status of <code>ACTIVE</code> 
         can be included in a search.</p> |
| `backup_vault_arn` | String | <p>An ARN that uniquely identifies the backup vault where the recovery 
         point index is stored.</p>
         <p>For example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>.</p> |
| `index_deletion_date` | String | <p>The date and time that a backup index was deleted, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `index_completion_date` | String | <p>The date and time that a backup index finished creation, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recovery_point_index_details outputs
recovery_point_index_details_id = recovery_point_index_details.id
recovery_point_index_details_index_status_message = recovery_point_index_details.index_status_message
recovery_point_index_details_recovery_point_arn = recovery_point_index_details.recovery_point_arn
recovery_point_index_details_source_resource_arn = recovery_point_index_details.source_resource_arn
recovery_point_index_details_index_creation_date = recovery_point_index_details.index_creation_date
recovery_point_index_details_total_items_indexed = recovery_point_index_details.total_items_indexed
recovery_point_index_details_index_status = recovery_point_index_details.index_status
recovery_point_index_details_backup_vault_arn = recovery_point_index_details.backup_vault_arn
recovery_point_index_details_index_deletion_date = recovery_point_index_details.index_deletion_date
recovery_point_index_details_index_completion_date = recovery_point_index_details.index_completion_date
```

---


### Report_plan

ReportPlan resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `idempotency_token` | String |  | <p>A customer-chosen string that you can use to distinguish between otherwise identical
         calls to <code>CreateReportPlanInput</code>. Retrying a successful request with the same
         idempotency token results in a success message with no action taken.</p> |
| `report_plan_description` | String |  | <p>An optional description of the report plan with a maximum of 1,024 characters.</p> |
| `report_delivery_channel` | String | ✅ | <p>A structure that contains information about where and how to deliver your reports,
         specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your
         reports.</p> |
| `report_setting` | String | ✅ | <p>Identifies the report template for the report. Reports are built using a report
         template. The report templates are:</p>
         <p>
            <code>RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT |
            COPY_JOB_REPORT | RESTORE_JOB_REPORT</code>
         </p>
         <p>If the report template is <code>RESOURCE_COMPLIANCE_REPORT</code> or
            <code>CONTROL_COMPLIANCE_REPORT</code>, this API resource also describes the report
         coverage by Amazon Web Services Regions and frameworks.</p> |
| `report_plan_tags` | HashMap<String, String> |  | <p>The tags to assign to the report plan.</p> |
| `report_plan_name` | String | ✅ | <p>The unique name of the report plan. The name must be between 1 and 256 characters,
         starting with a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and
         underscores (_).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_plan` | String | <p>Returns details about the report plan that is specified by its name. These details
         include the report plan's Amazon Resource Name (ARN), description, settings, delivery
         channel, deployment status, creation time, and last attempted and successful run
         times.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create report_plan
report_plan = provider.backup.Report_plan {
    report_delivery_channel = "value"  # <p>A structure that contains information about where and how to deliver your reports,
         specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your
         reports.</p>
    report_setting = "value"  # <p>Identifies the report template for the report. Reports are built using a report
         template. The report templates are:</p>
         <p>
            <code>RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT |
            COPY_JOB_REPORT | RESTORE_JOB_REPORT</code>
         </p>
         <p>If the report template is <code>RESOURCE_COMPLIANCE_REPORT</code> or
            <code>CONTROL_COMPLIANCE_REPORT</code>, this API resource also describes the report
         coverage by Amazon Web Services Regions and frameworks.</p>
    report_plan_name = "value"  # <p>The unique name of the report plan. The name must be between 1 and 256 characters,
         starting with a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and
         underscores (_).</p>
}

# Access report_plan outputs
report_plan_id = report_plan.id
report_plan_report_plan = report_plan.report_plan
```

---


### Restore_access_backup_vault

RestoreAccessBackupVault resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_backup_vault_arn` | String | ✅ | <p>The ARN of the source backup vault containing the recovery points to which temporary access is requested.</p> |
| `backup_vault_name` | String |  | <p>The name of the backup vault to associate with an MPA approval team.</p> |
| `backup_vault_tags` | HashMap<String, String> |  | <p>Optional tags to assign to the restore access backup vault.</p> |
| `creator_request_id` | String |  | <p>A unique string that identifies the request and allows failed requests to be retried without the risk of executing the operation twice.</p> |
| `requester_comment` | String |  | <p>A comment explaining the reason for requesting restore access to the backup vault.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create restore_access_backup_vault
restore_access_backup_vault = provider.backup.Restore_access_backup_vault {
    source_backup_vault_arn = "value"  # <p>The ARN of the source backup vault containing the recovery points to which temporary access is requested.</p>
}

```

---


### Supported_resource_types

SupportedResourceTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_types` | Vec<String> | <p>Contains a string with the supported Amazon Web Services resource types:</p>
         <ul>
            <li>
               <p>
                  <code>Aurora</code> for Amazon Aurora</p>
            </li>
            <li>
               <p>
                  <code>CloudFormation</code> for CloudFormation</p>
            </li>
            <li>
               <p>
                  <code>DocumentDB</code> for Amazon DocumentDB (with MongoDB compatibility)</p>
            </li>
            <li>
               <p>
                  <code>DynamoDB</code> for Amazon DynamoDB</p>
            </li>
            <li>
               <p>
                  <code>EBS</code> for Amazon Elastic Block Store</p>
            </li>
            <li>
               <p>
                  <code>EC2</code> for Amazon Elastic Compute Cloud</p>
            </li>
            <li>
               <p>
                  <code>EFS</code> for Amazon Elastic File System</p>
            </li>
            <li>
               <p>
                  <code>FSx</code> for Amazon FSx</p>
            </li>
            <li>
               <p>
                  <code>Neptune</code> for Amazon Neptune</p>
            </li>
            <li>
               <p>
                  <code>RDS</code> for Amazon Relational Database Service</p>
            </li>
            <li>
               <p>
                  <code>Redshift</code> for Amazon Redshift</p>
            </li>
            <li>
               <p>
                  <code>S3</code> for Amazon Simple Storage Service (Amazon S3)</p>
            </li>
            <li>
               <p>
                  <code>SAP HANA on Amazon EC2</code> for SAP HANA databases 
            on Amazon Elastic Compute Cloud instances</p>
            </li>
            <li>
               <p>
                  <code>Storage Gateway</code> for Storage Gateway</p>
            </li>
            <li>
               <p>
                  <code>Timestream</code> for Amazon Timestream</p>
            </li>
            <li>
               <p>
                  <code>VirtualMachine</code> for VMware virtual machines</p>
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

# Access supported_resource_types outputs
supported_resource_types_id = supported_resource_types.id
supported_resource_types_resource_types = supported_resource_types.resource_types
```

---


### Restore_job

RestoreJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expected_completion_time_minutes` | i64 | <p>The amount of time in minutes that a job restoring a recovery point is expected to
         take.</p> |
| `created_by` | String | <p>Contains identifying information about the creation of a restore job.</p> |
| `creation_date` | String | <p>The date and time that a restore job is created, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `backup_size_in_bytes` | i64 | <p>The size, in bytes, of the restored resource.</p> |
| `validation_status` | String | <p>The status of validation run on the indicated 
         restore job.</p> |
| `resource_type` | String | <p>Returns metadata associated with a restore job listed by resource type.</p> |
| `status` | String | <p>Status code specifying the state of the job that is initiated by Backup to
         restore a recovery point.</p> |
| `recovery_point_creation_date` | String | <p>The creation date of the recovery point made by the specifed restore job.</p> |
| `deletion_status_message` | String | <p>This describes the restore job deletion status.</p> |
| `recovery_point_arn` | String | <p>An ARN that uniquely identifies a recovery point; for example,
            <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |
| `backup_vault_arn` | String | <p>The Amazon Resource Name (ARN) of the backup vault containing the recovery point being restored. This helps identify vault access policies and permissions.</p> |
| `created_resource_arn` | String | <p>The Amazon Resource Name (ARN) of the resource that 
      was created by the restore job.</p>
         <p>The format of the ARN depends on the resource type of the backed-up
         resource.</p> |
| `restore_job_id` | String | <p>Uniquely identifies the job that restores a recovery point.</p> |
| `deletion_status` | String | <p>The status of the data generated by the restore test.</p> |
| `source_resource_arn` | String | <p>The Amazon Resource Name (ARN) of the original resource that was backed up. This provides context about what resource is being restored.</p> |
| `iam_role_arn` | String | <p>Specifies the IAM role ARN used to create the target recovery point; for example,
            <code>arn:aws:iam::123456789012:role/S3Access</code>.</p> |
| `validation_status_message` | String | <p>The status message.</p> |
| `completion_date` | String | <p>The date and time that a job to restore a recovery point is completed, in Unix format
         and Coordinated Universal Time (UTC). The value of <code>CompletionDate</code> is accurate
         to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018
         12:11:30.087 AM.</p> |
| `account_id` | String | <p>Returns the account ID that owns the restore job.</p> |
| `status_message` | String | <p>A message showing the status of a job to restore a recovery point.</p> |
| `percent_done` | String | <p>Contains an estimated percentage that is complete of a job at the time the job status
         was queried.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access restore_job outputs
restore_job_id = restore_job.id
restore_job_expected_completion_time_minutes = restore_job.expected_completion_time_minutes
restore_job_created_by = restore_job.created_by
restore_job_creation_date = restore_job.creation_date
restore_job_backup_size_in_bytes = restore_job.backup_size_in_bytes
restore_job_validation_status = restore_job.validation_status
restore_job_resource_type = restore_job.resource_type
restore_job_status = restore_job.status
restore_job_recovery_point_creation_date = restore_job.recovery_point_creation_date
restore_job_deletion_status_message = restore_job.deletion_status_message
restore_job_recovery_point_arn = restore_job.recovery_point_arn
restore_job_backup_vault_arn = restore_job.backup_vault_arn
restore_job_created_resource_arn = restore_job.created_resource_arn
restore_job_restore_job_id = restore_job.restore_job_id
restore_job_deletion_status = restore_job.deletion_status
restore_job_source_resource_arn = restore_job.source_resource_arn
restore_job_iam_role_arn = restore_job.iam_role_arn
restore_job_validation_status_message = restore_job.validation_status_message
restore_job_completion_date = restore_job.completion_date
restore_job_account_id = restore_job.account_id
restore_job_status_message = restore_job.status_message
restore_job_percent_done = restore_job.percent_done
```

---


### Restore_validation_result

RestoreValidationResult resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restore_job_id` | String | ✅ | <p>This is a unique identifier of a restore job within Backup.</p> |
| `validation_status` | String | ✅ | <p>The status of your restore validation.</p> |
| `validation_status_message` | String |  | <p>This is an optional message string you can input to 
         describe the validation status for the restore test validation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create restore_validation_result
restore_validation_result = provider.backup.Restore_validation_result {
    restore_job_id = "value"  # <p>This is a unique identifier of a restore job within Backup.</p>
    validation_status = "value"  # <p>The status of your restore validation.</p>
}

```

---


### Backup_plan_from_json

BackupPlanFromJSON resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_plan` | String | <p>Specifies the body of a backup plan. Includes a <code>BackupPlanName</code> and one or
         more sets of <code>Rules</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access backup_plan_from_json outputs
backup_plan_from_json_id = backup_plan_from_json.id
backup_plan_from_json_backup_plan = backup_plan_from_json.backup_plan
```

---


### Backup_vault_access_policy

BackupVaultAccessPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | <p>The backup vault access policy document in JSON format.</p> |
| `backup_vault_name` | String | ✅ | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The backup vault access policy document in JSON format.</p> |
| `backup_vault_name` | String | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Region where they are
         created.</p> |
| `backup_vault_arn` | String | <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup_vault_access_policy
backup_vault_access_policy = provider.backup.Backup_vault_access_policy {
    backup_vault_name = "value"  # <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created.</p>
}

# Access backup_vault_access_policy outputs
backup_vault_access_policy_id = backup_vault_access_policy.id
backup_vault_access_policy_policy = backup_vault_access_policy.policy
backup_vault_access_policy_backup_vault_name = backup_vault_access_policy.backup_vault_name
backup_vault_access_policy_backup_vault_arn = backup_vault_access_policy.backup_vault_arn
```

---


### Backup_vault_lock_configuration

BackupVaultLockConfiguration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `changeable_for_days` | i64 |  | <p>The Backup Vault Lock configuration that specifies the number of days before
         the lock date. For example, setting <code>ChangeableForDays</code> to 30 on Jan. 1, 2022 at
         8pm UTC will set the lock date to Jan. 31, 2022 at 8pm UTC.</p>
         <p>Backup enforces a 72-hour cooling-off period before Vault Lock takes effect
         and becomes immutable. Therefore, you must set <code>ChangeableForDays</code> to 3 or
         greater.</p>
         <p>Before the lock date, you can delete Vault Lock from the vault using
            <code>DeleteBackupVaultLockConfiguration</code> or change the Vault Lock configuration
         using <code>PutBackupVaultLockConfiguration</code>. On and after the lock date, the Vault
         Lock becomes immutable and cannot be changed or deleted.</p>
         <p>If this parameter is not specified, you can delete Vault Lock from the vault using
            <code>DeleteBackupVaultLockConfiguration</code> or change the Vault Lock configuration
         using <code>PutBackupVaultLockConfiguration</code> at any time.</p> |
| `min_retention_days` | i64 |  | <p>The Backup Vault Lock configuration that specifies the minimum retention
         period that the vault retains its recovery points. This setting can be useful if, for
         example, your organization's policies require you to retain certain data for at least seven
         years (2555 days).</p>
         <p>This parameter is required when a vault lock is created through CloudFormation;
         otherwise, this parameter is optional. If this parameter is not specified, Vault Lock will
         not enforce a minimum retention period.</p>
         <p>If this parameter is specified, any backup or copy job to the vault must have a
         lifecycle policy with a retention period equal to or longer than the minimum retention
         period. If the job's retention period is shorter than that minimum retention period, then
         the vault fails that backup or copy job, and you should either modify your lifecycle
         settings or use a different vault. The shortest minimum retention period
         you can specify is 1 day. Recovery points already saved in the vault prior to
         Vault Lock are not affected.</p> |
| `backup_vault_name` | String | ✅ | <p>The Backup Vault Lock configuration that specifies the name of the backup
         vault it protects.</p> |
| `max_retention_days` | i64 |  | <p>The Backup Vault Lock configuration that specifies the maximum retention
         period that the vault retains its recovery points. This setting can be useful if, for
         example, your organization's policies require you to destroy certain data after retaining
         it for four years (1460 days).</p>
         <p>If this parameter is not included, Vault Lock does not enforce a maximum retention
         period on the recovery points in the vault. If this parameter is included without a value,
         Vault Lock will not enforce a maximum retention period.</p>
         <p>If this parameter is specified, any backup or copy job to the vault must have a
         lifecycle policy with a retention period equal to or shorter than the maximum retention
         period. If the job's retention period is longer than that maximum retention period, then
         the vault fails the backup or copy job, and you should either modify your lifecycle
         settings or use a different vault. The longest maximum retention period
         you can specify is 36500 days (approximately 100 years).
         Recovery points already saved in the vault prior to
         Vault Lock are not affected.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup_vault_lock_configuration
backup_vault_lock_configuration = provider.backup.Backup_vault_lock_configuration {
    backup_vault_name = "value"  # <p>The Backup Vault Lock configuration that specifies the name of the backup
         vault it protects.</p>
}

```

---


### Restore_testing_inferred_metadata

RestoreTestingInferredMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inferred_metadata` | HashMap<String, String> | <p>This is a string map of the metadata inferred from the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access restore_testing_inferred_metadata outputs
restore_testing_inferred_metadata_id = restore_testing_inferred_metadata.id
restore_testing_inferred_metadata_inferred_metadata = restore_testing_inferred_metadata.inferred_metadata
```

---


### Backup_selection

BackupSelection resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creator_request_id` | String |  | <p>A unique string that identifies the request and allows failed requests to be retried
         without the risk of running the operation twice. This parameter is optional.</p>
         <p>If used, this parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p> |
| `backup_plan_id` | String | ✅ | <p>The ID of the backup plan.</p> |
| `backup_selection` | String | ✅ | <p>The body of a request to assign a set of resources to a backup plan.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creator_request_id` | String | <p>A unique string that identifies the request and allows failed requests to be retried
         without the risk of running the operation twice.</p> |
| `selection_id` | String | <p>Uniquely identifies the body of a request to assign a set of resources to a backup
         plan.</p> |
| `backup_plan_id` | String | <p>Uniquely identifies a backup plan.</p> |
| `creation_date` | String | <p>The date and time a backup selection is created, in Unix format and Coordinated
         Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds.
         For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087
         AM.</p> |
| `backup_selection` | String | <p>Specifies the body of a request to assign a set of resources to a backup plan.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup_selection
backup_selection = provider.backup.Backup_selection {
    backup_plan_id = "value"  # <p>The ID of the backup plan.</p>
    backup_selection = "value"  # <p>The body of a request to assign a set of resources to a backup plan.</p>
}

# Access backup_selection outputs
backup_selection_id = backup_selection.id
backup_selection_creator_request_id = backup_selection.creator_request_id
backup_selection_selection_id = backup_selection.selection_id
backup_selection_backup_plan_id = backup_selection.backup_plan_id
backup_selection_creation_date = backup_selection.creation_date
backup_selection_backup_selection = backup_selection.backup_selection
```

---


### Legal_hold

LegalHold resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String | ✅ | <p>The title of the legal hold.</p> |
| `description` | String | ✅ | <p>The description of the legal hold.</p> |
| `idempotency_token` | String |  | <p>This is a user-chosen string used to distinguish between otherwise identical 
         calls. Retrying a successful request with the 
         same idempotency token results in a success message with no action taken.</p> |
| `tags` | HashMap<String, String> |  | <p>Optional tags to include. A tag is a key-value pair you can use to manage, 
         filter, and search for your resources. Allowed characters include UTF-8 letters, 
         numbers, spaces, and the following characters: + - = . _ : /. </p> |
| `recovery_point_selection` | String |  | <p>The criteria to assign a set of resources, such as resource types or backup vaults.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The time when the legal hold was created.</p> |
| `retain_record_until` | String | <p>The date and time until which the legal hold record is retained.</p> |
| `cancel_description` | String | <p>The reason for removing the legal hold.</p> |
| `recovery_point_selection` | String | <p>The criteria to assign a set of resources, such as resource types or backup vaults.</p> |
| `title` | String | <p>The title of the legal hold.</p> |
| `legal_hold_arn` | String | <p>The framework ARN for the specified legal hold. The format 
         of the ARN depends on the resource type.</p> |
| `description` | String | <p>The description of the legal hold.</p> |
| `status` | String | <p>The status of the legal hold.</p> |
| `legal_hold_id` | String | <p>The ID of the legal hold.</p> |
| `cancellation_date` | String | <p>The time when the legal hold was cancelled.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create legal_hold
legal_hold = provider.backup.Legal_hold {
    title = "value"  # <p>The title of the legal hold.</p>
    description = "value"  # <p>The description of the legal hold.</p>
}

# Access legal_hold outputs
legal_hold_id = legal_hold.id
legal_hold_creation_date = legal_hold.creation_date
legal_hold_retain_record_until = legal_hold.retain_record_until
legal_hold_cancel_description = legal_hold.cancel_description
legal_hold_recovery_point_selection = legal_hold.recovery_point_selection
legal_hold_title = legal_hold.title
legal_hold_legal_hold_arn = legal_hold.legal_hold_arn
legal_hold_description = legal_hold.description
legal_hold_status = legal_hold.status
legal_hold_legal_hold_id = legal_hold.legal_hold_id
legal_hold_cancellation_date = legal_hold.cancellation_date
```

---


### Copy_job

CopyJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `copy_job` | String | <p>Contains detailed information about a copy job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access copy_job outputs
copy_job_id = copy_job.id
copy_job_copy_job = copy_job.copy_job
```

---


### Recovery_point_index_settings

RecoveryPointIndexSettings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `recovery_point_arn` | String | ✅ | <p>An ARN that uniquely identifies a recovery point; for example,
         <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |
| `index` | String | ✅ | <p>Index can have 1 of 2 possible values, either <code>ENABLED</code> or 
         <code>DISABLED</code>.</p>
         <p>To create a backup index for an eligible <code>ACTIVE</code> recovery point 
         that does not yet have a backup index, set value to <code>ENABLED</code>.</p>
         <p>To delete a backup index, set value to <code>DISABLED</code>.</p> |
| `backup_vault_name` | String | ✅ | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Region where they are
         created.</p>
         <p>Accepted characters include lowercase letters, numbers, and hyphens.</p> |
| `iam_role_arn` | String |  | <p>This specifies the IAM role ARN used for this operation.</p>
         <p>For example, arn:aws:iam::123456789012:role/S3Access</p> |



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


### Report_job

ReportJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_job` | String | <p>The information about a report job, including its completion and creation times,
         report destination, unique report job ID, Amazon Resource Name (ARN), report template,
         status, and status message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access report_job outputs
report_job_id = report_job.id
report_job_report_job = report_job.report_job
```

---


### Backup_vault_notifications

BackupVaultNotifications resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_vault_events` | Vec<String> | ✅ | <p>An array of events that indicate the status of jobs to back up resources to the backup
         vault. For the list of supported events, common use cases, and code samples, see <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/backup-notifications.html">Notification options
            with Backup</a>.</p> |
| `backup_vault_name` | String | ✅ | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created.</p> |
| `sns_topic_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that specifies the topic for a backup vault’s events; for
         example, <code>arn:aws:sns:us-west-2:111122223333:MyVaultTopic</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_vault_arn` | String | <p>An Amazon Resource Name (ARN) that uniquely identifies a backup vault; for example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>.</p> |
| `backup_vault_name` | String | <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Region where they are
         created.</p> |
| `sns_topic_arn` | String | <p>An ARN that uniquely identifies an Amazon Simple Notification Service (Amazon SNS)
         topic; for example, <code>arn:aws:sns:us-west-2:111122223333:MyTopic</code>.</p> |
| `backup_vault_events` | Vec<String> | <p>An array of events that indicate the status of jobs to back up resources to the backup
         vault.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup_vault_notifications
backup_vault_notifications = provider.backup.Backup_vault_notifications {
    backup_vault_events = "value"  # <p>An array of events that indicate the status of jobs to back up resources to the backup
         vault. For the list of supported events, common use cases, and code samples, see <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/backup-notifications.html">Notification options
            with Backup</a>.</p>
    backup_vault_name = "value"  # <p>The name of a logical container where backups are stored. Backup vaults are identified
         by names that are unique to the account used to create them and the Amazon Web Services
         Region where they are created.</p>
    sns_topic_arn = "value"  # <p>The Amazon Resource Name (ARN) that specifies the topic for a backup vault’s events; for
         example, <code>arn:aws:sns:us-west-2:111122223333:MyVaultTopic</code>.</p>
}

# Access backup_vault_notifications outputs
backup_vault_notifications_id = backup_vault_notifications.id
backup_vault_notifications_backup_vault_arn = backup_vault_notifications.backup_vault_arn
backup_vault_notifications_backup_vault_name = backup_vault_notifications.backup_vault_name
backup_vault_notifications_sns_topic_arn = backup_vault_notifications.sns_topic_arn
backup_vault_notifications_backup_vault_events = backup_vault_notifications.backup_vault_events
```

---


### Logically_air_gapped_backup_vault

LogicallyAirGappedBackupVault resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backup_vault_name` | String | ✅ | <p>The name of a logical container where backups are stored. Logically air-gapped 
         backup vaults are identified by names that are unique to the account used to create 
         them and the Region where they are created.</p> |
| `creator_request_id` | String |  | <p>The ID of the creation request.</p>
         <p>This parameter is optional. If used, this parameter must contain 
         1 to 50 alphanumeric or '-_.' characters.</p> |
| `backup_vault_tags` | HashMap<String, String> |  | <p>The tags to assign to the vault.</p> |
| `min_retention_days` | i64 | ✅ | <p>This setting specifies the minimum retention period
         that the vault retains its recovery points.</p>
         <p>The minimum value accepted is 7 days.</p> |
| `max_retention_days` | i64 | ✅ | <p>The maximum retention period that the vault retains its recovery points.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logically_air_gapped_backup_vault
logically_air_gapped_backup_vault = provider.backup.Logically_air_gapped_backup_vault {
    backup_vault_name = "value"  # <p>The name of a logical container where backups are stored. Logically air-gapped 
         backup vaults are identified by names that are unique to the account used to create 
         them and the Region where they are created.</p>
    min_retention_days = "value"  # <p>This setting specifies the minimum retention period
         that the vault retains its recovery points.</p>
         <p>The minimum value accepted is 7 days.</p>
    max_retention_days = "value"  # <p>The maximum retention period that the vault retains its recovery points.</p>
}

```

---


### Global_settings

GlobalSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `global_settings` | HashMap<String, String> |  | <p>Inputs can include:</p>
         <p>A value for <code>isCrossAccountBackupEnabled</code> and a Region. Example:
            <code>update-global-settings --global-settings isCrossAccountBackupEnabled=false
            --region us-west-2</code>.</p>
         <p>A value for Multi-party approval, styled as "Mpa": <code>isMpaEnabled</code>. Values can
         be true or false. Example:
         <code>update-global-settings --global-settings isMpaEnabled=false
            --region us-west-2</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_settings` | HashMap<String, String> | <p>The status of the flags <code>isCrossAccountBackupEnabled</code> and
          <code>isMpaEnabled</code> ('Mpa' refers to multi-party approval).</p> |
| `last_update_time` | String | <p>The date and time that the flag <code>isCrossAccountBackupEnabled</code> was last
         updated. This update is in Unix format and Coordinated Universal Time (UTC). The value of
            <code>LastUpdateTime</code> is accurate to milliseconds. For example, the value
         1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_settings outputs
global_settings_id = global_settings.id
global_settings_global_settings = global_settings.global_settings
global_settings_last_update_time = global_settings.last_update_time
```

---


### Recovery_point_restore_metadata

RecoveryPointRestoreMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_type` | String | <p>The resource type of the recovery point.</p> |
| `backup_vault_arn` | String | <p>An ARN that uniquely identifies a backup vault; for example,
         <code>arn:aws:backup:us-east-1:123456789012:backup-vault:aBackupVault</code>.</p> |
| `restore_metadata` | HashMap<String, String> | <p>The set of metadata key-value pairs that describe the original configuration of the
         backed-up resource. These values vary depending on the service that is being
         restored.</p> |
| `recovery_point_arn` | String | <p>An ARN that uniquely identifies a recovery point; for example,
            <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recovery_point_restore_metadata outputs
recovery_point_restore_metadata_id = recovery_point_restore_metadata.id
recovery_point_restore_metadata_resource_type = recovery_point_restore_metadata.resource_type
recovery_point_restore_metadata_backup_vault_arn = recovery_point_restore_metadata.backup_vault_arn
recovery_point_restore_metadata_restore_metadata = recovery_point_restore_metadata.restore_metadata
recovery_point_restore_metadata_recovery_point_arn = recovery_point_restore_metadata.recovery_point_arn
```

---


### Backup_plan_from_template

BackupPlanFromTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_plan_document` | String | <p>Returns the body of a backup plan based on the target template, including the name,
         rules, and backup vault of the plan.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access backup_plan_from_template outputs
backup_plan_from_template_id = backup_plan_from_template.id
backup_plan_from_template_backup_plan_document = backup_plan_from_template.backup_plan_document
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple framework resources
framework_0 = provider.backup.Framework {
    framework_controls = "value-0"
    framework_name = "value-0"
}
framework_1 = provider.backup.Framework {
    framework_controls = "value-1"
    framework_name = "value-1"
}
framework_2 = provider.backup.Framework {
    framework_controls = "value-2"
    framework_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    framework = provider.backup.Framework {
        framework_controls = "production-value"
        framework_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Backup Documentation](https://docs.aws.amazon.com/backup/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
