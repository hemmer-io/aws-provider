# Auditmanager Service



**Resources**: 23

---

## Overview

The auditmanager service provides access to 23 resource types:

- [Assessment_control](#assessment_control) [U]
- [Assessment_report](#assessment_report) [CD]
- [Assessment_control_set_status](#assessment_control_set_status) [U]
- [Organization_admin_account](#organization_admin_account) [R]
- [Assessment_status](#assessment_status) [U]
- [Evidence_file_upload_url](#evidence_file_upload_url) [R]
- [Assessment](#assessment) [CRUD]
- [Change_logs](#change_logs) [R]
- [Insights_by_assessment](#insights_by_assessment) [R]
- [Evidence_by_evidence_folder](#evidence_by_evidence_folder) [R]
- [Assessment_framework](#assessment_framework) [CRUD]
- [Insights](#insights) [R]
- [Control](#control) [CRUD]
- [Services_in_scope](#services_in_scope) [R]
- [Evidence](#evidence) [R]
- [Assessment_framework_share](#assessment_framework_share) [UD]
- [Evidence_folder](#evidence_folder) [R]
- [Settings](#settings) [RU]
- [Evidence_folders_by_assessment](#evidence_folders_by_assessment) [R]
- [Assessment_report_url](#assessment_report_url) [R]
- [Account_status](#account_status) [R]
- [Delegations](#delegations) [R]
- [Evidence_folders_by_assessment_control](#evidence_folders_by_assessment_control) [R]

---

## Resources


### Assessment_control

AssessmentControl resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `control_set_id` | String | ✅ | <p> The unique identifier for the control set. </p> |
| `comment_body` | String |  | <p> The comment body text for the control. </p> |
| `assessment_id` | String | ✅ | <p> The unique identifier for the assessment. </p> |
| `control_id` | String | ✅ | <p> The unique identifier for the control. </p> |
| `control_status` | String |  | <p> The status of the control. </p> |



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


### Assessment_report

AssessmentReport resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_statement` | String |  | <p>A SQL statement that represents an evidence finder query.</p>
         <p>Provide this parameter when you want to generate an assessment report from the results
         of an evidence finder search query. When you use this parameter, Audit Manager
         generates a one-time report using only the evidence from the query output. This report does
         not include any assessment evidence that was manually <a href="https://docs.aws.amazon.com/audit-manager/latest/userguide/generate-assessment-report.html#generate-assessment-report-include-evidence">added to a report using the console</a>, or <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_BatchAssociateAssessmentReportEvidence.html">associated with a report using the API</a>. </p>
         <p>To use this parameter, the <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_EvidenceFinderEnablement.html#auditmanager-Type-EvidenceFinderEnablement-enablementStatus">enablementStatus</a> of evidence finder must be <code>ENABLED</code>. </p>
         <p> For examples and help resolving <code>queryStatement</code> validation exceptions, see
            <a href="https://docs.aws.amazon.com/audit-manager/latest/userguide/evidence-finder-issues.html#querystatement-exceptions">Troubleshooting evidence finder issues</a> in the
               <i>Audit Manager User Guide.</i>
         </p> |
| `description` | String |  | <p> The description of the assessment report. </p> |
| `name` | String | ✅ | <p> The name of the new assessment report. </p> |
| `assessment_id` | String | ✅ | <p> The identifier for the assessment. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create assessment_report
assessment_report = provider.auditmanager.Assessment_report {
    name = "value"  # <p> The name of the new assessment report. </p>
    assessment_id = "value"  # <p> The identifier for the assessment. </p>
}

```

---


### Assessment_control_set_status

AssessmentControlSetStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment` | String | ✅ | <p> The comment that's related to the status update. </p> |
| `control_set_id` | String | ✅ | <p> The unique identifier for the control set. </p> |
| `status` | String | ✅ | <p> The status of the control set that's being updated. </p> |
| `assessment_id` | String | ✅ | <p> The unique identifier for the assessment. </p> |



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


### Organization_admin_account

OrganizationAdminAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `admin_account_id` | String | <p> The identifier for the administrator account. </p> |
| `organization_id` | String | <p> The identifier for the organization. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_admin_account outputs
organization_admin_account_id = organization_admin_account.id
organization_admin_account_admin_account_id = organization_admin_account.admin_account_id
organization_admin_account_organization_id = organization_admin_account.organization_id
```

---


### Assessment_status

AssessmentStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assessment_id` | String | ✅ | <p> The unique identifier for the assessment. </p> |
| `status` | String | ✅ | <p> The current status of the assessment. </p> |



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


### Evidence_file_upload_url

EvidenceFileUploadUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evidence_file_name` | String | <p>The name of the uploaded manual evidence file that the presigned URL was generated
         for.</p> |
| `upload_url` | String | <p>The presigned URL that was generated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access evidence_file_upload_url outputs
evidence_file_upload_url_id = evidence_file_upload_url.id
evidence_file_upload_url_evidence_file_name = evidence_file_upload_url.evidence_file_name
evidence_file_upload_url_upload_url = evidence_file_upload_url.upload_url
```

---


### Assessment

Assessment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assessment_reports_destination` | String | ✅ | <p> The assessment report storage destination for the assessment that's being created.
      </p> |
| `tags` | HashMap<String, String> |  | <p> The tags that are associated with the assessment. </p> |
| `roles` | Vec<String> | ✅ | <p> The list of roles for the assessment. </p> |
| `name` | String | ✅ | <p> The name of the assessment to be created. </p> |
| `framework_id` | String | ✅ | <p> The identifier for the framework that the assessment will be created from. </p> |
| `description` | String |  | <p> The optional description of the assessment to be created. </p> |
| `scope` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assessment` | String |  |
| `user_role` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create assessment
assessment = provider.auditmanager.Assessment {
    assessment_reports_destination = "value"  # <p> The assessment report storage destination for the assessment that's being created.
      </p>
    roles = "value"  # <p> The list of roles for the assessment. </p>
    name = "value"  # <p> The name of the assessment to be created. </p>
    framework_id = "value"  # <p> The identifier for the framework that the assessment will be created from. </p>
    scope = "value"  # Required field
}

# Access assessment outputs
assessment_id = assessment.id
assessment_assessment = assessment.assessment
assessment_user_role = assessment.user_role
```

---


### Change_logs

ChangeLogs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_logs` | Vec<String> | <p>The list of user activity for the control. </p> |
| `next_token` | String | <p>The pagination token that's used to fetch the next set of results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access change_logs outputs
change_logs_id = change_logs.id
change_logs_change_logs = change_logs.change_logs
change_logs_next_token = change_logs.next_token
```

---


### Insights_by_assessment

InsightsByAssessment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insights` | String | <p> The assessment analytics data that the <code>GetInsightsByAssessment</code> API
         returned. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insights_by_assessment outputs
insights_by_assessment_id = insights_by_assessment.id
insights_by_assessment_insights = insights_by_assessment.insights
```

---


### Evidence_by_evidence_folder

EvidenceByEvidenceFolder resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evidence` | Vec<String> | <p> The list of evidence that the <code>GetEvidenceByEvidenceFolder</code> API returned.
      </p> |
| `next_token` | String | <p> The pagination token that's used to fetch the next set of results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access evidence_by_evidence_folder outputs
evidence_by_evidence_folder_id = evidence_by_evidence_folder.id
evidence_by_evidence_folder_evidence = evidence_by_evidence_folder.evidence
evidence_by_evidence_folder_next_token = evidence_by_evidence_folder.next_token
```

---


### Assessment_framework

AssessmentFramework resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p> The name of the new custom framework. </p> |
| `description` | String |  | <p> An optional description for the new custom framework. </p> |
| `compliance_type` | String |  | <p> The compliance type that the new custom framework supports, such as CIS or HIPAA.
      </p> |
| `control_sets` | Vec<String> | ✅ | <p> The control sets that are associated with the framework. </p>
         <note>
            <p>The <code>Controls</code> object returns a partial response when called through Framework
            APIs. For a complete <code>Controls</code> object, use <code>GetControl</code>.</p>
         </note> |
| `tags` | HashMap<String, String> |  | <p> The tags that are associated with the framework. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `framework` | String | <p> The framework that the <code>GetAssessmentFramework</code> API returned. </p>
         <note>
            <p>The <code>Controls</code> object returns a partial response when called through
            Framework APIs. For a complete <code>Controls</code> object, use
            <code>GetControl</code>.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create assessment_framework
assessment_framework = provider.auditmanager.Assessment_framework {
    name = "value"  # <p> The name of the new custom framework. </p>
    control_sets = "value"  # <p> The control sets that are associated with the framework. </p>
         <note>
            <p>The <code>Controls</code> object returns a partial response when called through Framework
            APIs. For a complete <code>Controls</code> object, use <code>GetControl</code>.</p>
         </note>
}

# Access assessment_framework outputs
assessment_framework_id = assessment_framework.id
assessment_framework_framework = assessment_framework.framework
```

---


### Insights

Insights resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insights` | String | <p>The analytics data that the <code>GetInsights</code> API returned. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insights outputs
insights_id = insights.id
insights_insights = insights.insights
```

---


### Control

Control resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `control_mapping_sources` | Vec<String> | ✅ | <p> The data mapping sources for the control. </p> |
| `testing_information` | String |  | <p> The steps to follow to determine if the control is satisfied. </p> |
| `name` | String | ✅ | <p> The name of the control. </p> |
| `description` | String |  | <p> The description of the control. </p> |
| `tags` | HashMap<String, String> |  | <p> The tags that are associated with the control. </p> |
| `action_plan_instructions` | String |  | <p> The recommended actions to carry out if the control isn't fulfilled. </p> |
| `action_plan_title` | String |  | <p> The title of the action plan for remediating the control. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `control` | String | <p> The details of the control that the <code>GetControl</code> API returned. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create control
control = provider.auditmanager.Control {
    control_mapping_sources = "value"  # <p> The data mapping sources for the control. </p>
    name = "value"  # <p> The name of the control. </p>
}

# Access control outputs
control_id = control.id
control_control = control.control
```

---


### Services_in_scope

ServicesInScope resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_metadata` | Vec<String> | <p> The metadata that's associated with the Amazon Web Services service. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access services_in_scope outputs
services_in_scope_id = services_in_scope.id
services_in_scope_service_metadata = services_in_scope.service_metadata
```

---


### Evidence

Evidence resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evidence` | String | <p> The evidence that the <code>GetEvidence</code> API returned. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access evidence outputs
evidence_id = evidence.id
evidence_evidence = evidence.evidence
```

---


### Assessment_framework_share

AssessmentFrameworkShare resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String | ✅ | <p>Specifies the update action for the share request.</p> |
| `request_id` | String | ✅ | <p> The unique identifier for the share request. </p> |
| `request_type` | String | ✅ | <p>Specifies whether the share request is a sent request or a received request.</p> |



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


### Evidence_folder

EvidenceFolder resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evidence_folder` | String | <p> The folder that the evidence is stored in. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access evidence_folder outputs
evidence_folder_id = evidence_folder.id
evidence_folder_evidence_folder = evidence_folder.evidence_folder
```

---


### Settings

Settings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sns_topic` | String |  | <p> The Amazon Simple Notification Service (Amazon SNS) topic that Audit Manager sends
         notifications to. </p> |
| `evidence_finder_enabled` | bool |  | <p>Specifies whether the evidence finder feature is enabled. Change this attribute to
         enable or disable evidence finder.</p>
         <important>
            <p>When you use this attribute to disable evidence finder, Audit Manager deletes the
            event data store that’s used to query your evidence data. As a result, you can’t
            re-enable evidence finder and use the feature again. Your only alternative is to <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_DeregisterAccount.html">deregister</a> and then <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_RegisterAccount.html">re-register</a>
            Audit Manager. </p>
         </important> |
| `deregistration_policy` | String |  | <p>The deregistration policy for your Audit Manager data. You can
         use this attribute to determine how your data is handled when you deregister Audit Manager.</p> |
| `default_assessment_reports_destination` | String |  | <p> The default S3 destination bucket for storing assessment reports. </p> |
| `default_export_destination` | String |  | <p> The default S3 destination bucket for storing evidence finder exports. </p> |
| `default_process_owners` | Vec<String> |  | <p> A list of the default audit owners. </p> |
| `kms_key` | String |  | <p> The KMS key details. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `settings` | String | <p> The settings object that holds all supported Audit Manager settings. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access settings outputs
settings_id = settings.id
settings_settings = settings.settings
```

---


### Evidence_folders_by_assessment

EvidenceFoldersByAssessment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evidence_folders` | Vec<String> | <p> The list of evidence folders that the <code>GetEvidenceFoldersByAssessment</code> API
         returned. </p> |
| `next_token` | String | <p> The pagination token that's used to fetch the next set of results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access evidence_folders_by_assessment outputs
evidence_folders_by_assessment_id = evidence_folders_by_assessment.id
evidence_folders_by_assessment_evidence_folders = evidence_folders_by_assessment.evidence_folders
evidence_folders_by_assessment_next_token = evidence_folders_by_assessment.next_token
```

---


### Assessment_report_url

AssessmentReportUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pre_signed_url` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access assessment_report_url outputs
assessment_report_url_id = assessment_report_url.id
assessment_report_url_pre_signed_url = assessment_report_url.pre_signed_url
```

---


### Account_status

AccountStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p> The status of the Amazon Web Services account. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_status outputs
account_status_id = account_status.id
account_status_status = account_status.status
```

---


### Delegations

Delegations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delegations` | Vec<String> | <p> The list of delegations that the <code>GetDelegations</code> API returned. </p> |
| `next_token` | String | <p> The pagination token that's used to fetch the next set of results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access delegations outputs
delegations_id = delegations.id
delegations_delegations = delegations.delegations
delegations_next_token = delegations.next_token
```

---


### Evidence_folders_by_assessment_control

EvidenceFoldersByAssessmentControl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p> The pagination token that's used to fetch the next set of results. </p> |
| `evidence_folders` | Vec<String> | <p> The list of evidence folders that the
            <code>GetEvidenceFoldersByAssessmentControl</code> API returned. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access evidence_folders_by_assessment_control outputs
evidence_folders_by_assessment_control_id = evidence_folders_by_assessment_control.id
evidence_folders_by_assessment_control_next_token = evidence_folders_by_assessment_control.next_token
evidence_folders_by_assessment_control_evidence_folders = evidence_folders_by_assessment_control.evidence_folders
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple assessment_control resources
assessment_control_0 = provider.auditmanager.Assessment_control {
    control_set_id = "value-0"
    assessment_id = "value-0"
    control_id = "value-0"
}
assessment_control_1 = provider.auditmanager.Assessment_control {
    control_set_id = "value-1"
    assessment_id = "value-1"
    control_id = "value-1"
}
assessment_control_2 = provider.auditmanager.Assessment_control {
    control_set_id = "value-2"
    assessment_id = "value-2"
    control_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    assessment_control = provider.auditmanager.Assessment_control {
        control_set_id = "production-value"
        assessment_id = "production-value"
        control_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Auditmanager Documentation](https://docs.aws.amazon.com/auditmanager/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
