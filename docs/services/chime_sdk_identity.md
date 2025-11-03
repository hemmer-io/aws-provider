# Chime_sdk_identity Service



**Resources**: 7

---

## Overview

The chime_sdk_identity service provides access to 7 resource types:

- [App_instance_user_endpoint](#app_instance_user_endpoint) [RU]
- [App_instance_retention_settings](#app_instance_retention_settings) [CR]
- [App_instance](#app_instance) [CRUD]
- [App_instance_admin](#app_instance_admin) [CRD]
- [App_instance_bot](#app_instance_bot) [CRUD]
- [App_instance_user](#app_instance_user) [CRUD]
- [App_instance_user_expiration_settings](#app_instance_user_expiration_settings) [C]

---

## Resources


### App_instance_user_endpoint

AppInstanceUserEndpoint resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the <code>AppInstanceUserEndpoint</code>.</p> |
| `app_instance_user_arn` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code>.</p> |
| `endpoint_id` | String | ✅ | <p>The unique identifier of the <code>AppInstanceUserEndpoint</code>.</p> |
| `allow_messages` | String |  | <p>Boolean that controls whether the <code>AppInstanceUserEndpoint</code> is opted in to receive messages. <code>ALL</code> indicates the endpoint will receive all messages. 
         <code>NONE</code> indicates the endpoint will receive no messages.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_instance_user_endpoint` | String | <p>The full details of an <code>AppInstanceUserEndpoint</code>: the <code>AppInstanceUserArn</code>, ID, name, type, resource ARN, attributes, 
         allow messages, state, and created and last updated timestamps. All timestamps use epoch milliseconds.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_instance_user_endpoint outputs
app_instance_user_endpoint_id = app_instance_user_endpoint.id
app_instance_user_endpoint_app_instance_user_endpoint = app_instance_user_endpoint.app_instance_user_endpoint
```

---


### App_instance_retention_settings

AppInstanceRetentionSettings resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_instance_retention_settings` | String | ✅ | <p>The time in days to retain data. Data type: number.</p> |
| `app_instance_arn` | String | ✅ | <p>The ARN of the <code>AppInstance</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_instance_retention_settings` | String | <p>The retention settings for the <code>AppInstance</code>.</p> |
| `initiate_deletion_timestamp` | String | <p>The timestamp representing the time at which the specified items are retained, in Epoch
         Seconds.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_instance_retention_settings
app_instance_retention_settings = provider.chime_sdk_identity.App_instance_retention_settings {
    app_instance_retention_settings = "value"  # <p>The time in days to retain data. Data type: number.</p>
    app_instance_arn = "value"  # <p>The ARN of the <code>AppInstance</code>.</p>
}

# Access app_instance_retention_settings outputs
app_instance_retention_settings_id = app_instance_retention_settings.id
app_instance_retention_settings_app_instance_retention_settings = app_instance_retention_settings.app_instance_retention_settings
app_instance_retention_settings_initiate_deletion_timestamp = app_instance_retention_settings.initiate_deletion_timestamp
```

---


### App_instance

AppInstance resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the <code>AppInstance</code>.</p> |
| `metadata` | String |  | <p>The metadata of the <code>AppInstance</code>. Limited to a 1KB string in UTF-8.</p> |
| `tags` | Vec<String> |  | <p>Tags assigned to the <code>AppInstance</code>.</p> |
| `client_request_token` | String | ✅ | <p>The unique ID of the request. Use different tokens to create different <code>AppInstances</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_instance` | String | <p>The ARN, metadata, created and last-updated timestamps, and the name of the
            <code>AppInstance</code>. All timestamps use epoch milliseconds.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_instance
app_instance = provider.chime_sdk_identity.App_instance {
    name = "value"  # <p>The name of the <code>AppInstance</code>.</p>
    client_request_token = "value"  # <p>The unique ID of the request. Use different tokens to create different <code>AppInstances</code>.</p>
}

# Access app_instance outputs
app_instance_id = app_instance.id
app_instance_app_instance = app_instance.app_instance
```

---


### App_instance_admin

AppInstanceAdmin resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_instance_arn` | String | ✅ | <p>The ARN of the <code>AppInstance</code>.</p> |
| `app_instance_admin_arn` | String | ✅ | <p>The ARN of the administrator of the current <code>AppInstance</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_instance_admin` | String | <p>The ARN and name of the <code>AppInstanceUser</code>, the ARN of the
            <code>AppInstance</code>, and the created and last-updated timestamps. All timestamps
         use epoch milliseconds.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_instance_admin
app_instance_admin = provider.chime_sdk_identity.App_instance_admin {
    app_instance_arn = "value"  # <p>The ARN of the <code>AppInstance</code>.</p>
    app_instance_admin_arn = "value"  # <p>The ARN of the administrator of the current <code>AppInstance</code>.</p>
}

# Access app_instance_admin outputs
app_instance_admin_id = app_instance_admin.id
app_instance_admin_app_instance_admin = app_instance_admin.app_instance_admin
```

---


### App_instance_bot

AppInstanceBot resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags assigned to the <code>AppInstanceBot</code>.</p> |
| `configuration` | String | ✅ | <p>Configuration information about the Amazon Lex V2 V2 bot.</p> |
| `client_request_token` | String | ✅ | <p>The unique ID for the client making the request. Use different tokens for different <code>AppInstanceBots</code>.</p> |
| `app_instance_arn` | String | ✅ | <p>The ARN of the <code>AppInstance</code> request.</p> |
| `metadata` | String |  | <p>The request metadata. Limited to a 1KB string in UTF-8.</p> |
| `name` | String |  | <p>The user's name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_instance_bot` | String | <p>The detials of the <code>AppInstanceBot</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_instance_bot
app_instance_bot = provider.chime_sdk_identity.App_instance_bot {
    configuration = "value"  # <p>Configuration information about the Amazon Lex V2 V2 bot.</p>
    client_request_token = "value"  # <p>The unique ID for the client making the request. Use different tokens for different <code>AppInstanceBots</code>.</p>
    app_instance_arn = "value"  # <p>The ARN of the <code>AppInstance</code> request.</p>
}

# Access app_instance_bot outputs
app_instance_bot_id = app_instance_bot.id
app_instance_bot_app_instance_bot = app_instance_bot.app_instance_bot
```

---


### App_instance_user

AppInstanceUser resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metadata` | String |  | <p>The request's metadata. Limited to a 1KB string in UTF-8.</p> |
| `app_instance_arn` | String | ✅ | <p>The ARN of the <code>AppInstance</code> request.</p> |
| `tags` | Vec<String> |  | <p>Tags assigned to the <code>AppInstanceUser</code>.</p> |
| `name` | String | ✅ | <p>The user's name.</p> |
| `client_request_token` | String | ✅ | <p>The unique ID of the request. Use different tokens to request additional <code>AppInstances</code>.</p> |
| `expiration_settings` | String |  | <p>Settings that control the interval after which the <code>AppInstanceUser</code> is automatically deleted.</p> |
| `app_instance_user_id` | String | ✅ | <p>The user ID of the <code>AppInstance</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_instance_user` | String | <p>The name of the <code>AppInstanceUser</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_instance_user
app_instance_user = provider.chime_sdk_identity.App_instance_user {
    app_instance_arn = "value"  # <p>The ARN of the <code>AppInstance</code> request.</p>
    name = "value"  # <p>The user's name.</p>
    client_request_token = "value"  # <p>The unique ID of the request. Use different tokens to request additional <code>AppInstances</code>.</p>
    app_instance_user_id = "value"  # <p>The user ID of the <code>AppInstance</code>.</p>
}

# Access app_instance_user outputs
app_instance_user_id = app_instance_user.id
app_instance_user_app_instance_user = app_instance_user.app_instance_user
```

---


### App_instance_user_expiration_settings

AppInstanceUserExpirationSettings resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_instance_user_arn` | String | ✅ | <p>The ARN of the <code>AppInstanceUser</code>.</p> |
| `expiration_settings` | String |  | <p>Settings that control the interval after which an <code>AppInstanceUser</code> is automatically deleted.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_instance_user_expiration_settings
app_instance_user_expiration_settings = provider.chime_sdk_identity.App_instance_user_expiration_settings {
    app_instance_user_arn = "value"  # <p>The ARN of the <code>AppInstanceUser</code>.</p>
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

# Create multiple app_instance_user_endpoint resources
app_instance_user_endpoint_0 = provider.chime_sdk_identity.App_instance_user_endpoint {
    app_instance_user_arn = "value-0"
    endpoint_id = "value-0"
}
app_instance_user_endpoint_1 = provider.chime_sdk_identity.App_instance_user_endpoint {
    app_instance_user_arn = "value-1"
    endpoint_id = "value-1"
}
app_instance_user_endpoint_2 = provider.chime_sdk_identity.App_instance_user_endpoint {
    app_instance_user_arn = "value-2"
    endpoint_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    app_instance_user_endpoint = provider.chime_sdk_identity.App_instance_user_endpoint {
        app_instance_user_arn = "production-value"
        endpoint_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Chime_sdk_identity Documentation](https://docs.aws.amazon.com/chime_sdk_identity/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
