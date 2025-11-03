# Trustedadvisor Service



**Resources**: 4

---

## Overview

The trustedadvisor service provides access to 4 resource types:

- [Organization_recommendation_lifecycle](#organization_recommendation_lifecycle) [U]
- [Organization_recommendation](#organization_recommendation) [R]
- [Recommendation](#recommendation) [R]
- [Recommendation_lifecycle](#recommendation_lifecycle) [U]

---

## Resources


### Organization_recommendation_lifecycle

OrganizationRecommendationLifecycle resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lifecycle_stage` | String | ✅ | <p>The new lifecycle stage</p> |
| `update_reason` | String |  | <p>Reason for the lifecycle stage change</p> |
| `update_reason_code` | String |  | <p>Reason code for the lifecycle state change</p> |
| `organization_recommendation_identifier` | String | ✅ | <p>The Recommendation identifier for AWS Trusted Advisor Priority recommendations</p> |



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


### Organization_recommendation

OrganizationRecommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_recommendation` | String | <p>The Recommendation</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_recommendation outputs
organization_recommendation_id = organization_recommendation.id
organization_recommendation_organization_recommendation = organization_recommendation.organization_recommendation
```

---


### Recommendation

Recommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommendation` | String | <p>The Recommendation</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommendation outputs
recommendation_id = recommendation.id
recommendation_recommendation = recommendation.recommendation
```

---


### Recommendation_lifecycle

RecommendationLifecycle resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_reason` | String |  | <p>Reason for the lifecycle stage change</p> |
| `lifecycle_stage` | String | ✅ | <p>The new lifecycle stage</p> |
| `update_reason_code` | String |  | <p>Reason code for the lifecycle state change</p> |
| `recommendation_identifier` | String | ✅ | <p>The Recommendation identifier for AWS Trusted Advisor Priority recommendations</p> |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple organization_recommendation_lifecycle resources
organization_recommendation_lifecycle_0 = provider.trustedadvisor.Organization_recommendation_lifecycle {
    lifecycle_stage = "value-0"
    organization_recommendation_identifier = "value-0"
}
organization_recommendation_lifecycle_1 = provider.trustedadvisor.Organization_recommendation_lifecycle {
    lifecycle_stage = "value-1"
    organization_recommendation_identifier = "value-1"
}
organization_recommendation_lifecycle_2 = provider.trustedadvisor.Organization_recommendation_lifecycle {
    lifecycle_stage = "value-2"
    organization_recommendation_identifier = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    organization_recommendation_lifecycle = provider.trustedadvisor.Organization_recommendation_lifecycle {
        lifecycle_stage = "production-value"
        organization_recommendation_identifier = "production-value"
    }
```

---

## Related Documentation

- [AWS Trustedadvisor Documentation](https://docs.aws.amazon.com/trustedadvisor/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
