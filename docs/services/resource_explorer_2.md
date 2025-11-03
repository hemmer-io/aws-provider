# Resource_explorer_2 Service



**Resources**: 7

---

## Overview

The resource_explorer_2 service provides access to 7 resource types:

- [Service_index](#service_index) [R]
- [Index](#index) [R]
- [Service_view](#service_view) [R]
- [Resource_explorer_setup](#resource_explorer_setup) [CRD]
- [Account_level_service_configuration](#account_level_service_configuration) [R]
- [Default_view](#default_view) [R]
- [Managed_view](#managed_view) [R]

---

## Resources


### Service_index

ServiceIndex resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The Amazon Resource Name (ARN) of the Resource Explorer index in the current Region.</p> |
| `type` | String | <p>The type of the index. Valid values are <code>LOCAL</code> (contains resources from the current Region only) or <code>AGGREGATOR</code> (contains replicated resource information from all Regions).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_index outputs
service_index_id = service_index.id
service_index_arn = service_index.arn
service_index_type = service_index.type
```

---


### Index

Index resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The type of the index in this Region. For information about the aggregator index and how it differs from a local index, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-aggregator-region.html">Turning on cross-Region search by creating an aggregator index</a>.</p> |
| `arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index.</p> |
| `replicating_from` | Vec<String> | <p>This response value is present only if this index is <code>Type=AGGREGATOR</code>.</p> <p>A list of the Amazon Web Services Regions that replicate their content to the index in this Region.</p> |
| `last_updated_at` | String | <p>The date and time when the index was last updated.</p> |
| `tags` | HashMap<String, String> | <p>Tag key and value pairs that are attached to the index.</p> |
| `created_at` | String | <p>The date and time when the index was originally created.</p> |
| `state` | String | <p>The current state of the index in this Amazon Web Services Region.</p> |
| `replicating_to` | Vec<String> | <p>This response value is present only if this index is <code>Type=LOCAL</code>.</p> <p>The Amazon Web Services Region that contains the aggregator index, if one exists. If an aggregator index does exist then the Region in which you called this operation replicates its index information to the Region specified in this response value. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access index outputs
index_id = index.id
index_type = index.type
index_arn = index.arn
index_replicating_from = index.replicating_from
index_last_updated_at = index.last_updated_at
index_tags = index.tags
index_created_at = index.created_at
index_state = index.state
index_replicating_to = index.replicating_to
```

---


### Service_view

ServiceView resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `view` | String | <p>A <code>ServiceView</code> object that contains the details and configuration of the requested service view.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_view outputs
service_view_id = service_view.id
service_view_view = service_view.view
```

---


### Resource_explorer_setup

ResourceExplorerSetup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aggregator_regions` | Vec<String> |  | <p>A list of Amazon Web Services Regions that should be configured as aggregator Regions. Aggregator Regions receive replicated index information from all other Regions where there is a user-owned index.</p> |
| `view_name` | String | ✅ | <p>The name for the view to be created as part of the Resource Explorer setup. The view name must be unique within the Amazon Web Services account and Region.</p> |
| `region_list` | Vec<String> | ✅ | <p>A list of Amazon Web Services Regions where Resource Explorer should be configured. Each Region in the list will have a user-owned index created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regions` | Vec<String> | <p>A list of Region status objects that describe the current state of Resource Explorer configuration in each Region.</p> |
| `next_token` | String | <p>The pagination token to use in a subsequent <code>GetResourceExplorerSetup</code> request to retrieve the next set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_explorer_setup
resource_explorer_setup = provider.resource_explorer_2.Resource_explorer_setup {
    view_name = "value"  # <p>The name for the view to be created as part of the Resource Explorer setup. The view name must be unique within the Amazon Web Services account and Region.</p>
    region_list = "value"  # <p>A list of Amazon Web Services Regions where Resource Explorer should be configured. Each Region in the list will have a user-owned index created.</p>
}

# Access resource_explorer_setup outputs
resource_explorer_setup_id = resource_explorer_setup.id
resource_explorer_setup_regions = resource_explorer_setup.regions
resource_explorer_setup_next_token = resource_explorer_setup.next_token
```

---


### Account_level_service_configuration

AccountLevelServiceConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `org_configuration` | String | <p>Details about the organization, and whether configuration is <code>ENABLED</code> or <code>DISABLED</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_level_service_configuration outputs
account_level_service_configuration_id = account_level_service_configuration.id
account_level_service_configuration_org_configuration = account_level_service_configuration.org_configuration
```

---


### Default_view

DefaultView resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `view_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that is the current default for the Amazon Web Services Region in which you called this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_view outputs
default_view_id = default_view.id
default_view_view_arn = default_view.view_arn
```

---


### Managed_view

ManagedView resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_view` | String | <p>Details about the specified managed view. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_view outputs
managed_view_id = managed_view.id
managed_view_managed_view = managed_view.managed_view
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple service_index resources
service_index_0 = provider.resource_explorer_2.Service_index {
}
service_index_1 = provider.resource_explorer_2.Service_index {
}
service_index_2 = provider.resource_explorer_2.Service_index {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service_index = provider.resource_explorer_2.Service_index {
    }
```

---

## Related Documentation

- [AWS Resource_explorer_2 Documentation](https://docs.aws.amazon.com/resource_explorer_2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
