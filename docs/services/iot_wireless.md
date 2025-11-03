# Iot_wireless Service



**Resources**: 29

---

## Overview

The iot_wireless service provides access to 29 resource types:

- [Resource_log_level](#resource_log_level) [CR]
- [Metrics](#metrics) [R]
- [Partner_account](#partner_account) [RU]
- [Fuota_task](#fuota_task) [CRUD]
- [Wireless_gateway_statistics](#wireless_gateway_statistics) [R]
- [Event_configuration_by_resource_types](#event_configuration_by_resource_types) [RU]
- [Metric_configuration](#metric_configuration) [RU]
- [Service_profile](#service_profile) [CRD]
- [Wireless_gateway_task](#wireless_gateway_task) [CRD]
- [Multicast_group_session](#multicast_group_session) [R]
- [Device_profile](#device_profile) [CRD]
- [Wireless_gateway_firmware_information](#wireless_gateway_firmware_information) [R]
- [Wireless_gateway_task_definition](#wireless_gateway_task_definition) [CRD]
- [Destination](#destination) [CRUD]
- [Log_levels_by_resource_types](#log_levels_by_resource_types) [RU]
- [Position](#position) [RU]
- [Service_endpoint](#service_endpoint) [R]
- [Wireless_gateway_certificate](#wireless_gateway_certificate) [R]
- [Resource_event_configuration](#resource_event_configuration) [RU]
- [Position_estimate](#position_estimate) [R]
- [Resource_position](#resource_position) [RU]
- [Wireless_device_statistics](#wireless_device_statistics) [R]
- [Position_configuration](#position_configuration) [CR]
- [Network_analyzer_configuration](#network_analyzer_configuration) [CRUD]
- [Wireless_device](#wireless_device) [CRUD]
- [Wireless_device_import_task](#wireless_device_import_task) [RUD]
- [Multicast_group](#multicast_group) [CRUD]
- [Wireless_gateway](#wireless_gateway) [CRUD]
- [Queued_messages](#queued_messages) [D]

---

## Resources


### Resource_log_level

ResourceLogLevel resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `log_level` | String | ✅ |  |
| `resource_identifier` | String | ✅ |  |
| `resource_type` | String | ✅ | <p>The type of resource, which can be <code>WirelessDevice</code>,
                <code>WirelessGateway</code>, or <code>FuotaTask</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_level` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_log_level
resource_log_level = provider.iot_wireless.Resource_log_level {
    log_level = "value"  # Required field
    resource_identifier = "value"  # Required field
    resource_type = "value"  # <p>The type of resource, which can be <code>WirelessDevice</code>,
                <code>WirelessGateway</code>, or <code>FuotaTask</code>.</p>
}

# Access resource_log_level outputs
resource_log_level_id = resource_log_level.id
resource_log_level_log_level = resource_log_level.log_level
```

---


### Metrics

Metrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summary_metric_query_results` | Vec<String> | <p>The list of summary metrics that were retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metrics outputs
metrics_id = metrics.id
metrics_summary_metric_query_results = metrics.summary_metric_query_results
```

---


### Partner_account

PartnerAccount resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partner_type` | String | ✅ | <p>The partner type.</p> |
| `partner_account_id` | String | ✅ | <p>The ID of the partner account to update.</p> |
| `sidewalk` | String | ✅ | <p>The Sidewalk account credentials.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_linked` | bool | <p>Whether the partner account is linked to the AWS account.</p> |
| `sidewalk` | String | <p>The Sidewalk account credentials.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access partner_account outputs
partner_account_id = partner_account.id
partner_account_account_linked = partner_account.account_linked
partner_account_sidewalk = partner_account.sidewalk
```

---


### Fuota_task

FuotaTask resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  |  |
| `fragment_interval_ms` | i64 |  |  |
| `descriptor` | String |  |  |
| `fragment_size_bytes` | i64 |  |  |
| `client_request_token` | String |  |  |
| `redundancy_percent` | i64 |  |  |
| `firmware_update_role` | String | ✅ |  |
| `name` | String |  |  |
| `lo_ra_wan` | String |  |  |
| `firmware_update_image` | String | ✅ |  |
| `tags` | Vec<String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String |  |
| `id` | String |  |
| `created_at` | String |  |
| `redundancy_percent` | i64 |  |
| `arn` | String |  |
| `descriptor` | String |  |
| `status` | String |  |
| `lo_ra_wan` | String |  |
| `firmware_update_image` | String |  |
| `firmware_update_role` | String |  |
| `name` | String |  |
| `fragment_size_bytes` | i64 |  |
| `fragment_interval_ms` | i64 |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fuota_task
fuota_task = provider.iot_wireless.Fuota_task {
    firmware_update_role = "value"  # Required field
    firmware_update_image = "value"  # Required field
}

# Access fuota_task outputs
fuota_task_id = fuota_task.id
fuota_task_description = fuota_task.description
fuota_task_id = fuota_task.id
fuota_task_created_at = fuota_task.created_at
fuota_task_redundancy_percent = fuota_task.redundancy_percent
fuota_task_arn = fuota_task.arn
fuota_task_descriptor = fuota_task.descriptor
fuota_task_status = fuota_task.status
fuota_task_lo_ra_wan = fuota_task.lo_ra_wan
fuota_task_firmware_update_image = fuota_task.firmware_update_image
fuota_task_firmware_update_role = fuota_task.firmware_update_role
fuota_task_name = fuota_task.name
fuota_task_fragment_size_bytes = fuota_task.fragment_size_bytes
fuota_task_fragment_interval_ms = fuota_task.fragment_interval_ms
```

---


### Wireless_gateway_statistics

WirelessGatewayStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection_status` | String | <p>The connection status of the wireless gateway.</p> |
| `wireless_gateway_id` | String | <p>The ID of the wireless gateway.</p> |
| `last_uplink_received_at` | String | <p>The date and time when the most recent uplink was received.</p>
         <note>
            <p>This value is only valid for 3 months.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access wireless_gateway_statistics outputs
wireless_gateway_statistics_id = wireless_gateway_statistics.id
wireless_gateway_statistics_connection_status = wireless_gateway_statistics.connection_status
wireless_gateway_statistics_wireless_gateway_id = wireless_gateway_statistics.wireless_gateway_id
wireless_gateway_statistics_last_uplink_received_at = wireless_gateway_statistics.last_uplink_received_at
```

---


### Event_configuration_by_resource_types

EventConfigurationByResourceTypes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_registration_state` | String |  | <p>Device registration state resource type event configuration object for enabling and
            disabling wireless gateway topic.</p> |
| `message_delivery_status` | String |  | <p>Message delivery status resource type event configuration object for enabling and
            disabling wireless device topic.</p> |
| `join` | String |  | <p>Join resource type event configuration object for enabling and disabling wireless
            device topic.</p> |
| `connection_status` | String |  | <p>Connection status resource type event configuration object for enabling and disabling
            wireless gateway topic.</p> |
| `proximity` | String |  | <p>Proximity resource type event configuration object for enabling and disabling wireless
            gateway topic.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_registration_state` | String | <p>Resource type event configuration for the device registration state event.</p> |
| `proximity` | String | <p>Resource type event configuration for the proximity event.</p> |
| `join` | String | <p>Resource type event configuration for the join event.</p> |
| `connection_status` | String | <p>Resource type event configuration for the connection status event.</p> |
| `message_delivery_status` | String | <p>Resource type event configuration object for the message delivery status event.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_configuration_by_resource_types outputs
event_configuration_by_resource_types_id = event_configuration_by_resource_types.id
event_configuration_by_resource_types_device_registration_state = event_configuration_by_resource_types.device_registration_state
event_configuration_by_resource_types_proximity = event_configuration_by_resource_types.proximity
event_configuration_by_resource_types_join = event_configuration_by_resource_types.join
event_configuration_by_resource_types_connection_status = event_configuration_by_resource_types.connection_status
event_configuration_by_resource_types_message_delivery_status = event_configuration_by_resource_types.message_delivery_status
```

---


### Metric_configuration

MetricConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `summary_metric` | String |  | <p>The value to be used to set summary metric configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summary_metric` | String | <p>The configuration status of the AWS account for summary metric aggregation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metric_configuration outputs
metric_configuration_id = metric_configuration.id
metric_configuration_summary_metric = metric_configuration.summary_metric
```

---


### Service_profile

ServiceProfile resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the new resource.</p>
         <note>
            <p>The following special characters aren't accepted: <code><>^#~$</code>
            </p>
         </note> |
| `tags` | Vec<String> |  | <p>The tags to attach to the new service profile. Tags are metadata that you can use to
            manage a resource.</p> |
| `lo_ra_wan` | String |  | <p>The service profile information to use to create the service profile.</p> |
| `client_request_token` | String |  | <p>Each resource must have a unique client request token. The client token is used to
            implement idempotency. It ensures that the request completes no more than one time. If
            you retry a request with the same token and the same parameters, the request will
            complete successfully. However, if you try to create a new resource using the same token
            but different parameters, an HTTP 409 conflict occurs. If you omit this value, AWS SDKs
            will automatically generate a unique client request. For more information about
            idempotency, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency in Amazon
                EC2 API requests</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the resource.</p> |
| `id` | String | <p>The ID of the service profile.</p> |
| `lo_ra_wan` | String | <p>Information about the service profile.</p> |
| `arn` | String | <p>The Amazon Resource Name of the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service_profile
service_profile = provider.iot_wireless.Service_profile {
}

# Access service_profile outputs
service_profile_id = service_profile.id
service_profile_name = service_profile.name
service_profile_id = service_profile.id
service_profile_lo_ra_wan = service_profile.lo_ra_wan
service_profile_arn = service_profile.arn
```

---


### Wireless_gateway_task

WirelessGatewayTask resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The ID of the resource to update.</p> |
| `wireless_gateway_task_definition_id` | String | ✅ | <p>The ID of the WirelessGatewayTaskDefinition.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `wireless_gateway_id` | String | <p>The ID of the wireless gateway.</p> |
| `task_created_at` | String | <p>The date and time when the task was created.</p> |
| `status` | String | <p>The status of the request.</p> |
| `last_uplink_received_at` | String | <p>The date and time when the most recent uplink was received.</p>
         <note>
            <p>This value is only valid for 3 months.</p>
         </note> |
| `wireless_gateway_task_definition_id` | String | <p>The ID of the WirelessGatewayTask.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create wireless_gateway_task
wireless_gateway_task = provider.iot_wireless.Wireless_gateway_task {
    id = "value"  # <p>The ID of the resource to update.</p>
    wireless_gateway_task_definition_id = "value"  # <p>The ID of the WirelessGatewayTaskDefinition.</p>
}

# Access wireless_gateway_task outputs
wireless_gateway_task_id = wireless_gateway_task.id
wireless_gateway_task_wireless_gateway_id = wireless_gateway_task.wireless_gateway_id
wireless_gateway_task_task_created_at = wireless_gateway_task.task_created_at
wireless_gateway_task_status = wireless_gateway_task.status
wireless_gateway_task_last_uplink_received_at = wireless_gateway_task.last_uplink_received_at
wireless_gateway_task_wireless_gateway_task_definition_id = wireless_gateway_task.wireless_gateway_task_definition_id
```

---


### Multicast_group_session

MulticastGroupSession resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lo_ra_wan` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access multicast_group_session outputs
multicast_group_session_id = multicast_group_session.id
multicast_group_session_lo_ra_wan = multicast_group_session.lo_ra_wan
```

---


### Device_profile

DeviceProfile resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lo_ra_wan` | String |  | <p>The device profile information to use to create the device profile.</p> |
| `client_request_token` | String |  | <p>Each resource must have a unique client request token. The client token is used to
            implement idempotency. It ensures that the request completes no more than one time. If
            you retry a request with the same token and the same parameters, the request will
            complete successfully. However, if you try to create a new resource using the same token
            but different parameters, an HTTP 409 conflict occurs. If you omit this value, AWS SDKs
            will automatically generate a unique client request. For more information about
            idempotency, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency in Amazon
                EC2 API requests</a>.</p> |
| `tags` | Vec<String> |  | <p>The tags to attach to the new device profile. Tags are metadata that you can use to
            manage a resource.</p> |
| `sidewalk` | String |  | <p>The Sidewalk-related information for creating the Sidewalk device profile.</p> |
| `name` | String |  | <p>The name of the new resource.</p>
         <note>
            <p>The following special characters aren't accepted: <code><>^#~$</code>
            </p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the resource.</p> |
| `sidewalk` | String | <p>Information about the Sidewalk parameters in the device profile.</p> |
| `lo_ra_wan` | String | <p>Information about the device profile.</p> |
| `arn` | String | <p>The Amazon Resource Name of the resource.</p> |
| `id` | String | <p>The ID of the device profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create device_profile
device_profile = provider.iot_wireless.Device_profile {
}

# Access device_profile outputs
device_profile_id = device_profile.id
device_profile_name = device_profile.name
device_profile_sidewalk = device_profile.sidewalk
device_profile_lo_ra_wan = device_profile.lo_ra_wan
device_profile_arn = device_profile.arn
device_profile_id = device_profile.id
```

---


### Wireless_gateway_firmware_information

WirelessGatewayFirmwareInformation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lo_ra_wan` | String | <p>Information about the wireless gateway's firmware.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access wireless_gateway_firmware_information outputs
wireless_gateway_firmware_information_id = wireless_gateway_firmware_information.id
wireless_gateway_firmware_information_lo_ra_wan = wireless_gateway_firmware_information.lo_ra_wan
```

---


### Wireless_gateway_task_definition

WirelessGatewayTaskDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags to attach to the specified resource. Tags are metadata that you can use to
            manage a resource.</p> |
| `auto_create_tasks` | bool | ✅ | <p>Whether to automatically create tasks using this task definition for all gateways with
            the specified current version. If <code>false</code>, the task must me created by
            calling <code>CreateWirelessGatewayTask</code>.</p> |
| `name` | String |  | <p>The name of the new resource.</p> |
| `update` | String |  | <p>Information about the gateways to update.</p> |
| `client_request_token` | String |  | <p>Each resource must have a unique client request token. The client token is used to
            implement idempotency. It ensures that the request completes no more than one time. If
            you retry a request with the same token and the same parameters, the request will
            complete successfully. However, if you try to create a new resource using the same token
            but different parameters, an HTTP 409 conflict occurs. If you omit this value, AWS SDKs
            will automatically generate a unique client request. For more information about
            idempotency, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency in Amazon
                EC2 API requests</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the resource.</p> |
| `update` | String | <p>Information about the gateways to update.</p> |
| `auto_create_tasks` | bool | <p>Whether to automatically create tasks using this task definition for all gateways with
            the specified current version. If <code>false</code>, the task must me created by
            calling <code>CreateWirelessGatewayTask</code>.</p> |
| `arn` | String | <p>The Amazon Resource Name of the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create wireless_gateway_task_definition
wireless_gateway_task_definition = provider.iot_wireless.Wireless_gateway_task_definition {
    auto_create_tasks = "value"  # <p>Whether to automatically create tasks using this task definition for all gateways with
            the specified current version. If <code>false</code>, the task must me created by
            calling <code>CreateWirelessGatewayTask</code>.</p>
}

# Access wireless_gateway_task_definition outputs
wireless_gateway_task_definition_id = wireless_gateway_task_definition.id
wireless_gateway_task_definition_name = wireless_gateway_task_definition.name
wireless_gateway_task_definition_update = wireless_gateway_task_definition.update
wireless_gateway_task_definition_auto_create_tasks = wireless_gateway_task_definition.auto_create_tasks
wireless_gateway_task_definition_arn = wireless_gateway_task_definition.arn
```

---


### Destination

Destination resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the new resource.</p> |
| `expression_type` | String | ✅ | <p>The type of value in <code>Expression</code>.</p> |
| `expression` | String | ✅ | <p>The rule name or topic rule to send messages to.</p> |
| `client_request_token` | String |  | <p>Each resource must have a unique client request token. The client token is used to
            implement idempotency. It ensures that the request completes no more than one time. If
            you retry a request with the same token and the same parameters, the request will
            complete successfully. However, if you try to create a new resource using the same token
            but different parameters, an HTTP 409 conflict occurs. If you omit this value, AWS SDKs
            will automatically generate a unique client request. For more information about
            idempotency, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency in Amazon
                EC2 API requests</a>.</p> |
| `description` | String |  | <p>The description of the new resource.</p> |
| `tags` | Vec<String> |  | <p>The tags to attach to the new destination. Tags are metadata that you can use to
            manage a resource.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the IAM Role that authorizes the destination.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_arn` | String | <p>The ARN of the IAM Role that authorizes the destination.</p> |
| `arn` | String | <p>The Amazon Resource Name of the resource.</p> |
| `description` | String | <p>The description of the resource.</p> |
| `expression` | String | <p>The rule name or topic rule to send messages to.</p> |
| `expression_type` | String | <p>The type of value in <code>Expression</code>.</p> |
| `name` | String | <p>The name of the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create destination
destination = provider.iot_wireless.Destination {
    name = "value"  # <p>The name of the new resource.</p>
    expression_type = "value"  # <p>The type of value in <code>Expression</code>.</p>
    expression = "value"  # <p>The rule name or topic rule to send messages to.</p>
    role_arn = "value"  # <p>The ARN of the IAM Role that authorizes the destination.</p>
}

# Access destination outputs
destination_id = destination.id
destination_role_arn = destination.role_arn
destination_arn = destination.arn
destination_description = destination.description
destination_expression = destination.expression
destination_expression_type = destination.expression_type
destination_name = destination.name
```

---


### Log_levels_by_resource_types

LogLevelsByResourceTypes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `wireless_device_log_options` | Vec<String> |  |  |
| `wireless_gateway_log_options` | Vec<String> |  |  |
| `default_log_level` | String |  |  |
| `fuota_task_log_options` | Vec<String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_log_level` | String |  |
| `wireless_gateway_log_options` | Vec<String> |  |
| `fuota_task_log_options` | Vec<String> |  |
| `wireless_device_log_options` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access log_levels_by_resource_types outputs
log_levels_by_resource_types_id = log_levels_by_resource_types.id
log_levels_by_resource_types_default_log_level = log_levels_by_resource_types.default_log_level
log_levels_by_resource_types_wireless_gateway_log_options = log_levels_by_resource_types.wireless_gateway_log_options
log_levels_by_resource_types_fuota_task_log_options = log_levels_by_resource_types.fuota_task_log_options
log_levels_by_resource_types_wireless_device_log_options = log_levels_by_resource_types.wireless_device_log_options
```

---


### Position

Position resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_identifier` | String | ✅ | <p>Resource identifier of the resource for which position is updated.</p> |
| `position` | Vec<String> | ✅ | <p>The position information of the resource.</p> |
| `resource_type` | String | ✅ | <p>Resource type of the resource for which position is updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `solver_provider` | String | <p>The vendor of the positioning solver.</p> |
| `accuracy` | String | <p>The accuracy of the estimated position in meters. An empty value indicates that no
            position data is available. A value of ‘0.0’ value indicates that position data is
            available. This data corresponds to the position information that you specified instead
            of the position computed by solver.</p> |
| `solver_type` | String | <p>The type of solver used to identify the position of the resource.</p> |
| `position` | Vec<String> | <p>The position information of the resource.</p> |
| `solver_version` | String | <p>The version of the positioning solver.</p> |
| `timestamp` | String | <p>The timestamp at which the device's position was determined.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access position outputs
position_id = position.id
position_solver_provider = position.solver_provider
position_accuracy = position.accuracy
position_solver_type = position.solver_type
position_position = position.position
position_solver_version = position.solver_version
position_timestamp = position.timestamp
```

---


### Service_endpoint

ServiceEndpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `server_trust` | String | <p>The Root CA of the server trust certificate.</p> |
| `service_type` | String | <p>The endpoint's service type.</p> |
| `service_endpoint` | String | <p>The service endpoint value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_endpoint outputs
service_endpoint_id = service_endpoint.id
service_endpoint_server_trust = service_endpoint.server_trust
service_endpoint_service_type = service_endpoint.service_type
service_endpoint_service_endpoint = service_endpoint.service_endpoint
```

---


### Wireless_gateway_certificate

WirelessGatewayCertificate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lo_ra_wan_network_server_certificate_id` | String | <p>The ID of the certificate that is associated with the wireless gateway and used for
            the LoRaWANNetworkServer endpoint.</p> |
| `iot_certificate_id` | String | <p>The ID of the certificate associated with the wireless gateway.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access wireless_gateway_certificate outputs
wireless_gateway_certificate_id = wireless_gateway_certificate.id
wireless_gateway_certificate_lo_ra_wan_network_server_certificate_id = wireless_gateway_certificate.lo_ra_wan_network_server_certificate_id
wireless_gateway_certificate_iot_certificate_id = wireless_gateway_certificate.iot_certificate_id
```

---


### Resource_event_configuration

ResourceEventConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `proximity` | String |  | <p>Event configuration for the proximity event.</p> |
| `join` | String |  | <p>Event configuration for the join event.</p> |
| `identifier_type` | String | ✅ | <p>Identifier type of the particular resource identifier for event configuration.</p> |
| `message_delivery_status` | String |  | <p>Event configuration for the message delivery status event.</p> |
| `identifier` | String | ✅ | <p>Resource identifier to opt in for event messaging.</p> |
| `device_registration_state` | String |  | <p>Event configuration for the device registration state event.</p> |
| `connection_status` | String |  | <p>Event configuration for the connection status event.</p> |
| `partner_type` | String |  | <p>Partner type of the resource if the identifier type is
            <code>PartnerAccountId</code>
         </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `join` | String | <p>Event configuration for the join event.</p> |
| `device_registration_state` | String | <p>Event configuration for the device registration state event.</p> |
| `connection_status` | String | <p>Event configuration for the connection status event.</p> |
| `proximity` | String | <p>Event configuration for the proximity event.</p> |
| `message_delivery_status` | String | <p>Event configuration for the message delivery status event.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_event_configuration outputs
resource_event_configuration_id = resource_event_configuration.id
resource_event_configuration_join = resource_event_configuration.join
resource_event_configuration_device_registration_state = resource_event_configuration.device_registration_state
resource_event_configuration_connection_status = resource_event_configuration.connection_status
resource_event_configuration_proximity = resource_event_configuration.proximity
resource_event_configuration_message_delivery_status = resource_event_configuration.message_delivery_status
```

---


### Position_estimate

PositionEstimate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `geo_json_payload` | String | <p>The position information of the resource, displayed as a JSON payload. The payload is
            of type blob and uses the <a href="https://geojson.org/">GeoJSON</a> format,
            which a format that's used to encode geographic data structures. A sample payload
            contains the timestamp information, the WGS84 coordinates of the location, and the
            accuracy and confidence level. For more information and examples, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/location-resolve-console.html">Resolve device location (console)</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access position_estimate outputs
position_estimate_id = position_estimate.id
position_estimate_geo_json_payload = position_estimate.geo_json_payload
```

---


### Resource_position

ResourcePosition resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_identifier` | String | ✅ | <p>The identifier of the resource for which position information is updated. It can be
            the wireless device ID or the wireless gateway ID, depending on the resource
            type.</p> |
| `resource_type` | String | ✅ | <p>The type of resource for which position information is updated, which can be a
            wireless device or a wireless gateway.</p> |
| `geo_json_payload` | String |  | <p>The position information of the resource, displayed as a JSON payload. The payload
            uses the GeoJSON format, which a format that's used to encode geographic data
            structures. For more information, see <a href="https://geojson.org/">GeoJSON</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `geo_json_payload` | String | <p>The position information of the resource, displayed as a JSON payload. The payload
            uses the GeoJSON format, which a format that's used to encode geographic data
            structures. For more information, see <a href="https://geojson.org/">GeoJSON</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_position outputs
resource_position_id = resource_position.id
resource_position_geo_json_payload = resource_position.geo_json_payload
```

---


### Wireless_device_statistics

WirelessDeviceStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lo_ra_wan` | String | <p>Information about the wireless device's operations.</p> |
| `sidewalk` | String | <p>MetaData for Sidewalk device.</p> |
| `wireless_device_id` | String | <p>The ID of the wireless device.</p> |
| `last_uplink_received_at` | String | <p>The date and time when the most recent uplink was received.</p>
         <note>
            <p>This value is only valid for 3 months.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access wireless_device_statistics outputs
wireless_device_statistics_id = wireless_device_statistics.id
wireless_device_statistics_lo_ra_wan = wireless_device_statistics.lo_ra_wan
wireless_device_statistics_sidewalk = wireless_device_statistics.sidewalk
wireless_device_statistics_wireless_device_id = wireless_device_statistics.wireless_device_id
wireless_device_statistics_last_uplink_received_at = wireless_device_statistics.last_uplink_received_at
```

---


### Position_configuration

PositionConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination` | String |  | <p>The position data destination that describes the AWS IoT rule that processes the
            device's position data for use by AWS IoT Core for LoRaWAN.</p> |
| `solvers` | String |  | <p>The positioning solvers used to update the position configuration of the
            resource.</p> |
| `resource_identifier` | String | ✅ | <p>Resource identifier used to update the position configuration.</p> |
| `resource_type` | String | ✅ | <p>Resource type of the resource for which you want to update the position
            configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination` | String | <p>The position data destination that describes the AWS IoT rule that processes the
            device's position data for use by AWS IoT Core for LoRaWAN.</p> |
| `solvers` | String | <p>The wrapper for the solver configuration details object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create position_configuration
position_configuration = provider.iot_wireless.Position_configuration {
    resource_identifier = "value"  # <p>Resource identifier used to update the position configuration.</p>
    resource_type = "value"  # <p>Resource type of the resource for which you want to update the position
            configuration.</p>
}

# Access position_configuration outputs
position_configuration_id = position_configuration.id
position_configuration_destination = position_configuration.destination
position_configuration_solvers = position_configuration.solvers
```

---


### Network_analyzer_configuration

NetworkAnalyzerConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `multicast_groups` | Vec<String> |  | <p>Multicast Group resources to add to the network analyzer configruation. Provide the
                <code>MulticastGroupId</code> of the resource to add in the input array.</p> |
| `wireless_gateways` | Vec<String> |  | <p>Wireless gateway resources to add to the network analyzer configuration. Provide the
                <code>WirelessGatewayId</code> of the resource to add in the input array.</p> |
| `client_request_token` | String |  |  |
| `name` | String | ✅ |  |
| `trace_content` | String |  |  |
| `wireless_devices` | Vec<String> |  | <p>Wireless device resources to add to the network analyzer configuration. Provide the
                <code>WirelessDeviceId</code> of the resource to add in the input array.</p> |
| `description` | String |  |  |
| `tags` | Vec<String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String |  |
| `multicast_groups` | Vec<String> | <p>List of multicast group resources that have been added to the network analyzer
            configuration.</p> |
| `wireless_devices` | Vec<String> | <p>List of wireless device resources that have been added to the network analyzer
            configuration.</p> |
| `arn` | String | <p>The Amazon Resource Name of the new resource.</p> |
| `trace_content` | String |  |
| `wireless_gateways` | Vec<String> | <p>List of wireless gateway resources that have been added to the network analyzer
            configuration.</p> |
| `description` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_analyzer_configuration
network_analyzer_configuration = provider.iot_wireless.Network_analyzer_configuration {
    name = "value"  # Required field
}

# Access network_analyzer_configuration outputs
network_analyzer_configuration_id = network_analyzer_configuration.id
network_analyzer_configuration_name = network_analyzer_configuration.name
network_analyzer_configuration_multicast_groups = network_analyzer_configuration.multicast_groups
network_analyzer_configuration_wireless_devices = network_analyzer_configuration.wireless_devices
network_analyzer_configuration_arn = network_analyzer_configuration.arn
network_analyzer_configuration_trace_content = network_analyzer_configuration.trace_content
network_analyzer_configuration_wireless_gateways = network_analyzer_configuration.wireless_gateways
network_analyzer_configuration_description = network_analyzer_configuration.description
```

---


### Wireless_device

WirelessDevice resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the new resource.</p> |
| `client_request_token` | String |  | <p>Each resource must have a unique client request token. The client token is used to
            implement idempotency. It ensures that the request completes no more than one time. If
            you retry a request with the same token and the same parameters, the request will
            complete successfully. However, if you try to create a new resource using the same token
            but different parameters, an HTTP 409 conflict occurs. If you omit this value, AWS SDKs
            will automatically generate a unique client request. For more information about
            idempotency, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency in Amazon
                EC2 API requests</a>.</p> |
| `sidewalk` | String |  | <p>The device configuration information to use to create the Sidewalk device.</p> |
| `type` | String | ✅ | <p>The wireless device type.</p> |
| `destination_name` | String | ✅ | <p>The name of the destination to assign to the new wireless device.</p> |
| `tags` | Vec<String> |  | <p>The tags to attach to the new wireless device. Tags are metadata that you can use to
            manage a resource.</p> |
| `name` | String |  | <p>The name of the new resource.</p>
         <note>
            <p>The following special characters aren't accepted: <code><>^#~$</code>
            </p>
         </note> |
| `lo_ra_wan` | String |  | <p>The device configuration information to use to create the wireless device.</p> |
| `positioning` | String |  | <p>FPort values for the GNSS, stream, and ClockSync functions of the positioning
            information.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lo_ra_wan` | String | <p>Information about the wireless device.</p> |
| `positioning` | String | <p>FPort values for the GNSS, stream, and ClockSync functions of the positioning
            information.</p> |
| `thing_name` | String | <p>The name of the thing associated with the wireless device. The value is empty if a
            thing isn't associated with the device.</p> |
| `type` | String | <p>The wireless device type.</p> |
| `arn` | String | <p>The Amazon Resource Name of the resource.</p> |
| `sidewalk` | String | <p>Sidewalk device object.</p> |
| `thing_arn` | String | <p>The ARN of the thing associated with the wireless device.</p> |
| `description` | String | <p>The description of the resource.</p> |
| `id` | String | <p>The ID of the wireless device.</p> |
| `destination_name` | String | <p>The name of the destination to which the device is assigned.</p> |
| `name` | String | <p>The name of the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create wireless_device
wireless_device = provider.iot_wireless.Wireless_device {
    type = "value"  # <p>The wireless device type.</p>
    destination_name = "value"  # <p>The name of the destination to assign to the new wireless device.</p>
}

# Access wireless_device outputs
wireless_device_id = wireless_device.id
wireless_device_lo_ra_wan = wireless_device.lo_ra_wan
wireless_device_positioning = wireless_device.positioning
wireless_device_thing_name = wireless_device.thing_name
wireless_device_type = wireless_device.type
wireless_device_arn = wireless_device.arn
wireless_device_sidewalk = wireless_device.sidewalk
wireless_device_thing_arn = wireless_device.thing_arn
wireless_device_description = wireless_device.description
wireless_device_id = wireless_device.id
wireless_device_destination_name = wireless_device.destination_name
wireless_device_name = wireless_device.name
```

---


### Wireless_device_import_task

WirelessDeviceImportTask resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The identifier of the import task to be updated.</p> |
| `sidewalk` | String | ✅ | <p>The Sidewalk-related parameters of the import task to be updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_reason` | String | <p>The reason for the provided status information, such as a validation error that causes
            the import task to fail.</p> |
| `id` | String | <p>The identifier of the import task for which information is retrieved.</p> |
| `onboarded_imported_device_count` | i64 | <p>The number of devices in the import task that have been onboarded to the import
            task.</p> |
| `arn` | String | <p>The ARN (Amazon Resource Name) of the import task.</p> |
| `sidewalk` | String | <p>The Sidewalk-related information about an import task.</p> |
| `creation_time` | String | <p>The time at which the import task was created.</p> |
| `destination_name` | String | <p>The name of the destination that's assigned to the wireless devices in the import
            task.</p> |
| `pending_imported_device_count` | i64 | <p>The number of devices in the import task that are waiting in the import task queue to
            be onboarded.</p> |
| `status` | String | <p>The import task status.</p> |
| `initialized_imported_device_count` | i64 | <p>The number of devices in the import task that are waiting for the control log to start
            processing.</p> |
| `failed_imported_device_count` | i64 | <p>The number of devices in the import task that failed to onboard to the import
            task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access wireless_device_import_task outputs
wireless_device_import_task_id = wireless_device_import_task.id
wireless_device_import_task_status_reason = wireless_device_import_task.status_reason
wireless_device_import_task_id = wireless_device_import_task.id
wireless_device_import_task_onboarded_imported_device_count = wireless_device_import_task.onboarded_imported_device_count
wireless_device_import_task_arn = wireless_device_import_task.arn
wireless_device_import_task_sidewalk = wireless_device_import_task.sidewalk
wireless_device_import_task_creation_time = wireless_device_import_task.creation_time
wireless_device_import_task_destination_name = wireless_device_import_task.destination_name
wireless_device_import_task_pending_imported_device_count = wireless_device_import_task.pending_imported_device_count
wireless_device_import_task_status = wireless_device_import_task.status
wireless_device_import_task_initialized_imported_device_count = wireless_device_import_task.initialized_imported_device_count
wireless_device_import_task_failed_imported_device_count = wireless_device_import_task.failed_imported_device_count
```

---


### Multicast_group

MulticastGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lo_ra_wan` | String | ✅ |  |
| `tags` | Vec<String> |  |  |
| `name` | String |  |  |
| `client_request_token` | String |  | <p>Each resource must have a unique client request token. The client token is used to
            implement idempotency. It ensures that the request completes no more than one time. If
            you retry a request with the same token and the same parameters, the request will
            complete successfully. However, if you try to create a new resource using the same token
            but different parameters, an HTTP 409 conflict occurs. If you omit this value, AWS SDKs
            will automatically generate a unique client request. For more information about
            idempotency, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency in Amazon
                EC2 API requests</a>.</p> |
| `description` | String |  | <p>The description of the multicast group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String |  |
| `lo_ra_wan` | String |  |
| `created_at` | String |  |
| `arn` | String |  |
| `name` | String |  |
| `description` | String |  |
| `status` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multicast_group
multicast_group = provider.iot_wireless.Multicast_group {
    lo_ra_wan = "value"  # Required field
}

# Access multicast_group outputs
multicast_group_id = multicast_group.id
multicast_group_id = multicast_group.id
multicast_group_lo_ra_wan = multicast_group.lo_ra_wan
multicast_group_created_at = multicast_group.created_at
multicast_group_arn = multicast_group.arn
multicast_group_name = multicast_group.name
multicast_group_description = multicast_group.description
multicast_group_status = multicast_group.status
```

---


### Wireless_gateway

WirelessGateway resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags to attach to the new wireless gateway. Tags are metadata that you can use to
            manage a resource.</p> |
| `name` | String |  | <p>The name of the new resource.</p>
         <note>
            <p>The following special characters aren't accepted: <code><>^#~$</code>
            </p>
         </note> |
| `description` | String |  | <p>The description of the new resource.</p> |
| `lo_ra_wan` | String | ✅ | <p>The gateway configuration information to use to create the wireless gateway.</p> |
| `client_request_token` | String |  | <p>Each resource must have a unique client request token. The client token is used to
            implement idempotency. It ensures that the request completes no more than one time. If
            you retry a request with the same token and the same parameters, the request will
            complete successfully. However, if you try to create a new resource using the same token
            but different parameters, an HTTP 409 conflict occurs. If you omit this value, AWS SDKs
            will automatically generate a unique client request. For more information about
            idempotency, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency in Amazon
                EC2 API requests</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The Amazon Resource Name of the resource.</p> |
| `description` | String | <p>The description of the resource.</p> |
| `name` | String | <p>The name of the resource.</p> |
| `thing_name` | String | <p>The name of the thing associated with the wireless gateway. The value is empty if a
            thing isn't associated with the gateway.</p> |
| `lo_ra_wan` | String | <p>Information about the wireless gateway.</p> |
| `thing_arn` | String | <p>The ARN of the thing associated with the wireless gateway.</p> |
| `id` | String | <p>The ID of the wireless gateway.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create wireless_gateway
wireless_gateway = provider.iot_wireless.Wireless_gateway {
    lo_ra_wan = "value"  # <p>The gateway configuration information to use to create the wireless gateway.</p>
}

# Access wireless_gateway outputs
wireless_gateway_id = wireless_gateway.id
wireless_gateway_arn = wireless_gateway.arn
wireless_gateway_description = wireless_gateway.description
wireless_gateway_name = wireless_gateway.name
wireless_gateway_thing_name = wireless_gateway.thing_name
wireless_gateway_lo_ra_wan = wireless_gateway.lo_ra_wan
wireless_gateway_thing_arn = wireless_gateway.thing_arn
wireless_gateway_id = wireless_gateway.id
```

---


### Queued_messages

QueuedMessages resource

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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_log_level resources
resource_log_level_0 = provider.iot_wireless.Resource_log_level {
    log_level = "value-0"
    resource_identifier = "value-0"
    resource_type = "value-0"
}
resource_log_level_1 = provider.iot_wireless.Resource_log_level {
    log_level = "value-1"
    resource_identifier = "value-1"
    resource_type = "value-1"
}
resource_log_level_2 = provider.iot_wireless.Resource_log_level {
    log_level = "value-2"
    resource_identifier = "value-2"
    resource_type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_log_level = provider.iot_wireless.Resource_log_level {
        log_level = "production-value"
        resource_identifier = "production-value"
        resource_type = "production-value"
    }
```

---

## Related Documentation

- [AWS Iot_wireless Documentation](https://docs.aws.amazon.com/iot_wireless/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
