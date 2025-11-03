# Ssm Service



**Resources**: 68

---

## Overview

The ssm service provides access to 68 resource types:

- [Document_default_version](#document_default_version) [U]
- [Maintenance_window_targets](#maintenance_window_targets) [R]
- [Parameter_history](#parameter_history) [R]
- [Effective_patches_for_patch_baseline](#effective_patches_for_patch_baseline) [R]
- [Maintenance_window_target](#maintenance_window_target) [U]
- [Association](#association) [CRUD]
- [Association_execution_targets](#association_execution_targets) [R]
- [Calendar_state](#calendar_state) [R]
- [Ops_item](#ops_item) [CRUD]
- [Maintenance_window_executions](#maintenance_window_executions) [R]
- [Activation](#activation) [CD]
- [Effective_instance_associations](#effective_instance_associations) [R]
- [Execution_preview](#execution_preview) [R]
- [Resource_policies](#resource_policies) [R]
- [Association_batch](#association_batch) [C]
- [Instance_associations_status](#instance_associations_status) [R]
- [Document](#document) [CRUD]
- [Ops_summary](#ops_summary) [R]
- [Command_invocation](#command_invocation) [R]
- [Compliance_items](#compliance_items) [C]
- [Patch_baselines](#patch_baselines) [R]
- [Access_token](#access_token) [R]
- [Maintenance_window_tasks](#maintenance_window_tasks) [R]
- [Maintenance_window_execution_tasks](#maintenance_window_execution_tasks) [R]
- [Deployable_patch_snapshot_for_instance](#deployable_patch_snapshot_for_instance) [R]
- [Maintenance_window_schedule](#maintenance_window_schedule) [R]
- [Document_metadata](#document_metadata) [U]
- [Association_status](#association_status) [U]
- [Maintenance_window_execution_task_invocations](#maintenance_window_execution_task_invocations) [R]
- [Instance_patch_states_for_patch_group](#instance_patch_states_for_patch_group) [R]
- [Patch_groups](#patch_groups) [R]
- [Automation_step_executions](#automation_step_executions) [R]
- [Patch_baseline_for_patch_group](#patch_baseline_for_patch_group) [R]
- [Ops_metadata](#ops_metadata) [CRUD]
- [Inventory](#inventory) [CRD]
- [Patch_properties](#patch_properties) [R]
- [Instance_patch_states](#instance_patch_states) [R]
- [Instance_information](#instance_information) [R]
- [Resource_policy](#resource_policy) [CD]
- [Activations](#activations) [R]
- [Maintenance_window_execution_task_invocation](#maintenance_window_execution_task_invocation) [R]
- [Maintenance_windows_for_target](#maintenance_windows_for_target) [R]
- [Service_setting](#service_setting) [RU]
- [Automation_execution](#automation_execution) [R]
- [Patch_group_state](#patch_group_state) [R]
- [Maintenance_window](#maintenance_window) [CRUD]
- [Parameter](#parameter) [CRD]
- [Sessions](#sessions) [R]
- [Resource_data_sync](#resource_data_sync) [CUD]
- [Managed_instance_role](#managed_instance_role) [U]
- [Default_patch_baseline](#default_patch_baseline) [R]
- [Automation_executions](#automation_executions) [R]
- [Connection_status](#connection_status) [R]
- [Ops_items](#ops_items) [R]
- [Maintenance_window_execution](#maintenance_window_execution) [R]
- [Maintenance_window_task](#maintenance_window_task) [RU]
- [Document_permission](#document_permission) [R]
- [Instance_properties](#instance_properties) [R]
- [Inventory_schema](#inventory_schema) [R]
- [Association_executions](#association_executions) [R]
- [Parameters_by_path](#parameters_by_path) [R]
- [Patch_baseline](#patch_baseline) [CRUD]
- [Parameters](#parameters) [RD]
- [Available_patches](#available_patches) [R]
- [Maintenance_window_execution_task](#maintenance_window_execution_task) [R]
- [Instance_patches](#instance_patches) [R]
- [Inventory_deletions](#inventory_deletions) [R]
- [Maintenance_windows](#maintenance_windows) [R]

---

## Resources


### Document_default_version

DocumentDefaultVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of a custom document that you want to set as the default version.</p> |
| `document_version` | String | ✅ | <p>The version of a custom document that you want to set as the default version.</p> |



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


### Maintenance_window_targets

MaintenanceWindowTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |
| `targets` | Vec<String> | <p>Information about the targets in the maintenance window.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_targets outputs
maintenance_window_targets_id = maintenance_window_targets.id
maintenance_window_targets_next_token = maintenance_window_targets.next_token
maintenance_window_targets_targets = maintenance_window_targets.targets
```

---


### Parameter_history

ParameterHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | <p>A list of parameters returned by the request.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access parameter_history outputs
parameter_history_id = parameter_history.id
parameter_history_parameters = parameter_history.parameters
parameter_history_next_token = parameter_history.next_token
```

---


### Effective_patches_for_patch_baseline

EffectivePatchesForPatchBaseline resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effective_patches` | Vec<String> | <p>An array of patches and patch status.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access effective_patches_for_patch_baseline outputs
effective_patches_for_patch_baseline_id = effective_patches_for_patch_baseline.id
effective_patches_for_patch_baseline_effective_patches = effective_patches_for_patch_baseline.effective_patches
effective_patches_for_patch_baseline_next_token = effective_patches_for_patch_baseline.next_token
```

---


### Maintenance_window_target

MaintenanceWindowTarget resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>An optional description for the update.</p> |
| `window_target_id` | String | ✅ | <p>The target ID to modify.</p> |
| `window_id` | String | ✅ | <p>The maintenance window ID with which to modify the target.</p> |
| `name` | String |  | <p>A name for the update.</p> |
| `owner_information` | String |  | <p>User-provided value that will be included in any Amazon CloudWatch Events events raised while
   running tasks for these targets in this maintenance window.</p> |
| `targets` | Vec<String> |  | <p>The targets to add or replace.</p> |
| `replace` | bool |  | <p>If <code>True</code>, then all fields that are required by the <a>RegisterTargetWithMaintenanceWindow</a> operation are also required for this API
   request. Optional fields that aren't specified are set to null.</p> |



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


### Association

Association resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the SSM Command document or Automation runbook that contains the configuration
   information for the managed node.</p>
         <p>You can specify Amazon Web Services-predefined documents, documents you created, or a document that is
   shared with you from another Amazon Web Services account.</p>
         <p>For Systems Manager documents (SSM documents) that are shared with you from other Amazon Web Services accounts, you
   must specify the complete SSM document ARN, in the following format:</p>
         <p>
            <code>arn:<i>partition</i>:ssm:<i>region</i>:<i>account-id</i>:document/<i>document-name</i>
            </code>
         </p>
         <p>For example:</p>
         <p>
            <code>arn:aws:ssm:us-east-2:12345678912:document/My-Shared-Document</code>
         </p>
         <p>For Amazon Web Services-predefined documents and SSM documents you created in your account, you only need
   to specify the document name. For example, <code>AWS-ApplyPatchBaseline</code> or
    <code>My-Document</code>.</p> |
| `sync_compliance` | String |  | <p>The mode for generating association compliance. You can specify <code>AUTO</code> or
    <code>MANUAL</code>. In <code>AUTO</code> mode, the system uses the status of the association
   execution to determine the compliance status. If the association execution runs successfully,
   then the association is <code>COMPLIANT</code>. If the association execution doesn't run
   successfully, the association is <code>NON-COMPLIANT</code>.</p>
         <p>In <code>MANUAL</code> mode, you must specify the <code>AssociationId</code> as a parameter
   for the <a>PutComplianceItems</a> API operation. In this case, compliance data isn't
   managed by State Manager. It is managed by your direct call to the <a>PutComplianceItems</a> API operation.</p>
         <p>By default, all associations use <code>AUTO</code> mode.</p> |
| `schedule_expression` | String |  | <p>A cron expression when the association will be applied to the targets.</p> |
| `alarm_configuration` | String |  |  |
| `duration` | i64 |  | <p>The number of hours the association can run before it is canceled. Duration applies to
   associations that are currently running, and any pending and in progress commands on all targets.
   If a target was taken offline for the association to run, it is made available again immediately,
   without a reboot. </p>
         <p>The <code>Duration</code> parameter applies only when both these conditions are true:</p>
         <ul>
            <li>
               <p>The association for which you specify a duration is cancelable according to the parameters
     of the SSM command document or Automation runbook associated with this execution. </p>
            </li>
            <li>
               <p>The command specifies the <code>
                     <a href="https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_CreateAssociation.html#systemsmanager-CreateAssociation-request-ApplyOnlyAtCronInterval">ApplyOnlyAtCronInterval</a>
                  </code> parameter, which means that the association doesn't
     run immediately after it is created, but only according to the specified schedule.</p>
            </li>
         </ul> |
| `parameters` | HashMap<String, Vec<String>> |  | <p>The parameters for the runtime configuration of the document.</p> |
| `instance_id` | String |  | <p>The managed node ID.</p>
         <note>
            <p>
               <code>InstanceId</code> has been deprecated. To specify a managed node ID for an
    association, use the <code>Targets</code> parameter. Requests that
    include the parameter <code>InstanceID</code> with Systems Manager documents (SSM documents) that use
    schema version 2.0 or later will fail. In addition, if you use the
    parameter <code>InstanceId</code>, you can't use the parameters <code>AssociationName</code>,
     <code>DocumentVersion</code>, <code>MaxErrors</code>, <code>MaxConcurrency</code>,
     <code>OutputLocation</code>, or <code>ScheduleExpression</code>. To use these parameters, you
    must use the <code>Targets</code> parameter.</p>
         </note> |
| `output_location` | String |  | <p>An Amazon Simple Storage Service (Amazon S3) bucket where you want to store the output
   details of the request.</p> |
| `max_errors` | String |  | <p>The number of errors that are allowed before the system stops sending requests to run the
   association on additional targets. You can specify either an absolute number of errors, for
   example 10, or a percentage of the target set, for example 10%. If you specify 3, for example,
   the system stops sending requests when the fourth error is received. If you specify 0, then the
   system stops sending requests after the first error is returned. If you run an association on 50
   managed nodes and set <code>MaxError</code> to 10%, then the system stops sending the request
   when the sixth error is received.</p>
         <p>Executions that are already running an association when <code>MaxErrors</code> is reached
   are allowed to complete, but some of these executions may fail as well. If you need to ensure
   that there won't be more than max-errors failed executions, set <code>MaxConcurrency</code> to 1
   so that executions proceed one at a time.</p> |
| `max_concurrency` | String |  | <p>The maximum number of targets allowed to run the association at the same time. You can
   specify a number, for example 10, or a percentage of the target set, for example 10%. The default
   value is 100%, which means all targets run the association at the same time.</p>
         <p>If a new managed node starts and attempts to run an association while Systems Manager is running
    <code>MaxConcurrency</code> associations, the association is allowed to run. During the next
   association interval, the new managed node will process its association within the limit
   specified for <code>MaxConcurrency</code>.</p> |
| `apply_only_at_cron_interval` | bool |  | <p>By default, when you create a new association, the system runs it immediately after it is
   created and then according to the schedule you specified and when target changes are detected.
   Specify <code>true</code> for <code>ApplyOnlyAtCronInterval</code>if you want the association to
   run only according to the schedule you specified.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/state-manager-about.html#state-manager-about-scheduling">Understanding when associations are applied to resources</a> and <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/state-manager-about.html#runbook-target-updates">>About
    target updates with Automation runbooks</a> in the
   <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <p>This parameter isn't supported for rate expressions.</p> |
| `tags` | Vec<String> |  | <p>Adds or overwrites one or more tags for a State Manager association.
    <i>Tags</i> are metadata that you can assign to your Amazon Web Services resources. Tags enable
   you to categorize your resources in different ways, for example, by purpose, owner, or
   environment. Each tag consists of a key and an optional value, both of which you define. </p> |
| `association_name` | String |  | <p>Specify a descriptive name for the association.</p> |
| `schedule_offset` | i64 |  | <p>Number of days to wait after the scheduled day to run an association. For example, if you
   specified a cron schedule of <code>cron(0 0 ? * THU#2 *)</code>, you could specify an offset of 3
   to run the association each Sunday after the second Thursday of the month. For more information
   about cron schedules for associations, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html">Reference: Cron
    and rate expressions for Systems Manager</a> in the <i>Amazon Web Services Systems Manager User Guide</i>. </p>
         <note>
            <p>To use offsets, you must specify the <code>ApplyOnlyAtCronInterval</code> parameter. This
    option tells the system not to run an association immediately after you create it. </p>
         </note> |
| `target_locations` | Vec<String> |  | <p>A location is a combination of Amazon Web Services Regions and Amazon Web Services accounts where you want to run the
   association. Use this action to create an association in multiple Regions and multiple
   accounts.</p>
         <note>
            <p>The <code>IncludeChildOrganizationUnits</code> parameter is not supported by State Manager.</p>
         </note> |
| `calendar_names` | Vec<String> |  | <p>The names of Amazon Resource Names (ARNs) of the Change Calendar type documents you want to
   gate your associations under. The associations only run when that change calendar is open. For
   more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-change-calendar">Amazon Web Services Systems Manager Change
    Calendar</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `target_maps` | Vec<HashMap<String, Vec<String>>> |  | <p>A key-value mapping of document parameters to target resources. Both Targets and TargetMaps
   can't be specified together.</p> |
| `document_version` | String |  | <p>The document version you want to associate with the targets. Can be a specific version or
   the default version.</p>
         <important>
            <p>State Manager doesn't support running associations that use a new version of a document if
    that document is shared from another account. State Manager always runs the <code>default</code>
    version of a document if shared from another account, even though the Systems Manager console shows that a
    new version was processed. If you want to run an association using a new version of a document
    shared form another account, you must set the document version to <code>default</code>.</p>
         </important> |
| `targets` | Vec<String> |  | <p>The targets for the association. You can target managed nodes by using tags, Amazon Web Services resource
   groups, all managed nodes in an Amazon Web Services account, or individual managed node IDs. You can target all
   managed nodes in an Amazon Web Services account by specifying the <code>InstanceIds</code> key with a value of
    <code>*</code>. For more information about choosing targets for an association, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-state-manager-targets-and-rate-controls.html">Understanding targets and rate controls in State Manager associations</a> in the
    <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `automation_target_parameter_name` | String |  | <p>Choose the parameter that will define how your automation will branch out. This target is
   required for associations that use an Automation runbook and target resources by using rate
   controls. Automation is a tool in Amazon Web Services Systems Manager.</p> |
| `compliance_severity` | String |  | <p>The severity level to assign to the association.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `association_description` | String | <p>Information about the association.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create association
association = provider.ssm.Association {
    name = "value"  # <p>The name of the SSM Command document or Automation runbook that contains the configuration
   information for the managed node.</p>
         <p>You can specify Amazon Web Services-predefined documents, documents you created, or a document that is
   shared with you from another Amazon Web Services account.</p>
         <p>For Systems Manager documents (SSM documents) that are shared with you from other Amazon Web Services accounts, you
   must specify the complete SSM document ARN, in the following format:</p>
         <p>
            <code>arn:<i>partition</i>:ssm:<i>region</i>:<i>account-id</i>:document/<i>document-name</i>
            </code>
         </p>
         <p>For example:</p>
         <p>
            <code>arn:aws:ssm:us-east-2:12345678912:document/My-Shared-Document</code>
         </p>
         <p>For Amazon Web Services-predefined documents and SSM documents you created in your account, you only need
   to specify the document name. For example, <code>AWS-ApplyPatchBaseline</code> or
    <code>My-Document</code>.</p>
}

# Access association outputs
association_id = association.id
association_association_description = association.association_description
```

---


### Association_execution_targets

AssociationExecutionTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |
| `association_execution_targets` | Vec<String> | <p>Information about the execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access association_execution_targets outputs
association_execution_targets_id = association_execution_targets.id
association_execution_targets_next_token = association_execution_targets.next_token
association_execution_targets_association_execution_targets = association_execution_targets.association_execution_targets
```

---


### Calendar_state

CalendarState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | <p>The state of the calendar. An <code>OPEN</code> calendar indicates that actions are allowed
   to proceed, and a <code>CLOSED</code> calendar indicates that actions aren't allowed to
   proceed.</p> |
| `at_time` | String | <p>The time, as an <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601</a> string,
   that you specified in your command. If you don't specify a time, <code>GetCalendarState</code>
   uses the current time.</p> |
| `next_transition_time` | String | <p>The time, as an <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601</a> string,
   that the calendar state will change. If the current calendar state is <code>OPEN</code>,
    <code>NextTransitionTime</code> indicates when the calendar state changes to
   <code>CLOSED</code>, and vice-versa.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access calendar_state outputs
calendar_state_id = calendar_state.id
calendar_state_state = calendar_state.state
calendar_state_at_time = calendar_state.at_time
calendar_state_next_transition_time = calendar_state.next_transition_time
```

---


### Ops_item

OpsItem resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>User-defined text that contains information about the OpsItem, in Markdown format. </p>
         <note>
            <p>Provide enough information so that users viewing this OpsItem for the first time understand
    the issue. </p>
         </note> |
| `actual_end_time` | String |  | <p>The time a runbook workflow ended. Currently reported only for the OpsItem type
    <code>/aws/changerequest</code>.</p> |
| `priority` | i64 |  | <p>The importance of this OpsItem in relation to other OpsItems in the system.</p> |
| `actual_start_time` | String |  | <p>The time a runbook workflow started. Currently reported only for the OpsItem type
    <code>/aws/changerequest</code>.</p> |
| `operational_data` | HashMap<String, String> |  | <p>Operational data is custom data that provides useful reference details about the OpsItem.
   For example, you can specify log files, error strings, license keys, troubleshooting tips, or
   other relevant data. You enter operational data as key-value pairs. The key has a maximum length
   of 128 characters. The value has a maximum size of 20 KB.</p>
         <important>
            <p>Operational data keys <i>can't</i> begin with the following:
     <code>amazon</code>, <code>aws</code>, <code>amzn</code>, <code>ssm</code>,
     <code>/amazon</code>, <code>/aws</code>, <code>/amzn</code>, <code>/ssm</code>.</p>
         </important>
         <p>You can choose to make the data searchable by other users in the account or you can restrict
   search access. Searchable data means that all users with access to the OpsItem Overview page (as
   provided by the <a>DescribeOpsItems</a> API operation) can view and search on the
   specified data. Operational data that isn't searchable is only viewable by users who have access
   to the OpsItem (as provided by the <a>GetOpsItem</a> API operation).</p>
         <p>Use the <code>/aws/resources</code> key in OperationalData to specify a related resource in
   the request. Use the <code>/aws/automations</code> key in OperationalData to associate an
   Automation runbook with the OpsItem. To view Amazon Web Services CLI example commands that use these keys, see
    <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-manually-create-OpsItems.html">Create OpsItems
    manually</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `source` | String | ✅ | <p>The origin of the OpsItem, such as Amazon EC2 or Systems Manager.</p>
         <note>
            <p>The source name can't contain the following strings: <code>aws</code>, <code>amazon</code>,
    and <code>amzn</code>. </p>
         </note> |
| `tags` | Vec<String> |  | <p>Optional metadata that you assign to a resource.</p>
         <p>Tags use a key-value pair. For example:</p>
         <p>
            <code>Key=Department,Value=Finance</code>
         </p>
         <important>
            <p>To add tags to a new OpsItem, a user must have IAM permissions for both the
     <code>ssm:CreateOpsItems</code> operation and the <code>ssm:AddTagsToResource</code> operation.
    To add tags to an existing OpsItem, use the <a>AddTagsToResource</a>
    operation.</p>
         </important> |
| `planned_end_time` | String |  | <p>The time specified in a change request for a runbook workflow to end. Currently supported
   only for the OpsItem type <code>/aws/changerequest</code>.</p> |
| `ops_item_type` | String |  | <p>The type of OpsItem to create. Systems Manager supports the following types of OpsItems:</p>
         <ul>
            <li>
               <p>
                  <code>/aws/issue</code>
               </p>
               <p>This type of OpsItem is used for default OpsItems created by OpsCenter. </p>
            </li>
            <li>
               <p>
                  <code>/aws/changerequest</code>
               </p>
               <p>This type of OpsItem is used by Change Manager for reviewing and approving or rejecting change
     requests. </p>
            </li>
            <li>
               <p>
                  <code>/aws/insight</code>
               </p>
               <p>This type of OpsItem is used by OpsCenter for aggregating and reporting on duplicate
     OpsItems. </p>
            </li>
         </ul> |
| `title` | String | ✅ | <p>A short heading that describes the nature of the OpsItem and the impacted resource.</p> |
| `severity` | String |  | <p>Specify a severity to assign to an OpsItem.</p> |
| `notifications` | Vec<String> |  | <p>The Amazon Resource Name (ARN) of an SNS topic where notifications are sent when this
   OpsItem is edited or changed.</p> |
| `category` | String |  | <p>Specify a category to assign to an OpsItem. </p> |
| `planned_start_time` | String |  | <p>The time specified in a change request for a runbook workflow to start. Currently supported
   only for the OpsItem type <code>/aws/changerequest</code>.</p> |
| `account_id` | String |  | <p>The target Amazon Web Services account where you want to create an OpsItem. To make this call, your account
   must be configured to work with OpsItems across accounts. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/OpsCenter-setup.html">Set up
    OpsCenter</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `related_ops_items` | Vec<String> |  | <p>One or more OpsItems that share something in common with the current OpsItems. For example,
   related OpsItems can include OpsItems with similar error messages, impacted resources, or
   statuses for the impacted resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ops_item` | String | <p>The OpsItem.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ops_item
ops_item = provider.ssm.Ops_item {
    description = "value"  # <p>User-defined text that contains information about the OpsItem, in Markdown format. </p>
         <note>
            <p>Provide enough information so that users viewing this OpsItem for the first time understand
    the issue. </p>
         </note>
    source = "value"  # <p>The origin of the OpsItem, such as Amazon EC2 or Systems Manager.</p>
         <note>
            <p>The source name can't contain the following strings: <code>aws</code>, <code>amazon</code>,
    and <code>amzn</code>. </p>
         </note>
    title = "value"  # <p>A short heading that describes the nature of the OpsItem and the impacted resource.</p>
}

# Access ops_item outputs
ops_item_id = ops_item.id
ops_item_ops_item = ops_item.ops_item
```

---


### Maintenance_window_executions

MaintenanceWindowExecutions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `window_executions` | Vec<String> | <p>Information about the maintenance window executions.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_executions outputs
maintenance_window_executions_id = maintenance_window_executions.id
maintenance_window_executions_window_executions = maintenance_window_executions.window_executions
maintenance_window_executions_next_token = maintenance_window_executions.next_token
```

---


### Activation

Activation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A user-defined description of the resource that you want to register with Systems Manager. </p>
         <important>
            <p>Don't enter personally identifiable information in this field.</p>
         </important> |
| `iam_role` | String | ✅ | <p>The name of the Identity and Access Management (IAM) role that you want to assign to
   the managed node. This IAM role must provide AssumeRole permissions for the
   Amazon Web Services Systems Manager service principal <code>ssm.amazonaws.com</code>. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/hybrid-multicloud-service-role.html">Create the IAM service role required for Systems Manager in a hybrid and multicloud
    environments</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <note>
            <p>You can't specify an IAM service-linked role for this parameter. You must
    create a unique role.</p>
         </note> |
| `registration_limit` | i64 |  | <p>Specify the maximum number of managed nodes you want to register. The default value is
    <code>1</code>.</p> |
| `registration_metadata` | Vec<String> |  | <p>Reserved for internal use.</p> |
| `expiration_date` | String |  | <p>The date by which this activation request should expire, in timestamp format, such as
   "2024-07-07T00:00:00". You can specify a date up to 30 days in advance. If you don't provide an
   expiration date, the activation code expires in 24 hours.</p> |
| `default_instance_name` | String |  | <p>The name of the registered, managed node as it will appear in the Amazon Web Services Systems Manager console or when
   you use the Amazon Web Services command line tools to list Systems Manager resources.</p>
         <important>
            <p>Don't enter personally identifiable information in this field.</p>
         </important> |
| `tags` | Vec<String> |  | <p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in
   different ways, such as by purpose, owner, or environment. For example, you might want to tag an
   activation to identify which servers or virtual machines (VMs) in your on-premises environment
   you intend to activate. In this case, you could specify the following key-value pairs:</p>
         <ul>
            <li>
               <p>
                  <code>Key=OS,Value=Windows</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=Environment,Value=Production</code>
               </p>
            </li>
         </ul>
         <important>
            <p>When you install SSM Agent on your on-premises servers and VMs, you specify an activation ID
    and code. When you specify the activation ID and code, tags assigned to the activation are
    automatically applied to the on-premises servers or VMs.</p>
         </important>
         <p>You can't add tags to or delete tags from an existing activation. You can tag your
   on-premises servers, edge devices, and VMs after they connect to Systems Manager for the first time and are
   assigned a managed node ID. This means they are listed in the Amazon Web Services Systems Manager console with an ID that
   is prefixed with "mi-". For information about how to add tags to your managed nodes, see <a>AddTagsToResource</a>. For information about how to remove tags from your managed nodes,
   see <a>RemoveTagsFromResource</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create activation
activation = provider.ssm.Activation {
    iam_role = "value"  # <p>The name of the Identity and Access Management (IAM) role that you want to assign to
   the managed node. This IAM role must provide AssumeRole permissions for the
   Amazon Web Services Systems Manager service principal <code>ssm.amazonaws.com</code>. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/hybrid-multicloud-service-role.html">Create the IAM service role required for Systems Manager in a hybrid and multicloud
    environments</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <note>
            <p>You can't specify an IAM service-linked role for this parameter. You must
    create a unique role.</p>
         </note>
}

```

---


### Effective_instance_associations

EffectiveInstanceAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>The associations for the requested managed node.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access effective_instance_associations outputs
effective_instance_associations_id = effective_instance_associations.id
effective_instance_associations_associations = effective_instance_associations.associations
effective_instance_associations_next_token = effective_instance_associations.next_token
```

---


### Execution_preview

ExecutionPreview resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_preview` | String |  |
| `ended_at` | String | <p>A UTC timestamp indicating when the execution preview operation ended.</p> |
| `status` | String | <p>The current status of the execution preview operation.</p> |
| `status_message` | String | <p>Supplemental information about the current status of the execution preview.</p> |
| `execution_preview_id` | String | <p>The generated ID for the existing execution preview.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access execution_preview outputs
execution_preview_id = execution_preview.id
execution_preview_execution_preview = execution_preview.execution_preview
execution_preview_ended_at = execution_preview.ended_at
execution_preview_status = execution_preview.status
execution_preview_status_message = execution_preview.status_message
execution_preview_execution_preview_id = execution_preview.execution_preview_id
```

---


### Resource_policies

ResourcePolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policies` | Vec<String> | <p>An array of the <code>Policy</code> object.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_policies outputs
resource_policies_id = resource_policies.id
resource_policies_policies = resource_policies.policies
resource_policies_next_token = resource_policies.next_token
```

---


### Association_batch

AssociationBatch resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> | ✅ | <p>One or more associations.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create association_batch
association_batch = provider.ssm.Association_batch {
    entries = "value"  # <p>One or more associations.</p>
}

```

---


### Instance_associations_status

InstanceAssociationsStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_association_status_infos` | Vec<String> | <p>Status information about the association.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_associations_status outputs
instance_associations_status_id = instance_associations_status.id
instance_associations_status_instance_association_status_infos = instance_associations_status.instance_association_status_infos
instance_associations_status_next_token = instance_associations_status.next_token
```

---


### Document

Document resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_type` | String |  | <p>Specify a target type to define the kinds of resources the document can run on. For example,
   to run a document on EC2 instances, specify the following value:
   <code>/AWS::EC2::Instance</code>. If you specify a value of '/' the document can run on all types
   of resources. If you don't specify a value, the document can't run on any resources. For a list
   of valid resource types, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Amazon Web Services resource and
    property types reference</a> in the <i>CloudFormation User Guide</i>. </p> |
| `name` | String | ✅ | <p>A name for the SSM document.</p>
         <important>
            <p>You can't use the following strings as document name prefixes. These are reserved by Amazon Web Services
    for use as document name prefixes:</p>
            <ul>
               <li>
                  <p>
                     <code>aws</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>amazon</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>amzn</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>AWSEC2</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>AWSConfigRemediation</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>AWSSupport</code>
                  </p>
               </li>
            </ul>
         </important> |
| `attachments` | Vec<String> |  | <p>A list of key-value pairs that describe attachments to a version of a document.</p> |
| `version_name` | String |  | <p>An optional field specifying the version of the artifact you are creating with the document.
   For example, <code>Release12.1</code>. This value is unique across all versions of a document,
   and can't be changed.</p> |
| `document_format` | String |  | <p>Specify the document format for the request. The document format can be JSON, YAML, or TEXT.
   JSON is the default format.</p> |
| `content` | String | ✅ | <p>The content for the new SSM document in JSON or YAML format. The content of the document
   must not exceed 64KB. This quota also includes the content specified for input parameters at
   runtime. We recommend storing the contents for your new document in an external JSON or YAML file
   and referencing the file in a command.</p>
         <p>For examples, see the following topics in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <ul>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/documents-using.html#create-ssm-console">Create an SSM
      document (console)</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/documents-using.html#create-ssm-document-cli">Create an
      SSM document (command line)</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/documents-using.html#create-ssm-document-api">Create an
      SSM document (API)</a>
               </p>
            </li>
         </ul> |
| `display_name` | String |  | <p>An optional field where you can specify a friendly name for the SSM document. This value can
   differ for each version of the document. You can update this value at a later time using the
    <a>UpdateDocument</a> operation.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in
   different ways, such as by purpose, owner, or environment. For example, you might want to tag an
   SSM document to identify the types of targets or the environment where it will run. In this case,
   you could specify the following key-value pairs:</p>
         <ul>
            <li>
               <p>
                  <code>Key=OS,Value=Windows</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=Environment,Value=Production</code>
               </p>
            </li>
         </ul>
         <note>
            <p>To add tags to an existing SSM document, use the <a>AddTagsToResource</a>
    operation.</p>
         </note> |
| `requires` | Vec<String> |  | <p>A list of SSM documents required by a document. This parameter is used exclusively by
   AppConfig. When a user creates an AppConfig configuration in an SSM document, the user must also
   specify a required document for validation purposes. In this case, an
    <code>ApplicationConfiguration</code> document requires an
    <code>ApplicationConfigurationSchema</code> document for validation purposes. For more
   information, see <a href="https://docs.aws.amazon.com/appconfig/latest/userguide/what-is-appconfig.html">What is AppConfig?</a> in the
    <i>AppConfig User Guide</i>.</p> |
| `document_type` | String |  | <p>The type of document to create.</p>
         <note>
            <p>The <code>DeploymentStrategy</code> document type is an internal-use-only document type
    reserved for AppConfig.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_format` | String | <p>The document format, either JSON or YAML.</p> |
| `display_name` | String | <p>The friendly name of the SSM document. This value can differ for each version of the
   document. If you want to update this value, see <a>UpdateDocument</a>.</p> |
| `status_information` | String | <p>A message returned by Amazon Web Services Systems Manager that explains the <code>Status</code> value. For example, a
    <code>Failed</code> status might be explained by the <code>StatusInformation</code> message,
   "The specified S3 bucket doesn't exist. Verify that the URL of the S3 bucket is correct."</p> |
| `created_date` | String | <p>The date the SSM document was created.</p> |
| `status` | String | <p>The status of the SSM document, such as <code>Creating</code>, <code>Active</code>,
    <code>Updating</code>, <code>Failed</code>, and <code>Deleting</code>.</p> |
| `document_version` | String | <p>The document version.</p> |
| `review_status` | String | <p>The current review status of a new custom Systems Manager document (SSM document) created by a member
   of your organization, or of the latest version of an existing SSM document.</p>
         <p>Only one version of an SSM document can be in the APPROVED state at a time. When a new
   version is approved, the status of the previous version changes to REJECTED.</p>
         <p>Only one version of an SSM document can be in review, or PENDING, at a time.</p> |
| `version_name` | String | <p>The version of the artifact associated with the document. For example, 12.6. This value is
   unique across all versions of a document, and can't be changed.</p> |
| `name` | String | <p>The name of the SSM document.</p> |
| `document_type` | String | <p>The document type.</p> |
| `requires` | Vec<String> | <p>A list of SSM documents required by a document. For example, an
    <code>ApplicationConfiguration</code> document requires an
    <code>ApplicationConfigurationSchema</code> document.</p> |
| `attachments_content` | Vec<String> | <p>A description of the document attachments, including names, locations, sizes, and so
   on.</p> |
| `content` | String | <p>The contents of the SSM document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create document
document = provider.ssm.Document {
    name = "value"  # <p>A name for the SSM document.</p>
         <important>
            <p>You can't use the following strings as document name prefixes. These are reserved by Amazon Web Services
    for use as document name prefixes:</p>
            <ul>
               <li>
                  <p>
                     <code>aws</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>amazon</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>amzn</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>AWSEC2</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>AWSConfigRemediation</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>AWSSupport</code>
                  </p>
               </li>
            </ul>
         </important>
    content = "value"  # <p>The content for the new SSM document in JSON or YAML format. The content of the document
   must not exceed 64KB. This quota also includes the content specified for input parameters at
   runtime. We recommend storing the contents for your new document in an external JSON or YAML file
   and referencing the file in a command.</p>
         <p>For examples, see the following topics in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <ul>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/documents-using.html#create-ssm-console">Create an SSM
      document (console)</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/documents-using.html#create-ssm-document-cli">Create an
      SSM document (command line)</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/documents-using.html#create-ssm-document-api">Create an
      SSM document (API)</a>
               </p>
            </li>
         </ul>
}

# Access document outputs
document_id = document.id
document_document_format = document.document_format
document_display_name = document.display_name
document_status_information = document.status_information
document_created_date = document.created_date
document_status = document.status
document_document_version = document.document_version
document_review_status = document.review_status
document_version_name = document.version_name
document_name = document.name
document_document_type = document.document_type
document_requires = document.requires
document_attachments_content = document.attachments_content
document_content = document.content
```

---


### Ops_summary

OpsSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |
| `entities` | Vec<String> | <p>The list of aggregated details and filtered OpsData.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ops_summary outputs
ops_summary_id = ops_summary.id
ops_summary_next_token = ops_summary.next_token
ops_summary_entities = ops_summary.entities
```

---


### Command_invocation

CommandInvocation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comment` | String | <p>The comment text for the command.</p> |
| `response_code` | i64 | <p>The error level response code for the plugin script. If the response code is
   <code>-1</code>, then the command hasn't started running on the managed node, or it wasn't
   received by the node.</p> |
| `execution_elapsed_time` | String | <p>Duration since <code>ExecutionStartDateTime</code>.</p> |
| `execution_end_date_time` | String | <p>The date and time the plugin finished running. Date and time are written in ISO 8601 format.
   For example, June 7, 2017 is represented as 2017-06-7. The following sample Amazon Web Services CLI command uses
   the <code>InvokedAfter</code> filter.</p>
         <p>
            <code>aws ssm list-commands --filters key=InvokedAfter,value=2017-06-07T00:00:00Z</code>
         </p>
         <p>If the plugin hasn't started to run, the string is empty.</p> |
| `standard_output_url` | String | <p>The URL for the complete text written by the plugin to <code>stdout</code> in Amazon Simple Storage Service (Amazon S3). If an S3 bucket wasn't specified, then this string is
   empty.</p> |
| `document_name` | String | <p>The name of the document that was run. For example, <code>AWS-RunShellScript</code>.</p> |
| `plugin_name` | String | <p>The name of the plugin, or <i>step name</i>, for which details are reported.
   For example, <code>aws:RunShellScript</code> is a plugin.</p> |
| `execution_start_date_time` | String | <p>The date and time the plugin started running. Date and time are written in ISO 8601 format.
   For example, June 7, 2017 is represented as 2017-06-7. The following sample Amazon Web Services CLI command uses
   the <code>InvokedBefore</code> filter.</p>
         <p>
            <code>aws ssm list-commands --filters key=InvokedBefore,value=2017-06-07T00:00:00Z</code>
         </p>
         <p>If the plugin hasn't started to run, the string is empty.</p> |
| `status_details` | String | <p>A detailed status of the command execution for an invocation. <code>StatusDetails</code>
   includes more information than <code>Status</code> because it includes states resulting from
   error and concurrency control parameters. <code>StatusDetails</code> can show different results
   than <code>Status</code>. For more information about these statuses, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-commands.html">Understanding
    command statuses</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.
    <code>StatusDetails</code> can be one of the following values:</p>
         <ul>
            <li>
               <p>Pending: The command hasn't been sent to the managed node.</p>
            </li>
            <li>
               <p>In Progress: The command has been sent to the managed node but hasn't reached a terminal
     state.</p>
            </li>
            <li>
               <p>Delayed: The system attempted to send the command to the target, but the target wasn't
     available. The managed node might not be available because of network issues, because the node
     was stopped, or for similar reasons. The system will try to send the command again.</p>
            </li>
            <li>
               <p>Success: The command or plugin ran successfully. This is a terminal state.</p>
            </li>
            <li>
               <p>Delivery Timed Out: The command wasn't delivered to the managed node before the delivery
     timeout expired. Delivery timeouts don't count against the parent command's
      <code>MaxErrors</code> limit, but they do contribute to whether the parent command status is
     Success or Incomplete. This is a terminal state.</p>
            </li>
            <li>
               <p>Execution Timed Out: The command started to run on the managed node, but the execution
     wasn't complete before the timeout expired. Execution timeouts count against the
      <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p>
            </li>
            <li>
               <p>Failed: The command wasn't run successfully on the managed node. For a plugin, this
     indicates that the result code wasn't zero. For a command invocation, this indicates that the
     result code for one or more plugins wasn't zero. Invocation failures count against the
      <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p>
            </li>
            <li>
               <p>Cancelled: The command was terminated before it was completed. This is a terminal
     state.</p>
            </li>
            <li>
               <p>Undeliverable: The command can't be delivered to the managed node. The node might not
     exist or might not be responding. Undeliverable invocations don't count against the parent
     command's <code>MaxErrors</code> limit and don't contribute to whether the parent command
     status is Success or Incomplete. This is a terminal state.</p>
            </li>
            <li>
               <p>Terminated: The parent command exceeded its <code>MaxErrors</code> limit and subsequent
     command invocations were canceled by the system. This is a terminal state.</p>
            </li>
         </ul> |
| `standard_error_content` | String | <p>The first 8,000 characters written by the plugin to <code>stderr</code>. If the command
   hasn't finished running, then this string is empty.</p> |
| `standard_error_url` | String | <p>The URL for the complete text written by the plugin to <code>stderr</code>. If the command
   hasn't finished running, then this string is empty.</p> |
| `command_id` | String | <p>The parent command ID of the invocation plugin.</p> |
| `document_version` | String | <p>The Systems Manager document (SSM document) version used in the request.</p> |
| `cloud_watch_output_config` | String | <p>Amazon CloudWatch Logs information where Systems Manager sent the command output.</p> |
| `standard_output_content` | String | <p>The first 24,000 characters written by the plugin to <code>stdout</code>. If the command
   hasn't finished running, if <code>ExecutionStatus</code> is neither Succeeded nor Failed, then
   this string is empty.</p> |
| `status` | String | <p>The status of this invocation plugin. This status can be different than
    <code>StatusDetails</code>.</p> |
| `instance_id` | String | <p>The ID of the managed node targeted by the command. A <i>managed node</i> can
   be an Amazon Elastic Compute Cloud (Amazon EC2) instance, edge device, or on-premises server or VM in your hybrid
   environment that is configured for Amazon Web Services Systems Manager.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access command_invocation outputs
command_invocation_id = command_invocation.id
command_invocation_comment = command_invocation.comment
command_invocation_response_code = command_invocation.response_code
command_invocation_execution_elapsed_time = command_invocation.execution_elapsed_time
command_invocation_execution_end_date_time = command_invocation.execution_end_date_time
command_invocation_standard_output_url = command_invocation.standard_output_url
command_invocation_document_name = command_invocation.document_name
command_invocation_plugin_name = command_invocation.plugin_name
command_invocation_execution_start_date_time = command_invocation.execution_start_date_time
command_invocation_status_details = command_invocation.status_details
command_invocation_standard_error_content = command_invocation.standard_error_content
command_invocation_standard_error_url = command_invocation.standard_error_url
command_invocation_command_id = command_invocation.command_id
command_invocation_document_version = command_invocation.document_version
command_invocation_cloud_watch_output_config = command_invocation.cloud_watch_output_config
command_invocation_standard_output_content = command_invocation.standard_output_content
command_invocation_status = command_invocation.status
command_invocation_instance_id = command_invocation.instance_id
```

---


### Compliance_items

ComplianceItems resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `compliance_type` | String | ✅ | <p>Specify the compliance type. For example, specify Association (for a State Manager
   association), Patch, or Custom:<code>string</code>.</p> |
| `items` | Vec<String> | ✅ | <p>Information about the compliance as defined by the resource type. For example, for a patch
   compliance type, <code>Items</code> includes information about the PatchSeverity, Classification,
   and so on.</p> |
| `upload_type` | String |  | <p>The mode for uploading compliance items. You can specify <code>COMPLETE</code> or
    <code>PARTIAL</code>. In <code>COMPLETE</code> mode, the system overwrites all existing
   compliance information for the resource. You must provide a full list of compliance items each
   time you send the request.</p>
         <p>In <code>PARTIAL</code> mode, the system overwrites compliance information for a specific
   association. The association must be configured with <code>SyncCompliance</code> set to
    <code>MANUAL</code>. By default, all requests use <code>COMPLETE</code> mode.</p>
         <note>
            <p>This attribute is only valid for association compliance.</p>
         </note> |
| `resource_type` | String | ✅ | <p>Specify the type of resource. <code>ManagedInstance</code> is currently the only supported
   resource type.</p> |
| `resource_id` | String | ✅ | <p>Specify an ID for this resource. For a managed node, this is the node ID.</p> |
| `execution_summary` | String | ✅ | <p>A summary of the call execution that includes an execution ID, the type of execution (for
   example, <code>Command</code>), and the date/time of the execution using a datetime object that
   is saved in the following format: <code>yyyy-MM-dd'T'HH:mm:ss'Z'</code>
         </p> |
| `item_content_hash` | String |  | <p>MD5 or SHA-256 content hash. The content hash is used to determine if existing information
   should be overwritten or ignored. If the content hashes match, the request to put compliance
   information is ignored.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create compliance_items
compliance_items = provider.ssm.Compliance_items {
    compliance_type = "value"  # <p>Specify the compliance type. For example, specify Association (for a State Manager
   association), Patch, or Custom:<code>string</code>.</p>
    items = "value"  # <p>Information about the compliance as defined by the resource type. For example, for a patch
   compliance type, <code>Items</code> includes information about the PatchSeverity, Classification,
   and so on.</p>
    resource_type = "value"  # <p>Specify the type of resource. <code>ManagedInstance</code> is currently the only supported
   resource type.</p>
    resource_id = "value"  # <p>Specify an ID for this resource. For a managed node, this is the node ID.</p>
    execution_summary = "value"  # <p>A summary of the call execution that includes an execution ID, the type of execution (for
   example, <code>Command</code>), and the date/time of the execution using a datetime object that
   is saved in the following format: <code>yyyy-MM-dd'T'HH:mm:ss'Z'</code>
         </p>
}

```

---


### Patch_baselines

PatchBaselines resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `baseline_identities` | Vec<String> | <p>An array of <code>PatchBaselineIdentity</code> elements.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access patch_baselines outputs
patch_baselines_id = patch_baselines.id
patch_baselines_baseline_identities = patch_baselines.baseline_identities
patch_baselines_next_token = patch_baselines.next_token
```

---


### Access_token

AccessToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credentials` | String | <p>The temporary security credentials which can be used to start just-in-time node access
   sessions.</p> |
| `access_request_status` | String | <p>The status of the access request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_token outputs
access_token_id = access_token.id
access_token_credentials = access_token.credentials
access_token_access_request_status = access_token.access_request_status
```

---


### Maintenance_window_tasks

MaintenanceWindowTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tasks` | Vec<String> | <p>Information about the tasks in the maintenance window.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_tasks outputs
maintenance_window_tasks_id = maintenance_window_tasks.id
maintenance_window_tasks_tasks = maintenance_window_tasks.tasks
maintenance_window_tasks_next_token = maintenance_window_tasks.next_token
```

---


### Maintenance_window_execution_tasks

MaintenanceWindowExecutionTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `window_execution_task_identities` | Vec<String> | <p>Information about the task executions.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_execution_tasks outputs
maintenance_window_execution_tasks_id = maintenance_window_execution_tasks.id
maintenance_window_execution_tasks_window_execution_task_identities = maintenance_window_execution_tasks.window_execution_task_identities
maintenance_window_execution_tasks_next_token = maintenance_window_execution_tasks.next_token
```

---


### Deployable_patch_snapshot_for_instance

DeployablePatchSnapshotForInstance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_id` | String | <p>The managed node ID.</p> |
| `snapshot_id` | String | <p>The user-defined snapshot ID.</p> |
| `product` | String | <p>Returns the specific operating system (for example Windows Server 2012 or Amazon Linux
   2015.09) on the managed node for the specified patch snapshot.</p> |
| `snapshot_download_url` | String | <p>A pre-signed Amazon Simple Storage Service (Amazon S3) URL that can be used to download the
   patch snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deployable_patch_snapshot_for_instance outputs
deployable_patch_snapshot_for_instance_id = deployable_patch_snapshot_for_instance.id
deployable_patch_snapshot_for_instance_instance_id = deployable_patch_snapshot_for_instance.instance_id
deployable_patch_snapshot_for_instance_snapshot_id = deployable_patch_snapshot_for_instance.snapshot_id
deployable_patch_snapshot_for_instance_product = deployable_patch_snapshot_for_instance.product
deployable_patch_snapshot_for_instance_snapshot_download_url = deployable_patch_snapshot_for_instance.snapshot_download_url
```

---


### Maintenance_window_schedule

MaintenanceWindowSchedule resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scheduled_window_executions` | Vec<String> | <p>Information about maintenance window executions scheduled for the specified time
   range.</p> |
| `next_token` | String | <p>The token for the next set of items to return. (You use this token in the next call.)</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_schedule outputs
maintenance_window_schedule_id = maintenance_window_schedule.id
maintenance_window_schedule_scheduled_window_executions = maintenance_window_schedule.scheduled_window_executions
maintenance_window_schedule_next_token = maintenance_window_schedule.next_token
```

---


### Document_metadata

DocumentMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document_version` | String |  | <p>The version of a change template in which to update approval metadata.</p> |
| `document_reviews` | String | ✅ | <p>The change template review details to update.</p> |
| `name` | String | ✅ | <p>The name of the change template for which a version's metadata is to be updated.</p> |



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


### Association_status

AssociationStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `association_status` | String | ✅ | <p>The association status.</p> |
| `name` | String | ✅ | <p>The name of the SSM document.</p> |
| `instance_id` | String | ✅ | <p>The managed node ID.</p> |



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


### Maintenance_window_execution_task_invocations

MaintenanceWindowExecutionTaskInvocations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `window_execution_task_invocation_identities` | Vec<String> | <p>Information about the task invocation results per invocation.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_execution_task_invocations outputs
maintenance_window_execution_task_invocations_id = maintenance_window_execution_task_invocations.id
maintenance_window_execution_task_invocations_window_execution_task_invocation_identities = maintenance_window_execution_task_invocations.window_execution_task_invocation_identities
maintenance_window_execution_task_invocations_next_token = maintenance_window_execution_task_invocations.next_token
```

---


### Instance_patch_states_for_patch_group

InstancePatchStatesForPatchGroup resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |
| `instance_patch_states` | Vec<String> | <p>The high-level patch state for the requested managed nodes. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_patch_states_for_patch_group outputs
instance_patch_states_for_patch_group_id = instance_patch_states_for_patch_group.id
instance_patch_states_for_patch_group_next_token = instance_patch_states_for_patch_group.next_token
instance_patch_states_for_patch_group_instance_patch_states = instance_patch_states_for_patch_group.instance_patch_states
```

---


### Patch_groups

PatchGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |
| `mappings` | Vec<String> | <p>Each entry in the array contains:</p>
         <ul>
            <li>
               <p>
                  <code>PatchGroup</code>: string (between 1 and 256 characters. Regex:
      <code>^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$)</code>
               </p>
            </li>
            <li>
               <p>
                  <code>PatchBaselineIdentity</code>: A <code>PatchBaselineIdentity</code> element.</p>
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

# Access patch_groups outputs
patch_groups_id = patch_groups.id
patch_groups_next_token = patch_groups.next_token
patch_groups_mappings = patch_groups.mappings
```

---


### Automation_step_executions

AutomationStepExecutions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `step_executions` | Vec<String> | <p>A list of details about the current state of all steps that make up an execution.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access automation_step_executions outputs
automation_step_executions_id = automation_step_executions.id
automation_step_executions_step_executions = automation_step_executions.step_executions
automation_step_executions_next_token = automation_step_executions.next_token
```

---


### Patch_baseline_for_patch_group

PatchBaselineForPatchGroup resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `patch_group` | String | <p>The name of the patch group.</p> |
| `operating_system` | String | <p>The operating system rule specified for patch groups using the patch baseline.</p> |
| `baseline_id` | String | <p>The ID of the patch baseline that should be used for the patch group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access patch_baseline_for_patch_group outputs
patch_baseline_for_patch_group_id = patch_baseline_for_patch_group.id
patch_baseline_for_patch_group_patch_group = patch_baseline_for_patch_group.patch_group
patch_baseline_for_patch_group_operating_system = patch_baseline_for_patch_group.operating_system
patch_baseline_for_patch_group_baseline_id = patch_baseline_for_patch_group.baseline_id
```

---


### Ops_metadata

OpsMetadata resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_id` | String | ✅ | <p>A resource ID for a new Application Manager application.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata that you assign to a resource. You can specify a maximum of five tags for
   an OpsMetadata object. Tags enable you to categorize a resource in different ways, such as by
   purpose, owner, or environment. For example, you might want to tag an OpsMetadata object to
   identify an environment or target Amazon Web Services Region. In this case, you could specify the following
   key-value pairs:</p>
         <ul>
            <li>
               <p>
                  <code>Key=Environment,Value=Production</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=Region,Value=us-east-2</code>
               </p>
            </li>
         </ul> |
| `metadata` | HashMap<String, String> |  | <p>Metadata for a new Application Manager application. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | <p>OpsMetadata for an Application Manager application.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |
| `resource_id` | String | <p>The resource ID of the Application Manager application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ops_metadata
ops_metadata = provider.ssm.Ops_metadata {
    resource_id = "value"  # <p>A resource ID for a new Application Manager application.</p>
}

# Access ops_metadata outputs
ops_metadata_id = ops_metadata.id
ops_metadata_metadata = ops_metadata.metadata
ops_metadata_next_token = ops_metadata.next_token
ops_metadata_resource_id = ops_metadata.resource_id
```

---


### Inventory

Inventory resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `items` | Vec<String> | ✅ | <p>The inventory items that you want to add or update on managed nodes.</p> |
| `instance_id` | String | ✅ | <p>An managed node ID where you want to add or update inventory items.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entities` | Vec<String> | <p>Collection of inventory entities such as a collection of managed node inventory. </p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create inventory
inventory = provider.ssm.Inventory {
    items = "value"  # <p>The inventory items that you want to add or update on managed nodes.</p>
    instance_id = "value"  # <p>An managed node ID where you want to add or update inventory items.</p>
}

# Access inventory outputs
inventory_id = inventory.id
inventory_entities = inventory.entities
inventory_next_token = inventory.next_token
```

---


### Patch_properties

PatchProperties resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `properties` | Vec<HashMap<String, String>> | <p>A list of the properties for patches matching the filter request parameters.</p> |
| `next_token` | String | <p>The token for the next set of items to return. (You use this token in the next call.)</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access patch_properties outputs
patch_properties_id = patch_properties.id
patch_properties_properties = patch_properties.properties
patch_properties_next_token = patch_properties.next_token
```

---


### Instance_patch_states

InstancePatchStates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |
| `instance_patch_states` | Vec<String> | <p>The high-level patch state for the requested managed nodes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_patch_states outputs
instance_patch_states_id = instance_patch_states.id
instance_patch_states_next_token = instance_patch_states.next_token
instance_patch_states_instance_patch_states = instance_patch_states.instance_patch_states
```

---


### Instance_information

InstanceInformation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty. </p> |
| `instance_information_list` | Vec<String> | <p>The managed node information list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_information outputs
instance_information_id = instance_information.id
instance_information_next_token = instance_information.next_token
instance_information_instance_information_list = instance_information.instance_information_list
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_hash` | String |  | <p>ID of the current policy version. The hash helps to prevent a situation where multiple users
   attempt to overwrite a policy. You must provide this hash when updating or deleting a
   policy.</p> |
| `policy_id` | String |  | <p>The policy ID.</p> |
| `policy` | String | ✅ | <p>A policy you want to associate with a resource.</p> |
| `resource_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the resource to which you want to attach a policy.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.ssm.Resource_policy {
    policy = "value"  # <p>A policy you want to associate with a resource.</p>
    resource_arn = "value"  # <p>Amazon Resource Name (ARN) of the resource to which you want to attach a policy.</p>
}

```

---


### Activations

Activations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activation_list` | Vec<String> | <p>A list of activations for your Amazon Web Services account.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access activations outputs
activations_id = activations.id
activations_activation_list = activations.activation_list
activations_next_token = activations.next_token
```

---


### Maintenance_window_execution_task_invocation

MaintenanceWindowExecutionTaskInvocation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invocation_id` | String | <p>The invocation ID.</p> |
| `window_execution_id` | String | <p>The maintenance window execution ID.</p> |
| `status` | String | <p>The task status for an invocation.</p> |
| `status_details` | String | <p>The details explaining the status. Details are only available for certain status
   values.</p> |
| `window_target_id` | String | <p>The maintenance window target ID.</p> |
| `owner_information` | String | <p>User-provided value to be included in any Amazon CloudWatch Events or Amazon EventBridge
   events raised while running tasks for these targets in this maintenance window.</p> |
| `task_type` | String | <p>Retrieves the task type for a maintenance window.</p> |
| `end_time` | String | <p>The time that the task finished running on the target.</p> |
| `task_execution_id` | String | <p>The task execution ID.</p> |
| `parameters` | String | <p>The parameters used at the time that the task ran.</p> |
| `execution_id` | String | <p>The execution ID.</p> |
| `start_time` | String | <p>The time that the task started running on the target.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_execution_task_invocation outputs
maintenance_window_execution_task_invocation_id = maintenance_window_execution_task_invocation.id
maintenance_window_execution_task_invocation_invocation_id = maintenance_window_execution_task_invocation.invocation_id
maintenance_window_execution_task_invocation_window_execution_id = maintenance_window_execution_task_invocation.window_execution_id
maintenance_window_execution_task_invocation_status = maintenance_window_execution_task_invocation.status
maintenance_window_execution_task_invocation_status_details = maintenance_window_execution_task_invocation.status_details
maintenance_window_execution_task_invocation_window_target_id = maintenance_window_execution_task_invocation.window_target_id
maintenance_window_execution_task_invocation_owner_information = maintenance_window_execution_task_invocation.owner_information
maintenance_window_execution_task_invocation_task_type = maintenance_window_execution_task_invocation.task_type
maintenance_window_execution_task_invocation_end_time = maintenance_window_execution_task_invocation.end_time
maintenance_window_execution_task_invocation_task_execution_id = maintenance_window_execution_task_invocation.task_execution_id
maintenance_window_execution_task_invocation_parameters = maintenance_window_execution_task_invocation.parameters
maintenance_window_execution_task_invocation_execution_id = maintenance_window_execution_task_invocation.execution_id
maintenance_window_execution_task_invocation_start_time = maintenance_window_execution_task_invocation.start_time
```

---


### Maintenance_windows_for_target

MaintenanceWindowsForTarget resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next set of items to return. (You use this token in the next call.)</p> |
| `window_identities` | Vec<String> | <p>Information about the maintenance window targets and tasks a managed node is associated
   with.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_windows_for_target outputs
maintenance_windows_for_target_id = maintenance_windows_for_target.id
maintenance_windows_for_target_next_token = maintenance_windows_for_target.next_token
maintenance_windows_for_target_window_identities = maintenance_windows_for_target.window_identities
```

---


### Service_setting

ServiceSetting resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `setting_id` | String | ✅ | <p>The Amazon Resource Name (ARN) of the service setting to update. For example,
    <code>arn:aws:ssm:us-east-1:111122223333:servicesetting/ssm/parameter-store/high-throughput-enabled</code>.
   The setting ID can be one of the following.</p>
         <ul>
            <li>
               <p>
                  <code>/ssm/appmanager/appmanager-enabled</code>
               </p>
            </li>
            <li>
               <p>
                  <code>/ssm/automation/customer-script-log-destination</code>
               </p>
            </li>
            <li>
               <p>
                  <code>/ssm/automation/customer-script-log-group-name</code>
               </p>
            </li>
            <li>
               <p>/ssm/automation/enable-adaptive-concurrency</p>
            </li>
            <li>
               <p>
                  <code>/ssm/documents/console/public-sharing-permission</code>
               </p>
            </li>
            <li>
               <p>
                  <code>/ssm/managed-instance/activation-tier</code>
               </p>
            </li>
            <li>
               <p>
                  <code>/ssm/managed-instance/default-ec2-instance-management-role</code>
               </p>
            </li>
            <li>
               <p>
                  <code>/ssm/opsinsights/opscenter</code>
               </p>
            </li>
            <li>
               <p>
                  <code>/ssm/parameter-store/default-parameter-tier</code>
               </p>
            </li>
            <li>
               <p>
                  <code>/ssm/parameter-store/high-throughput-enabled</code>
               </p>
            </li>
         </ul>
         <note>
            <p>Permissions to update the
     <code>/ssm/managed-instance/default-ec2-instance-management-role</code> setting should only be
    provided to administrators. Implement least privilege access when allowing individuals to
    configure or modify the Default Host Management Configuration.</p>
         </note> |
| `setting_value` | String | ✅ | <p>The new value to specify for the service setting. The following list specifies the available
   values for each setting.</p>
         <ul>
            <li>
               <p>For <code>/ssm/appmanager/appmanager-enabled</code>, enter <code>True</code> or
      <code>False</code>.</p>
            </li>
            <li>
               <p>For <code>/ssm/automation/customer-script-log-destination</code>, enter <code>CloudWatch</code>.</p>
            </li>
            <li>
               <p>For <code>/ssm/automation/customer-script-log-group-name</code>, enter the name of an
      Amazon CloudWatch Logs log group.</p>
            </li>
            <li>
               <p>For <code>/ssm/documents/console/public-sharing-permission</code>, enter
      <code>Enable</code> or <code>Disable</code>.</p>
            </li>
            <li>
               <p>For <code>/ssm/managed-instance/activation-tier</code>, enter <code>standard</code> or
      <code>advanced</code>.</p>
            </li>
            <li>
               <p>For <code>/ssm/managed-instance/default-ec2-instance-management-role</code>, enter the
     name of an IAM role. </p>
            </li>
            <li>
               <p> For <code>/ssm/opsinsights/opscenter</code>, enter <code>Enabled</code> or
      <code>Disabled</code>. </p>
            </li>
            <li>
               <p>For <code>/ssm/parameter-store/default-parameter-tier</code>, enter <code>Standard</code>,
      <code>Advanced</code>, or <code>Intelligent-Tiering</code>
               </p>
            </li>
            <li>
               <p>For <code>/ssm/parameter-store/high-throughput-enabled</code>, enter <code>true</code> or
      <code>false</code>.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_setting` | String | <p>The query result of the current service setting.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_setting outputs
service_setting_id = service_setting.id
service_setting_service_setting = service_setting.service_setting
```

---


### Automation_execution

AutomationExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `automation_execution` | String | <p>Detailed information about the current state of an automation execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access automation_execution outputs
automation_execution_id = automation_execution.id
automation_execution_automation_execution = automation_execution.automation_execution
```

---


### Patch_group_state

PatchGroupState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instances_with_installed_rejected_patches` | i64 | <p>The number of managed nodes with patches installed that are specified in a
    <code>RejectedPatches</code> list. Patches with a status of <code>INSTALLED_REJECTED</code> were
   typically installed before they were added to a <code>RejectedPatches</code> list.</p>
         <note>
            <p>If <code>ALLOW_AS_DEPENDENCY</code> is the specified option for
     <code>RejectedPatchesAction</code>, the value of
     <code>InstancesWithInstalledRejectedPatches</code> will always be <code>0</code> (zero).</p>
         </note> |
| `instances_with_installed_other_patches` | i64 | <p>The number of managed nodes with patches installed that aren't defined in the patch
   baseline.</p> |
| `instances` | i64 | <p>The number of managed nodes in the patch group.</p> |
| `instances_with_installed_pending_reboot_patches` | i64 | <p>The number of managed nodes with patches installed by Patch Manager that haven't been
   rebooted after the patch installation. The status of these managed nodes is
    <code>NON_COMPLIANT</code>.</p> |
| `instances_with_missing_patches` | i64 | <p>The number of managed nodes with missing patches from the patch baseline.</p> |
| `instances_with_security_non_compliant_patches` | i64 | <p>The number of managed nodes where patches that are specified as <code>Security</code> in a
   patch advisory aren't installed. These patches might be missing, have failed installation, were
   rejected, or were installed but awaiting a required managed node reboot. The status of these
   managed nodes is <code>NON_COMPLIANT</code>.</p> |
| `instances_with_available_security_updates` | i64 | <p>The number of managed nodes for which security-related patches are available but not
   approved because because they didn't meet the patch baseline requirements. For example, an
   updated version of a patch might have been released before the specified auto-approval period was
   over.</p>
         <p>Applies to Windows Server managed nodes only.</p> |
| `instances_with_failed_patches` | i64 | <p>The number of managed nodes with patches from the patch baseline that failed to
   install.</p> |
| `instances_with_unreported_not_applicable_patches` | i64 | <p>The number of managed nodes with <code>NotApplicable</code> patches beyond the supported
   limit, which aren't reported by name to Inventory. Inventory is a tool in Amazon Web Services Systems Manager.</p> |
| `instances_with_installed_patches` | i64 | <p>The number of managed nodes with installed patches.</p> |
| `instances_with_other_non_compliant_patches` | i64 | <p>The number of managed nodes with patches installed that are specified as other than
    <code>Critical</code> or <code>Security</code> but aren't compliant with the patch baseline. The
   status of these managed nodes is <code>NON_COMPLIANT</code>.</p> |
| `instances_with_not_applicable_patches` | i64 | <p>The number of managed nodes with patches that aren't applicable.</p> |
| `instances_with_critical_non_compliant_patches` | i64 | <p>The number of managed nodes where patches that are specified as <code>Critical</code> for
   compliance reporting in the patch baseline aren't installed. These patches might be missing, have
   failed installation, were rejected, or were installed but awaiting a required managed node
   reboot. The status of these managed nodes is <code>NON_COMPLIANT</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access patch_group_state outputs
patch_group_state_id = patch_group_state.id
patch_group_state_instances_with_installed_rejected_patches = patch_group_state.instances_with_installed_rejected_patches
patch_group_state_instances_with_installed_other_patches = patch_group_state.instances_with_installed_other_patches
patch_group_state_instances = patch_group_state.instances
patch_group_state_instances_with_installed_pending_reboot_patches = patch_group_state.instances_with_installed_pending_reboot_patches
patch_group_state_instances_with_missing_patches = patch_group_state.instances_with_missing_patches
patch_group_state_instances_with_security_non_compliant_patches = patch_group_state.instances_with_security_non_compliant_patches
patch_group_state_instances_with_available_security_updates = patch_group_state.instances_with_available_security_updates
patch_group_state_instances_with_failed_patches = patch_group_state.instances_with_failed_patches
patch_group_state_instances_with_unreported_not_applicable_patches = patch_group_state.instances_with_unreported_not_applicable_patches
patch_group_state_instances_with_installed_patches = patch_group_state.instances_with_installed_patches
patch_group_state_instances_with_other_non_compliant_patches = patch_group_state.instances_with_other_non_compliant_patches
patch_group_state_instances_with_not_applicable_patches = patch_group_state.instances_with_not_applicable_patches
patch_group_state_instances_with_critical_non_compliant_patches = patch_group_state.instances_with_critical_non_compliant_patches
```

---


### Maintenance_window

MaintenanceWindow resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_date` | String |  | <p>The date and time, in ISO-8601 Extended format, for when you want the maintenance window to
   become inactive. <code>EndDate</code> allows you to set a date and time in the future when the
   maintenance window will no longer run.</p> |
| `client_token` | String |  | <p>User-provided idempotency token.</p> |
| `schedule` | String | ✅ | <p>The schedule of the maintenance window in the form of a cron or rate expression.</p> |
| `name` | String | ✅ | <p>The name of the maintenance window.</p> |
| `start_date` | String |  | <p>The date and time, in ISO-8601 Extended format, for when you want the maintenance window to
   become active. <code>StartDate</code> allows you to delay activation of the maintenance window
   until the specified future date.</p>
         <note>
            <p>When using a rate schedule, if you provide a start date that occurs in the past, the
    current date and time are used as the start date. </p>
         </note> |
| `allow_unassociated_targets` | bool | ✅ | <p>Enables a maintenance window task to run on managed nodes, even if you haven't registered
   those nodes as targets. If enabled, then you must specify the unregistered managed nodes (by node
   ID) when you register a task with the maintenance window.</p>
         <p>If you don't enable this option, then you must specify previously-registered targets when
   you register a task with the maintenance window.</p> |
| `schedule_timezone` | String |  | <p>The time zone that the scheduled maintenance window executions are based on, in Internet
   Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "UTC", or
   "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time
    Zone Database</a> on the IANA website.</p> |
| `description` | String |  | <p>An optional description for the maintenance window. We recommend specifying a description to
   help you organize your maintenance windows. </p> |
| `schedule_offset` | i64 |  | <p>The number of days to wait after the date and time specified by a cron expression before
   running the maintenance window.</p>
         <p>For example, the following cron expression schedules a maintenance window to run on the
   third Tuesday of every month at 11:30 PM.</p>
         <p>
            <code>cron(30 23 ? * TUE#3 *)</code>
         </p>
         <p>If the schedule offset is <code>2</code>, the maintenance window won't run until two days
   later.</p> |
| `cutoff` | i64 | ✅ | <p>The number of hours before the end of the maintenance window that Amazon Web Services Systems Manager stops scheduling
   new tasks for execution.</p> |
| `duration` | i64 | ✅ | <p>The duration of the maintenance window in hours.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in
   different ways, such as by purpose, owner, or environment. For example, you might want to tag a
   maintenance window to identify the type of tasks it will run, the types of targets, and the
   environment it will run in. In this case, you could specify the following key-value pairs:</p>
         <ul>
            <li>
               <p>
                  <code>Key=TaskType,Value=AgentUpdate</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=OS,Value=Windows</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=Environment,Value=Production</code>
               </p>
            </li>
         </ul>
         <note>
            <p>To add tags to an existing maintenance window, use the <a>AddTagsToResource</a>
    operation.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the maintenance window.</p> |
| `description` | String | <p>The description of the maintenance window.</p> |
| `duration` | i64 | <p>The duration of the maintenance window in hours.</p> |
| `enabled` | bool | <p>Indicates whether the maintenance window is enabled.</p> |
| `schedule` | String | <p>The schedule of the maintenance window in the form of a cron or rate expression.</p> |
| `allow_unassociated_targets` | bool | <p>Whether targets must be registered with the maintenance window before tasks can be defined
   for those targets.</p> |
| `schedule_offset` | i64 | <p>The number of days to wait to run a maintenance window after the scheduled cron expression
   date and time.</p> |
| `modified_date` | String | <p>The date the maintenance window was last modified.</p> |
| `start_date` | String | <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled
   to become active. The maintenance window won't run before this specified time.</p> |
| `created_date` | String | <p>The date the maintenance window was created.</p> |
| `next_execution_time` | String | <p>The next time the maintenance window will actually run, taking into account any specified
   times for the maintenance window to become active or inactive.</p> |
| `schedule_timezone` | String | <p>The time zone that the scheduled maintenance window executions are based on, in Internet
   Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "UTC", or
   "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time
    Zone Database</a> on the IANA website.</p> |
| `cutoff` | i64 | <p>The number of hours before the end of the maintenance window that Amazon Web Services Systems Manager stops scheduling
   new tasks for execution.</p> |
| `end_date` | String | <p>The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled
   to become inactive. The maintenance window won't run after this specified time.</p> |
| `window_id` | String | <p>The ID of the created maintenance window.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create maintenance_window
maintenance_window = provider.ssm.Maintenance_window {
    schedule = "value"  # <p>The schedule of the maintenance window in the form of a cron or rate expression.</p>
    name = "value"  # <p>The name of the maintenance window.</p>
    allow_unassociated_targets = "value"  # <p>Enables a maintenance window task to run on managed nodes, even if you haven't registered
   those nodes as targets. If enabled, then you must specify the unregistered managed nodes (by node
   ID) when you register a task with the maintenance window.</p>
         <p>If you don't enable this option, then you must specify previously-registered targets when
   you register a task with the maintenance window.</p>
    cutoff = "value"  # <p>The number of hours before the end of the maintenance window that Amazon Web Services Systems Manager stops scheduling
   new tasks for execution.</p>
    duration = "value"  # <p>The duration of the maintenance window in hours.</p>
}

# Access maintenance_window outputs
maintenance_window_id = maintenance_window.id
maintenance_window_name = maintenance_window.name
maintenance_window_description = maintenance_window.description
maintenance_window_duration = maintenance_window.duration
maintenance_window_enabled = maintenance_window.enabled
maintenance_window_schedule = maintenance_window.schedule
maintenance_window_allow_unassociated_targets = maintenance_window.allow_unassociated_targets
maintenance_window_schedule_offset = maintenance_window.schedule_offset
maintenance_window_modified_date = maintenance_window.modified_date
maintenance_window_start_date = maintenance_window.start_date
maintenance_window_created_date = maintenance_window.created_date
maintenance_window_next_execution_time = maintenance_window.next_execution_time
maintenance_window_schedule_timezone = maintenance_window.schedule_timezone
maintenance_window_cutoff = maintenance_window.cutoff
maintenance_window_end_date = maintenance_window.end_date
maintenance_window_window_id = maintenance_window.window_id
```

---


### Parameter

Parameter resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_type` | String |  | <p>The data type for a <code>String</code> parameter. Supported data types include plain text
   and Amazon Machine Image (AMI) IDs.</p>
         <p>
            <b>The following data type values are supported.</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>text</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aws:ec2:image</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aws:ssm:integration</code>
               </p>
            </li>
         </ul>
         <p>When you create a <code>String</code> parameter and specify <code>aws:ec2:image</code>,
   Amazon Web Services Systems Manager validates the parameter value is in the required format, such as
    <code>ami-12345abcdeEXAMPLE</code>, and that the specified AMI is available in your
   Amazon Web Services account.</p>
         <note>
            <p>If the action is successful, the service sends back an HTTP 200 response which indicates a
    successful <code>PutParameter</code> call for all cases except for data type
     <code>aws:ec2:image</code>. If you call <code>PutParameter</code> with
     <code>aws:ec2:image</code> data type, a successful HTTP 200 response does not guarantee that
    your parameter was successfully created or updated. The <code>aws:ec2:image</code> value is
    validated asynchronously, and the <code>PutParameter</code> call returns before the validation
    is complete. If you submit an invalid AMI value, the PutParameter operation will return success,
    but the asynchronous validation will fail and the parameter will not be created or updated. To
    monitor whether your <code>aws:ec2:image</code> parameters are created successfully, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-cwe.html">Setting
     up notifications or trigger actions based on Parameter Store events</a>. For more
    information about AMI format validation , see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-ec2-aliases.html">Native parameter
     support for Amazon Machine Image IDs</a>. </p>
         </note> |
| `description` | String |  | <p>Information about the parameter that you want to add to the system. Optional but
   recommended.</p>
         <important>
            <p>Don't enter personally identifiable information in this field.</p>
         </important> |
| `value` | String | ✅ | <p>The parameter value that you want to add to the system. Standard parameters have a value
   limit of 4 KB. Advanced parameters have a value limit of 8 KB.</p>
         <note>
            <p>Parameters can't be referenced or nested in the values of other parameters. You can't
    include values wrapped in double brackets <code>{{}}</code> or
      <code>{{ssm:<i>parameter-name</i>}}</code> in a parameter value.</p>
         </note> |
| `name` | String | ✅ | <p>The fully qualified name of the parameter that you want to create or update.</p>
         <note>
            <p>You can't enter the Amazon Resource Name (ARN) for a parameter, only the parameter name
    itself.</p>
         </note>
         <p>The fully qualified name includes the complete hierarchy of the parameter path and name. For
   parameters in a hierarchy, you must include a leading forward slash character (/) when you create
   or reference a parameter. For example: <code>/Dev/DBServer/MySQL/db-string13</code>
         </p>
         <p>Naming Constraints:</p>
         <ul>
            <li>
               <p>Parameter names are case sensitive.</p>
            </li>
            <li>
               <p>A parameter name must be unique within an Amazon Web Services Region</p>
            </li>
            <li>
               <p>A parameter name can't be prefixed with "<code>aws</code>" or "<code>ssm</code>"
     (case-insensitive).</p>
            </li>
            <li>
               <p>Parameter names can include only the following symbols and letters:
      <code>a-zA-Z0-9_.-</code>
               </p>
               <p>In addition, the slash character ( / ) is used to delineate hierarchies in parameter
     names. For example: <code>/Dev/Production/East/Project-ABC/MyParameter</code>
               </p>
            </li>
            <li>
               <p>Parameter names can't contain spaces. The service removes any spaces specified for
     the beginning or end of a parameter name. If the specified name for a parameter contains spaces
     between characters, the request fails with a <code>ValidationException</code> error.</p>
            </li>
            <li>
               <p>Parameter hierarchies are limited to a maximum depth of fifteen levels.</p>
            </li>
         </ul>
         <p>For additional information about valid values for parameter names, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-su-create.html">Creating Systems Manager parameters</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <note>
            <p>The reported maximum length of 2048 characters for a parameter name includes 1037
    characters that are reserved for internal use by Systems Manager. The maximum length for a parameter name
    that you specify is 1011 characters.</p>
            <p>This count of 1011 characters includes the characters in the ARN that precede the name you
    specify. This ARN length will vary depending on your partition and Region. For example, the
    following 45 characters count toward the 1011 character maximum for a parameter created in the
    US East (Ohio) Region: <code>arn:aws:ssm:us-east-2:111122223333:parameter/</code>.</p>
         </note> |
| `type` | String |  | <p>The type of parameter that you want to create.</p>
         <note>
            <p>
               <code>SecureString</code> isn't currently supported for CloudFormation templates.</p>
         </note>
         <p>Items in a <code>StringList</code> must be separated by a comma (,). You can't
   use other punctuation or special character to escape items in the list. If you have a parameter
   value that requires a comma, then use the <code>String</code> data type.</p>
         <important>
            <p>Specifying a parameter type isn't required when updating a parameter. You must specify a
    parameter type when creating a parameter.</p>
         </important> |
| `allowed_pattern` | String |  | <p>A regular expression used to validate the parameter value. For example, for String types
   with values restricted to numbers, you can specify the following: AllowedPattern=^\d+$ </p> |
| `tags` | Vec<String> |  | <p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in
   different ways, such as by purpose, owner, or environment. For example, you might want to tag a
   Systems Manager parameter to identify the type of resource to which it applies, the environment, or the
   type of configuration data referenced by the parameter. In this case, you could specify the
   following key-value pairs:</p>
         <ul>
            <li>
               <p>
                  <code>Key=Resource,Value=S3bucket</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=OS,Value=Windows</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=ParameterType,Value=LicenseKey</code>
               </p>
            </li>
         </ul>
         <note>
            <p>To add tags to an existing Systems Manager parameter, use the <a>AddTagsToResource</a>
    operation.</p>
         </note> |
| `key_id` | String |  | <p>The Key Management Service (KMS) ID that you want to use to encrypt a
   parameter. Use a custom key for better security. Required for parameters that use the <code>SecureString</code> data type.</p>
         <p>If you don't specify a key ID, the system uses the default key associated with your
   Amazon Web Services account, which is not as secure as using a custom key.</p>
         <ul>
            <li>
               <p>To use a custom KMS key, choose the <code>SecureString</code>
     data type with the <code>Key ID</code> parameter.</p>
            </li>
         </ul> |
| `overwrite` | bool |  | <p>Overwrite an existing parameter. The default value is <code>false</code>.</p> |
| `tier` | String |  | <p>The parameter tier to assign to a parameter.</p>
         <p>Parameter Store offers a standard tier and an advanced tier for parameters. Standard
   parameters have a content size limit of 4 KB and can't be configured to use parameter policies.
   You can create a maximum of 10,000 standard parameters for each Region in an Amazon Web Services account.
   Standard parameters are offered at no additional cost. </p>
         <p>Advanced parameters have a content size limit of 8 KB and can be configured to use parameter
   policies. You can create a maximum of 100,000 advanced parameters for each Region in an
   Amazon Web Services account. Advanced parameters incur a charge. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-advanced-parameters.html">Managing
    parameter tiers</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <p>You can change a standard parameter to an advanced parameter any time. But you can't revert
   an advanced parameter to a standard parameter. Reverting an advanced parameter to a standard
   parameter would result in data loss because the system would truncate the size of the parameter
   from 8 KB to 4 KB. Reverting would also remove any policies attached to the parameter. Lastly,
   advanced parameters use a different form of encryption than standard parameters. </p>
         <p>If you no longer need an advanced parameter, or if you no longer want to incur charges for
   an advanced parameter, you must delete it and recreate it as a new standard parameter. </p>
         <p>
            <b>Using the Default Tier Configuration</b>
         </p>
         <p>In <code>PutParameter</code> requests, you can specify the tier to create the parameter in.
   Whenever you specify a tier in the request, Parameter Store creates or updates the parameter
   according to that request. However, if you don't specify a tier in a request, Parameter Store
   assigns the tier based on the current Parameter Store default tier configuration.</p>
         <p>The default tier when you begin using Parameter Store is the standard-parameter tier. If you
   use the advanced-parameter tier, you can specify one of the following as the default:</p>
         <ul>
            <li>
               <p>
                  <b>Advanced</b>: With this option, Parameter Store evaluates all
     requests as advanced parameters. </p>
            </li>
            <li>
               <p>
                  <b>Intelligent-Tiering</b>: With this option, Parameter Store
     evaluates each request to determine if the parameter is standard or advanced. </p>
               <p>If the request doesn't include any options that require an advanced parameter, the
     parameter is created in the standard-parameter tier. If one or more options requiring an
     advanced parameter are included in the request, Parameter Store create a parameter in the
     advanced-parameter tier.</p>
               <p>This approach helps control your parameter-related costs by always creating standard
     parameters unless an advanced parameter is necessary. </p>
            </li>
         </ul>
         <p>Options that require an advanced parameter include the following:</p>
         <ul>
            <li>
               <p>The content size of the parameter is more than 4 KB.</p>
            </li>
            <li>
               <p>The parameter uses a parameter policy.</p>
            </li>
            <li>
               <p>More than 10,000 parameters already exist in your Amazon Web Services account in the current
     Amazon Web Services Region.</p>
            </li>
         </ul>
         <p>For more information about configuring the default tier option, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-advanced-parameters.html#ps-default-tier">Specifying a default parameter tier</a> in the
   <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `policies` | String |  | <p>One or more policies to apply to a parameter. This operation takes a JSON array. Parameter
   Store, a tool in Amazon Web Services Systems Manager supports the following policy types:</p>
         <p>Expiration: This policy deletes the parameter after it expires. When you create the policy,
   you specify the expiration date. You can update the expiration date and time by updating the
   policy. Updating the <i>parameter</i> doesn't affect the expiration date and time.
   When the expiration time is reached, Parameter Store deletes the parameter.</p>
         <p>ExpirationNotification: This policy initiates an event in Amazon CloudWatch Events that
   notifies you about the expiration. By using this policy, you can receive notification before or
   after the expiration time is reached, in units of days or hours.</p>
         <p>NoChangeNotification: This policy initiates a CloudWatch Events event if a parameter hasn't
   been modified for a specified period of time. This policy type is useful when, for example, a
   secret needs to be changed within a period of time, but it hasn't been changed.</p>
         <p>All existing policies are preserved until you send new policies or an empty policy. For more
   information about parameter policies, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-policies.html">Assigning parameter
    policies</a>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameter` | String | <p>Information about a parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create parameter
parameter = provider.ssm.Parameter {
    value = "value"  # <p>The parameter value that you want to add to the system. Standard parameters have a value
   limit of 4 KB. Advanced parameters have a value limit of 8 KB.</p>
         <note>
            <p>Parameters can't be referenced or nested in the values of other parameters. You can't
    include values wrapped in double brackets <code>{{}}</code> or
      <code>{{ssm:<i>parameter-name</i>}}</code> in a parameter value.</p>
         </note>
    name = "value"  # <p>The fully qualified name of the parameter that you want to create or update.</p>
         <note>
            <p>You can't enter the Amazon Resource Name (ARN) for a parameter, only the parameter name
    itself.</p>
         </note>
         <p>The fully qualified name includes the complete hierarchy of the parameter path and name. For
   parameters in a hierarchy, you must include a leading forward slash character (/) when you create
   or reference a parameter. For example: <code>/Dev/DBServer/MySQL/db-string13</code>
         </p>
         <p>Naming Constraints:</p>
         <ul>
            <li>
               <p>Parameter names are case sensitive.</p>
            </li>
            <li>
               <p>A parameter name must be unique within an Amazon Web Services Region</p>
            </li>
            <li>
               <p>A parameter name can't be prefixed with "<code>aws</code>" or "<code>ssm</code>"
     (case-insensitive).</p>
            </li>
            <li>
               <p>Parameter names can include only the following symbols and letters:
      <code>a-zA-Z0-9_.-</code>
               </p>
               <p>In addition, the slash character ( / ) is used to delineate hierarchies in parameter
     names. For example: <code>/Dev/Production/East/Project-ABC/MyParameter</code>
               </p>
            </li>
            <li>
               <p>Parameter names can't contain spaces. The service removes any spaces specified for
     the beginning or end of a parameter name. If the specified name for a parameter contains spaces
     between characters, the request fails with a <code>ValidationException</code> error.</p>
            </li>
            <li>
               <p>Parameter hierarchies are limited to a maximum depth of fifteen levels.</p>
            </li>
         </ul>
         <p>For additional information about valid values for parameter names, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-su-create.html">Creating Systems Manager parameters</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <note>
            <p>The reported maximum length of 2048 characters for a parameter name includes 1037
    characters that are reserved for internal use by Systems Manager. The maximum length for a parameter name
    that you specify is 1011 characters.</p>
            <p>This count of 1011 characters includes the characters in the ARN that precede the name you
    specify. This ARN length will vary depending on your partition and Region. For example, the
    following 45 characters count toward the 1011 character maximum for a parameter created in the
    US East (Ohio) Region: <code>arn:aws:ssm:us-east-2:111122223333:parameter/</code>.</p>
         </note>
}

# Access parameter outputs
parameter_id = parameter.id
parameter_parameter = parameter.parameter
```

---


### Sessions

Sessions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next set of items to return. (You received this token from a previous
   call.)</p> |
| `sessions` | Vec<String> | <p>A list of sessions meeting the request parameters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sessions outputs
sessions_id = sessions.id
sessions_next_token = sessions.next_token
sessions_sessions = sessions.sessions
```

---


### Resource_data_sync

ResourceDataSync resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sync_type` | String |  | <p>Specify <code>SyncToDestination</code> to create a resource data sync that synchronizes data
   to an S3 bucket for Inventory. If you specify <code>SyncToDestination</code>, you must provide a
   value for <code>S3Destination</code>. Specify <code>SyncFromSource</code> to synchronize data
   from a single account and multiple Regions, or multiple Amazon Web Services accounts and Amazon Web Services Regions, as
   listed in Organizations for Explorer. If you specify <code>SyncFromSource</code>, you must provide a
   value for <code>SyncSource</code>. The default value is <code>SyncToDestination</code>.</p> |
| `sync_source` | String |  | <p>Specify information about the data sources to synchronize. This parameter is required if the
    <code>SyncType</code> value is SyncFromSource.</p> |
| `s3_destination` | String |  | <p>Amazon S3 configuration details for the sync. This parameter is required if the
    <code>SyncType</code> value is SyncToDestination.</p> |
| `sync_name` | String | ✅ | <p>A name for the configuration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_data_sync
resource_data_sync = provider.ssm.Resource_data_sync {
    sync_name = "value"  # <p>A name for the configuration.</p>
}

```

---


### Managed_instance_role

ManagedInstanceRole resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The ID of the managed node where you want to update the role.</p> |
| `iam_role` | String | ✅ | <p>The name of the Identity and Access Management (IAM) role that you want to assign to
   the managed node. This IAM role must provide AssumeRole permissions for the
   Amazon Web Services Systems Manager service principal <code>ssm.amazonaws.com</code>. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/hybrid-multicloud-service-role.html">Create the IAM service role required for Systems Manager in hybrid and multicloud
    environments</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         <note>
            <p>You can't specify an IAM service-linked role for this parameter. You must
    create a unique role.</p>
         </note> |



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


### Default_patch_baseline

DefaultPatchBaseline resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `baseline_id` | String | <p>The ID of the default patch baseline.</p> |
| `operating_system` | String | <p>The operating system for the returned patch baseline. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_patch_baseline outputs
default_patch_baseline_id = default_patch_baseline.id
default_patch_baseline_baseline_id = default_patch_baseline.baseline_id
default_patch_baseline_operating_system = default_patch_baseline.operating_system
```

---


### Automation_executions

AutomationExecutions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |
| `automation_execution_metadata_list` | Vec<String> | <p>The list of details about each automation execution which has occurred which matches the
   filter specification, if any.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access automation_executions outputs
automation_executions_id = automation_executions.id
automation_executions_next_token = automation_executions.next_token
automation_executions_automation_execution_metadata_list = automation_executions.automation_execution_metadata_list
```

---


### Connection_status

ConnectionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the connection to the managed node.</p> |
| `target` | String | <p>The ID of the managed node to check connection status. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection_status outputs
connection_status_id = connection_status.id
connection_status_status = connection_status.status
connection_status_target = connection_status.target
```

---


### Ops_items

OpsItems resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ops_item_summaries` | Vec<String> | <p>A list of OpsItems.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ops_items outputs
ops_items_id = ops_items.id
ops_items_ops_item_summaries = ops_items.ops_item_summaries
ops_items_next_token = ops_items.next_token
```

---


### Maintenance_window_execution

MaintenanceWindowExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_ids` | Vec<String> | <p>The ID of the task executions from the maintenance window execution.</p> |
| `end_time` | String | <p>The time the maintenance window finished running.</p> |
| `status` | String | <p>The status of the maintenance window execution.</p> |
| `status_details` | String | <p>The details explaining the status. Not available for all status values.</p> |
| `window_execution_id` | String | <p>The ID of the maintenance window execution.</p> |
| `start_time` | String | <p>The time the maintenance window started running.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_execution outputs
maintenance_window_execution_id = maintenance_window_execution.id
maintenance_window_execution_task_ids = maintenance_window_execution.task_ids
maintenance_window_execution_end_time = maintenance_window_execution.end_time
maintenance_window_execution_status = maintenance_window_execution.status
maintenance_window_execution_status_details = maintenance_window_execution.status_details
maintenance_window_execution_window_execution_id = maintenance_window_execution.window_execution_id
maintenance_window_execution_start_time = maintenance_window_execution.start_time
```

---


### Maintenance_window_task

MaintenanceWindowTask resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `targets` | Vec<String> |  | <p>The targets (either managed nodes or tags) to modify. Managed nodes are specified using the
   format <code>Key=instanceids,Values=instanceID_1,instanceID_2</code>. Tags are specified using
   the format <code> Key=tag_name,Values=tag_value</code>. </p>
         <note>
            <p>One or more targets must be specified for maintenance window Run Command-type tasks.
    Depending on the task, targets are optional for other maintenance window task types (Automation,
     Lambda, and Step Functions). For more information about running tasks
    that don't specify targets, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">Registering
     maintenance window tasks without targets</a> in the
    <i>Amazon Web Services Systems Manager User Guide</i>.</p>
         </note> |
| `priority` | i64 |  | <p>The new task priority to specify. The lower the number, the higher the priority. Tasks that
   have the same priority are scheduled in parallel.</p> |
| `window_id` | String | ✅ | <p>The maintenance window ID that contains the task to modify.</p> |
| `alarm_configuration` | String |  | <p>The CloudWatch alarm you want to apply to your maintenance window task.</p> |
| `service_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM service role for
                Amazon Web Services Systems Manager to assume when running a maintenance window task. If you do not specify a
                service role ARN, Systems Manager uses a service-linked role in your account. If no
                appropriate service-linked role for Systems Manager exists in your account, it is created when
                you run <code>RegisterTaskWithMaintenanceWindow</code>.</p>
         <p>However, for an improved security posture, we strongly recommend creating a custom
                policy and custom service role for running your maintenance window tasks. The policy
                can be crafted to provide only the permissions needed for your particular
                maintenance window tasks. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-maintenance-permissions.html">Setting up Maintenance Windows</a> in the in the
                    <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `replace` | bool |  | <p>If True, then all fields that are required by the <a>RegisterTaskWithMaintenanceWindow</a> operation are also required for this API request.
   Optional fields that aren't specified are set to null.</p> |
| `max_errors` | String |  | <p>The new <code>MaxErrors</code> value to specify. <code>MaxErrors</code> is the maximum
   number of errors that are allowed before the task stops being scheduled.</p>
         <note>
            <p>Although this element is listed as "Required: No", a value can be omitted only when you are
    registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless
     task</a> You must provide a value in all other cases.</p>
            <p>For maintenance window tasks without a target specified, you can't supply a value for this
    option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't
    affect the running of your task.</p>
         </note> |
| `logging_info` | String |  | <p>The new logging location in Amazon S3 to specify.</p>
         <note>
            <p>
               <code>LoggingInfo</code> has been deprecated. To specify an Amazon Simple Storage Service (Amazon S3) bucket to contain logs, instead use the
      <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure.
      For information about how Amazon Web Services Systems Manager handles these options for the supported maintenance
      window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p>
         </note> |
| `task_arn` | String |  | <p>The task ARN to modify.</p> |
| `task_parameters` | HashMap<String, String> |  | <p>The parameters to modify.</p>
         <note>
            <p>
               <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs,
      instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information
      about how Systems Manager handles these options for the supported maintenance window task
      types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p>
         </note>
         <p>The map has the following format:</p>
         <p>Key: string, between 1 and 255 characters</p>
         <p>Value: an array of strings, each string is between 1 and 255 characters</p> |
| `window_task_id` | String | ✅ | <p>The task ID to modify.</p> |
| `task_invocation_parameters` | String |  | <p>The parameters that the task should use during execution. Populate only the fields that
   match the task type. All other fields should be empty.</p>
         <important>
            <p>When you update a maintenance window task that has options specified in
     <code>TaskInvocationParameters</code>, you must provide again all the
     <code>TaskInvocationParameters</code> values that you want to retain. The values you don't
    specify again are removed. For example, suppose that when you registered a Run Command task, you
    specified <code>TaskInvocationParameters</code> values for <code>Comment</code>,
     <code>NotificationConfig</code>, and <code>OutputS3BucketName</code>. If you update the
    maintenance window task and specify only a different <code>OutputS3BucketName</code> value, the
    values for <code>Comment</code> and <code>NotificationConfig</code> are removed.</p>
         </important> |
| `max_concurrency` | String |  | <p>The new <code>MaxConcurrency</code> value you want to specify. <code>MaxConcurrency</code>
   is the number of targets that are allowed to run this task, in parallel.</p>
         <note>
            <p>Although this element is listed as "Required: No", a value can be omitted only when you are
    registering or updating a <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/maintenance-windows-targetless-tasks.html">targetless
     task</a> You must provide a value in all other cases.</p>
            <p>For maintenance window tasks without a target specified, you can't supply a value for this
    option. Instead, the system inserts a placeholder value of <code>1</code>. This value doesn't
    affect the running of your task.</p>
         </note> |
| `name` | String |  | <p>The new task name to specify.</p> |
| `description` | String |  | <p>The new task description to specify.</p> |
| `cutoff_behavior` | String |  | <p>Indicates whether tasks should continue to run after the cutoff time specified in the
   maintenance windows is reached. </p>
         <ul>
            <li>
               <p>
                  <code>CONTINUE_TASK</code>: When the cutoff time is reached, any tasks that are running
     continue. The default value.</p>
            </li>
            <li>
               <p>
                  <code>CANCEL_TASK</code>:</p>
               <ul>
                  <li>
                     <p>For Automation, Lambda, Step Functions tasks: When the cutoff
       time is reached, any task invocations that are already running continue, but no new task
       invocations are started.</p>
                  </li>
                  <li>
                     <p>For Run Command tasks: When the cutoff time is reached, the system sends a <a>CancelCommand</a> operation that attempts to cancel the command associated with the
       task. However, there is no guarantee that the command will be terminated and the underlying
       process stopped.</p>
                  </li>
               </ul>
               <p>The status for tasks that are not completed is <code>TIMED_OUT</code>.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alarm_configuration` | String | <p>The details for the CloudWatch alarm you applied to your maintenance window
   task.</p> |
| `window_task_id` | String | <p>The retrieved maintenance window task ID.</p> |
| `task_arn` | String | <p>The resource that the task used during execution. For <code>RUN_COMMAND</code> and
    <code>AUTOMATION</code> task types, the value of <code>TaskArn</code> is the SSM document
   name/ARN. For <code>LAMBDA</code> tasks, the value is the function name/ARN. For
    <code>STEP_FUNCTIONS</code> tasks, the value is the state machine ARN.</p> |
| `name` | String | <p>The retrieved task name.</p> |
| `service_role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM service role for
                Amazon Web Services Systems Manager to assume when running a maintenance window task. If you do not specify a
                service role ARN, Systems Manager uses a service-linked role in your account. If no
                appropriate service-linked role for Systems Manager exists in your account, it is created when
                you run <code>RegisterTaskWithMaintenanceWindow</code>.</p>
         <p>However, for an improved security posture, we strongly recommend creating a custom
                policy and custom service role for running your maintenance window tasks. The policy
                can be crafted to provide only the permissions needed for your particular
                maintenance window tasks. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-maintenance-permissions.html">Setting up Maintenance Windows</a> in the in the
                    <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `task_type` | String | <p>The type of task to run.</p> |
| `max_errors` | String | <p>The maximum number of errors allowed before the task stops being scheduled.</p>
         <note>
            <p>For maintenance window tasks without a target specified, you can't supply a value for this
    option. Instead, the system inserts a placeholder value of <code>1</code>, which may be reported
    in the response to this command. This value doesn't affect the running of your task and can be
    ignored.</p>
         </note> |
| `logging_info` | String | <p>The location in Amazon Simple Storage Service (Amazon S3) where the task results are
   logged.</p>
         <note>
            <p>
               <code>LoggingInfo</code> has been deprecated. To specify an Amazon Simple Storage Service (Amazon S3) bucket to contain logs, instead use the
      <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure.
      For information about how Amazon Web Services Systems Manager handles these options for the supported maintenance
      window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p>
         </note> |
| `targets` | Vec<String> | <p>The targets where the task should run.</p> |
| `description` | String | <p>The retrieved task description.</p> |
| `max_concurrency` | String | <p>The maximum number of targets allowed to run this task in parallel.</p>
         <note>
            <p>For maintenance window tasks without a target specified, you can't supply a value for this
    option. Instead, the system inserts a placeholder value of <code>1</code>, which may be reported
    in the response to this command. This value doesn't affect the running of your task and can be
    ignored.</p>
         </note> |
| `window_id` | String | <p>The retrieved maintenance window ID.</p> |
| `cutoff_behavior` | String | <p>The action to take on tasks when the maintenance window cutoff time is reached.
    <code>CONTINUE_TASK</code> means that tasks continue to run. For Automation, Lambda, Step Functions tasks, <code>CANCEL_TASK</code> means that currently
   running task invocations continue, but no new task invocations are started. For Run Command
   tasks, <code>CANCEL_TASK</code> means the system attempts to stop the task by sending a
    <code>CancelCommand</code> operation.</p> |
| `task_invocation_parameters` | String | <p>The parameters to pass to the task when it runs.</p> |
| `priority` | i64 | <p>The priority of the task when it runs. The lower the number, the higher the priority. Tasks
   that have the same priority are scheduled in parallel.</p> |
| `task_parameters` | HashMap<String, String> | <p>The parameters to pass to the task when it runs.</p>
         <note>
            <p>
               <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs,
      instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information
      about how Systems Manager handles these options for the supported maintenance window task
      types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_task outputs
maintenance_window_task_id = maintenance_window_task.id
maintenance_window_task_alarm_configuration = maintenance_window_task.alarm_configuration
maintenance_window_task_window_task_id = maintenance_window_task.window_task_id
maintenance_window_task_task_arn = maintenance_window_task.task_arn
maintenance_window_task_name = maintenance_window_task.name
maintenance_window_task_service_role_arn = maintenance_window_task.service_role_arn
maintenance_window_task_task_type = maintenance_window_task.task_type
maintenance_window_task_max_errors = maintenance_window_task.max_errors
maintenance_window_task_logging_info = maintenance_window_task.logging_info
maintenance_window_task_targets = maintenance_window_task.targets
maintenance_window_task_description = maintenance_window_task.description
maintenance_window_task_max_concurrency = maintenance_window_task.max_concurrency
maintenance_window_task_window_id = maintenance_window_task.window_id
maintenance_window_task_cutoff_behavior = maintenance_window_task.cutoff_behavior
maintenance_window_task_task_invocation_parameters = maintenance_window_task.task_invocation_parameters
maintenance_window_task_priority = maintenance_window_task.priority
maintenance_window_task_task_parameters = maintenance_window_task.task_parameters
```

---


### Document_permission

DocumentPermission resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_sharing_info_list` | Vec<String> | <p>A list of Amazon Web Services accounts where the current document is shared and the version shared with
   each account.</p> |
| `account_ids` | Vec<String> | <p>The account IDs that have permission to use this document. The ID can be either an
   Amazon Web Services account number or <code>all</code>.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_permission outputs
document_permission_id = document_permission.id
document_permission_account_sharing_info_list = document_permission.account_sharing_info_list
document_permission_account_ids = document_permission.account_ids
document_permission_next_token = document_permission.next_token
```

---


### Instance_properties

InstanceProperties resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next set of properties to return. Use this token to get the next set of
   results.</p> |
| `instance_properties` | Vec<String> | <p>Properties for the managed instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_properties outputs
instance_properties_id = instance_properties.id
instance_properties_next_token = instance_properties.next_token
instance_properties_instance_properties = instance_properties.instance_properties
```

---


### Inventory_schema

InventorySchema resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |
| `schemas` | Vec<String> | <p>Inventory schemas returned by the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access inventory_schema outputs
inventory_schema_id = inventory_schema.id
inventory_schema_next_token = inventory_schema.next_token
inventory_schema_schemas = inventory_schema.schemas
```

---


### Association_executions

AssociationExecutions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `association_executions` | Vec<String> | <p>A list of the executions for the specified association ID.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access association_executions outputs
association_executions_id = association_executions.id
association_executions_association_executions = association_executions.association_executions
association_executions_next_token = association_executions.next_token
```

---


### Parameters_by_path

ParametersByPath resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | <p>A list of parameters found in the specified hierarchy.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access parameters_by_path outputs
parameters_by_path_id = parameters_by_path.id
parameters_by_path_parameters = parameters_by_path.parameters
parameters_by_path_next_token = parameters_by_path.next_token
```

---


### Patch_baseline

PatchBaseline resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rejected_patches_action` | String |  | <p>The action for Patch Manager to take on patches included in the
    <code>RejectedPackages</code> list.</p>
         <dl>
            <dt>ALLOW_AS_DEPENDENCY</dt>
            <dd>
               <p>
                  <b>Linux and macOS</b>: A package in the rejected patches list
      is installed only if it is a dependency of another package. It is considered compliant with
      the patch baseline, and its status is reported as <code>INSTALLED_OTHER</code>. This is the
      default action if no option is specified.</p>
               <p>
                  <b>Windows Server</b>: Windows Server doesn't support the
      concept of package dependencies. If a package in the rejected patches list and already
      installed on the node, its status is reported as <code>INSTALLED_OTHER</code>. Any package not
      already installed on the node is skipped. This is the default action if no option is
      specified.</p>
            </dd>
            <dt>BLOCK</dt>
            <dd>
               <p>
                  <b>All OSs</b>: Packages in the rejected patches list, and
      packages that include them as dependencies, aren't installed by Patch Manager under any
      circumstances. </p>
               <p>State value assignment for patch compliance:</p>
               <ul>
                  <li>
                     <p>If a package was installed before it was added to the rejected patches list, or is
        installed outside of Patch Manager afterward, it's considered noncompliant with the patch
        baseline and its status is reported as <code>INSTALLED_REJECTED</code>.</p>
                  </li>
                  <li>
                     <p>If an update attempts to install a dependency package that is now rejected by the
        baseline, when previous versions of the package were not rejected, the package being updated
        is reported as <code>MISSING</code> for <code>SCAN</code> operations and as
         <code>FAILED</code> for <code>INSTALL</code> operations.</p>
                  </li>
               </ul>
            </dd>
         </dl> |
| `approved_patches` | Vec<String> |  | <p>A list of explicitly approved patches for the baseline.</p>
         <p>For information about accepted formats for lists of approved patches and rejected patches,
                        see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package
                        name formats for approved and rejected patch lists</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `name` | String | ✅ | <p>The name of the patch baseline.</p> |
| `approved_patches_compliance_level` | String |  | <p>Defines the compliance level for approved patches. When an approved patch is reported as
   missing, this value describes the severity of the compliance violation. The default value is
    <code>UNSPECIFIED</code>.</p> |
| `approved_patches_enable_non_security` | bool |  | <p>Indicates whether the list of approved patches includes non-security updates that should be
   applied to the managed nodes. The default value is <code>false</code>. Applies to Linux managed
   nodes only.</p> |
| `available_security_updates_compliance_status` | String |  | <p>Indicates the status you want to assign to security patches that are available but not
   approved because they don't meet the installation criteria specified in the patch
   baseline.</p>
         <p>Example scenario: Security patches that you might want installed can be skipped if you have
   specified a long period to wait after a patch is released before installation. If an update to
   the patch is released during your specified waiting period, the waiting period for installing the
   patch starts over. If the waiting period is too long, multiple versions of the patch could be
   released but never installed.</p>
         <p>Supported for Windows Server managed nodes only.</p> |
| `operating_system` | String |  | <p>Defines the operating system the patch baseline applies to. The default value is
    <code>WINDOWS</code>.</p> |
| `client_token` | String |  | <p>User-provided idempotency token.</p> |
| `sources` | Vec<String> |  | <p>Information about the patches to use to update the managed nodes, including target operating
   systems and source repositories. Applies to Linux managed nodes only.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata that you assign to a resource. Tags enable you to categorize a resource in
   different ways, such as by purpose, owner, or environment. For example, you might want to tag a
   patch baseline to identify the severity level of patches it specifies and the operating system
   family it applies to. In this case, you could specify the following key-value pairs:</p>
         <ul>
            <li>
               <p>
                  <code>Key=PatchSeverity,Value=Critical</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Key=OS,Value=Windows</code>
               </p>
            </li>
         </ul>
         <note>
            <p>To add tags to an existing patch baseline, use the <a>AddTagsToResource</a>
    operation.</p>
         </note> |
| `description` | String |  | <p>A description of the patch baseline.</p> |
| `rejected_patches` | Vec<String> |  | <p>A list of explicitly rejected patches for the baseline.</p>
         <p>For information about accepted formats for lists of approved patches and rejected patches,
                        see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package
                        name formats for approved and rejected patch lists</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p> |
| `global_filters` | String |  | <p>A set of global filters used to include patches in the baseline.</p>
         <important>
            <p>The <code>GlobalFilters</code> parameter can be configured only by using the CLI or an Amazon Web Services SDK. It can't be configured from the Patch Manager
    console, and its value isn't displayed in the console.</p>
         </important> |
| `approval_rules` | String |  | <p>A set of rules used to include patches in the baseline.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>A description of the patch baseline.</p> |
| `rejected_patches_action` | String | <p>The action specified to take on patches included in the <code>RejectedPatches</code> list. A
   patch can be allowed only if it is a dependency of another package, or blocked entirely along
   with packages that include it as a dependency.</p> |
| `approved_patches_compliance_level` | String | <p>Returns the specified compliance severity level for approved patches in the patch
   baseline.</p> |
| `approval_rules` | String | <p>A set of rules used to include patches in the baseline.</p> |
| `available_security_updates_compliance_status` | String | <p>Indicates the compliance status of managed nodes for which security-related patches are
   available but were not approved. This preference is specified when the
    <code>CreatePatchBaseline</code> or <code>UpdatePatchBaseline</code> commands are run.</p>
         <p>Applies to Windows Server managed nodes only.</p> |
| `approved_patches_enable_non_security` | bool | <p>Indicates whether the list of approved patches includes non-security updates that should be
   applied to the managed nodes. The default value is <code>false</code>. Applies to Linux managed
   nodes only.</p> |
| `global_filters` | String | <p>A set of global filters used to exclude patches from the baseline.</p> |
| `created_date` | String | <p>The date the patch baseline was created.</p> |
| `baseline_id` | String | <p>The ID of the retrieved patch baseline.</p> |
| `patch_groups` | Vec<String> | <p>Patch groups included in the patch baseline.</p> |
| `name` | String | <p>The name of the patch baseline.</p> |
| `operating_system` | String | <p>Returns the operating system specified for the patch baseline.</p> |
| `rejected_patches` | Vec<String> | <p>A list of explicitly rejected patches for the baseline.</p> |
| `sources` | Vec<String> | <p>Information about the patches to use to update the managed nodes, including target operating
   systems and source repositories. Applies to Linux managed nodes only.</p> |
| `approved_patches` | Vec<String> | <p>A list of explicitly approved patches for the baseline.</p> |
| `modified_date` | String | <p>The date the patch baseline was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create patch_baseline
patch_baseline = provider.ssm.Patch_baseline {
    name = "value"  # <p>The name of the patch baseline.</p>
}

# Access patch_baseline outputs
patch_baseline_id = patch_baseline.id
patch_baseline_description = patch_baseline.description
patch_baseline_rejected_patches_action = patch_baseline.rejected_patches_action
patch_baseline_approved_patches_compliance_level = patch_baseline.approved_patches_compliance_level
patch_baseline_approval_rules = patch_baseline.approval_rules
patch_baseline_available_security_updates_compliance_status = patch_baseline.available_security_updates_compliance_status
patch_baseline_approved_patches_enable_non_security = patch_baseline.approved_patches_enable_non_security
patch_baseline_global_filters = patch_baseline.global_filters
patch_baseline_created_date = patch_baseline.created_date
patch_baseline_baseline_id = patch_baseline.baseline_id
patch_baseline_patch_groups = patch_baseline.patch_groups
patch_baseline_name = patch_baseline.name
patch_baseline_operating_system = patch_baseline.operating_system
patch_baseline_rejected_patches = patch_baseline.rejected_patches
patch_baseline_sources = patch_baseline.sources
patch_baseline_approved_patches = patch_baseline.approved_patches
patch_baseline_modified_date = patch_baseline.modified_date
```

---


### Parameters

Parameters resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | Vec<String> | <p>A list of details for a parameter.</p> |
| `invalid_parameters` | Vec<String> | <p>A list of parameters that aren't formatted correctly or don't run during an
   execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access parameters outputs
parameters_id = parameters.id
parameters_parameters = parameters.parameters
parameters_invalid_parameters = parameters.invalid_parameters
```

---


### Available_patches

AvailablePatches resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `patches` | Vec<String> | <p>An array of patches. Each entry in the array is a patch structure.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access available_patches outputs
available_patches_id = available_patches.id
available_patches_patches = available_patches.patches
available_patches_next_token = available_patches.next_token
```

---


### Maintenance_window_execution_task

MaintenanceWindowExecutionTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_arn` | String | <p>The Amazon Resource Name (ARN) of the task that ran.</p> |
| `max_concurrency` | String | <p>The defined maximum number of task executions that could be run in parallel.</p> |
| `start_time` | String | <p>The time the task execution started.</p> |
| `status_details` | String | <p>The details explaining the status. Not available for all status values.</p> |
| `window_execution_id` | String | <p>The ID of the maintenance window execution that includes the task.</p> |
| `task_parameters` | Vec<HashMap<String, String>> | <p>The parameters passed to the task when it was run.</p>
         <note>
            <p>
               <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs,
      instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information
      about how Systems Manager handles these options for the supported maintenance window task
      types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p>
         </note>
         <p>The map has the following format:</p>
         <ul>
            <li>
               <p>
                  <code>Key</code>: string, between 1 and 255 characters</p>
            </li>
            <li>
               <p>
                  <code>Value</code>: an array of strings, each between 1 and 255 characters</p>
            </li>
         </ul> |
| `end_time` | String | <p>The time the task execution completed.</p> |
| `task_execution_id` | String | <p>The ID of the specific task execution in the maintenance window task that was
   retrieved.</p> |
| `alarm_configuration` | String | <p>The details for the CloudWatch alarm you applied to your maintenance window
   task.</p> |
| `priority` | i64 | <p>The priority of the task.</p> |
| `service_role` | String | <p>The role that was assumed when running the task.</p> |
| `status` | String | <p>The status of the task.</p> |
| `type` | String | <p>The type of task that was run.</p> |
| `triggered_alarms` | Vec<String> | <p>The CloudWatch alarms that were invoked by the maintenance window task.</p> |
| `max_errors` | String | <p>The defined maximum number of task execution errors allowed before scheduling of the task
   execution would have been stopped.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_window_execution_task outputs
maintenance_window_execution_task_id = maintenance_window_execution_task.id
maintenance_window_execution_task_task_arn = maintenance_window_execution_task.task_arn
maintenance_window_execution_task_max_concurrency = maintenance_window_execution_task.max_concurrency
maintenance_window_execution_task_start_time = maintenance_window_execution_task.start_time
maintenance_window_execution_task_status_details = maintenance_window_execution_task.status_details
maintenance_window_execution_task_window_execution_id = maintenance_window_execution_task.window_execution_id
maintenance_window_execution_task_task_parameters = maintenance_window_execution_task.task_parameters
maintenance_window_execution_task_end_time = maintenance_window_execution_task.end_time
maintenance_window_execution_task_task_execution_id = maintenance_window_execution_task.task_execution_id
maintenance_window_execution_task_alarm_configuration = maintenance_window_execution_task.alarm_configuration
maintenance_window_execution_task_priority = maintenance_window_execution_task.priority
maintenance_window_execution_task_service_role = maintenance_window_execution_task.service_role
maintenance_window_execution_task_status = maintenance_window_execution_task.status
maintenance_window_execution_task_type = maintenance_window_execution_task.type
maintenance_window_execution_task_triggered_alarms = maintenance_window_execution_task.triggered_alarms
maintenance_window_execution_task_max_errors = maintenance_window_execution_task.max_errors
```

---


### Instance_patches

InstancePatches resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `patches` | Vec<String> | <p>Each entry in the array is a structure containing:</p>
         <ul>
            <li>
               <p>Title (string)</p>
            </li>
            <li>
               <p>KBId (string)</p>
            </li>
            <li>
               <p>Classification (string)</p>
            </li>
            <li>
               <p>Severity (string)</p>
            </li>
            <li>
               <p>State (string, such as "INSTALLED" or "FAILED")</p>
            </li>
            <li>
               <p>InstalledTime (DateTime)</p>
            </li>
            <li>
               <p>InstalledBy (string)</p>
            </li>
         </ul> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_patches outputs
instance_patches_id = instance_patches.id
instance_patches_patches = instance_patches.patches
instance_patches_next_token = instance_patches.next_token
```

---


### Inventory_deletions

InventoryDeletions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inventory_deletions` | Vec<String> | <p>A list of status items for deleted inventory.</p> |
| `next_token` | String | <p>The token for the next set of items to return. Use this token to get the next set of
   results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access inventory_deletions outputs
inventory_deletions_id = inventory_deletions.id
inventory_deletions_inventory_deletions = inventory_deletions.inventory_deletions
inventory_deletions_next_token = inventory_deletions.next_token
```

---


### Maintenance_windows

MaintenanceWindows resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `window_identities` | Vec<String> | <p>Information about the maintenance windows.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to
   return, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access maintenance_windows outputs
maintenance_windows_id = maintenance_windows.id
maintenance_windows_window_identities = maintenance_windows.window_identities
maintenance_windows_next_token = maintenance_windows.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple document_default_version resources
document_default_version_0 = provider.ssm.Document_default_version {
    name = "value-0"
    document_version = "value-0"
}
document_default_version_1 = provider.ssm.Document_default_version {
    name = "value-1"
    document_version = "value-1"
}
document_default_version_2 = provider.ssm.Document_default_version {
    name = "value-2"
    document_version = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    document_default_version = provider.ssm.Document_default_version {
        name = "production-value"
        document_version = "production-value"
    }
```

---

## Related Documentation

- [AWS Ssm Documentation](https://docs.aws.amazon.com/ssm/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
