# Qbusiness Service



**Resources**: 12

---

## Overview

The qbusiness service provides access to 12 resource types:

- [Chat_response_configuration](#chat_response_configuration) [CRUD]
- [Anonymous_web_experience_url](#anonymous_web_experience_url) [C]
- [User](#user) [CRUD]
- [Conversation](#conversation) [D]
- [Attachment](#attachment) [D]
- [Document_content](#document_content) [R]
- [Policy](#policy) [R]
- [Chat_controls_configuration](#chat_controls_configuration) [RUD]
- [Feedback](#feedback) [C]
- [Subscription](#subscription) [CU]
- [Media](#media) [R]
- [Group](#group) [CRD]

---

## Resources


### Chat_response_configuration

ChatResponseConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier to ensure idempotency of the request. This helps prevent the same configuration from being created multiple times if retries occur.</p> |
| `response_configurations` | HashMap<String, String> | ✅ | <p>A collection of response configuration settings that define how Amazon Q Business will generate and format responses to user queries in chat interactions.</p> |
| `application_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application for which to create the new chat response configuration.</p> |
| `display_name` | String | ✅ | <p>A human-readable name for the new chat response configuration, making it easier to identify and manage among multiple configurations.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs to apply as tags to the new chat response configuration, enabling categorization and management of resources across Amazon Web Services services.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `chat_response_configuration_arn` | String | <p>The Amazon Resource Name (ARN) of the retrieved chat response configuration, which uniquely identifies the resource across all Amazon Web Services services. </p> |
| `chat_response_configuration_id` | String | <p>The unique identifier of the retrieved chat response configuration.</p> |
| `display_name` | String | <p>The human-readable name of the retrieved chat response configuration, making it easier to identify among multiple configurations.</p> |
| `last_update_configuration` | String | <p>Information about the most recent update to the configuration, including timestamp and modification details.</p> |
| `created_at` | String | <p>The timestamp indicating when the chat response configuration was initially created.</p> |
| `in_use_configuration` | String | <p>The currently active configuration settings that are being used to generate responses in the Amazon Q Business application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create chat_response_configuration
chat_response_configuration = provider.qbusiness.Chat_response_configuration {
    response_configurations = "value"  # <p>A collection of response configuration settings that define how Amazon Q Business will generate and format responses to user queries in chat interactions.</p>
    application_id = "value"  # <p>The unique identifier of the Amazon Q Business application for which to create the new chat response configuration.</p>
    display_name = "value"  # <p>A human-readable name for the new chat response configuration, making it easier to identify and manage among multiple configurations.</p>
}

# Access chat_response_configuration outputs
chat_response_configuration_id = chat_response_configuration.id
chat_response_configuration_chat_response_configuration_arn = chat_response_configuration.chat_response_configuration_arn
chat_response_configuration_chat_response_configuration_id = chat_response_configuration.chat_response_configuration_id
chat_response_configuration_display_name = chat_response_configuration.display_name
chat_response_configuration_last_update_configuration = chat_response_configuration.last_update_configuration
chat_response_configuration_created_at = chat_response_configuration.created_at
chat_response_configuration_in_use_configuration = chat_response_configuration.in_use_configuration
```

---


### Anonymous_web_experience_url

AnonymousWebExperienceUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_duration_in_minutes` | i64 |  | <p>The duration of the session associated with the unique URL for the web experience.</p> |
| `application_id` | String | ✅ | <p>The identifier of the Amazon Q Business application environment attached to the web experience.</p> |
| `web_experience_id` | String | ✅ | <p>The identifier of the web experience.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create anonymous_web_experience_url
anonymous_web_experience_url = provider.qbusiness.Anonymous_web_experience_url {
    application_id = "value"  # <p>The identifier of the Amazon Q Business application environment attached to the web experience.</p>
    web_experience_id = "value"  # <p>The identifier of the web experience.</p>
}

```

---


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | <p>The user emails attached to a user mapping.</p> |
| `user_aliases` | Vec<String> |  | <p>The list of user aliases in the mapping.</p> |
| `application_id` | String | ✅ | <p>The identifier of the application for which the user mapping will be created.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create your Amazon Q Business user mapping.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_aliases` | Vec<String> | <p>A list of user aliases attached to a user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.qbusiness.User {
    user_id = "value"  # <p>The user emails attached to a user mapping.</p>
    application_id = "value"  # <p>The identifier of the application for which the user mapping will be created.</p>
}

# Access user outputs
user_id = user.id
user_user_aliases = user.user_aliases
```

---


### Conversation

Conversation resource

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


### Attachment

Attachment resource

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


### Document_content

DocumentContent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mime_type` | String | <p>The MIME type of the document content. When outputFormat is RAW, this corresponds to the original document's MIME type (e.g., application/pdf, text/plain, application/vnd.openxmlformats-officedocument.wordprocessingml.document). When outputFormat is EXTRACTED, the MIME type is always application/json.</p> |
| `presigned_url` | String | <p>A pre-signed URL that provides temporary access to download the document content directly from Amazon Q Business. The URL expires after 5 minutes for security purposes. This URL is generated only after successful ACL validation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_content outputs
document_content_id = document_content.id
document_content_mime_type = document_content.mime_type
document_content_presigned_url = document_content.presigned_url
```

---


### Policy

Policy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The JSON representation of the permission policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access policy outputs
policy_id = policy.id
policy_policy = policy.policy
```

---


### Chat_controls_configuration

ChatControlsConfiguration resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `topic_configurations_to_delete` | Vec<String> |  | <p>The configured topic specific chat controls you want to delete.</p> |
| `application_id` | String | ✅ | <p>The identifier of the application for which the chat controls are configured.</p> |
| `blocked_phrases_configuration_update` | String |  | <p>The phrases blocked from chat by your chat control configuration.</p> |
| `hallucination_reduction_configuration` | String |  | <p> The hallucination reduction settings for your application.</p> |
| `creator_mode_configuration` | String |  | <p>The configuration details for <code>CREATOR_MODE</code>.</p> |
| `topic_configurations_to_create_or_update` | Vec<String> |  | <p>The configured topic specific chat controls you want to update.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to update a Amazon Q Business application chat configuration.</p> |
| `orchestration_configuration` | String |  | <p> The chat response orchestration settings for your application.</p> |
| `response_scope` | String |  | <p>The response scope configured for your application. This determines whether your application uses its retrieval augmented generation (RAG) system to generate answers only from your enterprise data, or also uses the large language models (LLM) knowledge to respons to end user questions in chat.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_scope` | String | <p>The response scope configured for a Amazon Q Business application. This determines whether your application uses its retrieval augmented generation (RAG) system to generate answers only from your enterprise data, or also uses the large language models (LLM) knowledge to respons to end user questions in chat.</p> |
| `orchestration_configuration` | String | <p> The chat response orchestration settings for your application.</p> <note> <p>Chat orchestration is optimized to work for English language content. For more details on language support in Amazon Q Business, see <a href="https://docs.aws.amazon.com/amazonq/latest/qbusiness-ug/supported-languages.html">Supported languages</a>.</p> </note> |
| `topic_configurations` | Vec<String> | <p>The topic specific controls configured for a Amazon Q Business application.</p> |
| `hallucination_reduction_configuration` | String | <p> The hallucination reduction settings for your application.</p> |
| `blocked_phrases` | String | <p>The phrases blocked from chat by your chat control configuration.</p> |
| `creator_mode_configuration` | String | <p>The configuration details for <code>CREATOR_MODE</code>.</p> |
| `next_token` | String | <p>If the <code>maxResults</code> response was incomplete because there is more data to retrieve, Amazon Q Business returns a pagination token in the response. You can use this pagination token to retrieve the next set of Amazon Q Business chat controls configured.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access chat_controls_configuration outputs
chat_controls_configuration_id = chat_controls_configuration.id
chat_controls_configuration_response_scope = chat_controls_configuration.response_scope
chat_controls_configuration_orchestration_configuration = chat_controls_configuration.orchestration_configuration
chat_controls_configuration_topic_configurations = chat_controls_configuration.topic_configurations
chat_controls_configuration_hallucination_reduction_configuration = chat_controls_configuration.hallucination_reduction_configuration
chat_controls_configuration_blocked_phrases = chat_controls_configuration.blocked_phrases
chat_controls_configuration_creator_mode_configuration = chat_controls_configuration.creator_mode_configuration
chat_controls_configuration_next_token = chat_controls_configuration.next_token
```

---


### Feedback

Feedback resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The identifier of the application associated with the feedback.</p> |
| `conversation_id` | String | ✅ | <p>The identifier of the conversation the feedback is attached to.</p> |
| `message_copied_at` | String |  | <p>The timestamp for when the feedback was recorded.</p> |
| `message_id` | String | ✅ | <p>The identifier of the chat message that the feedback was given for.</p> |
| `message_usefulness` | String |  | <p>The feedback usefulness value given by the user to the chat message.</p> |
| `user_id` | String |  | <p>The identifier of the user giving the feedback.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create feedback
feedback = provider.qbusiness.Feedback {
    application_id = "value"  # <p>The identifier of the application associated with the feedback.</p>
    conversation_id = "value"  # <p>The identifier of the conversation the feedback is attached to.</p>
    message_id = "value"  # <p>The identifier of the chat message that the feedback was given for.</p>
}

```

---


### Subscription

Subscription resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal` | String | ✅ | <p>The IAM Identity Center <code>UserId</code> or <code>GroupId</code> of a user or group in the IAM Identity Center instance connected to the Amazon Q Business application.</p> |
| `client_token` | String |  | <p>A token that you provide to identify the request to create a subscription for your Amazon Q Business application.</p> |
| `type` | String | ✅ | <p>The type of Amazon Q Business subscription you want to create.</p> |
| `application_id` | String | ✅ | <p>The identifier of the Amazon Q Business application the subscription should be added to.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription
subscription = provider.qbusiness.Subscription {
    principal = "value"  # <p>The IAM Identity Center <code>UserId</code> or <code>GroupId</code> of a user or group in the IAM Identity Center instance connected to the Amazon Q Business application.</p>
    type = "value"  # <p>The type of Amazon Q Business subscription you want to create.</p>
    application_id = "value"  # <p>The identifier of the Amazon Q Business application the subscription should be added to.</p>
}

```

---


### Media

Media resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `media_bytes` | String | <p>The base64-encoded bytes of the media object.</p> |
| `media_mime_type` | String | <p>The MIME type of the media object (image/png).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access media outputs
media_id = media.id
media_media_bytes = media.media_bytes
media_media_mime_type = media.media_mime_type
```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_id` | String | ✅ | <p>The identifier of the index in which you want to map users to their groups.</p> |
| `group_members` | String | ✅ |  |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role that has access to the S3 file that contains your list of users that belong to a group.</p> |
| `group_name` | String | ✅ | <p>The list that contains your users or sub groups that belong the same group. For example, the group "Company" includes the user "CEO" and the sub groups "Research", "Engineering", and "Sales and Marketing".</p> |
| `type` | String | ✅ | <p>The type of the group.</p> |
| `application_id` | String | ✅ | <p>The identifier of the application in which the user and group mapping belongs.</p> |
| `data_source_id` | String |  | <p>The identifier of the data source for which you want to map users to their groups. This is useful if a group is tied to multiple data sources, but you only want the group to access documents of a certain data source. For example, the groups "Research", "Engineering", and "Sales and Marketing" are all tied to the company's documents stored in the data sources Confluence and Salesforce. However, "Sales and Marketing" team only needs access to customer-related documents stored in Salesforce.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The current status of the group.</p> |
| `status_history` | Vec<String> | <p>The status history of the group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.qbusiness.Group {
    index_id = "value"  # <p>The identifier of the index in which you want to map users to their groups.</p>
    group_members = "value"  # Required field
    group_name = "value"  # <p>The list that contains your users or sub groups that belong the same group. For example, the group "Company" includes the user "CEO" and the sub groups "Research", "Engineering", and "Sales and Marketing".</p>
    type = "value"  # <p>The type of the group.</p>
    application_id = "value"  # <p>The identifier of the application in which the user and group mapping belongs.</p>
}

# Access group outputs
group_id = group.id
group_status = group.status
group_status_history = group.status_history
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple chat_response_configuration resources
chat_response_configuration_0 = provider.qbusiness.Chat_response_configuration {
    response_configurations = "value-0"
    application_id = "value-0"
    display_name = "value-0"
}
chat_response_configuration_1 = provider.qbusiness.Chat_response_configuration {
    response_configurations = "value-1"
    application_id = "value-1"
    display_name = "value-1"
}
chat_response_configuration_2 = provider.qbusiness.Chat_response_configuration {
    response_configurations = "value-2"
    application_id = "value-2"
    display_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    chat_response_configuration = provider.qbusiness.Chat_response_configuration {
        response_configurations = "production-value"
        application_id = "production-value"
        display_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Qbusiness Documentation](https://docs.aws.amazon.com/qbusiness/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
