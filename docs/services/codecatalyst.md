# Codecatalyst Service



**Resources**: 1

---

## Overview

The codecatalyst service provides access to 1 resource type:

- [User_details](#user_details) [R]

---

## Resources


### User_details

UserDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_name` | String | <p>The name of the user as displayed in Amazon CodeCatalyst.</p> |
| `user_id` | String | <p>The system-generated unique ID of the user.</p> |
| `display_name` | String | <p>The friendly name displayed for the user in Amazon CodeCatalyst.</p> |
| `primary_email` | String | <p>The email address provided by the user when they signed up.</p> |
| `version` | String | <p/> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_details outputs
user_details_id = user_details.id
user_details_user_name = user_details.user_name
user_details_user_id = user_details.user_id
user_details_display_name = user_details.display_name
user_details_primary_email = user_details.primary_email
user_details_version = user_details.version
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple user_details resources
user_details_0 = provider.codecatalyst.User_details {
}
user_details_1 = provider.codecatalyst.User_details {
}
user_details_2 = provider.codecatalyst.User_details {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user_details = provider.codecatalyst.User_details {
    }
```

---

## Related Documentation

- [AWS Codecatalyst Documentation](https://docs.aws.amazon.com/codecatalyst/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
