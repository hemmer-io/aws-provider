# Workspaces_instances Service



**Resources**: 2

---

## Overview

The workspaces_instances service provides access to 2 resource types:

- [Workspace_instance](#workspace_instance) [CRD]
- [Volume](#volume) [CD]

---

## Resources


### Workspace_instance

WorkspaceInstance resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique token to ensure idempotent instance creation, preventing duplicate workspace launches.</p> |
| `managed_instance` | String | ✅ | <p>Comprehensive configuration settings for the WorkSpaces Instance, including network, compute, and storage parameters.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata tags for categorizing and managing WorkSpaces Instances.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provision_state` | String | <p>Current provisioning state of the WorkSpaces Instance.</p> |
| `workspace_instance_errors` | Vec<String> | <p>Captures any errors specific to the WorkSpace Instance lifecycle.</p> |
| `ec2_instance_errors` | Vec<String> | <p>Includes any underlying EC2 instance errors encountered.</p> |
| `workspace_instance_id` | String | <p>Unique identifier of the retrieved WorkSpaces Instance.</p> |
| `ec2_managed_instance` | String | <p>Details of the associated EC2 managed instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workspace_instance
workspace_instance = provider.workspaces_instances.Workspace_instance {
    managed_instance = "value"  # <p>Comprehensive configuration settings for the WorkSpaces Instance, including network, compute, and storage parameters.</p>
}

# Access workspace_instance outputs
workspace_instance_id = workspace_instance.id
workspace_instance_provision_state = workspace_instance.provision_state
workspace_instance_workspace_instance_errors = workspace_instance.workspace_instance_errors
workspace_instance_ec2_instance_errors = workspace_instance.ec2_instance_errors
workspace_instance_workspace_instance_id = workspace_instance.workspace_instance_id
workspace_instance_ec2_managed_instance = workspace_instance.ec2_managed_instance
```

---


### Volume

Volume resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `throughput` | i64 |  | <p>Volume throughput performance.</p> |
| `size_in_gb` | i64 |  | <p>Volume size in gigabytes.</p> |
| `kms_key_id` | String |  | <p>KMS key for volume encryption.</p> |
| `client_token` | String |  | <p>Unique token to prevent duplicate volume creation.</p> |
| `encrypted` | bool |  | <p>Indicates if the volume should be encrypted.</p> |
| `snapshot_id` | String |  | <p>Source snapshot for volume creation.</p> |
| `tag_specifications` | Vec<String> |  | <p>Metadata tags for the volume.</p> |
| `availability_zone` | String | ✅ | <p>Availability zone for the volume.</p> |
| `iops` | i64 |  | <p>Input/output operations per second for the volume.</p> |
| `volume_type` | String |  | <p>Type of EBS volume.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create volume
volume = provider.workspaces_instances.Volume {
    availability_zone = "value"  # <p>Availability zone for the volume.</p>
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

# Create multiple workspace_instance resources
workspace_instance_0 = provider.workspaces_instances.Workspace_instance {
    managed_instance = "value-0"
}
workspace_instance_1 = provider.workspaces_instances.Workspace_instance {
    managed_instance = "value-1"
}
workspace_instance_2 = provider.workspaces_instances.Workspace_instance {
    managed_instance = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    workspace_instance = provider.workspaces_instances.Workspace_instance {
        managed_instance = "production-value"
    }
```

---

## Related Documentation

- [AWS Workspaces_instances Documentation](https://docs.aws.amazon.com/workspaces_instances/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
