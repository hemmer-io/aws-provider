# Service_catalog_appregistry Service



**Resources**: 4

---

## Overview

The service_catalog_appregistry service provides access to 4 resource types:

- [Attribute_group](#attribute_group) [CRUD]
- [Configuration](#configuration) [CR]
- [Application](#application) [CRUD]
- [Associated_resource](#associated_resource) [R]

---

## Resources


### Attribute_group

AttributeGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the attribute group.</p> |
| `description` | String |  | <p>The description of the attribute group that the user provides.</p> |
| `tags` | HashMap<String, String> |  | <p>Key-value pairs you can use to associate with the attribute group.</p> |
| `client_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If you retry a request that
      completed successfully using the same client token and the same parameters, the retry succeeds
      without performing any further actions. If you retry a successful request using the same
      client token, but one or more of the parameters are different, the retry fails.</p> |
| `attributes` | String | ✅ | <p>A JSON string in the form of nested key-value pairs that represent the attributes in the group and describes an application and its components.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_by` | String | <p>The service principal that created the attribute group.</p> |
| `description` | String | <p>The description of the attribute group that the user provides.</p> |
| `name` | String | <p>The name of the attribute group.</p> |
| `attributes` | String | <p>A JSON string in the form of nested key-value pairs that represent the attributes in the group and describes an application and its components.</p> |
| `id` | String | <p>The identifier of the attribute group.</p> |
| `arn` | String | <p>The Amazon resource name (ARN) that specifies the attribute group across services.</p> |
| `last_update_time` | String | <p>The ISO-8601 formatted timestamp of the moment the attribute group was last updated. This time is the same as the creationTime for a newly created attribute group.</p> |
| `creation_time` | String | <p>The ISO-8601 formatted timestamp of the moment the attribute group was created.</p> |
| `tags` | HashMap<String, String> | <p>Key-value pairs associated with the attribute group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create attribute_group
attribute_group = provider.service_catalog_appregistry.Attribute_group {
    name = "value"  # <p>The name of the attribute group.</p>
    client_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If you retry a request that
      completed successfully using the same client token and the same parameters, the retry succeeds
      without performing any further actions. If you retry a successful request using the same
      client token, but one or more of the parameters are different, the retry fails.</p>
    attributes = "value"  # <p>A JSON string in the form of nested key-value pairs that represent the attributes in the group and describes an application and its components.</p>
}

# Access attribute_group outputs
attribute_group_id = attribute_group.id
attribute_group_created_by = attribute_group.created_by
attribute_group_description = attribute_group.description
attribute_group_name = attribute_group.name
attribute_group_attributes = attribute_group.attributes
attribute_group_id = attribute_group.id
attribute_group_arn = attribute_group.arn
attribute_group_last_update_time = attribute_group.last_update_time
attribute_group_creation_time = attribute_group.creation_time
attribute_group_tags = attribute_group.tags
```

---


### Configuration

Configuration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration` | String | ✅ | <p>
      Associates a <code>TagKey</code> configuration 
      to an account.
    </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration` | String | <p>
      Retrieves <code>TagKey</code> configuration 
      from an account.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration
configuration = provider.service_catalog_appregistry.Configuration {
    configuration = "value"  # <p>
      Associates a <code>TagKey</code> configuration 
      to an account.
    </p>
}

# Access configuration outputs
configuration_id = configuration.id
configuration_configuration = configuration.configuration
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the application.</p> |
| `client_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If you retry a request that
      completed successfully using the same client token and the same parameters, the retry succeeds
      without performing any further actions. If you retry a successful request using the same
      client token, but one or more of the parameters are different, the retry fails.</p> |
| `name` | String | ✅ | <p>The name of the application. The name must be unique in the region in which you are creating the application.</p> |
| `tags` | HashMap<String, String> |  | <p>Key-value pairs you can use to associate with the application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The identifier of the application.</p> |
| `creation_time` | String | <p>The ISO-8601 formatted timestamp of the moment when the application was created.</p> |
| `tags` | HashMap<String, String> | <p>Key-value pairs associated with the application.</p> |
| `arn` | String | <p>The Amazon resource name (ARN) that specifies the application across services.</p> |
| `description` | String | <p>The description of the application.</p> |
| `integrations` | String | <p>
       The information 
       about the integration 
       of the application 
       with other services, 
       such as
        Resource Groups.
     </p> |
| `associated_resource_count` | i64 | <p>The number of top-level resources that were registered as part of this application.</p> |
| `application_tag` | HashMap<String, String> | <p>
      A key-value pair that identifies an associated resource.
    </p> |
| `name` | String | <p>The name of the application. The name must be unique in the region in which you are creating the application.</p> |
| `last_update_time` | String | <p>The ISO-8601 formatted timestamp of the moment when the application was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.service_catalog_appregistry.Application {
    client_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If you retry a request that
      completed successfully using the same client token and the same parameters, the retry succeeds
      without performing any further actions. If you retry a successful request using the same
      client token, but one or more of the parameters are different, the retry fails.</p>
    name = "value"  # <p>The name of the application. The name must be unique in the region in which you are creating the application.</p>
}

# Access application outputs
application_id = application.id
application_id = application.id
application_creation_time = application.creation_time
application_tags = application.tags
application_arn = application.arn
application_description = application.description
application_integrations = application.integrations
application_associated_resource_count = application.associated_resource_count
application_application_tag = application.application_tag
application_name = application.name
application_last_update_time = application.last_update_time
```

---


### Associated_resource

AssociatedResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource` | String | <p>The resource associated with the application.</p> |
| `options` | Vec<String> | <p>
      Determines whether an application tag is applied or skipped.
    </p> |
| `application_tag_result` | String | <p>
      The result of the application that's tag applied to a resource.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access associated_resource outputs
associated_resource_id = associated_resource.id
associated_resource_resource = associated_resource.resource
associated_resource_options = associated_resource.options
associated_resource_application_tag_result = associated_resource.application_tag_result
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple attribute_group resources
attribute_group_0 = provider.service_catalog_appregistry.Attribute_group {
    name = "value-0"
    client_token = "value-0"
    attributes = "value-0"
}
attribute_group_1 = provider.service_catalog_appregistry.Attribute_group {
    name = "value-1"
    client_token = "value-1"
    attributes = "value-1"
}
attribute_group_2 = provider.service_catalog_appregistry.Attribute_group {
    name = "value-2"
    client_token = "value-2"
    attributes = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    attribute_group = provider.service_catalog_appregistry.Attribute_group {
        name = "production-value"
        client_token = "production-value"
        attributes = "production-value"
    }
```

---

## Related Documentation

- [AWS Service_catalog_appregistry Documentation](https://docs.aws.amazon.com/service_catalog_appregistry/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
