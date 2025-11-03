# Cloudformation Service



**Resources**: 25

---

## Overview

The cloudformation service provides access to 25 resource types:

- [Type_registration](#type_registration) [R]
- [Change_set](#change_set) [CRD]
- [Resource_scan](#resource_scan) [R]
- [Organizations_access](#organizations_access) [R]
- [Stacks](#stacks) [R]
- [Stack_set_operation](#stack_set_operation) [R]
- [Stack](#stack) [CUD]
- [Stack_events](#stack_events) [R]
- [Publisher](#publisher) [R]
- [Stack_policy](#stack_policy) [R]
- [Stack_resources](#stack_resources) [R]
- [Change_set_hooks](#change_set_hooks) [R]
- [Stack_refactor](#stack_refactor) [CR]
- [Type](#type) [R]
- [Stack_resource](#stack_resource) [R]
- [Termination_protection](#termination_protection) [U]
- [Stack_set](#stack_set) [CRUD]
- [Stack_resource_drifts](#stack_resource_drifts) [R]
- [Template_summary](#template_summary) [R]
- [Stack_instances](#stack_instances) [CUD]
- [Stack_drift_detection_status](#stack_drift_detection_status) [R]
- [Account_limits](#account_limits) [R]
- [Stack_instance](#stack_instance) [R]
- [Template](#template) [R]
- [Generated_template](#generated_template) [CRUD]

---

## Resources


### Type_registration

TypeRegistration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `progress_status` | String | <p>The current status of the extension registration request.</p> |
| `type_version_arn` | String | <p>The Amazon Resource Name (ARN) of this specific version of the extension being
      registered.</p>
         <p>For registration requests with a <code>ProgressStatus</code> of other than
        <code>COMPLETE</code>, this will be <code>null</code>.</p> |
| `type_arn` | String | <p>The Amazon Resource Name (ARN) of the extension being registered.</p>
         <p>For registration requests with a <code>ProgressStatus</code> of other than
        <code>COMPLETE</code>, this will be <code>null</code>.</p> |
| `description` | String | <p>The description of the extension registration request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access type_registration outputs
type_registration_id = type_registration.id
type_registration_progress_status = type_registration.progress_status
type_registration_type_version_arn = type_registration.type_version_arn
type_registration_type_arn = type_registration.type_arn
type_registration_description = type_registration.description
```

---


### Change_set

ChangeSet resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description to help you identify this change set.</p> |
| `on_stack_failure` | String |  | <p>Determines what action will be taken if stack creation fails. If this parameter is
      specified, the <code>DisableRollback</code> parameter to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ExecuteChangeSet.html">ExecuteChangeSet</a> API operation must not be specified. This must be one of these
      values:</p>
         <ul>
            <li>
               <p>
                  <code>DELETE</code> - Deletes the change set if the stack creation fails. This is only
          valid when the <code>ChangeSetType</code> parameter is set to <code>CREATE</code>. If the
          deletion of the stack fails, the status of the stack is <code>DELETE_FAILED</code>.</p>
            </li>
            <li>
               <p>
                  <code>DO_NOTHING</code> - if the stack creation fails, do nothing. This is equivalent
          to specifying <code>true</code> for the <code>DisableRollback</code> parameter to the
            <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ExecuteChangeSet.html">ExecuteChangeSet</a> API operation.</p>
            </li>
            <li>
               <p>
                  <code>ROLLBACK</code> - if the stack creation fails, roll back the stack. This is
          equivalent to specifying <code>false</code> for the <code>DisableRollback</code> parameter
          to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ExecuteChangeSet.html">ExecuteChangeSet</a> API operation.</p>
            </li>
         </ul>
         <p>For nested stacks, when the <code>OnStackFailure</code> parameter is set to
        <code>DELETE</code> for the change set for the parent stack, any failure in a child stack
      will cause the parent stack creation to fail and all stacks to be deleted.</p> |
| `notification_ar_ns` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) of Amazon SNS topics that CloudFormation associates with the
      stack. To remove all associated notification topics, specify an empty list.</p> |
| `import_existing_resources` | bool |  | <p>Indicates if the change set auto-imports resources that already exist. For more
      information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/import-resources-automatically.html">Import Amazon Web Services
        resources into a CloudFormation stack automatically</a> in the
        <i>CloudFormation User Guide</i>.</p>
         <note>
            <p>This parameter can only import resources that have custom names in templates. For more
        information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-properties-name.html">name
          type</a> in the <i>CloudFormation User Guide</i>. To import resources that do not
        accept custom names, such as EC2 instances, use the <code>ResourcesToImport</code> parameter
        instead.</p>
         </note> |
| `resources_to_import` | Vec<String> |  | <p>The resources to import into your stack.</p> |
| `stack_name` | String | ✅ | <p>The name or the unique ID of the stack for which you are creating a change set. CloudFormation
      generates the change set by comparing this stack's information with the information that you
      submit, such as a modified template or different parameter input values.</p> |
| `include_nested_stacks` | bool |  | <p>Creates a change set for the all nested stacks specified in the template. The default
      behavior of this action is set to <code>False</code>. To include nested sets in a change set,
      specify <code>True</code>.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role that CloudFormation assumes when executing the
      change set. CloudFormation uses the role's credentials to make calls on your behalf. CloudFormation
      uses this role for all future operations on the stack. Provided that users have permission to
      operate on the stack, CloudFormation uses this role even if the users don't have permission to
      pass it. Ensure that the role grants least permission.</p>
         <p>If you don't specify a value, CloudFormation uses the role that was previously associated with
      the stack. If no role is available, CloudFormation uses a temporary session that is generated from
      your user credentials.</p> |
| `rollback_configuration` | String |  | <p>The rollback triggers for CloudFormation to monitor during stack creation and updating
      operations, and for the specified monitoring period afterwards.</p> |
| `change_set_type` | String |  | <p>The type of change set operation. To create a change set for a new stack, specify
        <code>CREATE</code>. To create a change set for an existing stack, specify
        <code>UPDATE</code>. To create a change set for an import operation, specify
        <code>IMPORT</code>.</p>
         <p>If you create a change set for a new stack, CloudFormation creates a stack with a unique stack
      ID, but no template or resources. The stack will be in the <code>REVIEW_IN_PROGRESS</code>
      state until you execute the change set.</p>
         <p>By default, CloudFormation specifies <code>UPDATE</code>. You can't use the
        <code>UPDATE</code> type to create a change set for a new stack or the <code>CREATE</code>
      type to create a change set for an existing stack.</p> |
| `use_previous_template` | bool |  | <p>Whether to reuse the template that's associated with the stack to create the change
      set.</p> |
| `template_url` | String |  | <p>The URL of the file that contains the revised template. The URL must point to a template
      (max size: 1 MB) that's located in an Amazon S3 bucket or a Systems Manager document. CloudFormation
      generates the change set by comparing this template with the stack that you specified. The
      location for an Amazon S3 bucket must start with <code>https://</code>. URLs from S3 static
      websites are not supported.</p>
         <p>Conditional: You must specify only <code>TemplateBody</code> or
      <code>TemplateURL</code>.</p> |
| `template_body` | String |  | <p>A structure that contains the body of the revised template, with a minimum length of 1
      byte and a maximum length of 51,200 bytes. CloudFormation generates the change set by comparing
      this template with the template of the stack that you specified.</p>
         <p>Conditional: You must specify only <code>TemplateBody</code> or
      <code>TemplateURL</code>.</p> |
| `parameters` | Vec<String> |  | <p>A list of <code>Parameter</code> structures that specify input parameters for the change
      set. For more information, see the <a>Parameter</a> data type.</p> |
| `resource_types` | Vec<String> |  | <p>The template resource types that you have permissions to work with if you execute this
      change set, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or
        <code>Custom::MyCustomInstance</code>.</p>
         <p>If the list of resource types doesn't include a resource type that you're updating, the
      stack update fails. By default, CloudFormation grants permissions to all resource types. IAM
      uses this parameter for condition keys in IAM policies for CloudFormation. For more information,
      see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/control-access-with-iam.html">Control access with
        Identity and Access Management</a> in the <i>CloudFormation User Guide</i>.</p>
         <note>
            <p>Only one of the <code>Capabilities</code> and <code>ResourceType</code> parameters can
        be specified.</p>
         </note> |
| `change_set_name` | String | ✅ | <p>The name of the change set. The name must be unique among all change sets that are
      associated with the specified stack.</p>
         <p>A change set name can contain only alphanumeric, case sensitive characters, and hyphens.
      It must start with an alphabetical character and can't exceed 128 characters.</p> |
| `client_token` | String |  | <p>A unique identifier for this <code>CreateChangeSet</code> request. Specify this token if
      you plan to retry requests so that CloudFormation knows that you're not attempting to create
      another change set with the same name. You might retry <code>CreateChangeSet</code> requests
      to ensure that CloudFormation successfully received them.</p> |
| `tags` | Vec<String> |  | <p>Key-value pairs to associate with this stack. CloudFormation also propagates these tags to
      resources in the stack. You can specify a maximum of 50 tags.</p> |
| `capabilities` | Vec<String> |  | <p>In some cases, you must explicitly acknowledge that your stack template contains certain
      capabilities in order for CloudFormation to create the stack.</p>
         <ul>
            <li>
               <p>
                  <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>
               </p>
               <p>Some stack templates might include resources that can affect permissions in your
          Amazon Web Services account; for example, by creating new IAM users. For those stacks, you must
          explicitly acknowledge this by specifying one of these capabilities.</p>
               <p>The following IAM resources require you to specify either the
            <code>CAPABILITY_IAM</code> or <code>CAPABILITY_NAMED_IAM</code> capability.</p>
               <ul>
                  <li>
                     <p>If you have IAM resources, you can specify either capability.</p>
                  </li>
                  <li>
                     <p>If you have IAM resources with custom names, you <i>must</i>
              specify <code>CAPABILITY_NAMED_IAM</code>.</p>
                  </li>
                  <li>
                     <p>If you don't specify either of these capabilities, CloudFormation returns an
                <code>InsufficientCapabilities</code> error.</p>
                  </li>
               </ul>
               <p>If your stack template contains these resources, we suggest that you review all
          permissions associated with them and edit their permissions if necessary.</p>
               <ul>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-accesskey.html">
                AWS::IAM::AccessKey</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-group.html">
                AWS::IAM::Group</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-managedpolicy.html"> AWS::IAM::ManagedPolicy</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-policy.html">
                AWS::IAM::Policy</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-role.html">
                AWS::IAM::Role</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-user.html">
                AWS::IAM::User</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-usertogroupaddition.html">AWS::IAM::UserToGroupAddition</a>
                     </p>
                  </li>
               </ul>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/control-access-with-iam.html#using-iam-capabilities">Acknowledging IAM resources in CloudFormation templates</a>.</p>
            </li>
            <li>
               <p>
                  <code>CAPABILITY_AUTO_EXPAND</code>
               </p>
               <p>Some template contain macros. Macros perform custom processing on templates; this can
          include simple actions like find-and-replace operations, all the way to extensive
          transformations of entire templates. Because of this, users typically create a change set
          from the processed template, so that they can review the changes resulting from the macros
          before actually creating the stack. If your stack template contains one or more macros,
          and you choose to create a stack directly from the processed template, without first
          reviewing the resulting changes in a change set, you must acknowledge this capability.
          This includes the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-include.html">AWS::Include</a>
          and <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by CloudFormation.</p>
               <note>
                  <p>This capacity doesn't apply to creating change sets, and specifying it when creating
            change sets has no effect.</p>
                  <p>If you want to create a stack from a stack template that contains macros
              <i>and</i> nested stacks, you must create or update the stack directly
            from the template using the <a>CreateStack</a> or <a>UpdateStack</a> action, and specifying this capability.</p>
               </note>
               <p>For more information about macros, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Perform custom processing
            on CloudFormation templates with template macros</a>.</p>
            </li>
         </ul>
         <note>
            <p>Only one of the <code>Capabilities</code> and <code>ResourceType</code> parameters can
        be specified.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `include_nested_stacks` | bool | <p>Verifies if <code>IncludeNestedStacks</code> is set to <code>True</code>.</p> |
| `capabilities` | Vec<String> | <p>If you execute the change set, the list of capabilities that were explicitly acknowledged
      when the change set was created.</p> |
| `change_set_id` | String | <p>The Amazon Resource Name (ARN) of the change set.</p> |
| `stack_id` | String | <p>The Amazon Resource Name (ARN) of the stack that's associated with the change set.</p> |
| `root_change_set_id` | String | <p>Specifies the change set ID of the root change set in the current nested change set
      hierarchy.</p> |
| `notification_ar_ns` | Vec<String> | <p>The ARNs of the Amazon SNS topics that will be associated with the stack if you execute the
      change set.</p> |
| `import_existing_resources` | bool | <p>Indicates if the change set imports resources that already exist.</p>
         <note>
            <p>This parameter can only import resources that have <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-properties-name.html">custom
          names</a> in templates. To import resources that do not accept custom names, such as
        EC2 instances, use the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/resource-import.html">resource import</a>
        feature instead.</p>
         </note> |
| `creation_time` | String | <p>The start time when the change set was created, in UTC.</p> |
| `description` | String | <p>Information about the change set.</p> |
| `tags` | Vec<String> | <p>If you execute the change set, the tags that will be associated with the stack.</p> |
| `changes` | Vec<String> | <p>A list of <code>Change</code> structures that describes the resources CloudFormation changes
      if you execute the change set.</p> |
| `status` | String | <p>The current status of the change set, such as <code>CREATE_PENDING</code>,
        <code>CREATE_COMPLETE</code>, or <code>FAILED</code>.</p> |
| `next_token` | String | <p>If the output exceeds 1 MB, a string that identifies the next page of changes. If there is
      no additional page, this value is null.</p> |
| `stack_name` | String | <p>The name of the stack that's associated with the change set.</p> |
| `on_stack_failure` | String | <p>Determines what action will be taken if stack creation fails. When this parameter is
      specified, the <code>DisableRollback</code> parameter to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ExecuteChangeSet.html">ExecuteChangeSet</a> API operation must not be specified. This must be one of these
      values:</p>
         <ul>
            <li>
               <p>
                  <code>DELETE</code> - Deletes the change set if the stack creation fails. This is only
          valid when the <code>ChangeSetType</code> parameter is set to <code>CREATE</code>. If the
          deletion of the stack fails, the status of the stack is <code>DELETE_FAILED</code>.</p>
            </li>
            <li>
               <p>
                  <code>DO_NOTHING</code> - if the stack creation fails, do nothing. This is equivalent
          to specifying <code>true</code> for the <code>DisableRollback</code> parameter to the
            <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ExecuteChangeSet.html">ExecuteChangeSet</a> API operation.</p>
            </li>
            <li>
               <p>
                  <code>ROLLBACK</code> - if the stack creation fails, roll back the stack. This is
          equivalent to specifying <code>false</code> for the <code>DisableRollback</code> parameter
          to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ExecuteChangeSet.html">ExecuteChangeSet</a> API operation.</p>
            </li>
         </ul> |
| `parent_change_set_id` | String | <p>Specifies the change set ID of the parent change set in the current nested change set
      hierarchy.</p> |
| `execution_status` | String | <p>If the change set execution status is <code>AVAILABLE</code>, you can execute the change
      set. If you can't execute the change set, the status indicates why. For example, a change set
      might be in an <code>UNAVAILABLE</code> state because CloudFormation is still creating it or in an
        <code>OBSOLETE</code> state because the stack was already updated.</p> |
| `parameters` | Vec<String> | <p>A list of <code>Parameter</code> structures that describes the input parameters and their
      values used to create the change set. For more information, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p> |
| `change_set_name` | String | <p>The name of the change set.</p> |
| `status_reason` | String | <p>A description of the change set's status. For example, if your attempt to create a change
      set failed, CloudFormation shows the error message.</p> |
| `rollback_configuration` | String | <p>The rollback triggers for CloudFormation to monitor during stack creation and updating
      operations, and for the specified monitoring period afterwards.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create change_set
change_set = provider.cloudformation.Change_set {
    stack_name = "value"  # <p>The name or the unique ID of the stack for which you are creating a change set. CloudFormation
      generates the change set by comparing this stack's information with the information that you
      submit, such as a modified template or different parameter input values.</p>
    change_set_name = "value"  # <p>The name of the change set. The name must be unique among all change sets that are
      associated with the specified stack.</p>
         <p>A change set name can contain only alphanumeric, case sensitive characters, and hyphens.
      It must start with an alphabetical character and can't exceed 128 characters.</p>
}

# Access change_set outputs
change_set_id = change_set.id
change_set_include_nested_stacks = change_set.include_nested_stacks
change_set_capabilities = change_set.capabilities
change_set_change_set_id = change_set.change_set_id
change_set_stack_id = change_set.stack_id
change_set_root_change_set_id = change_set.root_change_set_id
change_set_notification_ar_ns = change_set.notification_ar_ns
change_set_import_existing_resources = change_set.import_existing_resources
change_set_creation_time = change_set.creation_time
change_set_description = change_set.description
change_set_tags = change_set.tags
change_set_changes = change_set.changes
change_set_status = change_set.status
change_set_next_token = change_set.next_token
change_set_stack_name = change_set.stack_name
change_set_on_stack_failure = change_set.on_stack_failure
change_set_parent_change_set_id = change_set.parent_change_set_id
change_set_execution_status = change_set.execution_status
change_set_parameters = change_set.parameters
change_set_change_set_name = change_set.change_set_name
change_set_status_reason = change_set.status_reason
change_set_rollback_configuration = change_set.rollback_configuration
```

---


### Resource_scan

ResourceScan resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | <p>The time that the resource scan was finished.</p> |
| `resources_scanned` | i64 | <p>The number of resources that were listed. This is only available for scans with a
        <code>Status</code> set to <code>COMPLETE</code>, <code>EXPIRED</code>, or <code>FAILED
      </code>.</p> |
| `status_reason` | String | <p>The reason for the resource scan status, providing more information if a failure
      happened.</p> |
| `status` | String | <p>Status of the resource scan.</p>
         <dl>
            <dt>
          
            IN_PROGRESS
          
        </dt>
            <dd>
               <p>The resource scan is still in progress.</p>
            </dd>
            <dt>
          
            COMPLETE
          
        </dt>
            <dd>
               <p>The resource scan is complete.</p>
            </dd>
            <dt>
          
            EXPIRED
          
        </dt>
            <dd>
               <p>The resource scan has expired.</p>
            </dd>
            <dt>
          
            FAILED
          
        </dt>
            <dd>
               <p>The resource scan has failed.</p>
            </dd>
         </dl> |
| `resources_read` | i64 | <p>The number of resources that were read. This is only available for scans with a
        <code>Status</code> set to <code>COMPLETE</code>, <code>EXPIRED</code>, or
        <code>FAILED</code>.</p>
         <note>
            <p>This field may be 0 if the resource scan failed with a
          <code>ResourceScanLimitExceededException</code>.</p>
         </note> |
| `scan_filters` | Vec<String> | <p>The scan filters that were used.</p> |
| `resource_types` | Vec<String> | <p>The list of resource types for the specified scan. Resource types are only available for
      scans with a <code>Status</code> set to <code>COMPLETE</code> or <code>FAILED </code>.</p> |
| `resource_scan_id` | String | <p>The Amazon Resource Name (ARN) of the resource scan. The format is
        <code>arn:${Partition}:cloudformation:${Region}:${Account}:resourceScan/${Id}</code>. An
      example is
          <code>arn:aws:cloudformation:<i>us-east-1</i>:<i>123456789012</i>:resourceScan/<i>f5b490f7-7ed4-428a-aa06-31ff25db0772</i>
            </code>.</p> |
| `start_time` | String | <p>The time that the resource scan was started.</p> |
| `percentage_completed` | f64 | <p>The percentage of the resource scan that has been completed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_scan outputs
resource_scan_id = resource_scan.id
resource_scan_end_time = resource_scan.end_time
resource_scan_resources_scanned = resource_scan.resources_scanned
resource_scan_status_reason = resource_scan.status_reason
resource_scan_status = resource_scan.status
resource_scan_resources_read = resource_scan.resources_read
resource_scan_scan_filters = resource_scan.scan_filters
resource_scan_resource_types = resource_scan.resource_types
resource_scan_resource_scan_id = resource_scan.resource_scan_id
resource_scan_start_time = resource_scan.start_time
resource_scan_percentage_completed = resource_scan.percentage_completed
```

---


### Organizations_access

OrganizationsAccess resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Presents the status of the <code>OrganizationAccess</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organizations_access outputs
organizations_access_id = organizations_access.id
organizations_access_status = organizations_access.status
```

---


### Stacks

Stacks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If the output exceeds 1 MB in size, a string that identifies the next page of stacks. If
      no additional page exists, this value is null.</p> |
| `stacks` | Vec<String> | <p>A list of stack structures.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stacks outputs
stacks_id = stacks.id
stacks_next_token = stacks.next_token
stacks_stacks = stacks.stacks
```

---


### Stack_set_operation

StackSetOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_set_operation` | String | <p>The specified StackSet operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_set_operation outputs
stack_set_operation_id = stack_set_operation.id
stack_set_operation_stack_set_operation = stack_set_operation.stack_set_operation
```

---


### Stack

Stack resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_types` | Vec<String> |  | <p>The template resource types that you have permissions to work with for this create stack
      action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or
        <code>Custom::MyCustomInstance</code>. Use the following syntax to describe template
      resource types: <code>AWS::*</code> (for all Amazon Web Services resources), <code>Custom::*</code> (for all
      custom resources), <code>Custom::<i>logical_ID</i>
            </code> (for a specific custom resource),
        <code>AWS::<i>service_name</i>::*</code> (for all resources of a particular
      Amazon Web Services service), and
          <code>AWS::<i>service_name</i>::<i>resource_logical_ID</i>
            </code> (for a specific Amazon Web Services resource).</p>
         <p>If the list of resource types doesn't include a resource that you're creating, the stack
      creation fails. By default, CloudFormation grants permissions to all resource types. IAM uses
      this parameter for CloudFormation-specific condition keys in IAM policies. For more information,
      see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/control-access-with-iam.html">Control access with
        Identity and Access Management</a>.</p>
         <note>
            <p>Only one of the <code>Capabilities</code> and <code>ResourceType</code> parameters can
        be specified.</p>
         </note> |
| `tags` | Vec<String> |  | <p>Key-value pairs to associate with this stack. CloudFormation also propagates these tags to the
      resources created in the stack. A maximum number of 50 tags can be specified.</p> |
| `stack_policy_body` | String |  | <p>Structure that contains the stack policy body. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html">Prevent updates to stack resources</a> in the <i>CloudFormation User Guide</i>.
      You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code>
      parameter, but not both.</p> |
| `template_url` | String |  | <p>The URL of a file that contains the template body. The URL must point to a template (max
      size: 1 MB) that's located in an Amazon S3 bucket or a Systems Manager document. The location for
      an Amazon S3 bucket must start with <code>https://</code>. URLs from S3 static websites are not
      supported.</p>
         <p>Conditional: You must specify either the <code>TemplateBody</code> or the
        <code>TemplateURL</code> parameter, but not both.</p> |
| `notification_ar_ns` | Vec<String> |  | <p>The Amazon SNS topic ARNs to publish stack related events. You can find your Amazon SNS topic ARNs
      using the Amazon SNS console or your Command Line Interface (CLI).</p> |
| `disable_rollback` | bool |  | <p>Set to <code>true</code> to disable rollback of the stack if stack creation failed. You
      can specify either <code>DisableRollback</code> or <code>OnFailure</code>, but not
      both.</p>
         <p>Default: <code>false</code>
         </p> |
| `timeout_in_minutes` | i64 |  | <p>The amount of time that can pass before the stack status becomes
        <code>CREATE_FAILED</code>; if <code>DisableRollback</code> is not set or is set to
        <code>false</code>, the stack will be rolled back.</p> |
| `capabilities` | Vec<String> |  | <p>In some cases, you must explicitly acknowledge that your stack template contains certain
      capabilities in order for CloudFormation to create the stack.</p>
         <ul>
            <li>
               <p>
                  <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>
               </p>
               <p>Some stack templates might include resources that can affect permissions in your
          Amazon Web Services account; for example, by creating new IAM users. For those stacks, you must
          explicitly acknowledge this by specifying one of these capabilities.</p>
               <p>The following IAM resources require you to specify either the
            <code>CAPABILITY_IAM</code> or <code>CAPABILITY_NAMED_IAM</code> capability.</p>
               <ul>
                  <li>
                     <p>If you have IAM resources, you can specify either capability.</p>
                  </li>
                  <li>
                     <p>If you have IAM resources with custom names, you <i>must</i>
              specify <code>CAPABILITY_NAMED_IAM</code>.</p>
                  </li>
                  <li>
                     <p>If you don't specify either of these capabilities, CloudFormation returns an
                <code>InsufficientCapabilities</code> error.</p>
                  </li>
               </ul>
               <p>If your stack template contains these resources, we recommend that you review all
          permissions associated with them and edit their permissions if necessary.</p>
               <ul>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-accesskey.html">AWS::IAM::AccessKey</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-group.html">AWS::IAM::Group</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-managedpolicy.html"> AWS::IAM::ManagedPolicy</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-policy.html">AWS::IAM::Policy</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-role.html">AWS::IAM::Role</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-user.html">AWS::IAM::User</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-usertogroupaddition.html">AWS::IAM::UserToGroupAddition</a>
                     </p>
                  </li>
               </ul>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/control-access-with-iam.html#using-iam-capabilities">Acknowledging IAM resources in CloudFormation templates</a>.</p>
            </li>
            <li>
               <p>
                  <code>CAPABILITY_AUTO_EXPAND</code>
               </p>
               <p>Some template contain macros. Macros perform custom processing on templates; this can
          include simple actions like find-and-replace operations, all the way to extensive
          transformations of entire templates. Because of this, users typically create a change set
          from the processed template, so that they can review the changes resulting from the macros
          before actually creating the stack. If your stack template contains one or more macros,
          and you choose to create a stack directly from the processed template, without first
          reviewing the resulting changes in a change set, you must acknowledge this capability.
          This includes the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-include.html">AWS::Include</a>
          and <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by CloudFormation.</p>
               <p>If you want to create a stack from a stack template that contains macros
            <i>and</i> nested stacks, you must create the stack directly from the
          template using this capability.</p>
               <important>
                  <p>You should only create stacks directly from a stack template that contains macros if
            you know what processing the macro performs.</p>
                  <p>Each macro relies on an underlying Lambda service function for processing stack
            templates. Be aware that the Lambda function owner can update the function operation
            without CloudFormation being notified.</p>
               </important>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Perform custom processing
            on CloudFormation templates with template macros</a>.</p>
            </li>
         </ul>
         <note>
            <p>Only one of the <code>Capabilities</code> and <code>ResourceType</code> parameters can
        be specified.</p>
         </note> |
| `client_request_token` | String |  | <p>A unique identifier for this <code>CreateStack</code> request. Specify this token if you
      plan to retry requests so that CloudFormation knows that you're not attempting to create a stack
      with the same name. You might retry <code>CreateStack</code> requests to ensure that
      CloudFormation successfully received them.</p>
         <p>All events initiated by a given stack operation are assigned the same client request
      token, which you can use to track operations. For example, if you execute a
        <code>CreateStack</code> operation with the token <code>token1</code>, then all the
        <code>StackEvents</code> generated by that operation will have
        <code>ClientRequestToken</code> set as <code>token1</code>.</p>
         <p>In the console, stack operations display the client request token on the Events tab. Stack
      operations that are initiated from the console use the token format
        <i>Console-StackOperation-ID</i>, which helps you easily identify the stack
      operation . For example, if you create a stack using the console, each stack event would be
      assigned the same token in the following format:
        <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>.</p> |
| `rollback_configuration` | String |  | <p>The rollback triggers for CloudFormation to monitor during stack creation and updating
      operations, and for the specified monitoring period afterwards.</p> |
| `stack_name` | String | ✅ | <p>The name that's associated with the stack. The name must be unique in the Region in which
      you are creating the stack.</p>
         <note>
            <p>A stack name can contain only alphanumeric characters (case sensitive) and hyphens. It
        must start with an alphabetical character and can't be longer than 128 characters.</p>
         </note> |
| `stack_policy_url` | String |  | <p>Location of a file that contains the stack policy. The URL must point to a policy (maximum
      size: 16 KB) located in an S3 bucket in the same Region as the stack. The location for an Amazon S3
      bucket must start with <code>https://</code>. URLs from S3 static websites are not
      supported.</p>
         <p>You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code>
      parameter, but not both.</p> |
| `parameters` | Vec<String> |  | <p>A list of <code>Parameter</code> structures that specify input parameters for the stack.
      For more information, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data
      type.</p> |
| `retain_except_on_create` | bool |  | <p>When set to <code>true</code>, newly created resources are deleted when the operation
      rolls back. This includes newly created resources marked with a deletion policy of
        <code>Retain</code>.</p>
         <p>Default: <code>false</code>
         </p> |
| `enable_termination_protection` | bool |  | <p>Whether to enable termination protection on the specified stack. If a user attempts to
      delete a stack with termination protection enabled, the operation fails and the stack remains
      unchanged. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protect CloudFormation
        stacks from being deleted</a> in the <i>CloudFormation User Guide</i>. Termination
      protection is deactivated on stacks by default.</p>
         <p>For <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>,
      termination protection is set on the root stack and can't be changed directly on the nested
      stack.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role that CloudFormation assumes to create the
      stack. CloudFormation uses the role's credentials to make calls on your behalf. CloudFormation always
      uses this role for all future operations on the stack. Provided that users have permission to
      operate on the stack, CloudFormation uses this role even if the users don't have permission to
      pass it. Ensure that the role grants least privilege.</p>
         <p>If you don't specify a value, CloudFormation uses the role that was previously associated with
      the stack. If no role is available, CloudFormation uses a temporary session that's generated from
      your user credentials.</p> |
| `template_body` | String |  | <p>Structure that contains the template body with a minimum length of 1 byte and a maximum
      length of 51,200 bytes.</p>
         <p>Conditional: You must specify either <code>TemplateBody</code> or
      <code>TemplateURL</code>, but not both.</p> |
| `on_failure` | String |  | <p>Determines what action will be taken if stack creation fails. This must be one of:
        <code>DO_NOTHING</code>, <code>ROLLBACK</code>, or <code>DELETE</code>. You can specify
      either <code>OnFailure</code> or <code>DisableRollback</code>, but not both.</p>
         <note>
            <p>Although the default setting is <code>ROLLBACK</code>, there is one exception. This
        exception occurs when a StackSet attempts to deploy a stack instance and the stack instance
        fails to create successfully. In this case, the <code>CreateStack</code> call overrides the
        default setting and sets the value of <code>OnFailure</code> to <code>DELETE</code>.</p>
         </note>
         <p>Default: <code>ROLLBACK</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stack
stack = provider.cloudformation.Stack {
    stack_name = "value"  # <p>The name that's associated with the stack. The name must be unique in the Region in which
      you are creating the stack.</p>
         <note>
            <p>A stack name can contain only alphanumeric characters (case sensitive) and hyphens. It
        must start with an alphabetical character and can't be longer than 128 characters.</p>
         </note>
}

```

---


### Stack_events

StackEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_events` | Vec<String> | <p>A list of <code>StackEvents</code> structures.</p> |
| `next_token` | String | <p>If the output exceeds 1 MB in size, a string that identifies the next page of events. If
      no additional page exists, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_events outputs
stack_events_id = stack_events.id
stack_events_stack_events = stack_events.stack_events
stack_events_next_token = stack_events.next_token
```

---


### Publisher

Publisher resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publisher_status` | String | <p>Whether the publisher is verified. Currently, all registered publishers are
      verified.</p> |
| `identity_provider` | String | <p>The type of account used as the identity provider when registering this publisher with
      CloudFormation.</p> |
| `publisher_id` | String | <p>The ID of the extension publisher.</p> |
| `publisher_profile` | String | <p>The URL to the publisher's profile with the identity provider.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access publisher outputs
publisher_id = publisher.id
publisher_publisher_status = publisher.publisher_status
publisher_identity_provider = publisher.identity_provider
publisher_publisher_id = publisher.publisher_id
publisher_publisher_profile = publisher.publisher_profile
```

---


### Stack_policy

StackPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_policy_body` | String | <p>Structure that contains the stack policy body. (For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html">Prevent updates to stack resources</a> in the
      <i>CloudFormation User Guide</i>.)</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_policy outputs
stack_policy_id = stack_policy.id
stack_policy_stack_policy_body = stack_policy.stack_policy_body
```

---


### Stack_resources

StackResources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_resources` | Vec<String> | <p>A list of <code>StackResource</code> structures.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_resources outputs
stack_resources_id = stack_resources.id
stack_resources_stack_resources = stack_resources.stack_resources
```

---


### Change_set_hooks

ChangeSetHooks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_set_id` | String | <p>The change set identifier (stack ID).</p> |
| `next_token` | String | <p>Pagination token, <code>null</code> or empty if no more results.</p> |
| `stack_id` | String | <p>The stack identifier (stack ID).</p> |
| `change_set_name` | String | <p>The change set name.</p> |
| `hooks` | Vec<String> | <p>List of Hook objects.</p> |
| `stack_name` | String | <p>The stack name.</p> |
| `status` | String | <p>Provides the status of the change set hook.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access change_set_hooks outputs
change_set_hooks_id = change_set_hooks.id
change_set_hooks_change_set_id = change_set_hooks.change_set_id
change_set_hooks_next_token = change_set_hooks.next_token
change_set_hooks_stack_id = change_set_hooks.stack_id
change_set_hooks_change_set_name = change_set_hooks.change_set_name
change_set_hooks_hooks = change_set_hooks.hooks
change_set_hooks_stack_name = change_set_hooks.stack_name
change_set_hooks_status = change_set_hooks.status
```

---


### Stack_refactor

StackRefactor resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_mappings` | Vec<String> |  | <p>The mappings for the stack resource <code>Source</code> and stack resource
        <code>Destination</code>.</p> |
| `stack_definitions` | Vec<String> | ✅ | <p>The stacks being refactored.</p> |
| `description` | String |  | <p>A description to help you identify the stack refactor.</p> |
| `enable_stack_creation` | bool |  | <p>Determines if a new stack is created with the refactor.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The stack refactor operation status that's provided after calling the <a>CreateStackRefactor</a> action.</p> |
| `description` | String | <p>A description to help you identify the refactor.</p> |
| `execution_status` | String | <p>The stack refactor execution operation status that's provided after calling the <a>ExecuteStackRefactor</a> action.</p> |
| `stack_ids` | Vec<String> | <p>The unique ID for each stack.</p> |
| `execution_status_reason` | String | <p>A detailed explanation for the stack refactor <code>ExecutionStatus</code>.</p> |
| `status_reason` | String | <p>A detailed explanation for the stack refactor operation <code>Status</code>.</p> |
| `stack_refactor_id` | String | <p>The ID associated with the stack refactor created from the <a>CreateStackRefactor</a> action.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stack_refactor
stack_refactor = provider.cloudformation.Stack_refactor {
    stack_definitions = "value"  # <p>The stacks being refactored.</p>
}

# Access stack_refactor outputs
stack_refactor_id = stack_refactor.id
stack_refactor_status = stack_refactor.status
stack_refactor_description = stack_refactor.description
stack_refactor_execution_status = stack_refactor.execution_status
stack_refactor_stack_ids = stack_refactor.stack_ids
stack_refactor_execution_status_reason = stack_refactor.execution_status_reason
stack_refactor_status_reason = stack_refactor.status_reason
stack_refactor_stack_refactor_id = stack_refactor.stack_refactor_id
```

---


### Type

Type resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_schema` | String | <p>A JSON string that represent the current configuration data for the extension in this
      account and Region.</p>
         <p>To set the configuration data for an extension, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_SetTypeConfiguration.html">SetTypeConfiguration</a>.</p> |
| `latest_public_version` | String | <p>The latest version of a public extension <i>that is available</i> for
      use.</p>
         <p>This only applies if you specify a public extension, and you don't specify a version. For
      all other requests, CloudFormation returns <code>null</code>.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the extension.</p> |
| `is_activated` | bool | <p>Whether the extension is activated in the account and Region.</p>
         <p>This only applies to public third-party extensions. For all other extensions, CloudFormation
      returns <code>null</code>.</p> |
| `type_tests_status_description` | String | <p>The description of the test status. To return the extension test status of a specific
      extension version, you must specify <code>VersionId</code>.</p>
         <p>This applies only to registered private extension versions. CloudFormation doesn't return this
      information for public extensions, whether they are activated in your account.</p> |
| `logging_config` | String | <p>Contains logging configuration information for private extensions. This applies only to
      private extensions you have registered in your account. For public extensions, both those
      provided by Amazon Web Services and published by third parties, CloudFormation returns <code>null</code>. For
      more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_RegisterType.html">RegisterType</a>.</p> |
| `execution_role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM execution role used to register the extension.
      This applies only to private extensions you have registered in your account. For more
      information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_RegisterType.html">RegisterType</a>.</p>
         <p>If the registered extension calls any Amazon Web Services APIs, you must create an <i>
               <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html">IAM execution
        role</a>
            </i> that includes the necessary permissions to call those Amazon Web Services APIs,
      and provision that execution role in your account. CloudFormation then assumes that execution role
      to provide your extension with the appropriate credentials.</p> |
| `last_updated` | String | <p>When the specified extension version was registered. This applies only to:</p>
         <ul>
            <li>
               <p>Private extensions you have registered in your account. For more information, see
            <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_RegisterType.html">RegisterType</a>.</p>
            </li>
            <li>
               <p>Public extensions you have activated in your account with auto-update specified. For
          more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ActivateType.html">ActivateType</a>.</p>
            </li>
         </ul> |
| `original_type_name` | String | <p>For public extensions that have been activated for this account and Region, the type name
      of the public extension.</p>
         <p>If you specified a <code>TypeNameAlias</code> when enabling the extension in this account
      and Region, CloudFormation treats that alias as the extension's type name within the account and
      Region, not the type name of the public extension. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/registry-public.html#registry-public-enable-alias">Use aliases to refer to extensions</a> in the
      <i>CloudFormation User Guide</i>.</p> |
| `public_version_number` | String | <p>The version number of a public third-party extension.</p>
         <p>This applies only if you specify a public extension you have activated in your account, or
      specify a public extension without specifying a version. For all other extensions, CloudFormation
      returns <code>null</code>.</p> |
| `auto_update` | bool | <p>Whether CloudFormation automatically updates the extension in this account and Region when a
      new <i>minor</i> version is published by the extension publisher. Major versions
      released by the publisher must be manually updated. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/registry-public.html#registry-public-enable-auto">Automatically use new versions of extensions</a> in the
        <i>CloudFormation User Guide</i>.</p> |
| `provisioning_type` | String | <p>For resource type extensions, the provisioning behavior of the resource type. CloudFormation
      determines the provisioning type during registration, based on the types of handlers in the
      schema handler package submitted.</p>
         <p>Valid values include:</p>
         <ul>
            <li>
               <p>
                  <code>FULLY_MUTABLE</code>: The resource type includes an update handler to process
          updates to the type during stack update operations.</p>
            </li>
            <li>
               <p>
                  <code>IMMUTABLE</code>: The resource type doesn't include an update handler, so the
          type can't be updated and must instead be replaced during stack update operations.</p>
            </li>
            <li>
               <p>
                  <code>NON_PROVISIONABLE</code>: The resource type doesn't include all the following
          handlers, and therefore can't actually be provisioned.</p>
               <ul>
                  <li>
                     <p>create</p>
                  </li>
                  <li>
                     <p>read</p>
                  </li>
                  <li>
                     <p>delete</p>
                  </li>
               </ul>
            </li>
         </ul> |
| `required_activated_types` | Vec<String> | <p>For extensions that are modules, the public third-party extensions that must be activated
      in your account in order for the module itself to be activated.</p> |
| `schema` | String | <p>The schema that defines the extension.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">Resource type
        schema</a> in the <i>CloudFormation Command Line Interface (CLI) User Guide</i> and the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/hooks-userguide/what-is-cloudformation-hooks.html">CloudFormation
        Hooks User Guide</a>.</p> |
| `publisher_id` | String | <p>The publisher ID of the extension publisher.</p>
         <p>This applies only to public third-party extensions. For private registered extensions, and
      extensions provided by Amazon Web Services, CloudFormation returns <code>null</code>.</p> |
| `time_created` | String | <p>When the specified private extension version was registered or activated in your
      account.</p> |
| `default_version_id` | String | <p>The ID of the default version of the extension. The default version is used when the
      extension version isn't specified.</p>
         <p>This applies only to private extensions you have registered in your account. For public
      extensions, both those provided by Amazon Web Services and published by third parties, CloudFormation returns
        <code>null</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_RegisterType.html">RegisterType</a>.</p>
         <p>To set the default version of an extension, use <a>SetTypeDefaultVersion</a>.</p> |
| `is_default_version` | bool | <p>Whether the specified extension version is set as the default version.</p>
         <p>This applies only to private extensions you have registered in your account, and
      extensions published by Amazon Web Services. For public third-party extensions, whether they are activated
      in your account, CloudFormation returns <code>null</code>.</p> |
| `description` | String | <p>The description of the extension.</p> |
| `documentation_url` | String | <p>The URL of a page providing detailed documentation for this extension.</p> |
| `type_name` | String | <p>The name of the extension.</p>
         <p>If the extension is a public third-party type you have activated with a type name alias,
      CloudFormation returns the type name alias. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_ActivateType.html">ActivateType</a>.</p> |
| `type` | String | <p>The kind of extension.</p> |
| `original_type_arn` | String | <p>For public extensions that have been activated for this account and Region, the Amazon
      Resource Name (ARN) of the public extension.</p> |
| `deprecated_status` | String | <p>The deprecation status of the extension version.</p>
         <p>Valid values include:</p>
         <ul>
            <li>
               <p>
                  <code>LIVE</code>: The extension is activated or registered and can be used in
          CloudFormation operations, dependent on its provisioning behavior and visibility scope.</p>
            </li>
            <li>
               <p>
                  <code>DEPRECATED</code>: The extension has been deactivated or deregistered and can no
          longer be used in CloudFormation operations.</p>
            </li>
         </ul>
         <p>For public third-party extensions, CloudFormation returns <code>null</code>.</p> |
| `visibility` | String | <p>The scope at which the extension is visible and usable in CloudFormation operations.</p>
         <p>Valid values include:</p>
         <ul>
            <li>
               <p>
                  <code>PRIVATE</code>: The extension is only visible and usable within the account in
          which it is registered. CloudFormation marks any extensions you register as
            <code>PRIVATE</code>.</p>
            </li>
            <li>
               <p>
                  <code>PUBLIC</code>: The extension is publicly visible and usable within any Amazon Web Services
          account.</p>
            </li>
         </ul> |
| `source_url` | String | <p>The URL of the source code for the extension.</p> |
| `type_tests_status` | String | <p>The contract test status of the registered extension version. To return the extension test
      status of a specific extension version, you must specify <code>VersionId</code>.</p>
         <p>This applies only to registered private extension versions. CloudFormation doesn't return this
      information for public extensions, whether they are activated in your account.</p>
         <ul>
            <li>
               <p>
                  <code>PASSED</code>: The extension has passed all its contract tests.</p>
               <p>An extension must have a test status of <code>PASSED</code> before it can be
          published. For more information, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-publish.html">Publishing
            extensions to make them available for public use</a> in the
            <i>CloudFormation Command Line Interface (CLI) User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code>: The extension has failed one or more contract tests.</p>
            </li>
            <li>
               <p>
                  <code>IN_PROGRESS</code>: Contract tests are currently being performed on the
          extension.</p>
            </li>
            <li>
               <p>
                  <code>NOT_TESTED</code>: Contract tests haven't been performed on the
          extension.</p>
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

# Access type outputs
type_id = type.id
type_configuration_schema = type.configuration_schema
type_latest_public_version = type.latest_public_version
type_arn = type.arn
type_is_activated = type.is_activated
type_type_tests_status_description = type.type_tests_status_description
type_logging_config = type.logging_config
type_execution_role_arn = type.execution_role_arn
type_last_updated = type.last_updated
type_original_type_name = type.original_type_name
type_public_version_number = type.public_version_number
type_auto_update = type.auto_update
type_provisioning_type = type.provisioning_type
type_required_activated_types = type.required_activated_types
type_schema = type.schema
type_publisher_id = type.publisher_id
type_time_created = type.time_created
type_default_version_id = type.default_version_id
type_is_default_version = type.is_default_version
type_description = type.description
type_documentation_url = type.documentation_url
type_type_name = type.type_name
type_type = type.type
type_original_type_arn = type.original_type_arn
type_deprecated_status = type.deprecated_status
type_visibility = type.visibility
type_source_url = type.source_url
type_type_tests_status = type.type_tests_status
```

---


### Stack_resource

StackResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_resource_detail` | String | <p>A <code>StackResourceDetail</code> structure that contains the description of the
      specified resource in the specified stack.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_resource outputs
stack_resource_id = stack_resource.id
stack_resource_stack_resource_detail = stack_resource.stack_resource_detail
```

---


### Termination_protection

TerminationProtection resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stack_name` | String | ✅ | <p>The name or unique ID of the stack for which you want to set termination
      protection.</p> |
| `enable_termination_protection` | bool | ✅ | <p>Whether to enable termination protection on the specified stack.</p> |



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


### Stack_set

StackSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `call_as` | String |  | <p>Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account.</p>
         <p>By default, <code>SELF</code> is specified. Use <code>SELF</code> for StackSets with
      self-managed permissions.</p>
         <ul>
            <li>
               <p>To create a StackSet with service-managed permissions while signed in to the management account, specify <code>SELF</code>.</p>
            </li>
            <li>
               <p>To create a StackSet with service-managed permissions while signed in to a delegated
          administrator account, specify <code>DELEGATED_ADMIN</code>.</p>
               <p>Your Amazon Web Services account must be registered as a delegated admin in the management account. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-delegated-admin.html">Register a
            delegated administrator</a> in the <i>CloudFormation User Guide</i>.</p>
            </li>
         </ul>
         <p>StackSets with service-managed permissions are created in the management account, including StackSets that are created by delegated administrators.</p>
         <p>Valid only if the permissions model is <code>SERVICE_MANAGED</code>.</p> |
| `template_url` | String |  | <p>The URL of a file that contains the template body. The URL must point to a template
      (maximum size: 1 MB) that's located in an Amazon S3 bucket or a Systems Manager document. The
      location for an Amazon S3 bucket must start with <code>https://</code>. S3 static website URLs are
      not supported.</p>
         <p>Conditional: You must specify either the <code>TemplateBody</code> or the
        <code>TemplateURL</code> parameter, but not both.</p> |
| `execution_role_name` | String |  | <p>The name of the IAM execution role to use to create the StackSet. If you do not specify
      an execution role, CloudFormation uses the <code>AWSCloudFormationStackSetExecutionRole</code>
      role for the StackSet operation.</p>
         <p>Specify an IAM role only if you are using customized execution roles to control which
      stack resources users and groups can include in their StackSets.</p>
         <p>Valid only if the permissions model is <code>SELF_MANAGED</code>.</p> |
| `client_request_token` | String |  | <p>A unique identifier for this <code>CreateStackSet</code> request. Specify this token if
      you plan to retry requests so that CloudFormation knows that you're not attempting to create
      another StackSet with the same name. You might retry <code>CreateStackSet</code> requests to
      ensure that CloudFormation successfully received them.</p>
         <p>If you don't specify an operation ID, the SDK generates one
      automatically.</p> |
| `managed_execution` | String |  | <p>Describes whether CloudFormation performs non-conflicting operations concurrently and queues
      conflicting operations.</p> |
| `stack_set_name` | String | ✅ | <p>The name to associate with the StackSet. The name must be unique in the Region where you
      create your StackSet.</p>
         <note>
            <p>A stack name can contain only alphanumeric characters (case-sensitive) and hyphens. It
        must start with an alphabetic character and can't be longer than 128 characters.</p>
         </note> |
| `description` | String |  | <p>A description of the StackSet. You can use the description to identify the StackSet's
      purpose or other important information.</p> |
| `template_body` | String |  | <p>The structure that contains the template body, with a minimum length of 1 byte and a
      maximum length of 51,200 bytes.</p>
         <p>Conditional: You must specify either the <code>TemplateBody</code> or the
        <code>TemplateURL</code> parameter, but not both.</p> |
| `parameters` | Vec<String> |  | <p>The input parameters for the StackSet template.</p> |
| `capabilities` | Vec<String> |  | <p>In some cases, you must explicitly acknowledge that your StackSet template contains
      certain capabilities in order for CloudFormation to create the StackSet and related stack
      instances.</p>
         <ul>
            <li>
               <p>
                  <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>
               </p>
               <p>Some stack templates might include resources that can affect permissions in your
          Amazon Web Services account; for example, by creating new IAM users. For those StackSets, you must
          explicitly acknowledge this by specifying one of these capabilities.</p>
               <p>The following IAM resources require you to specify either the
            <code>CAPABILITY_IAM</code> or <code>CAPABILITY_NAMED_IAM</code> capability.</p>
               <ul>
                  <li>
                     <p>If you have IAM resources, you can specify either capability.</p>
                  </li>
                  <li>
                     <p>If you have IAM resources with custom names, you <i>must</i>
              specify <code>CAPABILITY_NAMED_IAM</code>.</p>
                  </li>
                  <li>
                     <p>If you don't specify either of these capabilities, CloudFormation returns an
                <code>InsufficientCapabilities</code> error.</p>
                  </li>
               </ul>
               <p>If your stack template contains these resources, we recommend that you review all
          permissions associated with them and edit their permissions if necessary.</p>
               <ul>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-accesskey.html">AWS::IAM::AccessKey</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-group.html">AWS::IAM::Group</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-policy.html">AWS::IAM::Policy</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-role.html">AWS::IAM::Role</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-user.html">AWS::IAM::User</a>
                     </p>
                  </li>
                  <li>
                     <p>
                        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-iam-usertogroupaddition.html">AWS::IAM::UserToGroupAddition</a>
                     </p>
                  </li>
               </ul>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/control-access-with-iam.html#using-iam-capabilities">Acknowledging IAM resources in CloudFormation templates</a>.</p>
            </li>
            <li>
               <p>
                  <code>CAPABILITY_AUTO_EXPAND</code>
               </p>
               <p>Some templates reference macros. If your StackSet template references one or more
          macros, you must create the StackSet directly from the processed template, without first
          reviewing the resulting changes in a change set. To create the StackSet directly, you must
          acknowledge this capability. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Perform custom processing
            on CloudFormation templates with template macros</a>.</p>
               <important>
                  <p>StackSets with service-managed permissions don't currently support the use of macros
            in templates. (This includes the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-include.html">AWS::Include</a> and <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by CloudFormation.) Even if
            you specify this capability for a StackSet with service-managed permissions, if you
            reference a macro in your template the StackSet operation will fail.</p>
               </important>
            </li>
         </ul> |
| `administration_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role to use to create this StackSet.</p>
         <p>Specify an IAM role only if you are using customized administrator roles to control
      which users or groups can manage specific StackSets within the same administrator account. For
      more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs-self-managed.html">Grant
        self-managed permissions</a> in the <i>CloudFormation User Guide</i>.</p>
         <p>Valid only if the permissions model is <code>SELF_MANAGED</code>.</p> |
| `permission_model` | String |  | <p>Describes how the IAM roles required for StackSet operations are created. By default,
        <code>SELF-MANAGED</code> is specified.</p>
         <ul>
            <li>
               <p>With <code>self-managed</code> permissions, you must create the administrator and
          execution roles required to deploy to target accounts. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs-self-managed.html">Grant
            self-managed permissions</a>.</p>
            </li>
            <li>
               <p>With <code>service-managed</code> permissions, StackSets automatically creates the
          IAM roles required to deploy to accounts managed by Organizations. For more
          information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-activate-trusted-access.html">Activate trusted access for StackSets with Organizations</a>.</p>
            </li>
         </ul> |
| `auto_deployment` | String |  | <p>Describes whether StackSets automatically deploys to Organizations accounts that
      are added to the target organization or organizational unit (OU). For more information, see
        <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-manage-auto-deployment.html">Enable or disable automatic deployments for StackSets in Organizations</a>
      in the <i>CloudFormation User Guide</i>.</p>
         <p>Required if the permissions model is <code>SERVICE_MANAGED</code>. (Not used with
      self-managed permissions.)</p> |
| `stack_id` | String |  | <p>The stack ID you are importing into a new StackSet. Specify the Amazon Resource Name (ARN)
      of the stack.</p> |
| `tags` | Vec<String> |  | <p>The key-value pairs to associate with this StackSet and the stacks created from it.
      CloudFormation also propagates these tags to supported resources that are created in the stacks. A
      maximum number of 50 tags can be specified.</p>
         <p>If you specify tags as part of a <code>CreateStackSet</code> action, CloudFormation checks to
      see if you have the required IAM permission to tag resources. If you don't, the entire
        <code>CreateStackSet</code> action fails with an <code>access denied</code> error, and the
      StackSet is not created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_set` | String | <p>The specified StackSet.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stack_set
stack_set = provider.cloudformation.Stack_set {
    stack_set_name = "value"  # <p>The name to associate with the StackSet. The name must be unique in the Region where you
      create your StackSet.</p>
         <note>
            <p>A stack name can contain only alphanumeric characters (case-sensitive) and hyphens. It
        must start with an alphabetic character and can't be longer than 128 characters.</p>
         </note>
}

# Access stack_set outputs
stack_set_id = stack_set.id
stack_set_stack_set = stack_set.stack_set
```

---


### Stack_resource_drifts

StackResourceDrifts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_resource_drifts` | Vec<String> | <p>Drift information for the resources that have been checked for drift in the specified
      stack. This includes actual and expected configuration values for resources where CloudFormation
      detects drift.</p>
         <p>For a given stack, there will be one <code>StackResourceDrift</code> for each stack
      resource that has been checked for drift. Resources that haven't yet been checked for drift
      aren't included. Resources that do not currently support drift detection aren't checked, and
      so not included. For a list of resources that support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/resource-import-supported-resources.html">Resource
        type support for imports and drift detection</a>.</p> |
| `next_token` | String | <p>If the request doesn't return all the remaining results, <code>NextToken</code> is set to
      a token. To retrieve the next set of results, call <code>DescribeStackResourceDrifts</code>
      again and assign that token to the request object's <code>NextToken</code> parameter. If the
      request returns all results, <code>NextToken</code> is set to <code>null</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_resource_drifts outputs
stack_resource_drifts_id = stack_resource_drifts.id
stack_resource_drifts_stack_resource_drifts = stack_resource_drifts.stack_resource_drifts
stack_resource_drifts_next_token = stack_resource_drifts.next_token
```

---


### Template_summary

TemplateSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capabilities` | Vec<String> | <p>The capabilities found within the template. If your template contains IAM resources, you
      must specify the <code>CAPABILITY_IAM</code> or <code>CAPABILITY_NAMED_IAM</code> value for
      this parameter when you use the <a>CreateStack</a> or <a>UpdateStack</a>
      actions with your template; otherwise, those actions return an
        <code>InsufficientCapabilities</code> error.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/control-access-with-iam.html#using-iam-capabilities">Acknowledging IAM resources in CloudFormation templates</a>.</p> |
| `capabilities_reason` | String | <p>The list of resources that generated the values in the <code>Capabilities</code> response
      element.</p> |
| `metadata` | String | <p>The value that's defined for the <code>Metadata</code> property of the template.</p> |
| `declared_transforms` | Vec<String> | <p>A list of the transforms that are declared in the template.</p> |
| `resource_identifier_summaries` | Vec<String> | <p>A list of resource identifier summaries that describe the target resources of an import
      operation and the properties you can provide during the import to identify the target
      resources. For example, <code>BucketName</code> is a possible identifier property for an
        <code>AWS::S3::Bucket</code> resource.</p> |
| `parameters` | Vec<String> | <p>A list of parameter declarations that describe various properties for each
      parameter.</p> |
| `description` | String | <p>The value that's defined in the <code>Description</code> property of the template.</p> |
| `version` | String | <p>The Amazon Web Services template format version, which identifies the capabilities of the
      template.</p> |
| `warnings` | String | <p>An object that contains any warnings returned.</p> |
| `resource_types` | Vec<String> | <p>A list of all the template resource types that are defined in the template, such as
        <code>AWS::EC2::Instance</code>, <code>AWS::Dynamo::Table</code>, and
        <code>Custom::MyCustomInstance</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access template_summary outputs
template_summary_id = template_summary.id
template_summary_capabilities = template_summary.capabilities
template_summary_capabilities_reason = template_summary.capabilities_reason
template_summary_metadata = template_summary.metadata
template_summary_declared_transforms = template_summary.declared_transforms
template_summary_resource_identifier_summaries = template_summary.resource_identifier_summaries
template_summary_parameters = template_summary.parameters
template_summary_description = template_summary.description
template_summary_version = template_summary.version
template_summary_warnings = template_summary.warnings
template_summary_resource_types = template_summary.resource_types
```

---


### Stack_instances

StackInstances resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stack_set_name` | String | ✅ | <p>The name or unique ID of the StackSet that you want to create stack instances from.</p> |
| `deployment_targets` | String |  | <p>[Service-managed permissions] The Organizations accounts in which to create stack
      instances in the specified Amazon Web Services Regions.</p>
         <p>You can specify <code>Accounts</code> or <code>DeploymentTargets</code>, but not
      both.</p> |
| `accounts` | Vec<String> |  | <p>[Self-managed permissions] The account IDs of one or more Amazon Web Services accounts that you want to
      create stack instances in the specified Region(s) for.</p>
         <p>You can specify <code>Accounts</code> or <code>DeploymentTargets</code>, but not
      both.</p> |
| `regions` | Vec<String> | ✅ | <p>The names of one or more Amazon Web Services Regions where you want to create stack instances using the
      specified Amazon Web Services accounts.</p> |
| `call_as` | String |  | <p>[Service-managed permissions] Specifies whether you are acting as an account administrator
      in the organization's management account or as a delegated administrator in a
      member account.</p>
         <p>By default, <code>SELF</code> is specified. Use <code>SELF</code> for StackSets with
      self-managed permissions.</p>
         <ul>
            <li>
               <p>If you are signed in to the management account, specify
          <code>SELF</code>.</p>
            </li>
            <li>
               <p>If you are signed in to a delegated administrator account, specify
            <code>DELEGATED_ADMIN</code>.</p>
               <p>Your Amazon Web Services account must be registered as a delegated administrator in the management account. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-delegated-admin.html">Register a
            delegated administrator</a> in the <i>CloudFormation User Guide</i>.</p>
            </li>
         </ul> |
| `operation_id` | String |  | <p>The unique identifier for this StackSet operation.</p>
         <p>The operation ID also functions as an idempotency token, to ensure that CloudFormation
      performs the StackSet operation only once, even if you retry the request multiple times. You
      might retry StackSet operation requests to ensure that CloudFormation successfully received
      them.</p>
         <p>If you don't specify an operation ID, the SDK generates one
      automatically.</p>
         <p>Repeating this StackSet operation with a new operation ID retries all stack instances
      whose status is <code>OUTDATED</code>.</p> |
| `operation_preferences` | String |  | <p>Preferences for how CloudFormation performs this StackSet operation.</p> |
| `parameter_overrides` | Vec<String> |  | <p>A list of StackSet parameters whose values you want to override in the selected stack
      instances.</p>
         <p>Any overridden parameter values will be applied to all stack instances in the specified
      accounts and Amazon Web Services Regions. When specifying parameters and their values, be aware of how
      CloudFormation sets parameter values during stack instance operations:</p>
         <ul>
            <li>
               <p>To override the current value for a parameter, include the parameter and specify its
          value.</p>
            </li>
            <li>
               <p>To leave an overridden parameter set to its present value, include the parameter and
          specify <code>UsePreviousValue</code> as <code>true</code>. (You can't specify both a
          value and set <code>UsePreviousValue</code> to <code>true</code>.)</p>
            </li>
            <li>
               <p>To set an overridden parameter back to the value specified in the StackSet, specify a
          parameter list but don't include the parameter in the list.</p>
            </li>
            <li>
               <p>To leave all parameters set to their present values, don't specify this property at
          all.</p>
            </li>
         </ul>
         <p>During StackSet updates, any parameter values overridden for a stack instance aren't
      updated, but retain their overridden value.</p>
         <p>You can only override the parameter <i>values</i> that are specified in the
      StackSet; to add or delete a parameter itself, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a>
      to update the StackSet template.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stack_instances
stack_instances = provider.cloudformation.Stack_instances {
    stack_set_name = "value"  # <p>The name or unique ID of the StackSet that you want to create stack instances from.</p>
    regions = "value"  # <p>The names of one or more Amazon Web Services Regions where you want to create stack instances using the
      specified Amazon Web Services accounts.</p>
}

```

---


### Stack_drift_detection_status

StackDriftDetectionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_drift_detection_id` | String | <p>The ID of the drift detection results of this operation.</p>
         <p>CloudFormation generates new results, with a new drift detection ID, each time this operation
      is run. However, the number of reports CloudFormation retains for any given stack, and for how
      long, may vary.</p> |
| `stack_drift_status` | String | <p>Status of the stack's actual configuration compared to its expected configuration.</p>
         <ul>
            <li>
               <p>
                  <code>DRIFTED</code>: The stack differs from its expected template configuration. A
          stack is considered to have drifted if one or more of its resources have drifted.</p>
            </li>
            <li>
               <p>
                  <code>NOT_CHECKED</code>: CloudFormation hasn't checked if the stack differs from its
          expected template configuration.</p>
            </li>
            <li>
               <p>
                  <code>IN_SYNC</code>: The stack's actual configuration matches its expected template
          configuration.</p>
            </li>
            <li>
               <p>
                  <code>UNKNOWN</code>: CloudFormation could not run drift detection for a resource in the
          stack. See the <code>DetectionStatusReason</code> for details.</p>
            </li>
         </ul> |
| `stack_id` | String | <p>The ID of the stack.</p> |
| `detection_status` | String | <p>The status of the stack drift detection operation.</p>
         <ul>
            <li>
               <p>
                  <code>DETECTION_COMPLETE</code>: The stack drift detection operation has successfully
          completed for all resources in the stack that support drift detection. (Resources that
          don't currently support stack detection remain unchecked.)</p>
               <p>If you specified logical resource IDs for CloudFormation to use as a filter for the stack
          drift detection operation, only the resources with those logical IDs are checked for
          drift.</p>
            </li>
            <li>
               <p>
                  <code>DETECTION_FAILED</code>: The stack drift detection operation has failed for at
          least one resource in the stack. Results will be available for resources on which
          CloudFormation successfully completed drift detection.</p>
            </li>
            <li>
               <p>
                  <code>DETECTION_IN_PROGRESS</code>: The stack drift detection operation is currently
          in progress.</p>
            </li>
         </ul> |
| `detection_status_reason` | String | <p>The reason the stack drift detection operation has its current status.</p> |
| `drifted_stack_resource_count` | i64 | <p>Total number of stack resources that have drifted. This is NULL until the drift detection
      operation reaches a status of <code>DETECTION_COMPLETE</code>. This value will be 0 for stacks
      whose drift status is <code>IN_SYNC</code>.</p> |
| `timestamp` | String | <p>Time at which the stack drift detection operation was initiated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_drift_detection_status outputs
stack_drift_detection_status_id = stack_drift_detection_status.id
stack_drift_detection_status_stack_drift_detection_id = stack_drift_detection_status.stack_drift_detection_id
stack_drift_detection_status_stack_drift_status = stack_drift_detection_status.stack_drift_status
stack_drift_detection_status_stack_id = stack_drift_detection_status.stack_id
stack_drift_detection_status_detection_status = stack_drift_detection_status.detection_status
stack_drift_detection_status_detection_status_reason = stack_drift_detection_status.detection_status_reason
stack_drift_detection_status_drifted_stack_resource_count = stack_drift_detection_status.drifted_stack_resource_count
stack_drift_detection_status_timestamp = stack_drift_detection_status.timestamp
```

---


### Account_limits

AccountLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_limits` | Vec<String> | <p>An account limit structure that contain a list of CloudFormation account limits and their
      values.</p> |
| `next_token` | String | <p>If the output exceeds 1 MB in size, a string that identifies the next page of limits. If
      no additional page exists, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_limits outputs
account_limits_id = account_limits.id
account_limits_account_limits = account_limits.account_limits
account_limits_next_token = account_limits.next_token
```

---


### Stack_instance

StackInstance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stack_instance` | String | <p>The stack instance that matches the specified request parameters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stack_instance outputs
stack_instance_id = stack_instance.id
stack_instance_stack_instance = stack_instance.stack_instance
```

---


### Template

Template resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_body` | String | <p>Structure that contains the template body.</p>
         <p>CloudFormation returns the same template that was used when the stack was created.</p> |
| `stages_available` | Vec<String> | <p>The stage of the template that you can retrieve. For stacks, the <code>Original</code> and
        <code>Processed</code> templates are always available. For change sets, the
        <code>Original</code> template is always available. After CloudFormation finishes creating the
      change set, the <code>Processed</code> template becomes available.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access template outputs
template_id = template.id
template_template_body = template.template_body
template_stages_available = template.stages_available
```

---


### Generated_template

GeneratedTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resources` | Vec<String> |  | <p>An optional list of resources to be included in the generated template.</p>
         <p>If no resources are specified,the template will be created without any resources.
      Resources can be added to the template using the <code>UpdateGeneratedTemplate</code> API
      action.</p> |
| `generated_template_name` | String | ✅ | <p>The name assigned to the generated template.</p> |
| `stack_name` | String |  | <p>An optional name or ARN of a stack to use as the base stack for the generated
      template.</p> |
| `template_configuration` | String |  | <p>The configuration details of the generated template, including the
        <code>DeletionPolicy</code> and <code>UpdateReplacePolicy</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the template generation. Supported values are:</p>
         <ul>
            <li>
               <p>
                  <code>CreatePending</code> - the creation of the template is pending.</p>
            </li>
            <li>
               <p>
                  <code>CreateInProgress</code> - the creation of the template is in progress.</p>
            </li>
            <li>
               <p>
                  <code>DeletePending</code> - the deletion of the template is pending.</p>
            </li>
            <li>
               <p>
                  <code>DeleteInProgress</code> - the deletion of the template is in progress.</p>
            </li>
            <li>
               <p>
                  <code>UpdatePending</code> - the update of the template is pending.</p>
            </li>
            <li>
               <p>
                  <code>UpdateInProgress</code> - the update of the template is in progress.</p>
            </li>
            <li>
               <p>
                  <code>Failed</code> - the template operation failed.</p>
            </li>
            <li>
               <p>
                  <code>Complete</code> - the template operation is complete.</p>
            </li>
         </ul> |
| `template_body` | String | <p>The template body of the generated template, in the language specified by the
        <code>Language</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create generated_template
generated_template = provider.cloudformation.Generated_template {
    generated_template_name = "value"  # <p>The name assigned to the generated template.</p>
}

# Access generated_template outputs
generated_template_id = generated_template.id
generated_template_status = generated_template.status
generated_template_template_body = generated_template.template_body
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple type_registration resources
type_registration_0 = provider.cloudformation.Type_registration {
}
type_registration_1 = provider.cloudformation.Type_registration {
}
type_registration_2 = provider.cloudformation.Type_registration {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    type_registration = provider.cloudformation.Type_registration {
    }
```

---

## Related Documentation

- [AWS Cloudformation Documentation](https://docs.aws.amazon.com/cloudformation/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
