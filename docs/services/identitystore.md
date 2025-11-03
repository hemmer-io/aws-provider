# Identitystore Service



**Resources**: 3

---

## Overview

The identitystore service provides access to 3 resource types:

- [User_id](#user_id) [R]
- [Group_id](#group_id) [R]
- [Group_membership_id](#group_membership_id) [R]

---

## Resources


### User_id

UserId resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_id` | String | <p>The identifier for a user in the identity store.</p> |
| `identity_store_id` | String | <p>The globally unique identifier for the identity store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_id outputs
user_id_id = user_id.id
user_id_user_id = user_id.user_id
user_id_identity_store_id = user_id.identity_store_id
```

---


### Group_id

GroupId resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_store_id` | String | <p>The globally unique identifier for the identity store.</p> |
| `group_id` | String | <p>The identifier for a group in the identity store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access group_id outputs
group_id_id = group_id.id
group_id_identity_store_id = group_id.identity_store_id
group_id_group_id = group_id.group_id
```

---


### Group_membership_id

GroupMembershipId resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_store_id` | String | <p>The globally unique identifier for the identity store.</p> |
| `membership_id` | String | <p>The identifier for a <code>GroupMembership</code> in an identity store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access group_membership_id outputs
group_membership_id_id = group_membership_id.id
group_membership_id_identity_store_id = group_membership_id.identity_store_id
group_membership_id_membership_id = group_membership_id.membership_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple user_id resources
user_id_0 = provider.identitystore.User_id {
}
user_id_1 = provider.identitystore.User_id {
}
user_id_2 = provider.identitystore.User_id {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user_id = provider.identitystore.User_id {
    }
```

---

## Related Documentation

- [AWS Identitystore Documentation](https://docs.aws.amazon.com/identitystore/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
