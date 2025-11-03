# Mpa Service



**Resources**: 2

---

## Overview

The mpa service provides access to 2 resource types:

- [Policy_version](#policy_version) [R]
- [Resource_policy](#resource_policy) [R]

---

## Resources


### Policy_version

PolicyVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_version` | String | <p>A <code>PolicyVersion</code> object. Contains details for the version of the policy. Policies define the permissions for team resources.</p> <p>The protected operation for a service integration might require specific permissions. For more information, see <a href="https://docs.aws.amazon.com/mpa/latest/userguide/mpa-integrations.html">How other services work with Multi-party approval</a> in the <i>Multi-party approval User Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access policy_version outputs
policy_version_id = policy_version.id
policy_version_policy_version = policy_version.policy_version
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_name` | String | <p>Name of the policy.</p> |
| `policy_type` | String | <p>The type of policy</p> |
| `policy_document` | String | <p>Document that contains the contents for the policy.</p> |
| `resource_arn` | String | <p>Amazon Resource Name (ARN) for the resource.</p> |
| `policy_version_arn` | String | <p>Amazon Resource Name (ARN) for the policy version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy_name = resource_policy.policy_name
resource_policy_policy_type = resource_policy.policy_type
resource_policy_policy_document = resource_policy.policy_document
resource_policy_resource_arn = resource_policy.resource_arn
resource_policy_policy_version_arn = resource_policy.policy_version_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple policy_version resources
policy_version_0 = provider.mpa.Policy_version {
}
policy_version_1 = provider.mpa.Policy_version {
}
policy_version_2 = provider.mpa.Policy_version {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    policy_version = provider.mpa.Policy_version {
    }
```

---

## Related Documentation

- [AWS Mpa Documentation](https://docs.aws.amazon.com/mpa/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
