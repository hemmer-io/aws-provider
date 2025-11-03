# Inspector Service



**Resources**: 15

---

## Overview

The inspector service provides access to 15 resource types:

- [Assessment_target](#assessment_target) [CUD]
- [Exclusions_preview](#exclusions_preview) [CR]
- [Findings](#findings) [R]
- [Resource_groups](#resource_groups) [R]
- [Assessment_run](#assessment_run) [D]
- [Exclusions](#exclusions) [R]
- [Rules_packages](#rules_packages) [R]
- [Assessment_targets](#assessment_targets) [R]
- [Assessment_report](#assessment_report) [R]
- [Assessment_runs](#assessment_runs) [R]
- [Assessment_templates](#assessment_templates) [R]
- [Assessment_template](#assessment_template) [CD]
- [Cross_account_access_role](#cross_account_access_role) [R]
- [Telemetry_metadata](#telemetry_metadata) [R]
- [Resource_group](#resource_group) [C]

---

## Resources


### Assessment_target

AssessmentTarget resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_group_arn` | String |  | <p>The ARN that specifies the resource group that is used to create the assessment
         target. If resourceGroupArn is not specified, all EC2 instances in the current AWS account
         and region are included in the assessment target.</p> |
| `assessment_target_name` | String | ✅ | <p>The user-defined name that identifies the assessment target that you want to create.
         The name must be unique within the AWS account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create assessment_target
assessment_target = provider.inspector.Assessment_target {
    assessment_target_name = "value"  # <p>The user-defined name that identifies the assessment target that you want to create.
         The name must be unique within the AWS account.</p>
}

```

---


### Exclusions_preview

ExclusionsPreview resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assessment_template_arn` | String | ✅ | <p>The ARN that specifies the assessment template for which you want to create an
         exclusions preview.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `preview_status` | String | <p>Specifies the status of the request to generate an exclusions preview.</p> |
| `exclusion_previews` | Vec<String> | <p>Information about the exclusions included in the preview.</p> |
| `next_token` | String | <p>When a response is generated, if there is more data to be listed, this parameters is
         present in the response and contains the value to use for the nextToken parameter in a
         subsequent pagination request. If there is no more data to be listed, this parameter is set
         to null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create exclusions_preview
exclusions_preview = provider.inspector.Exclusions_preview {
    assessment_template_arn = "value"  # <p>The ARN that specifies the assessment template for which you want to create an
         exclusions preview.</p>
}

# Access exclusions_preview outputs
exclusions_preview_id = exclusions_preview.id
exclusions_preview_preview_status = exclusions_preview.preview_status
exclusions_preview_exclusion_previews = exclusions_preview.exclusion_previews
exclusions_preview_next_token = exclusions_preview.next_token
```

---


### Findings

Findings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failed_items` | HashMap<String, String> | <p>Finding details that cannot be described. An error code is provided for each failed
         item.</p> |
| `findings` | Vec<String> | <p>Information about the finding.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings outputs
findings_id = findings.id
findings_failed_items = findings.failed_items
findings_findings = findings.findings
```

---


### Resource_groups

ResourceGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failed_items` | HashMap<String, String> | <p>Resource group details that cannot be described. An error code is provided for each
         failed item.</p> |
| `resource_groups` | Vec<String> | <p>Information about a resource group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_groups outputs
resource_groups_id = resource_groups.id
resource_groups_failed_items = resource_groups.failed_items
resource_groups_resource_groups = resource_groups.resource_groups
```

---


### Assessment_run

AssessmentRun resource

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


### Exclusions

Exclusions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `exclusions` | HashMap<String, String> | <p>Information about the exclusions.</p> |
| `failed_items` | HashMap<String, String> | <p>Exclusion details that cannot be described. An error code is provided for each failed
         item.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access exclusions outputs
exclusions_id = exclusions.id
exclusions_exclusions = exclusions.exclusions
exclusions_failed_items = exclusions.failed_items
```

---


### Rules_packages

RulesPackages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failed_items` | HashMap<String, String> | <p>Rules package details that cannot be described. An error code is provided for each
         failed item.</p> |
| `rules_packages` | Vec<String> | <p>Information about the rules package.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rules_packages outputs
rules_packages_id = rules_packages.id
rules_packages_failed_items = rules_packages.failed_items
rules_packages_rules_packages = rules_packages.rules_packages
```

---


### Assessment_targets

AssessmentTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assessment_targets` | Vec<String> | <p>Information about the assessment targets.</p> |
| `failed_items` | HashMap<String, String> | <p>Assessment target details that cannot be described. An error code is provided for
         each failed item.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access assessment_targets outputs
assessment_targets_id = assessment_targets.id
assessment_targets_assessment_targets = assessment_targets.assessment_targets
assessment_targets_failed_items = assessment_targets.failed_items
```

---


### Assessment_report

AssessmentReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | <p>Specifies the URL where you can find the generated assessment report. This parameter
         is only returned if the report is successfully generated.</p> |
| `status` | String | <p>Specifies the status of the request to generate an assessment report. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access assessment_report outputs
assessment_report_id = assessment_report.id
assessment_report_url = assessment_report.url
assessment_report_status = assessment_report.status
```

---


### Assessment_runs

AssessmentRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assessment_runs` | Vec<String> | <p>Information about the assessment run.</p> |
| `failed_items` | HashMap<String, String> | <p>Assessment run details that cannot be described. An error code is provided for each
         failed item.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access assessment_runs outputs
assessment_runs_id = assessment_runs.id
assessment_runs_assessment_runs = assessment_runs.assessment_runs
assessment_runs_failed_items = assessment_runs.failed_items
```

---


### Assessment_templates

AssessmentTemplates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failed_items` | HashMap<String, String> | <p>Assessment template details that cannot be described. An error code is provided for
         each failed item.</p> |
| `assessment_templates` | Vec<String> | <p>Information about the assessment templates.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access assessment_templates outputs
assessment_templates_id = assessment_templates.id
assessment_templates_failed_items = assessment_templates.failed_items
assessment_templates_assessment_templates = assessment_templates.assessment_templates
```

---


### Assessment_template

AssessmentTemplate resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_attributes_for_findings` | Vec<String> |  | <p>The user-defined attributes that are assigned to every finding that is generated by
         the assessment run that uses this assessment template. An attribute is a key and value pair
         (an <a>Attribute</a> object). Within an assessment template, each key must be
         unique.</p> |
| `assessment_target_arn` | String | ✅ | <p>The ARN that specifies the assessment target for which you want to create the
         assessment template.</p> |
| `rules_package_arns` | Vec<String> | ✅ | <p>The ARNs that specify the rules packages that you want to attach to the assessment
         template.</p> |
| `duration_in_seconds` | i64 | ✅ | <p>The duration of the assessment run in seconds.</p> |
| `assessment_template_name` | String | ✅ | <p>The user-defined name that identifies the assessment template that you want to
         create. You can create several assessment templates for an assessment target. The names of
         the assessment templates that correspond to a particular assessment target must be
         unique.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create assessment_template
assessment_template = provider.inspector.Assessment_template {
    assessment_target_arn = "value"  # <p>The ARN that specifies the assessment target for which you want to create the
         assessment template.</p>
    rules_package_arns = "value"  # <p>The ARNs that specify the rules packages that you want to attach to the assessment
         template.</p>
    duration_in_seconds = "value"  # <p>The duration of the assessment run in seconds.</p>
    assessment_template_name = "value"  # <p>The user-defined name that identifies the assessment template that you want to
         create. You can create several assessment templates for an assessment target. The names of
         the assessment templates that correspond to a particular assessment target must be
         unique.</p>
}

```

---


### Cross_account_access_role

CrossAccountAccessRole resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_arn` | String | <p>The ARN that specifies the IAM role that Amazon Inspector uses to access your AWS
         account.</p> |
| `valid` | bool | <p>A Boolean value that specifies whether the IAM role has the necessary policies
         attached to enable Amazon Inspector to access your AWS account.</p> |
| `registered_at` | String | <p>The date when the cross-account access role was registered.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cross_account_access_role outputs
cross_account_access_role_id = cross_account_access_role.id
cross_account_access_role_role_arn = cross_account_access_role.role_arn
cross_account_access_role_valid = cross_account_access_role.valid
cross_account_access_role_registered_at = cross_account_access_role.registered_at
```

---


### Telemetry_metadata

TelemetryMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `telemetry_metadata` | Vec<String> | <p>Telemetry details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access telemetry_metadata outputs
telemetry_metadata_id = telemetry_metadata.id
telemetry_metadata_telemetry_metadata = telemetry_metadata.telemetry_metadata
```

---


### Resource_group

ResourceGroup resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_group_tags` | Vec<String> | ✅ | <p>A collection of keys and an array of possible values,
         '[{"key":"key1","values":["Value1","Value2"]},{"key":"Key2","values":["Value3"]}]'.</p>
         <p>For example,'[{"key":"Name","values":["TestEC2Instance"]}]'.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_group
resource_group = provider.inspector.Resource_group {
    resource_group_tags = "value"  # <p>A collection of keys and an array of possible values,
         '[{"key":"key1","values":["Value1","Value2"]},{"key":"Key2","values":["Value3"]}]'.</p>
         <p>For example,'[{"key":"Name","values":["TestEC2Instance"]}]'.</p>
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

# Create multiple assessment_target resources
assessment_target_0 = provider.inspector.Assessment_target {
    assessment_target_name = "value-0"
}
assessment_target_1 = provider.inspector.Assessment_target {
    assessment_target_name = "value-1"
}
assessment_target_2 = provider.inspector.Assessment_target {
    assessment_target_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    assessment_target = provider.inspector.Assessment_target {
        assessment_target_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Inspector Documentation](https://docs.aws.amazon.com/inspector/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
