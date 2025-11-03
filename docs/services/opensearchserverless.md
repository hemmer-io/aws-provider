# Opensearchserverless Service



**Resources**: 5

---

## Overview

The opensearchserverless service provides access to 5 resource types:

- [Security_policy](#security_policy) [C]
- [Policies_stats](#policies_stats) [R]
- [Vpc_endpoint](#vpc_endpoint) [U]
- [Lifecycle_policy](#lifecycle_policy) [C]
- [Account_settings](#account_settings) [RU]

---

## Resources


### Security_policy

SecurityPolicy resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the policy. Typically used to store information about the permissions defined in the policy.</p> |
| `name` | String | ✅ | <p>The name of the policy.</p> |
| `type` | String | ✅ | <p>The type of security policy.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p> |
| `policy` | String | ✅ | <p>The JSON policy document to use as the content for the new policy.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create security_policy
security_policy = provider.opensearchserverless.Security_policy {
    name = "value"  # <p>The name of the policy.</p>
    type = "value"  # <p>The type of security policy.</p>
    policy = "value"  # <p>The JSON policy document to use as the content for the new policy.</p>
}

```

---


### Policies_stats

PoliciesStats resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_policy_stats` | String | <p>Information about the security policies in your account.</p> |
| `lifecycle_policy_stats` | String | <p>Information about the lifecycle policies in your account.</p> |
| `total_policy_count` | i64 | <p>The total number of OpenSearch Serverless security policies and configurations in your account.</p> |
| `security_config_stats` | String | <p>Information about the security configurations in your account.</p> |
| `access_policy_stats` | String | <p>Information about the data access policies in your account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access policies_stats outputs
policies_stats_id = policies_stats.id
policies_stats_security_policy_stats = policies_stats.security_policy_stats
policies_stats_lifecycle_policy_stats = policies_stats.lifecycle_policy_stats
policies_stats_total_policy_count = policies_stats.total_policy_count
policies_stats_security_config_stats = policies_stats.security_config_stats
policies_stats_access_policy_stats = policies_stats.access_policy_stats
```

---


### Vpc_endpoint

VpcEndpoint resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `remove_security_group_ids` | Vec<String> |  | <p>The unique identifiers of the security groups to remove from the endpoint.</p> |
| `id` | String | ✅ | <p>The unique identifier of the interface endpoint to update.</p> |
| `remove_subnet_ids` | Vec<String> |  | <p>The unique identifiers of the subnets to remove from the endpoint.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p> |
| `add_security_group_ids` | Vec<String> |  | <p>The unique identifiers of the security groups to add to the endpoint. Security groups define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.</p> |
| `add_subnet_ids` | Vec<String> |  | <p>The ID of one or more subnets to add to the endpoint.</p> |



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


### Lifecycle_policy

LifecyclePolicy resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The JSON policy document to use as the content for the lifecycle policy.</p> |
| `description` | String |  | <p>A description of the lifecycle policy.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier to ensure idempotency of the request.</p> |
| `type` | String | ✅ | <p>The type of lifecycle policy.</p> |
| `name` | String | ✅ | <p>The name of the lifecycle policy.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_policy
lifecycle_policy = provider.opensearchserverless.Lifecycle_policy {
    policy = "value"  # <p>The JSON policy document to use as the content for the lifecycle policy.</p>
    type = "value"  # <p>The type of lifecycle policy.</p>
    name = "value"  # <p>The name of the lifecycle policy.</p>
}

```

---


### Account_settings

AccountSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `capacity_limits` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_settings_detail` | String | <p>OpenSearch Serverless-related details for the current account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_settings outputs
account_settings_id = account_settings.id
account_settings_account_settings_detail = account_settings.account_settings_detail
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple security_policy resources
security_policy_0 = provider.opensearchserverless.Security_policy {
    name = "value-0"
    type = "value-0"
    policy = "value-0"
}
security_policy_1 = provider.opensearchserverless.Security_policy {
    name = "value-1"
    type = "value-1"
    policy = "value-1"
}
security_policy_2 = provider.opensearchserverless.Security_policy {
    name = "value-2"
    type = "value-2"
    policy = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    security_policy = provider.opensearchserverless.Security_policy {
        name = "production-value"
        type = "production-value"
        policy = "production-value"
    }
```

---

## Related Documentation

- [AWS Opensearchserverless Documentation](https://docs.aws.amazon.com/opensearchserverless/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
