# Glacier Service



**Resources**: 8

---

## Overview

The glacier service provides access to 8 resource types:

- [Data_retrieval_policy](#data_retrieval_policy) [R]
- [Job](#job) [R]
- [Job_output](#job_output) [R]
- [Archive](#archive) [D]
- [Vault_lock](#vault_lock) [R]
- [Vault](#vault) [CRD]
- [Vault_notifications](#vault_notifications) [RD]
- [Vault_access_policy](#vault_access_policy) [RD]

---

## Resources


### Data_retrieval_policy

DataRetrievalPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>Contains the returned data retrieval policy in JSON format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_retrieval_policy outputs
data_retrieval_policy_id = data_retrieval_policy.id
data_retrieval_policy_policy = data_retrieval_policy.policy
```

---


### Job

Job resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `select_parameters` | String | <p>Contains the parameters used for a select.</p> |
| `inventory_size_in_bytes` | i64 | <p>For an inventory retrieval job, this value is the size in bytes of the inventory
            requested for download. For an archive retrieval or select job, this value is
            null.</p> |
| `output_location` | String | <p>Contains the location where the data from the select job is stored.</p> |
| `inventory_retrieval_parameters` | String | <p>Parameters used for range inventory retrieval.</p> |
| `completion_date` | String | <p>The UTC time that the job request completed. While the job is in progress, the
            value is null.</p> |
| `sha256_tree_hash` | String | <p>For an archive retrieval job, this value is the checksum of the archive. Otherwise,
            this value is null.</p>
        <p>The SHA256 tree hash value for the requested range of an archive. If the <b>InitiateJob</b> request for an archive specified a tree-hash
            aligned range, then this field returns a value.</p>
        <p>If the whole archive is retrieved, this value is the same as the
            ArchiveSHA256TreeHash value.</p>
        <p>This field is null for the following:</p>
        <ul>
            <li>
                <p>Archive retrieval jobs that specify a range that is not tree-hash
                    aligned</p>
            </li>
         </ul>
        <ul>
            <li>
                <p>Archival jobs that specify a range that is equal to the whole archive, when
                    the job status is <code>InProgress</code>
               </p>
            </li>
         </ul>
        <ul>
            <li>
                <p>Inventory jobs</p>
            </li>
            <li>
                <p>Select jobs</p>
            </li>
         </ul> |
| `archive_id` | String | <p>The archive ID requested for a select job or archive retrieval. Otherwise, this
            field is null.</p> |
| `creation_date` | String | <p>The UTC date when the job was created. This value is a string representation of ISO
            8601 date format, for example <code>"2012-03-20T17:03:43.221Z"</code>.</p> |
| `status_code` | String | <p>The status code can be <code>InProgress</code>, <code>Succeeded</code>, or
                <code>Failed</code>, and indicates the status of the job.</p> |
| `action` | String | <p>The job type. This value is either <code>ArchiveRetrieval</code>,
                <code>InventoryRetrieval</code>, or
            <code>Select</code>. </p> |
| `job_description` | String | <p>The job description provided when initiating the job.</p> |
| `vault_arn` | String | <p>The Amazon Resource Name (ARN) of the vault from which an archive retrieval was
            requested.</p> |
| `job_id` | String | <p>An opaque string that identifies an Amazon S3 Glacier job.</p> |
| `completed` | bool | <p>The job status. When a job is completed, you get the job's output using Get Job
            Output (GET output).</p> |
| `sns_topic` | String | <p>An Amazon SNS topic that receives notification.</p> |
| `archive_sha256_tree_hash` | String | <p>The SHA256 tree hash of the entire archive for an archive retrieval. For inventory
            retrieval or select jobs, this field is null.</p> |
| `retrieval_byte_range` | String | <p>The retrieved byte range for archive retrieval jobs in the form
                <i>StartByteValue</i>-<i>EndByteValue</i>. If no range
            was specified in the archive retrieval, then the whole archive is retrieved. In this
            case, <i>StartByteValue</i> equals 0 and <i>EndByteValue</i>
            equals the size of the archive minus 1. For inventory retrieval or select jobs, this
            field is null. </p> |
| `job_output_path` | String | <p>Contains the job output location.</p> |
| `status_message` | String | <p>A friendly message that describes the job status.</p> |
| `archive_size_in_bytes` | i64 | <p>For an archive retrieval job, this value is the size in bytes of the archive being
            requested for download. For an inventory retrieval or select job, this value is
            null.</p> |
| `tier` | String | <p>The tier to use for a select or an archive retrieval. Valid values are
                <code>Expedited</code>, <code>Standard</code>, or <code>Bulk</code>.
                <code>Standard</code> is the default.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job outputs
job_id = job.id
job_select_parameters = job.select_parameters
job_inventory_size_in_bytes = job.inventory_size_in_bytes
job_output_location = job.output_location
job_inventory_retrieval_parameters = job.inventory_retrieval_parameters
job_completion_date = job.completion_date
job_sha256_tree_hash = job.sha256_tree_hash
job_archive_id = job.archive_id
job_creation_date = job.creation_date
job_status_code = job.status_code
job_action = job.action
job_job_description = job.job_description
job_vault_arn = job.vault_arn
job_job_id = job.job_id
job_completed = job.completed
job_sns_topic = job.sns_topic
job_archive_sha256_tree_hash = job.archive_sha256_tree_hash
job_retrieval_byte_range = job.retrieval_byte_range
job_job_output_path = job.job_output_path
job_status_message = job.status_message
job_archive_size_in_bytes = job.archive_size_in_bytes
job_tier = job.tier
```

---


### Job_output

JobOutput resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accept_ranges` | String | <p>Indicates the range units accepted. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html">RFC2616</a>. </p> |
| `archive_description` | String | <p>The description of an archive.</p> |
| `content_type` | String | <p>The Content-Type depends on whether the job output is an archive or a vault
         inventory. For archive data, the Content-Type is application/octet-stream. For vault
         inventory, if you requested CSV format when you initiated the job, the Content-Type is
         text/csv. Otherwise, by default, vault inventory is returned as JSON, and the Content-Type
         is application/json.</p> |
| `content_range` | String | <p>The range of bytes returned by Amazon S3 Glacier. If only partial output is downloaded,
         the response provides the range of bytes Amazon S3 Glacier returned. For example, bytes
         0-1048575/8388608 returns the first 1 MB from 8 MB.</p> |
| `body` | String | <p>The job data, either archive data or inventory data.</p> |
| `checksum` | String | <p>The checksum of the data in the response. This header is returned only when
         retrieving the output for an archive retrieval job. Furthermore, this header appears only
         under the following conditions:</p>
         <ul>
            <li>
                <p>You get the entire range of the archive.</p>
            </li>
            <li>
               <p>You request a range to return of the archive that starts and ends on a multiple
               of 1 MB. For example, if you have an 3.1 MB archive and you specify a range to return
               that starts at 1 MB and ends at 2 MB, then the x-amz-sha256-tree-hash is returned as
               a response header.</p>
            </li>
            <li>
               <p>You request a range of the archive to return that starts on a multiple of 1 MB
               and goes to the end of the archive. For example, if you have a 3.1 MB archive and you
               specify a range that starts at 2 MB and ends at 3.1 MB (the end of the archive), then
               the x-amz-sha256-tree-hash is returned as a response header.</p>
            </li>
         </ul> |
| `status` | i64 | <p>The HTTP response code for a job output request. The value depends on whether a range
         was specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_output outputs
job_output_id = job_output.id
job_output_accept_ranges = job_output.accept_ranges
job_output_archive_description = job_output.archive_description
job_output_content_type = job_output.content_type
job_output_content_range = job_output.content_range
job_output_body = job_output.body
job_output_checksum = job_output.checksum
job_output_status = job_output.status
```

---


### Archive

Archive resource

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


### Vault_lock

VaultLock resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The UTC date and time at which the vault lock was put into the
            <code>InProgress</code> state.</p> |
| `state` | String | <p>The state of the vault lock. <code>InProgress</code> or
         <code>Locked</code>.</p> |
| `policy` | String | <p>The vault lock policy as a JSON string, which uses "\" as an escape
         character.</p> |
| `expiration_date` | String | <p>The UTC date and time at which the lock ID expires. This value can be
            <code>null</code> if the vault lock is in a <code>Locked</code> state.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vault_lock outputs
vault_lock_id = vault_lock.id
vault_lock_creation_date = vault_lock.creation_date
vault_lock_state = vault_lock.state
vault_lock_policy = vault_lock.policy
vault_lock_expiration_date = vault_lock.expiration_date
```

---


### Vault

Vault resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vault_name` | String | ✅ | <p>The name of the vault.</p> |
| `account_id` | String | ✅ | <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS
         account ID associated with the credentials used to sign the request. You can either specify
         an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3
         Glacier uses the AWS account ID associated with the credentials used to sign the request.
         If you specify your account ID, do not include any hyphens ('-') in the ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vault_arn` | String | <p>The Amazon Resource Name (ARN) of the vault.</p> |
| `creation_date` | String | <p>The Universal Coordinated Time (UTC) date when the vault was created. This value
         should be a string in the ISO 8601 date format, for example
            <code>2012-03-20T17:03:43.221Z</code>.</p> |
| `number_of_archives` | i64 | <p>The number of archives in the vault as of the last inventory date. This field will
         return <code>null</code> if an inventory has not yet run on the vault, for example if you
         just created the vault.</p> |
| `last_inventory_date` | String | <p>The Universal Coordinated Time (UTC) date when Amazon S3 Glacier completed the last
         vault inventory.  This value should be a string in the ISO 8601 date format, for example
            <code>2012-03-20T17:03:43.221Z</code>.</p> |
| `vault_name` | String | <p>The name of the vault.</p> |
| `size_in_bytes` | i64 | <p>Total size, in bytes, of the archives in the vault as of the last inventory date.
         This field will return null if an inventory has not yet run on the vault, for example if
         you just created the vault.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vault
vault = provider.glacier.Vault {
    vault_name = "value"  # <p>The name of the vault.</p>
    account_id = "value"  # <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS
         account ID associated with the credentials used to sign the request. You can either specify
         an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3
         Glacier uses the AWS account ID associated with the credentials used to sign the request.
         If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
}

# Access vault outputs
vault_id = vault.id
vault_vault_arn = vault.vault_arn
vault_creation_date = vault.creation_date
vault_number_of_archives = vault.number_of_archives
vault_last_inventory_date = vault.last_inventory_date
vault_vault_name = vault.vault_name
vault_size_in_bytes = vault.size_in_bytes
```

---


### Vault_notifications

VaultNotifications resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vault_notification_config` | String | <p>Returns the notification configuration set on the vault.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vault_notifications outputs
vault_notifications_id = vault_notifications.id
vault_notifications_vault_notification_config = vault_notifications.vault_notification_config
```

---


### Vault_access_policy

VaultAccessPolicy resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>Contains the returned vault access policy as a JSON string.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vault_access_policy outputs
vault_access_policy_id = vault_access_policy.id
vault_access_policy_policy = vault_access_policy.policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple data_retrieval_policy resources
data_retrieval_policy_0 = provider.glacier.Data_retrieval_policy {
}
data_retrieval_policy_1 = provider.glacier.Data_retrieval_policy {
}
data_retrieval_policy_2 = provider.glacier.Data_retrieval_policy {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    data_retrieval_policy = provider.glacier.Data_retrieval_policy {
    }
```

---

## Related Documentation

- [AWS Glacier Documentation](https://docs.aws.amazon.com/glacier/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
