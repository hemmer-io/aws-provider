# Billing Service



**Resources**: 2

---

## Overview

The billing service provides access to 2 resource types:

- [Resource_policy](#resource_policy) [R]
- [Billing_view](#billing_view) [CRUD]

---

## Resources


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The resource-based policy document attached to the resource in <code>JSON</code> format. </p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the billing view resource to which the policy is attached to. </p> |


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
resource_policy_policy = resource_policy.policy
resource_policy_resource_arn = resource_policy.resource_arn
```

---


### Billing_view

BillingView resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p> The description of the billing view. </p> |
| `source_views` | Vec<String> | ✅ | <p>A list of billing views used as the data source for the custom billing view.</p> |
| `data_filter_expression` | String |  | <p> See <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_billing_Expression.html">Expression</a>. Billing view only supports <code>LINKED_ACCOUNT</code> and <code>Tags</code>. </p> |
| `resource_tags` | Vec<String> |  | <p>A list of key value map specifying tags associated to the billing view being created. </p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier you specify to ensure idempotency of the request. Idempotency ensures that an API request completes no more than one time. If the original request completes successfully, any subsequent retries complete successfully without performing any further actions with an idempotent request. </p> |
| `name` | String | ✅ | <p> The name of the billing view. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `billing_view` | String | <p>The billing view element associated with the specified ARN. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create billing_view
billing_view = provider.billing.Billing_view {
    source_views = "value"  # <p>A list of billing views used as the data source for the custom billing view.</p>
    name = "value"  # <p> The name of the billing view. </p>
}

# Access billing_view outputs
billing_view_id = billing_view.id
billing_view_billing_view = billing_view.billing_view
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_policy resources
resource_policy_0 = provider.billing.Resource_policy {
}
resource_policy_1 = provider.billing.Resource_policy {
}
resource_policy_2 = provider.billing.Resource_policy {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_policy = provider.billing.Resource_policy {
    }
```

---

## Related Documentation

- [AWS Billing Documentation](https://docs.aws.amazon.com/billing/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
