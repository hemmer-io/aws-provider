# Iotthingsgraph Service



**Resources**: 9

---

## Overview

The iotthingsgraph service provides access to 9 resource types:

- [Flow_template](#flow_template) [CRUD]
- [Namespace_deletion_status](#namespace_deletion_status) [R]
- [Flow_template_revisions](#flow_template_revisions) [R]
- [Entities](#entities) [R]
- [System_template_revisions](#system_template_revisions) [R]
- [System_instance](#system_instance) [CRD]
- [Namespace](#namespace) [RD]
- [Upload_status](#upload_status) [R]
- [System_template](#system_template) [CRUD]

---

## Resources


### Flow_template

FlowTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `definition` | String | ✅ | <p>The workflow <code>DefinitionDocument</code>.</p> |
| `compatible_namespace_version` | i64 |  | <p>The namespace version in which the workflow is to be created.</p>
         <p>If no value is specified, the latest version is used by default.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The object that describes the specified workflow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create flow_template
flow_template = provider.iotthingsgraph.Flow_template {
    definition = "value"  # <p>The workflow <code>DefinitionDocument</code>.</p>
}

# Access flow_template outputs
flow_template_id = flow_template.id
flow_template_description = flow_template.description
```

---


### Namespace_deletion_status

NamespaceDeletionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `namespace_arn` | String | <p>The ARN of the namespace that is being deleted.</p> |
| `error_message` | String | <p>An error code returned by the namespace deletion task.</p> |
| `status` | String | <p>The status of the deletion request.</p> |
| `error_code` | String | <p>An error code returned by the namespace deletion task.</p> |
| `namespace_name` | String | <p>The name of the namespace that is being deleted.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access namespace_deletion_status outputs
namespace_deletion_status_id = namespace_deletion_status.id
namespace_deletion_status_namespace_arn = namespace_deletion_status.namespace_arn
namespace_deletion_status_error_message = namespace_deletion_status.error_message
namespace_deletion_status_status = namespace_deletion_status.status
namespace_deletion_status_error_code = namespace_deletion_status.error_code
namespace_deletion_status_namespace_name = namespace_deletion_status.namespace_name
```

---


### Flow_template_revisions

FlowTemplateRevisions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The string to specify as <code>nextToken</code> when you request the next page of results.</p> |
| `summaries` | Vec<String> | <p>An array of objects that provide summary data about each revision.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flow_template_revisions outputs
flow_template_revisions_id = flow_template_revisions.id
flow_template_revisions_next_token = flow_template_revisions.next_token
flow_template_revisions_summaries = flow_template_revisions.summaries
```

---


### Entities

Entities resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `descriptions` | Vec<String> | <p>An array of descriptions for the specified entities.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entities outputs
entities_id = entities.id
entities_descriptions = entities.descriptions
```

---


### System_template_revisions

SystemTemplateRevisions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summaries` | Vec<String> | <p>An array of objects that contain summary data about the system template revisions.</p> |
| `next_token` | String | <p>The string to specify as <code>nextToken</code> when you request the next page of results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access system_template_revisions outputs
system_template_revisions_id = system_template_revisions.id
system_template_revisions_summaries = system_template_revisions.summaries
system_template_revisions_next_token = system_template_revisions.next_token
```

---


### System_instance

SystemInstance resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target` | String | ✅ | <p>The target type of the deployment. Valid values are <code>GREENGRASS</code> and <code>CLOUD</code>.</p> |
| `definition` | String | ✅ |  |
| `greengrass_group_name` | String |  | <p>The name of the Greengrass group where the system instance will be deployed. This value is required if 
      the value of the <code>target</code> parameter is <code>GREENGRASS</code>.</p> |
| `s3_bucket_name` | String |  | <p>The name of the Amazon Simple Storage Service bucket that will be used to store and deploy the system instance's resource file. This value is required if 
         the value of the <code>target</code> parameter is <code>GREENGRASS</code>.</p> |
| `flow_actions_role_arn` | String |  | <p>The ARN of the IAM role that AWS IoT Things Graph will assume when it executes the flow. This role must have 
      read and write access to AWS Lambda and AWS IoT and any other AWS services that the flow uses when it executes.  This 
      value is required if the value of the <code>target</code> parameter is <code>CLOUD</code>.</p> |
| `metrics_configuration` | String |  |  |
| `tags` | Vec<String> |  | <p>Metadata, consisting of key-value pairs, that can be used to categorize your system instances.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>An object that describes the system instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create system_instance
system_instance = provider.iotthingsgraph.System_instance {
    target = "value"  # <p>The target type of the deployment. Valid values are <code>GREENGRASS</code> and <code>CLOUD</code>.</p>
    definition = "value"  # Required field
}

# Access system_instance outputs
system_instance_id = system_instance.id
system_instance_description = system_instance.description
```

---


### Namespace

Namespace resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tracking_namespace_version` | i64 | <p>The version of the public namespace that the latest version is tracking.</p> |
| `tracking_namespace_name` | String | <p>The name of the public namespace that the latest namespace version is tracking.</p> |
| `namespace_version` | i64 | <p>The version of the user's namespace to describe.</p> |
| `namespace_name` | String | <p>The name of the namespace.</p> |
| `namespace_arn` | String | <p>The ARN of the namespace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access namespace outputs
namespace_id = namespace.id
namespace_tracking_namespace_version = namespace.tracking_namespace_version
namespace_tracking_namespace_name = namespace.tracking_namespace_name
namespace_namespace_version = namespace.namespace_version
namespace_namespace_name = namespace.namespace_name
namespace_namespace_arn = namespace.namespace_arn
```

---


### Upload_status

UploadStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upload_status` | String | <p>The status of the upload. The initial status is <code>IN_PROGRESS</code>. The response show all validation failures if the upload fails.</p> |
| `upload_id` | String | <p>The ID of the upload.</p> |
| `namespace_name` | String | <p>The name of the upload's namespace.</p> |
| `created_date` | String | <p>The date at which the upload was created.</p> |
| `namespace_arn` | String | <p>The ARN of the upload.</p> |
| `failure_reason` | String | <p>The reason for an upload failure.</p> |
| `namespace_version` | i64 | <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access upload_status outputs
upload_status_id = upload_status.id
upload_status_upload_status = upload_status.upload_status
upload_status_upload_id = upload_status.upload_id
upload_status_namespace_name = upload_status.namespace_name
upload_status_created_date = upload_status.created_date
upload_status_namespace_arn = upload_status.namespace_arn
upload_status_failure_reason = upload_status.failure_reason
upload_status_namespace_version = upload_status.namespace_version
```

---


### System_template

SystemTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `definition` | String | ✅ | <p>The <code>DefinitionDocument</code> used to create the system.</p> |
| `compatible_namespace_version` | i64 |  | <p>The namespace version in which the system is to be created.</p>
         <p>If no value is specified, the latest version is used by default.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>An object that contains summary data about the system.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create system_template
system_template = provider.iotthingsgraph.System_template {
    definition = "value"  # <p>The <code>DefinitionDocument</code> used to create the system.</p>
}

# Access system_template outputs
system_template_id = system_template.id
system_template_description = system_template.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple flow_template resources
flow_template_0 = provider.iotthingsgraph.Flow_template {
    definition = "value-0"
}
flow_template_1 = provider.iotthingsgraph.Flow_template {
    definition = "value-1"
}
flow_template_2 = provider.iotthingsgraph.Flow_template {
    definition = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    flow_template = provider.iotthingsgraph.Flow_template {
        definition = "production-value"
    }
```

---

## Related Documentation

- [AWS Iotthingsgraph Documentation](https://docs.aws.amazon.com/iotthingsgraph/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
