# Codestar_connections Service



**Resources**: 8

---

## Overview

The codestar_connections service provides access to 8 resource types:

- [Repository_sync_status](#repository_sync_status) [R]
- [Repository_link](#repository_link) [CRUD]
- [Sync_blocker](#sync_blocker) [U]
- [Sync_blocker_summary](#sync_blocker_summary) [R]
- [Host](#host) [CRUD]
- [Connection](#connection) [CRD]
- [Sync_configuration](#sync_configuration) [CRUD]
- [Resource_sync_status](#resource_sync_status) [R]

---

## Resources


### Repository_sync_status

RepositorySyncStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_sync` | String | <p>The status of the latest sync returned for a specified repository and branch.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access repository_sync_status outputs
repository_sync_status_id = repository_sync_status.id
repository_sync_status_latest_sync = repository_sync_status.latest_sync
```

---


### Repository_link

RepositoryLink resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository to be associated with the repository link.</p> |
| `owner_id` | String | ✅ | <p>The owner ID for the repository associated with a specific sync configuration, such as
      the owner ID in GitHub.</p> |
| `tags` | Vec<String> |  | <p>The tags for the repository to be associated with the repository link.</p> |
| `connection_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the connection to be associated with the repository link.</p> |
| `encryption_key_arn` | String |  | <p>The Amazon Resource Name (ARN) encryption key for the repository to be associated with the repository link.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repository_link_info` | String | <p>The information returned for a specified repository link.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository_link
repository_link = provider.codestar_connections.Repository_link {
    repository_name = "value"  # <p>The name of the repository to be associated with the repository link.</p>
    owner_id = "value"  # <p>The owner ID for the repository associated with a specific sync configuration, such as
      the owner ID in GitHub.</p>
    connection_arn = "value"  # <p>The Amazon Resource Name (ARN) of the connection to be associated with the repository link.</p>
}

# Access repository_link outputs
repository_link_id = repository_link.id
repository_link_repository_link_info = repository_link.repository_link_info
```

---


### Sync_blocker

SyncBlocker resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_name` | String | ✅ | <p>The name of the resource for the sync blocker to be updated.</p> |
| `resolved_reason` | String | ✅ | <p>The reason for resolving the sync blocker.</p> |
| `id` | String | ✅ | <p>The ID of the sync blocker to be updated.</p> |
| `sync_type` | String | ✅ | <p>The sync type of the sync blocker to be updated.</p> |



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


### Sync_blocker_summary

SyncBlockerSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sync_blocker_summary` | String | <p>The list of sync blockers for a specified resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sync_blocker_summary outputs
sync_blocker_summary_id = sync_blocker_summary.id
sync_blocker_summary_sync_blocker_summary = sync_blocker_summary.sync_blocker_summary
```

---


### Host

Host resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provider_type` | String | ✅ | <p>The name of the installed provider to be associated with your connection. The host
      resource represents the infrastructure where your provider type is installed. The valid
      provider type is GitHub Enterprise Server.</p> |
| `tags` | Vec<String> |  | <p>Tags for the host to be created.</p> |
| `name` | String | ✅ | <p>The name of the host to be created.</p> |
| `provider_endpoint` | String | ✅ | <p>The endpoint of the infrastructure to be represented by the host after it is
      created.</p> |
| `vpc_configuration` | String |  | <p>The VPC configuration to be provisioned for the host. A VPC must be configured and the
      infrastructure to be represented by the host must already be connected to the VPC.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the requested host.</p> |
| `vpc_configuration` | String | <p>The VPC configuration of the requested host.</p> |
| `status` | String | <p>The status of the requested host.</p> |
| `provider_type` | String | <p>The provider type of the requested host, such as GitHub Enterprise Server.</p> |
| `provider_endpoint` | String | <p>The endpoint of the infrastructure represented by the requested host.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create host
host = provider.codestar_connections.Host {
    provider_type = "value"  # <p>The name of the installed provider to be associated with your connection. The host
      resource represents the infrastructure where your provider type is installed. The valid
      provider type is GitHub Enterprise Server.</p>
    name = "value"  # <p>The name of the host to be created.</p>
    provider_endpoint = "value"  # <p>The endpoint of the infrastructure to be represented by the host after it is
      created.</p>
}

# Access host outputs
host_id = host.id
host_name = host.name
host_vpc_configuration = host.vpc_configuration
host_status = host.status
host_provider_type = host.provider_type
host_provider_endpoint = host.provider_endpoint
```

---


### Connection

Connection resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_name` | String | ✅ | <p>The name of the connection to be created.</p> |
| `provider_type` | String |  | <p>The name of the external provider where your third-party code repository is
      configured.</p> |
| `tags` | Vec<String> |  | <p>The key-value pair to use when tagging the resource.</p> |
| `host_arn` | String |  | <p>The Amazon Resource Name (ARN) of the host associated with the connection to be created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection` | String | <p>The connection details, such as status, owner, and provider type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.codestar_connections.Connection {
    connection_name = "value"  # <p>The name of the connection to be created.</p>
}

# Access connection outputs
connection_id = connection.id
connection_connection = connection.connection
```

---


### Sync_configuration

SyncConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `publish_deployment_status` | String |  | <p>Whether to enable or disable publishing of deployment status to source providers.</p> |
| `repository_link_id` | String | ✅ | <p>The ID of the repository link created for the connection. A repository link allows Git
      sync to monitor and sync changes to files in a specified Git repository.</p> |
| `config_file` | String | ✅ | <p>The file name of the configuration file that manages syncing between the connection and the repository. This configuration file is stored in the repository.</p> |
| `branch` | String | ✅ | <p>The branch in the repository from which changes will be synced.</p> |
| `resource_name` | String | ✅ | <p>The name of the Amazon Web Services resource (for example, a CloudFormation stack in the
      case of CFN_STACK_SYNC) that will be synchronized from the linked repository.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the IAM role that grants permission for Amazon Web Services to use Git sync to
      update a given Amazon Web Services resource on your behalf.</p> |
| `sync_type` | String | ✅ | <p>The type of sync configuration.</p> |
| `trigger_resource_update_on` | String |  | <p>When to trigger Git sync to begin the stack update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sync_configuration` | String | <p>The details about the sync configuration for which you want to retrieve information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sync_configuration
sync_configuration = provider.codestar_connections.Sync_configuration {
    repository_link_id = "value"  # <p>The ID of the repository link created for the connection. A repository link allows Git
      sync to monitor and sync changes to files in a specified Git repository.</p>
    config_file = "value"  # <p>The file name of the configuration file that manages syncing between the connection and the repository. This configuration file is stored in the repository.</p>
    branch = "value"  # <p>The branch in the repository from which changes will be synced.</p>
    resource_name = "value"  # <p>The name of the Amazon Web Services resource (for example, a CloudFormation stack in the
      case of CFN_STACK_SYNC) that will be synchronized from the linked repository.</p>
    role_arn = "value"  # <p>The ARN of the IAM role that grants permission for Amazon Web Services to use Git sync to
      update a given Amazon Web Services resource on your behalf.</p>
    sync_type = "value"  # <p>The type of sync configuration.</p>
}

# Access sync_configuration outputs
sync_configuration_id = sync_configuration.id
sync_configuration_sync_configuration = sync_configuration.sync_configuration
```

---


### Resource_sync_status

ResourceSyncStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `desired_state` | String | <p>The desired state of the Amazon Web Services resource for the sync status with the Git
      repository.</p> |
| `latest_successful_sync` | String | <p>The latest successful sync for the sync status with the Git repository.</p> |
| `latest_sync` | String | <p>The latest sync for the sync status with the Git repository, whether successful or not.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_sync_status outputs
resource_sync_status_id = resource_sync_status.id
resource_sync_status_desired_state = resource_sync_status.desired_state
resource_sync_status_latest_successful_sync = resource_sync_status.latest_successful_sync
resource_sync_status_latest_sync = resource_sync_status.latest_sync
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple repository_sync_status resources
repository_sync_status_0 = provider.codestar_connections.Repository_sync_status {
}
repository_sync_status_1 = provider.codestar_connections.Repository_sync_status {
}
repository_sync_status_2 = provider.codestar_connections.Repository_sync_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    repository_sync_status = provider.codestar_connections.Repository_sync_status {
    }
```

---

## Related Documentation

- [AWS Codestar_connections Documentation](https://docs.aws.amazon.com/codestar_connections/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
