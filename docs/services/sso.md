# Sso Service



**Resources**: 1

---

## Overview

The sso service provides access to 1 resource type:

- [Role_credentials](#role_credentials) [R]

---

## Resources


### Role_credentials

RoleCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_credentials` | String | <p>The credentials for the role that is assigned to the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access role_credentials outputs
role_credentials_id = role_credentials.id
role_credentials_role_credentials = role_credentials.role_credentials
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple role_credentials resources
role_credentials_0 = provider.sso.Role_credentials {
}
role_credentials_1 = provider.sso.Role_credentials {
}
role_credentials_2 = provider.sso.Role_credentials {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    role_credentials = provider.sso.Role_credentials {
    }
```

---

## Related Documentation

- [AWS Sso Documentation](https://docs.aws.amazon.com/sso/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
