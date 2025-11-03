# Ssm_quicksetup Service



**Resources**: 4

---

## Overview

The ssm_quicksetup service provides access to 4 resource types:

- [Service_settings](#service_settings) [RU]
- [Configuration](#configuration) [R]
- [Configuration_definition](#configuration_definition) [U]
- [Configuration_manager](#configuration_manager) [CRUD]

---

## Resources


### Service_settings

ServiceSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `explorer_enabling_role_arn` | String |  | <p>The IAM role used to enable Explorer.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_settings` | String | <p>Returns details about the settings for Quick Setup in the requesting Amazon Web Services account and Amazon Web Services Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_settings outputs
service_settings_id = service_settings.id
service_settings_service_settings = service_settings.service_settings
```

---


### Configuration

Configuration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `manager_arn` | String | <p>The ARN of the configuration manager.</p> |
| `region` | String | <p>The Amazon Web Services Region where the configuration was deployed.</p> |
| `id` | String | <p>A service generated identifier for the configuration.</p> |
| `type_version` | String | <p>The version of the Quick Setup type used.</p> |
| `account` | String | <p>The ID of the Amazon Web Services account where the configuration was deployed.</p> |
| `created_at` | String | <p>The datetime stamp when the configuration manager was created.</p> |
| `last_modified_at` | String | <p>The datetime stamp when the configuration manager was last updated.</p> |
| `status_summaries` | Vec<String> | <p>A summary of the state of the configuration manager. This includes deployment
                  statuses, association statuses, drift statuses, health checks, and more.</p> |
| `configuration_definition_id` | String | <p>The ID of the configuration definition.</p> |
| `parameters` | HashMap<String, String> | <p>The parameters for the configuration definition type.</p> |
| `type` | String | <p>The type of the Quick Setup configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration outputs
configuration_id = configuration.id
configuration_manager_arn = configuration.manager_arn
configuration_region = configuration.region
configuration_id = configuration.id
configuration_type_version = configuration.type_version
configuration_account = configuration.account
configuration_created_at = configuration.created_at
configuration_last_modified_at = configuration.last_modified_at
configuration_status_summaries = configuration.status_summaries
configuration_configuration_definition_id = configuration.configuration_definition_id
configuration_parameters = configuration.parameters
configuration_type = configuration.type
```

---


### Configuration_definition

ConfigurationDefinition resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `manager_arn` | String | ✅ | <p>The ARN of the configuration manager associated with the definition to
                  update.</p> |
| `id` | String | ✅ | <p>The ID of the configuration definition you want to update.</p> |
| `type_version` | String |  | <p>The version of the Quick Setup type to use.</p> |
| `parameters` | HashMap<String, String> |  | <p>The parameters for the configuration definition type.</p> |
| `local_deployment_execution_role_name` | String |  | <p>The name of the IAM role used to deploy local
                  configurations.</p> |
| `local_deployment_administration_role_arn` | String |  | <p>The ARN of the IAM role used to administrate local configuration
                  deployments.</p> |



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


### Configuration_manager

ConfigurationManager resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_definitions` | Vec<String> | ✅ | <p>The definition of the Quick Setup configuration that the configuration manager
                  deploys.</p> |
| `tags` | HashMap<String, String> |  | <p>Key-value pairs of metadata to assign to the configuration manager.</p> |
| `description` | String |  | <p>A description of the configuration manager.</p> |
| `name` | String |  | <p>A name for the configuration manager.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_definitions` | Vec<String> | <p>The configuration definitions association with the configuration manager.</p> |
| `name` | String | <p>The name of the configuration manager.</p> |
| `created_at` | String | <p>The datetime stamp when the configuration manager was created.</p> |
| `last_modified_at` | String | <p>The datetime stamp when the configuration manager was last updated.</p> |
| `tags` | HashMap<String, String> | <p>Key-value pairs of metadata to assign to the configuration manager.</p> |
| `manager_arn` | String | <p>The ARN of the configuration manager.</p> |
| `status_summaries` | Vec<String> | <p>A summary of the state of the configuration manager. This includes deployment
                  statuses, association statuses, drift statuses, health checks, and more.</p> |
| `description` | String | <p>The description of the configuration manager.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_manager
configuration_manager = provider.ssm_quicksetup.Configuration_manager {
    configuration_definitions = "value"  # <p>The definition of the Quick Setup configuration that the configuration manager
                  deploys.</p>
}

# Access configuration_manager outputs
configuration_manager_id = configuration_manager.id
configuration_manager_configuration_definitions = configuration_manager.configuration_definitions
configuration_manager_name = configuration_manager.name
configuration_manager_created_at = configuration_manager.created_at
configuration_manager_last_modified_at = configuration_manager.last_modified_at
configuration_manager_tags = configuration_manager.tags
configuration_manager_manager_arn = configuration_manager.manager_arn
configuration_manager_status_summaries = configuration_manager.status_summaries
configuration_manager_description = configuration_manager.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple service_settings resources
service_settings_0 = provider.ssm_quicksetup.Service_settings {
}
service_settings_1 = provider.ssm_quicksetup.Service_settings {
}
service_settings_2 = provider.ssm_quicksetup.Service_settings {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service_settings = provider.ssm_quicksetup.Service_settings {
    }
```

---

## Related Documentation

- [AWS Ssm_quicksetup Documentation](https://docs.aws.amazon.com/ssm_quicksetup/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
