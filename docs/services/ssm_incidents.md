# Ssm_incidents Service



**Resources**: 8

---

## Overview

The ssm_incidents service provides access to 8 resource types:

- [Incident_record](#incident_record) [RUD]
- [Resource_policies](#resource_policies) [R]
- [Resource_policy](#resource_policy) [CD]
- [Related_items](#related_items) [U]
- [Deletion_protection](#deletion_protection) [U]
- [Response_plan](#response_plan) [CRUD]
- [Replication_set](#replication_set) [CRUD]
- [Timeline_event](#timeline_event) [CRUD]

---

## Resources


### Incident_record

IncidentRecord resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the incident record you are updating.</p> |
| `title` | String |  | <p>A brief description of the incident.</p> |
| `notification_targets` | Vec<String> |  | <p>The Amazon SNS targets that Incident Manager notifies when a client updates an
      incident.</p>
         <p>Using multiple SNS topics creates redundancy in the event that a Region is down during the
      incident.</p> |
| `impact` | i64 |  | <p>Defines the impact of the incident to customers and applications. If you provide an impact
      for an incident, it overwrites the impact provided by the response plan.</p>
         <p class="title">
            <b>Supported impact codes</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>1</code> - Critical</p>
            </li>
            <li>
               <p>
                  <code>2</code> - High</p>
            </li>
            <li>
               <p>
                  <code>3</code> - Medium</p>
            </li>
            <li>
               <p>
                  <code>4</code> - Low</p>
            </li>
            <li>
               <p>
                  <code>5</code> - No Impact</p>
            </li>
         </ul> |
| `summary` | String |  | <p>A longer description of what occurred during the incident.</p> |
| `status` | String |  | <p>The status of the incident. Possible statuses are <code>Open</code> or
        <code>Resolved</code>.</p> |
| `client_token` | String |  | <p>A token that ensures that a client calls the operation only once with the specified
      details.</p> |
| `chat_channel` | String |  | <p>The Chatbot chat channel where responders can collaborate.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `incident_record` | String | <p>Details the structure of the incident record.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access incident_record outputs
incident_record_id = incident_record.id
incident_record_incident_record = incident_record.incident_record
```

---


### Resource_policies

ResourcePolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_policies` | Vec<String> | <p>Details about the resource policy attached to the response plan.</p> |
| `next_token` | String | <p>The pagination token to use when requesting the next set of items. If there are no
      additional items to return, the string is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_policies outputs
resource_policies_id = resource_policies.id
resource_policies_resource_policies = resource_policies.resource_policies
resource_policies_next_token = resource_policies.next_token
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the response plan to add the resource policy to.</p> |
| `policy` | String | ✅ | <p>Details of the resource policy.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.ssm_incidents.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the response plan to add the resource policy to.</p>
    policy = "value"  # <p>Details of the resource policy.</p>
}

```

---


### Related_items

RelatedItems resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `incident_record_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the incident record that contains the related items that
      you update.</p> |
| `client_token` | String |  | <p>A token that ensures that a client calls the operation only once with the specified
      details.</p> |
| `related_items_update` | String | ✅ | <p>Details about the item that you are add to, or delete from, an incident.</p> |



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


### Deletion_protection

DeletionProtection resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deletion_protected` | bool | ✅ | <p>Specifies if deletion protection is turned on or off in your account. </p> |
| `client_token` | String |  | <p>A token that ensures that the operation is called only once with the specified
      details.</p> |
| `arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the replication set to update.</p> |



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


### Response_plan

ResponsePlan resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The short format name of the response plan. Can't include spaces.</p> |
| `display_name` | String |  | <p>The long format of the response plan name. This field can contain spaces.</p> |
| `actions` | Vec<String> |  | <p>The actions that the response plan starts at the beginning of an incident.</p> |
| `engagements` | Vec<String> |  | <p>The Amazon Resource Name (ARN) for the contacts and escalation plans that the response
      plan engages during an incident.</p> |
| `client_token` | String |  | <p>A token ensuring that the operation is called only once with the specified details.</p> |
| `integrations` | Vec<String> |  | <p>Information about third-party services integrated into the response plan.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags that you are adding to the response plan.</p> |
| `incident_template` | String | ✅ | <p>Details used to create an incident when using this response plan.</p> |
| `chat_channel` | String |  | <p>The Chatbot chat channel used for collaboration during an incident.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `actions` | Vec<String> | <p>The actions that this response plan takes at the beginning of the incident.</p> |
| `name` | String | <p>The short format name of the response plan. The name can't contain spaces.</p> |
| `incident_template` | String | <p>Details used to create the incident when using this response plan.</p> |
| `integrations` | Vec<String> | <p>Information about third-party services integrated into the Incident Manager response
      plan.</p> |
| `arn` | String | <p>The ARN of the response plan.</p> |
| `display_name` | String | <p>The long format name of the response plan. Can contain spaces.</p> |
| `engagements` | Vec<String> | <p>The Amazon Resource Name (ARN) for the contacts and escalation plans that the response
      plan engages during an incident.</p> |
| `chat_channel` | String | <p>The Chatbot chat channel used for collaboration during an incident.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create response_plan
response_plan = provider.ssm_incidents.Response_plan {
    name = "value"  # <p>The short format name of the response plan. Can't include spaces.</p>
    incident_template = "value"  # <p>Details used to create an incident when using this response plan.</p>
}

# Access response_plan outputs
response_plan_id = response_plan.id
response_plan_actions = response_plan.actions
response_plan_name = response_plan.name
response_plan_incident_template = response_plan.incident_template
response_plan_integrations = response_plan.integrations
response_plan_arn = response_plan.arn
response_plan_display_name = response_plan.display_name
response_plan_engagements = response_plan.engagements
response_plan_chat_channel = response_plan.chat_channel
```

---


### Replication_set

ReplicationSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A list of tags to add to the replication set.</p> |
| `client_token` | String |  | <p>A token that ensures that the operation is called only once with the specified
      details.</p> |
| `regions` | HashMap<String, String> | ✅ | <p>The Regions that Incident Manager replicates your data to. You can have up to three Regions in
      your replication set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_set` | String | <p>Details of the replication set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_set
replication_set = provider.ssm_incidents.Replication_set {
    regions = "value"  # <p>The Regions that Incident Manager replicates your data to. You can have up to three Regions in
      your replication set.</p>
}

# Access replication_set outputs
replication_set_id = replication_set.id
replication_set_replication_set = replication_set.replication_set
```

---


### Timeline_event

TimelineEvent resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_data` | String | ✅ | <p>A short description of the event.</p> |
| `incident_record_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the incident record that the action adds the incident
      to.</p> |
| `client_token` | String |  | <p>A token that ensures that a client calls the action only once with the specified
      details.</p> |
| `event_time` | String | ✅ | <p>The timestamp for when the event occurred.</p> |
| `event_type` | String | ✅ | <p>The type of event. You can create timeline events of type <code>Custom Event</code> and
        <code>Note</code>.</p>
         <p>To make a Note-type event appear on the <i>Incident notes</i> panel in the
      console, specify <code>eventType</code> as <code>Note</code>and enter the Amazon Resource Name
      (ARN) of the incident as the value for <code>eventReference</code>.</p> |
| `event_references` | Vec<String> |  | <p>Adds one or more references to the <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter
      its Amazon Resource Name (ARN). You can also specify a related item associated with a
      resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a
      resource, use the table's ARN. You can also specify an Amazon CloudWatch metric associated
      with the DynamoDB table as a related item.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event` | String | <p>Details about the timeline event.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create timeline_event
timeline_event = provider.ssm_incidents.Timeline_event {
    event_data = "value"  # <p>A short description of the event.</p>
    incident_record_arn = "value"  # <p>The Amazon Resource Name (ARN) of the incident record that the action adds the incident
      to.</p>
    event_time = "value"  # <p>The timestamp for when the event occurred.</p>
    event_type = "value"  # <p>The type of event. You can create timeline events of type <code>Custom Event</code> and
        <code>Note</code>.</p>
         <p>To make a Note-type event appear on the <i>Incident notes</i> panel in the
      console, specify <code>eventType</code> as <code>Note</code>and enter the Amazon Resource Name
      (ARN) of the incident as the value for <code>eventReference</code>.</p>
}

# Access timeline_event outputs
timeline_event_id = timeline_event.id
timeline_event_event = timeline_event.event
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple incident_record resources
incident_record_0 = provider.ssm_incidents.Incident_record {
    arn = "value-0"
}
incident_record_1 = provider.ssm_incidents.Incident_record {
    arn = "value-1"
}
incident_record_2 = provider.ssm_incidents.Incident_record {
    arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    incident_record = provider.ssm_incidents.Incident_record {
        arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Ssm_incidents Documentation](https://docs.aws.amazon.com/ssm_incidents/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
