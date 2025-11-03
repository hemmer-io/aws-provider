# Schemas Service



**Resources**: 8

---

## Overview

The schemas service provides access to 8 resource types:

- [Schema_version](#schema_version) [D]
- [Code_binding_source](#code_binding_source) [R]
- [Code_binding](#code_binding) [CR]
- [Registry](#registry) [CRUD]
- [Schema](#schema) [CRUD]
- [Discovered_schema](#discovered_schema) [R]
- [Resource_policy](#resource_policy) [CRD]
- [Discoverer](#discoverer) [CRUD]

---

## Resources


### Schema_version

SchemaVersion resource

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


### Code_binding_source

CodeBindingSource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `body` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access code_binding_source outputs
code_binding_source_id = code_binding_source.id
code_binding_source_body = code_binding_source.body
```

---


### Code_binding

CodeBinding resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_version` | String |  | <p>Specifying this limits the results to only this schema version.</p> |
| `registry_name` | String | ✅ | <p>The name of the registry.</p> |
| `language` | String | ✅ | <p>The language of the code binding.</p> |
| `schema_name` | String | ✅ | <p>The name of the schema.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The current status of code binding generation.</p> |
| `creation_date` | String | <p>The time and date that the code binding was created.</p> |
| `last_modified` | String | <p>The date and time that code bindings were modified.</p> |
| `schema_version` | String | <p>The version number of the schema.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create code_binding
code_binding = provider.schemas.Code_binding {
    registry_name = "value"  # <p>The name of the registry.</p>
    language = "value"  # <p>The language of the code binding.</p>
    schema_name = "value"  # <p>The name of the schema.</p>
}

# Access code_binding outputs
code_binding_id = code_binding.id
code_binding_status = code_binding.status
code_binding_creation_date = code_binding.creation_date
code_binding_last_modified = code_binding.last_modified
code_binding_schema_version = code_binding.schema_version
```

---


### Registry

Registry resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registry_name` | String | ✅ | <p>The name of the registry.</p> |
| `description` | String |  | <p>A description of the registry to be created.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to associate with the registry.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the registry.</p> |
| `registry_arn` | String | <p>The ARN of the registry.</p> |
| `registry_name` | String | <p>The name of the registry.</p> |
| `tags` | HashMap<String, String> | <p>Tags associated with the registry.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registry
registry = provider.schemas.Registry {
    registry_name = "value"  # <p>The name of the registry.</p>
}

# Access registry outputs
registry_id = registry.id
registry_description = registry.description
registry_registry_arn = registry.registry_arn
registry_registry_name = registry.registry_name
registry_tags = registry.tags
```

---


### Schema

Schema resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | String | ✅ | <p>The source of the schema definition.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags associated with the schema.</p> |
| `registry_name` | String | ✅ | <p>The name of the registry.</p> |
| `schema_name` | String | ✅ | <p>The name of the schema.</p> |
| `type` | String | ✅ | <p>The type of schema.</p> |
| `description` | String |  | <p>A description of the schema.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | <p>The source of the schema definition.</p> |
| `tags` | HashMap<String, String> | <p>Tags associated with the resource.</p> |
| `schema_arn` | String | <p>The ARN of the schema.</p> |
| `version_created_date` | String | <p>The date the schema version was created.</p> |
| `description` | String | <p>The description of the schema.</p> |
| `schema_name` | String | <p>The name of the schema.</p> |
| `schema_version` | String | <p>The version number of the schema</p> |
| `last_modified` | String | <p>The date and time that schema was modified.</p> |
| `type` | String | <p>The type of the schema.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schema
schema = provider.schemas.Schema {
    content = "value"  # <p>The source of the schema definition.</p>
    registry_name = "value"  # <p>The name of the registry.</p>
    schema_name = "value"  # <p>The name of the schema.</p>
    type = "value"  # <p>The type of schema.</p>
}

# Access schema outputs
schema_id = schema.id
schema_content = schema.content
schema_tags = schema.tags
schema_schema_arn = schema.schema_arn
schema_version_created_date = schema.version_created_date
schema_description = schema.description
schema_schema_name = schema.schema_name
schema_schema_version = schema.schema_version
schema_last_modified = schema.last_modified
schema_type = schema.type
```

---


### Discovered_schema

DiscoveredSchema resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | <p>The source of the schema definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access discovered_schema outputs
discovered_schema_id = discovered_schema.id
discovered_schema_content = discovered_schema.content
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `revision_id` | String |  | <p>The revision ID of the policy.</p> |
| `policy` | String | ✅ | <p>The resource-based policy.</p> |
| `registry_name` | String |  | <p>The name of the registry.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `revision_id` | String | <p>The revision ID.</p> |
| `policy` | String | <p>The resource-based policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.schemas.Resource_policy {
    policy = "value"  # <p>The resource-based policy.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_revision_id = resource_policy.revision_id
resource_policy_policy = resource_policy.policy
```

---


### Discoverer

Discoverer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cross_account` | bool |  | <p>Support discovery of schemas in events sent to the bus from another account. (default: true).</p> |
| `tags` | HashMap<String, String> |  | <p>Tags associated with the resource.</p> |
| `description` | String |  | <p>A description for the discoverer.</p> |
| `source_arn` | String | ✅ | <p>The ARN of the event bus.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>Tags associated with the resource.</p> |
| `discoverer_id` | String | <p>The ID of the discoverer.</p> |
| `description` | String | <p>The description of the discoverer.</p> |
| `discoverer_arn` | String | <p>The ARN of the discoverer.</p> |
| `source_arn` | String | <p>The ARN of the event bus.</p> |
| `cross_account` | bool | <p>The Status if the discoverer will discover schemas from events sent from another account.</p> |
| `state` | String | <p>The state of the discoverer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create discoverer
discoverer = provider.schemas.Discoverer {
    source_arn = "value"  # <p>The ARN of the event bus.</p>
}

# Access discoverer outputs
discoverer_id = discoverer.id
discoverer_tags = discoverer.tags
discoverer_discoverer_id = discoverer.discoverer_id
discoverer_description = discoverer.description
discoverer_discoverer_arn = discoverer.discoverer_arn
discoverer_source_arn = discoverer.source_arn
discoverer_cross_account = discoverer.cross_account
discoverer_state = discoverer.state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple schema_version resources
schema_version_0 = provider.schemas.Schema_version {
}
schema_version_1 = provider.schemas.Schema_version {
}
schema_version_2 = provider.schemas.Schema_version {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    schema_version = provider.schemas.Schema_version {
    }
```

---

## Related Documentation

- [AWS Schemas Documentation](https://docs.aws.amazon.com/schemas/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
