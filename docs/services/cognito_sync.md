# Cognito_sync Service



**Resources**: 7

---

## Overview

The cognito_sync service provides access to 7 resource types:

- [Cognito_events](#cognito_events) [R]
- [Identity_pool_usage](#identity_pool_usage) [R]
- [Records](#records) [U]
- [Identity_pool_configuration](#identity_pool_configuration) [R]
- [Bulk_publish_details](#bulk_publish_details) [R]
- [Dataset](#dataset) [RD]
- [Identity_usage](#identity_usage) [R]

---

## Resources


### Cognito_events

CognitoEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events` | HashMap<String, String> | <p>The Cognito Events returned from the GetCognitoEvents request</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cognito_events outputs
cognito_events_id = cognito_events.id
cognito_events_events = cognito_events.events
```

---


### Identity_pool_usage

IdentityPoolUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_pool_usage` | String | Information about the
      usage of the identity pool. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_pool_usage outputs
identity_pool_usage_id = identity_pool_usage.id
identity_pool_usage_identity_pool_usage = identity_pool_usage.identity_pool_usage
```

---


### Records

Records resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `identity_id` | String | ✅ | A name-spaced GUID (for example,
      us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is
      unique within a region. |
| `device_id` | String |  | <p>The unique ID generated for this device by Cognito.</p> |
| `dataset_name` | String | ✅ | A string of up to 128 characters.
      Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (dash), and '.'
      (dot). |
| `sync_session_token` | String | ✅ | The SyncSessionToken returned by a
      previous call to ListRecords for this dataset and identity. |
| `identity_pool_id` | String | ✅ | A name-spaced GUID (for example,
      us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is
      unique within a region. |
| `client_context` | String |  | Intended to supply a device ID that
      will populate the lastModifiedBy field referenced in other methods. The
         ClientContext field is not yet implemented. |
| `record_patches` | Vec<String> |  | A list of patch
      operations. |



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


### Identity_pool_configuration

IdentityPoolConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `push_sync` | String | <p>Options to apply to this identity pool for push synchronization.</p> |
| `cognito_streams` | String | Options to apply to this identity pool for Amazon Cognito streams. |
| `identity_pool_id` | String | <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by
         Amazon Cognito.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_pool_configuration outputs
identity_pool_configuration_id = identity_pool_configuration.id
identity_pool_configuration_push_sync = identity_pool_configuration.push_sync
identity_pool_configuration_cognito_streams = identity_pool_configuration.cognito_streams
identity_pool_configuration_identity_pool_id = identity_pool_configuration.identity_pool_id
```

---


### Bulk_publish_details

BulkPublishDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bulk_publish_status` | String | Status of the last bulk publish operation, valid values are: 
      <p>NOT_STARTED - No bulk publish has been requested for this identity pool</p>
      <p>IN_PROGRESS - Data is being published to the configured stream</p>
      <p>SUCCEEDED - All data for the identity pool has been published to the configured stream</p>
      <p>FAILED - Some portion of the data has failed to publish, check FailureMessage for the cause.</p> |
| `identity_pool_id` | String | A name-spaced GUID (for example,
      us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is
      unique within a region. |
| `failure_message` | String | If BulkPublishStatus is FAILED this field will contain the error message that caused the bulk publish to fail. |
| `bulk_publish_start_time` | String | The date/time at which the last bulk publish was initiated. |
| `bulk_publish_complete_time` | String | If BulkPublishStatus is SUCCEEDED, the time the last bulk publish operation completed. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bulk_publish_details outputs
bulk_publish_details_id = bulk_publish_details.id
bulk_publish_details_bulk_publish_status = bulk_publish_details.bulk_publish_status
bulk_publish_details_identity_pool_id = bulk_publish_details.identity_pool_id
bulk_publish_details_failure_message = bulk_publish_details.failure_message
bulk_publish_details_bulk_publish_start_time = bulk_publish_details.bulk_publish_start_time
bulk_publish_details_bulk_publish_complete_time = bulk_publish_details.bulk_publish_complete_time
```

---


### Dataset

Dataset resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset` | String | Meta data for a collection of data for an
      identity. An identity can have multiple datasets. A dataset can be general or associated with
      a particular entity in an application (like a saved game). Datasets are automatically created
      if they don't exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value
      pairs. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset = dataset.dataset
```

---


### Identity_usage

IdentityUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_usage` | String | Usage information for the
      identity. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_usage outputs
identity_usage_id = identity_usage.id
identity_usage_identity_usage = identity_usage.identity_usage
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple cognito_events resources
cognito_events_0 = provider.cognito_sync.Cognito_events {
}
cognito_events_1 = provider.cognito_sync.Cognito_events {
}
cognito_events_2 = provider.cognito_sync.Cognito_events {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cognito_events = provider.cognito_sync.Cognito_events {
    }
```

---

## Related Documentation

- [AWS Cognito_sync Documentation](https://docs.aws.amazon.com/cognito_sync/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
