# Connect Service



**Resources**: 81

---

## Overview

The connect service provides access to 81 resource types:

- [User_hierarchy](#user_hierarchy) [U]
- [User](#user) [CRD]
- [User_security_profiles](#user_security_profiles) [U]
- [Instance_attribute](#instance_attribute) [RU]
- [Contact_attributes](#contact_attributes) [RU]
- [Current_metric_data](#current_metric_data) [R]
- [Metric_data](#metric_data) [R]
- [Security_profile](#security_profile) [CRUD]
- [Use_case](#use_case) [CD]
- [Contact_metrics](#contact_metrics) [R]
- [Attached_file](#attached_file) [RD]
- [User_identity_info](#user_identity_info) [U]
- [User_hierarchy_group_name](#user_hierarchy_group_name) [U]
- [User_phone_config](#user_phone_config) [U]
- [View_content](#view_content) [U]
- [Contact_routing_data](#contact_routing_data) [U]
- [Flow_association](#flow_association) [R]
- [Email_address_metadata](#email_address_metadata) [U]
- [Email_address](#email_address) [CRD]
- [Queue](#queue) [CRD]
- [Evaluation_form](#evaluation_form) [CRUD]
- [Queue_hours_of_operation](#queue_hours_of_operation) [U]
- [Queue_max_contacts](#queue_max_contacts) [U]
- [Queue_status](#queue_status) [U]
- [Hours_of_operation_override](#hours_of_operation_override) [CRUD]
- [Contact_flow_module_metadata](#contact_flow_module_metadata) [U]
- [Routing_profile_name](#routing_profile_name) [U]
- [View_metadata](#view_metadata) [U]
- [Agent_status](#agent_status) [CRU]
- [Integration_association](#integration_association) [CD]
- [Quick_connect](#quick_connect) [CRD]
- [User_hierarchy_group](#user_hierarchy_group) [CRD]
- [Authentication_profile](#authentication_profile) [RU]
- [Contact_flow](#contact_flow) [CRD]
- [Prompt](#prompt) [CRUD]
- [Contact_evaluation](#contact_evaluation) [RUD]
- [Traffic_distribution](#traffic_distribution) [RU]
- [User_hierarchy_structure](#user_hierarchy_structure) [RU]
- [Contact_flow_module_content](#contact_flow_module_content) [U]
- [Queue_outbound_email_config](#queue_outbound_email_config) [U]
- [Quick_connect_name](#quick_connect_name) [U]
- [Routing_profile_default_outbound_queue](#routing_profile_default_outbound_queue) [U]
- [Federation_token](#federation_token) [R]
- [Quick_connect_config](#quick_connect_config) [U]
- [View](#view) [CRD]
- [Traffic_distribution_group](#traffic_distribution_group) [CRD]
- [Instance](#instance) [CRD]
- [Contact_flow_content](#contact_flow_content) [U]
- [Participant_role_config](#participant_role_config) [U]
- [Metric_data_v2](#metric_data_v2) [R]
- [Phone_number_metadata](#phone_number_metadata) [U]
- [User_proficiencies](#user_proficiencies) [U]
- [Contact_flow_version](#contact_flow_version) [CD]
- [Rule](#rule) [CRUD]
- [View_version](#view_version) [CD]
- [Contact_flow_module](#contact_flow_module) [CRD]
- [Contact_flow_metadata](#contact_flow_metadata) [U]
- [Routing_profile_concurrency](#routing_profile_concurrency) [U]
- [Push_notification_registration](#push_notification_registration) [CD]
- [Current_user_data](#current_user_data) [R]
- [Routing_profile_queues](#routing_profile_queues) [U]
- [User_routing_profile](#user_routing_profile) [U]
- [Task_template](#task_template) [CRUD]
- [Phone_number](#phone_number) [RU]
- [Hours_of_operation](#hours_of_operation) [CRUD]
- [User_status](#user_status) [C]
- [Contact](#contact) [CRU]
- [Persistent_contact_association](#persistent_contact_association) [C]
- [Routing_profile_agent_availability_timer](#routing_profile_agent_availability_timer) [U]
- [Predefined_attribute](#predefined_attribute) [CRUD]
- [Participant_authentication](#participant_authentication) [U]
- [Routing_profile](#routing_profile) [CRD]
- [Instance_storage_config](#instance_storage_config) [RU]
- [Participant](#participant) [C]
- [Contact_flow_name](#contact_flow_name) [U]
- [Queue_outbound_caller_config](#queue_outbound_caller_config) [U]
- [Effective_hours_of_operations](#effective_hours_of_operations) [R]
- [Contact_schedule](#contact_schedule) [U]
- [Queue_name](#queue_name) [U]
- [Prompt_file](#prompt_file) [R]
- [Vocabulary](#vocabulary) [CRD]

---

## Resources


### User_hierarchy

UserHierarchy resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hierarchy_group_id` | String |  | <p>The identifier of the hierarchy group.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `user_id` | String | ✅ | <p>The identifier of the user account.</p> |



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


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `routing_profile_id` | String | ✅ | <p>The identifier of the routing profile for the user.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `directory_user_id` | String |  | <p>The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users.
   If you include the identifier, we assume that Amazon Connect cannot access the directory.
   Otherwise, the identity information is used to authenticate users from your directory.</p>
         <p>This parameter is required if you are using an existing directory for identity management in
    Amazon Connect when Amazon Connect cannot access your directory to authenticate users.
   If you are using SAML for identity management and include this parameter, an error is
   returned.</p> |
| `security_profile_ids` | Vec<String> | ✅ | <p>The identifier of the security profile for the user.</p> |
| `username` | String | ✅ | <p>The user name for the account. For instances not using SAML for identity management, the
   user name can include up to 20 characters. If you are using SAML for identity management, the
   user name can include up to 64 characters from [a-zA-Z0-9_-.\@]+.</p>
         <p>Username can include @ only if used in an email format. For example:</p>
         <ul>
            <li>
               <p>Correct: testuser</p>
            </li>
            <li>
               <p>Correct: testuser@example.com</p>
            </li>
            <li>
               <p>Incorrect: testuser@example</p>
            </li>
         </ul> |
| `identity_info` | String |  | <p>The information about the identity of the user.</p> |
| `password` | String |  | <p>The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.</p> |
| `hierarchy_group_id` | String |  | <p>The identifier of the hierarchy group for the user.</p> |
| `phone_config` | String | ✅ | <p>The phone settings for the user.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | String | <p>Information about the user account and configuration settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.connect.User {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    routing_profile_id = "value"  # <p>The identifier of the routing profile for the user.</p>
    security_profile_ids = "value"  # <p>The identifier of the security profile for the user.</p>
    username = "value"  # <p>The user name for the account. For instances not using SAML for identity management, the
   user name can include up to 20 characters. If you are using SAML for identity management, the
   user name can include up to 64 characters from [a-zA-Z0-9_-.\@]+.</p>
         <p>Username can include @ only if used in an email format. For example:</p>
         <ul>
            <li>
               <p>Correct: testuser</p>
            </li>
            <li>
               <p>Correct: testuser@example.com</p>
            </li>
            <li>
               <p>Incorrect: testuser@example</p>
            </li>
         </ul>
    phone_config = "value"  # <p>The phone settings for the user.</p>
}

# Access user outputs
user_id = user.id
user_user = user.user
```

---


### User_security_profiles

UserSecurityProfiles resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_profile_ids` | Vec<String> | ✅ | <p>The identifiers of the security profiles for the user.</p> |
| `user_id` | String | ✅ | <p>The identifier of the user account.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### Instance_attribute

InstanceAttribute resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | String | ✅ | <p>The value for the attribute. Maximum character limit is 100. </p> |
| `attribute_type` | String | ✅ | <p>The type of attribute.</p>
         <note>
            <p>Only allowlisted customers can consume USE_CUSTOM_TTS_VOICES. To access this feature,
    contact Amazon Web ServicesSupport for allowlisting.</p>
         </note> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attribute` | String | <p>The
   type
   of attribute.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_attribute outputs
instance_attribute_id = instance_attribute.id
instance_attribute_attribute = instance_attribute.attribute
```

---


### Contact_attributes

ContactAttributes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `initial_contact_id` | String | ✅ | <p>The identifier of the contact. This is the identifier of the contact associated with the
   first interaction with the contact center.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `attributes` | HashMap<String, String> | ✅ | <p>The Amazon Connect attributes. These attributes can be accessed in flows just like any
   other contact attributes.</p>
         <p>You can have up to 32,768 UTF-8 bytes across all attributes for a contact. Attribute keys
   can include only alphanumeric, dash, and underscore characters.</p>
         <p>In the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/set-contact-attributes.html">Set contact attributes</a>
   block, when the attributes for a contact exceed 32 KB, the contact is routed down the Error
   branch of the flow. As a mitigation, consider the following options:</p>
         <ul>
            <li>
               <p>Remove unnecessary attributes by setting their values to empty.</p>
            </li>
            <li>
               <p>If the attributes are only used in one flow and don't need to be referred to outside of
     that flow (for example, by a Lambda or another flow), then use flow attributes. This way you
     aren't needlessly persisting the 32 KB of information from one flow to another. For more
     information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/set-contact-attributes.html">Flow block: Set contact
      attributes</a> in the <i>Amazon Connect Administrator Guide</i>. </p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | HashMap<String, String> | <p>Information about the attributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access contact_attributes outputs
contact_attributes_id = contact_attributes.id
contact_attributes_attributes = contact_attributes.attributes
```

---


### Current_metric_data

CurrentMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_snapshot_time` | String | <p>The time at which the metrics were retrieved and cached for pagination.</p> |
| `metric_results` | Vec<String> | <p>Information about the real-time metrics.</p> |
| `next_token` | String | <p>If there are additional results, this is the token for the next set of results.</p>
         <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use
   the token must use the same request parameters as the request that generated the token.</p> |
| `approximate_total_count` | i64 | <p>The total count of the result, regardless of the current page size. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access current_metric_data outputs
current_metric_data_id = current_metric_data.id
current_metric_data_data_snapshot_time = current_metric_data.data_snapshot_time
current_metric_data_metric_results = current_metric_data.metric_results
current_metric_data_next_token = current_metric_data.next_token
current_metric_data_approximate_total_count = current_metric_data.approximate_total_count
```

---


### Metric_data

MetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_results` | Vec<String> | <p>Information about the historical metrics.</p>
         <p>If no grouping is specified, a summary of metric data is returned.</p> |
| `next_token` | String | <p>If there are additional results, this is the token for the next set of results.</p>
         <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use
   the token must use the same request parameters as the request that generated the token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metric_data outputs
metric_data_id = metric_data.id
metric_data_metric_results = metric_data.metric_results
metric_data_next_token = metric_data.next_token
```

---


### Security_profile

SecurityProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hierarchy_restricted_resources` | Vec<String> |  | <p>The list of resources that a security profile applies hierarchy restrictions to in Amazon Connect. Following are acceptable ResourceNames: <code>User</code>.</p> |
| `applications` | Vec<String> |  | <p>A list of third-party applications that the security profile will give access to.</p> |
| `permissions` | Vec<String> |  | <p>Permissions assigned to the security profile. For a list of valid permissions, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-list.html">List of security
    profile permissions</a>. </p> |
| `allowed_access_control_tags` | HashMap<String, String> |  | <p>The list of tags that a security profile uses to restrict access to resources in Amazon Connect.</p> |
| `description` | String |  | <p>The description of the security profile.</p> |
| `tag_restricted_resources` | Vec<String> |  | <p>The list of resources that a security profile applies tag restrictions to in Amazon Connect. For a list of Amazon Connect resources that you can tag, see
    <a href="https://docs.aws.amazon.com/connect/latest/adminguide/tagging.html">Add tags to resources
    in Amazon Connect</a> in the <i>Amazon Connect Administrator
   Guide</i>. </p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `allowed_access_control_hierarchy_group_id` | String |  | <p>The identifier of the hierarchy group that a security profile uses to restrict access to
   resources in Amazon Connect.</p> |
| `security_profile_name` | String | ✅ | <p>The name of the security profile.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_profile` | String | <p>The security profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create security_profile
security_profile = provider.connect.Security_profile {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    security_profile_name = "value"  # <p>The name of the security profile.</p>
}

# Access security_profile outputs
security_profile_id = security_profile.id
security_profile_security_profile = security_profile.security_profile
```

---


### Use_case

UseCase resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `use_case_type` | String | ✅ | <p>The type of use case to associate to the integration association. Each integration
   association can have only one of each use case type.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `integration_association_id` | String | ✅ | <p>The identifier for the integration association.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create use_case
use_case = provider.connect.Use_case {
    use_case_type = "value"  # <p>The type of use case to associate to the integration association. Each integration
   association can have only one of each use case type.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    integration_association_id = "value"  # <p>The identifier for the integration association.</p>
}

```

---


### Contact_metrics

ContactMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The unique identifier of the contact for which metrics were retrieved.</p> |
| `arn` | String | <p>The ARN of the contact for which metrics were retrieved.</p> |
| `metric_results` | Vec<String> | <p>A list of metric results containing the calculated values for each requested metric. Each
   result includes the metric name and its corresponding calculated value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access contact_metrics outputs
contact_metrics_id = contact_metrics.id
contact_metrics_id = contact_metrics.id
contact_metrics_arn = contact_metrics.arn
contact_metrics_metric_results = contact_metrics.metric_results
```

---


### Attached_file

AttachedFile resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_name` | String | <p>A case-sensitive name of the attached file being uploaded.</p> |
| `file_use_case_type` | String | <p>The use case for the file.</p> |
| `associated_resource_arn` | String | <p>The resource to which the attached file is (being) uploaded to. <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_connect-cases_CreateCase.html">Cases</a> are the only
   current supported resource.</p> |
| `file_arn` | String | <p>The unique identifier of the attached file resource (ARN).</p> |
| `creation_time` | String | <p>The time of Creation of the file resource as an ISO timestamp. It's specified in ISO 8601
   format: <code>yyyy-MM-ddThh:mm:ss.SSSZ</code>. For example,
   <code>2024-05-03T02:41:28.172Z</code>.</p> |
| `file_size_in_bytes` | i64 | <p>The size of the attached file in bytes.</p> |
| `created_by` | String | <p>Represents the identity that created the file.</p> |
| `download_url_metadata` | String | <p>URL and expiry to be used when downloading the attached file. </p> |
| `file_status` | String | <p>The current status of the attached file.</p> |
| `file_id` | String | <p>The unique identifier of the attached file resource.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, <code>{
    "Tags": {"key1":"value1", "key2":"value2"} }</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access attached_file outputs
attached_file_id = attached_file.id
attached_file_file_name = attached_file.file_name
attached_file_file_use_case_type = attached_file.file_use_case_type
attached_file_associated_resource_arn = attached_file.associated_resource_arn
attached_file_file_arn = attached_file.file_arn
attached_file_creation_time = attached_file.creation_time
attached_file_file_size_in_bytes = attached_file.file_size_in_bytes
attached_file_created_by = attached_file.created_by
attached_file_download_url_metadata = attached_file.download_url_metadata
attached_file_file_status = attached_file.file_status
attached_file_file_id = attached_file.file_id
attached_file_tags = attached_file.tags
```

---


### User_identity_info

UserIdentityInfo resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | <p>The identifier of the user account.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `identity_info` | String | ✅ | <p>The identity information for the user.</p> |



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


### User_hierarchy_group_name

UserHierarchyGroupName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the hierarchy group. Must not be more than 100 characters.</p> |
| `hierarchy_group_id` | String | ✅ | <p>The identifier of the hierarchy group.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### User_phone_config

UserPhoneConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `phone_config` | String | ✅ | <p>Information about phone configuration settings for the user.</p> |
| `user_id` | String | ✅ | <p>The identifier of the user account.</p> |



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


### View_content

ViewContent resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String | ✅ | <p>Indicates the view status as either <code>SAVED</code> or <code>PUBLISHED</code>. The
    <code>PUBLISHED</code> status will initiate validation on the content.</p> |
| `content` | String | ✅ | <p>View content containing all content necessary to render a view except for runtime input data
   and the runtime input schema, which is auto-generated by this operation.</p>
         <p>The total uncompressed content has a maximum file size of 400kB.</p> |
| `view_id` | String | ✅ | <p>The identifier of the view. Both <code>ViewArn</code> and <code>ViewId</code> can be
   used.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of
   the instance.</p> |



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


### Contact_routing_data

ContactRoutingData resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `queue_priority` | i64 |  | <p>Priority of the contact in the queue. The default priority for new contacts is 5. You can
   raise the priority of a contact compared to other contacts in the queue by assigning them a
   higher priority, such as 1 or 2.</p> |
| `routing_criteria` | String |  | <p>Updates the routing criteria on the contact. These properties can be used to change how a 
   contact is routed within the queue.</p> |
| `queue_time_adjustment_seconds` | i64 |  | <p>The number of seconds to add or subtract from the contact's routing age. Contacts are routed
   to agents on a first-come, first-serve basis. This means that changing their amount of time in
   queue compared to others also changes their position in queue.</p> |
| `contact_id` | String | ✅ | <p>The identifier of the contact in this instance of Amazon Connect. </p> |



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


### Flow_association

FlowAssociation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_type` | String | <p>A valid resource type.</p> |
| `resource_id` | String | <p>The identifier of the resource.</p> |
| `flow_id` | String | <p>The identifier of the flow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flow_association outputs
flow_association_id = flow_association.id
flow_association_resource_type = flow_association.resource_type
flow_association_resource_id = flow_association.resource_id
flow_association_flow_id = flow_association.flow_id
```

---


### Email_address_metadata

EmailAddressMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p>The display name of email address.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `email_address_id` | String | ✅ | <p>The identifier of the email address.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `description` | String |  | <p>The description of the email address.</p> |



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


### Email_address

EmailAddress resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p>The display name of email address</p> |
| `description` | String |  | <p>The description of the email address.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `email_address` | String | ✅ | <p>The email address, including the domain.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `display_name` | String | <p>The display name of email address</p> |
| `alias_configurations` | Vec<String> | <p>A list of alias configurations associated with this email address. Contains details about
   email addresses that forward to this primary email address. The list can contain at most one
   alias configuration per email address.</p> |
| `modified_timestamp` | String | <p>The email address last modification timestamp in ISO 8601 Datetime.</p> |
| `create_timestamp` | String | <p>The email address creation timestamp in ISO 8601 Datetime.</p> |
| `email_address_id` | String | <p>The identifier of the email address.</p> |
| `email_address` | String | <p>The email address, including the domain.</p> |
| `description` | String | <p>The description of the email address.</p> |
| `email_address_arn` | String | <p>The Amazon Resource Name (ARN) of the email address.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_address
email_address = provider.connect.Email_address {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    email_address = "value"  # <p>The email address, including the domain.</p>
}

# Access email_address outputs
email_address_id = email_address.id
email_address_tags = email_address.tags
email_address_display_name = email_address.display_name
email_address_alias_configurations = email_address.alias_configurations
email_address_modified_timestamp = email_address.modified_timestamp
email_address_create_timestamp = email_address.create_timestamp
email_address_email_address_id = email_address.email_address_id
email_address_email_address = email_address.email_address
email_address_description = email_address.description
email_address_email_address_arn = email_address.email_address_arn
```

---


### Queue

Queue resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outbound_caller_config` | String |  | <p>The outbound caller ID name, number, and outbound whisper flow.</p> |
| `name` | String | ✅ | <p>The name of the queue.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `description` | String |  | <p>The description of the queue.</p> |
| `max_contacts` | i64 |  | <p>The maximum number of contacts that can be in the queue before it is considered full.</p> |
| `outbound_email_config` | String |  | <p>The outbound email address ID for a specified queue.</p> |
| `hours_of_operation_id` | String | ✅ | <p>The identifier for the hours of operation.</p> |
| `quick_connect_ids` | Vec<String> |  | <p>The quick connects available to agents who are working the queue.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `queue` | String | <p>The name of the queue.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create queue
queue = provider.connect.Queue {
    name = "value"  # <p>The name of the queue.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    hours_of_operation_id = "value"  # <p>The identifier for the hours of operation.</p>
}

# Access queue outputs
queue_id = queue.id
queue_queue = queue.queue
```

---


### Evaluation_form

EvaluationForm resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `items` | Vec<String> | ✅ | <p>Items that are part of the evaluation form.  The total number of sections and questions must not exceed 100 each.  Questions must be contained in a section.</p> |
| `description` | String |  | <p>The description of the evaluation form.</p> |
| `title` | String | ✅ | <p>A title of the evaluation form.</p> |
| `scoring_strategy` | String |  | <p>A scoring strategy of the evaluation form.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_form` | String | <p>Information about the evaluation form.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create evaluation_form
evaluation_form = provider.connect.Evaluation_form {
    items = "value"  # <p>Items that are part of the evaluation form.  The total number of sections and questions must not exceed 100 each.  Questions must be contained in a section.</p>
    title = "value"  # <p>A title of the evaluation form.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
}

# Access evaluation_form outputs
evaluation_form_id = evaluation_form.id
evaluation_form_evaluation_form = evaluation_form.evaluation_form
```

---


### Queue_hours_of_operation

QueueHoursOfOperation resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `queue_id` | String | ✅ | <p>The identifier for the queue.</p> |
| `hours_of_operation_id` | String | ✅ | <p>The identifier for the hours of operation.</p> |



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


### Queue_max_contacts

QueueMaxContacts resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `queue_id` | String | ✅ | <p>The identifier for the queue.</p> |
| `max_contacts` | i64 |  | <p>The maximum number of contacts that can be in the queue before it is considered full.</p> |



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


### Queue_status

QueueStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String | ✅ | <p>The status of the queue.</p> |
| `queue_id` | String | ✅ | <p>The identifier for the queue.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### Hours_of_operation_override

HoursOfOperationOverride resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `effective_till` | String | ✅ | <p>The date until when the hours of operation override is effective.</p> |
| `name` | String | ✅ | <p>The name of the hours of operation override.</p> |
| `config` | Vec<String> | ✅ | <p>Configuration information for the hours of operation override: day, start time, and end
   time.</p> |
| `effective_from` | String | ✅ | <p>The date from when the hours of operation override is effective.</p> |
| `hours_of_operation_id` | String | ✅ | <p>The identifier for the hours of operation</p> |
| `description` | String |  | <p>The description of the hours of operation override.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hours_of_operation_override` | String | <p>Information about the hours of operations override. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hours_of_operation_override
hours_of_operation_override = provider.connect.Hours_of_operation_override {
    effective_till = "value"  # <p>The date until when the hours of operation override is effective.</p>
    name = "value"  # <p>The name of the hours of operation override.</p>
    config = "value"  # <p>Configuration information for the hours of operation override: day, start time, and end
   time.</p>
    effective_from = "value"  # <p>The date from when the hours of operation override is effective.</p>
    hours_of_operation_id = "value"  # <p>The identifier for the hours of operation</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance.</p>
}

# Access hours_of_operation_override outputs
hours_of_operation_override_id = hours_of_operation_override.id
hours_of_operation_override_hours_of_operation_override = hours_of_operation_override.hours_of_operation_override
```

---


### Contact_flow_module_metadata

ContactFlowModuleMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the flow module.</p> |
| `state` | String |  | <p>The state of flow module.</p> |
| `description` | String |  | <p>The description of the flow module.</p> |
| `contact_flow_module_id` | String | ✅ | <p>The identifier of the flow module.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### Routing_profile_name

RoutingProfileName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the routing profile. Must not be more than 127 characters.</p> |
| `routing_profile_id` | String | ✅ | <p>The identifier of the routing profile.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `description` | String |  | <p>The description of the routing profile. Must not be more than 250 characters.</p> |



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


### View_metadata

ViewMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the view.</p> |
| `description` | String |  | <p>The description of the view.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of
   the instance.</p> |
| `view_id` | String | ✅ | <p>The identifier of the view. Both <code>ViewArn</code> and <code>ViewId</code> can be
   used.</p> |



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


### Agent_status

AgentStatus resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the status.</p> |
| `name` | String | ✅ | <p>The name of the status.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `state` | String | ✅ | <p>The state of the status.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `display_order` | i64 |  | <p>The display order of the status.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `agent_status` | String | <p>The agent status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create agent_status
agent_status = provider.connect.Agent_status {
    name = "value"  # <p>The name of the status.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    state = "value"  # <p>The state of the status.</p>
}

# Access agent_status outputs
agent_status_id = agent_status.id
agent_status_agent_status = agent_status.agent_status
```

---


### Integration_association

IntegrationAssociation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `source_application_url` | String |  | <p>The URL for the external application. This field is only required for the EVENT integration type.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `integration_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the integration.</p>
         <note>
            <p>When integrating with Amazon Web Services End User Messaging, the Amazon Connect and
     Amazon Web Services End User Messaging instances must be in the same account.</p>
         </note> |
| `source_application_name` | String |  | <p>The name of the external application. This field is only required for the EVENT integration type.</p> |
| `source_type` | String |  | <p>The type of the data source. This field is only required for the EVENT integration type.</p> |
| `integration_type` | String | ✅ | <p>The type of information to be ingested.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration_association
integration_association = provider.connect.Integration_association {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    integration_arn = "value"  # <p>The Amazon Resource Name (ARN) of the integration.</p>
         <note>
            <p>When integrating with Amazon Web Services End User Messaging, the Amazon Connect and
     Amazon Web Services End User Messaging instances must be in the same account.</p>
         </note>
    integration_type = "value"  # <p>The type of information to be ingested.</p>
}

```

---


### Quick_connect

QuickConnect resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A unique name of the quick connect.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `quick_connect_config` | String | ✅ | <p>Configuration settings for the quick connect.</p> |
| `description` | String |  | <p>The description of the quick connect.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `quick_connect` | String | <p>Information about the quick connect.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create quick_connect
quick_connect = provider.connect.Quick_connect {
    name = "value"  # <p>A unique name of the quick connect.</p>
    quick_connect_config = "value"  # <p>Configuration settings for the quick connect.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
}

# Access quick_connect outputs
quick_connect_id = quick_connect.id
quick_connect_quick_connect = quick_connect.quick_connect
```

---


### User_hierarchy_group

UserHierarchyGroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `name` | String | ✅ | <p>The name of the user hierarchy group. Must not be more than 100 characters.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `parent_group_id` | String |  | <p>The identifier for the parent hierarchy group. The user hierarchy is created at level one if
   the parent group ID is null.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hierarchy_group` | String | <p>Information about the hierarchy group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_hierarchy_group
user_hierarchy_group = provider.connect.User_hierarchy_group {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    name = "value"  # <p>The name of the user hierarchy group. Must not be more than 100 characters.</p>
}

# Access user_hierarchy_group outputs
user_hierarchy_group_id = user_hierarchy_group.id
user_hierarchy_group_hierarchy_group = user_hierarchy_group.hierarchy_group
```

---


### Authentication_profile

AuthenticationProfile resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `authentication_profile_id` | String | ✅ | <p>A unique identifier for the authentication profile. </p> |
| `description` | String |  | <p>The description for the authentication profile.</p> |
| `name` | String |  | <p>The name for the authentication profile.</p> |
| `blocked_ips` | Vec<String> |  | <p>A list of IP address range strings that are blocked from accessing the instance. For more
   information on how to configure IP addresses, For more information on how to configure IP
   addresses, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/authentication-profiles.html#configure-ip-based-ac">Configure
    IP-based access control</a> in the <i>Amazon Connect Administrator
    Guide</i>. </p> |
| `periodic_session_duration` | i64 |  | <p>The short lived session duration configuration for users logged in to Amazon Connect, in
   minutes. This value determines the maximum possible time before an agent is authenticated. For
   more information, For more information on how to configure IP addresses, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/authentication-profiles.html#configure-session-timeouts">Configure session timeouts</a> in the <i>Amazon Connect Administrator
    Guide</i>. </p> |
| `allowed_ips` | Vec<String> |  | <p>A list of IP address range strings that are allowed to access the instance. For more
   information on how to configure IP addresses, see<a href="https://docs.aws.amazon.com/connect/latest/adminguide/authentication-profiles.html#configure-session-timeouts">Configure session timeouts</a> in the <i>Amazon Connect Administrator
    Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authentication_profile` | String | <p>The authentication profile object being described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access authentication_profile outputs
authentication_profile_id = authentication_profile.id
authentication_profile_authentication_profile = authentication_profile.authentication_profile
```

---


### Contact_flow

ContactFlow resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>The type of the flow. For descriptions of the available types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/create-contact-flow.html#contact-flow-types">Choose a flow type</a> in the <i>Amazon Connect Administrator
   Guide</i>.</p> |
| `description` | String |  | <p>The description of the flow. </p> |
| `content` | String | ✅ | <p>The JSON string that represents the content of the flow. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example
    flow in Amazon Connect Flow language</a>. </p>
         <p>Length Constraints: Minimum length of 1. Maximum length of 256000.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance.</p> |
| `status` | String |  | <p>Indicates the flow status as either <code>SAVED</code> or <code>PUBLISHED</code>. The
    <code>PUBLISHED</code> status will initiate validation on the content. the <code>SAVED</code>
   status does not initiate validation of the content. <code>SAVED</code> |
   <code>PUBLISHED</code>.</p> |
| `name` | String | ✅ | <p>The name of the flow.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contact_flow` | String | <p>Information about the flow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact_flow
contact_flow = provider.connect.Contact_flow {
    type = "value"  # <p>The type of the flow. For descriptions of the available types, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/create-contact-flow.html#contact-flow-types">Choose a flow type</a> in the <i>Amazon Connect Administrator
   Guide</i>.</p>
    content = "value"  # <p>The JSON string that represents the content of the flow. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example
    flow in Amazon Connect Flow language</a>. </p>
         <p>Length Constraints: Minimum length of 1. Maximum length of 256000.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance.</p>
    name = "value"  # <p>The name of the flow.</p>
}

# Access contact_flow outputs
contact_flow_id = contact_flow.id
contact_flow_contact_flow = contact_flow.contact_flow
```

---


### Prompt

Prompt resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `name` | String | ✅ | <p>The name of the prompt.</p> |
| `description` | String |  | <p>The description of the prompt.</p> |
| `s3_uri` | String | ✅ | <p>The URI for the S3 bucket where the prompt is stored. You can provide S3 pre-signed URLs returned by the 
<a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_GetPromptFile.html">GetPromptFile</a>
 API instead of providing S3 URIs.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prompt` | String | <p>Information about the prompt.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create prompt
prompt = provider.connect.Prompt {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    name = "value"  # <p>The name of the prompt.</p>
    s3_uri = "value"  # <p>The URI for the S3 bucket where the prompt is stored. You can provide S3 pre-signed URLs returned by the 
<a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_GetPromptFile.html">GetPromptFile</a>
 API instead of providing S3 URIs.</p>
}

# Access prompt outputs
prompt_id = prompt.id
prompt_prompt = prompt.prompt
```

---


### Contact_evaluation

ContactEvaluation resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `evaluation_id` | String | ✅ | <p>A unique identifier for the contact evaluation.</p> |
| `answers` | HashMap<String, String> |  | <p>A map of question identifiers to answer value.</p> |
| `notes` | HashMap<String, String> |  | <p>A map of question identifiers to note value.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_form` | String | <p>Information about the evaluation form.</p> |
| `evaluation` | String | <p>Information about the evaluation form completed for a specific contact.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access contact_evaluation outputs
contact_evaluation_id = contact_evaluation.id
contact_evaluation_evaluation_form = contact_evaluation.evaluation_form
contact_evaluation_evaluation = contact_evaluation.evaluation
```

---


### Traffic_distribution

TrafficDistribution resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `telephony_config` | String |  | <p>The distribution of traffic between the instance and its replica(s).</p> |
| `id` | String | ✅ | <p>The identifier of the traffic distribution group.
This can be the ID or the ARN if the API is being called in the Region where the traffic distribution group was created.
The ARN must be provided if the call is from the replicated Region. </p> |
| `sign_in_config` | String |  | <p>The distribution that determines which Amazon Web Services Regions should be used to sign in
   agents in to both the instance and its replica(s).</p> |
| `agent_config` | String |  | <p>The distribution of agents between the instance and its replica(s).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The identifier of the traffic distribution group.
This can be the ID or the ARN if the API is being called in the Region where the traffic distribution group was created.
The ARN must be provided if the call is from the replicated Region.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the traffic distribution group.</p> |
| `agent_config` | String | <p>The distribution of agents between the instance and its replica(s).</p> |
| `sign_in_config` | String | <p>The distribution that determines which Amazon Web Services Regions should be used to sign in
   agents in to both the instance and its replica(s).</p> |
| `telephony_config` | String | <p>The distribution of traffic between the instance and its replicas.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access traffic_distribution outputs
traffic_distribution_id = traffic_distribution.id
traffic_distribution_id = traffic_distribution.id
traffic_distribution_arn = traffic_distribution.arn
traffic_distribution_agent_config = traffic_distribution.agent_config
traffic_distribution_sign_in_config = traffic_distribution.sign_in_config
traffic_distribution_telephony_config = traffic_distribution.telephony_config
```

---


### User_hierarchy_structure

UserHierarchyStructure resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hierarchy_structure` | String | ✅ | <p>The hierarchy levels to update.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hierarchy_structure` | String | <p>Information about the hierarchy structure.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_hierarchy_structure outputs
user_hierarchy_structure_id = user_hierarchy_structure.id
user_hierarchy_structure_hierarchy_structure = user_hierarchy_structure.hierarchy_structure
```

---


### Contact_flow_module_content

ContactFlowModuleContent resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `contact_flow_module_id` | String | ✅ | <p>The identifier of the flow module.</p> |
| `content` | String | ✅ | <p>The JSON string that represents the content of the flow. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example
    flow in Amazon Connect Flow language</a>. </p> |



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


### Queue_outbound_email_config

QueueOutboundEmailConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `outbound_email_config` | String | ✅ | <p>The outbound email address ID for a specified queue.</p> |
| `queue_id` | String | ✅ | <p>The identifier for the queue.</p> |



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


### Quick_connect_name

QuickConnectName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `quick_connect_id` | String | ✅ | <p>The identifier for the quick connect.</p> |
| `name` | String |  | <p>The name of the quick connect.</p> |
| `description` | String |  | <p>The description of the quick connect.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### Routing_profile_default_outbound_queue

RoutingProfileDefaultOutboundQueue resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `routing_profile_id` | String | ✅ | <p>The identifier of the routing profile.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `default_outbound_queue_id` | String | ✅ | <p>The identifier for the default outbound queue.</p> |



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


### Federation_token

FederationToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_id` | String | <p>The identifier for the user. This can be the ID or the ARN of the user.</p> |
| `sign_in_url` | String | <p>The URL to sign into the user's instance. </p> |
| `credentials` | String | <p>The credentials to use for federation.</p> |
| `user_arn` | String | <p>The Amazon Resource Name (ARN) of the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access federation_token outputs
federation_token_id = federation_token.id
federation_token_user_id = federation_token.user_id
federation_token_sign_in_url = federation_token.sign_in_url
federation_token_credentials = federation_token.credentials
federation_token_user_arn = federation_token.user_arn
```

---


### Quick_connect_config

QuickConnectConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `quick_connect_config` | String | ✅ | <p>Information about the configuration settings for the quick connect.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `quick_connect_id` | String | ✅ | <p>The identifier for the quick connect.</p> |



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


### View

View resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the view.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags associated with the view resource (not specific to view version).These tags can be
   used to organize, track, or control access for this resource. For example, { "tags":
   {"key1":"value1", "key2":"value2"} }.</p> |
| `client_token` | String |  | <p>A unique Id for each create view request to avoid duplicate view creation. For example, the
   view is idempotent ClientToken is provided.</p> |
| `status` | String | ✅ | <p>Indicates the view status as either <code>SAVED</code> or <code>PUBLISHED</code>. The
    <code>PUBLISHED</code> status will initiate validation on the content.</p> |
| `name` | String | ✅ | <p>The name of the view.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of
   the instance.</p> |
| `content` | String | ✅ | <p>View content containing all content necessary to render a view except for runtime input
   data.</p>
         <p>The total uncompressed content has a maximum file size of 400kB.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `view` | String | <p>All view data is contained within the View object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create view
view = provider.connect.View {
    status = "value"  # <p>Indicates the view status as either <code>SAVED</code> or <code>PUBLISHED</code>. The
    <code>PUBLISHED</code> status will initiate validation on the content.</p>
    name = "value"  # <p>The name of the view.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of
   the instance.</p>
    content = "value"  # <p>View content containing all content necessary to render a view except for runtime input
   data.</p>
         <p>The total uncompressed content has a maximum file size of 400kB.</p>
}

# Access view outputs
view_id = view.id
view_view = view.view
```

---


### Traffic_distribution_group

TrafficDistributionGroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance that has been replicated. You can find the
    <code>instanceId</code> in the ARN of the instance.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `description` | String |  | <p>A description for the traffic distribution group.</p> |
| `name` | String | ✅ | <p>The name for the traffic distribution group. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traffic_distribution_group` | String | <p>Information about the traffic distribution group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_distribution_group
traffic_distribution_group = provider.connect.Traffic_distribution_group {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance that has been replicated. You can find the
    <code>instanceId</code> in the ARN of the instance.</p>
    name = "value"  # <p>The name for the traffic distribution group. </p>
}

# Access traffic_distribution_group outputs
traffic_distribution_group_id = traffic_distribution_group.id
traffic_distribution_group_traffic_distribution_group = traffic_distribution_group.traffic_distribution_group
```

---


### Instance

Instance resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `directory_id` | String |  | <p>The identifier for the directory.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, <code>{
    "tags": {"key1":"value1", "key2":"value2"} }</code>.</p> |
| `instance_alias` | String |  | <p>The name for your instance.</p> |
| `client_token` | String |  | <p>The idempotency token.</p> |
| `inbound_calls_enabled` | bool | ✅ | <p>Your contact center handles incoming contacts.</p> |
| `outbound_calls_enabled` | bool | ✅ | <p>Your contact center allows outbound calls.</p> |
| `identity_management_type` | String | ✅ | <p>The type of identity management for your Amazon Connect users.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_configuration` | String | <p>Status information about the replication process. This field is included only when you are
   using the <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_ReplicateInstance.html">ReplicateInstance</a> API to
   replicate an Amazon Connect instance across Amazon Web Services Regions. For information about
   replicating Amazon Connect instances, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/create-replica-connect-instance.html">Create a replica of your
    existing Amazon Connect instance</a> in the <i>Amazon Connect Administrator
    Guide</i>.</p> |
| `instance` | String | <p>The name of the instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance
instance = provider.connect.Instance {
    inbound_calls_enabled = "value"  # <p>Your contact center handles incoming contacts.</p>
    outbound_calls_enabled = "value"  # <p>Your contact center allows outbound calls.</p>
    identity_management_type = "value"  # <p>The type of identity management for your Amazon Connect users.</p>
}

# Access instance outputs
instance_id = instance.id
instance_replication_configuration = instance.replication_configuration
instance_instance = instance.instance
```

---


### Contact_flow_content

ContactFlowContent resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance.</p> |
| `contact_flow_id` | String | ✅ | <p>The identifier of the flow.</p> |
| `content` | String | ✅ | <p>The JSON string that represents the content of the flow. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example
    flow in Amazon Connect Flow language</a>. </p>
         <p>Length Constraints: Minimum length of 1. Maximum length of 256000.</p> |



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


### Participant_role_config

ParticipantRoleConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `contact_id` | String | ✅ | <p>The identifier of the contact in this instance of Amazon Connect. </p> |
| `channel_configuration` | String | ✅ | <p>The Amazon Connect channel you want to configure.</p> |



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


### Metric_data_v2

MetricDataV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_results` | Vec<String> | <p>Information about the metrics requested in the API request If no grouping is specified, a
   summary of metric data is returned. </p> |
| `next_token` | String | <p>If there are additional results, this is the token for the next set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metric_data_v2 outputs
metric_data_v2_id = metric_data_v2.id
metric_data_v2_metric_results = metric_data_v2.metric_results
metric_data_v2_next_token = metric_data_v2.next_token
```

---


### Phone_number_metadata

PhoneNumberMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phone_number_id` | String | ✅ | <p>The Amazon Resource Name (ARN) or resource ID of the phone number.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `phone_number_description` | String |  | <p>The description of the phone number.</p> |



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


### User_proficiencies

UserProficiencies resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_proficiencies` | Vec<String> | ✅ | <p>The proficiencies to be updated for the user. Proficiencies must first be associated to the
   user. You can do this using AssociateUserProficiencies API.</p> |
| `instance_id` | String | ✅ | <p> The identifier of the Amazon Connect instance. You can find the instance ID in the Amazon Resource
   Name (ARN) of the instance.</p> |
| `user_id` | String | ✅ | <p>The identifier of the user account.</p> |



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


### Contact_flow_version

ContactFlowVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `flow_content_sha256` | String |  | <p>Indicates the checksum value of the flow content.</p> |
| `contact_flow_version` | i64 |  | <p>The identifier of the flow version.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance.</p> |
| `last_modified_time` | String |  | <p>The Amazon Web Services Region where this resource was last modified.</p> |
| `description` | String |  | <p>The description of the flow version.</p> |
| `last_modified_region` | String |  | <p>The Amazon Web Services Region where this resource was last modified.</p> |
| `contact_flow_id` | String | ✅ | <p>The identifier of the flow.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact_flow_version
contact_flow_version = provider.connect.Contact_flow_version {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance.</p>
    contact_flow_id = "value"  # <p>The identifier of the flow.</p>
}

```

---


### Rule

Rule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `function` | String | ✅ | <p>The conditions of the rule.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `trigger_event_source` | String | ✅ | <p>The event source to trigger the rule.</p> |
| `name` | String | ✅ | <p>A unique name for the rule.</p> |
| `actions` | Vec<String> | ✅ | <p>A list of actions to be run when the rule is triggered.</p> |
| `publish_status` | String | ✅ | <p>The publish status of the rule.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule` | String | <p>Information about the rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule
rule = provider.connect.Rule {
    function = "value"  # <p>The conditions of the rule.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    trigger_event_source = "value"  # <p>The event source to trigger the rule.</p>
    name = "value"  # <p>A unique name for the rule.</p>
    actions = "value"  # <p>A list of actions to be run when the rule is triggered.</p>
    publish_status = "value"  # <p>The publish status of the rule.</p>
}

# Access rule outputs
rule_id = rule.id
rule_rule = rule.rule
```

---


### View_version

ViewVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `view_content_sha256` | String |  | <p>Indicates the checksum value of the latest published view content.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of
   the instance.</p> |
| `view_id` | String | ✅ | <p>The identifier of the view. Both <code>ViewArn</code> and <code>ViewId</code> can be
   used.</p> |
| `version_description` | String |  | <p>The description for the version being published.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create view_version
view_version = provider.connect.View_version {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of
   the instance.</p>
    view_id = "value"  # <p>The identifier of the view. Both <code>ViewArn</code> and <code>ViewId</code> can be
   used.</p>
}

```

---


### Contact_flow_module

ContactFlowModule resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `name` | String | ✅ | <p>The name of the flow module.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `description` | String |  | <p>The description of the flow module. </p> |
| `content` | String | ✅ | <p>The JSON string that represents the content of the flow. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example
    flow in Amazon Connect Flow language</a>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contact_flow_module` | String | <p>Information about the flow module.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact_flow_module
contact_flow_module = provider.connect.Contact_flow_module {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    name = "value"  # <p>The name of the flow module.</p>
    content = "value"  # <p>The JSON string that represents the content of the flow. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example
    flow in Amazon Connect Flow language</a>. </p>
}

# Access contact_flow_module outputs
contact_flow_module_id = contact_flow_module.id
contact_flow_module_contact_flow_module = contact_flow_module.contact_flow_module
```

---


### Contact_flow_metadata

ContactFlowMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_flow_state` | String |  | <p>The state of flow.</p> |
| `contact_flow_id` | String | ✅ | <p>The identifier of the flow.</p> |
| `name` | String |  | <p>The name of the flow.</p> |
| `description` | String |  | <p>The description of the flow.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### Routing_profile_concurrency

RoutingProfileConcurrency resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `routing_profile_id` | String | ✅ | <p>The identifier of the routing profile.</p> |
| `media_concurrencies` | Vec<String> | ✅ | <p>The channels that agents can handle in the Contact Control Panel (CCP).</p> |



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


### Push_notification_registration

PushNotificationRegistration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_token` | String | ✅ | <p>The push notification token issued by the Apple or Google gateways.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the
   Amazon Resource Name (ARN) of the instance.</p> |
| `contact_configuration` | String | ✅ | <p>The contact configuration for push notification registration.</p> |
| `device_type` | String | ✅ | <p>The device type to use when sending the message.</p> |
| `pinpoint_app_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Pinpoint application.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create push_notification_registration
push_notification_registration = provider.connect.Push_notification_registration {
    device_token = "value"  # <p>The push notification token issued by the Apple or Google gateways.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the
   Amazon Resource Name (ARN) of the instance.</p>
    contact_configuration = "value"  # <p>The contact configuration for push notification registration.</p>
    device_type = "value"  # <p>The device type to use when sending the message.</p>
    pinpoint_app_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Pinpoint application.</p>
}

```

---


### Current_user_data

CurrentUserData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If there are additional results, this is the token for the next set of results.</p> |
| `user_data_list` | Vec<String> | <p>A list of the user data that is returned.</p> |
| `approximate_total_count` | i64 | <p>The total count of the result, regardless of the current page size.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access current_user_data outputs
current_user_data_id = current_user_data.id
current_user_data_next_token = current_user_data.next_token
current_user_data_user_data_list = current_user_data.user_data_list
current_user_data_approximate_total_count = current_user_data.approximate_total_count
```

---


### Routing_profile_queues

RoutingProfileQueues resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `routing_profile_id` | String | ✅ | <p>The identifier of the routing profile.</p> |
| `queue_configs` | Vec<String> | ✅ | <p>The queues to be updated for this routing profile.
   Queues must first be associated to the routing
   profile. You can do this using AssociateRoutingProfileQueues.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### User_routing_profile

UserRoutingProfile resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | <p>The identifier of the user account.</p> |
| `routing_profile_id` | String | ✅ | <p>The identifier of the routing profile for the user.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |



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


### Task_template

TaskTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `self_assign_flow_id` | String |  | <p>The ContactFlowId for the flow that will be run if this template is used to create a
   self-assigned task.</p> |
| `name` | String | ✅ | <p>The name of the task template.</p> |
| `defaults` | String |  | <p>The default values for fields when a task is created by referencing this template.</p> |
| `contact_flow_id` | String |  | <p>The identifier of the flow that runs by default when a task is created by referencing this template.</p> |
| `description` | String |  | <p>The description of the task template.</p> |
| `constraints` | String |  | <p>Constraints that are applicable to the fields listed.</p> |
| `status` | String |  | <p>Marks a template as <code>ACTIVE</code> or <code>INACTIVE</code> for a task to refer to it. 
Tasks can only be created from <code>ACTIVE</code> templates.
If a template is marked as <code>INACTIVE</code>, then a task that refers to this template cannot be created. </p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `fields` | Vec<String> | ✅ | <p>Fields that are part of the template.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The Amazon Resource Name (ARN).</p> |
| `description` | String | <p>The description of the task template.</p> |
| `fields` | Vec<String> | <p>Fields that are part of the template.</p> |
| `self_assign_flow_id` | String | <p>The ContactFlowId for the flow that will be run if this template is used to create a
   self-assigned task.</p> |
| `instance_id` | String | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `constraints` | String | <p>Constraints that are applicable to the fields listed.</p> |
| `name` | String | <p>The name of the task template.</p> |
| `status` | String | <p>Marks a template as <code>ACTIVE</code> or <code>INACTIVE</code> for a task to refer to it. 
Tasks can only be created from <code>ACTIVE</code> templates.
If a template is marked as <code>INACTIVE</code>, then a task that refers to this template cannot be created.</p> |
| `last_modified_time` | String | <p>The timestamp when the task template was last modified.</p> |
| `created_time` | String | <p>The timestamp when the task template was created.</p> |
| `contact_flow_id` | String | <p>The identifier of the flow that runs by default when a task is created by referencing this template.</p> |
| `defaults` | String | <p>The default values for fields when a task is created by referencing this template.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `id` | String | <p>A unique identifier for the task template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create task_template
task_template = provider.connect.Task_template {
    name = "value"  # <p>The name of the task template.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    fields = "value"  # <p>Fields that are part of the template.</p>
}

# Access task_template outputs
task_template_id = task_template.id
task_template_arn = task_template.arn
task_template_description = task_template.description
task_template_fields = task_template.fields
task_template_self_assign_flow_id = task_template.self_assign_flow_id
task_template_instance_id = task_template.instance_id
task_template_constraints = task_template.constraints
task_template_name = task_template.name
task_template_status = task_template.status
task_template_last_modified_time = task_template.last_modified_time
task_template_created_time = task_template.created_time
task_template_contact_flow_id = task_template.contact_flow_id
task_template_defaults = task_template.defaults
task_template_tags = task_template.tags
task_template_id = task_template.id
```

---


### Phone_number

PhoneNumber resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String |  | <p>The identifier of the Amazon Connect instance that phone numbers are claimed to. You
   can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the
    instance ID</a> in the Amazon Resource Name (ARN) of the instance. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p> |
| `phone_number_id` | String | ✅ | <p>A unique identifier for the phone number.</p> |
| `target_arn` | String |  | <p>The Amazon Resource Name (ARN) for Amazon Connect instances or traffic distribution groups that phone number inbound traffic is routed through. You must enter <code>InstanceId</code> or <code>TargetArn</code>. </p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `claimed_phone_number_summary` | String | <p>Information about a phone number that's been claimed to your Amazon Connect instance or
   traffic distribution group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access phone_number outputs
phone_number_id = phone_number.id
phone_number_claimed_phone_number_summary = phone_number.claimed_phone_number_summary
```

---


### Hours_of_operation

HoursOfOperation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `description` | String |  | <p>The description of the hours of operation.</p> |
| `time_zone` | String | ✅ | <p>The time zone of the hours of operation.</p> |
| `name` | String | ✅ | <p>The name of the hours of operation.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `config` | Vec<String> | ✅ | <p>Configuration information for the hours of operation: day, start time, and end time.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hours_of_operation` | String | <p>The hours of operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hours_of_operation
hours_of_operation = provider.connect.Hours_of_operation {
    time_zone = "value"  # <p>The time zone of the hours of operation.</p>
    name = "value"  # <p>The name of the hours of operation.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    config = "value"  # <p>Configuration information for the hours of operation: day, start time, and end time.</p>
}

# Access hours_of_operation outputs
hours_of_operation_id = hours_of_operation.id
hours_of_operation_hours_of_operation = hours_of_operation.hours_of_operation
```

---


### User_status

UserStatus resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | <p>The identifier of the user.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `agent_status_id` | String | ✅ | <p>The identifier of the agent status.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_status
user_status = provider.connect.User_status {
    user_id = "value"  # <p>The identifier of the user.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    agent_status_id = "value"  # <p>The identifier of the agent status.</p>
}

```

---


### Contact

Contact resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiry_duration_in_minutes` | i64 |  | <p>Number of minutes the contact will be active for before expiring</p> |
| `initiate_as` | String |  | <p>Initial state of the contact when it's created. Only TASK channel contacts can be initiated
   with <code>COMPLETED</code> state.</p> |
| `initiation_method` | String | ✅ | <p>Indicates how the contact was initiated. </p>
         <important>
            <p>CreateContact only supports the following initiation methods. Valid values by channel are: </p>
            <ul>
               <li>
                  <p>For VOICE: <code>TRANSFER</code> and the subtype <code>connect:ExternalAudio</code>
                  </p>
               </li>
               <li>
                  <p>For EMAIL: <code>OUTBOUND</code> | <code>AGENT_REPLY</code> | <code>FLOW</code>
                  </p>
               </li>
               <li>
                  <p>For TASK: <code>API</code>
                  </p>
               </li>
            </ul>
            <p>The other channels listed below are incorrect. We're working to correct this
    information.</p>
         </important> |
| `description` | String |  | <p>A description of the contact.</p> |
| `user_info` | String |  | <p>User details for the contact</p>
         <important>
            <p>UserInfo is required when creating an EMAIL contact with <code>OUTBOUND</code> and
     <code>AGENT_REPLY</code> contact initiation methods.</p>
         </important> |
| `segment_attributes` | HashMap<String, String> |  | <p>A set of system defined key-value pairs stored on individual contact segments (unique
   contact ID) using an attribute map. The attributes are standard Amazon Connect attributes.
   They can be accessed in flows.</p>
         <p>Attribute keys can include only alphanumeric, -, and _.</p>
         <p>This field can be used to set Segment Contact Expiry as a duration in minutes.</p>
         <note>
            <p>To set contact expiry, a ValueMap must be specified containing the integer number of
    minutes the contact will be active for before expiring, with <code>SegmentAttributes</code> like
    { <code> "connect:ContactExpiry": {"ValueMap" : { "ExpiryDuration": { "ValueInteger":
     135}}}}</code>. </p>
         </note> |
| `attributes` | HashMap<String, String> |  | <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in flows just like any other contact attributes.</p>
         <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys
   can include only alphanumeric, dash, and underscore characters.</p> |
| `previous_contact_id` | String |  | <p>The ID of the previous contact when creating a transfer contact. This value can be provided
   only for external audio contacts. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Integrate Amazon Connect Contact Lens
    with external voice systems</a> in the <i>Amazon Connect Administrator
    Guide</i>.</p> |
| `references` | HashMap<String, String> |  | <p>A formatted URL that is shown to an agent in the Contact Control Panel (CCP). Tasks can have
   the following reference types at the time of creation: <code>URL</code> | <code>NUMBER</code> |
    <code>STRING</code> | <code>DATE</code> | <code>EMAIL</code> | <code>ATTACHMENT</code>.</p> |
| `name` | String |  | <p>The name of a the contact.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `related_contact_id` | String |  | <p>The identifier of the contact in this instance of Amazon Connect. </p> |
| `channel` | String | ✅ | <p>The channel for the contact.</p>
         <important>
            <p>The CHAT channel is not supported. The following information is incorrect. We're working to
    correct it.</p>
         </important> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contact` | String | <p>Information about the contact.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact
contact = provider.connect.Contact {
    initiation_method = "value"  # <p>Indicates how the contact was initiated. </p>
         <important>
            <p>CreateContact only supports the following initiation methods. Valid values by channel are: </p>
            <ul>
               <li>
                  <p>For VOICE: <code>TRANSFER</code> and the subtype <code>connect:ExternalAudio</code>
                  </p>
               </li>
               <li>
                  <p>For EMAIL: <code>OUTBOUND</code> | <code>AGENT_REPLY</code> | <code>FLOW</code>
                  </p>
               </li>
               <li>
                  <p>For TASK: <code>API</code>
                  </p>
               </li>
            </ul>
            <p>The other channels listed below are incorrect. We're working to correct this
    information.</p>
         </important>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    channel = "value"  # <p>The channel for the contact.</p>
         <important>
            <p>The CHAT channel is not supported. The following information is incorrect. We're working to
    correct it.</p>
         </important>
}

# Access contact outputs
contact_id = contact.id
contact_contact = contact.contact
```

---


### Persistent_contact_association

PersistentContactAssociation resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rehydration_type` | String | ✅ | <p>The contactId chosen for rehydration depends on the type chosen.</p>
         <ul>
            <li>
               <p>
                  <code>ENTIRE_PAST_SESSION</code>: Rehydrates a chat from the most recently terminated past
     chat contact of the specified past ended chat session. To use this type, provide the
      <code>initialContactId</code> of the past ended chat session in the
      <code>sourceContactId</code> field. In this type, Amazon Connect determines what the most
     recent chat contact on the past ended chat session and uses it to start a persistent chat.
    </p>
            </li>
            <li>
               <p>
                  <code>FROM_SEGMENT</code>: Rehydrates a chat from the specified past chat contact provided
     in the <code>sourceContactId</code> field. </p>
            </li>
         </ul>
         <p>The actual contactId used for rehydration is provided in the response of this API.</p>
         <p>To illustrate how to use rehydration type, consider the following example: A customer starts
   a chat session. Agent a1 accepts the chat and a conversation starts between the customer and
   Agent a1. This first contact creates a contact ID <b>C1</b>. Agent a1
   then transfers the chat to Agent a2. This creates another contact ID <b>C2</b>. At this point Agent a2 ends the chat. The customer is forwarded to the
   disconnect flow for a post chat survey that creates another contact ID <b>C3</b>. After the chat survey, the chat session ends. Later, the customer returns and
   wants to resume their past chat session. At this point, the customer can have following use
   cases: </p>
         <ul>
            <li>
               <p>
                  <b>Use Case 1</b>: The customer wants to continue the past chat
     session but they want to hide the post chat survey. For this they will use the following
     configuration:</p>
               <ul>
                  <li>
                     <p>
                        <b>Configuration</b>
                     </p>
                     <ul>
                        <li>
                           <p>SourceContactId = "C2"</p>
                        </li>
                        <li>
                           <p>RehydrationType = "FROM_SEGMENT"</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <b>Expected behavior</b>
                     </p>
                     <ul>
                        <li>
                           <p>This starts a persistent chat session from the specified past ended contact (C2).
         Transcripts of past chat sessions C2 and C1 are accessible in the current persistent chat
         session. Note that chat segment C3 is dropped from the persistent chat session.</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>Use Case 2</b>: The customer wants to continue the past chat
     session and see the transcript of the entire past engagement, including the post chat survey.
     For this they will use the following configuration:</p>
               <ul>
                  <li>
                     <p>
                        <b>Configuration</b>
                     </p>
                     <ul>
                        <li>
                           <p>SourceContactId = "C1"</p>
                        </li>
                        <li>
                           <p>RehydrationType = "ENTIRE_PAST_SESSION"</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <b>Expected behavior</b>
                     </p>
                     <ul>
                        <li>
                           <p>This starts a persistent chat session from the most recently ended chat contact (C3).
         Transcripts of past chat sessions C3, C2 and C1 are accessible in the current persistent
         chat session.</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </li>
         </ul> |
| `initial_contact_id` | String | ✅ | <p>This is the contactId of the current contact that the
    <code>CreatePersistentContactAssociation</code> API is being called from.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `source_contact_id` | String | ✅ | <p>The contactId from which a persistent chat session must be started.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create persistent_contact_association
persistent_contact_association = provider.connect.Persistent_contact_association {
    rehydration_type = "value"  # <p>The contactId chosen for rehydration depends on the type chosen.</p>
         <ul>
            <li>
               <p>
                  <code>ENTIRE_PAST_SESSION</code>: Rehydrates a chat from the most recently terminated past
     chat contact of the specified past ended chat session. To use this type, provide the
      <code>initialContactId</code> of the past ended chat session in the
      <code>sourceContactId</code> field. In this type, Amazon Connect determines what the most
     recent chat contact on the past ended chat session and uses it to start a persistent chat.
    </p>
            </li>
            <li>
               <p>
                  <code>FROM_SEGMENT</code>: Rehydrates a chat from the specified past chat contact provided
     in the <code>sourceContactId</code> field. </p>
            </li>
         </ul>
         <p>The actual contactId used for rehydration is provided in the response of this API.</p>
         <p>To illustrate how to use rehydration type, consider the following example: A customer starts
   a chat session. Agent a1 accepts the chat and a conversation starts between the customer and
   Agent a1. This first contact creates a contact ID <b>C1</b>. Agent a1
   then transfers the chat to Agent a2. This creates another contact ID <b>C2</b>. At this point Agent a2 ends the chat. The customer is forwarded to the
   disconnect flow for a post chat survey that creates another contact ID <b>C3</b>. After the chat survey, the chat session ends. Later, the customer returns and
   wants to resume their past chat session. At this point, the customer can have following use
   cases: </p>
         <ul>
            <li>
               <p>
                  <b>Use Case 1</b>: The customer wants to continue the past chat
     session but they want to hide the post chat survey. For this they will use the following
     configuration:</p>
               <ul>
                  <li>
                     <p>
                        <b>Configuration</b>
                     </p>
                     <ul>
                        <li>
                           <p>SourceContactId = "C2"</p>
                        </li>
                        <li>
                           <p>RehydrationType = "FROM_SEGMENT"</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <b>Expected behavior</b>
                     </p>
                     <ul>
                        <li>
                           <p>This starts a persistent chat session from the specified past ended contact (C2).
         Transcripts of past chat sessions C2 and C1 are accessible in the current persistent chat
         session. Note that chat segment C3 is dropped from the persistent chat session.</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>Use Case 2</b>: The customer wants to continue the past chat
     session and see the transcript of the entire past engagement, including the post chat survey.
     For this they will use the following configuration:</p>
               <ul>
                  <li>
                     <p>
                        <b>Configuration</b>
                     </p>
                     <ul>
                        <li>
                           <p>SourceContactId = "C1"</p>
                        </li>
                        <li>
                           <p>RehydrationType = "ENTIRE_PAST_SESSION"</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <b>Expected behavior</b>
                     </p>
                     <ul>
                        <li>
                           <p>This starts a persistent chat session from the most recently ended chat contact (C3).
         Transcripts of past chat sessions C3, C2 and C1 are accessible in the current persistent
         chat session.</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </li>
         </ul>
    initial_contact_id = "value"  # <p>This is the contactId of the current contact that the
    <code>CreatePersistentContactAssociation</code> API is being called from.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    source_contact_id = "value"  # <p>The contactId from which a persistent chat session must be started.</p>
}

```

---


### Routing_profile_agent_availability_timer

RoutingProfileAgentAvailabilityTimer resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `agent_availability_timer` | String | ✅ | <p>Whether agents with this routing profile will have their routing order calculated based on
    <i>time since their last inbound contact</i> or <i>longest idle
    time</i>. </p> |
| `routing_profile_id` | String | ✅ | <p>The identifier of the routing profile.</p> |



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


### Predefined_attribute

PredefinedAttribute resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can find the instance ID in the Amazon Resource
   Name (ARN) of the instance.</p> |
| `purposes` | Vec<String> |  | <p>Values that enable you to categorize your predefined attributes. You can use them in custom UI elements across the Amazon Connect admin website.</p> |
| `attribute_configuration` | String |  | <p>Custom metadata that is associated to predefined attributes to control behavior 
in upstream services, such as controlling 
how a predefined attribute should be displayed in the Amazon Connect admin website.</p> |
| `name` | String | ✅ | <p> The name of the predefined attribute. </p> |
| `values` | String |  | <p> The values of the predefined attribute. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `predefined_attribute` | String | <p>Information about the predefined attribute.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create predefined_attribute
predefined_attribute = provider.connect.Predefined_attribute {
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can find the instance ID in the Amazon Resource
   Name (ARN) of the instance.</p>
    name = "value"  # <p> The name of the predefined attribute. </p>
}

# Access predefined_attribute outputs
predefined_attribute_id = predefined_attribute.id
predefined_attribute_predefined_attribute = predefined_attribute.predefined_attribute
```

---


### Participant_authentication

ParticipantAuthentication resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `code` | String |  | <p>The <code>code</code> query parameter provided by Cognito in the
   <code>redirectUri</code>.</p> |
| `error` | String |  | <p>The <code>error</code> query parameter provided by Cognito in the
   <code>redirectUri</code>.</p> |
| `error_description` | String |  | <p>The <code>error_description</code> parameter provided by Cognito in the
    <code>redirectUri</code>.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `state` | String | ✅ | <p>The <code>state</code> query parameter that was provided by Cognito in the
    <code>redirectUri</code>. This will also match the <code>state</code> parameter provided in the
    <code>AuthenticationUrl</code> from the <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_GetAuthenticationUrl.html">GetAuthenticationUrl</a>
   response.</p> |



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


### Routing_profile

RoutingProfile resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_outbound_queue_id` | String | ✅ | <p>The default outbound queue for the routing profile.</p> |
| `agent_availability_timer` | String |  | <p>Whether agents with this routing profile will have their routing order calculated based on
    <i>longest idle time</i> or <i>time since their last inbound
    contact</i>. </p> |
| `name` | String | ✅ | <p>The name of the routing profile. Must not be more than 127 characters.</p> |
| `manual_assignment_queue_configs` | Vec<String> |  | <p>The manual assignment queues associated with the routing profile. If no queue is added,
   agents and supervisors can't pick or assign any contacts from this routing profile. The limit of
   10 array members applies to the maximum number of RoutingProfileManualAssignmentQueueConfig
   objects that can be passed during a CreateRoutingProfile API request. It is different from the
   quota of 50 queues per routing profile per instance that is listed in Amazon Connect service
   quotas.</p> |
| `queue_configs` | Vec<String> |  | <p>The inbound queues associated with the routing profile. If no queue is added, the agent can
   make only outbound calls.</p>
         <p>The limit of 10 array members applies to the maximum number of
    <code>RoutingProfileQueueConfig</code> objects that can be passed during a CreateRoutingProfile
   API request. It is different from the quota of 50 queues per routing profile per instance that is
   listed in <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">Amazon Connect service
    quotas</a>. </p> |
| `description` | String | ✅ | <p>Description of the routing profile. Must not be more than 250 characters.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `media_concurrencies` | Vec<String> | ✅ | <p>The channels that agents can handle in the Contact Control Panel (CCP) for this routing
   profile.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `routing_profile` | String | <p>The routing profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create routing_profile
routing_profile = provider.connect.Routing_profile {
    default_outbound_queue_id = "value"  # <p>The default outbound queue for the routing profile.</p>
    name = "value"  # <p>The name of the routing profile. Must not be more than 127 characters.</p>
    description = "value"  # <p>Description of the routing profile. Must not be more than 250 characters.</p>
    media_concurrencies = "value"  # <p>The channels that agents can handle in the Contact Control Panel (CCP) for this routing
   profile.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
}

# Access routing_profile outputs
routing_profile_id = routing_profile.id
routing_profile_routing_profile = routing_profile.routing_profile
```

---


### Instance_storage_config

InstanceStorageConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `storage_config` | String | ✅ |  |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `association_id` | String | ✅ | <p>The existing association identifier that uniquely identifies the resource type and storage config for the given instance ID.</p> |
| `resource_type` | String | ✅ | <p>A valid resource type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `storage_config` | String | <p>A valid storage type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_storage_config outputs
instance_storage_config_id = instance_storage_config.id
instance_storage_config_storage_config = instance_storage_config.storage_config
```

---


### Participant

Participant resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `participant_details` | String | ✅ | <p>Information identifying the participant.</p>
         <important>
            <p>The only valid value for <code>ParticipantRole</code> is <code>CUSTOM_BOT</code> for chat
    contact and <code>CUSTOMER</code> for voice contact.</p>
         </important> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance. </p> |
| `contact_id` | String | ✅ | <p>The identifier of the contact in this instance of Amazon Connect.  Supports contacts in the CHAT channel and VOICE (WebRTC) channels. For WebRTC
   calls, this should be the initial contact ID that was generated when the contact was first
   created (from the StartWebRTCContact API) in the VOICE channel</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create participant
participant = provider.connect.Participant {
    participant_details = "value"  # <p>Information identifying the participant.</p>
         <important>
            <p>The only valid value for <code>ParticipantRole</code> is <code>CUSTOM_BOT</code> for chat
    contact and <code>CUSTOMER</code> for voice contact.</p>
         </important>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance. </p>
    contact_id = "value"  # <p>The identifier of the contact in this instance of Amazon Connect.  Supports contacts in the CHAT channel and VOICE (WebRTC) channels. For WebRTC
   calls, this should be the initial contact ID that was generated when the contact was first
   created (from the StartWebRTCContact API) in the VOICE channel</p>
}

```

---


### Contact_flow_name

ContactFlowName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the flow.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance.</p> |
| `contact_flow_id` | String | ✅ | <p>The identifier of the flow.</p> |
| `name` | String |  | <p>The name of the flow.</p> |



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


### Queue_outbound_caller_config

QueueOutboundCallerConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outbound_caller_config` | String | ✅ | <p>The outbound caller ID name, number, and outbound whisper flow.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `queue_id` | String | ✅ | <p>The identifier for the queue.</p> |



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


### Effective_hours_of_operations

EffectiveHoursOfOperations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_zone` | String | <p>The time zone for the hours of operation.</p> |
| `effective_hours_of_operation_list` | Vec<String> | <p>Information about the effective hours of operations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access effective_hours_of_operations outputs
effective_hours_of_operations_id = effective_hours_of_operations.id
effective_hours_of_operations_time_zone = effective_hours_of_operations.time_zone
effective_hours_of_operations_effective_hours_of_operation_list = effective_hours_of_operations.effective_hours_of_operation_list
```

---


### Contact_schedule

ContactSchedule resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `scheduled_time` | String | ✅ | <p>The timestamp, in Unix Epoch seconds format, at which to start running the inbound flow. The scheduled time cannot be in the past. It must be within up to 6 days in future. </p> |
| `contact_id` | String | ✅ | <p>The identifier of the contact.</p> |



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


### Queue_name

QueueName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `name` | String |  | <p>The name of the queue.</p> |
| `queue_id` | String | ✅ | <p>The identifier for the queue.</p> |
| `description` | String |  | <p>The description of the queue.</p> |



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


### Prompt_file

PromptFile resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prompt_presigned_url` | String | <p>A generated URL to the prompt that can be given to an unauthorized user so they can access
   the prompt in S3.</p> |
| `last_modified_time` | String | <p>The timestamp when this resource was last modified.</p> |
| `last_modified_region` | String | <p>The Amazon Web Services Region where this resource was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access prompt_file outputs
prompt_file_id = prompt_file.id
prompt_file_prompt_presigned_url = prompt_file.prompt_presigned_url
prompt_file_last_modified_time = prompt_file.last_modified_time
prompt_file_last_modified_region = prompt_file.last_modified_region
```

---


### Vocabulary

Vocabulary resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vocabulary_name` | String | ✅ | <p>A unique name of the custom vocabulary.</p> |
| `language_code` | String | ✅ | <p>The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see 
<a href="https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html">What is Amazon Transcribe?</a>
         </p> |
| `content` | String | ✅ | <p>The content of the custom vocabulary in plain-text format with a table of values. Each row
   in the table represents a word or a phrase, described with <code>Phrase</code>, <code>IPA</code>,
    <code>SoundsLike</code>, and <code>DisplayAs</code> fields. Separate the fields with TAB
   characters. The size limit is 50KB. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table">Create a custom
    vocabulary using a table</a>.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>. If a create request is received more than once with same client token,
   subsequent requests return the previous response without creating a vocabulary again.</p> |
| `instance_id` | String | ✅ | <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "Tags": {"key1":"value1", "key2":"value2"} }.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vocabulary` | String | <p>A list of specific words that you want Contact Lens for Amazon Connect to recognize in your audio input. They are
   generally domain-specific words and phrases, words that Contact Lens is not recognizing, or proper
   nouns.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vocabulary
vocabulary = provider.connect.Vocabulary {
    vocabulary_name = "value"  # <p>A unique name of the custom vocabulary.</p>
    language_code = "value"  # <p>The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see 
<a href="https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html">What is Amazon Transcribe?</a>
         </p>
    content = "value"  # <p>The content of the custom vocabulary in plain-text format with a table of values. Each row
   in the table represents a word or a phrase, described with <code>Phrase</code>, <code>IPA</code>,
    <code>SoundsLike</code>, and <code>DisplayAs</code> fields. Separate the fields with TAB
   characters. The size limit is 50KB. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table">Create a custom
    vocabulary using a table</a>.</p>
    instance_id = "value"  # <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
}

# Access vocabulary outputs
vocabulary_id = vocabulary.id
vocabulary_vocabulary = vocabulary.vocabulary
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple user_hierarchy resources
user_hierarchy_0 = provider.connect.User_hierarchy {
    instance_id = "value-0"
    user_id = "value-0"
}
user_hierarchy_1 = provider.connect.User_hierarchy {
    instance_id = "value-1"
    user_id = "value-1"
}
user_hierarchy_2 = provider.connect.User_hierarchy {
    instance_id = "value-2"
    user_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user_hierarchy = provider.connect.User_hierarchy {
        instance_id = "production-value"
        user_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Connect Documentation](https://docs.aws.amazon.com/connect/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
