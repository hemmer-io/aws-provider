# Cloudwatch_events Service



**Resources**: 12

---

## Overview

The cloudwatch_events service provides access to 12 resource types:

- [Connection](#connection) [CRUD]
- [Api_destination](#api_destination) [CRUD]
- [Rule](#rule) [CRD]
- [Partner_events](#partner_events) [C]
- [Replay](#replay) [R]
- [Event_bus](#event_bus) [CRD]
- [Targets](#targets) [C]
- [Partner_event_source](#partner_event_source) [CRD]
- [Event_source](#event_source) [R]
- [Archive](#archive) [CRUD]
- [Permission](#permission) [C]
- [Events](#events) [C]

---

## Resources


### Connection

Connection resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the connection to create.</p> |
| `authorization_type` | String | ✅ | <p>The type of authorization to use for the connection.</p> |
| `auth_parameters` | String | ✅ | <p>A <code>CreateConnectionAuthRequestParameters</code> object that contains the
      authorization parameters to use to authorize with the endpoint. </p> |
| `name` | String | ✅ | <p>The name for the connection to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_authorized_time` | String | <p>A time stamp for the time that the connection was last authorized.</p> |
| `connection_arn` | String | <p>The ARN of the connection retrieved.</p> |
| `connection_state` | String | <p>The state of the connection retrieved.</p> |
| `description` | String | <p>The description for the connection retrieved.</p> |
| `auth_parameters` | String | <p>The parameters to use for authorization for the connection.</p> |
| `name` | String | <p>The name of the connection retrieved.</p> |
| `state_reason` | String | <p>The reason that the connection is in the current connection state.</p> |
| `secret_arn` | String | <p>The ARN of the secret created from the authorization parameters specified for the
      connection.</p> |
| `creation_time` | String | <p>A time stamp for the time that the connection was created.</p> |
| `authorization_type` | String | <p>The type of authorization specified for the connection.</p> |
| `last_modified_time` | String | <p>A time stamp for the time that the connection was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.cloudwatch_events.Connection {
    authorization_type = "value"  # <p>The type of authorization to use for the connection.</p>
    auth_parameters = "value"  # <p>A <code>CreateConnectionAuthRequestParameters</code> object that contains the
      authorization parameters to use to authorize with the endpoint. </p>
    name = "value"  # <p>The name for the connection to create.</p>
}

# Access connection outputs
connection_id = connection.id
connection_last_authorized_time = connection.last_authorized_time
connection_connection_arn = connection.connection_arn
connection_connection_state = connection.connection_state
connection_description = connection.description
connection_auth_parameters = connection.auth_parameters
connection_name = connection.name
connection_state_reason = connection.state_reason
connection_secret_arn = connection.secret_arn
connection_creation_time = connection.creation_time
connection_authorization_type = connection.authorization_type
connection_last_modified_time = connection.last_modified_time
```

---


### Api_destination

ApiDestination resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invocation_endpoint` | String | ✅ | <p>The URL to the HTTP invocation endpoint for the API destination.</p> |
| `invocation_rate_limit_per_second` | i64 |  | <p>The maximum number of requests per second to send to the HTTP invocation endpoint.</p> |
| `description` | String |  | <p>A description for the API destination to create.</p> |
| `connection_arn` | String | ✅ | <p>The ARN of the connection to use for the API destination. The destination endpoint must
      support the authorization type specified for the connection.</p> |
| `http_method` | String | ✅ | <p>The method to use for the request to the HTTP invocation endpoint.</p> |
| `name` | String | ✅ | <p>The name for the API destination to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the API destination retrieved.</p> |
| `description` | String | <p>The description for the API destination retrieved.</p> |
| `invocation_endpoint` | String | <p>The URL to use to connect to the HTTP endpoint.</p> |
| `http_method` | String | <p>The method to use to connect to the HTTP endpoint.</p> |
| `invocation_rate_limit_per_second` | i64 | <p>The maximum number of invocations per second to specified for the API destination. Note
      that if you set the invocation rate maximum to a value lower the rate necessary to send all
      events received on to the destination HTTP endpoint, some events may not be delivered within
      the 24-hour retry window. If you plan to set the rate lower than the rate necessary to deliver
      all events, consider using a dead-letter queue to catch events that are not delivered within
      24 hours.</p> |
| `last_modified_time` | String | <p>A time stamp for the time that the API destination was last modified.</p> |
| `creation_time` | String | <p>A time stamp for the time that the API destination was created.</p> |
| `api_destination_arn` | String | <p>The ARN of the API destination retrieved.</p> |
| `connection_arn` | String | <p>The ARN of the connection specified for the API destination retrieved.</p> |
| `api_destination_state` | String | <p>The state of the API destination retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api_destination
api_destination = provider.cloudwatch_events.Api_destination {
    invocation_endpoint = "value"  # <p>The URL to the HTTP invocation endpoint for the API destination.</p>
    connection_arn = "value"  # <p>The ARN of the connection to use for the API destination. The destination endpoint must
      support the authorization type specified for the connection.</p>
    http_method = "value"  # <p>The method to use for the request to the HTTP invocation endpoint.</p>
    name = "value"  # <p>The name for the API destination to create.</p>
}

# Access api_destination outputs
api_destination_id = api_destination.id
api_destination_name = api_destination.name
api_destination_description = api_destination.description
api_destination_invocation_endpoint = api_destination.invocation_endpoint
api_destination_http_method = api_destination.http_method
api_destination_invocation_rate_limit_per_second = api_destination.invocation_rate_limit_per_second
api_destination_last_modified_time = api_destination.last_modified_time
api_destination_creation_time = api_destination.creation_time
api_destination_api_destination_arn = api_destination.api_destination_arn
api_destination_connection_arn = api_destination.connection_arn
api_destination_api_destination_state = api_destination.api_destination_state
```

---


### Rule

Rule resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>
         <p>If you're setting an event bus in another account as the target and that account granted
      permission to your account through an organization instead of directly by the account ID, you
      must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code>
      structure, instead of here in this parameter.</p> |
| `tags` | Vec<String> |  | <p>The list of key-value pairs to associate with the rule.</p> |
| `event_bus_name` | String |  | <p>The name or ARN of the event bus to associate with this rule. If you omit this, the
      default event bus is used.</p> |
| `state` | String |  | <p>Indicates whether the rule is enabled or disabled.</p> |
| `description` | String |  | <p>A description of the rule.</p> |
| `schedule_expression` | String |  | <p>The scheduling expression. For example, "cron(0 20 * * ? *)" or "rate(5 minutes)".</p> |
| `name` | String | ✅ | <p>The name of the rule that you are creating or updating.</p> |
| `event_pattern` | String |  | <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Events and Event
        Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the rule.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p> |
| `schedule_expression` | String | <p>The scheduling expression. For example, "cron(0 20 * * ? *)", "rate(5 minutes)".</p> |
| `event_bus_name` | String | <p>The name of the event bus associated with the rule.</p> |
| `event_pattern` | String | <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Events and Event
        Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p> |
| `created_by` | String | <p>The account ID of the user that created the rule. If you use <code>PutRule</code> to put a
      rule on an event bus in another account, the other account is the owner of the rule, and the
      rule ARN includes the account ID for that account. However, the value for
        <code>CreatedBy</code> is the account ID as the account that created the rule in the other
      account.</p> |
| `state` | String | <p>Specifies whether the rule is enabled or disabled.</p> |
| `name` | String | <p>The name of the rule.</p> |
| `managed_by` | String | <p>If this is a managed rule, created by an Amazon Web Services service on your behalf, this field displays
      the principal name of the Amazon Web Services service that created the rule.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule
rule = provider.cloudwatch_events.Rule {
    name = "value"  # <p>The name of the rule that you are creating or updating.</p>
}

# Access rule outputs
rule_id = rule.id
rule_description = rule.description
rule_role_arn = rule.role_arn
rule_schedule_expression = rule.schedule_expression
rule_event_bus_name = rule.event_bus_name
rule_event_pattern = rule.event_pattern
rule_created_by = rule.created_by
rule_state = rule.state
rule_name = rule.name
rule_managed_by = rule.managed_by
rule_arn = rule.arn
```

---


### Partner_events

PartnerEvents resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> | ✅ | <p>The list of events to write to the event bus.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partner_events
partner_events = provider.cloudwatch_events.Partner_events {
    entries = "value"  # <p>The list of events to write to the event bus.</p>
}

```

---


### Replay

Replay resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination` | String | <p>A <code>ReplayDestination</code> object that contains details about the replay.</p> |
| `description` | String | <p>The description of the replay.</p> |
| `state_reason` | String | <p>The reason that the replay is in the current state.</p> |
| `event_start_time` | String | <p>The time stamp of the first event that was last replayed from the archive.</p> |
| `event_end_time` | String | <p>The time stamp for the last event that was replayed from the archive.</p> |
| `replay_arn` | String | <p>The ARN of the replay.</p> |
| `event_source_arn` | String | <p>The ARN of the archive events were replayed from.</p> |
| `event_last_replayed_time` | String | <p>The time that the event was last replayed.</p> |
| `replay_name` | String | <p>The name of the replay.</p> |
| `replay_start_time` | String | <p>A time stamp for the time that the replay started.</p> |
| `replay_end_time` | String | <p>A time stamp for the time that the replay stopped.</p> |
| `state` | String | <p>The current state of the replay.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replay outputs
replay_id = replay.id
replay_destination = replay.destination
replay_description = replay.description
replay_state_reason = replay.state_reason
replay_event_start_time = replay.event_start_time
replay_event_end_time = replay.event_end_time
replay_replay_arn = replay.replay_arn
replay_event_source_arn = replay.event_source_arn
replay_event_last_replayed_time = replay.event_last_replayed_time
replay_replay_name = replay.replay_name
replay_replay_start_time = replay.replay_start_time
replay_replay_end_time = replay.replay_end_time
replay_state = replay.state
```

---


### Event_bus

EventBus resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Tags to associate with the event bus.</p> |
| `name` | String | ✅ | <p>The name of the new event bus. </p>
         <p>Event bus names cannot contain the / character. You can't use the name
        <code>default</code> for a custom event bus, as this name is already used for your account's
      default event bus.</p>
         <p>If this is a partner event bus, the name must exactly match the name of the partner event
      source that this event bus is matched to.</p> |
| `event_source_name` | String |  | <p>If you are creating a partner event bus, this specifies the partner event source that the
      new event bus will be matched with.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The policy that enables the external account to send events to your account.</p> |
| `name` | String | <p>The name of the event bus. Currently, this is always <code>default</code>.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the account permitted to write events to the current
      account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_bus
event_bus = provider.cloudwatch_events.Event_bus {
    name = "value"  # <p>The name of the new event bus. </p>
         <p>Event bus names cannot contain the / character. You can't use the name
        <code>default</code> for a custom event bus, as this name is already used for your account's
      default event bus.</p>
         <p>If this is a partner event bus, the name must exactly match the name of the partner event
      source that this event bus is matched to.</p>
}

# Access event_bus outputs
event_bus_id = event_bus.id
event_bus_policy = event_bus.policy
event_bus_name = event_bus.name
event_bus_arn = event_bus.arn
```

---


### Targets

Targets resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_bus_name` | String |  | <p>The name or ARN of the event bus associated with the rule. If you omit this, the default
      event bus is used.</p> |
| `targets` | Vec<String> | ✅ | <p>The targets to update or add to the rule.</p> |
| `rule` | String | ✅ | <p>The name of the rule.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create targets
targets = provider.cloudwatch_events.Targets {
    targets = "value"  # <p>The targets to update or add to the rule.</p>
    rule = "value"  # <p>The name of the rule.</p>
}

```

---


### Partner_event_source

PartnerEventSource resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the partner event source. This name must be unique and must be in the format
          <code>
               <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i>
            </code>.
      The Amazon Web Services account that wants to use this partner event source must create a partner event bus
      with a name that matches the name of the partner event source.</p> |
| `account` | String | ✅ | <p>The Amazon Web Services account ID that is permitted to create a matching partner event bus for this
      partner event source.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the event source.</p> |
| `arn` | String | <p>The ARN of the event source.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partner_event_source
partner_event_source = provider.cloudwatch_events.Partner_event_source {
    name = "value"  # <p>The name of the partner event source. This name must be unique and must be in the format
          <code>
               <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i>
            </code>.
      The Amazon Web Services account that wants to use this partner event source must create a partner event bus
      with a name that matches the name of the partner event source.</p>
    account = "value"  # <p>The Amazon Web Services account ID that is permitted to create a matching partner event bus for this
      partner event source.</p>
}

# Access partner_event_source outputs
partner_event_source_id = partner_event_source.id
partner_event_source_name = partner_event_source.name
partner_event_source_arn = partner_event_source.arn
```

---


### Event_source

EventSource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the partner event source.</p> |
| `state` | String | <p>The state of the event source. If it is ACTIVE, you have already created a matching event
      bus for this event source, and that event bus is active. If it is PENDING, either you haven't
      yet created a matching event bus, or that event bus is deactivated. If it is DELETED, you have
      created a matching event bus, but the event source has since been deleted.</p> |
| `created_by` | String | <p>The name of the SaaS partner that created the event source.</p> |
| `expiration_time` | String | <p>The date and time that the event source will expire if you do not create a matching event
      bus.</p> |
| `creation_time` | String | <p>The date and time that the event source was created.</p> |
| `arn` | String | <p>The ARN of the partner event source.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_source outputs
event_source_id = event_source.id
event_source_name = event_source.name
event_source_state = event_source.state
event_source_created_by = event_source.created_by
event_source_expiration_time = event_source.expiration_time
event_source_creation_time = event_source.creation_time
event_source_arn = event_source.arn
```

---


### Archive

Archive resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_days` | i64 |  | <p>The number of days to retain events for. Default value is 0. If set to 0, events are
      retained indefinitely</p> |
| `event_pattern` | String |  | <p>An event pattern to use to filter events sent to the archive.</p> |
| `description` | String |  | <p>A description for the archive.</p> |
| `archive_name` | String | ✅ | <p>The name for the archive to create.</p> |
| `event_source_arn` | String | ✅ | <p>The ARN of the event bus that sends events to the archive.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | <p>The state of the archive.</p> |
| `size_bytes` | i64 | <p>The size of the archive in bytes.</p> |
| `archive_name` | String | <p>The name of the archive.</p> |
| `event_source_arn` | String | <p>The ARN of the event source associated with the archive.</p> |
| `event_count` | i64 | <p>The number of events in the archive.</p> |
| `event_pattern` | String | <p>The event pattern used to filter events sent to the archive.</p> |
| `retention_days` | i64 | <p>The number of days to retain events for in the archive.</p> |
| `description` | String | <p>The description of the archive.</p> |
| `creation_time` | String | <p>The time at which the archive was created.</p> |
| `archive_arn` | String | <p>The ARN of the archive.</p> |
| `state_reason` | String | <p>The reason that the archive is in the state.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create archive
archive = provider.cloudwatch_events.Archive {
    archive_name = "value"  # <p>The name for the archive to create.</p>
    event_source_arn = "value"  # <p>The ARN of the event bus that sends events to the archive.</p>
}

# Access archive outputs
archive_id = archive.id
archive_state = archive.state
archive_size_bytes = archive.size_bytes
archive_archive_name = archive.archive_name
archive_event_source_arn = archive.event_source_arn
archive_event_count = archive.event_count
archive_event_pattern = archive.event_pattern
archive_retention_days = archive.retention_days
archive_description = archive.description
archive_creation_time = archive.creation_time
archive_archive_arn = archive.archive_arn
archive_state_reason = archive.state_reason
```

---


### Permission

Permission resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | <p>The action that you are enabling the other account to perform.</p> |
| `event_bus_name` | String |  | <p>The name of the event bus associated with the rule. If you omit this, the default event
      bus is used.</p> |
| `principal` | String |  | <p>The 12-digit Amazon Web Services account ID that you are permitting to put events to your default event
      bus. Specify "*" to permit any account to put events to your default event bus.</p>
         <p>If you specify "*" without specifying <code>Condition</code>, avoid creating rules that
      may match undesirable events. To create more secure rules, make sure that the event pattern
      for each rule contains an <code>account</code> field with a specific account ID from which to
      receive events. Rules with an account field do not match any events sent from other
      accounts.</p> |
| `condition` | String |  | <p>This parameter enables you to limit the permission to accounts that fulfill a certain
      condition, such as being a member of a certain Amazon Web Services organization. For more information about
      Amazon Web Services Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html">What Is Amazon Web Services 
        Organizations</a> in the <i>Amazon Web Services Organizations User Guide</i>.</p>
         <p>If you specify <code>Condition</code> with an Amazon Web Services organization ID, and specify "*" as the
      value for <code>Principal</code>, you grant permission to all the accounts in the named
      organization.</p>
         <p>The <code>Condition</code> is a JSON string which must contain <code>Type</code>,
        <code>Key</code>, and <code>Value</code> fields.</p> |
| `statement_id` | String |  | <p>An identifier string for the external account that you are granting permissions to. If you
      later want to revoke the permission for this external account, specify this
      <code>StatementId</code> when you run <a href="https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_RemovePermission.html">RemovePermission</a>.</p> |
| `policy` | String |  | <p>A JSON string that describes the permission policy statement. You can include a
        <code>Policy</code> parameter in the request instead of using the <code>StatementId</code>,
        <code>Action</code>, <code>Principal</code>, or <code>Condition</code> parameters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission
permission = provider.cloudwatch_events.Permission {
}

```

---


### Events

Events resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> | ✅ | <p>The entry that defines an event in your system. You can specify several parameters for the
      entry such as the source and type of the event, resources associated with the event, and so
      on.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create events
events = provider.cloudwatch_events.Events {
    entries = "value"  # <p>The entry that defines an event in your system. You can specify several parameters for the
      entry such as the source and type of the event, resources associated with the event, and so
      on.</p>
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

# Create multiple connection resources
connection_0 = provider.cloudwatch_events.Connection {
    authorization_type = "value-0"
    auth_parameters = "value-0"
    name = "value-0"
}
connection_1 = provider.cloudwatch_events.Connection {
    authorization_type = "value-1"
    auth_parameters = "value-1"
    name = "value-1"
}
connection_2 = provider.cloudwatch_events.Connection {
    authorization_type = "value-2"
    auth_parameters = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    connection = provider.cloudwatch_events.Connection {
        authorization_type = "production-value"
        auth_parameters = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Cloudwatch_events Documentation](https://docs.aws.amazon.com/cloudwatch_events/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
