# Accessanalyzer Service



**Resources**: 8

---

## Overview

The accessanalyzer service provides access to 8 resource types:

- [Finding_v2](#finding_v2) [R]
- [Generated_policy](#generated_policy) [R]
- [Finding_recommendation](#finding_recommendation) [R]
- [Analyzed_resource](#analyzed_resource) [R]
- [Access_preview](#access_preview) [CR]
- [Finding](#finding) [R]
- [Findings](#findings) [U]
- [Findings_statistics](#findings_statistics) [R]

---

## Resources


### Finding_v2

FindingV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource` | String | <p>The resource that generated the finding.</p> |
| `status` | String | <p>The status of the finding.</p> |
| `next_token` | String | <p>A token used for pagination of results returned.</p> |
| `resource_owner_account` | String | <p>Tye Amazon Web Services account ID that owns the resource.</p> |
| `updated_at` | String | <p>The time at which the finding was updated.</p> |
| `resource_type` | String | <p>The type of the resource identified in the finding.</p> |
| `error` | String | <p>An error.</p> |
| `id` | String | <p>The ID of the finding to retrieve.</p> |
| `finding_details` | Vec<String> | <p>A localized message that explains the finding and provides guidance on how to address it.</p> |
| `analyzed_at` | String | <p>The time at which the resource-based policy or IAM entity that generated the finding was analyzed.</p> |
| `finding_type` | String | <p>The type of the finding. For external access analyzers, the type is <code>ExternalAccess</code>. For unused access analyzers, the type can be <code>UnusedIAMRole</code>, <code>UnusedIAMUserAccessKey</code>, <code>UnusedIAMUserPassword</code>, or <code>UnusedPermission</code>. For internal access analyzers, the type is <code>InternalAccess</code>.</p> |
| `created_at` | String | <p>The time at which the finding was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access finding_v2 outputs
finding_v2_id = finding_v2.id
finding_v2_resource = finding_v2.resource
finding_v2_status = finding_v2.status
finding_v2_next_token = finding_v2.next_token
finding_v2_resource_owner_account = finding_v2.resource_owner_account
finding_v2_updated_at = finding_v2.updated_at
finding_v2_resource_type = finding_v2.resource_type
finding_v2_error = finding_v2.error
finding_v2_id = finding_v2.id
finding_v2_finding_details = finding_v2.finding_details
finding_v2_analyzed_at = finding_v2.analyzed_at
finding_v2_finding_type = finding_v2.finding_type
finding_v2_created_at = finding_v2.created_at
```

---


### Generated_policy

GeneratedPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_details` | String | <p>A <code>GeneratedPolicyDetails</code> object that contains details about the generated policy.</p> |
| `generated_policy_result` | String | <p>A <code>GeneratedPolicyResult</code> object that contains the generated policies and associated details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access generated_policy outputs
generated_policy_id = generated_policy.id
generated_policy_job_details = generated_policy.job_details
generated_policy_generated_policy_result = generated_policy.generated_policy_result
```

---


### Finding_recommendation

FindingRecommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `completed_at` | String | <p>The time at which the retrieval of the finding recommendation was completed.</p> |
| `error` | String | <p>Detailed information about the reason that the retrieval of a recommendation for the finding failed.</p> |
| `resource_arn` | String | <p>The ARN of the resource of the finding.</p> |
| `started_at` | String | <p>The time at which the retrieval of the finding recommendation was started.</p> |
| `status` | String | <p>The status of the retrieval of the finding recommendation.</p> |
| `recommended_steps` | Vec<String> | <p>A group of recommended steps for the finding.</p> |
| `recommendation_type` | String | <p>The type of recommendation for the finding.</p> |
| `next_token` | String | <p>A token used for pagination of results returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access finding_recommendation outputs
finding_recommendation_id = finding_recommendation.id
finding_recommendation_completed_at = finding_recommendation.completed_at
finding_recommendation_error = finding_recommendation.error
finding_recommendation_resource_arn = finding_recommendation.resource_arn
finding_recommendation_started_at = finding_recommendation.started_at
finding_recommendation_status = finding_recommendation.status
finding_recommendation_recommended_steps = finding_recommendation.recommended_steps
finding_recommendation_recommendation_type = finding_recommendation.recommendation_type
finding_recommendation_next_token = finding_recommendation.next_token
```

---


### Analyzed_resource

AnalyzedResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource` | String | <p>An <code>AnalyzedResource</code> object that contains information that IAM Access Analyzer found when it analyzed the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access analyzed_resource outputs
analyzed_resource_id = analyzed_resource.id
analyzed_resource_resource = analyzed_resource.resource
```

---


### Access_preview

AccessPreview resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A client token.</p> |
| `analyzer_arn` | String | ✅ | <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the account analyzer</a> used to generate the access preview. You can only create an access preview for analyzers with an <code>Account</code> type and <code>Active</code> status.</p> |
| `configurations` | HashMap<String, String> | ✅ | <p>Access control configuration for your resource that is used to generate the access preview. The access preview includes findings for external access allowed to the resource with the proposed access control configuration. The configuration must contain exactly one element.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_preview` | String | <p>An object that contains information about the access preview.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_preview
access_preview = provider.accessanalyzer.Access_preview {
    analyzer_arn = "value"  # <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the account analyzer</a> used to generate the access preview. You can only create an access preview for analyzers with an <code>Account</code> type and <code>Active</code> status.</p>
    configurations = "value"  # <p>Access control configuration for your resource that is used to generate the access preview. The access preview includes findings for external access allowed to the resource with the proposed access control configuration. The configuration must contain exactly one element.</p>
}

# Access access_preview outputs
access_preview_id = access_preview.id
access_preview_access_preview = access_preview.access_preview
```

---


### Finding

Finding resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `finding` | String | <p>A <code>finding</code> object that contains finding details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access finding outputs
finding_id = finding.id
finding_finding = finding.finding
```

---


### Findings

Findings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `analyzer_arn` | String | ✅ | <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the analyzer</a> that generated the findings to update.</p> |
| `status` | String | ✅ | <p>The state represents the action to take to update the finding Status. Use <code>ARCHIVE</code> to change an Active finding to an Archived finding. Use <code>ACTIVE</code> to change an Archived finding to an Active finding.</p> |
| `resource_arn` | String |  | <p>The ARN of the resource identified in the finding.</p> |
| `client_token` | String |  | <p>A client token.</p> |
| `ids` | Vec<String> |  | <p>The IDs of the findings to update.</p> |



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


### Findings_statistics

FindingsStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_at` | String | <p>The time at which the retrieval of the findings statistics was last updated. If the findings statistics have not been previously retrieved for the specified analyzer, this field will not be populated.</p> |
| `findings_statistics` | Vec<String> | <p>A group of external access or unused access findings statistics.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings_statistics outputs
findings_statistics_id = findings_statistics.id
findings_statistics_last_updated_at = findings_statistics.last_updated_at
findings_statistics_findings_statistics = findings_statistics.findings_statistics
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple finding_v2 resources
finding_v2_0 = provider.accessanalyzer.Finding_v2 {
}
finding_v2_1 = provider.accessanalyzer.Finding_v2 {
}
finding_v2_2 = provider.accessanalyzer.Finding_v2 {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    finding_v2 = provider.accessanalyzer.Finding_v2 {
    }
```

---

## Related Documentation

- [AWS Accessanalyzer Documentation](https://docs.aws.amazon.com/accessanalyzer/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
