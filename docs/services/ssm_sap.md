# Ssm_sap Service



**Resources**: 7

---

## Overview

The ssm_sap service provides access to 7 resource types:

- [Application_settings](#application_settings) [U]
- [Application](#application) [R]
- [Configuration_check_operation](#configuration_check_operation) [R]
- [Operation](#operation) [R]
- [Database](#database) [R]
- [Resource_permission](#resource_permission) [CRD]
- [Component](#component) [R]

---

## Resources


### Application_settings

ApplicationSettings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The ID of the application.</p> |
| `database_arn` | String |  | <p>The Amazon Resource Name of the SAP HANA database that replaces the current SAP HANA connection with the SAP_ABAP application.</p> |
| `credentials_to_remove` | Vec<String> |  | <p>The credentials to be removed.</p> |
| `credentials_to_add_or_update` | Vec<String> |  | <p>The credentials to be added or updated.</p> |
| `backint` | String |  | <p>Installation of AWS Backint Agent for SAP HANA.</p> |



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


### Application

Application resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags of a registered application.</p> |
| `application` | String | <p>Returns all of the metadata of an application registered with AWS Systems Manager for SAP.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application outputs
application_id = application.id
application_tags = application.tags
application_application = application.application
```

---


### Configuration_check_operation

ConfigurationCheckOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_check_operation` | String | <p>Returns the details of a configuration check operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_check_operation outputs
configuration_check_operation_id = configuration_check_operation.id
configuration_check_operation_configuration_check_operation = configuration_check_operation.configuration_check_operation
```

---


### Operation

Operation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operation` | String | <p>Returns the details of an operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access operation outputs
operation_id = operation.id
operation_operation = operation.operation
```

---


### Database

Database resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `database` | String | <p>The SAP HANA database of an application registered with AWS Systems Manager for SAP.</p> |
| `tags` | HashMap<String, String> | <p>The tags of a database.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access database outputs
database_id = database.id
database_database = database.database
database_tags = database.tags
```

---


### Resource_permission

ResourcePermission resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p/> |
| `action_type` | String | ✅ | <p/> |
| `source_resource_arn` | String | ✅ | <p/> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p/> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_permission
resource_permission = provider.ssm_sap.Resource_permission {
    resource_arn = "value"  # <p/>
    action_type = "value"  # <p/>
    source_resource_arn = "value"  # <p/>
}

# Access resource_permission outputs
resource_permission_id = resource_permission.id
resource_permission_policy = resource_permission.policy
```

---


### Component

Component resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags of a component.</p> |
| `component` | String | <p>The component of an application registered with AWS Systems Manager for SAP.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access component outputs
component_id = component.id
component_tags = component.tags
component_component = component.component
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple application_settings resources
application_settings_0 = provider.ssm_sap.Application_settings {
    application_id = "value-0"
}
application_settings_1 = provider.ssm_sap.Application_settings {
    application_id = "value-1"
}
application_settings_2 = provider.ssm_sap.Application_settings {
    application_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    application_settings = provider.ssm_sap.Application_settings {
        application_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Ssm_sap Documentation](https://docs.aws.amazon.com/ssm_sap/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
