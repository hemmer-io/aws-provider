# Proton Service



**Resources**: 4

---

## Overview

The proton service provides access to 4 resource types:

- [Service_instance_sync_status](#service_instance_sync_status) [R]
- [Repository_sync_status](#repository_sync_status) [R]
- [Template_sync_status](#template_sync_status) [R]
- [Resources_summary](#resources_summary) [R]

---

## Resources


### Service_instance_sync_status

ServiceInstanceSyncStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_successful_sync` | String | <p>The detailed data of the latest successful sync with the service instance.</p> |
| `desired_state` | String | <p>The service instance sync desired state that's returned by Proton</p> |
| `latest_sync` | String | <p>The detailed data of the latest sync with the service instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_instance_sync_status outputs
service_instance_sync_status_id = service_instance_sync_status.id
service_instance_sync_status_latest_successful_sync = service_instance_sync_status.latest_successful_sync
service_instance_sync_status_desired_state = service_instance_sync_status.desired_state
service_instance_sync_status_latest_sync = service_instance_sync_status.latest_sync
```

---


### Repository_sync_status

RepositorySyncStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_sync` | String | <p>The repository sync status detail data that's returned by Proton.</p> |


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


### Template_sync_status

TemplateSyncStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `desired_state` | String | <p>The template sync desired state that's returned by Proton.</p> |
| `latest_successful_sync` | String | <p>The details of the last successful sync that's returned by Proton.</p> |
| `latest_sync` | String | <p>The details of the last sync that's returned by Proton.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access template_sync_status outputs
template_sync_status_id = template_sync_status.id
template_sync_status_desired_state = template_sync_status.desired_state
template_sync_status_latest_successful_sync = template_sync_status.latest_successful_sync
template_sync_status_latest_sync = template_sync_status.latest_sync
```

---


### Resources_summary

ResourcesSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `counts` | String | <p>Summary counts of each Proton resource type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resources_summary outputs
resources_summary_id = resources_summary.id
resources_summary_counts = resources_summary.counts
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple service_instance_sync_status resources
service_instance_sync_status_0 = provider.proton.Service_instance_sync_status {
}
service_instance_sync_status_1 = provider.proton.Service_instance_sync_status {
}
service_instance_sync_status_2 = provider.proton.Service_instance_sync_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service_instance_sync_status = provider.proton.Service_instance_sync_status {
    }
```

---

## Related Documentation

- [AWS Proton Documentation](https://docs.aws.amazon.com/proton/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
