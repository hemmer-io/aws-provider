# Vpc_lattice Service



**Resources**: 2

---

## Overview

The vpc_lattice service provides access to 2 resource types:

- [Auth_policy](#auth_policy) [CRD]
- [Resource_policy](#resource_policy) [CRD]

---

## Resources


### Auth_policy

AuthPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_identifier` | String | ✅ | <p>The ID or ARN of the service network or service for which the policy is created.</p> |
| `policy` | String | ✅ | <p>The auth policy. The policy string in JSON must not contain newlines or blank lines.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_at` | String | <p>The date and time that the auth policy was last updated, in ISO-8601 format.</p> |
| `state` | String | <p>The state of the auth policy. The auth policy is only active when the auth type is set to <code>AWS_IAM</code>. If you provide a policy, then authentication and authorization decisions are made based on this policy and the client's IAM policy. If the auth type is <code>NONE</code>, then any auth policy that you provide remains inactive. For more information, see <a href="https://docs.aws.amazon.com/vpc-lattice/latest/ug/service-networks.html#create-service-network">Create a service network</a> in the <i>Amazon VPC Lattice User Guide</i>.</p> |
| `policy` | String | <p>The auth policy.</p> |
| `created_at` | String | <p>The date and time that the auth policy was created, in ISO-8601 format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auth_policy
auth_policy = provider.vpc_lattice.Auth_policy {
    resource_identifier = "value"  # <p>The ID or ARN of the service network or service for which the policy is created.</p>
    policy = "value"  # <p>The auth policy. The policy string in JSON must not contain newlines or blank lines.</p>
}

# Access auth_policy outputs
auth_policy_id = auth_policy.id
auth_policy_last_updated_at = auth_policy.last_updated_at
auth_policy_state = auth_policy.state
auth_policy_policy = auth_policy.policy
auth_policy_created_at = auth_policy.created_at
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The ID or ARN of the service network or service for which the policy is created.</p> |
| `policy` | String | ✅ | <p>An IAM policy. The policy string in JSON must not contain newlines or blank lines.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>An IAM policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.vpc_lattice.Resource_policy {
    resource_arn = "value"  # <p>The ID or ARN of the service network or service for which the policy is created.</p>
    policy = "value"  # <p>An IAM policy. The policy string in JSON must not contain newlines or blank lines.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple auth_policy resources
auth_policy_0 = provider.vpc_lattice.Auth_policy {
    resource_identifier = "value-0"
    policy = "value-0"
}
auth_policy_1 = provider.vpc_lattice.Auth_policy {
    resource_identifier = "value-1"
    policy = "value-1"
}
auth_policy_2 = provider.vpc_lattice.Auth_policy {
    resource_identifier = "value-2"
    policy = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    auth_policy = provider.vpc_lattice.Auth_policy {
        resource_identifier = "production-value"
        policy = "production-value"
    }
```

---

## Related Documentation

- [AWS Vpc_lattice Documentation](https://docs.aws.amazon.com/vpc_lattice/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
