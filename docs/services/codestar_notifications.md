# Codestar_notifications Service



**Resources**: 2

---

## Overview

The codestar_notifications service provides access to 2 resource types:

- [Notification_rule](#notification_rule) [CRUD]
- [Target](#target) [D]

---

## Resources


### Notification_rule

NotificationRule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `targets` | Vec<String> | ✅ | <p>A list of Amazon Resource Names (ARNs) of Amazon Simple Notification Service topics and Amazon Q Developer in chat applications clients to associate with the
      notification rule.</p> |
| `event_type_ids` | Vec<String> | ✅ | <p>A list of event types associated with this notification rule. For a list of allowed
            events, see <a>EventTypeSummary</a>.</p> |
| `resource` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource to associate with the notification rule. Supported resources include pipelines in CodePipeline,
      repositories in CodeCommit, and build projects in CodeBuild.</p> |
| `detail_type` | String | ✅ | <p>The level of detail to include in the notifications for this resource. <code>BASIC</code> will include only the 
        contents of the event as it would appear in Amazon CloudWatch. <code>FULL</code> will include any supplemental information
        provided by CodeStar Notifications and/or the service for the resource for which the notification is created.</p> |
| `status` | String |  | <p>The status of the notification rule. The default value is <code>ENABLED</code>. If the status is
            set to <code>DISABLED</code>, notifications aren't sent for the notification rule.</p> |
| `name` | String | ✅ | <p>The name for the notification rule. Notification rule names must be unique in your Amazon Web Services account.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags to apply to this notification rule. Key names cannot start with "<code>aws</code>". </p> |
| `client_request_token` | String |  | <p>A unique, client-generated idempotency token that, when provided in a request, ensures
            the request cannot be repeated with a changed parameter. If a request with the same
            parameters is received and a token is included, the request returns information about
            the initial request that used that token.</p>
         <note>
            <p>The Amazon Web Services SDKs prepopulate client request tokens. If you are using an Amazon Web Services SDK, an
                idempotency token is created for you.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the notification rule.</p> |
| `created_by` | String | <p>The name or email alias of the person who created the notification rule.</p> |
| `created_timestamp` | String | <p>The date and time the notification rule was created, in timestamp format.</p> |
| `last_modified_timestamp` | String | <p>The date and time the notification rule was most recently updated, in timestamp
            format.</p> |
| `tags` | HashMap<String, String> | <p>The tags associated with the notification rule.</p> |
| `resource` | String | <p>The Amazon Resource Name (ARN) of the resource associated with the notification
      rule.</p> |
| `status` | String | <p>The status of the notification rule. Valid statuses are on (sending notifications) or off
      (not sending notifications).</p> |
| `event_types` | Vec<String> | <p>A list of the event types associated with the notification rule.</p> |
| `detail_type` | String | <p>The level of detail included in the notifications for this resource. BASIC will include only the 
            contents of the event as it would appear in Amazon CloudWatch. FULL will include any supplemental information
            provided by CodeStar Notifications and/or the service for the resource for which the notification is created.</p> |
| `targets` | Vec<String> | <p>A list of the Amazon Q Developer in chat applications topics and Amazon Q Developer in chat applications clients associated with the notification rule.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the notification rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notification_rule
notification_rule = provider.codestar_notifications.Notification_rule {
    targets = "value"  # <p>A list of Amazon Resource Names (ARNs) of Amazon Simple Notification Service topics and Amazon Q Developer in chat applications clients to associate with the
      notification rule.</p>
    event_type_ids = "value"  # <p>A list of event types associated with this notification rule. For a list of allowed
            events, see <a>EventTypeSummary</a>.</p>
    resource = "value"  # <p>The Amazon Resource Name (ARN) of the resource to associate with the notification rule. Supported resources include pipelines in CodePipeline,
      repositories in CodeCommit, and build projects in CodeBuild.</p>
    detail_type = "value"  # <p>The level of detail to include in the notifications for this resource. <code>BASIC</code> will include only the 
        contents of the event as it would appear in Amazon CloudWatch. <code>FULL</code> will include any supplemental information
        provided by CodeStar Notifications and/or the service for the resource for which the notification is created.</p>
    name = "value"  # <p>The name for the notification rule. Notification rule names must be unique in your Amazon Web Services account.</p>
}

# Access notification_rule outputs
notification_rule_id = notification_rule.id
notification_rule_name = notification_rule.name
notification_rule_created_by = notification_rule.created_by
notification_rule_created_timestamp = notification_rule.created_timestamp
notification_rule_last_modified_timestamp = notification_rule.last_modified_timestamp
notification_rule_tags = notification_rule.tags
notification_rule_resource = notification_rule.resource
notification_rule_status = notification_rule.status
notification_rule_event_types = notification_rule.event_types
notification_rule_detail_type = notification_rule.detail_type
notification_rule_targets = notification_rule.targets
notification_rule_arn = notification_rule.arn
```

---


### Target

Target resource

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

# Create multiple notification_rule resources
notification_rule_0 = provider.codestar_notifications.Notification_rule {
    targets = "value-0"
    event_type_ids = "value-0"
    resource = "value-0"
    detail_type = "value-0"
    name = "value-0"
}
notification_rule_1 = provider.codestar_notifications.Notification_rule {
    targets = "value-1"
    event_type_ids = "value-1"
    resource = "value-1"
    detail_type = "value-1"
    name = "value-1"
}
notification_rule_2 = provider.codestar_notifications.Notification_rule {
    targets = "value-2"
    event_type_ids = "value-2"
    resource = "value-2"
    detail_type = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    notification_rule = provider.codestar_notifications.Notification_rule {
        targets = "production-value"
        event_type_ids = "production-value"
        resource = "production-value"
        detail_type = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Codestar_notifications Documentation](https://docs.aws.amazon.com/codestar_notifications/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
