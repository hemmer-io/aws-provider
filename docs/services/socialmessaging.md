# Socialmessaging Service



**Resources**: 3

---

## Overview

The socialmessaging service provides access to 3 resource types:

- [Whats_app_message_template_media](#whats_app_message_template_media) [C]
- [Whats_app_message_template_from_library](#whats_app_message_template_from_library) [C]
- [Whats_app_message_template](#whats_app_message_template) [CRUD]

---

## Resources


### Whats_app_message_template_media

WhatsAppMessageTemplateMedia resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_s3_file` | String |  |  |
| `id` | String | ✅ | <p>The ID of the WhatsApp Business Account associated with this media upload.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create whats_app_message_template_media
whats_app_message_template_media = provider.socialmessaging.Whats_app_message_template_media {
    id = "value"  # <p>The ID of the WhatsApp Business Account associated with this media upload.</p>
}

```

---


### Whats_app_message_template_from_library

WhatsAppMessageTemplateFromLibrary resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `meta_library_template` | String | ✅ | <p>The template configuration from Meta's library, including customizations for buttons and body text.</p> |
| `id` | String | ✅ | <p>The ID of the WhatsApp Business Account to associate with this template.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create whats_app_message_template_from_library
whats_app_message_template_from_library = provider.socialmessaging.Whats_app_message_template_from_library {
    meta_library_template = "value"  # <p>The template configuration from Meta's library, including customizations for buttons and body text.</p>
    id = "value"  # <p>The ID of the WhatsApp Business Account to associate with this template.</p>
}

```

---


### Whats_app_message_template

WhatsAppMessageTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_definition` | String | ✅ | <p>The complete template definition as a JSON blob.</p> |
| `id` | String | ✅ | <p>The ID of the WhatsApp Business Account to associate with this template.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template` | String | <p>The complete template definition as a JSON string (maximum 6000 characters).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create whats_app_message_template
whats_app_message_template = provider.socialmessaging.Whats_app_message_template {
    template_definition = "value"  # <p>The complete template definition as a JSON blob.</p>
    id = "value"  # <p>The ID of the WhatsApp Business Account to associate with this template.</p>
}

# Access whats_app_message_template outputs
whats_app_message_template_id = whats_app_message_template.id
whats_app_message_template_template = whats_app_message_template.template
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple whats_app_message_template_media resources
whats_app_message_template_media_0 = provider.socialmessaging.Whats_app_message_template_media {
    id = "value-0"
}
whats_app_message_template_media_1 = provider.socialmessaging.Whats_app_message_template_media {
    id = "value-1"
}
whats_app_message_template_media_2 = provider.socialmessaging.Whats_app_message_template_media {
    id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    whats_app_message_template_media = provider.socialmessaging.Whats_app_message_template_media {
        id = "production-value"
    }
```

---

## Related Documentation

- [AWS Socialmessaging Documentation](https://docs.aws.amazon.com/socialmessaging/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
