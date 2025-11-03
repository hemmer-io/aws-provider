# Codeguru_reviewer Service



**Resources**: 3

---

## Overview

The codeguru_reviewer service provides access to 3 resource types:

- [Code_review](#code_review) [CR]
- [Recommendation_feedback](#recommendation_feedback) [CR]
- [Repository_association](#repository_association) [R]

---

## Resources


### Code_review

CodeReview resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the code review. The name of each code review in your Amazon Web Services account must be
         unique.</p> |
| `type` | String | ✅ | <p>The type of code review to create. This is specified using a <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReviewType.html">CodeReviewType</a>
         object. You can create a code review only of type <code>RepositoryAnalysis</code>.</p> |
| `client_request_token` | String |  | <p>Amazon CodeGuru Reviewer uses this value to prevent the accidental creation of duplicate code reviews
         if there are failures and retries.</p> |
| `repository_association_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociation.html">RepositoryAssociation</a> object. You can retrieve this ARN by calling <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_ListRepositoryAssociations.html">ListRepositoryAssociations</a>.</p>
         <p>A code review can only be created on an associated repository. This is the ARN of the
         associated repository.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `code_review` | String | <p>Information about the code review.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create code_review
code_review = provider.codeguru_reviewer.Code_review {
    name = "value"  # <p>The name of the code review. The name of each code review in your Amazon Web Services account must be
         unique.</p>
    type = "value"  # <p>The type of code review to create. This is specified using a <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReviewType.html">CodeReviewType</a>
         object. You can create a code review only of type <code>RepositoryAnalysis</code>.</p>
    repository_association_arn = "value"  # <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociation.html">RepositoryAssociation</a> object. You can retrieve this ARN by calling <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_ListRepositoryAssociations.html">ListRepositoryAssociations</a>.</p>
         <p>A code review can only be created on an associated repository. This is the ARN of the
         associated repository.</p>
}

# Access code_review outputs
code_review_id = code_review.id
code_review_code_review = code_review.code_review
```

---


### Recommendation_feedback

RecommendationFeedback resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reactions` | Vec<String> | ✅ | <p>List for storing reactions. Reactions are utf-8 text code for emojis. If you send an
         empty list it clears all your feedback.</p> |
| `code_review_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html">CodeReview</a> object.
      </p> |
| `recommendation_id` | String | ✅ | <p>The recommendation ID that can be used to track the provided recommendations and then to
         collect the feedback.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommendation_feedback` | String | <p>The recommendation feedback given by the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recommendation_feedback
recommendation_feedback = provider.codeguru_reviewer.Recommendation_feedback {
    reactions = "value"  # <p>List for storing reactions. Reactions are utf-8 text code for emojis. If you send an
         empty list it clears all your feedback.</p>
    code_review_arn = "value"  # <p>The Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_CodeReview.html">CodeReview</a> object.
      </p>
    recommendation_id = "value"  # <p>The recommendation ID that can be used to track the provided recommendations and then to
         collect the feedback.</p>
}

# Access recommendation_feedback outputs
recommendation_feedback_id = recommendation_feedback.id
recommendation_feedback_recommendation_feedback = recommendation_feedback.recommendation_feedback
```

---


### Repository_association

RepositoryAssociation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repository_association` | String | <p>Information about the repository association.</p> |
| `tags` | HashMap<String, String> | <p>An array of key-value pairs used to tag an associated repository. A tag is a custom attribute label with two parts:</p>
         <ul>
            <li>
               <p>A <i>tag key</i> (for example, <code>CostCenter</code>,
					<code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag
					keys are case sensitive.</p>
            </li>
            <li>
               <p>An optional field known as a <i>tag value</i> (for example,
					<code>111122223333</code>, <code>Production</code>, or a team name).
					Omitting the tag value is the same as using an empty string. Like tag keys, tag
					values are case sensitive.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access repository_association outputs
repository_association_id = repository_association.id
repository_association_repository_association = repository_association.repository_association
repository_association_tags = repository_association.tags
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple code_review resources
code_review_0 = provider.codeguru_reviewer.Code_review {
    name = "value-0"
    type = "value-0"
    repository_association_arn = "value-0"
}
code_review_1 = provider.codeguru_reviewer.Code_review {
    name = "value-1"
    type = "value-1"
    repository_association_arn = "value-1"
}
code_review_2 = provider.codeguru_reviewer.Code_review {
    name = "value-2"
    type = "value-2"
    repository_association_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    code_review = provider.codeguru_reviewer.Code_review {
        name = "production-value"
        type = "production-value"
        repository_association_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Codeguru_reviewer Documentation](https://docs.aws.amazon.com/codeguru_reviewer/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
