# Wellarchitected Service



**Resources**: 21

---

## Overview

The wellarchitected service provides access to 21 resource types:

- [Lens_share](#lens_share) [CD]
- [Lens](#lens) [RD]
- [Lens_review_report](#lens_review_report) [R]
- [Review_template](#review_template) [CRUD]
- [Share_invitation](#share_invitation) [U]
- [Review_template_lens_review](#review_template_lens_review) [RU]
- [Integration](#integration) [U]
- [Lens_version](#lens_version) [C]
- [Workload](#workload) [CRUD]
- [Consolidated_report](#consolidated_report) [R]
- [Review_template_answer](#review_template_answer) [RU]
- [Profile_share](#profile_share) [CD]
- [Profile](#profile) [CRUD]
- [Workload_share](#workload_share) [CUD]
- [Template_share](#template_share) [CD]
- [Milestone](#milestone) [CR]
- [Answer](#answer) [RU]
- [Lens_review](#lens_review) [RU]
- [Lens_version_difference](#lens_version_difference) [R]
- [Profile_template](#profile_template) [R]
- [Global_settings](#global_settings) [RU]

---

## Resources


### Lens_share

LensShare resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shared_with` | String | ✅ |  |
| `lens_alias` | String | ✅ |  |
| `client_request_token` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lens_share
lens_share = provider.wellarchitected.Lens_share {
    shared_with = "value"  # Required field
    lens_alias = "value"  # Required field
    client_request_token = "value"  # Required field
}

```

---


### Lens

Lens resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lens` | String | <p>A lens return object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lens outputs
lens_id = lens.id
lens_lens = lens.lens
```

---


### Lens_review_report

LensReviewReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `milestone_number` | i64 |  |
| `lens_review_report` | String |  |
| `workload_id` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lens_review_report outputs
lens_review_report_id = lens_review_report.id
lens_review_report_milestone_number = lens_review_report.milestone_number
lens_review_report_lens_review_report = lens_review_report.lens_review_report
lens_review_report_workload_id = lens_review_report.workload_id
```

---


### Review_template

ReviewTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String | ✅ |  |
| `lenses` | Vec<String> | ✅ | <p>Lenses applied to the review template.</p> |
| `notes` | String |  |  |
| `description` | String | ✅ | <p>The review template description.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags assigned to the review template.</p> |
| `template_name` | String | ✅ | <p>Name of the review template.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `review_template` | String | <p>The review template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create review_template
review_template = provider.wellarchitected.Review_template {
    client_request_token = "value"  # Required field
    lenses = "value"  # <p>Lenses applied to the review template.</p>
    description = "value"  # <p>The review template description.</p>
    template_name = "value"  # <p>Name of the review template.</p>
}

# Access review_template outputs
review_template_id = review_template.id
review_template_review_template = review_template.review_template
```

---


### Share_invitation

ShareInvitation resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `share_invitation_action` | String | ✅ |  |
| `share_invitation_id` | String | ✅ | <p>The ID assigned to the share invitation.</p> |



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


### Review_template_lens_review

ReviewTemplateLensReview resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lens_alias` | String | ✅ |  |
| `lens_notes` | String |  |  |
| `template_arn` | String | ✅ | <p>The review template ARN.</p> |
| `pillar_notes` | HashMap<String, String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lens_review` | String | <p>A lens review of a question.</p> |
| `template_arn` | String | <p>The review template ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access review_template_lens_review outputs
review_template_lens_review_id = review_template_lens_review.id
review_template_lens_review_lens_review = review_template_lens_review.lens_review
review_template_lens_review_template_arn = review_template_lens_review.template_arn
```

---


### Integration

Integration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String | ✅ |  |
| `workload_id` | String | ✅ |  |
| `integrating_service` | String | ✅ | <p>Which integrated service to update.</p> |



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


### Lens_version

LensVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lens_alias` | String | ✅ |  |
| `lens_version` | String | ✅ | <p>The version of the lens being created.</p> |
| `is_major_version` | bool |  | <p>Set to true if this new major lens version.</p> |
| `client_request_token` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lens_version
lens_version = provider.wellarchitected.Lens_version {
    lens_alias = "value"  # Required field
    lens_version = "value"  # <p>The version of the lens being created.</p>
    client_request_token = "value"  # Required field
}

```

---


### Workload

Workload resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `discovery_config` | String |  | <p>Well-Architected discovery configuration settings associated to the workload.</p> |
| `jira_configuration` | String |  | <p>Jira configuration settings when creating a workload.</p> |
| `industry` | String |  |  |
| `non_aws_regions` | Vec<String> |  |  |
| `tags` | HashMap<String, String> |  | <p>The tags to be associated with the workload.</p> |
| `industry_type` | String |  |  |
| `lenses` | Vec<String> | ✅ |  |
| `description` | String | ✅ |  |
| `review_owner` | String |  |  |
| `aws_regions` | Vec<String> |  |  |
| `pillar_priorities` | Vec<String> |  |  |
| `notes` | String |  |  |
| `applications` | Vec<String> |  | <p>List of AppRegistry application ARNs associated to the workload.</p> |
| `architectural_design` | String |  |  |
| `environment` | String | ✅ |  |
| `account_ids` | Vec<String> |  |  |
| `client_request_token` | String | ✅ |  |
| `profile_arns` | Vec<String> |  | <p>The list of profile ARNs associated with the workload.</p> |
| `workload_name` | String | ✅ |  |
| `review_template_arns` | Vec<String> |  | <p>The list of review template ARNs to associate with the workload.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workload
workload = provider.wellarchitected.Workload {
    lenses = "value"  # Required field
    description = "value"  # Required field
    environment = "value"  # Required field
    client_request_token = "value"  # Required field
    workload_name = "value"  # Required field
}

# Access workload outputs
workload_id = workload.id
workload_workload = workload.workload
```

---


### Consolidated_report

ConsolidatedReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `base64_string` | String |  |
| `metrics` | Vec<String> | <p>The metrics that make up the consolidated report.</p>
         <p>Only returned when <code>JSON</code> format is requested.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access consolidated_report outputs
consolidated_report_id = consolidated_report.id
consolidated_report_base64_string = consolidated_report.base64_string
consolidated_report_metrics = consolidated_report.metrics
consolidated_report_next_token = consolidated_report.next_token
```

---


### Review_template_answer

ReviewTemplateAnswer resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `question_id` | String | ✅ |  |
| `notes` | String |  |  |
| `is_applicable` | bool |  |  |
| `reason` | String |  | <p>The update reason.</p> |
| `lens_alias` | String | ✅ |  |
| `choice_updates` | HashMap<String, String> |  | <p>A list of choices to be updated.</p> |
| `selected_choices` | Vec<String> |  |  |
| `template_arn` | String | ✅ | <p>The review template ARN.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lens_alias` | String |  |
| `answer` | String | <p>An answer of the question.</p> |
| `template_arn` | String | <p>The review template ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access review_template_answer outputs
review_template_answer_id = review_template_answer.id
review_template_answer_lens_alias = review_template_answer.lens_alias
review_template_answer_answer = review_template_answer.answer
review_template_answer_template_arn = review_template_answer.template_arn
```

---


### Profile_share

ProfileShare resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `profile_arn` | String | ✅ | <p>The profile ARN.</p> |
| `client_request_token` | String | ✅ |  |
| `shared_with` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile_share
profile_share = provider.wellarchitected.Profile_share {
    profile_arn = "value"  # <p>The profile ARN.</p>
    client_request_token = "value"  # Required field
    shared_with = "value"  # Required field
}

```

---


### Profile

Profile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `profile_questions` | Vec<String> | ✅ | <p>The profile questions.</p> |
| `profile_description` | String | ✅ | <p>The profile description.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags assigned to the profile.</p> |
| `profile_name` | String | ✅ | <p>Name of the profile.</p> |
| `client_request_token` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile` | String | <p>The profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile
profile = provider.wellarchitected.Profile {
    profile_questions = "value"  # <p>The profile questions.</p>
    profile_description = "value"  # <p>The profile description.</p>
    profile_name = "value"  # <p>Name of the profile.</p>
    client_request_token = "value"  # Required field
}

# Access profile outputs
profile_id = profile.id
profile_profile = profile.profile
```

---


### Workload_share

WorkloadShare resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workload_id` | String | ✅ |  |
| `shared_with` | String | ✅ |  |
| `permission_type` | String | ✅ |  |
| `client_request_token` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workload_share
workload_share = provider.wellarchitected.Workload_share {
    workload_id = "value"  # Required field
    shared_with = "value"  # Required field
    permission_type = "value"  # Required field
    client_request_token = "value"  # Required field
}

```

---


### Template_share

TemplateShare resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String | ✅ |  |
| `shared_with` | String | ✅ |  |
| `template_arn` | String | ✅ | <p>The review template ARN.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create template_share
template_share = provider.wellarchitected.Template_share {
    client_request_token = "value"  # Required field
    shared_with = "value"  # Required field
    template_arn = "value"  # <p>The review template ARN.</p>
}

```

---


### Milestone

Milestone resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `milestone_name` | String | ✅ |  |
| `client_request_token` | String | ✅ |  |
| `workload_id` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `milestone` | String |  |
| `workload_id` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create milestone
milestone = provider.wellarchitected.Milestone {
    milestone_name = "value"  # Required field
    client_request_token = "value"  # Required field
    workload_id = "value"  # Required field
}

# Access milestone outputs
milestone_id = milestone.id
milestone_milestone = milestone.milestone
milestone_workload_id = milestone.workload_id
```

---


### Answer

Answer resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reason` | String |  | <p>The reason why a question is not applicable to your workload.</p> |
| `lens_alias` | String | ✅ |  |
| `question_id` | String | ✅ |  |
| `notes` | String |  |  |
| `is_applicable` | bool |  |  |
| `workload_id` | String | ✅ |  |
| `selected_choices` | Vec<String> |  |  |
| `choice_updates` | HashMap<String, String> |  | <p>A list of choices to update on a question in your workload.  The String key 
            corresponds to the choice ID to be updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lens_alias` | String |  |
| `milestone_number` | i64 |  |
| `workload_id` | String |  |
| `answer` | String |  |
| `lens_arn` | String | <p>The ARN for the lens.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access answer outputs
answer_id = answer.id
answer_lens_alias = answer.lens_alias
answer_milestone_number = answer.milestone_number
answer_workload_id = answer.workload_id
answer_answer = answer.answer
answer_lens_arn = answer.lens_arn
```

---


### Lens_review

LensReview resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pillar_notes` | HashMap<String, String> |  |  |
| `lens_alias` | String | ✅ |  |
| `workload_id` | String | ✅ |  |
| `jira_configuration` | String |  | <p>Configuration of the Jira integration.</p> |
| `lens_notes` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lens_review` | String |  |
| `workload_id` | String |  |
| `milestone_number` | i64 |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lens_review outputs
lens_review_id = lens_review.id
lens_review_lens_review = lens_review.lens_review
lens_review_workload_id = lens_review.workload_id
lens_review_milestone_number = lens_review.milestone_number
```

---


### Lens_version_difference

LensVersionDifference resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lens_alias` | String |  |
| `latest_lens_version` | String | <p>The latest version of the lens.</p> |
| `base_lens_version` | String | <p>The base version of the lens.</p> |
| `lens_arn` | String | <p>The ARN for the lens.</p> |
| `version_differences` | String |  |
| `target_lens_version` | String | <p>The target lens version for the lens.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lens_version_difference outputs
lens_version_difference_id = lens_version_difference.id
lens_version_difference_lens_alias = lens_version_difference.lens_alias
lens_version_difference_latest_lens_version = lens_version_difference.latest_lens_version
lens_version_difference_base_lens_version = lens_version_difference.base_lens_version
lens_version_difference_lens_arn = lens_version_difference.lens_arn
lens_version_difference_version_differences = lens_version_difference.version_differences
lens_version_difference_target_lens_version = lens_version_difference.target_lens_version
```

---


### Profile_template

ProfileTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile_template` | String | <p>The profile template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access profile_template outputs
profile_template_id = profile_template.id
profile_template_profile_template = profile_template.profile_template
```

---


### Global_settings

GlobalSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_sharing_status` | String |  | <p>The status of organization sharing settings.</p> |
| `discovery_integration_status` | String |  | <p>The status of discovery support settings.</p> |
| `jira_configuration` | String |  | <p>The status of Jira integration settings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `discovery_integration_status` | String | <p>Discovery integration status.</p> |
| `jira_configuration` | String | <p>Jira configuration status.</p> |
| `organization_sharing_status` | String | <p>Amazon Web Services Organizations sharing status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_settings outputs
global_settings_id = global_settings.id
global_settings_discovery_integration_status = global_settings.discovery_integration_status
global_settings_jira_configuration = global_settings.jira_configuration
global_settings_organization_sharing_status = global_settings.organization_sharing_status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple lens_share resources
lens_share_0 = provider.wellarchitected.Lens_share {
    shared_with = "value-0"
    lens_alias = "value-0"
    client_request_token = "value-0"
}
lens_share_1 = provider.wellarchitected.Lens_share {
    shared_with = "value-1"
    lens_alias = "value-1"
    client_request_token = "value-1"
}
lens_share_2 = provider.wellarchitected.Lens_share {
    shared_with = "value-2"
    lens_alias = "value-2"
    client_request_token = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    lens_share = provider.wellarchitected.Lens_share {
        shared_with = "production-value"
        lens_alias = "production-value"
        client_request_token = "production-value"
    }
```

---

## Related Documentation

- [AWS Wellarchitected Documentation](https://docs.aws.amazon.com/wellarchitected/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
