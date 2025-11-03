# Workmailmessageflow Service



**Resources**: 1

---

## Overview

The workmailmessageflow service provides access to 1 resource type:

- [Raw_message_content](#raw_message_content) [CR]

---

## Resources


### Raw_message_content

RawMessageContent resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | String | ✅ | <p>Describes the raw message content of the updated email message.</p> |
| `message_id` | String | ✅ | <p>The identifier of the email message being updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message_content` | String | <p>The raw content of the email message, in MIME format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create raw_message_content
raw_message_content = provider.workmailmessageflow.Raw_message_content {
    content = "value"  # <p>Describes the raw message content of the updated email message.</p>
    message_id = "value"  # <p>The identifier of the email message being updated.</p>
}

# Access raw_message_content outputs
raw_message_content_id = raw_message_content.id
raw_message_content_message_content = raw_message_content.message_content
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple raw_message_content resources
raw_message_content_0 = provider.workmailmessageflow.Raw_message_content {
    content = "value-0"
    message_id = "value-0"
}
raw_message_content_1 = provider.workmailmessageflow.Raw_message_content {
    content = "value-1"
    message_id = "value-1"
}
raw_message_content_2 = provider.workmailmessageflow.Raw_message_content {
    content = "value-2"
    message_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    raw_message_content = provider.workmailmessageflow.Raw_message_content {
        content = "production-value"
        message_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Workmailmessageflow Documentation](https://docs.aws.amazon.com/workmailmessageflow/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
