# Savingsplans Service



**Resources**: 6

---

## Overview

The savingsplans service provides access to 6 resource types:

- [Savings_plans](#savings_plans) [R]
- [Savings_plans_offering_rates](#savings_plans_offering_rates) [R]
- [Queued_savings_plan](#queued_savings_plan) [D]
- [Savings_plans_offerings](#savings_plans_offerings) [R]
- [Savings_plan](#savings_plan) [C]
- [Savings_plan_rates](#savings_plan_rates) [R]

---

## Resources


### Savings_plans

SavingsPlans resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `savings_plans` | Vec<String> | <p>Information about the Savings Plans.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plans outputs
savings_plans_id = savings_plans.id
savings_plans_savings_plans = savings_plans.savings_plans
savings_plans_next_token = savings_plans.next_token
```

---


### Savings_plans_offering_rates

SavingsPlansOfferingRates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_results` | Vec<String> | <p>Information about the Savings Plans offering rates.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plans_offering_rates outputs
savings_plans_offering_rates_id = savings_plans_offering_rates.id
savings_plans_offering_rates_search_results = savings_plans_offering_rates.search_results
savings_plans_offering_rates_next_token = savings_plans_offering_rates.next_token
```

---


### Queued_savings_plan

QueuedSavingsPlan resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Savings_plans_offerings

SavingsPlansOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_results` | Vec<String> | <p>Information about the Savings Plans offerings.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plans_offerings outputs
savings_plans_offerings_id = savings_plans_offerings.id
savings_plans_offerings_search_results = savings_plans_offerings.search_results
savings_plans_offerings_next_token = savings_plans_offerings.next_token
```

---


### Savings_plan

SavingsPlan resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `purchase_time` | String |  | <p>The purchase time of the Savings Plan in UTC format
         (YYYY-MM-DDTHH:MM:SSZ).</p> |
| `savings_plan_offering_id` | String | ✅ | <p>The ID of the offering.</p> |
| `commitment` | String | ✅ | <p>The hourly commitment, in the same currency of the <code>savingsPlanOfferingId</code>.
         This is a value between 0.001 and 1 million. You cannot specify more than five digits after
         the decimal point.</p> |
| `upfront_payment_amount` | String |  | <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the
         total value of the Savings Plan. This parameter is only supported if the
         payment option is <code>Partial Upfront</code>.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
         request.</p> |
| `tags` | HashMap<String, String> |  | <p>One or more tags.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create savings_plan
savings_plan = provider.savingsplans.Savings_plan {
    savings_plan_offering_id = "value"  # <p>The ID of the offering.</p>
    commitment = "value"  # <p>The hourly commitment, in the same currency of the <code>savingsPlanOfferingId</code>.
         This is a value between 0.001 and 1 million. You cannot specify more than five digits after
         the decimal point.</p>
}

```

---


### Savings_plan_rates

SavingsPlanRates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `savings_plan_id` | String | <p>The ID of the Savings Plan.</p> |
| `search_results` | Vec<String> | <p>Information about the Savings Plan rates.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plan_rates outputs
savings_plan_rates_id = savings_plan_rates.id
savings_plan_rates_savings_plan_id = savings_plan_rates.savings_plan_id
savings_plan_rates_search_results = savings_plan_rates.search_results
savings_plan_rates_next_token = savings_plan_rates.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple savings_plans resources
savings_plans_0 = provider.savingsplans.Savings_plans {
}
savings_plans_1 = provider.savingsplans.Savings_plans {
}
savings_plans_2 = provider.savingsplans.Savings_plans {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    savings_plans = provider.savingsplans.Savings_plans {
    }
```

---

## Related Documentation

- [AWS Savingsplans Documentation](https://docs.aws.amazon.com/savingsplans/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
