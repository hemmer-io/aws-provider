# Cost_optimization_hub Service



**Resources**: 3

---

## Overview

The cost_optimization_hub service provides access to 3 resource types:

- [Recommendation](#recommendation) [R]
- [Enrollment_status](#enrollment_status) [U]
- [Preferences](#preferences) [RU]

---

## Resources


### Recommendation

Recommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `current_resource_details` | String | <p>The details for the resource.</p> |
| `account_id` | String | <p>The account to which the recommendation applies.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the resource.</p> |
| `currency_code` | String | <p>The currency code used for the recommendation.</p> |
| `cost_calculation_lookback_period_in_days` | i64 | <p>The lookback period used to calculate cost impact for a recommendation.</p> |
| `rollback_possible` | bool | <p>Whether or not implementing the recommendation can be rolled back.</p> |
| `recommendation_lookback_period_in_days` | i64 | <p>The lookback period that's used to generate the recommendation.</p> |
| `tags` | Vec<String> | <p>A list of tags associated with the resource for which the recommendation exists.</p> |
| `last_refresh_timestamp` | String | <p>The time when the recommendation was last generated.</p> |
| `region` | String | <p>The Amazon Web Services Region of the resource.</p> |
| `recommended_resource_type` | String | <p>The resource type of the recommendation.</p> |
| `restart_needed` | bool | <p>Whether or not implementing the recommendation requires a restart.</p> |
| `recommended_resource_details` | String | <p>The details about the recommended resource.</p> |
| `recommendation_id` | String | <p>The ID for the recommendation.</p> |
| `current_resource_type` | String | <p>The type of resource.</p> |
| `estimated_monthly_cost` | f64 | <p>The estimated monthly cost of the current resource. For Reserved Instances and Savings Plans, it refers to the cost for eligible usage.</p> |
| `estimated_savings_percentage` | f64 | <p>The estimated savings percentage relative to the total cost over the cost calculation lookback period.</p> |
| `resource_id` | String | <p>The unique identifier for the resource. This is the same as the Amazon Resource Name (ARN), if available.</p> |
| `implementation_effort` | String | <p>The effort required to implement the recommendation.</p> |
| `action_type` | String | <p>The type of action you can take by adopting the recommendation.</p> |
| `source` | String | <p>The source of the recommendation.</p> |
| `estimated_savings_over_cost_calculation_lookback_period` | f64 | <p>The estimated savings amount over the lookback period used to calculate cost impact for a recommendation.</p> |
| `estimated_monthly_savings` | f64 | <p>The estimated monthly savings amount for the recommendation.</p> |


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
recommendation_current_resource_details = recommendation.current_resource_details
recommendation_account_id = recommendation.account_id
recommendation_resource_arn = recommendation.resource_arn
recommendation_currency_code = recommendation.currency_code
recommendation_cost_calculation_lookback_period_in_days = recommendation.cost_calculation_lookback_period_in_days
recommendation_rollback_possible = recommendation.rollback_possible
recommendation_recommendation_lookback_period_in_days = recommendation.recommendation_lookback_period_in_days
recommendation_tags = recommendation.tags
recommendation_last_refresh_timestamp = recommendation.last_refresh_timestamp
recommendation_region = recommendation.region
recommendation_recommended_resource_type = recommendation.recommended_resource_type
recommendation_restart_needed = recommendation.restart_needed
recommendation_recommended_resource_details = recommendation.recommended_resource_details
recommendation_recommendation_id = recommendation.recommendation_id
recommendation_current_resource_type = recommendation.current_resource_type
recommendation_estimated_monthly_cost = recommendation.estimated_monthly_cost
recommendation_estimated_savings_percentage = recommendation.estimated_savings_percentage
recommendation_resource_id = recommendation.resource_id
recommendation_implementation_effort = recommendation.implementation_effort
recommendation_action_type = recommendation.action_type
recommendation_source = recommendation.source
recommendation_estimated_savings_over_cost_calculation_lookback_period = recommendation.estimated_savings_over_cost_calculation_lookback_period
recommendation_estimated_monthly_savings = recommendation.estimated_monthly_savings
```

---


### Enrollment_status

EnrollmentStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `include_member_accounts` | bool |  | <p>Indicates whether to enroll member accounts of the organization if the account is the management account or delegated administrator.</p> |
| `status` | String | ✅ | <p>Sets the account status.</p> |



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


### Preferences

Preferences resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_account_discount_visibility` | String |  | <p>Sets the "member account discount visibility" preference.</p> |
| `preferred_commitment` | String |  | <p>Sets the preferences for how Reserved Instances and Savings Plans cost-saving opportunities are prioritized in terms of payment option and term length.</p> |
| `savings_estimation_mode` | String |  | <p>Sets the "savings estimation mode" preference.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `savings_estimation_mode` | String | <p>Retrieves the status of the "savings estimation mode" preference.</p> |
| `member_account_discount_visibility` | String | <p>Retrieves the status of the "member account discount visibility" preference.</p> |
| `preferred_commitment` | String | <p>Retrieves the current preferences for how Reserved Instances and Savings Plans cost-saving opportunities are prioritized in terms of payment option and term length.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access preferences outputs
preferences_id = preferences.id
preferences_savings_estimation_mode = preferences.savings_estimation_mode
preferences_member_account_discount_visibility = preferences.member_account_discount_visibility
preferences_preferred_commitment = preferences.preferred_commitment
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple recommendation resources
recommendation_0 = provider.cost_optimization_hub.Recommendation {
}
recommendation_1 = provider.cost_optimization_hub.Recommendation {
}
recommendation_2 = provider.cost_optimization_hub.Recommendation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    recommendation = provider.cost_optimization_hub.Recommendation {
    }
```

---

## Related Documentation

- [AWS Cost_optimization_hub Documentation](https://docs.aws.amazon.com/cost_optimization_hub/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
