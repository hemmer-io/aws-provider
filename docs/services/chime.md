# Chime Service



**Resources**: 14

---

## Overview

The chime service provides access to 14 resource types:

- [Retention_settings](#retention_settings) [CR]
- [User_settings](#user_settings) [RU]
- [Phone_number_settings](#phone_number_settings) [RU]
- [Phone_number_order](#phone_number_order) [CR]
- [Account_settings](#account_settings) [RU]
- [Room](#room) [CRUD]
- [Room_membership](#room_membership) [CUD]
- [Events_configuration](#events_configuration) [CRD]
- [Phone_number](#phone_number) [RUD]
- [Account](#account) [CRUD]
- [Global_settings](#global_settings) [RU]
- [Meeting_dial_out](#meeting_dial_out) [C]
- [User](#user) [CRU]
- [Bot](#bot) [CRU]

---

## Resources


### Retention_settings

RetentionSettings resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |
| `retention_settings` | String | ✅ | <p>The retention settings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `initiate_deletion_timestamp` | String | <p>The timestamp representing the time at which the specified items are permanently deleted, in ISO 8601 format.</p> |
| `retention_settings` | String | <p>The retention settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create retention_settings
retention_settings = provider.chime.Retention_settings {
    account_id = "value"  # <p>The Amazon Chime account ID.</p>
    retention_settings = "value"  # <p>The retention settings.</p>
}

# Access retention_settings outputs
retention_settings_id = retention_settings.id
retention_settings_initiate_deletion_timestamp = retention_settings.initiate_deletion_timestamp
retention_settings_retention_settings = retention_settings.retention_settings
```

---


### User_settings

UserSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_settings` | String | ✅ | <p>The user settings to update.</p> |
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |
| `user_id` | String | ✅ | <p>The user ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_settings` | String | <p>The user settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_settings outputs
user_settings_id = user_settings.id
user_settings_user_settings = user_settings.user_settings
```

---


### Phone_number_settings

PhoneNumberSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `calling_name` | String | ✅ | <p>The default outbound calling name for the account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `calling_name_updated_timestamp` | String | <p>The updated outbound calling name timestamp, in ISO 8601 format.</p> |
| `calling_name` | String | <p>The default outbound calling name for the account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access phone_number_settings outputs
phone_number_settings_id = phone_number_settings.id
phone_number_settings_calling_name_updated_timestamp = phone_number_settings.calling_name_updated_timestamp
phone_number_settings_calling_name = phone_number_settings.calling_name
```

---


### Phone_number_order

PhoneNumberOrder resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `e164_phone_numbers` | Vec<String> | ✅ | <p>List of phone numbers, in E.164 format.</p> |
| `product_type` | String | ✅ | <p>The phone number product type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `phone_number_order` | String | <p>The phone number order details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create phone_number_order
phone_number_order = provider.chime.Phone_number_order {
    e164_phone_numbers = "value"  # <p>List of phone numbers, in E.164 format.</p>
    product_type = "value"  # <p>The phone number product type.</p>
}

# Access phone_number_order outputs
phone_number_order_id = phone_number_order.id
phone_number_order_phone_number_order = phone_number_order.phone_number_order
```

---


### Account_settings

AccountSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |
| `account_settings` | String | ✅ | <p>The Amazon Chime account settings to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_settings` | String | <p>The Amazon Chime account settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_settings outputs
account_settings_id = account_settings.id
account_settings_account_settings = account_settings.account_settings
```

---


### Room

Room resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>The idempotency token for the request.</p> |
| `name` | String | ✅ | <p>The room name.</p> |
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `room` | String | <p>The room details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create room
room = provider.chime.Room {
    name = "value"  # <p>The room name.</p>
    account_id = "value"  # <p>The Amazon Chime account ID.</p>
}

# Access room outputs
room_id = room.id
room_room = room.room
```

---


### Room_membership

RoomMembership resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |
| `member_id` | String | ✅ | <p>The Amazon Chime member ID (user ID or bot ID).</p> |
| `role` | String |  | <p>The role of the member.</p> |
| `room_id` | String | ✅ | <p>The room ID.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create room_membership
room_membership = provider.chime.Room_membership {
    account_id = "value"  # <p>The Amazon Chime account ID.</p>
    member_id = "value"  # <p>The Amazon Chime member ID (user ID or bot ID).</p>
    room_id = "value"  # <p>The room ID.</p>
}

```

---


### Events_configuration

EventsConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outbound_events_https_endpoint` | String |  | <p>HTTPS endpoint that allows the bot to receive outgoing events.</p> |
| `bot_id` | String | ✅ | <p>The bot ID.</p> |
| `lambda_function_arn` | String |  | <p>Lambda function ARN that allows the bot to receive outgoing events.</p> |
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events_configuration` | String | <p>The events configuration details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create events_configuration
events_configuration = provider.chime.Events_configuration {
    bot_id = "value"  # <p>The bot ID.</p>
    account_id = "value"  # <p>The Amazon Chime account ID.</p>
}

# Access events_configuration outputs
events_configuration_id = events_configuration.id
events_configuration_events_configuration = events_configuration.events_configuration
```

---


### Phone_number

PhoneNumber resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `calling_name` | String |  | <p>The outbound calling name associated with the phone number.</p> |
| `phone_number_id` | String | ✅ | <p>The phone number ID.</p> |
| `product_type` | String |  | <p>The product type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `phone_number` | String | <p>The phone number details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access phone_number outputs
phone_number_id = phone_number.id
phone_number_phone_number = phone_number.phone_number
```

---


### Account

Account resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the Amazon Chime account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account` | String | <p>The Amazon Chime account details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account
account = provider.chime.Account {
    name = "value"  # <p>The name of the Amazon Chime account.</p>
}

# Access account outputs
account_id = account.id
account_account = account.account
```

---


### Global_settings

GlobalSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `voice_connector` | String |  | <p>The Amazon Chime Voice Connector settings.</p> |
| `business_calling` | String |  | <p>The Amazon Chime Business Calling settings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_connector` | String | <p>The Amazon Chime Voice Connector settings.</p> |
| `business_calling` | String | <p>The Amazon Chime Business Calling settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_settings outputs
global_settings_id = global_settings.id
global_settings_voice_connector = global_settings.voice_connector
global_settings_business_calling = global_settings.business_calling
```

---


### Meeting_dial_out

MeetingDialOut resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `from_phone_number` | String | ✅ | <p>Phone number used as the caller ID when the remote party receives a call.</p> |
| `meeting_id` | String | ✅ | <p>The Amazon Chime SDK meeting ID.</p> |
| `to_phone_number` | String | ✅ | <p>Phone number called when inviting someone to a meeting.</p> |
| `join_token` | String | ✅ | <p>Token used by the Amazon Chime SDK attendee. Call the <a href="https://docs.aws.amazon.com/chime/latest/APIReference/API_CreateAttendee.html">CreateAttendee</a> action to get a join token.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create meeting_dial_out
meeting_dial_out = provider.chime.Meeting_dial_out {
    from_phone_number = "value"  # <p>Phone number used as the caller ID when the remote party receives a call.</p>
    meeting_id = "value"  # <p>The Amazon Chime SDK meeting ID.</p>
    to_phone_number = "value"  # <p>Phone number called when inviting someone to a meeting.</p>
    join_token = "value"  # <p>Token used by the Amazon Chime SDK attendee. Call the <a href="https://docs.aws.amazon.com/chime/latest/APIReference/API_CreateAttendee.html">CreateAttendee</a> action to get a join token.</p>
}

```

---


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | String |  | <p>The user's email address.</p> |
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |
| `username` | String |  | <p>The user name.</p> |
| `user_type` | String |  | <p>The user type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | String | <p>The user details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.chime.User {
    account_id = "value"  # <p>The Amazon Chime account ID.</p>
}

# Access user outputs
user_id = user.id
user_user = user.user
```

---


### Bot

Bot resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain` | String |  | <p>The domain of the Amazon Chime Enterprise account.</p> |
| `account_id` | String | ✅ | <p>The Amazon Chime account ID.</p> |
| `display_name` | String | ✅ | <p>The bot display name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot` | String | <p>The chat bot details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot
bot = provider.chime.Bot {
    account_id = "value"  # <p>The Amazon Chime account ID.</p>
    display_name = "value"  # <p>The bot display name.</p>
}

# Access bot outputs
bot_id = bot.id
bot_bot = bot.bot
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple retention_settings resources
retention_settings_0 = provider.chime.Retention_settings {
    account_id = "value-0"
    retention_settings = "value-0"
}
retention_settings_1 = provider.chime.Retention_settings {
    account_id = "value-1"
    retention_settings = "value-1"
}
retention_settings_2 = provider.chime.Retention_settings {
    account_id = "value-2"
    retention_settings = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    retention_settings = provider.chime.Retention_settings {
        account_id = "production-value"
        retention_settings = "production-value"
    }
```

---

## Related Documentation

- [AWS Chime Documentation](https://docs.aws.amazon.com/chime/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
