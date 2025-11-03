# Repostspace Service



**Resources**: 2

---

## Overview

The repostspace service provides access to 2 resource types:

- [Channel](#channel) [CRU]
- [Space](#space) [CRUD]

---

## Resources


### Channel

Channel resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `space_id` | String | ✅ | <p>The unique ID of the private re:Post.</p> |
| `channel_description` | String |  | <p>A description for the channel. This is used only to help you identify this channel.</p> |
| `channel_name` | String | ✅ | <p>The name for the channel. This must be unique per private re:Post.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_date_time` | String | <p>The date when the channel was deleted.</p> |
| `channel_roles` | HashMap<String, Vec<String>> | <p>The channel roles associated to the users and groups of the channel.</p> |
| `channel_status` | String | <p>The status pf the channel.</p> |
| `channel_id` | String | <p>The unique ID of the private re:Post channel.</p> |
| `space_id` | String | <p>The unique ID of the private re:Post.</p> |
| `create_date_time` | String | <p>The date when the channel was created.</p> |
| `channel_description` | String | <p>A description for the channel. This is used only to help you identify this channel.</p> |
| `channel_name` | String | <p>The name for the channel. This must be unique per private re:Post.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel
channel = provider.repostspace.Channel {
    space_id = "value"  # <p>The unique ID of the private re:Post.</p>
    channel_name = "value"  # <p>The name for the channel. This must be unique per private re:Post.</p>
}

# Access channel outputs
channel_id = channel.id
channel_delete_date_time = channel.delete_date_time
channel_channel_roles = channel.channel_roles
channel_channel_status = channel.channel_status
channel_channel_id = channel.channel_id
channel_space_id = channel.space_id
channel_create_date_time = channel.create_date_time
channel_channel_description = channel.channel_description
channel_channel_name = channel.channel_name
```

---


### Space

Space resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_kms_key` | String |  | <p>The AWS KMS key ARN that’s used for the AWS KMS encryption. If you don't provide a key, your data is encrypted by default with a key that AWS owns and manages for you.</p> |
| `tier` | String | ✅ | <p>The pricing tier for the private re:Post.</p> |
| `role_arn` | String |  | <p>The IAM role that grants permissions to the private re:Post to convert unanswered questions into AWS support tickets.</p> |
| `subdomain` | String | ✅ | <p>The subdomain that you use to access your AWS re:Post Private private re:Post. All custom subdomains must be approved by AWS before use. In addition to your custom subdomain, all private re:Posts are issued an AWS generated subdomain for immediate use.</p> |
| `description` | String |  | <p>A description for the private re:Post. This is used only to help you identify this private re:Post.</p> |
| `tags` | HashMap<String, String> |  | <p>The list of tags associated with the private re:Post.</p> |
| `name` | String | ✅ | <p>The name for the private re:Post. This must be unique in your account.</p> |
| `supported_email_domains` | String |  | <p/> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer_role_arn` | String | <p>The IAM role that grants permissions to the private re:Post to convert unanswered questions into AWS support tickets.</p> |
| `group_admins` | Vec<String> | <p>The list of groups that are administrators of the private re:Post.</p> |
| `client_id` | String | <p>The Identity Center identifier for the Application Instance.</p> |
| `vanity_domain_status` | String | <p>The approval status of the custom subdomain.</p> |
| `roles` | HashMap<String, Vec<String>> | <p>A map of accessor identifiers and their roles.</p> |
| `user_kms_key` | String | <p>The custom AWS KMS key ARN that’s used for the AWS KMS encryption.</p> |
| `user_count` | i64 | <p>The number of users that have onboarded to the private re:Post.</p> |
| `supported_email_domains` | String | <p/> |
| `vanity_domain` | String | <p>The custom subdomain that you use to access your private re:Post. All custom subdomains must be approved by AWS before use.</p> |
| `storage_limit` | i64 | <p>The storage limit of the private re:Post.</p> |
| `delete_date_time` | String | <p>The date when the private re:Post was deleted.</p> |
| `create_date_time` | String | <p>The date when the private re:Post was created.</p> |
| `identity_store_id` | String | <p/> |
| `arn` | String | <p>The ARN of the private re:Post.</p> |
| `space_id` | String | <p>The unique ID of the private re:Post.</p> |
| `name` | String | <p>The name of the private re:Post.</p> |
| `random_domain` | String | <p>The AWS generated subdomain of the private re:Post</p> |
| `user_admins` | Vec<String> | <p>The list of users that are administrators of the private re:Post.</p> |
| `description` | String | <p>The description of the private re:Post.</p> |
| `tier` | String | <p>The pricing tier of the private re:Post.</p> |
| `content_size` | i64 | <p>The content size of the private re:Post.</p> |
| `status` | String | <p>The creation or deletion status of the private re:Post.</p> |
| `application_arn` | String | <p/> |
| `configuration_status` | String | <p>The configuration status of the private re:Post.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create space
space = provider.repostspace.Space {
    tier = "value"  # <p>The pricing tier for the private re:Post.</p>
    subdomain = "value"  # <p>The subdomain that you use to access your AWS re:Post Private private re:Post. All custom subdomains must be approved by AWS before use. In addition to your custom subdomain, all private re:Posts are issued an AWS generated subdomain for immediate use.</p>
    name = "value"  # <p>The name for the private re:Post. This must be unique in your account.</p>
}

# Access space outputs
space_id = space.id
space_customer_role_arn = space.customer_role_arn
space_group_admins = space.group_admins
space_client_id = space.client_id
space_vanity_domain_status = space.vanity_domain_status
space_roles = space.roles
space_user_kms_key = space.user_kms_key
space_user_count = space.user_count
space_supported_email_domains = space.supported_email_domains
space_vanity_domain = space.vanity_domain
space_storage_limit = space.storage_limit
space_delete_date_time = space.delete_date_time
space_create_date_time = space.create_date_time
space_identity_store_id = space.identity_store_id
space_arn = space.arn
space_space_id = space.space_id
space_name = space.name
space_random_domain = space.random_domain
space_user_admins = space.user_admins
space_description = space.description
space_tier = space.tier
space_content_size = space.content_size
space_status = space.status
space_application_arn = space.application_arn
space_configuration_status = space.configuration_status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple channel resources
channel_0 = provider.repostspace.Channel {
    space_id = "value-0"
    channel_name = "value-0"
}
channel_1 = provider.repostspace.Channel {
    space_id = "value-1"
    channel_name = "value-1"
}
channel_2 = provider.repostspace.Channel {
    space_id = "value-2"
    channel_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    channel = provider.repostspace.Channel {
        space_id = "production-value"
        channel_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Repostspace Documentation](https://docs.aws.amazon.com/repostspace/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
