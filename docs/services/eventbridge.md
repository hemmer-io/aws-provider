# Eventbridge Service



**Resources**: 13

---

## Overview

The eventbridge service provides access to 13 resource types:

- [Replay](#replay) [R]
- [Events](#events) [C]
- [Endpoint](#endpoint) [CRUD]
- [Connection](#connection) [CRUD]
- [Event_bus](#event_bus) [CRUD]
- [Event_source](#event_source) [R]
- [Partner_events](#partner_events) [C]
- [Partner_event_source](#partner_event_source) [CRD]
- [Permission](#permission) [C]
- [Targets](#targets) [C]
- [Rule](#rule) [CRD]
- [Api_destination](#api_destination) [CRUD]
- [Archive](#archive) [CRUD]

---

## Resources


### Replay

Replay resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replay_start_time` | String | <p>A time stamp for the time that the replay started.</p> |
| `event_source_arn` | String | <p>The ARN of the archive events were replayed from.</p> |
| `replay_name` | String | <p>The name of the replay.</p> |
| `destination` | String | <p>A <code>ReplayDestination</code> object that contains details about the replay.</p> |
| `description` | String | <p>The description of the replay.</p> |
| `state` | String | <p>The current state of the replay.</p> |
| `state_reason` | String | <p>The reason that the replay is in the current state.</p> |
| `replay_arn` | String | <p>The ARN of the replay.</p> |
| `event_end_time` | String | <p>The time stamp for the last event that was replayed from the archive.</p> |
| `event_last_replayed_time` | String | <p>The time that the event was last replayed.</p> |
| `replay_end_time` | String | <p>A time stamp for the time that the replay stopped.</p> |
| `event_start_time` | String | <p>The time stamp of the first event that was last replayed from the archive.</p> |


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
replay_replay_start_time = replay.replay_start_time
replay_event_source_arn = replay.event_source_arn
replay_replay_name = replay.replay_name
replay_destination = replay.destination
replay_description = replay.description
replay_state = replay.state
replay_state_reason = replay.state_reason
replay_replay_arn = replay.replay_arn
replay_event_end_time = replay.event_end_time
replay_event_last_replayed_time = replay.event_last_replayed_time
replay_replay_end_time = replay.replay_end_time
replay_event_start_time = replay.event_start_time
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
| `endpoint_id` | String |  | <p>The URL subdomain of the endpoint. For example, if the URL for Endpoint is
      https://abcde.veo.endpoints.event.amazonaws.com, then the EndpointId is
      <code>abcde.veo</code>.</p>
         <important>
            <p>When using Java, you must include <code>auth-crt</code> on the class path.</p>
         </important> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create events
events = provider.eventbridge.Events {
    entries = "value"  # <p>The entry that defines an event in your system. You can specify several parameters for the
      entry such as the source and type of the event, resources associated with the event, and so
      on.</p>
}

```

---


### Endpoint

Endpoint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_arn` | String |  | <p>The ARN of the role used for replication.</p> |
| `replication_config` | String |  | <p>Enable or disable event replication. The default state is <code>ENABLED</code> which means
      you must supply a <code>RoleArn</code>. If you don't have a <code>RoleArn</code> or you don't
      want event replication enabled, set the state to <code>DISABLED</code>.</p> |
| `event_buses` | Vec<String> | ✅ | <p>Define the event buses used. </p>
         <important>
            <p>The names of the event buses must be identical in each Region.</p>
         </important> |
| `name` | String | ✅ | <p>The name of the global endpoint. For example,
        <code>"Name":"us-east-2-custom_bus_A-endpoint"</code>.</p> |
| `description` | String |  | <p>A description of the global endpoint.</p> |
| `routing_config` | String | ✅ | <p>Configure the routing policy, including the health check and secondary Region..</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_config` | String | <p>Whether replication is enabled or disabled for the endpoint you asked for information
      about.</p> |
| `state` | String | <p>The current state of the endpoint you asked for information about.</p> |
| `state_reason` | String | <p>The reason the endpoint you asked for information about is in its current state.</p> |
| `creation_time` | String | <p>The time the endpoint you asked for information about was created.</p> |
| `endpoint_id` | String | <p>The ID of the endpoint you asked for information about.</p> |
| `name` | String | <p>The name of the endpoint you asked for information about.</p> |
| `last_modified_time` | String | <p>The last time the endpoint you asked for information about was modified.</p> |
| `arn` | String | <p>The ARN of the endpoint you asked for information about.</p> |
| `event_buses` | Vec<String> | <p>The event buses being used by the endpoint you asked for information about.</p> |
| `description` | String | <p>The description of the endpoint you asked for information about.</p> |
| `routing_config` | String | <p>The routing configuration of the endpoint you asked for information about.</p> |
| `role_arn` | String | <p>The ARN of the role used by the endpoint you asked for information about.</p> |
| `endpoint_url` | String | <p>The URL of the endpoint you asked for information about.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint
endpoint = provider.eventbridge.Endpoint {
    event_buses = "value"  # <p>Define the event buses used. </p>
         <important>
            <p>The names of the event buses must be identical in each Region.</p>
         </important>
    name = "value"  # <p>The name of the global endpoint. For example,
        <code>"Name":"us-east-2-custom_bus_A-endpoint"</code>.</p>
    routing_config = "value"  # <p>Configure the routing policy, including the health check and secondary Region..</p>
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_replication_config = endpoint.replication_config
endpoint_state = endpoint.state
endpoint_state_reason = endpoint.state_reason
endpoint_creation_time = endpoint.creation_time
endpoint_endpoint_id = endpoint.endpoint_id
endpoint_name = endpoint.name
endpoint_last_modified_time = endpoint.last_modified_time
endpoint_arn = endpoint.arn
endpoint_event_buses = endpoint.event_buses
endpoint_description = endpoint.description
endpoint_routing_config = endpoint.routing_config
endpoint_role_arn = endpoint.role_arn
endpoint_endpoint_url = endpoint.endpoint_url
```

---


### Connection

Connection resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invocation_connectivity_parameters` | String |  | <p>For connections to private APIs, the parameters to use for invoking the API.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/connection-private.html">Connecting to private APIs</a> in the <i>
               <i>Amazon EventBridge User Guide</i>
            </i>.</p> |
| `name` | String | ✅ | <p>The name for the connection to create.</p> |
| `authorization_type` | String | ✅ | <p>The type of authorization to use for the connection.</p>
         <note>
            <p>OAUTH tokens are refreshed when a 401 or 407 response is returned.</p>
         </note> |
| `description` | String |  | <p>A description for the connection to create.</p> |
| `kms_key_identifier` | String |  | <p>The identifier of the KMS
      customer managed key for EventBridge to use, if you choose to use a customer managed key to encrypt this connection. The identifier can be the key 
      Amazon Resource Name (ARN), KeyId, key alias, or key alias ARN.</p>
         <p>If you do not specify a customer managed key identifier, EventBridge uses an
        Amazon Web Services owned key to encrypt the connection.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html">Identify and view keys</a> in the <i>Key Management Service
                                Developer Guide</i>. </p> |
| `auth_parameters` | String | ✅ | <p>The
      authorization parameters to use to authorize with the endpoint. </p>
         <p>You must include only authorization parameters for the <code>AuthorizationType</code> you specify.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auth_parameters` | String | <p>The parameters to use for authorization for the connection.</p> |
| `creation_time` | String | <p>A time stamp for the time that the connection was created.</p> |
| `state_reason` | String | <p>The reason that the connection is in the current connection state.</p> |
| `secret_arn` | String | <p>The ARN of the secret created from the authorization parameters specified for the
      connection.</p> |
| `connection_state` | String | <p>The state of the connection retrieved.</p> |
| `last_modified_time` | String | <p>A time stamp for the time that the connection was last modified.</p> |
| `connection_arn` | String | <p>The ARN of the connection retrieved.</p> |
| `kms_key_identifier` | String | <p>The identifier of the KMS
      customer managed key for EventBridge to use to encrypt the connection, if one has been specified.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/encryption-connections.html">Encrypting connections</a> in the <i>Amazon EventBridge User Guide</i>.</p> |
| `description` | String | <p>The description for the connection retrieved.</p> |
| `invocation_connectivity_parameters` | String | <p>For connections to private APIs The parameters EventBridge uses to invoke the resource
      endpoint.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/connection-private.html">Connecting to private APIs</a> in the <i>
               <i>Amazon EventBridge User Guide</i>
            </i>.</p> |
| `last_authorized_time` | String | <p>A time stamp for the time that the connection was last authorized.</p> |
| `authorization_type` | String | <p>The type of authorization specified for the connection.</p> |
| `name` | String | <p>The name of the connection retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.eventbridge.Connection {
    name = "value"  # <p>The name for the connection to create.</p>
    authorization_type = "value"  # <p>The type of authorization to use for the connection.</p>
         <note>
            <p>OAUTH tokens are refreshed when a 401 or 407 response is returned.</p>
         </note>
    auth_parameters = "value"  # <p>The
      authorization parameters to use to authorize with the endpoint. </p>
         <p>You must include only authorization parameters for the <code>AuthorizationType</code> you specify.</p>
}

# Access connection outputs
connection_id = connection.id
connection_auth_parameters = connection.auth_parameters
connection_creation_time = connection.creation_time
connection_state_reason = connection.state_reason
connection_secret_arn = connection.secret_arn
connection_connection_state = connection.connection_state
connection_last_modified_time = connection.last_modified_time
connection_connection_arn = connection.connection_arn
connection_kms_key_identifier = connection.kms_key_identifier
connection_description = connection.description
connection_invocation_connectivity_parameters = connection.invocation_connectivity_parameters
connection_last_authorized_time = connection.last_authorized_time
connection_authorization_type = connection.authorization_type
connection_name = connection.name
```

---


### Event_bus

EventBus resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_source_name` | String |  | <p>If you are creating a partner event bus, this specifies the partner event source that the
      new event bus will be matched with.</p> |
| `name` | String | ✅ | <p>The name of the new event bus. </p>
         <p>Custom event bus names can't contain the <code>/</code> character, but you can use the
        <code>/</code> character in partner event bus names. In addition, for partner event buses,
      the name must exactly match the name of the partner event source that this event bus is
      matched to.</p>
         <p>You can't use the name <code>default</code> for a custom event bus, as this name is
      already used for your account's default event bus.</p> |
| `dead_letter_config` | String |  |  |
| `description` | String |  | <p>The event bus description.</p> |
| `log_config` | String |  | <p>The logging configuration settings for the event bus.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eb-event-bus-logs.html">Configuring logs for event buses</a> in the <i>EventBridge User Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>Tags to associate with the event bus.</p> |
| `kms_key_identifier` | String |  | <p>The identifier of the KMS
      customer managed key for EventBridge to use, if you choose to use a customer managed key to encrypt events on this event bus. The identifier can be the key 
      Amazon Resource Name (ARN), KeyId, key alias, or key alias ARN.</p>
         <p>If you do not specify a customer managed key identifier, EventBridge uses an
        Amazon Web Services owned key to encrypt events on the event bus.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html">Identify and view keys</a> in the <i>Key Management Service
                                Developer Guide</i>. </p>
         <note>
            <p>Schema discovery is not supported for event buses encrypted using a
        customer managed key. EventBridge returns an error if: </p>
            <ul>
               <li>
                  <p>You call <code>
                        <a href="https://docs.aws.amazon.com/eventbridge/latest/schema-reference/v1-discoverers.html#CreateDiscoverer">CreateDiscoverer</a>
                     </code> on an event bus set to use a customer managed key for encryption.</p>
               </li>
               <li>
                  <p>You call <code>
                        <a href="https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_UpdatedEventBus.html">UpdatedEventBus</a>
                     </code> to set a customer managed key on an event bus with schema discovery enabled.</p>
               </li>
            </ul>
            <p>To enable schema discovery on an event bus, choose to
        use an Amazon Web Services owned key. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-encryption-event-bus-cmkey.html">Encrypting events</a> in the <i>Amazon EventBridge User Guide</i>.</p>
         </note>
         <important>
            <p>If you have specified that EventBridge use a customer managed key for encrypting the source event bus, we strongly recommend you also specify a 
        customer managed key for any archives for the event bus as well. </p>
            <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/encryption-archives.html">Encrypting archives</a> in the <i>Amazon EventBridge User Guide</i>.</p>
         </important> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The event bus description.</p> |
| `policy` | String | <p>The policy that enables the external account to send events to your account.</p> |
| `dead_letter_config` | String |  |
| `name` | String | <p>The name of the event bus. Currently, this is always <code>default</code>.</p> |
| `creation_time` | String | <p>The time the event bus was created.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the account permitted to write events to the current account.</p> |
| `last_modified_time` | String | <p>The time the event bus was last modified.</p> |
| `kms_key_identifier` | String | <p>The identifier of the KMS
      customer managed key for EventBridge to use to encrypt events on this event bus, if one has been specified.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-encryption.html">Data encryption in EventBridge</a> in the <i>Amazon EventBridge User Guide</i>.</p> |
| `log_config` | String | <p>The logging configuration settings for the event bus.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eb-event-bus-logs.html">Configuring logs for event buses</a> in the <i>EventBridge User Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_bus
event_bus = provider.eventbridge.Event_bus {
    name = "value"  # <p>The name of the new event bus. </p>
         <p>Custom event bus names can't contain the <code>/</code> character, but you can use the
        <code>/</code> character in partner event bus names. In addition, for partner event buses,
      the name must exactly match the name of the partner event source that this event bus is
      matched to.</p>
         <p>You can't use the name <code>default</code> for a custom event bus, as this name is
      already used for your account's default event bus.</p>
}

# Access event_bus outputs
event_bus_id = event_bus.id
event_bus_description = event_bus.description
event_bus_policy = event_bus.policy
event_bus_dead_letter_config = event_bus.dead_letter_config
event_bus_name = event_bus.name
event_bus_creation_time = event_bus.creation_time
event_bus_arn = event_bus.arn
event_bus_last_modified_time = event_bus.last_modified_time
event_bus_kms_key_identifier = event_bus.kms_key_identifier
event_bus_log_config = event_bus.log_config
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
| `creation_time` | String | <p>The date and time that the event source was created.</p> |
| `expiration_time` | String | <p>The date and time that the event source will expire if you do not create a matching event
      bus.</p> |
| `created_by` | String | <p>The name of the SaaS partner that created the event source.</p> |
| `arn` | String | <p>The ARN of the partner event source.</p> |
| `name` | String | <p>The name of the partner event source.</p> |
| `state` | String | <p>The state of the event source. If it is ACTIVE, you have already created a matching event
      bus for this event source, and that event bus is active. If it is PENDING, either you haven't
      yet created a matching event bus, or that event bus is deactivated. If it is DELETED, you have
      created a matching event bus, but the event source has since been deleted.</p> |


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
event_source_creation_time = event_source.creation_time
event_source_expiration_time = event_source.expiration_time
event_source_created_by = event_source.created_by
event_source_arn = event_source.arn
event_source_name = event_source.name
event_source_state = event_source.state
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
partner_events = provider.eventbridge.Partner_events {
    entries = "value"  # <p>The list of events to write to the event bus.</p>
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
      The Amazon Web Services account that wants to use this partner event source must create a
      partner event bus with a name that matches the name of the partner event source.</p> |
| `account` | String | ✅ | <p>The Amazon Web Services account ID that is permitted to create a matching partner event bus
      for this partner event source.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The ARN of the event source.</p> |
| `name` | String | <p>The name of the event source.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partner_event_source
partner_event_source = provider.eventbridge.Partner_event_source {
    name = "value"  # <p>The name of the partner event source. This name must be unique and must be in the format
          <code>
               <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i>
            </code>.
      The Amazon Web Services account that wants to use this partner event source must create a
      partner event bus with a name that matches the name of the partner event source.</p>
    account = "value"  # <p>The Amazon Web Services account ID that is permitted to create a matching partner event bus
      for this partner event source.</p>
}

# Access partner_event_source outputs
partner_event_source_id = partner_event_source.id
partner_event_source_arn = partner_event_source.arn
partner_event_source_name = partner_event_source.name
```

---


### Permission

Permission resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `statement_id` | String |  | <p>An identifier string for the external account that you are granting permissions to. If you
      later want to revoke the permission for this external account, specify this
        <code>StatementId</code> when you run <a href="https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_RemovePermission.html">RemovePermission</a>.</p>
         <note>
            <p>Each <code>StatementId</code> must be unique.</p>
         </note> |
| `action` | String |  | <p>The action that you are enabling the other account to perform.</p> |
| `condition` | String |  | <p>This parameter enables you to limit the permission to accounts that fulfill a certain
      condition, such as being a member of a certain Amazon Web Services organization. For more
      information about Amazon Web Services Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html">What Is Amazon Web Services
        Organizations</a> in the <i>Amazon Web Services Organizations User
        Guide</i>.</p>
         <p>If you specify <code>Condition</code> with an Amazon Web Services organization ID, and
      specify "*" as the value for <code>Principal</code>, you grant permission to all the accounts
      in the named organization.</p>
         <p>The <code>Condition</code> is a JSON string which must contain <code>Type</code>,
        <code>Key</code>, and <code>Value</code> fields.</p> |
| `event_bus_name` | String |  | <p>The name of the event bus associated with the rule. If you omit this, the default event
      bus is used.</p> |
| `policy` | String |  | <p>A JSON string that describes the permission policy statement. You can include a
        <code>Policy</code> parameter in the request instead of using the <code>StatementId</code>,
        <code>Action</code>, <code>Principal</code>, or <code>Condition</code> parameters.</p> |
| `principal` | String |  | <p>The 12-digit Amazon Web Services account ID that you are permitting to put events to your
      default event bus. Specify "*" to permit any account to put events to your default event
      bus.</p>
         <p>If you specify "*" without specifying <code>Condition</code>, avoid creating rules that
      may match undesirable events. To create more secure rules, make sure that the event pattern
      for each rule contains an <code>account</code> field with a specific account ID from which to
      receive events. Rules with an account field do not match any events sent from other
      accounts.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission
permission = provider.eventbridge.Permission {
}

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
| `rule` | String | ✅ | <p>The name of the rule.</p> |
| `targets` | Vec<String> | ✅ | <p>The targets to update or add to the rule.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create targets
targets = provider.eventbridge.Targets {
    rule = "value"  # <p>The name of the rule.</p>
    targets = "value"  # <p>The targets to update or add to the rule.</p>
}

```

---


### Rule

Rule resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | <p>The state of the rule.</p>
         <p>Valid values include:</p>
         <ul>
            <li>
               <p>
                  <code>DISABLED</code>: The rule is disabled. EventBridge does not match any events against the rule.</p>
            </li>
            <li>
               <p>
                  <code>ENABLED</code>: The rule is enabled. 
          EventBridge matches events against the rule, <i>except</i> for Amazon Web Services management events delivered through CloudTrail.</p>
            </li>
            <li>
               <p>
                  <code>ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS</code>: The rule is enabled for all
        events, including Amazon Web Services management events delivered through CloudTrail.</p>
               <p>Management events provide visibility into management operations that are performed on
          resources in your Amazon Web Services account. These are also known as control plane
          operations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-events-with-cloudtrail.html#logging-management-events">Logging management events</a> in the <i>CloudTrail User
            Guide</i>, and <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-cloudtrail">Filtering management events from Amazon Web Services services</a> in the
            <i>
                     <i>Amazon EventBridge User Guide</i>
                  </i>.</p>
               <p>This value is only valid for rules on the <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-what-is-how-it-works-concepts.html#eb-bus-concepts-buses">default</a> event bus 
          or <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-create-event-bus.html">custom event buses</a>. 
          It does not apply to <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-saas.html">partner event buses</a>.</p>
            </li>
         </ul> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>
         <p>If you're setting an event bus in another account as the target and that account granted
      permission to your account through an organization instead of directly by the account ID, you
      must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code>
      structure, instead of here in this parameter.</p> |
| `event_bus_name` | String |  | <p>The name or ARN of the event bus to associate with this rule. If you omit this, the
      default event bus is used.</p> |
| `schedule_expression` | String |  | <p>The scheduling expression. For example, "cron(0 20 * * ? *)" or "rate(5 minutes)".</p> |
| `description` | String |  | <p>A description of the rule.</p> |
| `event_pattern` | String |  | <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html">Amazon EventBridge event
        patterns</a> in the <i>
               <i>Amazon EventBridge User Guide</i>
            </i>.</p> |
| `tags` | Vec<String> |  | <p>The list of key-value pairs to associate with the rule.</p> |
| `name` | String | ✅ | <p>The name of the rule that you are creating or updating.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | <p>Specifies whether the rule is enabled or disabled.</p> |
| `managed_by` | String | <p>If this is a managed rule, created by an Amazon Web Services service on your behalf, this
      field displays the principal name of the Amazon Web Services service that created the
      rule.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the rule.</p> |
| `event_bus_name` | String | <p>The name of the event bus associated with the rule.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p> |
| `description` | String | <p>The description of the rule.</p> |
| `event_pattern` | String | <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Events and Event
        Patterns</a> in the <i>
               <i>Amazon EventBridge User Guide</i>
            </i>.</p> |
| `created_by` | String | <p>The account ID of the user that created the rule. If you use <code>PutRule</code> to put a
      rule on an event bus in another account, the other account is the owner of the rule, and the
      rule ARN includes the account ID for that account. However, the value for
        <code>CreatedBy</code> is the account ID as the account that created the rule in the other
      account.</p> |
| `name` | String | <p>The name of the rule.</p> |
| `schedule_expression` | String | <p>The scheduling expression. For example, "cron(0 20 * * ? *)", "rate(5 minutes)".</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule
rule = provider.eventbridge.Rule {
    name = "value"  # <p>The name of the rule that you are creating or updating.</p>
}

# Access rule outputs
rule_id = rule.id
rule_state = rule.state
rule_managed_by = rule.managed_by
rule_arn = rule.arn
rule_event_bus_name = rule.event_bus_name
rule_role_arn = rule.role_arn
rule_description = rule.description
rule_event_pattern = rule.event_pattern
rule_created_by = rule.created_by
rule_name = rule.name
rule_schedule_expression = rule.schedule_expression
```

---


### Api_destination

ApiDestination resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_arn` | String | ✅ | <p>The ARN of the connection to use for the API destination. The destination endpoint must
      support the authorization type specified for the connection.</p> |
| `invocation_endpoint` | String | ✅ | <p>The URL to the HTTP invocation endpoint for the API destination.</p> |
| `name` | String | ✅ | <p>The name for the API destination to create.</p> |
| `http_method` | String | ✅ | <p>The method to use for the request to the HTTP invocation endpoint.</p> |
| `invocation_rate_limit_per_second` | i64 |  | <p>The maximum number of requests per second to send to the HTTP invocation endpoint.</p> |
| `description` | String |  | <p>A description for the API destination to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description for the API destination retrieved.</p> |
| `connection_arn` | String | <p>The ARN of the connection specified for the API destination retrieved.</p> |
| `http_method` | String | <p>The method to use to connect to the HTTP endpoint.</p> |
| `name` | String | <p>The name of the API destination retrieved.</p> |
| `invocation_endpoint` | String | <p>The URL to use to connect to the HTTP endpoint.</p> |
| `invocation_rate_limit_per_second` | i64 | <p>The maximum number of invocations per second to specified for the API destination. Note
      that if you set the invocation rate maximum to a value lower the rate necessary to send all
      events received on to the destination HTTP endpoint, some events may not be delivered within
      the 24-hour retry window. If you plan to set the rate lower than the rate necessary to deliver
      all events, consider using a dead-letter queue to catch events that are not delivered within
      24 hours.</p> |
| `creation_time` | String | <p>A time stamp for the time that the API destination was created.</p> |
| `api_destination_state` | String | <p>The state of the API destination retrieved.</p> |
| `last_modified_time` | String | <p>A time stamp for the time that the API destination was last modified.</p> |
| `api_destination_arn` | String | <p>The ARN of the API destination retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api_destination
api_destination = provider.eventbridge.Api_destination {
    connection_arn = "value"  # <p>The ARN of the connection to use for the API destination. The destination endpoint must
      support the authorization type specified for the connection.</p>
    invocation_endpoint = "value"  # <p>The URL to the HTTP invocation endpoint for the API destination.</p>
    name = "value"  # <p>The name for the API destination to create.</p>
    http_method = "value"  # <p>The method to use for the request to the HTTP invocation endpoint.</p>
}

# Access api_destination outputs
api_destination_id = api_destination.id
api_destination_description = api_destination.description
api_destination_connection_arn = api_destination.connection_arn
api_destination_http_method = api_destination.http_method
api_destination_name = api_destination.name
api_destination_invocation_endpoint = api_destination.invocation_endpoint
api_destination_invocation_rate_limit_per_second = api_destination.invocation_rate_limit_per_second
api_destination_creation_time = api_destination.creation_time
api_destination_api_destination_state = api_destination.api_destination_state
api_destination_last_modified_time = api_destination.last_modified_time
api_destination_api_destination_arn = api_destination.api_destination_arn
```

---


### Archive

Archive resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_pattern` | String |  | <p>An event pattern to use to filter events sent to the archive.</p> |
| `description` | String |  | <p>A description for the archive.</p> |
| `kms_key_identifier` | String |  | <p>The identifier of the KMS
      customer managed key for EventBridge to use, if you choose to use a customer managed key to encrypt this archive. The identifier can be the key 
      Amazon Resource Name (ARN), KeyId, key alias, or key alias ARN.</p>
         <p>If you do not specify a customer managed key identifier, EventBridge uses an
        Amazon Web Services owned key to encrypt the archive.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html">Identify and view keys</a> in the <i>Key Management Service
                                Developer Guide</i>. </p>
         <important>
            <p>If you have specified that EventBridge use a customer managed key for encrypting the source event bus, we strongly recommend you also specify a 
        customer managed key for any archives for the event bus as well. </p>
            <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/encryption-archives.html">Encrypting archives</a> in the <i>Amazon EventBridge User Guide</i>.</p>
         </important> |
| `event_source_arn` | String | ✅ | <p>The ARN of the event bus that sends events to the archive.</p> |
| `archive_name` | String | ✅ | <p>The name for the archive to create.</p> |
| `retention_days` | i64 |  | <p>The number of days to retain events for. Default value is 0. If set to 0, events are
      retained indefinitely</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `archive_arn` | String | <p>The ARN of the archive.</p> |
| `size_bytes` | i64 | <p>The size of the archive in bytes.</p> |
| `event_pattern` | String | <p>The event pattern used to filter events sent to the archive.</p> |
| `state` | String | <p>The state of the archive.</p> |
| `retention_days` | i64 | <p>The number of days to retain events for in the archive.</p> |
| `event_count` | i64 | <p>The number of events in the archive.</p> |
| `creation_time` | String | <p>The time at which the archive was created.</p> |
| `state_reason` | String | <p>The reason that the archive is in the state.</p> |
| `archive_name` | String | <p>The name of the archive.</p> |
| `description` | String | <p>The description of the archive.</p> |
| `event_source_arn` | String | <p>The ARN of the event source associated with the archive.</p> |
| `kms_key_identifier` | String | <p>The identifier of the KMS
      customer managed key for EventBridge to use to encrypt this archive, if one has been specified.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/encryption-archives.html">Encrypting archives</a> in the <i>Amazon EventBridge User Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create archive
archive = provider.eventbridge.Archive {
    event_source_arn = "value"  # <p>The ARN of the event bus that sends events to the archive.</p>
    archive_name = "value"  # <p>The name for the archive to create.</p>
}

# Access archive outputs
archive_id = archive.id
archive_archive_arn = archive.archive_arn
archive_size_bytes = archive.size_bytes
archive_event_pattern = archive.event_pattern
archive_state = archive.state
archive_retention_days = archive.retention_days
archive_event_count = archive.event_count
archive_creation_time = archive.creation_time
archive_state_reason = archive.state_reason
archive_archive_name = archive.archive_name
archive_description = archive.description
archive_event_source_arn = archive.event_source_arn
archive_kms_key_identifier = archive.kms_key_identifier
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple replay resources
replay_0 = provider.eventbridge.Replay {
}
replay_1 = provider.eventbridge.Replay {
}
replay_2 = provider.eventbridge.Replay {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    replay = provider.eventbridge.Replay {
    }
```

---

## Related Documentation

- [AWS Eventbridge Documentation](https://docs.aws.amazon.com/eventbridge/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
