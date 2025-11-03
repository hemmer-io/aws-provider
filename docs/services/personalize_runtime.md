# Personalize_runtime Service



**Resources**: 3

---

## Overview

The personalize_runtime service provides access to 3 resource types:

- [Personalized_ranking](#personalized_ranking) [R]
- [Recommendations](#recommendations) [R]
- [Action_recommendations](#action_recommendations) [R]

---

## Resources


### Personalized_ranking

PersonalizedRanking resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `personalized_ranking` | Vec<String> | <p>A list of items in order of most likely interest to the user. The maximum is 500.</p> |
| `recommendation_id` | String | <p>The ID of the recommendation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access personalized_ranking outputs
personalized_ranking_id = personalized_ranking.id
personalized_ranking_personalized_ranking = personalized_ranking.personalized_ranking
personalized_ranking_recommendation_id = personalized_ranking.recommendation_id
```

---


### Recommendations

Recommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `item_list` | Vec<String> | <p>A list of recommendations sorted in descending order by prediction score. There can be a
      maximum of 500 items in the list.</p> |
| `recommendation_id` | String | <p>The ID of the recommendation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommendations outputs
recommendations_id = recommendations.id
recommendations_item_list = recommendations.item_list
recommendations_recommendation_id = recommendations.recommendation_id
```

---


### Action_recommendations

ActionRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommendation_id` | String | <p>The ID of the recommendation.</p> |
| `action_list` | Vec<String> | <p>A list of action recommendations sorted in descending order by prediction score. There can be a maximum of 100 actions
      in the list. For information about action scores, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/how-action-recommendation-scoring-works.html">How action recommendation scoring
      works</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access action_recommendations outputs
action_recommendations_id = action_recommendations.id
action_recommendations_recommendation_id = action_recommendations.recommendation_id
action_recommendations_action_list = action_recommendations.action_list
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple personalized_ranking resources
personalized_ranking_0 = provider.personalize_runtime.Personalized_ranking {
}
personalized_ranking_1 = provider.personalize_runtime.Personalized_ranking {
}
personalized_ranking_2 = provider.personalize_runtime.Personalized_ranking {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    personalized_ranking = provider.personalize_runtime.Personalized_ranking {
    }
```

---

## Related Documentation

- [AWS Personalize_runtime Documentation](https://docs.aws.amazon.com/personalize_runtime/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
