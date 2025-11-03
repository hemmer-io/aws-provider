# Chime_sdk_messaging Service



**Resources**: 14

---

## Overview

The chime_sdk_messaging service provides access to 14 resource types:

- [Channel_ban](#channel_ban) [CRD]
- [Channel_message](#channel_message) [RUD]
- [Channel_moderated_by_app_instance_user](#channel_moderated_by_app_instance_user) [R]
- [Channel_read_marker](#channel_read_marker) [U]
- [Channel_expiration_settings](#channel_expiration_settings) [C]
- [Messaging_streaming_configurations](#messaging_streaming_configurations) [CRD]
- [Channel_moderator](#channel_moderator) [CRD]
- [Channel_membership_preferences](#channel_membership_preferences) [CR]
- [Channel_message_status](#channel_message_status) [R]
- [Messaging_session_endpoint](#messaging_session_endpoint) [R]
- [Channel](#channel) [CRUD]
- [Channel_flow](#channel_flow) [CRUD]
- [Channel_membership](#channel_membership) [CRD]
- [Channel_membership_for_app_instance_user](#channel_membership_for_app_instance_user) [R]

---

## Resources


### Channel_ban

ChannelBan resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_arn` | String | ✅ | <p>The <code>AppInstanceUserArn</code> of the member being banned.</p> |
| `chime_bearer` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p> |
| `channel_arn` | String | ✅ | <p>The ARN of the ban request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_ban` | String | <p>The details of the ban.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_ban
channel_ban = provider.chime_sdk_messaging.Channel_ban {
    member_arn = "value"  # <p>The <code>AppInstanceUserArn</code> of the member being banned.</p>
    chime_bearer = "value"  # <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    channel_arn = "value"  # <p>The ARN of the ban request.</p>
}

# Access channel_ban outputs
channel_ban_id = channel_ban.id
channel_ban_channel_ban = channel_ban.channel_ban
```

---


### Channel_message

ChannelMessage resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message_id` | String | ✅ | <p>The ID string of the message being updated.</p> |
| `sub_channel_id` | String |  | <p>The ID of the SubChannel in the request.</p>
         <note>
            <p>Only required when updating messages in a SubChannel that the user belongs to.</p>
         </note> |
| `content_type` | String |  | <p>The content type of the channel message.</p> |
| `metadata` | String |  | <p>The metadata of the message being updated.</p> |
| `chime_bearer` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> 
         that makes the API call.</p> |
| `content` | String | ✅ | <p>The content of the channel message. </p> |
| `channel_arn` | String | ✅ | <p>The ARN of the channel.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_message` | String | <p>The details of and content in the message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access channel_message outputs
channel_message_id = channel_message.id
channel_message_channel_message = channel_message.channel_message
```

---


### Channel_moderated_by_app_instance_user

ChannelModeratedByAppInstanceUser resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel` | String | <p>The moderated channel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access channel_moderated_by_app_instance_user outputs
channel_moderated_by_app_instance_user_id = channel_moderated_by_app_instance_user.id
channel_moderated_by_app_instance_user_channel = channel_moderated_by_app_instance_user.channel
```

---


### Channel_read_marker

ChannelReadMarker resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `chime_bearer` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> 
         that makes the API call.</p> |
| `channel_arn` | String | ✅ | <p>The ARN of the channel.</p> |



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


### Channel_expiration_settings

ChannelExpirationSettings resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `chime_bearer` | String |  | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p> |
| `channel_arn` | String | ✅ | <p>The ARN of the channel.</p> |
| `expiration_settings` | String |  | <p>Settings that control the interval after which a channel is deleted.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_expiration_settings
channel_expiration_settings = provider.chime_sdk_messaging.Channel_expiration_settings {
    channel_arn = "value"  # <p>The ARN of the channel.</p>
}

```

---


### Messaging_streaming_configurations

MessagingStreamingConfigurations resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_instance_arn` | String | ✅ | <p>The ARN of the streaming configuration.</p> |
| `streaming_configurations` | Vec<String> | ✅ | <p>The streaming configurations.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `streaming_configurations` | Vec<String> | <p>The streaming settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create messaging_streaming_configurations
messaging_streaming_configurations = provider.chime_sdk_messaging.Messaging_streaming_configurations {
    app_instance_arn = "value"  # <p>The ARN of the streaming configuration.</p>
    streaming_configurations = "value"  # <p>The streaming configurations.</p>
}

# Access messaging_streaming_configurations outputs
messaging_streaming_configurations_id = messaging_streaming_configurations.id
messaging_streaming_configurations_streaming_configurations = messaging_streaming_configurations.streaming_configurations
```

---


### Channel_moderator

ChannelModerator resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `channel_arn` | String | ✅ | <p>The ARN of the channel.</p> |
| `channel_moderator_arn` | String | ✅ | <p>The <code>AppInstanceUserArn</code> of the moderator.</p> |
| `chime_bearer` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> 
         that makes the API call.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_moderator` | String | <p>The details of the channel moderator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_moderator
channel_moderator = provider.chime_sdk_messaging.Channel_moderator {
    channel_arn = "value"  # <p>The ARN of the channel.</p>
    channel_moderator_arn = "value"  # <p>The <code>AppInstanceUserArn</code> of the moderator.</p>
    chime_bearer = "value"  # <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> 
         that makes the API call.</p>
}

# Access channel_moderator outputs
channel_moderator_id = channel_moderator.id
channel_moderator_channel_moderator = channel_moderator.channel_moderator
```

---


### Channel_membership_preferences

ChannelMembershipPreferences resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_arn` | String | ✅ | <p>The ARN of the member setting the preferences.</p> |
| `channel_arn` | String | ✅ | <p>The ARN of the channel.</p> |
| `preferences` | String | ✅ | <p>The channel membership preferences of an <code>AppInstanceUser</code> .</p> |
| `chime_bearer` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_arn` | String | <p>The ARN of the channel.</p> |
| `member` | String | <p>The details of a user.</p> |
| `preferences` | String | <p>The channel membership preferences for an <code>AppInstanceUser</code> .</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_membership_preferences
channel_membership_preferences = provider.chime_sdk_messaging.Channel_membership_preferences {
    member_arn = "value"  # <p>The ARN of the member setting the preferences.</p>
    channel_arn = "value"  # <p>The ARN of the channel.</p>
    preferences = "value"  # <p>The channel membership preferences of an <code>AppInstanceUser</code> .</p>
    chime_bearer = "value"  # <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
}

# Access channel_membership_preferences outputs
channel_membership_preferences_id = channel_membership_preferences.id
channel_membership_preferences_channel_arn = channel_membership_preferences.channel_arn
channel_membership_preferences_member = channel_membership_preferences.member
channel_membership_preferences_preferences = channel_membership_preferences.preferences
```

---


### Channel_message_status

ChannelMessageStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The message status and details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access channel_message_status outputs
channel_message_status_id = channel_message_status.id
channel_message_status_status = channel_message_status.status
```

---


### Messaging_session_endpoint

MessagingSessionEndpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint` | String | <p>The endpoint returned in the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access messaging_session_endpoint outputs
messaging_session_endpoint_id = messaging_session_endpoint.id
messaging_session_endpoint_endpoint = messaging_session_endpoint.endpoint
```

---


### Channel

Channel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `privacy` | String |  | <p>The channel's privacy level: <code>PUBLIC</code> or <code>PRIVATE</code>. Private
         channels aren't discoverable by users outside the channel. Public channels are discoverable
         by anyone in the <code>AppInstance</code>.</p> |
| `app_instance_arn` | String | ✅ | <p>The ARN of the channel request.</p> |
| `channel_id` | String |  | <p>An ID for the channel being created. If you do not specify an ID, a UUID will be created for the channel.</p> |
| `moderator_arns` | Vec<String> |  | <p>The ARNs of the channel moderators in the request.</p> |
| `tags` | Vec<String> |  | <p>The tags for the creation request.</p> |
| `name` | String | ✅ | <p>The name of the channel.</p> |
| `chime_bearer` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p> |
| `expiration_settings` | String |  | <p>Settings that control the interval after which the channel is automatically deleted.</p> |
| `mode` | String |  | <p>The channel mode: <code>UNRESTRICTED</code> or <code>RESTRICTED</code>. Administrators,
         moderators, and channel members can add themselves and other members to unrestricted
         channels. Only administrators and moderators can add members to restricted channels.</p> |
| `metadata` | String |  | <p>The metadata of the creation request. Limited to 1KB and UTF-8.</p> |
| `client_request_token` | String | ✅ | <p>The client token for the request. An <code>Idempotency</code> token.</p> |
| `member_arns` | Vec<String> |  | <p>The ARNs of the channel members in the request.</p> |
| `elastic_channel_configuration` | String |  | <p>The attributes required to configure and create an elastic channel. An elastic channel can support a maximum of 1-million users, excluding moderators.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel` | String | <p>The channel details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel
channel = provider.chime_sdk_messaging.Channel {
    app_instance_arn = "value"  # <p>The ARN of the channel request.</p>
    name = "value"  # <p>The name of the channel.</p>
    chime_bearer = "value"  # <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    client_request_token = "value"  # <p>The client token for the request. An <code>Idempotency</code> token.</p>
}

# Access channel outputs
channel_id = channel.id
channel_channel = channel.channel
```

---


### Channel_flow

ChannelFlow resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_instance_arn` | String | ✅ | <p>The ARN of the channel flow request.</p> |
| `tags` | Vec<String> |  | <p>The tags for the creation request.</p> |
| `name` | String | ✅ | <p>The name of the channel flow.</p> |
| `client_request_token` | String | ✅ | <p>The client token for the request. An Idempotency token.</p> |
| `processors` | Vec<String> | ✅ | <p>Information about the processor Lambda functions.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_flow` | String | <p>The channel flow details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_flow
channel_flow = provider.chime_sdk_messaging.Channel_flow {
    app_instance_arn = "value"  # <p>The ARN of the channel flow request.</p>
    name = "value"  # <p>The name of the channel flow.</p>
    client_request_token = "value"  # <p>The client token for the request. An Idempotency token.</p>
    processors = "value"  # <p>Information about the processor Lambda functions.</p>
}

# Access channel_flow outputs
channel_flow_id = channel_flow.id
channel_flow_channel_flow = channel_flow.channel_flow
```

---


### Channel_membership

ChannelMembership resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `chime_bearer` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> 
         that makes the API call.</p> |
| `sub_channel_id` | String |  | <p>The ID of the SubChannel in the request.</p>
         <note>
            <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p>
         </note> |
| `member_arn` | String | ✅ | <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p> |
| `channel_arn` | String | ✅ | <p>The ARN of the channel to which you're adding users.</p> |
| `type` | String | ✅ | <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default
         members are always returned as part of <code>ListChannelMemberships</code>. Hidden members
         are only returned if the type filter in <code>ListChannelMemberships</code> equals
            <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported
         by moderators.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_membership` | String | <p>The details of the membership.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_membership
channel_membership = provider.chime_sdk_messaging.Channel_membership {
    chime_bearer = "value"  # <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> 
         that makes the API call.</p>
    member_arn = "value"  # <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p>
    channel_arn = "value"  # <p>The ARN of the channel to which you're adding users.</p>
    type = "value"  # <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default
         members are always returned as part of <code>ListChannelMemberships</code>. Hidden members
         are only returned if the type filter in <code>ListChannelMemberships</code> equals
            <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported
         by moderators.</p>
}

# Access channel_membership outputs
channel_membership_id = channel_membership.id
channel_membership_channel_membership = channel_membership.channel_membership
```

---


### Channel_membership_for_app_instance_user

ChannelMembershipForAppInstanceUser resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_membership` | String | <p>The channel to which a user belongs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access channel_membership_for_app_instance_user outputs
channel_membership_for_app_instance_user_id = channel_membership_for_app_instance_user.id
channel_membership_for_app_instance_user_channel_membership = channel_membership_for_app_instance_user.channel_membership
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple channel_ban resources
channel_ban_0 = provider.chime_sdk_messaging.Channel_ban {
    member_arn = "value-0"
    chime_bearer = "value-0"
    channel_arn = "value-0"
}
channel_ban_1 = provider.chime_sdk_messaging.Channel_ban {
    member_arn = "value-1"
    chime_bearer = "value-1"
    channel_arn = "value-1"
}
channel_ban_2 = provider.chime_sdk_messaging.Channel_ban {
    member_arn = "value-2"
    chime_bearer = "value-2"
    channel_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    channel_ban = provider.chime_sdk_messaging.Channel_ban {
        member_arn = "production-value"
        chime_bearer = "production-value"
        channel_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Chime_sdk_messaging Documentation](https://docs.aws.amazon.com/chime_sdk_messaging/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
