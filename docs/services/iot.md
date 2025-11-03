# Iot Service



**Resources**: 64

---

## Overview

The iot service provides access to 64 resource types:

- [Verification_state_on_violation](#verification_state_on_violation) [C]
- [Ota_update](#ota_update) [CRD]
- [Security_profile](#security_profile) [CRUD]
- [Behavior_model_training_summaries](#behavior_model_training_summaries) [R]
- [Endpoint](#endpoint) [R]
- [Default_authorizer](#default_authorizer) [R]
- [Detect_mitigation_actions_task](#detect_mitigation_actions_task) [R]
- [Effective_policies](#effective_policies) [R]
- [Mitigation_action](#mitigation_action) [CRUD]
- [Audit_task](#audit_task) [R]
- [Audit_finding](#audit_finding) [R]
- [Billing_group](#billing_group) [CRUD]
- [Policy_version](#policy_version) [CRD]
- [Package](#package) [CRUD]
- [Policy](#policy) [CRD]
- [Thing_group](#thing_group) [CRUD]
- [Topic_rule](#topic_rule) [CRD]
- [Job_template](#job_template) [CRD]
- [Stream](#stream) [CRUD]
- [Statistics](#statistics) [R]
- [Dynamic_thing_group](#dynamic_thing_group) [CUD]
- [Thing_groups_for_thing](#thing_groups_for_thing) [U]
- [Job_execution](#job_execution) [RD]
- [Keys_and_certificate](#keys_and_certificate) [C]
- [Cardinality](#cardinality) [R]
- [Indexing_configuration](#indexing_configuration) [RU]
- [V2_logging_level](#v2_logging_level) [D]
- [Audit_suppression](#audit_suppression) [CRUD]
- [Role_alias](#role_alias) [CRUD]
- [Certificate](#certificate) [RUD]
- [Dimension](#dimension) [CRUD]
- [V2_logging_options](#v2_logging_options) [R]
- [Certificate_from_csr](#certificate_from_csr) [C]
- [Provisioning_template](#provisioning_template) [CRUD]
- [Command_execution](#command_execution) [RD]
- [Index](#index) [R]
- [Registration_code](#registration_code) [RD]
- [Thing_registration_task](#thing_registration_task) [R]
- [Logging_options](#logging_options) [R]
- [Command](#command) [CRUD]
- [Event_configurations](#event_configurations) [RU]
- [Fleet_metric](#fleet_metric) [CRUD]
- [Topic_rule_destination](#topic_rule_destination) [CRUD]
- [Thing_connectivity_data](#thing_connectivity_data) [R]
- [Thing](#thing) [CRUD]
- [Encryption_configuration](#encryption_configuration) [RU]
- [Ca_certificate](#ca_certificate) [RUD]
- [Job_document](#job_document) [R]
- [Provisioning_claim](#provisioning_claim) [C]
- [Thing_type](#thing_type) [CRUD]
- [Scheduled_audit](#scheduled_audit) [CRUD]
- [Job](#job) [CRUD]
- [Package_configuration](#package_configuration) [RU]
- [Provisioning_template_version](#provisioning_template_version) [CRD]
- [Authorizer](#authorizer) [CRUD]
- [Certificate_provider](#certificate_provider) [CRUD]
- [Percentiles](#percentiles) [R]
- [Package_version](#package_version) [CRUD]
- [Buckets_aggregation](#buckets_aggregation) [R]
- [Audit_mitigation_actions_task](#audit_mitigation_actions_task) [R]
- [Custom_metric](#custom_metric) [CRUD]
- [Managed_job_template](#managed_job_template) [R]
- [Domain_configuration](#domain_configuration) [CRUD]
- [Account_audit_configuration](#account_audit_configuration) [RUD]

---

## Resources


### Verification_state_on_violation

VerificationStateOnViolation resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `verification_state` | String | ✅ | <p>The verification state of the violation.</p> |
| `violation_id` | String | ✅ | <p>The violation ID.</p> |
| `verification_state_description` | String |  | <p>The description of the verification state of the violation (detect alarm).</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create verification_state_on_violation
verification_state_on_violation = provider.iot.Verification_state_on_violation {
    verification_state = "value"  # <p>The verification state of the violation.</p>
    violation_id = "value"  # <p>The violation ID.</p>
}

```

---


### Ota_update

OTAUpdate resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_job_timeout_config` | String |  | <p>Specifies the amount of time each device has to finish its execution of the job.  A timer is 
            started when the job execution status is set to <code>IN_PROGRESS</code>. If the job execution 
            status is not set to another terminal state before the timer expires, it will be automatically 
            set to <code>TIMED_OUT</code>.</p> |
| `aws_job_presigned_url_config` | String |  | <p>Configuration information for pre-signed URLs.</p> |
| `target_selection` | String |  | <p>Specifies whether the update will continue to run (CONTINUOUS), or will be complete after all the things
            specified as targets have completed the update (SNAPSHOT). If continuous, the update may also be run on a
            thing when a change is detected in a target. For example, an update will run on a thing when the thing is
            added to a target group, even after the update was completed by all things originally in the group. Valid
            values: CONTINUOUS | SNAPSHOT.</p> |
| `ota_update_id` | String | ✅ | <p>The ID of the OTA update to be created.</p> |
| `targets` | Vec<String> | ✅ | <p>The devices targeted to receive OTA updates.</p> |
| `role_arn` | String | ✅ | <p>The IAM role that grants Amazon Web Services IoT Core access to the Amazon S3, IoT jobs and Amazon Web Services Code Signing resources 
            to create an OTA update job.</p> |
| `aws_job_abort_config` | String |  | <p>The criteria that determine when and how a job abort takes place.</p> |
| `description` | String |  | <p>The description of the OTA update.</p> |
| `protocols` | Vec<String> |  | <p>The protocol used to transfer the OTA update image. Valid values are [HTTP], [MQTT], [HTTP, MQTT]. When both
           HTTP and MQTT are specified, the target device can choose the protocol.</p> |
| `aws_job_executions_rollout_config` | String |  | <p>Configuration for the rollout of OTA updates.</p> |
| `files` | Vec<String> | ✅ | <p>The files to be streamed by the OTA update.</p> |
| `additional_parameters` | HashMap<String, String> |  | <p>A list of additional OTA update parameters, which are name-value pairs. 
            They won't be sent to devices as a part of the Job document.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage updates.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ota_update_info` | String | <p>The OTA update info.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ota_update
ota_update = provider.iot.Ota_update {
    ota_update_id = "value"  # <p>The ID of the OTA update to be created.</p>
    targets = "value"  # <p>The devices targeted to receive OTA updates.</p>
    role_arn = "value"  # <p>The IAM role that grants Amazon Web Services IoT Core access to the Amazon S3, IoT jobs and Amazon Web Services Code Signing resources 
            to create an OTA update job.</p>
    files = "value"  # <p>The files to be streamed by the OTA update.</p>
}

# Access ota_update outputs
ota_update_id = ota_update.id
ota_update_ota_update_info = ota_update.ota_update_info
```

---


### Security_profile

SecurityProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_profile_description` | String |  | <p>A description of the security profile.</p> |
| `tags` | Vec<String> |  | <p>Metadata that can be used to manage the security profile.</p> |
| `additional_metrics_to_retain_v2` | Vec<String> |  | <p>A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's <code>behaviors</code>, but it is also retained for any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p> |
| `security_profile_name` | String | ✅ | <p>The name you are giving to the security profile.</p> |
| `alert_targets` | HashMap<String, String> |  | <p>Specifies the destinations to which alerts are sent. (Alerts are always sent to the 
        console.) Alerts are generated when a device (thing) violates a behavior.</p> |
| `metrics_export_config` | String |  | <p>Specifies the MQTT topic and role ARN required for metric export.</p> |
| `behaviors` | Vec<String> |  | <p>Specifies the behaviors that, when violated by a device (thing), cause an alert.</p> |
| `additional_metrics_to_retain` | Vec<String> |  | <p>
            <i>Please use <a>CreateSecurityProfileRequest$additionalMetricsToRetainV2</a> instead.</i>
         </p>
         <p>A list of metrics whose data is retained (stored). By default, data is retained 
        for any metric used in the profile's <code>behaviors</code>, but it is also retained for 
        any metric specified here. Can be used with custom metrics; cannot be used with dimensions.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The time the security profile was created.</p> |
| `behaviors` | Vec<String> | <p>Specifies the behaviors that, when violated by a device (thing), cause an alert.</p> |
| `additional_metrics_to_retain` | Vec<String> | <p>
            <i>Please use
          <a>DescribeSecurityProfileResponse$additionalMetricsToRetainV2</a>
        instead.</i>
         </p>
         <p>A list of metrics
      whose data is retained (stored). By default, data is retained for any metric
      used in the profile's <code>behaviors</code>, but
      it is
      also retained for any metric specified here.</p> |
| `security_profile_name` | String | <p>The name of the security profile.</p> |
| `additional_metrics_to_retain_v2` | Vec<String> | <p>A list of metrics whose data is retained (stored). By default, data is retained for any
      metric used in the profile's behaviors, but
      it is
      also retained for any metric specified here.</p> |
| `version` | i64 | <p>The version of the security profile. A new version is generated whenever the
        security profile is updated.</p> |
| `alert_targets` | HashMap<String, String> | <p>Where the alerts are sent. (Alerts are always sent to the console.)</p> |
| `security_profile_arn` | String | <p>The ARN of the security profile.</p> |
| `last_modified_date` | String | <p>The time the security profile was last modified.</p> |
| `metrics_export_config` | String | <p>Specifies the MQTT topic and role ARN required for metric export.</p> |
| `security_profile_description` | String | <p>A description of the security profile (associated with the security profile
        when it was created or updated).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create security_profile
security_profile = provider.iot.Security_profile {
    security_profile_name = "value"  # <p>The name you are giving to the security profile.</p>
}

# Access security_profile outputs
security_profile_id = security_profile.id
security_profile_creation_date = security_profile.creation_date
security_profile_behaviors = security_profile.behaviors
security_profile_additional_metrics_to_retain = security_profile.additional_metrics_to_retain
security_profile_security_profile_name = security_profile.security_profile_name
security_profile_additional_metrics_to_retain_v2 = security_profile.additional_metrics_to_retain_v2
security_profile_version = security_profile.version
security_profile_alert_targets = security_profile.alert_targets
security_profile_security_profile_arn = security_profile.security_profile_arn
security_profile_last_modified_date = security_profile.last_modified_date
security_profile_metrics_export_config = security_profile.metrics_export_config
security_profile_security_profile_description = security_profile.security_profile_description
```

---


### Behavior_model_training_summaries

BehaviorModelTrainingSummaries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summaries` | Vec<String> | <p>
      A list of all ML Detect behaviors and their model status for a given Security Profile.
    </p> |
| `next_token` | String | <p>
      A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access behavior_model_training_summaries outputs
behavior_model_training_summaries_id = behavior_model_training_summaries.id
behavior_model_training_summaries_summaries = behavior_model_training_summaries.summaries
behavior_model_training_summaries_next_token = behavior_model_training_summaries.next_token
```

---


### Endpoint

Endpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_address` | String | <p>The endpoint. The format of the endpoint is as follows:
            <i>identifier</i>.iot.<i>region</i>.amazonaws.com.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_endpoint_address = endpoint.endpoint_address
```

---


### Default_authorizer

DefaultAuthorizer resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorizer_description` | String | <p>The default authorizer's description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_authorizer outputs
default_authorizer_id = default_authorizer.id
default_authorizer_authorizer_description = default_authorizer.authorizer_description
```

---


### Detect_mitigation_actions_task

DetectMitigationActionsTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_summary` | String | <p>
      The description of a task.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access detect_mitigation_actions_task outputs
detect_mitigation_actions_task_id = detect_mitigation_actions_task.id
detect_mitigation_actions_task_task_summary = detect_mitigation_actions_task.task_summary
```

---


### Effective_policies

EffectivePolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effective_policies` | Vec<String> | <p>The effective policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access effective_policies outputs
effective_policies_id = effective_policies.id
effective_policies_effective_policies = effective_policies.effective_policies
```

---


### Mitigation_action

MitigationAction resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_params` | String | ✅ | <p>Defines the type of action and the parameters for that action.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the IAM role that is used to apply the mitigation action.</p> |
| `tags` | Vec<String> |  | <p>Metadata that can be used to manage the mitigation action.</p> |
| `action_name` | String | ✅ | <p>A friendly name for the action. Choose a friendly name that accurately describes the action (for example, <code>EnableLoggingAction</code>).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_date` | String | <p>The date and time when the mitigation action was last changed.</p> |
| `action_arn` | String | <p>The ARN that identifies this migration action.</p> |
| `role_arn` | String | <p>The ARN of the IAM role used to apply this action.</p> |
| `action_id` | String | <p>A unique identifier for this action.</p> |
| `action_params` | String | <p>Parameters that control how the mitigation action is applied, specific to the type of mitigation action.</p> |
| `action_type` | String | <p>The type of mitigation action.</p> |
| `creation_date` | String | <p>The date and time when the mitigation action was added to your Amazon Web Services accounts.</p> |
| `action_name` | String | <p>The friendly name that uniquely identifies the mitigation action.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create mitigation_action
mitigation_action = provider.iot.Mitigation_action {
    action_params = "value"  # <p>Defines the type of action and the parameters for that action.</p>
    role_arn = "value"  # <p>The ARN of the IAM role that is used to apply the mitigation action.</p>
    action_name = "value"  # <p>A friendly name for the action. Choose a friendly name that accurately describes the action (for example, <code>EnableLoggingAction</code>).</p>
}

# Access mitigation_action outputs
mitigation_action_id = mitigation_action.id
mitigation_action_last_modified_date = mitigation_action.last_modified_date
mitigation_action_action_arn = mitigation_action.action_arn
mitigation_action_role_arn = mitigation_action.role_arn
mitigation_action_action_id = mitigation_action.action_id
mitigation_action_action_params = mitigation_action.action_params
mitigation_action_action_type = mitigation_action.action_type
mitigation_action_creation_date = mitigation_action.creation_date
mitigation_action_action_name = mitigation_action.action_name
```

---


### Audit_task

AuditTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_status` | String | <p>The status of the audit: one of "IN_PROGRESS", "COMPLETED",
            "FAILED", or "CANCELED".</p> |
| `task_type` | String | <p>The type of audit: "ON_DEMAND_AUDIT_TASK" or "SCHEDULED_AUDIT_TASK".</p> |
| `task_start_time` | String | <p>The time the audit started.</p> |
| `task_statistics` | String | <p>Statistical information about the audit.</p> |
| `scheduled_audit_name` | String | <p>The name of the scheduled audit (only if the audit was a scheduled audit).</p> |
| `audit_details` | HashMap<String, String> | <p>Detailed information about each check performed during this audit.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access audit_task outputs
audit_task_id = audit_task.id
audit_task_task_status = audit_task.task_status
audit_task_task_type = audit_task.task_type
audit_task_task_start_time = audit_task.task_start_time
audit_task_task_statistics = audit_task.task_statistics
audit_task_scheduled_audit_name = audit_task.scheduled_audit_name
audit_task_audit_details = audit_task.audit_details
```

---


### Audit_finding

AuditFinding resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `finding` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access audit_finding outputs
audit_finding_id = audit_finding.id
audit_finding_finding = audit_finding.finding
```

---


### Billing_group

BillingGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `billing_group_properties` | String |  | <p>The properties of the billing group.</p> |
| `billing_group_name` | String | ✅ | <p>The name you wish to give to the billing group.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the billing group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `billing_group_metadata` | String | <p>Additional information about the billing group.</p> |
| `version` | i64 | <p>The version of the billing group.</p> |
| `billing_group_name` | String | <p>The name of the billing group.</p> |
| `billing_group_id` | String | <p>The ID of the billing group.</p> |
| `billing_group_arn` | String | <p>The ARN of the billing group.</p> |
| `billing_group_properties` | String | <p>The properties of the billing group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create billing_group
billing_group = provider.iot.Billing_group {
    billing_group_name = "value"  # <p>The name you wish to give to the billing group.</p>
}

# Access billing_group outputs
billing_group_id = billing_group.id
billing_group_billing_group_metadata = billing_group.billing_group_metadata
billing_group_version = billing_group.version
billing_group_billing_group_name = billing_group.billing_group_name
billing_group_billing_group_id = billing_group.billing_group_id
billing_group_billing_group_arn = billing_group.billing_group_arn
billing_group_billing_group_properties = billing_group.billing_group_properties
```

---


### Policy_version

PolicyVersion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_document` | String | ✅ | <p>The JSON document that describes the policy. Minimum length of 1. Maximum length of
         2048, excluding whitespace.</p> |
| `policy_name` | String | ✅ | <p>The policy name.</p> |
| `set_as_default` | bool |  | <p>Specifies whether the policy version is set as the default. When this parameter is
         true, the new policy version becomes the operative version (that is, the version that is in
         effect for the certificates to which the policy is attached).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_name` | String | <p>The policy name.</p> |
| `creation_date` | String | <p>The date the policy was created.</p> |
| `last_modified_date` | String | <p>The date the policy was last modified.</p> |
| `policy_arn` | String | <p>The policy ARN.</p> |
| `policy_document` | String | <p>The JSON document that describes the policy.</p> |
| `is_default_version` | bool | <p>Specifies whether the policy version is the default.</p> |
| `generation_id` | String | <p>The generation ID of the policy version.</p> |
| `policy_version_id` | String | <p>The policy version ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy_version
policy_version = provider.iot.Policy_version {
    policy_document = "value"  # <p>The JSON document that describes the policy. Minimum length of 1. Maximum length of
         2048, excluding whitespace.</p>
    policy_name = "value"  # <p>The policy name.</p>
}

# Access policy_version outputs
policy_version_id = policy_version.id
policy_version_policy_name = policy_version.policy_name
policy_version_creation_date = policy_version.creation_date
policy_version_last_modified_date = policy_version.last_modified_date
policy_version_policy_arn = policy_version.policy_arn
policy_version_policy_document = policy_version.policy_document
policy_version_is_default_version = policy_version.is_default_version
policy_version_generation_id = policy_version.generation_id
policy_version_policy_version_id = policy_version.policy_version_id
```

---


### Package

Package resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A summary of the package being created. This can be used to outline the package's contents or purpose.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that can be used to manage the package.</p> |
| `package_name` | String | ✅ | <p>The name of the new software package.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. 
      Don't reuse this client token if a new idempotent request is required.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The date the package was created.</p> |
| `package_name` | String | <p>The name of the software package.</p> |
| `description` | String | <p>The package description.</p> |
| `default_version_name` | String | <p>The name of the default package version.</p> |
| `package_arn` | String | <p>The ARN for the package.</p> |
| `last_modified_date` | String | <p>The date when the package was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package
package = provider.iot.Package {
    package_name = "value"  # <p>The name of the new software package.</p>
}

# Access package outputs
package_id = package.id
package_creation_date = package.creation_date
package_package_name = package.package_name
package_description = package.description
package_default_version_name = package.default_version_name
package_package_arn = package.package_arn
package_last_modified_date = package.last_modified_date
```

---


### Policy

Policy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_name` | String | ✅ | <p>The policy name.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the policy.</p>
         <note>
            <p>For URI Request parameters use format: ...key1=value1&key2=value2...</p>
            <p>For the CLI command-line parameter use format: &&tags
            "key1=value1&key2=value2..."</p>
            <p>For the cli-input-json file use format: "tags":
            "key1=value1&key2=value2..."</p>
         </note> |
| `policy_document` | String | ✅ | <p>The JSON document that describes the policy. <b>policyDocument</b> must have a minimum length of 1, with a maximum length of
         2048, excluding whitespace.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generation_id` | String | <p>The generation ID of the policy.</p> |
| `last_modified_date` | String | <p>The date the policy was last modified.</p> |
| `policy_name` | String | <p>The policy name.</p> |
| `policy_arn` | String | <p>The policy ARN.</p> |
| `policy_document` | String | <p>The JSON document that describes the policy.</p> |
| `creation_date` | String | <p>The date the policy was created.</p> |
| `default_version_id` | String | <p>The default policy version ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy
policy = provider.iot.Policy {
    policy_name = "value"  # <p>The policy name.</p>
    policy_document = "value"  # <p>The JSON document that describes the policy. <b>policyDocument</b> must have a minimum length of 1, with a maximum length of
         2048, excluding whitespace.</p>
}

# Access policy outputs
policy_id = policy.id
policy_generation_id = policy.generation_id
policy_last_modified_date = policy.last_modified_date
policy_policy_name = policy.policy_name
policy_policy_arn = policy.policy_arn
policy_policy_document = policy.policy_document
policy_creation_date = policy.creation_date
policy_default_version_id = policy.default_version_id
```

---


### Thing_group

ThingGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_group_name` | String |  | <p>The name of the parent thing group.</p> |
| `thing_group_name` | String | ✅ | <p>The thing group name to create.</p> |
| `thing_group_properties` | String |  | <p>The thing group properties.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the thing group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `thing_group_arn` | String | <p>The thing group ARN.</p> |
| `thing_group_properties` | String | <p>The thing group properties.</p> |
| `query_version` | String | <p>The dynamic thing group query version.</p> |
| `thing_group_metadata` | String | <p>Thing group metadata.</p> |
| `thing_group_name` | String | <p>The name of the thing group.</p> |
| `index_name` | String | <p>The dynamic thing group index name.</p> |
| `status` | String | <p>The dynamic thing group status.</p> |
| `thing_group_id` | String | <p>The thing group ID.</p> |
| `query_string` | String | <p>The dynamic thing group search query string.</p> |
| `version` | i64 | <p>The version of the thing group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create thing_group
thing_group = provider.iot.Thing_group {
    thing_group_name = "value"  # <p>The thing group name to create.</p>
}

# Access thing_group outputs
thing_group_id = thing_group.id
thing_group_thing_group_arn = thing_group.thing_group_arn
thing_group_thing_group_properties = thing_group.thing_group_properties
thing_group_query_version = thing_group.query_version
thing_group_thing_group_metadata = thing_group.thing_group_metadata
thing_group_thing_group_name = thing_group.thing_group_name
thing_group_index_name = thing_group.index_name
thing_group_status = thing_group.status
thing_group_thing_group_id = thing_group.thing_group_id
thing_group_query_string = thing_group.query_string
thing_group_version = thing_group.version
```

---


### Topic_rule

TopicRule resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | String |  | <p>Metadata which can be used to manage the topic rule.</p>
         <note>
            <p>For URI Request parameters use format: ...key1=value1&key2=value2...</p>
            <p>For the CLI command-line parameter use format: --tags
            "key1=value1&key2=value2..."</p>
            <p>For the cli-input-json file use format: "tags":
            "key1=value1&key2=value2..."</p>
         </note> |
| `rule_name` | String | ✅ | <p>The name of the rule.</p> |
| `topic_rule_payload` | String | ✅ | <p>The rule payload.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule` | String | <p>The rule.</p> |
| `rule_arn` | String | <p>The rule ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create topic_rule
topic_rule = provider.iot.Topic_rule {
    rule_name = "value"  # <p>The name of the rule.</p>
    topic_rule_payload = "value"  # <p>The rule payload.</p>
}

# Access topic_rule outputs
topic_rule_id = topic_rule.id
topic_rule_rule = topic_rule.rule
topic_rule_rule_arn = topic_rule.rule_arn
```

---


### Job_template

JobTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_package_versions` | Vec<String> |  | <p>The package version Amazon Resource Names (ARNs) that are installed on the device when the job
            successfully completes. The package version must be in either the Published or
            Deprecated state when the job deploys. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/preparing-to-use-software-package-catalog.html#package-version-lifecycle">Package version lifecycle</a>.</p>
         <p>
            <b>Note:</b>The following Length Constraints relates to a
            single ARN. Up to 25 package version ARNs are allowed.</p> |
| `description` | String | ✅ | <p>A description of the job document.</p> |
| `presigned_url_config` | String |  |  |
| `document` | String |  | <p>The job document. Required if you don't specify a value for
                <code>documentSource</code>.</p> |
| `abort_config` | String |  |  |
| `job_arn` | String |  | <p>The ARN of the job to use as the basis for the job template.</p> |
| `job_executions_retry_config` | String |  | <p>Allows you to create the criteria to retry a job.</p> |
| `job_template_id` | String | ✅ | <p>A unique identifier for the job template. We recommend using a UUID. Alpha-numeric
            characters, "-", and "_" are valid for use here.</p> |
| `maintenance_windows` | Vec<String> |  | <p>Allows you to configure an optional maintenance window for the rollout of a job
            document to all devices in the target group for a job.</p> |
| `job_executions_rollout_config` | String |  |  |
| `timeout_config` | String |  |  |
| `tags` | Vec<String> |  | <p>Metadata that can be used to manage the job template.</p> |
| `document_source` | String |  | <p>An S3 link, or S3 object URL, to the job document. The link is an Amazon S3 object URL
            and is required if you don't specify a value for <code>document</code>.</p>
         <p>For example, <code>--document-source
                https://s3.<i>region-code</i>.amazonaws.com/example-firmware/device-firmware.1.0</code>
         </p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-bucket-intro.html">Methods for accessing a
                bucket</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination_package_versions` | Vec<String> | <p>The package version Amazon Resource Names (ARNs) that are installed on the device when the job
            successfully completes. The package version must be in either the Published or
            Deprecated state when the job deploys. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/preparing-to-use-software-package-catalog.html#package-version-lifecycle">Package version lifecycle</a>.</p>
         <p>
            <b>Note:</b>The following Length Constraints relates to a
            single ARN. Up to 25 package version ARNs are allowed.</p> |
| `document` | String | <p>The job document.</p> |
| `timeout_config` | String |  |
| `job_template_id` | String | <p>The unique identifier of the job template.</p> |
| `created_at` | String | <p>The time, in seconds since the epoch, when the job template was created.</p> |
| `presigned_url_config` | String |  |
| `abort_config` | String |  |
| `job_template_arn` | String | <p>The ARN of the job template.</p> |
| `job_executions_retry_config` | String | <p>The configuration that determines how many retries are allowed for each failure type
            for a job.</p> |
| `maintenance_windows` | Vec<String> | <p>Allows you to configure an optional maintenance window for the rollout of a job
            document to all devices in the target group for a job.</p> |
| `description` | String | <p>A description of the job template.</p> |
| `job_executions_rollout_config` | String |  |
| `document_source` | String | <p>An S3 link to the job document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job_template
job_template = provider.iot.Job_template {
    description = "value"  # <p>A description of the job document.</p>
    job_template_id = "value"  # <p>A unique identifier for the job template. We recommend using a UUID. Alpha-numeric
            characters, "-", and "_" are valid for use here.</p>
}

# Access job_template outputs
job_template_id = job_template.id
job_template_destination_package_versions = job_template.destination_package_versions
job_template_document = job_template.document
job_template_timeout_config = job_template.timeout_config
job_template_job_template_id = job_template.job_template_id
job_template_created_at = job_template.created_at
job_template_presigned_url_config = job_template.presigned_url_config
job_template_abort_config = job_template.abort_config
job_template_job_template_arn = job_template.job_template_arn
job_template_job_executions_retry_config = job_template.job_executions_retry_config
job_template_maintenance_windows = job_template.maintenance_windows
job_template_description = job_template.description
job_template_job_executions_rollout_config = job_template.job_executions_rollout_config
job_template_document_source = job_template.document_source
```

---


### Stream

Stream resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage streams.</p> |
| `stream_id` | String | ✅ | <p>The stream ID.</p> |
| `role_arn` | String | ✅ | <p>An IAM role that allows the IoT service principal to access your S3 files.</p> |
| `files` | Vec<String> | ✅ | <p>The files to stream.</p> |
| `description` | String |  | <p>A description of the stream.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stream_info` | String | <p>Information about the stream.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stream
stream = provider.iot.Stream {
    stream_id = "value"  # <p>The stream ID.</p>
    role_arn = "value"  # <p>An IAM role that allows the IoT service principal to access your S3 files.</p>
    files = "value"  # <p>The files to stream.</p>
}

# Access stream outputs
stream_id = stream.id
stream_stream_info = stream.stream_info
```

---


### Statistics

Statistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statistics` | String | <p>The statistics returned by the Fleet Indexing service based on the query and aggregation
      field.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access statistics outputs
statistics_id = statistics.id
statistics_statistics = statistics.statistics
```

---


### Dynamic_thing_group

DynamicThingGroup resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_version` | String |  | <p>The dynamic thing group query version.</p>
         <note>
            <p>Currently one query version is supported: "2017-09-30". If not specified, the
				query version defaults to this value.</p>
         </note> |
| `query_string` | String | ✅ | <p>The dynamic thing group search query string.</p>
         <p>See <a href="https://docs.aws.amazon.com/iot/latest/developerguide/query-syntax.html">Query Syntax</a> for information about query string syntax.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the dynamic thing group.</p> |
| `thing_group_name` | String | ✅ | <p>The dynamic thing group name to create.</p> |
| `thing_group_properties` | String |  | <p>The dynamic thing group properties.</p> |
| `index_name` | String |  | <p>The dynamic thing group index name.</p>
         <note>
            <p>Currently one index is supported: <code>AWS_Things</code>.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dynamic_thing_group
dynamic_thing_group = provider.iot.Dynamic_thing_group {
    query_string = "value"  # <p>The dynamic thing group search query string.</p>
         <p>See <a href="https://docs.aws.amazon.com/iot/latest/developerguide/query-syntax.html">Query Syntax</a> for information about query string syntax.</p>
    thing_group_name = "value"  # <p>The dynamic thing group name to create.</p>
}

```

---


### Thing_groups_for_thing

ThingGroupsForThing resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thing_groups_to_add` | Vec<String> |  | <p>The groups to which the thing will be added.</p> |
| `override_dynamic_groups` | bool |  | <p>Override dynamic thing groups with static thing groups when 10-group limit is
			reached. If a thing belongs to 10 thing groups, and one or more of those groups are
			dynamic thing groups, adding a thing to a static group removes the thing from the last
			dynamic group.</p> |
| `thing_groups_to_remove` | Vec<String> |  | <p>The groups from which the thing will be removed.</p> |
| `thing_name` | String |  | <p>The thing whose group memberships will be updated.</p> |



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


### Job_execution

JobExecution resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution` | String | <p>Information about the job execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_execution outputs
job_execution_id = job_execution.id
job_execution_execution = job_execution.execution
```

---


### Keys_and_certificate

KeysAndCertificate resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `set_as_active` | bool |  | <p>Specifies whether the certificate is active.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create keys_and_certificate
keys_and_certificate = provider.iot.Keys_and_certificate {
}

```

---


### Cardinality

Cardinality resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cardinality` | i64 | <p>The approximate count of unique values that match the query.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cardinality outputs
cardinality_id = cardinality.id
cardinality_cardinality = cardinality.cardinality
```

---


### Indexing_configuration

IndexingConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thing_indexing_configuration` | String |  | <p>Thing indexing configuration.</p> |
| `thing_group_indexing_configuration` | String |  | <p>Thing group indexing configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `thing_group_indexing_configuration` | String | <p>The index configuration.</p> |
| `thing_indexing_configuration` | String | <p>Thing indexing configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access indexing_configuration outputs
indexing_configuration_id = indexing_configuration.id
indexing_configuration_thing_group_indexing_configuration = indexing_configuration.thing_group_indexing_configuration
indexing_configuration_thing_indexing_configuration = indexing_configuration.thing_indexing_configuration
```

---


### V2_logging_level

V2LoggingLevel resource

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


### Audit_suppression

AuditSuppression resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `check_name` | String | ✅ |  |
| `client_request_token` | String | ✅ | <p>
      Each audit supression must have a unique client request token. If you try to create a new audit
      suppression with the same token as one that already exists, an exception occurs. If you omit this
      value, Amazon Web Services SDKs will automatically generate a unique client request.</p> |
| `resource_identifier` | String | ✅ |  |
| `expiration_date` | String |  | <p>
      The epoch timestamp in seconds at which this suppression expires.
    </p> |
| `suppress_indefinitely` | bool |  | <p>
      Indicates whether a suppression should exist indefinitely or not.
    </p> |
| `description` | String |  | <p>
      The description of the audit suppression.
    </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `check_name` | String |  |
| `suppress_indefinitely` | bool | <p>
      Indicates whether a suppression should exist indefinitely or not.
    </p> |
| `description` | String | <p>
      The description of the audit suppression.
    </p> |
| `resource_identifier` | String |  |
| `expiration_date` | String | <p>
      The epoch timestamp in seconds at which this suppression expires.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create audit_suppression
audit_suppression = provider.iot.Audit_suppression {
    check_name = "value"  # Required field
    client_request_token = "value"  # <p>
      Each audit supression must have a unique client request token. If you try to create a new audit
      suppression with the same token as one that already exists, an exception occurs. If you omit this
      value, Amazon Web Services SDKs will automatically generate a unique client request.</p>
    resource_identifier = "value"  # Required field
}

# Access audit_suppression outputs
audit_suppression_id = audit_suppression.id
audit_suppression_check_name = audit_suppression.check_name
audit_suppression_suppress_indefinitely = audit_suppression.suppress_indefinitely
audit_suppression_description = audit_suppression.description
audit_suppression_resource_identifier = audit_suppression.resource_identifier
audit_suppression_expiration_date = audit_suppression.expiration_date
```

---


### Role_alias

RoleAlias resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_alias` | String | ✅ | <p>The role alias that points to a role ARN. This allows you to change the role without
         having to update the device.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the role alias.</p>
         <note>
            <p>For URI Request parameters use format: ...key1=value1&key2=value2...</p>
            <p>For the CLI command-line parameter use format: &&tags
            "key1=value1&key2=value2..."</p>
            <p>For the cli-input-json file use format: "tags":
            "key1=value1&key2=value2..."</p>
         </note> |
| `credential_duration_seconds` | i64 |  | <p>How long (in seconds) the credentials will be valid. The default value is 3,600 seconds.</p>
         <p>This value must be less than or equal to the maximum session duration of the IAM role
      that the role alias references.</p> |
| `role_arn` | String | ✅ | <p>The role ARN.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_alias_description` | String | <p>The role alias description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create role_alias
role_alias = provider.iot.Role_alias {
    role_alias = "value"  # <p>The role alias that points to a role ARN. This allows you to change the role without
         having to update the device.</p>
    role_arn = "value"  # <p>The role ARN.</p>
}

# Access role_alias outputs
role_alias_id = role_alias.id
role_alias_role_alias_description = role_alias.role_alias_description
```

---


### Certificate

Certificate resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_id` | String | ✅ | <p>The ID of the certificate. (The last part of the certificate ARN contains the
         certificate ID.)</p> |
| `new_status` | String | ✅ | <p>The new status.</p>
         <p>
            <b>Note:</b> Setting the status to PENDING_TRANSFER  or PENDING_ACTIVATION will result
         in an exception being thrown. PENDING_TRANSFER and PENDING_ACTIVATION are statuses used internally by IoT. They 
         are not intended for developer use.</p>
         <p>
            <b>Note:</b> The status value REGISTER_INACTIVE is deprecated and
         should not be used.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate_description` | String | <p>The description of the certificate.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificate outputs
certificate_id = certificate.id
certificate_certificate_description = certificate.certificate_description
```

---


### Dimension

Dimension resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>Specifies the type of dimension. Supported types: <code>TOPIC_FILTER.</code>
         </p> |
| `string_values` | Vec<String> | ✅ | <p>Specifies the value or list of values for the dimension. For <code>TOPIC_FILTER</code> dimensions, this is a pattern used to match the MQTT topic (for example, "admin/#").</p> |
| `tags` | Vec<String> |  | <p>Metadata that can be used to manage the dimension.</p> |
| `name` | String | ✅ | <p>A unique identifier for the dimension. Choose something that describes the type and value to make it easy to remember what it does.</p> |
| `client_request_token` | String | ✅ | <p>Each dimension must have a unique client request token. If you try to create a new dimension with the same token as a dimension that already exists, an exception occurs. 
      If you omit this value, Amazon Web Services SDKs will automatically generate a unique client request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The date the dimension was created.</p> |
| `arn` | String | <p>The Amazon Resource Name
      (ARN)
      for
      the dimension.</p> |
| `string_values` | Vec<String> | <p>The value or list of values used to scope the dimension. For example, for topic filters, this is the pattern used to match the MQTT topic name.</p> |
| `type` | String | <p>The type of the dimension.</p> |
| `last_modified_date` | String | <p>The date the dimension was last modified.</p> |
| `name` | String | <p>The unique identifier for the dimension.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dimension
dimension = provider.iot.Dimension {
    type = "value"  # <p>Specifies the type of dimension. Supported types: <code>TOPIC_FILTER.</code>
         </p>
    string_values = "value"  # <p>Specifies the value or list of values for the dimension. For <code>TOPIC_FILTER</code> dimensions, this is a pattern used to match the MQTT topic (for example, "admin/#").</p>
    name = "value"  # <p>A unique identifier for the dimension. Choose something that describes the type and value to make it easy to remember what it does.</p>
    client_request_token = "value"  # <p>Each dimension must have a unique client request token. If you try to create a new dimension with the same token as a dimension that already exists, an exception occurs. 
      If you omit this value, Amazon Web Services SDKs will automatically generate a unique client request.</p>
}

# Access dimension outputs
dimension_id = dimension.id
dimension_creation_date = dimension.creation_date
dimension_arn = dimension.arn
dimension_string_values = dimension.string_values
dimension_type = dimension.type
dimension_last_modified_date = dimension.last_modified_date
dimension_name = dimension.name
```

---


### V2_logging_options

V2LoggingOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_log_level` | String | <p>The default log level.</p> |
| `disable_all_logs` | bool | <p>Disables all logs.</p> |
| `role_arn` | String | <p>The IAM role ARN IoT uses to write to your CloudWatch logs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access v2_logging_options outputs
v2_logging_options_id = v2_logging_options.id
v2_logging_options_default_log_level = v2_logging_options.default_log_level
v2_logging_options_disable_all_logs = v2_logging_options.disable_all_logs
v2_logging_options_role_arn = v2_logging_options.role_arn
```

---


### Certificate_from_csr

CertificateFromCsr resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_signing_request` | String | ✅ | <p>The certificate signing request (CSR).</p> |
| `set_as_active` | bool |  | <p>Specifies whether the certificate is active.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create certificate_from_csr
certificate_from_csr = provider.iot.Certificate_from_csr {
    certificate_signing_request = "value"  # <p>The certificate signing request (CSR).</p>
}

```

---


### Provisioning_template

ProvisioningTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_name` | String | ✅ | <p>The name of the provisioning template.</p> |
| `description` | String |  | <p>The description of the provisioning template.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the provisioning template.</p>
         <note>
            <p>For URI Request parameters use format: ...key1=value1&key2=value2...</p>
            <p>For the CLI command-line parameter use format: &&tags
            "key1=value1&key2=value2..."</p>
            <p>For the cli-input-json file use format: "tags":
            "key1=value1&key2=value2..."</p>
         </note> |
| `template_body` | String | ✅ | <p>The JSON formatted contents of the provisioning template.</p> |
| `enabled` | bool |  | <p>True to enable the provisioning template, otherwise false.</p> |
| `pre_provisioning_hook` | String |  | <p>Creates a pre-provisioning hook template. Only supports template of type
            <code>FLEET_PROVISIONING</code>. For more information about provisioning template types,
         see <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_CreateProvisioningTemplate.html#iot-CreateProvisioningTemplate-request-type">type</a>.</p> |
| `provisioning_role_arn` | String | ✅ | <p>The role ARN for the role associated with the provisioning template. This IoT role
         grants permission to provision a device.</p> |
| `type` | String |  | <p>The type you define in a provisioning template. You can create a template with only one type.
         You can't change the template type after its creation. The default value is <code>FLEET_PROVISIONING</code>.
         For more information about provisioning template, see: <a href="https://docs.aws.amazon.com/iot/latest/developerguide/provision-template.html">Provisioning template</a>.
      </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the provisioning template.</p> |
| `provisioning_role_arn` | String | <p>The ARN of the role associated with the provisioning template. This IoT role grants
         permission to provision a device.</p> |
| `last_modified_date` | String | <p>The date when the provisioning template was last modified.</p> |
| `template_name` | String | <p>The name of the provisioning template.</p> |
| `pre_provisioning_hook` | String | <p>Gets information about a pre-provisioned hook.</p> |
| `creation_date` | String | <p>The date when the provisioning template was created.</p> |
| `template_body` | String | <p>The JSON formatted contents of the provisioning template.</p> |
| `default_version_id` | i64 | <p>The default fleet template version ID.</p> |
| `enabled` | bool | <p>True if the provisioning template is enabled, otherwise false.</p> |
| `type` | String | <p>The type you define in a provisioning template. You can create a template with only one type.
         You can't change the template type after its creation. The default value is <code>FLEET_PROVISIONING</code>.
         For more information about provisioning template, see: <a href="https://docs.aws.amazon.com/iot/latest/developerguide/provision-template.html">Provisioning template</a>.
      </p> |
| `template_arn` | String | <p>The ARN of the provisioning template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create provisioning_template
provisioning_template = provider.iot.Provisioning_template {
    template_name = "value"  # <p>The name of the provisioning template.</p>
    template_body = "value"  # <p>The JSON formatted contents of the provisioning template.</p>
    provisioning_role_arn = "value"  # <p>The role ARN for the role associated with the provisioning template. This IoT role
         grants permission to provision a device.</p>
}

# Access provisioning_template outputs
provisioning_template_id = provisioning_template.id
provisioning_template_description = provisioning_template.description
provisioning_template_provisioning_role_arn = provisioning_template.provisioning_role_arn
provisioning_template_last_modified_date = provisioning_template.last_modified_date
provisioning_template_template_name = provisioning_template.template_name
provisioning_template_pre_provisioning_hook = provisioning_template.pre_provisioning_hook
provisioning_template_creation_date = provisioning_template.creation_date
provisioning_template_template_body = provisioning_template.template_body
provisioning_template_default_version_id = provisioning_template.default_version_id
provisioning_template_enabled = provisioning_template.enabled
provisioning_template_type = provisioning_template.type
provisioning_template_template_arn = provisioning_template.template_arn
```

---


### Command_execution

CommandExecution resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `started_at` | String | <p>The timestamp, when the command execution was started.</p> |
| `execution_timeout_seconds` | i64 | <p>Specifies the amount of time in seconds that the device can take to finish a command
            execution. A timer starts when the command execution is created. If the command
            execution status is not set to another terminal state before the timer expires, it will
            automatically update to <code>TIMED_OUT</code>.</p> |
| `created_at` | String | <p>The timestamp, when the command execution was created.</p> |
| `time_to_live` | String | <p>The time to live (TTL) parameter that indicates the duration for which executions will
        be retained in your account. The default value is six months.</p> |
| `completed_at` | String | <p>The timestamp, when the command execution was completed.</p> |
| `command_arn` | String | <p>The Amazon Resource Number (ARN) of the command. For example,
            <code></code>arn:aws:iot:<region>:<accountid>:command/<commandId></p> |
| `status` | String | <p>The status of the command execution. After your devices receive the command and start
            performing the operations specified in the command, it can use the
                <code>UpdateCommandExecution</code> MQTT API to update the status
            information.</p> |
| `result` | HashMap<String, String> | <p>The result value for the current state of the command execution. The status provides
            information about the progress of the command execution. The device can use the result
            field to share additional details about the execution such as a return value of a remote
            function call.</p>
         <note>
            <p>If you use the <code>AWS-IoT-FleetWise</code> namespace, then this field is not
                applicable in the API response.</p>
         </note> |
| `last_updated_at` | String | <p>The timestamp, when the command execution was last updated.</p> |
| `target_arn` | String | <p>The Amazon Resource Number (ARN) of the device on which the command execution is being
            performed.</p> |
| `parameters` | HashMap<String, String> | <p>The list of parameters that the <code>StartCommandExecution</code> API used when
            performing the command on the device.</p> |
| `execution_id` | String | <p>The unique identifier of the command execution.</p> |
| `status_reason` | String | <p>Your devices can use this parameter to provide additional context about the status of
            a command execution using a reason code and description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access command_execution outputs
command_execution_id = command_execution.id
command_execution_started_at = command_execution.started_at
command_execution_execution_timeout_seconds = command_execution.execution_timeout_seconds
command_execution_created_at = command_execution.created_at
command_execution_time_to_live = command_execution.time_to_live
command_execution_completed_at = command_execution.completed_at
command_execution_command_arn = command_execution.command_arn
command_execution_status = command_execution.status
command_execution_result = command_execution.result
command_execution_last_updated_at = command_execution.last_updated_at
command_execution_target_arn = command_execution.target_arn
command_execution_parameters = command_execution.parameters
command_execution_execution_id = command_execution.execution_id
command_execution_status_reason = command_execution.status_reason
```

---


### Index

Index resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema` | String | <p>Contains a value that specifies the type of indexing performed. Valid values
      are:</p>
         <ul>
            <li>
               <p>REGISTRY – Your thing index contains only registry data.</p>
            </li>
            <li>
               <p>REGISTRY_AND_SHADOW - Your thing index contains registry data and shadow data.</p>
            </li>
            <li>
               <p>REGISTRY_AND_CONNECTIVITY_STATUS - Your thing index contains registry data and
          thing connectivity status data.</p>
            </li>
            <li>
               <p>REGISTRY_AND_SHADOW_AND_CONNECTIVITY_STATUS - Your thing index contains registry
          data, shadow data, and thing connectivity status data.</p>
            </li>
            <li>
               <p>MULTI_INDEXING_MODE - Your thing index contains multiple data sources. For more information, see 
          <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_GetIndexingConfiguration.html">GetIndexingConfiguration</a>.</p>
            </li>
         </ul> |
| `index_status` | String | <p>The index status.</p> |
| `index_name` | String | <p>The index name.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access index outputs
index_id = index.id
index_schema = index.schema
index_index_status = index.index_status
index_index_name = index.index_name
```

---


### Registration_code

RegistrationCode resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registration_code` | String | <p>The CA certificate registration code.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registration_code outputs
registration_code_id = registration_code.id
registration_code_registration_code = registration_code.registration_code
```

---


### Thing_registration_task

ThingRegistrationTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_id` | String | <p>The task ID.</p> |
| `input_file_bucket` | String | <p>The S3 bucket that contains the input file.</p> |
| `role_arn` | String | <p>The role ARN that grants access to the input file bucket.</p> |
| `creation_date` | String | <p>The task creation date.</p> |
| `last_modified_date` | String | <p>The date when the task was last modified.</p> |
| `input_file_key` | String | <p>The input file key.</p> |
| `template_body` | String | <p>The task's template.</p> |
| `message` | String | <p>The message.</p> |
| `success_count` | i64 | <p>The number of things successfully provisioned.</p> |
| `percentage_progress` | i64 | <p>The progress of the bulk provisioning task expressed as a percentage.</p> |
| `failure_count` | i64 | <p>The number of things that failed to be provisioned.</p> |
| `status` | String | <p>The status of the bulk thing provisioning task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access thing_registration_task outputs
thing_registration_task_id = thing_registration_task.id
thing_registration_task_task_id = thing_registration_task.task_id
thing_registration_task_input_file_bucket = thing_registration_task.input_file_bucket
thing_registration_task_role_arn = thing_registration_task.role_arn
thing_registration_task_creation_date = thing_registration_task.creation_date
thing_registration_task_last_modified_date = thing_registration_task.last_modified_date
thing_registration_task_input_file_key = thing_registration_task.input_file_key
thing_registration_task_template_body = thing_registration_task.template_body
thing_registration_task_message = thing_registration_task.message
thing_registration_task_success_count = thing_registration_task.success_count
thing_registration_task_percentage_progress = thing_registration_task.percentage_progress
thing_registration_task_failure_count = thing_registration_task.failure_count
thing_registration_task_status = thing_registration_task.status
```

---


### Logging_options

LoggingOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_arn` | String | <p>The ARN of the IAM role that grants access.</p> |
| `log_level` | String | <p>The logging level.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access logging_options outputs
logging_options_id = logging_options.id
logging_options_role_arn = logging_options.role_arn
logging_options_log_level = logging_options.log_level
```

---


### Command

Command resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A short text decription of the command.</p> |
| `tags` | Vec<String> |  | <p>Name-value pairs that are used as metadata to manage a command.</p> |
| `mandatory_parameters` | Vec<String> |  | <p>A list of parameters that are required by the <code>StartCommandExecution</code> API.
            These parameters need to be specified only when using the <code>AWS-IoT-FleetWise</code>
            namespace. You can either specify them here or when running the command using the
                <code>StartCommandExecution</code> API.</p> |
| `command_id` | String | ✅ | <p>A unique identifier for the command. We recommend using UUID. Alpha-numeric
            characters, hyphens, and underscores are valid for use here.</p> |
| `display_name` | String |  | <p>The user-friendly name in the console for the command. This name doesn't have to be
            unique. You can update the user-friendly name after you define it.</p> |
| `namespace` | String |  | <p>The namespace of the command. The MQTT reserved topics and validations will be used
            for command executions according to the namespace setting.</p> |
| `payload` | String |  | <p>The payload object for the command. You must specify this information when using
        the <code>AWS-IoT</code> namespace.</p>
         <p>You can upload a static payload file from your local storage that contains the 
        instructions for the device to process. The payload file can use any format. To
        make sure that the device correctly interprets the payload, we recommend you to
        specify the payload content type.</p> |
| `role_arn` | String |  | <p>The IAM role that you must provide when using the <code>AWS-IoT-FleetWise</code> namespace.
        The role grants IoT Device Management the permission to access IoT FleetWise resources 
        for generating the payload for the command. This field is not required when you use the
        <code>AWS-IoT</code> namespace.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp, when the command was created.</p> |
| `namespace` | String | <p>The namespace of the command.</p> |
| `role_arn` | String | <p>The IAM role that you provided when creating the command with <code>AWS-IoT-FleetWise</code>
        as the namespace.</p> |
| `deprecated` | bool | <p>Indicates whether the command has been deprecated.</p> |
| `mandatory_parameters` | Vec<String> | <p>A list of parameters for the command created.</p> |
| `command_arn` | String | <p>The Amazon Resource Number (ARN) of the command. For example,
                <code>arn:aws:iot:<region>:<accountid>:command/<commandId></code>
         </p> |
| `payload` | String | <p>The payload object that you provided for the command.</p> |
| `pending_deletion` | bool | <p>Indicates whether the command is being deleted.</p> |
| `display_name` | String | <p>The user-friendly name in the console for the command.</p> |
| `last_updated_at` | String | <p>The timestamp, when the command was last updated.</p> |
| `command_id` | String | <p>The unique identifier of the command.</p> |
| `description` | String | <p>A short text description of the command.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create command
command = provider.iot.Command {
    command_id = "value"  # <p>A unique identifier for the command. We recommend using UUID. Alpha-numeric
            characters, hyphens, and underscores are valid for use here.</p>
}

# Access command outputs
command_id = command.id
command_created_at = command.created_at
command_namespace = command.namespace
command_role_arn = command.role_arn
command_deprecated = command.deprecated
command_mandatory_parameters = command.mandatory_parameters
command_command_arn = command.command_arn
command_payload = command.payload
command_pending_deletion = command.pending_deletion
command_display_name = command.display_name
command_last_updated_at = command.last_updated_at
command_command_id = command.command_id
command_description = command.description
```

---


### Event_configurations

EventConfigurations resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_configurations` | HashMap<String, String> |  | <p>The new event configuration values.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The creation date of the event configuration.</p> |
| `last_modified_date` | String | <p>The date the event configurations were last modified.</p> |
| `event_configurations` | HashMap<String, String> | <p>The event configurations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_configurations outputs
event_configurations_id = event_configurations.id
event_configurations_creation_date = event_configurations.creation_date
event_configurations_last_modified_date = event_configurations.last_modified_date
event_configurations_event_configurations = event_configurations.event_configurations
```

---


### Fleet_metric

FleetMetric resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_version` | String |  | <p>The query version.</p> |
| `query_string` | String | ✅ | <p>The search query string.</p> |
| `aggregation_type` | String | ✅ | <p>The type of the aggregation query.</p> |
| `period` | i64 | ✅ | <p>The time in seconds between fleet metric emissions. Range [60(1 min), 86400(1 day)] and must be multiple of 60.</p> |
| `description` | String |  | <p>The fleet metric description.</p> |
| `tags` | Vec<String> |  | <p>Metadata, which can be used to manage the fleet metric.</p> |
| `unit` | String |  | <p>Used to support unit transformation such as milliseconds to seconds. The unit must be
      supported by <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_MetricDatum.html">CW metric</a>. Default to null.</p> |
| `metric_name` | String | ✅ | <p>The name of the fleet metric to create.</p> |
| `index_name` | String |  | <p>The name of the index to search.</p> |
| `aggregation_field` | String | ✅ | <p>The field to aggregate.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_name` | String | <p>The name of the fleet metric to describe.</p> |
| `period` | i64 | <p>The time in seconds between fleet metric emissions. Range [60(1 min), 86400(1 day)] and must be multiple of 60.</p> |
| `last_modified_date` | String | <p>The date when the fleet metric is last modified.</p> |
| `aggregation_field` | String | <p>The field to aggregate.</p> |
| `description` | String | <p>The fleet metric description.</p> |
| `metric_arn` | String | <p>The ARN of the fleet metric to describe.</p> |
| `query_version` | String | <p>The query version.</p> |
| `version` | i64 | <p>The version of the fleet metric.</p> |
| `index_name` | String | <p>The name of the index to search.</p> |
| `query_string` | String | <p>The search query string.</p> |
| `aggregation_type` | String | <p>The type of the aggregation query.</p> |
| `creation_date` | String | <p>The date when the fleet metric is created.</p> |
| `unit` | String | <p>Used to support unit transformation such as milliseconds to seconds. The unit must be
      supported by <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_MetricDatum.html">CW metric</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fleet_metric
fleet_metric = provider.iot.Fleet_metric {
    query_string = "value"  # <p>The search query string.</p>
    aggregation_type = "value"  # <p>The type of the aggregation query.</p>
    period = "value"  # <p>The time in seconds between fleet metric emissions. Range [60(1 min), 86400(1 day)] and must be multiple of 60.</p>
    metric_name = "value"  # <p>The name of the fleet metric to create.</p>
    aggregation_field = "value"  # <p>The field to aggregate.</p>
}

# Access fleet_metric outputs
fleet_metric_id = fleet_metric.id
fleet_metric_metric_name = fleet_metric.metric_name
fleet_metric_period = fleet_metric.period
fleet_metric_last_modified_date = fleet_metric.last_modified_date
fleet_metric_aggregation_field = fleet_metric.aggregation_field
fleet_metric_description = fleet_metric.description
fleet_metric_metric_arn = fleet_metric.metric_arn
fleet_metric_query_version = fleet_metric.query_version
fleet_metric_version = fleet_metric.version
fleet_metric_index_name = fleet_metric.index_name
fleet_metric_query_string = fleet_metric.query_string
fleet_metric_aggregation_type = fleet_metric.aggregation_type
fleet_metric_creation_date = fleet_metric.creation_date
fleet_metric_unit = fleet_metric.unit
```

---


### Topic_rule_destination

TopicRuleDestination resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_configuration` | String | ✅ | <p>The topic rule destination configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topic_rule_destination` | String | <p>The topic rule destination.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create topic_rule_destination
topic_rule_destination = provider.iot.Topic_rule_destination {
    destination_configuration = "value"  # <p>The topic rule destination configuration.</p>
}

# Access topic_rule_destination outputs
topic_rule_destination_id = topic_rule_destination.id
topic_rule_destination_topic_rule_destination = topic_rule_destination.topic_rule_destination
```

---


### Thing_connectivity_data

ThingConnectivityData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connected` | bool | <p>A Boolean that indicates the connectivity status.</p> |
| `timestamp` | String | <p>The timestamp of when the event occurred.</p> |
| `disconnect_reason` | String | <p>The reason why the client is disconnecting.</p> |
| `thing_name` | String | <p>The name of your IoT thing.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access thing_connectivity_data outputs
thing_connectivity_data_id = thing_connectivity_data.id
thing_connectivity_data_connected = thing_connectivity_data.connected
thing_connectivity_data_timestamp = thing_connectivity_data.timestamp
thing_connectivity_data_disconnect_reason = thing_connectivity_data.disconnect_reason
thing_connectivity_data_thing_name = thing_connectivity_data.thing_name
```

---


### Thing

Thing resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attribute_payload` | String |  | <p>The attribute payload, which consists of up to three name/value pairs in a JSON
			document. For example:</p>
         <p>
            <code>{\"attributes\":{\"string1\":\"string2\"}}</code>
         </p> |
| `thing_name` | String | ✅ | <p>The name of the thing to create.</p>
         <p>You can't change a thing's name after you create it. To change a thing's name, you must create a
			new thing, give it the new name, and then delete the old thing.</p> |
| `billing_group_name` | String |  | <p>The name of the billing group the thing will be added to.</p> |
| `thing_type_name` | String |  | <p>The name of the thing type associated with the new thing.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `billing_group_name` | String | <p>The name of the billing group the thing belongs to.</p> |
| `thing_id` | String | <p>The ID of the thing to describe.</p> |
| `attributes` | HashMap<String, String> | <p>The thing attributes.</p> |
| `default_client_id` | String | <p>The default MQTT client ID. For a typical device, the thing name is also used as the default MQTT client ID.  
			Although we don’t require a mapping between a thing's registry name and its use of MQTT client IDs, certificates, or 
			shadow state, we recommend that you choose a thing name and use it as the MQTT client ID for the registry and the Device Shadow service.</p>
         <p>This lets you better organize your IoT fleet without removing the flexibility of the underlying device certificate model or shadows.</p> |
| `version` | i64 | <p>The current version of the thing record in the registry.</p>
         <note>
            <p>To avoid unintentional changes to the information in the registry, you can pass
				the version information in the <code>expectedVersion</code> parameter of the
					<code>UpdateThing</code> and <code>DeleteThing</code> calls.</p>
         </note> |
| `thing_name` | String | <p>The name of the thing.</p> |
| `thing_type_name` | String | <p>The thing type name.</p> |
| `thing_arn` | String | <p>The ARN of the thing to describe.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create thing
thing = provider.iot.Thing {
    thing_name = "value"  # <p>The name of the thing to create.</p>
         <p>You can't change a thing's name after you create it. To change a thing's name, you must create a
			new thing, give it the new name, and then delete the old thing.</p>
}

# Access thing outputs
thing_id = thing.id
thing_billing_group_name = thing.billing_group_name
thing_thing_id = thing.thing_id
thing_attributes = thing.attributes
thing_default_client_id = thing.default_client_id
thing_version = thing.version
thing_thing_name = thing.thing_name
thing_thing_type_name = thing.thing_type_name
thing_thing_arn = thing.thing_arn
```

---


### Encryption_configuration

EncryptionConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_arn` | String |  | <p>The ARN of the customer-managed KMS key.</p> |
| `kms_access_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on
         behalf of the customer.</p> |
| `encryption_type` | String | ✅ | <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_type` | String | <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p> |
| `kms_key_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on
         behalf of the customer.</p> |
| `kms_access_role_arn` | String | <p>The ARN of the customer-managed KMS key.</p> |
| `configuration_details` | String | <p>The encryption configuration details that include the status information of the KMS key
         and the KMS access role.</p> |
| `last_modified_date` | String | <p>The date when encryption configuration is last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access encryption_configuration outputs
encryption_configuration_id = encryption_configuration.id
encryption_configuration_encryption_type = encryption_configuration.encryption_type
encryption_configuration_kms_key_arn = encryption_configuration.kms_key_arn
encryption_configuration_kms_access_role_arn = encryption_configuration.kms_access_role_arn
encryption_configuration_configuration_details = encryption_configuration.configuration_details
encryption_configuration_last_modified_date = encryption_configuration.last_modified_date
```

---


### Ca_certificate

CACertificate resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `remove_auto_registration` | bool |  | <p>If true, removes auto registration.</p> |
| `certificate_id` | String | ✅ | <p>The CA certificate identifier.</p> |
| `new_status` | String |  | <p>The updated status of the CA certificate.</p>
         <p>
            <b>Note:</b> The status value REGISTER_INACTIVE is deprecated and
         should not be used.</p> |
| `registration_config` | String |  | <p>Information about the registration configuration.</p> |
| `new_auto_registration_status` | String |  | <p>The new value for the auto registration status. Valid values are: "ENABLE" or
         "DISABLE".</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registration_config` | String | <p>Information about the registration configuration.</p> |
| `certificate_description` | String | <p>The CA certificate description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ca_certificate outputs
ca_certificate_id = ca_certificate.id
ca_certificate_registration_config = ca_certificate.registration_config
ca_certificate_certificate_description = ca_certificate.certificate_description
```

---


### Job_document

JobDocument resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document` | String | <p>The job document content.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_document outputs
job_document_id = job_document.id
job_document_document = job_document.document
```

---


### Provisioning_claim

ProvisioningClaim resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_name` | String | ✅ | <p>The name of the provisioning template to use.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create provisioning_claim
provisioning_claim = provider.iot.Provisioning_claim {
    template_name = "value"  # <p>The name of the provisioning template to use.</p>
}

```

---


### Thing_type

ThingType resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the thing type.</p> |
| `thing_type_name` | String | ✅ | <p>The name of the thing type.</p> |
| `thing_type_properties` | String |  | <p>The ThingTypeProperties for the thing type to create. It contains information about
			the new thing type including a description, and a list of searchable thing attribute
			names.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `thing_type_arn` | String | <p>The thing type ARN.</p> |
| `thing_type_properties` | String | <p>The ThingTypeProperties contains information about the thing type including
			description, a list of searchable thing attribute names, and MQTT5 configuration.</p> |
| `thing_type_metadata` | String | <p>The ThingTypeMetadata contains additional information about the thing type
			including: creation date and time, a value indicating whether the thing type is
			deprecated, and a date and time when it was deprecated.</p> |
| `thing_type_id` | String | <p>The thing type ID.</p> |
| `thing_type_name` | String | <p>The name of the thing type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create thing_type
thing_type = provider.iot.Thing_type {
    thing_type_name = "value"  # <p>The name of the thing type.</p>
}

# Access thing_type outputs
thing_type_id = thing_type.id
thing_type_thing_type_arn = thing_type.thing_type_arn
thing_type_thing_type_properties = thing_type.thing_type_properties
thing_type_thing_type_metadata = thing_type.thing_type_metadata
thing_type_thing_type_id = thing_type.thing_type_id
thing_type_thing_type_name = thing_type.thing_type_name
```

---


### Scheduled_audit

ScheduledAudit resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Metadata that can be used to manage the scheduled audit.</p> |
| `target_check_names` | Vec<String> | ✅ | <p>Which checks are performed during the scheduled audit. Checks must be enabled 
            for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list
            of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> 
            to select which checks are enabled.)</p> |
| `day_of_week` | String |  | <p>The day of the week on which the scheduled audit takes
      place,
      either
      <code>SUN</code>,
      <code>MON</code>, <code>TUE</code>, <code>WED</code>, <code>THU</code>, <code>FRI</code>, or <code>SAT</code>. This field is required if the <code>frequency</code>
      parameter is set to <code>WEEKLY</code> or <code>BIWEEKLY</code>.</p> |
| `day_of_month` | String |  | <p>The day of the month on which the scheduled audit takes place.
      This
      can be "1" through "31" or "LAST". This field is required if the "frequency"
      parameter is set to <code>MONTHLY</code>. If days
      29
      to 31 are specified, and the month
      doesn't
      have that many days, the audit takes place on the <code>LAST</code> day of the month.</p> |
| `scheduled_audit_name` | String | ✅ | <p>The name you want to give to the scheduled audit. (Max. 128 chars)</p> |
| `frequency` | String | ✅ | <p>How often the scheduled audit takes
      place, either
      <code>DAILY</code>,
      <code>WEEKLY</code>, <code>BIWEEKLY</code> or <code>MONTHLY</code>. The start time of each audit is
      determined by the system.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `day_of_week` | String | <p>The day of the week on which the scheduled audit takes
      place,
      either one of
            <code>SUN</code>, <code>MON</code>, <code>TUE</code>, <code>WED</code>, <code>THU</code>, <code>FRI</code>, or <code>SAT</code>.</p> |
| `frequency` | String | <p>How often the scheduled audit takes
      place, either
      one of <code>DAILY</code>,
            <code>WEEKLY</code>, <code>BIWEEKLY</code>, or <code>MONTHLY</code>. The start time of each audit is determined by the
      system.</p> |
| `day_of_month` | String | <p>The day of the month on which the scheduled audit takes place.
      This is
      will be <code>1</code>
            through <code>31</code> or <code>LAST</code>. If days
      <code>29</code>-<code>31</code>
      are specified, and the month does not have that many days, the audit takes place on the <code>LAST</code>
      day of the month.</p> |
| `target_check_names` | Vec<String> | <p>Which checks are performed during the scheduled audit. Checks must be 
            enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list
            of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> 
            to select which checks are enabled.)</p> |
| `scheduled_audit_name` | String | <p>The name of the scheduled audit.</p> |
| `scheduled_audit_arn` | String | <p>The ARN of the scheduled audit.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scheduled_audit
scheduled_audit = provider.iot.Scheduled_audit {
    target_check_names = "value"  # <p>Which checks are performed during the scheduled audit. Checks must be enabled 
            for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list
            of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> 
            to select which checks are enabled.)</p>
    scheduled_audit_name = "value"  # <p>The name you want to give to the scheduled audit. (Max. 128 chars)</p>
    frequency = "value"  # <p>How often the scheduled audit takes
      place, either
      <code>DAILY</code>,
      <code>WEEKLY</code>, <code>BIWEEKLY</code> or <code>MONTHLY</code>. The start time of each audit is
      determined by the system.</p>
}

# Access scheduled_audit outputs
scheduled_audit_id = scheduled_audit.id
scheduled_audit_day_of_week = scheduled_audit.day_of_week
scheduled_audit_frequency = scheduled_audit.frequency
scheduled_audit_day_of_month = scheduled_audit.day_of_month
scheduled_audit_target_check_names = scheduled_audit.target_check_names
scheduled_audit_scheduled_audit_name = scheduled_audit.scheduled_audit_name
scheduled_audit_scheduled_audit_arn = scheduled_audit.scheduled_audit_arn
```

---


### Job

Job resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `targets` | Vec<String> | ✅ | <p>A list of things and thing groups to which the job should be sent.</p> |
| `document_source` | String |  | <p>An S3 link, or S3 object URL, to the job document. The link is an Amazon S3 object URL
            and is required if you don't specify a value for <code>document</code>.</p>
         <p>For example, <code>--document-source
                https://s3.<i>region-code</i>.amazonaws.com/example-firmware/device-firmware.1.0</code>
         </p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-bucket-intro.html">Methods for accessing a
                bucket</a>.</p> |
| `job_executions_rollout_config` | String |  | <p>Allows you to create a staged rollout of the job.</p> |
| `document_parameters` | HashMap<String, String> |  | <p>Parameters of an Amazon Web Services managed template that you can specify to create the job
            document.</p>
         <note>
            <p>
               <code>documentParameters</code> can only be used when creating jobs from Amazon Web Services
                managed templates. This parameter can't be used with custom job templates or to
                create jobs from them.</p>
         </note> |
| `namespace_id` | String |  | <p>The namespace used to indicate that a job is a customer-managed job.</p>
         <p>When you specify a value for this parameter, Amazon Web Services IoT Core sends jobs notifications to
            MQTT topics that contain the value in the following format.</p>
         <p>
            <code>$aws/things/<i>THING_NAME</i>/jobs/<i>JOB_ID</i>/notify-namespace-<i>NAMESPACE_ID</i>/</code>
         </p>
         <note>
            <p>The <code>namespaceId</code> feature is only supported by IoT Greengrass at this time. For
                more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/setting-up.html">Setting up IoT Greengrass core devices.</a>
            </p>
         </note> |
| `job_executions_retry_config` | String |  | <p>Allows you to create the criteria to retry a job.</p> |
| `timeout_config` | String |  | <p>Specifies the amount of time each device has to finish its execution of the job. The
            timer is started when the job execution status is set to <code>IN_PROGRESS</code>. If
            the job execution status is not set to another terminal state before the time expires,
            it will be automatically set to <code>TIMED_OUT</code>.</p> |
| `document` | String |  | <p>The job document. Required if you don't specify a value for
                <code>documentSource</code>.</p> |
| `job_template_arn` | String |  | <p>The ARN of the job template used to create the job.</p> |
| `destination_package_versions` | Vec<String> |  | <p>The package version Amazon Resource Names (ARNs) that are installed on the device when the job
            successfully completes. The package version must be in either the Published or
            Deprecated state when the job deploys. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/preparing-to-use-software-package-catalog.html#package-version-lifecycle">Package version lifecycle</a>. </p>
         <p>
            <b>Note:</b>The following Length Constraints relates to a
            single ARN. Up to 25 package version ARNs are allowed.</p> |
| `presigned_url_config` | String |  | <p>Configuration information for pre-signed S3 URLs.</p> |
| `abort_config` | String |  | <p>Allows you to create the criteria to abort a job.</p> |
| `scheduling_config` | String |  | <p>The configuration that allows you to schedule a job for a future date and time in
            addition to specifying the end behavior for each job execution.</p> |
| `job_id` | String | ✅ | <p>A job identifier which must be unique for your account. We recommend using a UUID.
            Alpha-numeric characters, "-" and "_" are valid for use here.</p> |
| `description` | String |  | <p>A short text description of the job.</p> |
| `target_selection` | String |  | <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete
            after all those things specified as targets have completed the job (SNAPSHOT). If
            continuous, the job may also be run on a thing when a change is detected in a target.
            For example, a job will run on a thing when the thing is added to a target group, even
            after the job was completed by all things originally in the group.</p>
         <note>
            <p>We recommend that you use continuous jobs instead of snapshot jobs for dynamic
                thing group targets. By using continuous jobs, devices that join the group receive
                the job execution even after the job has been created.</p>
         </note> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_source` | String | <p>An S3 link to the job document.</p> |
| `job` | String | <p>Information about the job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job
job = provider.iot.Job {
    targets = "value"  # <p>A list of things and thing groups to which the job should be sent.</p>
    job_id = "value"  # <p>A job identifier which must be unique for your account. We recommend using a UUID.
            Alpha-numeric characters, "-" and "_" are valid for use here.</p>
}

# Access job outputs
job_id = job.id
job_document_source = job.document_source
job_job = job.job
```

---


### Package_configuration

PackageConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. 
      Don't reuse this client token if a new idempotent request is required.</p> |
| `version_update_by_jobs_config` | String |  | <p>Configuration to manage job's package version reporting. This updates the thing's reserved named shadow that the job targets.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version_update_by_jobs_config` | String | <p>The version that is associated to a specific job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package_configuration outputs
package_configuration_id = package_configuration.id
package_configuration_version_update_by_jobs_config = package_configuration.version_update_by_jobs_config
```

---


### Provisioning_template_version

ProvisioningTemplateVersion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `set_as_default` | bool |  | <p>Sets a fleet provision template version as the default version.</p> |
| `template_name` | String | ✅ | <p>The name of the provisioning template.</p> |
| `template_body` | String | ✅ | <p>The JSON formatted contents of the provisioning template.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The date when the provisioning template version was created.</p> |
| `template_body` | String | <p>The JSON formatted contents of the provisioning template version.</p> |
| `is_default_version` | bool | <p>True if the provisioning template version is the default version.</p> |
| `version_id` | i64 | <p>The provisioning template version ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create provisioning_template_version
provisioning_template_version = provider.iot.Provisioning_template_version {
    template_name = "value"  # <p>The name of the provisioning template.</p>
    template_body = "value"  # <p>The JSON formatted contents of the provisioning template.</p>
}

# Access provisioning_template_version outputs
provisioning_template_version_id = provisioning_template_version.id
provisioning_template_version_creation_date = provisioning_template_version.creation_date
provisioning_template_version_template_body = provisioning_template_version.template_body
provisioning_template_version_is_default_version = provisioning_template_version.is_default_version
provisioning_template_version_version_id = provisioning_template_version.version_id
```

---


### Authorizer

Authorizer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_caching_for_http` | bool |  | <p>When <code>true</code>, the result from the authorizer’s Lambda function is
	  cached for clients that use persistent HTTP connections. The results are cached for the time
	  specified by the Lambda function in <code>refreshAfterInSeconds</code>. This value
     does not affect authorization of clients that use MQTT connections.</p>
         <p>The default value is <code>false</code>.</p> |
| `authorizer_name` | String | ✅ | <p>The authorizer name.</p> |
| `authorizer_function_arn` | String | ✅ | <p>The ARN of the authorizer's Lambda function.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the custom authorizer.</p>
         <note>
            <p>For URI Request parameters use format: ...key1=value1&key2=value2...</p>
            <p>For the CLI command-line parameter use format: &&tags
            "key1=value1&key2=value2..."</p>
            <p>For the cli-input-json file use format: "tags":
            "key1=value1&key2=value2..."</p>
         </note> |
| `signing_disabled` | bool |  | <p>Specifies whether IoT validates the token signature in an authorization request.</p> |
| `token_signing_public_keys` | HashMap<String, String> |  | <p>The public keys used to verify the digital signature returned by your custom
         authentication service.</p> |
| `status` | String |  | <p>The status of the create authorizer request.</p> |
| `token_key_name` | String |  | <p>The name of the token key used to extract the token from the HTTP headers.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorizer_description` | String | <p>The authorizer description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create authorizer
authorizer = provider.iot.Authorizer {
    authorizer_name = "value"  # <p>The authorizer name.</p>
    authorizer_function_arn = "value"  # <p>The ARN of the authorizer's Lambda function.</p>
}

# Access authorizer outputs
authorizer_id = authorizer.id
authorizer_authorizer_description = authorizer.authorizer_description
```

---


### Certificate_provider

CertificateProvider resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_provider_name` | String | ✅ | <p>The name of the certificate provider.</p> |
| `lambda_function_arn` | String | ✅ | <p>The ARN of the Lambda function that defines the authentication logic.</p> |
| `account_default_for_operations` | Vec<String> | ✅ | <p>A list of the operations that the certificate provider will use to generate certificates. 
         Valid value: <code>CreateCertificateFromCsr</code>.</p> |
| `client_token` | String |  | <p>A string that you can optionally pass in the <code>CreateCertificateProvider</code> request to make sure 
         the request is idempotent.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the certificate provider.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate_provider_arn` | String | <p>The ARN of the certificate provider.</p> |
| `account_default_for_operations` | Vec<String> | <p>A list of the operations that the certificate provider will use to generate certificates. 
         Valid value: <code>CreateCertificateFromCsr</code>.</p> |
| `creation_date` | String | <p>The date-time string that indicates when the certificate provider was created.</p> |
| `last_modified_date` | String | <p>The date-time string that indicates when the certificate provider was last updated.</p> |
| `certificate_provider_name` | String | <p>The name of the certificate provider.</p> |
| `lambda_function_arn` | String | <p>The Lambda function ARN that's associated with the certificate provider.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create certificate_provider
certificate_provider = provider.iot.Certificate_provider {
    certificate_provider_name = "value"  # <p>The name of the certificate provider.</p>
    lambda_function_arn = "value"  # <p>The ARN of the Lambda function that defines the authentication logic.</p>
    account_default_for_operations = "value"  # <p>A list of the operations that the certificate provider will use to generate certificates. 
         Valid value: <code>CreateCertificateFromCsr</code>.</p>
}

# Access certificate_provider outputs
certificate_provider_id = certificate_provider.id
certificate_provider_certificate_provider_arn = certificate_provider.certificate_provider_arn
certificate_provider_account_default_for_operations = certificate_provider.account_default_for_operations
certificate_provider_creation_date = certificate_provider.creation_date
certificate_provider_last_modified_date = certificate_provider.last_modified_date
certificate_provider_certificate_provider_name = certificate_provider.certificate_provider_name
certificate_provider_lambda_function_arn = certificate_provider.lambda_function_arn
```

---


### Percentiles

Percentiles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `percentiles` | Vec<String> | <p>The percentile values of the aggregated fields.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access percentiles outputs
percentiles_id = percentiles.id
percentiles_percentiles = percentiles.percentiles
```

---


### Package_version

PackageVersion resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `artifact` | String |  | <p>The various build components created during the build process such as libraries and
         configuration files that make up a software package version.</p> |
| `client_token` | String |  | <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. 
      Don't reuse this client token if a new idempotent request is required.</p> |
| `attributes` | HashMap<String, String> |  | <p>Metadata that can be used to define a package version’s configuration. For example, the S3 file location, configuration options that are being sent to the device or fleet.</p>
         <p>The combined size of all the attributes on a package version is limited to 3KB.</p> |
| `recipe` | String |  | <p>The inline job document associated with a software package version used for a quick job
         deployment.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that can be used to manage the package version.</p> |
| `description` | String |  | <p>A summary of the package version being created. This can be used to outline the package's contents or purpose.</p> |
| `version_name` | String | ✅ | <p>The name of the new package version.</p> |
| `package_name` | String | ✅ | <p>The name of the associated software package.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status associated to the package version. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/preparing-to-use-software-package-catalog.html#package-version-lifecycle">Package version lifecycle</a>.</p> |
| `version_name` | String | <p>The name of the package version.</p> |
| `error_reason` | String | <p>Error reason for a package version failure during creation or update.</p> |
| `attributes` | HashMap<String, String> | <p>Metadata that were added to the package version that can be used to define a package version’s configuration.</p> |
| `creation_date` | String | <p>The date when the package version was created.</p> |
| `package_version_arn` | String | <p>The ARN for the package version.</p> |
| `package_name` | String | <p>The name of the software package.</p> |
| `description` | String | <p>The package version description.</p> |
| `last_modified_date` | String | <p>The date when the package version was last updated.</p> |
| `artifact` | String | <p>The various components that make up a software package version.</p> |
| `sbom` | String | <p>The software bill of materials for a software package version.</p> |
| `sbom_validation_status` | String | <p>The status of the validation for a new software bill of materials added to a software
         package version.</p> |
| `recipe` | String | <p>The inline job document associated with a software package version used for a quick job
         deployment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package_version
package_version = provider.iot.Package_version {
    version_name = "value"  # <p>The name of the new package version.</p>
    package_name = "value"  # <p>The name of the associated software package.</p>
}

# Access package_version outputs
package_version_id = package_version.id
package_version_status = package_version.status
package_version_version_name = package_version.version_name
package_version_error_reason = package_version.error_reason
package_version_attributes = package_version.attributes
package_version_creation_date = package_version.creation_date
package_version_package_version_arn = package_version.package_version_arn
package_version_package_name = package_version.package_name
package_version_description = package_version.description
package_version_last_modified_date = package_version.last_modified_date
package_version_artifact = package_version.artifact
package_version_sbom = package_version.sbom
package_version_sbom_validation_status = package_version.sbom_validation_status
package_version_recipe = package_version.recipe
```

---


### Buckets_aggregation

BucketsAggregation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_count` | i64 | <p>The total number of things that fit the query string criteria.</p> |
| `buckets` | Vec<String> | <p>The main part of the response with a list of buckets. Each bucket contains a <code>keyValue</code> and a <code>count</code>.</p>
         <p>
            <code>keyValue</code>: The aggregation field value counted for the particular bucket.</p>
         <p>
            <code>count</code>: The number of documents that have that value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access buckets_aggregation outputs
buckets_aggregation_id = buckets_aggregation.id
buckets_aggregation_total_count = buckets_aggregation.total_count
buckets_aggregation_buckets = buckets_aggregation.buckets
```

---


### Audit_mitigation_actions_task

AuditMitigationActionsTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_check_to_actions_mapping` | HashMap<String, Vec<String>> | <p>Specifies the mitigation actions that should be applied to specific audit checks.</p> |
| `actions_definition` | Vec<String> | <p>Specifies the mitigation actions and their parameters that are applied as part of this task.</p> |
| `end_time` | String | <p>The date and time when the task was completed or canceled.</p> |
| `task_statistics` | HashMap<String, String> | <p>Aggregate counts of the results when the mitigation tasks were applied to the findings for this audit mitigation actions task.</p> |
| `target` | String | <p>Identifies the findings to which the mitigation actions are applied. This can be by audit checks, by audit task, or a set of findings.</p> |
| `task_status` | String | <p>The current status of the task.</p> |
| `start_time` | String | <p>The date and time when the task was started.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access audit_mitigation_actions_task outputs
audit_mitigation_actions_task_id = audit_mitigation_actions_task.id
audit_mitigation_actions_task_audit_check_to_actions_mapping = audit_mitigation_actions_task.audit_check_to_actions_mapping
audit_mitigation_actions_task_actions_definition = audit_mitigation_actions_task.actions_definition
audit_mitigation_actions_task_end_time = audit_mitigation_actions_task.end_time
audit_mitigation_actions_task_task_statistics = audit_mitigation_actions_task.task_statistics
audit_mitigation_actions_task_target = audit_mitigation_actions_task.target
audit_mitigation_actions_task_task_status = audit_mitigation_actions_task.task_status
audit_mitigation_actions_task_start_time = audit_mitigation_actions_task.start_time
```

---


### Custom_metric

CustomMetric resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p> The friendly name in the console for the custom metric. This name doesn't have to be
      unique. Don't use this name as the metric identifier in the device metric report. You can
      update the friendly name after you define it.</p> |
| `tags` | Vec<String> |  | <p>
      Metadata that can be used to manage the custom metric.
    </p> |
| `metric_type` | String | ✅ | <p> The type of the custom metric. </p>
         <important>
            <p>The type <code>number</code> only takes a single metric value as an input, but when you
        submit the metrics value in the DeviceMetrics report, you must pass it as an array with a
        single value.</p>
         </important> |
| `metric_name` | String | ✅ | <p> The name of the custom metric. This will be used in the metric report submitted from the
      device/thing. The name can't begin with <code>aws:</code>. You can't change the name after you
      define it.</p> |
| `client_request_token` | String | ✅ | <p>Each custom
      metric must have a unique client request token. If you try to create a new custom metric that
      already exists with a different token,
      an exception
      occurs. If you omit this value, Amazon Web Services SDKs will automatically generate a unique client request. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | <p>
      Field represents a friendly name in the console for the custom metric; doesn't have to be unique. Don't use this name as the metric identifier in the device metric report. Can be updated.
    </p> |
| `metric_name` | String | <p>
      The name of the custom metric.
    </p> |
| `creation_date` | String | <p>
      The creation date of the custom metric in milliseconds since epoch.
    </p> |
| `last_modified_date` | String | <p>
      The time the custom metric was last modified in milliseconds since epoch.
    </p> |
| `metric_type` | String | <p> The type of the custom metric. </p>
         <important>
            <p>The type <code>number</code> only takes a single metric value as an input, but while submitting the metrics value in the DeviceMetrics report, it must be passed as an array with a single value.</p>
         </important> |
| `metric_arn` | String | <p>
      The Amazon Resource Number (ARN) of the custom metric.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_metric
custom_metric = provider.iot.Custom_metric {
    metric_type = "value"  # <p> The type of the custom metric. </p>
         <important>
            <p>The type <code>number</code> only takes a single metric value as an input, but when you
        submit the metrics value in the DeviceMetrics report, you must pass it as an array with a
        single value.</p>
         </important>
    metric_name = "value"  # <p> The name of the custom metric. This will be used in the metric report submitted from the
      device/thing. The name can't begin with <code>aws:</code>. You can't change the name after you
      define it.</p>
    client_request_token = "value"  # <p>Each custom
      metric must have a unique client request token. If you try to create a new custom metric that
      already exists with a different token,
      an exception
      occurs. If you omit this value, Amazon Web Services SDKs will automatically generate a unique client request. </p>
}

# Access custom_metric outputs
custom_metric_id = custom_metric.id
custom_metric_display_name = custom_metric.display_name
custom_metric_metric_name = custom_metric.metric_name
custom_metric_creation_date = custom_metric.creation_date
custom_metric_last_modified_date = custom_metric.last_modified_date
custom_metric_metric_type = custom_metric.metric_type
custom_metric_metric_arn = custom_metric.metric_arn
```

---


### Managed_job_template

ManagedJobTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_parameters` | Vec<String> | <p>A map of key-value pairs that you can use as guidance to specify the inputs for
            creating a job from a managed template.</p>
         <note>
            <p>
               <code>documentParameters</code> can only be used when creating jobs from Amazon Web Services
                managed templates. This parameter can't be used with custom job templates or to
                create jobs from them.</p>
         </note> |
| `template_version` | String | <p>The version for a managed template.</p> |
| `template_arn` | String | <p>The unique Amazon Resource Name (ARN) of the managed template.</p> |
| `environments` | Vec<String> | <p>A list of environments that are supported with the managed job template.</p> |
| `document` | String | <p>The document schema for a managed job template.</p> |
| `description` | String | <p>The unique description of a managed template.</p> |
| `template_name` | String | <p>The unique name of a managed template, such as <code>AWS-Reboot</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_job_template outputs
managed_job_template_id = managed_job_template.id
managed_job_template_document_parameters = managed_job_template.document_parameters
managed_job_template_template_version = managed_job_template.template_version
managed_job_template_template_arn = managed_job_template.template_arn
managed_job_template_environments = managed_job_template.environments
managed_job_template_document = managed_job_template.document
managed_job_template_description = managed_job_template.description
managed_job_template_template_name = managed_job_template.template_name
```

---


### Domain_configuration

DomainConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_configuration_name` | String | ✅ | <p>The name of the domain configuration. This value must be unique to a region.</p> |
| `validation_certificate_arn` | String |  | <p>The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. 
         This value is not required for Amazon Web Services-managed domains.</p> |
| `service_type` | String |  | <p>The type of service delivered by the endpoint.</p>
         <note>
            <p>Amazon Web Services IoT Core currently supports only the <code>DATA</code> service type.</p>
         </note> |
| `authentication_type` | String |  | <p>An enumerated string that speciﬁes the authentication type.</p>
         <ul>
            <li>
               <p>
                  <code>CUSTOM_AUTH_X509</code> - Use custom authentication and authorization with additional details from the X.509 client certificate.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>CUSTOM_AUTH</code> - Use custom authentication and authorization. For more
               information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">Custom authentication and authorization</a>.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>AWS_X509</code> - Use X.509 client certificates without custom authentication and authorization. For more information,
               see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/x509-client-certs.html">X.509 client certificates</a>.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>AWS_SIGV4</code> - Use Amazon Web Services Signature Version 4. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">IAM users, groups, and roles</a>.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify authentication type.
               For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p>
            </li>
         </ul> |
| `application_protocol` | String |  | <p>An enumerated string that speciﬁes the application-layer protocol.</p>
         <ul>
            <li>
               <p>
                  <code>SECURE_MQTT</code> - MQTT over TLS.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>MQTT_WSS</code> - MQTT over WebSocket.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>HTTPS</code> - HTTP over TLS.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify application_layer protocol. 
               For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p>
            </li>
         </ul> |
| `domain_name` | String |  | <p>The name of the domain.</p> |
| `server_certificate_arns` | Vec<String> |  | <p>The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. 
      This value is not required for Amazon Web Services-managed domains.</p> |
| `authorizer_config` | String |  | <p>An object that specifies the authorization service for a domain.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the domain configuration.</p>
         <note>
            <p>For URI Request parameters use format: ...key1=value1&key2=value2...</p>
            <p>For the CLI command-line parameter use format: &&tags
            "key1=value1&key2=value2..."</p>
            <p>For the cli-input-json file use format: "tags":
            "key1=value1&key2=value2..."</p>
         </note> |
| `tls_config` | String |  | <p>An object that specifies the TLS configuration for a domain.</p> |
| `server_certificate_config` | String |  | <p>The server certificate configuration.</p> |
| `client_certificate_config` | String |  | <p>An object that speciﬁes the client certificate conﬁguration for a domain.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `server_certificates` | Vec<String> | <p>A list containing summary information about the server certificate included in the domain configuration.</p> |
| `domain_type` | String | <p>The type of the domain.</p> |
| `last_status_change_date` | String | <p>The date and time the domain configuration's status was last changed.</p> |
| `authentication_type` | String | <p>An enumerated string that speciﬁes the authentication type.</p>
         <ul>
            <li>
               <p>
                  <code>CUSTOM_AUTH_X509</code> - Use custom authentication and authorization with additional details from the X.509 client certificate.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>CUSTOM_AUTH</code> - Use custom authentication and authorization. For more
               information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">Custom authentication and authorization</a>.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>AWS_X509</code> - Use X.509 client certificates without custom authentication and authorization. For more information,
               see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/x509-client-certs.html">X.509 client certificates</a>.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>AWS_SIGV4</code> - Use Amazon Web Services Signature Version 4. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">IAM users, groups, and roles</a>.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify authentication type.
               For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p>
            </li>
         </ul> |
| `domain_configuration_name` | String | <p>The name of the domain configuration.</p> |
| `domain_configuration_arn` | String | <p>The ARN of the domain configuration.</p> |
| `domain_name` | String | <p>The name of the domain.</p> |
| `authorizer_config` | String | <p>An object that specifies the authorization service for a domain.</p> |
| `service_type` | String | <p>The type of service delivered by the endpoint.</p> |
| `tls_config` | String | <p>An object that specifies the TLS configuration for a domain.</p> |
| `client_certificate_config` | String | <p>An object that speciﬁes the client certificate conﬁguration for a domain.</p> |
| `domain_configuration_status` | String | <p>A Boolean value that specifies the current state of the domain configuration.</p> |
| `server_certificate_config` | String | <p>The server certificate configuration.</p> |
| `application_protocol` | String | <p>An enumerated string that speciﬁes the application-layer protocol.</p>
         <ul>
            <li>
               <p>
                  <code>SECURE_MQTT</code> - MQTT over TLS.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>MQTT_WSS</code> - MQTT over WebSocket.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>HTTPS</code> - HTTP over TLS.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify application_layer protocol. 
               For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p>
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

# Create domain_configuration
domain_configuration = provider.iot.Domain_configuration {
    domain_configuration_name = "value"  # <p>The name of the domain configuration. This value must be unique to a region.</p>
}

# Access domain_configuration outputs
domain_configuration_id = domain_configuration.id
domain_configuration_server_certificates = domain_configuration.server_certificates
domain_configuration_domain_type = domain_configuration.domain_type
domain_configuration_last_status_change_date = domain_configuration.last_status_change_date
domain_configuration_authentication_type = domain_configuration.authentication_type
domain_configuration_domain_configuration_name = domain_configuration.domain_configuration_name
domain_configuration_domain_configuration_arn = domain_configuration.domain_configuration_arn
domain_configuration_domain_name = domain_configuration.domain_name
domain_configuration_authorizer_config = domain_configuration.authorizer_config
domain_configuration_service_type = domain_configuration.service_type
domain_configuration_tls_config = domain_configuration.tls_config
domain_configuration_client_certificate_config = domain_configuration.client_certificate_config
domain_configuration_domain_configuration_status = domain_configuration.domain_configuration_status
domain_configuration_server_certificate_config = domain_configuration.server_certificate_config
domain_configuration_application_protocol = domain_configuration.application_protocol
```

---


### Account_audit_configuration

AccountAuditConfiguration resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `audit_notification_target_configurations` | HashMap<String, String> |  | <p>Information about the targets to which audit notifications are sent.</p> |
| `audit_check_configurations` | HashMap<String, String> |  | <p>Specifies which audit checks are enabled and disabled for this account. Use 
            <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those 
            that are currently enabled.</p>
         <p>Some data collection might start immediately when certain checks are enabled. 
            When a check is disabled, any data collected so far in relation to the check is deleted.</p>
         <p>You
      cannot
      disable a check if
      it's
      used by any scheduled audit. You must first delete the check from the scheduled audit or
      delete the scheduled audit itself.</p>
         <p>On the first call to <code>UpdateAccountAuditConfiguration</code>,
            this parameter is required and must specify at least one enabled check.</p> |
| `role_arn` | String |  | <p>The Amazon
      Resource Name
      (ARN)
      of the role that grants permission
      to
      IoT to access information about your devices, policies,
      certificates,
      and other items as required when performing an audit.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_check_configurations` | HashMap<String, String> | <p>Which audit checks are enabled and disabled for this account.</p> |
| `role_arn` | String | <p>The ARN of the role that grants permission to IoT to access information
            about your devices, policies, certificates, and other items as required when 
            performing an audit.</p>
         <p>On the first call to <code>UpdateAccountAuditConfiguration</code>,
            this parameter is required.</p> |
| `audit_notification_target_configurations` | HashMap<String, String> | <p>Information about the targets to which audit notifications are sent for 
            this account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_audit_configuration outputs
account_audit_configuration_id = account_audit_configuration.id
account_audit_configuration_audit_check_configurations = account_audit_configuration.audit_check_configurations
account_audit_configuration_role_arn = account_audit_configuration.role_arn
account_audit_configuration_audit_notification_target_configurations = account_audit_configuration.audit_notification_target_configurations
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple verification_state_on_violation resources
verification_state_on_violation_0 = provider.iot.Verification_state_on_violation {
    verification_state = "value-0"
    violation_id = "value-0"
}
verification_state_on_violation_1 = provider.iot.Verification_state_on_violation {
    verification_state = "value-1"
    violation_id = "value-1"
}
verification_state_on_violation_2 = provider.iot.Verification_state_on_violation {
    verification_state = "value-2"
    violation_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    verification_state_on_violation = provider.iot.Verification_state_on_violation {
        verification_state = "production-value"
        violation_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Iot Documentation](https://docs.aws.amazon.com/iot/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
