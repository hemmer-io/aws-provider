# Greengrass Service



**Resources**: 26

---

## Overview

The greengrass service provides access to 26 resource types:

- [Connectivity_info](#connectivity_info) [RU]
- [Bulk_deployment_status](#bulk_deployment_status) [R]
- [Logger_definition](#logger_definition) [CRUD]
- [Connector_definition](#connector_definition) [CRUD]
- [Subscription_definition_version](#subscription_definition_version) [CR]
- [Associated_role](#associated_role) [R]
- [Thing_runtime_configuration](#thing_runtime_configuration) [RU]
- [Resource_definition](#resource_definition) [CRUD]
- [Device_definition](#device_definition) [CRUD]
- [Function_definition_version](#function_definition_version) [CR]
- [Group_certificate_configuration](#group_certificate_configuration) [RU]
- [Software_update_job](#software_update_job) [C]
- [Subscription_definition](#subscription_definition) [CRUD]
- [Deployment_status](#deployment_status) [R]
- [Core_definition](#core_definition) [CRUD]
- [Resource_definition_version](#resource_definition_version) [CR]
- [Function_definition](#function_definition) [CRUD]
- [Logger_definition_version](#logger_definition_version) [CR]
- [Group_certificate_authority](#group_certificate_authority) [CR]
- [Deployment](#deployment) [C]
- [Core_definition_version](#core_definition_version) [CR]
- [Connector_definition_version](#connector_definition_version) [CR]
- [Device_definition_version](#device_definition_version) [CR]
- [Group](#group) [CRUD]
- [Group_version](#group_version) [CR]
- [Service_role_for_account](#service_role_for_account) [R]

---

## Resources


### Connectivity_info

ConnectivityInfo resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thing_name` | String | ✅ | The thing name. |
| `connectivity_info` | Vec<String> |  | A list of connectivity info. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message` | String | A message about the connectivity info request. |
| `connectivity_info` | Vec<String> | Connectivity info list. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connectivity_info outputs
connectivity_info_id = connectivity_info.id
connectivity_info_message = connectivity_info.message
connectivity_info_connectivity_info = connectivity_info.connectivity_info
```

---


### Bulk_deployment_status

BulkDeploymentStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bulk_deployment_metrics` | String | Relevant metrics on input records processed during bulk deployment. |
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `error_details` | Vec<String> | Error details |
| `created_at` | String | The time, in ISO format, when the deployment was created. |
| `error_message` | String | Error message |
| `bulk_deployment_status` | String | The status of the bulk deployment. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bulk_deployment_status outputs
bulk_deployment_status_id = bulk_deployment_status.id
bulk_deployment_status_bulk_deployment_metrics = bulk_deployment_status.bulk_deployment_metrics
bulk_deployment_status_tags = bulk_deployment_status.tags
bulk_deployment_status_error_details = bulk_deployment_status.error_details
bulk_deployment_status_created_at = bulk_deployment_status.created_at
bulk_deployment_status_error_message = bulk_deployment_status.error_message
bulk_deployment_status_bulk_deployment_status = bulk_deployment_status.bulk_deployment_status
```

---


### Logger_definition

LoggerDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `name` | String |  | The name of the logger definition. |
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |
| `initial_version` | String |  | Information about the initial version of the logger definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `arn` | String | The ARN of the definition. |
| `id` | String | The ID of the definition. |
| `name` | String | The name of the definition. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `latest_version` | String | The ID of the latest version associated with the definition. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logger_definition
logger_definition = provider.greengrass.Logger_definition {
}

# Access logger_definition outputs
logger_definition_id = logger_definition.id
logger_definition_last_updated_timestamp = logger_definition.last_updated_timestamp
logger_definition_arn = logger_definition.arn
logger_definition_id = logger_definition.id
logger_definition_name = logger_definition.name
logger_definition_latest_version_arn = logger_definition.latest_version_arn
logger_definition_creation_timestamp = logger_definition.creation_timestamp
logger_definition_tags = logger_definition.tags
logger_definition_latest_version = logger_definition.latest_version
```

---


### Connector_definition

ConnectorDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `name` | String |  | The name of the connector definition. |
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |
| `initial_version` | String |  | Information about the initial version of the connector definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the definition. |
| `arn` | String | The ARN of the definition. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `latest_version` | String | The ID of the latest version associated with the definition. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `id` | String | The ID of the definition. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connector_definition
connector_definition = provider.greengrass.Connector_definition {
}

# Access connector_definition outputs
connector_definition_id = connector_definition.id
connector_definition_name = connector_definition.name
connector_definition_arn = connector_definition.arn
connector_definition_latest_version_arn = connector_definition.latest_version_arn
connector_definition_latest_version = connector_definition.latest_version
connector_definition_creation_timestamp = connector_definition.creation_timestamp
connector_definition_last_updated_timestamp = connector_definition.last_updated_timestamp
connector_definition_tags = connector_definition.tags
connector_definition_id = connector_definition.id
```

---


### Subscription_definition_version

SubscriptionDefinitionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `subscription_definition_id` | String | ✅ | The ID of the subscription definition. |
| `subscriptions` | Vec<String> |  | A list of subscriptions. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | The ARN of the subscription definition version. |
| `version` | String | The version of the subscription definition version. |
| `id` | String | The ID of the subscription definition version. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the subscription definition version was created. |
| `next_token` | String | The token for the next set of results, or ''null'' if there are no additional results. |
| `definition` | String | Information about the subscription definition version. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription_definition_version
subscription_definition_version = provider.greengrass.Subscription_definition_version {
    subscription_definition_id = "value"  # The ID of the subscription definition.
}

# Access subscription_definition_version outputs
subscription_definition_version_id = subscription_definition_version.id
subscription_definition_version_arn = subscription_definition_version.arn
subscription_definition_version_version = subscription_definition_version.version
subscription_definition_version_id = subscription_definition_version.id
subscription_definition_version_creation_timestamp = subscription_definition_version.creation_timestamp
subscription_definition_version_next_token = subscription_definition_version.next_token
subscription_definition_version_definition = subscription_definition_version.definition
```

---


### Associated_role

AssociatedRole resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associated_at` | String | The time when the role was associated with the group. |
| `role_arn` | String | The ARN of the role that is associated with the group. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access associated_role outputs
associated_role_id = associated_role.id
associated_role_associated_at = associated_role.associated_at
associated_role_role_arn = associated_role.role_arn
```

---


### Thing_runtime_configuration

ThingRuntimeConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `telemetry_configuration` | String |  | Configuration for telemetry service. |
| `thing_name` | String | ✅ | The thing name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtime_configuration` | String | Runtime configuration for a thing. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access thing_runtime_configuration outputs
thing_runtime_configuration_id = thing_runtime_configuration.id
thing_runtime_configuration_runtime_configuration = thing_runtime_configuration.runtime_configuration
```

---


### Resource_definition

ResourceDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `initial_version` | String |  | Information about the initial version of the resource definition. |
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |
| `name` | String |  | The name of the resource definition. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `arn` | String | The ARN of the definition. |
| `name` | String | The name of the definition. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |
| `latest_version` | String | The ID of the latest version associated with the definition. |
| `id` | String | The ID of the definition. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_definition
resource_definition = provider.greengrass.Resource_definition {
}

# Access resource_definition outputs
resource_definition_id = resource_definition.id
resource_definition_tags = resource_definition.tags
resource_definition_last_updated_timestamp = resource_definition.last_updated_timestamp
resource_definition_arn = resource_definition.arn
resource_definition_name = resource_definition.name
resource_definition_latest_version_arn = resource_definition.latest_version_arn
resource_definition_creation_timestamp = resource_definition.creation_timestamp
resource_definition_latest_version = resource_definition.latest_version
resource_definition_id = resource_definition.id
```

---


### Device_definition

DeviceDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `name` | String |  | The name of the device definition. |
| `initial_version` | String |  | Information about the initial version of the device definition. |
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the definition. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `arn` | String | The ARN of the definition. |
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `latest_version` | String | The ID of the latest version associated with the definition. |
| `name` | String | The name of the definition. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create device_definition
device_definition = provider.greengrass.Device_definition {
}

# Access device_definition outputs
device_definition_id = device_definition.id
device_definition_id = device_definition.id
device_definition_latest_version_arn = device_definition.latest_version_arn
device_definition_arn = device_definition.arn
device_definition_tags = device_definition.tags
device_definition_creation_timestamp = device_definition.creation_timestamp
device_definition_last_updated_timestamp = device_definition.last_updated_timestamp
device_definition_latest_version = device_definition.latest_version
device_definition_name = device_definition.name
```

---


### Function_definition_version

FunctionDefinitionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `function_definition_id` | String | ✅ | The ID of the Lambda function definition. |
| `functions` | Vec<String> |  | A list of Lambda functions in this function definition version. |
| `default_config` | String |  | The default configuration that applies to all Lambda functions in this function definition version. Individual Lambda functions can override these settings. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `definition` | String | Information on the definition. |
| `next_token` | String | The token for the next set of results, or ''null'' if there are no additional results. |
| `version` | String | The version of the function definition version. |
| `id` | String | The ID of the function definition version. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the function definition version was created. |
| `arn` | String | The ARN of the function definition version. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create function_definition_version
function_definition_version = provider.greengrass.Function_definition_version {
    function_definition_id = "value"  # The ID of the Lambda function definition.
}

# Access function_definition_version outputs
function_definition_version_id = function_definition_version.id
function_definition_version_definition = function_definition_version.definition
function_definition_version_next_token = function_definition_version.next_token
function_definition_version_version = function_definition_version.version
function_definition_version_id = function_definition_version.id
function_definition_version_creation_timestamp = function_definition_version.creation_timestamp
function_definition_version_arn = function_definition_version.arn
```

---


### Group_certificate_configuration

GroupCertificateConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_expiry_in_milliseconds` | String |  | The amount of time remaining before the certificate expires, in milliseconds. |
| `group_id` | String | ✅ | The ID of the Greengrass group. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_id` | String | The ID of the group certificate configuration. |
| `certificate_expiry_in_milliseconds` | String | The amount of time remaining before the certificate expires, in milliseconds. |
| `certificate_authority_expiry_in_milliseconds` | String | The amount of time remaining before the certificate authority expires, in milliseconds. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access group_certificate_configuration outputs
group_certificate_configuration_id = group_certificate_configuration.id
group_certificate_configuration_group_id = group_certificate_configuration.group_id
group_certificate_configuration_certificate_expiry_in_milliseconds = group_certificate_configuration.certificate_expiry_in_milliseconds
group_certificate_configuration_certificate_authority_expiry_in_milliseconds = group_certificate_configuration.certificate_authority_expiry_in_milliseconds
```

---


### Software_update_job

SoftwareUpdateJob resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `update_targets_architecture` | String | ✅ |  |
| `s3_url_signer_role` | String | ✅ |  |
| `update_agent_log_level` | String |  |  |
| `software_to_update` | String | ✅ |  |
| `update_targets_operating_system` | String | ✅ |  |
| `update_targets` | Vec<String> | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create software_update_job
software_update_job = provider.greengrass.Software_update_job {
    update_targets_architecture = "value"  # Required field
    s3_url_signer_role = "value"  # Required field
    software_to_update = "value"  # Required field
    update_targets_operating_system = "value"  # Required field
    update_targets = "value"  # Required field
}

```

---


### Subscription_definition

SubscriptionDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `name` | String |  | The name of the subscription definition. |
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |
| `initial_version` | String |  | Information about the initial version of the subscription definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `id` | String | The ID of the definition. |
| `latest_version` | String | The ID of the latest version associated with the definition. |
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `name` | String | The name of the definition. |
| `arn` | String | The ARN of the definition. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription_definition
subscription_definition = provider.greengrass.Subscription_definition {
}

# Access subscription_definition outputs
subscription_definition_id = subscription_definition.id
subscription_definition_tags = subscription_definition.tags
subscription_definition_id = subscription_definition.id
subscription_definition_latest_version = subscription_definition.latest_version
subscription_definition_last_updated_timestamp = subscription_definition.last_updated_timestamp
subscription_definition_latest_version_arn = subscription_definition.latest_version_arn
subscription_definition_name = subscription_definition.name
subscription_definition_arn = subscription_definition.arn
subscription_definition_creation_timestamp = subscription_definition.creation_timestamp
```

---


### Deployment_status

DeploymentStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_details` | Vec<String> | Error details |
| `deployment_status` | String | The status of the deployment: ''InProgress'', ''Building'', ''Success'', or ''Failure''. |
| `error_message` | String | Error message |
| `updated_at` | String | The time, in milliseconds since the epoch, when the deployment status was updated. |
| `deployment_type` | String | The type of the deployment. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deployment_status outputs
deployment_status_id = deployment_status.id
deployment_status_error_details = deployment_status.error_details
deployment_status_deployment_status = deployment_status.deployment_status
deployment_status_error_message = deployment_status.error_message
deployment_status_updated_at = deployment_status.updated_at
deployment_status_deployment_type = deployment_status.deployment_type
```

---


### Core_definition

CoreDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `initial_version` | String |  | Information about the initial version of the core definition. |
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `name` | String |  | The name of the core definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | The ARN of the definition. |
| `latest_version` | String | The ID of the latest version associated with the definition. |
| `id` | String | The ID of the definition. |
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `name` | String | The name of the definition. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create core_definition
core_definition = provider.greengrass.Core_definition {
}

# Access core_definition outputs
core_definition_id = core_definition.id
core_definition_arn = core_definition.arn
core_definition_latest_version = core_definition.latest_version
core_definition_id = core_definition.id
core_definition_last_updated_timestamp = core_definition.last_updated_timestamp
core_definition_creation_timestamp = core_definition.creation_timestamp
core_definition_latest_version_arn = core_definition.latest_version_arn
core_definition_tags = core_definition.tags
core_definition_name = core_definition.name
```

---


### Resource_definition_version

ResourceDefinitionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resources` | Vec<String> |  | A list of resources. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `resource_definition_id` | String | ✅ | The ID of the resource definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the resource definition version. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the resource definition version was created. |
| `arn` | String | Arn of the resource definition version. |
| `definition` | String | Information about the definition. |
| `version` | String | The version of the resource definition version. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_definition_version
resource_definition_version = provider.greengrass.Resource_definition_version {
    resource_definition_id = "value"  # The ID of the resource definition.
}

# Access resource_definition_version outputs
resource_definition_version_id = resource_definition_version.id
resource_definition_version_id = resource_definition_version.id
resource_definition_version_creation_timestamp = resource_definition_version.creation_timestamp
resource_definition_version_arn = resource_definition_version.arn
resource_definition_version_definition = resource_definition_version.definition
resource_definition_version_version = resource_definition_version.version
```

---


### Function_definition

FunctionDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `name` | String |  | The name of the function definition. |
| `initial_version` | String |  | Information about the initial version of the function definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the definition. |
| `name` | String | The name of the definition. |
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `arn` | String | The ARN of the definition. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |
| `latest_version` | String | The ID of the latest version associated with the definition. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create function_definition
function_definition = provider.greengrass.Function_definition {
}

# Access function_definition outputs
function_definition_id = function_definition.id
function_definition_id = function_definition.id
function_definition_name = function_definition.name
function_definition_tags = function_definition.tags
function_definition_latest_version_arn = function_definition.latest_version_arn
function_definition_last_updated_timestamp = function_definition.last_updated_timestamp
function_definition_arn = function_definition.arn
function_definition_creation_timestamp = function_definition.creation_timestamp
function_definition_latest_version = function_definition.latest_version
```

---


### Logger_definition_version

LoggerDefinitionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `loggers` | Vec<String> |  | A list of loggers. |
| `logger_definition_id` | String | ✅ | The ID of the logger definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | The version of the logger definition version. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the logger definition version was created. |
| `arn` | String | The ARN of the logger definition version. |
| `definition` | String | Information about the logger definition version. |
| `id` | String | The ID of the logger definition version. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logger_definition_version
logger_definition_version = provider.greengrass.Logger_definition_version {
    logger_definition_id = "value"  # The ID of the logger definition.
}

# Access logger_definition_version outputs
logger_definition_version_id = logger_definition_version.id
logger_definition_version_version = logger_definition_version.version
logger_definition_version_creation_timestamp = logger_definition_version.creation_timestamp
logger_definition_version_arn = logger_definition_version.arn
logger_definition_version_definition = logger_definition_version.definition
logger_definition_version_id = logger_definition_version.id
```

---


### Group_certificate_authority

GroupCertificateAuthority resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_id` | String | ✅ | The ID of the Greengrass group. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_certificate_authority_id` | String | The ID of the certificate authority for the group. |
| `group_certificate_authority_arn` | String | The ARN of the certificate authority for the group. |
| `pem_encoded_certificate` | String | The PEM encoded certificate for the group. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group_certificate_authority
group_certificate_authority = provider.greengrass.Group_certificate_authority {
    group_id = "value"  # The ID of the Greengrass group.
}

# Access group_certificate_authority outputs
group_certificate_authority_id = group_certificate_authority.id
group_certificate_authority_group_certificate_authority_id = group_certificate_authority.group_certificate_authority_id
group_certificate_authority_group_certificate_authority_arn = group_certificate_authority.group_certificate_authority_arn
group_certificate_authority_pem_encoded_certificate = group_certificate_authority.pem_encoded_certificate
```

---


### Deployment

Deployment resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_type` | String | ✅ | The type of deployment. When used for ''CreateDeployment'', only ''NewDeployment'' and ''Redeployment'' are valid. |
| `deployment_id` | String |  | The ID of the deployment if you wish to redeploy a previous deployment. |
| `group_id` | String | ✅ | The ID of the Greengrass group. |
| `group_version_id` | String |  | The ID of the group version to be deployed. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment
deployment = provider.greengrass.Deployment {
    deployment_type = "value"  # The type of deployment. When used for ''CreateDeployment'', only ''NewDeployment'' and ''Redeployment'' are valid.
    group_id = "value"  # The ID of the Greengrass group.
}

```

---


### Core_definition_version

CoreDefinitionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `core_definition_id` | String | ✅ | The ID of the core definition. |
| `cores` | Vec<String> |  | A list of cores in the core definition version. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the core definition version. |
| `next_token` | String | The token for the next set of results, or ''null'' if there are no additional results. |
| `arn` | String | The ARN of the core definition version. |
| `definition` | String | Information about the core definition version. |
| `version` | String | The version of the core definition version. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the core definition version was created. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create core_definition_version
core_definition_version = provider.greengrass.Core_definition_version {
    core_definition_id = "value"  # The ID of the core definition.
}

# Access core_definition_version outputs
core_definition_version_id = core_definition_version.id
core_definition_version_id = core_definition_version.id
core_definition_version_next_token = core_definition_version.next_token
core_definition_version_arn = core_definition_version.arn
core_definition_version_definition = core_definition_version.definition
core_definition_version_version = core_definition_version.version
core_definition_version_creation_timestamp = core_definition_version.creation_timestamp
```

---


### Connector_definition_version

ConnectorDefinitionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connectors` | Vec<String> |  | A list of references to connectors in this version, with their corresponding configuration settings. |
| `connector_definition_id` | String | ✅ | The ID of the connector definition. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | The ARN of the connector definition version. |
| `id` | String | The ID of the connector definition version. |
| `definition` | String | Information about the connector definition version. |
| `next_token` | String | The token for the next set of results, or ''null'' if there are no additional results. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the connector definition version was created. |
| `version` | String | The version of the connector definition version. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connector_definition_version
connector_definition_version = provider.greengrass.Connector_definition_version {
    connector_definition_id = "value"  # The ID of the connector definition.
}

# Access connector_definition_version outputs
connector_definition_version_id = connector_definition_version.id
connector_definition_version_arn = connector_definition_version.arn
connector_definition_version_id = connector_definition_version.id
connector_definition_version_definition = connector_definition_version.definition
connector_definition_version_next_token = connector_definition_version.next_token
connector_definition_version_creation_timestamp = connector_definition_version.creation_timestamp
connector_definition_version_version = connector_definition_version.version
```

---


### Device_definition_version

DeviceDefinitionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_definition_id` | String | ✅ | The ID of the device definition. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `devices` | Vec<String> |  | A list of devices in the definition version. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the device definition version. |
| `version` | String | The version of the device definition version. |
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the device definition version was created. |
| `definition` | String | Information about the device definition version. |
| `next_token` | String | The token for the next set of results, or ''null'' if there are no additional results. |
| `arn` | String | The ARN of the device definition version. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create device_definition_version
device_definition_version = provider.greengrass.Device_definition_version {
    device_definition_id = "value"  # The ID of the device definition.
}

# Access device_definition_version outputs
device_definition_version_id = device_definition_version.id
device_definition_version_id = device_definition_version.id
device_definition_version_version = device_definition_version.version
device_definition_version_creation_timestamp = device_definition_version.creation_timestamp
device_definition_version_definition = device_definition_version.definition
device_definition_version_next_token = device_definition_version.next_token
device_definition_version_arn = device_definition_version.arn
```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the group. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `initial_version` | String |  | Information about the initial version of the group. |
| `tags` | HashMap<String, String> |  | Tag(s) to add to the new resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the definition was created. |
| `id` | String | The ID of the definition. |
| `latest_version_arn` | String | The ARN of the latest version associated with the definition. |
| `tags` | HashMap<String, String> | Tag(s) attached to the resource arn. |
| `last_updated_timestamp` | String | The time, in milliseconds since the epoch, when the definition was last updated. |
| `latest_version` | String | The ID of the latest version associated with the definition. |
| `arn` | String | The ARN of the definition. |
| `name` | String | The name of the definition. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.greengrass.Group {
    name = "value"  # The name of the group.
}

# Access group outputs
group_id = group.id
group_creation_timestamp = group.creation_timestamp
group_id = group.id
group_latest_version_arn = group.latest_version_arn
group_tags = group.tags
group_last_updated_timestamp = group.last_updated_timestamp
group_latest_version = group.latest_version
group_arn = group.arn
group_name = group.name
```

---


### Group_version

GroupVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logger_definition_version_arn` | String |  | The ARN of the logger definition version for this group. |
| `core_definition_version_arn` | String |  | The ARN of the core definition version for this group. |
| `device_definition_version_arn` | String |  | The ARN of the device definition version for this group. |
| `resource_definition_version_arn` | String |  | The ARN of the resource definition version for this group. |
| `amzn_client_token` | String |  | A client token used to correlate requests and responses. |
| `subscription_definition_version_arn` | String |  | The ARN of the subscription definition version for this group. |
| `connector_definition_version_arn` | String |  | The ARN of the connector definition version for this group. |
| `function_definition_version_arn` | String |  | The ARN of the function definition version for this group. |
| `group_id` | String | ✅ | The ID of the Greengrass group. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_timestamp` | String | The time, in milliseconds since the epoch, when the group version was created. |
| `arn` | String | The ARN of the group version. |
| `definition` | String | Information about the group version definition. |
| `id` | String | The ID of the group that the version is associated with. |
| `version` | String | The ID of the group version. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group_version
group_version = provider.greengrass.Group_version {
    group_id = "value"  # The ID of the Greengrass group.
}

# Access group_version outputs
group_version_id = group_version.id
group_version_creation_timestamp = group_version.creation_timestamp
group_version_arn = group_version.arn
group_version_definition = group_version.definition
group_version_id = group_version.id
group_version_version = group_version.version
```

---


### Service_role_for_account

ServiceRoleForAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_arn` | String | The ARN of the role which is associated with the account. |
| `associated_at` | String | The time when the service role was associated with the account. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_role_for_account outputs
service_role_for_account_id = service_role_for_account.id
service_role_for_account_role_arn = service_role_for_account.role_arn
service_role_for_account_associated_at = service_role_for_account.associated_at
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple connectivity_info resources
connectivity_info_0 = provider.greengrass.Connectivity_info {
    thing_name = "value-0"
}
connectivity_info_1 = provider.greengrass.Connectivity_info {
    thing_name = "value-1"
}
connectivity_info_2 = provider.greengrass.Connectivity_info {
    thing_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    connectivity_info = provider.greengrass.Connectivity_info {
        thing_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Greengrass Documentation](https://docs.aws.amazon.com/greengrass/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
