# Medialive Service



**Resources**: 27

---

## Overview

The medialive service provides access to 27 resource types:

- [Partner_input](#partner_input) [C]
- [Event_bridge_rule_template_group](#event_bridge_rule_template_group) [CRUD]
- [Multiplex](#multiplex) [CRUD]
- [Input_device](#input_device) [RU]
- [Input](#input) [CRUD]
- [Input_security_group](#input_security_group) [CRUD]
- [Node_registration_script](#node_registration_script) [C]
- [Network](#network) [CRUD]
- [Event_bridge_rule_template](#event_bridge_rule_template) [CRUD]
- [Schedule](#schedule) [RD]
- [Cloud_watch_alarm_template](#cloud_watch_alarm_template) [CRUD]
- [Node_state](#node_state) [U]
- [Thumbnails](#thumbnails) [R]
- [Cloud_watch_alarm_template_group](#cloud_watch_alarm_template_group) [CRUD]
- [Account_configuration](#account_configuration) [RU]
- [Offering](#offering) [R]
- [Signal_map](#signal_map) [CRD]
- [Cluster](#cluster) [CRUD]
- [Reservation](#reservation) [RUD]
- [Channel_placement_group](#channel_placement_group) [CRUD]
- [Node](#node) [CRUD]
- [Channel](#channel) [CRUD]
- [Tags](#tags) [CD]
- [Sdi_source](#sdi_source) [CRUD]
- [Multiplex_program](#multiplex_program) [CRUD]
- [Channel_class](#channel_class) [U]
- [Input_device_thumbnail](#input_device_thumbnail) [R]

---

## Resources


### Partner_input

PartnerInput resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |
| `request_id` | String |  | Unique identifier of the request to ensure the request is handled
exactly once in case of retries. |
| `input_id` | String | ✅ | Unique ID of the input. |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partner_input
partner_input = provider.medialive.Partner_input {
    input_id = "value"  # Unique ID of the input.
}

```

---


### Event_bridge_rule_template_group

EventBridgeRuleTemplateGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A resource's optional description. |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |
| `name` | String | ✅ | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `tags` | HashMap<String, String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | An eventbridge rule template group's id. AWS provided template groups have ids that start with `aws-` |
| `arn` | String | An eventbridge rule template group's ARN (Amazon Resource Name) |
| `description` | String | A resource's optional description. |
| `modified_at` | String |  |
| `name` | String | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `tags` | HashMap<String, String> |  |
| `created_at` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_bridge_rule_template_group
event_bridge_rule_template_group = provider.medialive.Event_bridge_rule_template_group {
    name = "value"  # A resource's name. Names must be unique within the scope of a resource type in a specific region.
}

# Access event_bridge_rule_template_group outputs
event_bridge_rule_template_group_id = event_bridge_rule_template_group.id
event_bridge_rule_template_group_id = event_bridge_rule_template_group.id
event_bridge_rule_template_group_arn = event_bridge_rule_template_group.arn
event_bridge_rule_template_group_description = event_bridge_rule_template_group.description
event_bridge_rule_template_group_modified_at = event_bridge_rule_template_group.modified_at
event_bridge_rule_template_group_name = event_bridge_rule_template_group.name
event_bridge_rule_template_group_tags = event_bridge_rule_template_group.tags
event_bridge_rule_template_group_created_at = event_bridge_rule_template_group.created_at
```

---


### Multiplex

Multiplex resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_id` | String | ✅ | Unique request ID. This prevents retries from creating multiple
resources. |
| `multiplex_settings` | String | ✅ | Configuration for a multiplex event. |
| `availability_zones` | Vec<String> | ✅ | A list of availability zones for the multiplex. You must specify exactly two. |
| `name` | String | ✅ | Name of multiplex. |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The unique id of the multiplex. |
| `arn` | String | The unique arn of the multiplex. |
| `destinations` | Vec<String> | A list of the multiplex output destinations. |
| `availability_zones` | Vec<String> | A list of availability zones for the multiplex. |
| `program_count` | i64 | The number of programs in the multiplex. |
| `state` | String | The current state of the multiplex. |
| `multiplex_settings` | String | Configuration for a multiplex event. |
| `name` | String | The name of the multiplex. |
| `tags` | HashMap<String, String> | A collection of key-value pairs. |
| `pipelines_running_count` | i64 | The number of currently healthy pipelines. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiplex
multiplex = provider.medialive.Multiplex {
    request_id = "value"  # Unique request ID. This prevents retries from creating multiple
resources.
    multiplex_settings = "value"  # Configuration for a multiplex event.
    availability_zones = "value"  # A list of availability zones for the multiplex. You must specify exactly two.
    name = "value"  # Name of multiplex.
}

# Access multiplex outputs
multiplex_id = multiplex.id
multiplex_id = multiplex.id
multiplex_arn = multiplex.arn
multiplex_destinations = multiplex.destinations
multiplex_availability_zones = multiplex.availability_zones
multiplex_program_count = multiplex.program_count
multiplex_state = multiplex.state
multiplex_multiplex_settings = multiplex.multiplex_settings
multiplex_name = multiplex.name
multiplex_tags = multiplex.tags
multiplex_pipelines_running_count = multiplex.pipelines_running_count
```

---


### Input_device

InputDevice resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uhd_device_settings` | String |  | The settings that you want to apply to the UHD input device. |
| `hd_device_settings` | String |  | The settings that you want to apply to the HD input device. |
| `name` | String |  | The name that you assigned to this input device (not the unique ID). |
| `availability_zone` | String |  | The Availability Zone you want associated with this input device. |
| `input_device_id` | String | ✅ | The unique ID of the input device. For example, hd-123456789abcdef. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection_state` | String | The state of the connection between the input device and AWS. |
| `device_settings_sync_state` | String | The status of the action to synchronize the device configuration. If you change the configuration of the input device (for example, the maximum bitrate), MediaLive sends the new data to the device. The device might not update itself immediately. SYNCED means the device has updated its configuration. SYNCING means that it has not updated its configuration. |
| `availability_zone` | String | The Availability Zone associated with this input device. |
| `tags` | HashMap<String, String> | A collection of key-value pairs. |
| `network_settings` | String | The network settings for the input device. |
| `type` | String | The type of the input device. |
| `arn` | String | The unique ARN of the input device. |
| `id` | String | The unique ID of the input device. |
| `device_update_status` | String | The status of software on the input device. |
| `name` | String | A name that you specify for the input device. |
| `output_type` | String | The output attachment type of the input device. Specifies MEDIACONNECT_FLOW if this device is the source for a MediaConnect flow. Specifies MEDIALIVE_INPUT if this device is the source for a MediaLive input. |
| `mac_address` | String | The network MAC address of the input device. |
| `serial_number` | String | The unique serial number of the input device. |
| `uhd_device_settings` | String | Settings that describe an input device that is type UHD. |
| `hd_device_settings` | String | Settings that describe an input device that is type HD. |
| `medialive_input_arns` | Vec<String> | An array of the ARNs for the MediaLive inputs attached to the device. Returned only if the outputType is MEDIALIVE_INPUT. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access input_device outputs
input_device_id = input_device.id
input_device_connection_state = input_device.connection_state
input_device_device_settings_sync_state = input_device.device_settings_sync_state
input_device_availability_zone = input_device.availability_zone
input_device_tags = input_device.tags
input_device_network_settings = input_device.network_settings
input_device_type = input_device.type
input_device_arn = input_device.arn
input_device_id = input_device.id
input_device_device_update_status = input_device.device_update_status
input_device_name = input_device.name
input_device_output_type = input_device.output_type
input_device_mac_address = input_device.mac_address
input_device_serial_number = input_device.serial_number
input_device_uhd_device_settings = input_device.uhd_device_settings
input_device_hd_device_settings = input_device.hd_device_settings
input_device_medialive_input_arns = input_device.medialive_input_arns
```

---


### Input

Input resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `multicast_settings` | String |  | Multicast Input settings. |
| `smpte2110_receiver_group_settings` | String |  | Include this parameter if the input is a SMPTE 2110 input, to identify the stream sources for this input. |
| `input_security_groups` | Vec<String> |  | A list of security groups referenced by IDs to attach to the input. |
| `name` | String |  | Name of the input. |
| `sources` | Vec<String> |  | The source URLs for a PULL-type input. Every PULL type input needs
exactly two source URLs for redundancy.
Only specify sources for PULL type Inputs. Leave Destinations empty. |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |
| `input_devices` | Vec<String> |  | Settings for the devices. |
| `media_connect_flows` | Vec<String> |  | A list of the MediaConnect Flows that you want to use in this input. You can specify as few as one
Flow and presently, as many as two. The only requirement is when you have more than one is that each Flow is in a
separate Availability Zone as this ensures your EML input is redundant to AZ issues. |
| `input_network_location` | String |  | The location of this input. AWS, for an input existing in the AWS Cloud, On-Prem for
an input in a customer network. |
| `srt_settings` | String |  | The settings associated with an SRT input. |
| `request_id` | String |  | Unique identifier of the request to ensure the request is handled
exactly once in case of retries. |
| `role_arn` | String |  | The Amazon Resource Name (ARN) of the role this input assumes during and after creation. |
| `destinations` | Vec<String> |  | Destination settings for PUSH type inputs. |
| `vpc` | String |  |  |
| `type` | String |  |  |
| `sdi_sources` | Vec<String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attached_channels` | Vec<String> | A list of channel IDs that that input is attached to (currently an input can only be attached to one channel). |
| `state` | String |  |
| `srt_settings` | String | The settings associated with an SRT input. |
| `role_arn` | String | The Amazon Resource Name (ARN) of the role this input assumes during and after creation. |
| `destinations` | Vec<String> | A list of the destinations of the input (PUSH-type). |
| `id` | String | The generated ID of the input (unique for user account, immutable). |
| `security_groups` | Vec<String> | A list of IDs for all the Input Security Groups attached to the input. |
| `type` | String |  |
| `sources` | Vec<String> | A list of the sources of the input (PULL-type). |
| `arn` | String | The Unique ARN of the input (generated, immutable). |
| `input_devices` | Vec<String> | Settings for the input devices. |
| `tags` | HashMap<String, String> | A collection of key-value pairs. |
| `input_network_location` | String | The location of this input. AWS, for an input existing in the AWS Cloud, On-Prem for
an input in a customer network. |
| `sdi_sources` | Vec<String> |  |
| `input_partner_ids` | Vec<String> | A list of IDs for all Inputs which are partners of this one. |
| `media_connect_flows` | Vec<String> | A list of MediaConnect Flows for this input. |
| `name` | String | The user-assigned name (This is a mutable value). |
| `input_class` | String | STANDARD - MediaLive expects two sources to be connected to this input. If the channel is also STANDARD, both sources will be ingested. If the channel is SINGLE_PIPELINE, only the first source will be ingested; the second source will always be ignored, even if the first source fails.
SINGLE_PIPELINE - You can connect only one source to this input. If the ChannelClass is also SINGLE_PIPELINE, this value is valid. If the ChannelClass is STANDARD, this value is not valid because the channel requires two sources in the input. |
| `smpte2110_receiver_group_settings` | String | Include this parameter if the input is a SMPTE 2110 input, to identify the stream sources for this input. |
| `multicast_settings` | String | Multicast Input settings. |
| `input_source_type` | String | Certain pull input sources can be dynamic, meaning that they can have their URL's dynamically changes
during input switch actions. Presently, this functionality only works with MP4_FILE and TS_FILE inputs. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create input
input = provider.medialive.Input {
}

# Access input outputs
input_id = input.id
input_attached_channels = input.attached_channels
input_state = input.state
input_srt_settings = input.srt_settings
input_role_arn = input.role_arn
input_destinations = input.destinations
input_id = input.id
input_security_groups = input.security_groups
input_type = input.type
input_sources = input.sources
input_arn = input.arn
input_input_devices = input.input_devices
input_tags = input.tags
input_input_network_location = input.input_network_location
input_sdi_sources = input.sdi_sources
input_input_partner_ids = input.input_partner_ids
input_media_connect_flows = input.media_connect_flows
input_name = input.name
input_input_class = input.input_class
input_smpte2110_receiver_group_settings = input.smpte2110_receiver_group_settings
input_multicast_settings = input.multicast_settings
input_input_source_type = input.input_source_type
```

---


### Input_security_group

InputSecurityGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `whitelist_rules` | Vec<String> |  | List of IPv4 CIDR addresses to whitelist |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inputs` | Vec<String> | The list of inputs currently using this Input Security Group. |
| `arn` | String | Unique ARN of Input Security Group |
| `whitelist_rules` | Vec<String> | Whitelist rules and their sync status |
| `id` | String | The Id of the Input Security Group |
| `tags` | HashMap<String, String> | A collection of key-value pairs. |
| `state` | String | The current state of the Input Security Group. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create input_security_group
input_security_group = provider.medialive.Input_security_group {
}

# Access input_security_group outputs
input_security_group_id = input_security_group.id
input_security_group_inputs = input_security_group.inputs
input_security_group_arn = input_security_group.arn
input_security_group_whitelist_rules = input_security_group.whitelist_rules
input_security_group_id = input_security_group.id
input_security_group_tags = input_security_group.tags
input_security_group_state = input_security_group.state
```

---


### Node_registration_script

NodeRegistrationScript resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Specify a pattern for MediaLive Anywhere to use to assign a name to each Node in the Cluster. The pattern can include the variables $hn (hostname of the node hardware) and $ts for the date and time that the Node is created, in UTC (for example, 2024-08-20T23:35:12Z). |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |
| `cluster_id` | String | ✅ | The ID of the cluster |
| `id` | String |  | If you're generating a re-registration script for an already existing node, this is where you provide the id. |
| `node_interface_mappings` | Vec<String> |  | Documentation update needed |
| `role` | String |  | The initial role of the Node in the Cluster. ACTIVE means the Node is available for encoding. BACKUP means the Node is a redundant Node and might get used if an ACTIVE Node fails. |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create node_registration_script
node_registration_script = provider.medialive.Node_registration_script {
    cluster_id = "value"  # The ID of the cluster
}

```

---


### Network

Network resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Specify a name that is unique in the AWS account. We recommend that you assign a name that hints at the type of traffic on the network. Names are case-sensitive. |
| `ip_pools` | Vec<String> |  | An array of IpPoolCreateRequests that identify a collection of IP addresses in your network that you want to reserve for use in MediaLive Anywhere. MediaLiveAnywhere uses these IP addresses for Push inputs (in both Bridge and NATnetworks) and for output destinations (only in Bridge networks). EachIpPoolUpdateRequest specifies one CIDR block. |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |
| `routes` | Vec<String> |  | An array of routes that MediaLive Anywhere needs to know about in order to route encoding traffic. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | The current state of the Network. Only MediaLive Anywhere can change the state. |
| `id` | String | The ID of the Network. Unique in the AWS account. The ID is the resource-id portion of the ARN. |
| `name` | String | The name that you specified for the Network. |
| `associated_cluster_ids` | Vec<String> |  |
| `ip_pools` | Vec<String> | An array of IpPools in your organization's network that identify a collection of IP addresses in this network that are reserved for use in MediaLive Anywhere. MediaLive Anywhere uses these IP addresses for Push inputs (in both Bridge and NAT networks) and for output destinations (only in Bridge networks). Each IpPool specifies one CIDR block. |
| `routes` | Vec<String> | An array of routes that MediaLive Anywhere needs to know about in order to route encoding traffic. |
| `arn` | String | The ARN of this Network. It is automatically assigned when the Network is created. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network
network = provider.medialive.Network {
}

# Access network outputs
network_id = network.id
network_state = network.state
network_id = network.id
network_name = network.name
network_associated_cluster_ids = network.associated_cluster_ids
network_ip_pools = network.ip_pools
network_routes = network.routes
network_arn = network.arn
```

---


### Event_bridge_rule_template

EventBridgeRuleTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A resource's optional description. |
| `event_type` | String | ✅ |  |
| `group_identifier` | String | ✅ | An eventbridge rule template group's identifier. Can be either be its id or current name. |
| `name` | String | ✅ | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `event_targets` | Vec<String> |  |  |
| `tags` | HashMap<String, String> |  |  |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `event_type` | String |  |
| `modified_at` | String |  |
| `arn` | String | An eventbridge rule template's ARN (Amazon Resource Name) |
| `description` | String | A resource's optional description. |
| `id` | String | An eventbridge rule template's id. AWS provided templates have ids that start with `aws-` |
| `created_at` | String |  |
| `tags` | HashMap<String, String> |  |
| `group_id` | String | An eventbridge rule template group's id. AWS provided template groups have ids that start with `aws-` |
| `event_targets` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_bridge_rule_template
event_bridge_rule_template = provider.medialive.Event_bridge_rule_template {
    event_type = "value"  # Required field
    group_identifier = "value"  # An eventbridge rule template group's identifier. Can be either be its id or current name.
    name = "value"  # A resource's name. Names must be unique within the scope of a resource type in a specific region.
}

# Access event_bridge_rule_template outputs
event_bridge_rule_template_id = event_bridge_rule_template.id
event_bridge_rule_template_name = event_bridge_rule_template.name
event_bridge_rule_template_event_type = event_bridge_rule_template.event_type
event_bridge_rule_template_modified_at = event_bridge_rule_template.modified_at
event_bridge_rule_template_arn = event_bridge_rule_template.arn
event_bridge_rule_template_description = event_bridge_rule_template.description
event_bridge_rule_template_id = event_bridge_rule_template.id
event_bridge_rule_template_created_at = event_bridge_rule_template.created_at
event_bridge_rule_template_tags = event_bridge_rule_template.tags
event_bridge_rule_template_group_id = event_bridge_rule_template.group_id
event_bridge_rule_template_event_targets = event_bridge_rule_template.event_targets
```

---


### Schedule

Schedule resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | The next token; for use in pagination. |
| `schedule_actions` | Vec<String> | The list of actions in the schedule. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access schedule outputs
schedule_id = schedule.id
schedule_next_token = schedule.next_token
schedule_schedule_actions = schedule.schedule_actions
```

---


### Cloud_watch_alarm_template

CloudWatchAlarmTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_resource_type` | String | ✅ |  |
| `group_identifier` | String | ✅ | A cloudwatch alarm template group's identifier. Can be either be its id or current name. |
| `comparison_operator` | String | ✅ |  |
| `threshold` | f64 | ✅ | The threshold value to compare with the specified statistic. |
| `description` | String |  | A resource's optional description. |
| `datapoints_to_alarm` | i64 |  | The number of datapoints within the evaluation period that must be breaching to trigger the alarm. |
| `evaluation_periods` | i64 | ✅ | The number of periods over which data is compared to the specified threshold. |
| `tags` | HashMap<String, String> |  |  |
| `period` | i64 | ✅ | The period, in seconds, over which the specified statistic is applied. |
| `treat_missing_data` | String | ✅ |  |
| `statistic` | String | ✅ |  |
| `name` | String | ✅ | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `metric_name` | String | ✅ | The name of the metric associated with the alarm. Must be compatible with targetResourceType. |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_id` | String | A cloudwatch alarm template group's id. AWS provided template groups have ids that start with `aws-` |
| `created_at` | String |  |
| `name` | String | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `statistic` | String |  |
| `description` | String | A resource's optional description. |
| `evaluation_periods` | i64 | The number of periods over which data is compared to the specified threshold. |
| `target_resource_type` | String |  |
| `threshold` | f64 | The threshold value to compare with the specified statistic. |
| `id` | String | A cloudwatch alarm template's id. AWS provided templates have ids that start with `aws-` |
| `period` | i64 | The period, in seconds, over which the specified statistic is applied. |
| `treat_missing_data` | String |  |
| `tags` | HashMap<String, String> |  |
| `arn` | String | A cloudwatch alarm template's ARN (Amazon Resource Name) |
| `comparison_operator` | String |  |
| `metric_name` | String | The name of the metric associated with the alarm. Must be compatible with targetResourceType. |
| `modified_at` | String |  |
| `datapoints_to_alarm` | i64 | The number of datapoints within the evaluation period that must be breaching to trigger the alarm. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cloud_watch_alarm_template
cloud_watch_alarm_template = provider.medialive.Cloud_watch_alarm_template {
    target_resource_type = "value"  # Required field
    group_identifier = "value"  # A cloudwatch alarm template group's identifier. Can be either be its id or current name.
    comparison_operator = "value"  # Required field
    threshold = "value"  # The threshold value to compare with the specified statistic.
    evaluation_periods = "value"  # The number of periods over which data is compared to the specified threshold.
    period = "value"  # The period, in seconds, over which the specified statistic is applied.
    treat_missing_data = "value"  # Required field
    statistic = "value"  # Required field
    name = "value"  # A resource's name. Names must be unique within the scope of a resource type in a specific region.
    metric_name = "value"  # The name of the metric associated with the alarm. Must be compatible with targetResourceType.
}

# Access cloud_watch_alarm_template outputs
cloud_watch_alarm_template_id = cloud_watch_alarm_template.id
cloud_watch_alarm_template_group_id = cloud_watch_alarm_template.group_id
cloud_watch_alarm_template_created_at = cloud_watch_alarm_template.created_at
cloud_watch_alarm_template_name = cloud_watch_alarm_template.name
cloud_watch_alarm_template_statistic = cloud_watch_alarm_template.statistic
cloud_watch_alarm_template_description = cloud_watch_alarm_template.description
cloud_watch_alarm_template_evaluation_periods = cloud_watch_alarm_template.evaluation_periods
cloud_watch_alarm_template_target_resource_type = cloud_watch_alarm_template.target_resource_type
cloud_watch_alarm_template_threshold = cloud_watch_alarm_template.threshold
cloud_watch_alarm_template_id = cloud_watch_alarm_template.id
cloud_watch_alarm_template_period = cloud_watch_alarm_template.period
cloud_watch_alarm_template_treat_missing_data = cloud_watch_alarm_template.treat_missing_data
cloud_watch_alarm_template_tags = cloud_watch_alarm_template.tags
cloud_watch_alarm_template_arn = cloud_watch_alarm_template.arn
cloud_watch_alarm_template_comparison_operator = cloud_watch_alarm_template.comparison_operator
cloud_watch_alarm_template_metric_name = cloud_watch_alarm_template.metric_name
cloud_watch_alarm_template_modified_at = cloud_watch_alarm_template.modified_at
cloud_watch_alarm_template_datapoints_to_alarm = cloud_watch_alarm_template.datapoints_to_alarm
```

---


### Node_state

NodeState resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | The state to apply to the Node. Set to ACTIVE (COMMISSIONED) to indicate that the Node is deployable. MediaLive Anywhere will consider this node it needs a Node to run a Channel on, or when it needs a Node to promote from a backup node to an active node. Set to DRAINING to isolate the Node so that MediaLive Anywhere won't use it. |
| `node_id` | String | ✅ | The ID of the node. |
| `cluster_id` | String | ✅ | The ID of the cluster |



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


### Thumbnails

Thumbnails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `thumbnail_details` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access thumbnails outputs
thumbnails_id = thumbnails.id
thumbnails_thumbnail_details = thumbnails.thumbnail_details
```

---


### Cloud_watch_alarm_template_group

CloudWatchAlarmTemplateGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |
| `name` | String | ✅ | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `description` | String |  | A resource's optional description. |
| `tags` | HashMap<String, String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `modified_at` | String |  |
| `description` | String | A resource's optional description. |
| `arn` | String | A cloudwatch alarm template group's ARN (Amazon Resource Name) |
| `tags` | HashMap<String, String> |  |
| `name` | String | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `id` | String | A cloudwatch alarm template group's id. AWS provided template groups have ids that start with `aws-` |
| `created_at` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cloud_watch_alarm_template_group
cloud_watch_alarm_template_group = provider.medialive.Cloud_watch_alarm_template_group {
    name = "value"  # A resource's name. Names must be unique within the scope of a resource type in a specific region.
}

# Access cloud_watch_alarm_template_group outputs
cloud_watch_alarm_template_group_id = cloud_watch_alarm_template_group.id
cloud_watch_alarm_template_group_modified_at = cloud_watch_alarm_template_group.modified_at
cloud_watch_alarm_template_group_description = cloud_watch_alarm_template_group.description
cloud_watch_alarm_template_group_arn = cloud_watch_alarm_template_group.arn
cloud_watch_alarm_template_group_tags = cloud_watch_alarm_template_group.tags
cloud_watch_alarm_template_group_name = cloud_watch_alarm_template_group.name
cloud_watch_alarm_template_group_id = cloud_watch_alarm_template_group.id
cloud_watch_alarm_template_group_created_at = cloud_watch_alarm_template_group.created_at
```

---


### Account_configuration

AccountConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_configuration` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_configuration` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_configuration outputs
account_configuration_id = account_configuration.id
account_configuration_account_configuration = account_configuration.account_configuration
```

---


### Offering

Offering resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fixed_price` | f64 | One-time charge for each reserved resource, e.g. '0.0' for a NO_UPFRONT offering |
| `duration_units` | String | Units for duration, e.g. 'MONTHS' |
| `currency_code` | String | Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. 'USD' |
| `resource_specification` | String | Resource configuration details |
| `duration` | i64 | Lease duration, e.g. '12' |
| `region` | String | AWS region, e.g. 'us-west-2' |
| `offering_type` | String | Offering type, e.g. 'NO_UPFRONT' |
| `offering_id` | String | Unique offering ID, e.g. '87654321' |
| `usage_price` | f64 | Recurring usage charge for each reserved resource, e.g. '157.0' |
| `arn` | String | Unique offering ARN, e.g. 'arn:aws:medialive:us-west-2:123456789012:offering:87654321' |
| `offering_description` | String | Offering description, e.g. 'HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)' |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access offering outputs
offering_id = offering.id
offering_fixed_price = offering.fixed_price
offering_duration_units = offering.duration_units
offering_currency_code = offering.currency_code
offering_resource_specification = offering.resource_specification
offering_duration = offering.duration
offering_region = offering.region
offering_offering_type = offering.offering_type
offering_offering_id = offering.offering_id
offering_usage_price = offering.usage_price
offering_arn = offering.arn
offering_offering_description = offering.offering_description
```

---


### Signal_map

SignalMap resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  |  |
| `description` | String |  | A resource's optional description. |
| `discovery_entry_point_arn` | String | ✅ | A top-level supported AWS resource ARN to discovery a signal map from. |
| `name` | String | ✅ | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |
| `cloud_watch_alarm_template_group_identifiers` | Vec<String> |  |  |
| `event_bridge_rule_template_group_identifiers` | Vec<String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `modified_at` | String |  |
| `media_resource_map` | HashMap<String, String> |  |
| `last_discovered_at` | String |  |
| `created_at` | String |  |
| `error_message` | String | Error message associated with a failed creation or failed update attempt of a signal map. |
| `failed_media_resource_map` | HashMap<String, String> |  |
| `status` | String |  |
| `tags` | HashMap<String, String> |  |
| `description` | String | A resource's optional description. |
| `monitor_changes_pending_deployment` | bool | If true, there are pending monitor changes for this signal map that can be deployed. |
| `event_bridge_rule_template_group_ids` | Vec<String> |  |
| `name` | String | A resource's name. Names must be unique within the scope of a resource type in a specific region. |
| `monitor_deployment` | String |  |
| `last_successful_monitor_deployment` | String |  |
| `discovery_entry_point_arn` | String | A top-level supported AWS resource ARN to discovery a signal map from. |
| `arn` | String | A signal map's ARN (Amazon Resource Name) |
| `id` | String | A signal map's id. |
| `cloud_watch_alarm_template_group_ids` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create signal_map
signal_map = provider.medialive.Signal_map {
    discovery_entry_point_arn = "value"  # A top-level supported AWS resource ARN to discovery a signal map from.
    name = "value"  # A resource's name. Names must be unique within the scope of a resource type in a specific region.
}

# Access signal_map outputs
signal_map_id = signal_map.id
signal_map_modified_at = signal_map.modified_at
signal_map_media_resource_map = signal_map.media_resource_map
signal_map_last_discovered_at = signal_map.last_discovered_at
signal_map_created_at = signal_map.created_at
signal_map_error_message = signal_map.error_message
signal_map_failed_media_resource_map = signal_map.failed_media_resource_map
signal_map_status = signal_map.status
signal_map_tags = signal_map.tags
signal_map_description = signal_map.description
signal_map_monitor_changes_pending_deployment = signal_map.monitor_changes_pending_deployment
signal_map_event_bridge_rule_template_group_ids = signal_map.event_bridge_rule_template_group_ids
signal_map_name = signal_map.name
signal_map_monitor_deployment = signal_map.monitor_deployment
signal_map_last_successful_monitor_deployment = signal_map.last_successful_monitor_deployment
signal_map_discovery_entry_point_arn = signal_map.discovery_entry_point_arn
signal_map_arn = signal_map.arn
signal_map_id = signal_map.id
signal_map_cloud_watch_alarm_template_group_ids = signal_map.cloud_watch_alarm_template_group_ids
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |
| `request_id` | String |  | The unique ID of the request. |
| `instance_role_arn` | String |  | The ARN of the IAM role for the Node in this Cluster. The role must include all the operations that you expect these Node to perform. If necessary, create a role in IAM, then attach it here. |
| `network_settings` | String |  | Network settings that connect the Nodes in the Cluster to one or more of the Networks that the Cluster is associated with. |
| `name` | String |  | Specify a name that is unique in the AWS account. We recommend that you assign a name that hints at the types of Nodes in the Cluster. Names are case-sensitive. |
| `cluster_type` | String |  | Specify a type. All the Nodes that you later add to this Cluster must be this type of hardware. One Cluster instance can't contain different hardware types. You won't be able to change this parameter after you create the Cluster. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | The current state of the Cluster. |
| `name` | String | The name that you specified for the Cluster. |
| `network_settings` | String | Network settings that connect the Nodes in the Cluster to one or more of the Networks that the Cluster is associated with. |
| `cluster_type` | String | The hardware type for the Cluster |
| `channel_ids` | Vec<String> |  |
| `id` | String | The ID of the  Cluster. Unique in the AWS account. The ID is the resource-id portion of the ARN. |
| `instance_role_arn` | String | The ARN of the IAM role for the Node in this Cluster. Any Nodes that are associated with this Cluster assume this role. The role gives permissions to the operations that you expect these Node to perform. |
| `arn` | String | The ARN of this Cluster. It is automatically assigned when the Cluster is created. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.medialive.Cluster {
}

# Access cluster outputs
cluster_id = cluster.id
cluster_state = cluster.state
cluster_name = cluster.name
cluster_network_settings = cluster.network_settings
cluster_cluster_type = cluster.cluster_type
cluster_channel_ids = cluster.channel_ids
cluster_id = cluster.id
cluster_instance_role_arn = cluster.instance_role_arn
cluster_arn = cluster.arn
```

---


### Reservation

Reservation resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reservation_id` | String | ✅ | Unique reservation ID, e.g. '1234567' |
| `name` | String |  | Name of the reservation |
| `renewal_settings` | String |  | Renewal settings for the reservation |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fixed_price` | f64 | One-time charge for each reserved resource, e.g. '0.0' for a NO_UPFRONT offering |
| `name` | String | User specified reservation name |
| `duration_units` | String | Units for duration, e.g. 'MONTHS' |
| `offering_type` | String | Offering type, e.g. 'NO_UPFRONT' |
| `arn` | String | Unique reservation ARN, e.g. 'arn:aws:medialive:us-west-2:123456789012:reservation:1234567' |
| `reservation_id` | String | Unique reservation ID, e.g. '1234567' |
| `state` | String | Current state of reservation, e.g. 'ACTIVE' |
| `offering_description` | String | Offering description, e.g. 'HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)' |
| `count` | i64 | Number of reserved resources |
| `offering_id` | String | Unique offering ID, e.g. '87654321' |
| `renewal_settings` | String | Renewal settings for the reservation |
| `currency_code` | String | Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. 'USD' |
| `duration` | i64 | Lease duration, e.g. '12' |
| `tags` | HashMap<String, String> | A collection of key-value pairs |
| `end` | String | Reservation UTC end date and time in ISO-8601 format, e.g. '2019-03-01T00:00:00' |
| `usage_price` | f64 | Recurring usage charge for each reserved resource, e.g. '157.0' |
| `resource_specification` | String | Resource configuration details |
| `start` | String | Reservation UTC start date and time in ISO-8601 format, e.g. '2018-03-01T00:00:00' |
| `region` | String | AWS region, e.g. 'us-west-2' |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reservation outputs
reservation_id = reservation.id
reservation_fixed_price = reservation.fixed_price
reservation_name = reservation.name
reservation_duration_units = reservation.duration_units
reservation_offering_type = reservation.offering_type
reservation_arn = reservation.arn
reservation_reservation_id = reservation.reservation_id
reservation_state = reservation.state
reservation_offering_description = reservation.offering_description
reservation_count = reservation.count
reservation_offering_id = reservation.offering_id
reservation_renewal_settings = reservation.renewal_settings
reservation_currency_code = reservation.currency_code
reservation_duration = reservation.duration
reservation_tags = reservation.tags
reservation_end = reservation.end
reservation_usage_price = reservation.usage_price
reservation_resource_specification = reservation.resource_specification
reservation_start = reservation.start
reservation_region = reservation.region
```

---


### Channel_placement_group

ChannelPlacementGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nodes` | Vec<String> |  | An array of one ID for the Node that you want to associate with the ChannelPlacementGroup. (You can't associate more than one Node with the ChannelPlacementGroup.) The Node and the ChannelPlacementGroup must be in the same Cluster. |
| `name` | String |  | Specify a name that is unique in the Cluster. You can't change the name. Names are case-sensitive. |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |
| `cluster_id` | String | ✅ | The ID of the cluster. |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. the request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | The current state of the ChannelPlacementGroup. |
| `name` | String | The name that you specified for the ChannelPlacementGroup. |
| `cluster_id` | String | The ID of the Cluster that the Node belongs to. |
| `channels` | Vec<String> | Used in ListChannelPlacementGroupsResult |
| `nodes` | Vec<String> | An array with one item, which is the single Node that is associated with the ChannelPlacementGroup. |
| `arn` | String | The ARN of this ChannelPlacementGroup. It is automatically assigned when the ChannelPlacementGroup is created. |
| `id` | String | The ID of the ChannelPlacementGroup. Unique in the AWS account. The ID is the resource-id portion of the ARN. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_placement_group
channel_placement_group = provider.medialive.Channel_placement_group {
    cluster_id = "value"  # The ID of the cluster.
}

# Access channel_placement_group outputs
channel_placement_group_id = channel_placement_group.id
channel_placement_group_state = channel_placement_group.state
channel_placement_group_name = channel_placement_group.name
channel_placement_group_cluster_id = channel_placement_group.cluster_id
channel_placement_group_channels = channel_placement_group.channels
channel_placement_group_nodes = channel_placement_group.nodes
channel_placement_group_arn = channel_placement_group.arn
channel_placement_group_id = channel_placement_group.id
```

---


### Node

Node resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The user-specified name of the Node to be created. |
| `node_interface_mappings` | Vec<String> |  | Documentation update needed |
| `cluster_id` | String | ✅ | The ID of the cluster. |
| `role` | String |  | The initial role of the Node in the Cluster. ACTIVE means the Node is available for encoding. BACKUP means the Node is a redundant Node and might get used if an ACTIVE Node fails. |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The unique ID of the Node. Unique in the Cluster. The ID is the resource-id portion of the ARN. |
| `connection_state` | String | The current connection state of the Node. |
| `sdi_source_mappings` | Vec<String> | An array of SDI source mappings. Each mapping connects one logical SdiSource to the physical SDI card and port that the physical SDI source uses. |
| `arn` | String | The ARN of the Node. It is automatically assigned when the Node is created. |
| `channel_placement_groups` | Vec<String> | An array of IDs. Each ID is one ChannelPlacementGroup that is associated with this Node. Empty if the Node is not yet associated with any groups. |
| `cluster_id` | String | The ID of the Cluster that the Node belongs to. |
| `role` | String | The initial role current role of the Node in the Cluster. ACTIVE means the Node is available for encoding. BACKUP means the Node is a redundant Node and might get used if an ACTIVE Node fails. |
| `state` | String | The current state of the Node. |
| `name` | String | The name that you specified for the Node. |
| `instance_arn` | String | The ARN of the EC2 instance hosting the Node. |
| `node_interface_mappings` | Vec<String> | Documentation update needed |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create node
node = provider.medialive.Node {
    cluster_id = "value"  # The ID of the cluster.
}

# Access node outputs
node_id = node.id
node_id = node.id
node_connection_state = node.connection_state
node_sdi_source_mappings = node.sdi_source_mappings
node_arn = node.arn
node_channel_placement_groups = node.channel_placement_groups
node_cluster_id = node.cluster_id
node_role = node.role
node_state = node.state
node_name = node.name
node_instance_arn = node.instance_arn
node_node_interface_mappings = node.node_interface_mappings
```

---


### Channel

Channel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encoder_settings` | String |  |  |
| `input_specification` | String |  | Specification of network and file inputs for this channel |
| `role_arn` | String |  | An optional Amazon Resource Name (ARN) of the role to assume when running the Channel. |
| `vpc` | String |  | Settings for the VPC outputs |
| `destinations` | Vec<String> |  |  |
| `dry_run` | bool |  |  |
| `log_level` | String |  | The log level to write to CloudWatch Logs. |
| `maintenance` | String |  | Maintenance settings for this channel. |
| `request_id` | String |  | Unique request ID to be specified. This is needed to prevent retries from
creating multiple resources. |
| `anywhere_settings` | String |  | The Elemental Anywhere settings for this channel. |
| `channel_class` | String |  | The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline. |
| `input_attachments` | Vec<String> |  | List of input attachments for channel. |
| `reserved` | String |  | Deprecated field that's only usable by whitelisted customers. |
| `name` | String |  | Name of channel. |
| `cdi_input_specification` | String |  | Specification of CDI inputs for this channel |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |
| `channel_engine_version` | String |  | The desired engine version for this channel. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encoder_settings` | String |  |
| `egress_endpoints` | Vec<String> | The endpoints where outgoing connections initiate from |
| `input_specification` | String | Specification of network and file inputs for this channel |
| `cdi_input_specification` | String | Specification of CDI inputs for this channel |
| `vpc` | String | Settings for VPC output |
| `maintenance` | String | Maintenance settings for this channel. |
| `pipeline_details` | Vec<String> | Runtime details for the pipelines of a running channel. |
| `arn` | String | The unique arn of the channel. |
| `pipelines_running_count` | i64 | The number of currently healthy pipelines. |
| `input_attachments` | Vec<String> | List of input attachments for channel. |
| `id` | String | The unique id of the channel. |
| `channel_class` | String | The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline. |
| `role_arn` | String | The Amazon Resource Name (ARN) of the role assumed when running the Channel. |
| `state` | String |  |
| `name` | String | The name of the channel. (user-mutable) |
| `tags` | HashMap<String, String> | A collection of key-value pairs. |
| `log_level` | String | The log level being written to CloudWatch Logs. |
| `destinations` | Vec<String> | A list of destinations of the channel. For UDP outputs, there is one
destination per output. For other types (HLS, for example), there is
one destination per packager. |
| `channel_engine_version` | String | Requested engine version for this channel. |
| `anywhere_settings` | String | Anywhere settings for this channel. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel
channel = provider.medialive.Channel {
}

# Access channel outputs
channel_id = channel.id
channel_encoder_settings = channel.encoder_settings
channel_egress_endpoints = channel.egress_endpoints
channel_input_specification = channel.input_specification
channel_cdi_input_specification = channel.cdi_input_specification
channel_vpc = channel.vpc
channel_maintenance = channel.maintenance
channel_pipeline_details = channel.pipeline_details
channel_arn = channel.arn
channel_pipelines_running_count = channel.pipelines_running_count
channel_input_attachments = channel.input_attachments
channel_id = channel.id
channel_channel_class = channel.channel_class
channel_role_arn = channel.role_arn
channel_state = channel.state
channel_name = channel.name
channel_tags = channel.tags
channel_log_level = channel.log_level
channel_destinations = channel.destinations
channel_channel_engine_version = channel.channel_engine_version
channel_anywhere_settings = channel.anywhere_settings
```

---


### Tags

Tags resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ |  |
| `tags` | HashMap<String, String> |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tags
tags = provider.medialive.Tags {
    resource_arn = "value"  # Required field
}

```

---


### Sdi_source

SdiSource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Specify a name that is unique in the AWS account. We recommend you assign a name that describes the source, for example curling-cameraA. Names are case-sensitive. |
| `request_id` | String |  | An ID that you assign to a create request. This ID ensures idempotency when creating resources. |
| `type` | String |  | Specify the  type of the SDI source: SINGLE: The source  is a single-link source. QUAD: The source  is one part of a quad-link source. |
| `mode` | String |  | Applies only if the type is QUAD. Specify the mode for handling the quad-link signal: QUADRANT or INTERLEAVE. |
| `tags` | HashMap<String, String> |  | A collection of key-value pairs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sdi_source` | String | Settings for the SDI source. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sdi_source
sdi_source = provider.medialive.Sdi_source {
}

# Access sdi_source outputs
sdi_source_id = sdi_source.id
sdi_source_sdi_source = sdi_source.sdi_source
```

---


### Multiplex_program

MultiplexProgram resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `multiplex_id` | String | ✅ | ID of the multiplex where the program is to be created. |
| `request_id` | String | ✅ | Unique request ID. This prevents retries from creating multiple
resources. |
| `multiplex_program_settings` | String | ✅ | The settings for this multiplex program. |
| `program_name` | String | ✅ | Name of multiplex program. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `multiplex_program_settings` | String | The settings for this multiplex program. |
| `pipeline_details` | Vec<String> | Contains information about the current sources for the specified program in the specified multiplex. Keep in mind that each multiplex pipeline connects to both pipelines in a given source channel (the channel identified by the program). But only one of those channel pipelines is ever active at one time. |
| `channel_id` | String | The MediaLive channel associated with the program. |
| `program_name` | String | The name of the multiplex program. |
| `packet_identifiers_map` | String | The packet identifier map for this multiplex program. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiplex_program
multiplex_program = provider.medialive.Multiplex_program {
    multiplex_id = "value"  # ID of the multiplex where the program is to be created.
    request_id = "value"  # Unique request ID. This prevents retries from creating multiple
resources.
    multiplex_program_settings = "value"  # The settings for this multiplex program.
    program_name = "value"  # Name of multiplex program.
}

# Access multiplex_program outputs
multiplex_program_id = multiplex_program.id
multiplex_program_multiplex_program_settings = multiplex_program.multiplex_program_settings
multiplex_program_pipeline_details = multiplex_program.pipeline_details
multiplex_program_channel_id = multiplex_program.channel_id
multiplex_program_program_name = multiplex_program.program_name
multiplex_program_packet_identifiers_map = multiplex_program.packet_identifiers_map
```

---


### Channel_class

ChannelClass resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `channel_id` | String | ✅ | Channel Id of the channel whose class should be updated. |
| `channel_class` | String | ✅ | The channel class that you wish to update this channel to use. |
| `destinations` | Vec<String> |  | A list of output destinations for this channel. |



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


### Input_device_thumbnail

InputDeviceThumbnail resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified` | String | The date and time the thumbnail was last updated at the device. |
| `content_type` | String | Specifies the media type of the thumbnail. |
| `e_tag` | String | The unique, cacheable version of this thumbnail. |
| `content_length` | i64 | The length of the content. |
| `body` | String | The binary data for the thumbnail that the Link device has most recently sent to MediaLive. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access input_device_thumbnail outputs
input_device_thumbnail_id = input_device_thumbnail.id
input_device_thumbnail_last_modified = input_device_thumbnail.last_modified
input_device_thumbnail_content_type = input_device_thumbnail.content_type
input_device_thumbnail_e_tag = input_device_thumbnail.e_tag
input_device_thumbnail_content_length = input_device_thumbnail.content_length
input_device_thumbnail_body = input_device_thumbnail.body
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple partner_input resources
partner_input_0 = provider.medialive.Partner_input {
    input_id = "value-0"
}
partner_input_1 = provider.medialive.Partner_input {
    input_id = "value-1"
}
partner_input_2 = provider.medialive.Partner_input {
    input_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    partner_input = provider.medialive.Partner_input {
        input_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Medialive Documentation](https://docs.aws.amazon.com/medialive/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
