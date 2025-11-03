# Quicksight Service



**Resources**: 68

---

## Overview

The quicksight service provides access to 68 resource types:

- [Account_subscription](#account_subscription) [CRD]
- [Dashboard_published_version](#dashboard_published_version) [U]
- [User](#user) [RUD]
- [Brand_assignment](#brand_assignment) [RUD]
- [Quick_sight_q_search_configuration](#quick_sight_q_search_configuration) [RU]
- [Action_connector_permissions](#action_connector_permissions) [RU]
- [Iam_policy_assignment](#iam_policy_assignment) [CRUD]
- [Dashboard_definition](#dashboard_definition) [R]
- [Application_with_token_exchange_grant](#application_with_token_exchange_grant) [U]
- [Account_customization](#account_customization) [CRUD]
- [Dashboard_permissions](#dashboard_permissions) [RU]
- [Analysis_permissions](#analysis_permissions) [RU]
- [Topic_permissions](#topic_permissions) [RU]
- [Analysis](#analysis) [CRUD]
- [Flow_permissions](#flow_permissions) [RU]
- [Folder](#folder) [CRUD]
- [Dashboard_snapshot_job_result](#dashboard_snapshot_job_result) [R]
- [Ip_restriction](#ip_restriction) [RU]
- [Session_embed_url](#session_embed_url) [R]
- [Group_membership](#group_membership) [CRD]
- [Template_definition](#template_definition) [R]
- [Dashboard_links](#dashboard_links) [U]
- [Asset_bundle_export_job](#asset_bundle_export_job) [R]
- [Folder_resolved_permissions](#folder_resolved_permissions) [R]
- [Dashboard](#dashboard) [CRUD]
- [Role_membership](#role_membership) [CD]
- [Data_set_permissions](#data_set_permissions) [RU]
- [Folder_permissions](#folder_permissions) [RU]
- [Topic_refresh_schedule](#topic_refresh_schedule) [CRUD]
- [Template_alias](#template_alias) [CRUD]
- [Theme](#theme) [CRUD]
- [Ingestion](#ingestion) [CR]
- [Spice_capacity_configuration](#spice_capacity_configuration) [U]
- [Action_connector](#action_connector) [CRUD]
- [Key_registration](#key_registration) [RU]
- [Custom_permissions](#custom_permissions) [CRUD]
- [Identity_propagation_config](#identity_propagation_config) [UD]
- [Q_personalization_configuration](#q_personalization_configuration) [RU]
- [Public_sharing_settings](#public_sharing_settings) [U]
- [Refresh_schedule](#refresh_schedule) [CRUD]
- [Default_q_business_application](#default_q_business_application) [RUD]
- [Dashboard_snapshot_job](#dashboard_snapshot_job) [R]
- [Folder_membership](#folder_membership) [CD]
- [Template](#template) [CRUD]
- [Asset_bundle_import_job](#asset_bundle_import_job) [R]
- [Brand](#brand) [CRUD]
- [User_by_principal_id](#user_by_principal_id) [D]
- [Vpc_connection](#vpc_connection) [CRUD]
- [Data_source](#data_source) [CRUD]
- [Theme_alias](#theme_alias) [CRUD]
- [Dashboards_qa_configuration](#dashboards_qa_configuration) [RU]
- [Data_set_refresh_properties](#data_set_refresh_properties) [CRD]
- [Group](#group) [CRUD]
- [Account_custom_permission](#account_custom_permission) [RUD]
- [Role_custom_permission](#role_custom_permission) [RUD]
- [Brand_published_version](#brand_published_version) [RU]
- [Template_permissions](#template_permissions) [RU]
- [Dashboard_embed_url](#dashboard_embed_url) [R]
- [Topic_refresh](#topic_refresh) [R]
- [Account_settings](#account_settings) [RU]
- [Data_source_permissions](#data_source_permissions) [RU]
- [Theme_permissions](#theme_permissions) [RU]
- [Analysis_definition](#analysis_definition) [R]
- [Data_set](#data_set) [CRUD]
- [Topic](#topic) [CRUD]
- [Namespace](#namespace) [CRD]
- [Flow_metadata](#flow_metadata) [R]
- [User_custom_permission](#user_custom_permission) [UD]

---

## Resources


### Account_subscription

AccountSubscription resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `admin_group` | Vec<String> |  | <p>The admin group associated with your Active Directory or IAM Identity Center account. Either this field or the <code>AdminProGroup</code> field is required if <code>ACTIVE_DIRECTORY</code> or <code>IAM_IDENTITY_CENTER</code> is the selected authentication method of the new Quick Sight account.</p>
         <p>For more information about using IAM Identity Center in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide. For more information about using Active Directory in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide.</p> |
| `first_name` | String |  | <p>The first name of the author of the Amazon Quick Sight account to use for future
            communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected
            edition of the new Amazon Quick Sight account.</p> |
| `authentication_method` | String | ✅ | <p>The method that you want to use to authenticate your Quick Sight account.</p>
         <p>If you choose <code>ACTIVE_DIRECTORY</code>, provide an <code>ActiveDirectoryName</code>
            and an <code>AdminGroup</code> associated with your Active Directory.</p>
         <p>If you choose <code>IAM_IDENTITY_CENTER</code>, provide an <code>AdminGroup</code> associated with your IAM Identity Center account.</p> |
| `admin_pro_group` | Vec<String> |  | <p>The admin pro group associated with your Active Directory or IAM Identity Center account. Either this field or the <code>AdminGroup</code> field is required if <code>ACTIVE_DIRECTORY</code> or <code>IAM_IDENTITY_CENTER</code> is the selected authentication method of the new Quick Sight account.</p>
         <p>For more information about using IAM Identity Center in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide. For more information about using Active Directory in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide.</p> |
| `author_group` | Vec<String> |  | <p>The author group associated with your Active Directory or IAM Identity Center account.</p>
         <p>For more information about using IAM Identity Center in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide. For more information about using Active Directory in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide.</p> |
| `author_pro_group` | Vec<String> |  | <p>The author pro group associated with your Active Directory or IAM Identity Center account.</p>
         <p>For more information about using IAM Identity Center in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide. For more information about using Active Directory in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide.</p> |
| `iam_identity_center_instance_arn` | String |  | <p>The Amazon Resource Name (ARN) for the IAM Identity Center instance.</p> |
| `last_name` | String |  | <p>The last name of the author of the Amazon Quick Sight account to use for future
            communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected
            edition of the new Amazon Quick Sight account.</p> |
| `realm` | String |  | <p>The realm of the Active Directory that is associated with your Quick Sight account. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Quick Sight account.</p> |
| `account_name` | String | ✅ | <p>The name of your Amazon Quick Sight account. This name is unique over all of Amazon Web Services, and it appears only when users sign in. You can't change
                <code>AccountName</code> value after the Amazon Quick Sight account is
            created.</p> |
| `email_address` | String |  | <p>The email address of the author of the Amazon Quick Sight account to use for future
            communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected
            edition of the new Amazon Quick Sight account.</p> |
| `reader_group` | Vec<String> |  | <p>The reader group associated with your Active Directory or IAM Identity Center account.</p>
         <p>For more information about using IAM Identity Center in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide. For more information about using Active Directory in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide.</p> |
| `edition` | String |  | <p>The edition of Amazon Quick Sight that you want your account to have. Currently, you can
            choose from <code>ENTERPRISE</code> or
                <code>ENTERPRISE_AND_Q</code>.</p>
         <p>If you choose <code>ENTERPRISE_AND_Q</code>, the following parameters are
            required:</p>
         <ul>
            <li>
               <p>
                  <code>FirstName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LastName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>EmailAddress</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ContactNumber</code>
               </p>
            </li>
         </ul> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID of the account that you're using to create your Quick Sight account.</p> |
| `active_directory_name` | String |  | <p>The name of your Active Directory. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Quick Sight account.</p> |
| `directory_id` | String |  | <p>The ID of the Active Directory that is associated with your Quick Sight account.</p> |
| `contact_number` | String |  | <p>A 10-digit phone number for the author of the Amazon Quick Sight account to use for
            future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the
            selected edition of the new Amazon Quick Sight account.</p> |
| `notification_email` | String | ✅ | <p>The email address that you want Quick Sight to send notifications to regarding your Quick Sight account or Quick Sight subscription.</p> |
| `reader_pro_group` | Vec<String> |  | <p>The reader pro group associated with your Active Directory or IAM Identity Center account.</p>
         <p>For more information about using IAM Identity Center in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide. For more information about using Active Directory in Amazon Quick Sight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon Quick Sight Enterprise Edition</a> in the Amazon Quick Sight User Guide.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `account_info` | String | <p>A structure that contains the following elements:</p>
         <ul>
            <li>
               <p>Your Quick Sight account name.</p>
            </li>
            <li>
               <p>The edition of Quick Sight that your account is using.</p>
            </li>
            <li>
               <p>The notification email address that is associated with the Amazon Quick Sight
                    account.
            </p>
            </li>
            <li>
               <p>The authentication type of the Quick Sight account.</p>
            </li>
            <li>
               <p>The status of the Quick Sight account's subscription.</p>
            </li>
         </ul> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_subscription
account_subscription = provider.quicksight.Account_subscription {
    authentication_method = "value"  # <p>The method that you want to use to authenticate your Quick Sight account.</p>
         <p>If you choose <code>ACTIVE_DIRECTORY</code>, provide an <code>ActiveDirectoryName</code>
            and an <code>AdminGroup</code> associated with your Active Directory.</p>
         <p>If you choose <code>IAM_IDENTITY_CENTER</code>, provide an <code>AdminGroup</code> associated with your IAM Identity Center account.</p>
    account_name = "value"  # <p>The name of your Amazon Quick Sight account. This name is unique over all of Amazon Web Services, and it appears only when users sign in. You can't change
                <code>AccountName</code> value after the Amazon Quick Sight account is
            created.</p>
    aws_account_id = "value"  # <p>The Amazon Web Services account ID of the account that you're using to create your Quick Sight account.</p>
    notification_email = "value"  # <p>The email address that you want Quick Sight to send notifications to regarding your Quick Sight account or Quick Sight subscription.</p>
}

# Access account_subscription outputs
account_subscription_id = account_subscription.id
account_subscription_request_id = account_subscription.request_id
account_subscription_account_info = account_subscription.account_info
account_subscription_status = account_subscription.status
```

---


### Dashboard_published_version

DashboardPublishedVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_number` | i64 | ✅ | <p>The version number of the dashboard.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the dashboard that you're
            updating.</p> |
| `dashboard_id` | String | ✅ | <p>The ID for the dashboard.</p> |



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

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_permissions_name` | String |  | <p>(Enterprise edition only) The name of the custom permissions profile that you want to
            assign to this user. Customized permissions allows you to control a user's access by
            restricting access the following operations:</p>
         <ul>
            <li>
               <p>Create and update data sources</p>
            </li>
            <li>
               <p>Create and update datasets</p>
            </li>
            <li>
               <p>Create and update email reports</p>
            </li>
            <li>
               <p>Subscribe to email reports</p>
            </li>
         </ul>
         <p>A set of custom permissions includes any combination of these restrictions. Currently,
            you need to create the profile names for custom permission sets by using the Quick Sight
            console. Then, you use the <code>RegisterUser</code> API operation to assign the named set of
            permissions to a Quick Sight user. </p>
         <p>Quick Sight custom permissions are applied through IAM policies. Therefore, they
            override the permissions typically granted by assigning Quick Sight users to one of the
            default security cohorts in Quick Sight (admin, author, reader).</p>
         <p>This feature is available only to Quick Sight Enterprise edition subscriptions.</p> |
| `role` | String | ✅ | <p>The Amazon Quick Sight role of the user. The role can be one of the
			following default security cohorts:</p>
         <ul>
            <li>
               <p>
                  <code>READER</code>: A user who has read-only access to dashboards.</p>
            </li>
            <li>
               <p>
                  <code>AUTHOR</code>: A user who can create data sources, datasets, analyses, and
					dashboards.</p>
            </li>
            <li>
               <p>
                  <code>ADMIN</code>: A user who is an author, who can also manage Amazon Quick Sight
					settings.</p>
            </li>
            <li>
               <p>
                  <code>READER_PRO</code>: Reader Pro adds Generative BI capabilities to the Reader role. Reader Pros have access to Amazon Q in Quick Sight, can build stories with Amazon Q, and can generate executive summaries from dashboards.</p>
            </li>
            <li>
               <p>
                  <code>AUTHOR_PRO</code>: Author Pro adds Generative BI capabilities to the Author role. Author Pros can author dashboards with natural language with Amazon Q, build stories with Amazon Q, create Topics for Q&A, and generate executive summaries from dashboards.</p>
            </li>
            <li>
               <p>
                  <code>ADMIN_PRO</code>: Admin Pros are Author Pros who can also manage Amazon Quick Sight administrative settings. Admin Pro users are billed at Author Pro pricing.</p>
            </li>
         </ul>
         <p>The name of the Quick Sight role is invisible to the user except for the console
	        screens dealing with permissions.</p> |
| `external_login_federation_provider_type` | String |  | <p>The type of supported external login provider that provides identity to let a user federate into Quick Sight with an associated Identity and Access Management(IAM) role. The type of supported external login provider can be one of the following.</p>
         <ul>
            <li>
               <p>
                  <code>COGNITO</code>: Amazon Cognito. The provider URL is cognito-identity.amazonaws.com. When choosing the <code>COGNITO</code> provider type, don’t use the "CustomFederationProviderUrl" parameter which is only needed when the external provider is custom.</p>
            </li>
            <li>
               <p>
                  <code>CUSTOM_OIDC</code>: Custom OpenID Connect (OIDC) provider. When choosing <code>CUSTOM_OIDC</code> type, use the <code>CustomFederationProviderUrl</code> parameter to provide the custom OIDC provider URL.</p>
            </li>
            <li>
               <p>
                  <code>NONE</code>: This clears all the previously saved external login information for a user. Use the
          <code>
                     <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DescribeUser.html">DescribeUser</a>
                  </code>
          API operation to check the external login information.</p>
            </li>
         </ul> |
| `external_login_id` | String |  | <p>The identity ID for a user in the external login provider.</p> |
| `user_name` | String | ✅ | <p>The Amazon Quick Sight user name that you want to update.</p> |
| `namespace` | String | ✅ | <p>The namespace. Currently, you should set this to <code>default</code>.</p> |
| `unapply_custom_permissions` | bool |  | <p>A flag that you use to indicate that you want to remove all custom permissions
            from this user. Using this parameter resets the user to the state
            it was in before a custom permissions profile was applied. This parameter defaults to
            NULL and it doesn't accept any other value.</p> |
| `custom_federation_provider_url` | String |  | <p>The URL of the custom OpenID Connect (OIDC) provider that provides identity to let a user federate
         into Quick Sight with an associated Identity and Access Management(IAM) role. This parameter should
         only be used when <code>ExternalLoginFederationProviderType</code> parameter is set to <code>CUSTOM_OIDC</code>.</p> |
| `email` | String | ✅ | <p>The email address of the user that you want to update.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that the user is in. Currently, you use the ID for the
			Amazon Web Services account that contains your Amazon Quick Sight account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | String | <p>The user name.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user outputs
user_id = user.id
user_user = user.user
user_request_id = user.request_id
user_status = user.status
```

---


### Brand_assignment

BrandAssignment resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `brand_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the brand.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that owns the brand assignment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `brand_arn` | String | <p>The Amazon Resource Name (ARN) of the brand.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access brand_assignment outputs
brand_assignment_id = brand_assignment.id
brand_assignment_request_id = brand_assignment.request_id
brand_assignment_brand_arn = brand_assignment.brand_arn
```

---


### Quick_sight_q_search_configuration

QuickSightQSearchConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the Quick Sight Q Search configuration that you want to update.</p> |
| `q_search_status` | String | ✅ | <p>The status of the Quick Sight Q Search configuration that the user wants to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `q_search_status` | String | <p>The status of Quick Sight Q Search configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access quick_sight_q_search_configuration outputs
quick_sight_q_search_configuration_id = quick_sight_q_search_configuration.id
quick_sight_q_search_configuration_status = quick_sight_q_search_configuration.status
quick_sight_q_search_configuration_request_id = quick_sight_q_search_configuration.request_id
quick_sight_q_search_configuration_q_search_status = quick_sight_q_search_configuration.q_search_status
```

---


### Action_connector_permissions

ActionConnectorPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_connector_id` | String | ✅ | <p>The unique identifier of the action connector whose permissions you want to update.</p> |
| `grant_permissions` | Vec<String> |  | <p>The permissions to grant to users and groups for this action connector.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID that contains the action connector.</p> |
| `revoke_permissions` | Vec<String> |  | <p>The permissions to revoke from users and groups for this action connector.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `action_connector_id` | String | <p>The unique identifier of the action connector.</p> |
| `permissions` | Vec<String> | <p>The list of permissions associated with the action connector, including the principals and their allowed actions.</p> |
| `status` | i64 | <p>The HTTP status code of the request.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the action connector.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access action_connector_permissions outputs
action_connector_permissions_id = action_connector_permissions.id
action_connector_permissions_request_id = action_connector_permissions.request_id
action_connector_permissions_action_connector_id = action_connector_permissions.action_connector_id
action_connector_permissions_permissions = action_connector_permissions.permissions
action_connector_permissions_status = action_connector_permissions.status
action_connector_permissions_arn = action_connector_permissions.arn
```

---


### Iam_policy_assignment

IAMPolicyAssignment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `namespace` | String | ✅ | <p>The namespace that contains the assignment.</p> |
| `assignment_status` | String | ✅ | <p>The status of the assignment. Possible values are as follows:</p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> - Anything specified in this assignment is used when
					creating the data source.</p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> - This assignment isn't used when creating the data
					source.</p>
            </li>
            <li>
               <p>
                  <code>DRAFT</code> - This assignment is an unfinished draft and isn't used
					when creating the data source.</p>
            </li>
         </ul> |
| `identities` | HashMap<String, Vec<String>> |  | <p>The Amazon Quick Sight users, groups, or both that you want to assign the policy
			to.</p> |
| `policy_arn` | String |  | <p>The ARN for the IAM policy to apply to the Amazon Quick Sight users and
			groups specified in this assignment.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account where you want to assign an IAM policy to Amazon Quick Sight users or groups.</p> |
| `assignment_name` | String | ✅ | <p>The name of the assignment, also called a rule.
			The
			name
			must be unique within the
			Amazon Web Services account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `iam_policy_assignment` | String | <p>Information describing the IAM policy assignment.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create iam_policy_assignment
iam_policy_assignment = provider.quicksight.Iam_policy_assignment {
    namespace = "value"  # <p>The namespace that contains the assignment.</p>
    assignment_status = "value"  # <p>The status of the assignment. Possible values are as follows:</p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> - Anything specified in this assignment is used when
					creating the data source.</p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> - This assignment isn't used when creating the data
					source.</p>
            </li>
            <li>
               <p>
                  <code>DRAFT</code> - This assignment is an unfinished draft and isn't used
					when creating the data source.</p>
            </li>
         </ul>
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account where you want to assign an IAM policy to Amazon Quick Sight users or groups.</p>
    assignment_name = "value"  # <p>The name of the assignment, also called a rule.
			The
			name
			must be unique within the
			Amazon Web Services account.</p>
}

# Access iam_policy_assignment outputs
iam_policy_assignment_id = iam_policy_assignment.id
iam_policy_assignment_status = iam_policy_assignment.status
iam_policy_assignment_iam_policy_assignment = iam_policy_assignment.iam_policy_assignment
iam_policy_assignment_request_id = iam_policy_assignment.request_id
```

---


### Dashboard_definition

DashboardDefinition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `dashboard_publish_options` | String | <p>Options for publishing the dashboard:</p>
         <ul>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This
                    status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is
                    set to <code>DISABLED</code>, Amazon Quick Sight disables the left filter pane on
                    the published dashboard, which can be used for ad hoc (one-time) filtering. This
                    option is <code>ENABLED</code> by default. </p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This
                    status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual
                    option to export data to .CSV format isn't enabled when this is set to
                        <code>DISABLED</code>. This option is <code>ENABLED</code> by default.
                </p>
            </li>
            <li>
               <p>
                  <code>VisibilityState</code> for <code>SheetControlsOption</code> - This
                    visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>.
                    This option is <code>COLLAPSED</code> by default. </p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>QuickSuiteActionsOption</code> -
                    This status can be either <code>ENABLED</code> or <code>DISABLED</code>.
                    Features related to Actions in Amazon Quick Suite on dashboards are disabled
                    when this is set to <code>DISABLED</code>. This option is <code>DISABLED</code>
                    by default.</p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>ExecutiveSummaryOption</code> - This
                    status can be either <code>ENABLED</code> or <code>DISABLED</code>. The option
                    to build an executive summary is disabled when this is set to
                        <code>DISABLED</code>. This option is <code>ENABLED</code> by
                    default.</p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>DataStoriesSharingOption</code> -
                    This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The
                    option to share a data story is disabled when this is set to
                        <code>DISABLED</code>. This option is <code>ENABLED</code> by
                    default.</p>
            </li>
         </ul> |
| `definition` | String | <p>The definition of a dashboard.</p>
         <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p> |
| `errors` | Vec<String> | <p>Errors associated with this dashboard version.</p> |
| `name` | String | <p>The display name of the dashboard.</p> |
| `dashboard_id` | String | <p>The ID of the dashboard described.</p> |
| `theme_arn` | String | <p>The ARN of the theme of the dashboard.</p> |
| `resource_status` | String | <p>Status associated with the dashboard version.</p>
         <ul>
            <li>
               <p>
                  <code>CREATION_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATION_SUCCESSFUL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATION_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_SUCCESSFUL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETED</code>
               </p>
            </li>
         </ul> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dashboard_definition outputs
dashboard_definition_id = dashboard_definition.id
dashboard_definition_status = dashboard_definition.status
dashboard_definition_dashboard_publish_options = dashboard_definition.dashboard_publish_options
dashboard_definition_definition = dashboard_definition.definition
dashboard_definition_errors = dashboard_definition.errors
dashboard_definition_name = dashboard_definition.name
dashboard_definition_dashboard_id = dashboard_definition.dashboard_id
dashboard_definition_theme_arn = dashboard_definition.theme_arn
dashboard_definition_resource_status = dashboard_definition.resource_status
dashboard_definition_request_id = dashboard_definition.request_id
```

---


### Application_with_token_exchange_grant

ApplicationWithTokenExchangeGrant resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `namespace` | String | ✅ | <p>The namespace of the QuickSight application.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account to be updated with a token exchange grant.</p> |



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


### Account_customization

AccountCustomization resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that you want to customize Quick Sight for.</p> |
| `account_customization` | String | ✅ | <p>The Quick Sight customizations you're adding. You can add
            these to an Amazon Web Services account and a QuickSight namespace. </p>
         <p>For example, you can add a default theme by setting <code>AccountCustomization</code>
            to the midnight theme: <code>"AccountCustomization": { "DefaultTheme":
                "arn:aws:quicksight::aws:theme/MIDNIGHT" }</code>. Or, you can add a custom theme by
            specifying <code>"AccountCustomization": { "DefaultTheme":
                "arn:aws:quicksight:us-west-2:111122223333:theme/bdb844d0-0fe9-4d9d-b520-0fe602d93639"
                }</code>. </p> |
| `tags` | Vec<String> |  | <p>A list of the tags that you want to attach to this resource.</p> |
| `namespace` | String |  | <p>The Quick Sight namespace that you want to add customizations to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_customization` | String | <p>The Quick Sight customizations that exist. </p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `aws_account_id` | String | <p>The ID for the Amazon Web Services account that you're describing.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the customization that's associated with this Amazon Web Services account.</p> |
| `namespace` | String | <p>The Quick Sight namespace that you're describing. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_customization
account_customization = provider.quicksight.Account_customization {
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account that you want to customize Quick Sight for.</p>
    account_customization = "value"  # <p>The Quick Sight customizations you're adding. You can add
            these to an Amazon Web Services account and a QuickSight namespace. </p>
         <p>For example, you can add a default theme by setting <code>AccountCustomization</code>
            to the midnight theme: <code>"AccountCustomization": { "DefaultTheme":
                "arn:aws:quicksight::aws:theme/MIDNIGHT" }</code>. Or, you can add a custom theme by
            specifying <code>"AccountCustomization": { "DefaultTheme":
                "arn:aws:quicksight:us-west-2:111122223333:theme/bdb844d0-0fe9-4d9d-b520-0fe602d93639"
                }</code>. </p>
}

# Access account_customization outputs
account_customization_id = account_customization.id
account_customization_account_customization = account_customization.account_customization
account_customization_request_id = account_customization.request_id
account_customization_status = account_customization.status
account_customization_aws_account_id = account_customization.aws_account_id
account_customization_arn = account_customization.arn
account_customization_namespace = account_customization.namespace
```

---


### Dashboard_permissions

DashboardPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grant_link_permissions` | Vec<String> |  | <p>Grants link permissions to all users in a defined namespace.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the dashboard whose permissions
            you're updating.</p> |
| `dashboard_id` | String | ✅ | <p>The ID for the dashboard.</p> |
| `grant_permissions` | Vec<String> |  | <p>The permissions that you want to grant on this resource.</p> |
| `revoke_permissions` | Vec<String> |  | <p>The permissions that you want to revoke from this resource.</p> |
| `revoke_link_permissions` | Vec<String> |  | <p>Revokes link permissions from all users in a defined namespace.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dashboard_id` | String | <p>The ID for the dashboard.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `dashboard_arn` | String | <p>The Amazon Resource Name (ARN) of the dashboard.</p> |
| `link_sharing_configuration` | String | <p>A structure that contains the configuration of a shareable link that grants access to
            the dashboard. Your users can use the link to view and interact with the dashboard, if
            the dashboard has been shared with them. For more information about sharing dashboards,
            see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sharing-a-dashboard.html">Sharing Dashboards</a>.</p> |
| `permissions` | Vec<String> | <p>A structure that contains the permissions for the dashboard.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dashboard_permissions outputs
dashboard_permissions_id = dashboard_permissions.id
dashboard_permissions_dashboard_id = dashboard_permissions.dashboard_id
dashboard_permissions_status = dashboard_permissions.status
dashboard_permissions_dashboard_arn = dashboard_permissions.dashboard_arn
dashboard_permissions_link_sharing_configuration = dashboard_permissions.link_sharing_configuration
dashboard_permissions_permissions = dashboard_permissions.permissions
dashboard_permissions_request_id = dashboard_permissions.request_id
```

---


### Analysis_permissions

AnalysisPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're
            updating. You must be using the Amazon Web Services account that the analysis is in.</p> |
| `revoke_permissions` | Vec<String> |  | <p>A structure that describes the permissions to remove and the principal to remove them
            from.</p> |
| `grant_permissions` | Vec<String> |  | <p>A structure that describes the permissions to add and the principal to add them
            to.</p> |
| `analysis_id` | String | ✅ | <p>The ID of the analysis whose permissions you're updating. The ID is part of the
            analysis URL.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `analysis_arn` | String | <p>The Amazon Resource Name (ARN) of the analysis whose permissions you're
            describing.</p> |
| `permissions` | Vec<String> | <p>A structure that describes the principals and the resource-level permissions on an
            analysis.</p> |
| `analysis_id` | String | <p>The ID of the analysis whose permissions you're describing.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access analysis_permissions outputs
analysis_permissions_id = analysis_permissions.id
analysis_permissions_analysis_arn = analysis_permissions.analysis_arn
analysis_permissions_permissions = analysis_permissions.permissions
analysis_permissions_analysis_id = analysis_permissions.analysis_id
analysis_permissions_status = analysis_permissions.status
analysis_permissions_request_id = analysis_permissions.request_id
```

---


### Topic_permissions

TopicPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grant_permissions` | Vec<String> |  | <p>The resource permissions that you want to grant to the topic.</p> |
| `topic_id` | String | ✅ | <p>The ID of the topic that you want to modify. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the topic that you want to update
         the permissions for.</p> |
| `revoke_permissions` | Vec<String> |  | <p>The resource permissions that you want to revoke from the topic.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permissions` | Vec<String> | <p>A list of resource permissions that are configured to the topic.</p> |
| `topic_arn` | String | <p>The Amazon Resource Name (ARN) of the topic.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `topic_id` | String | <p>The ID of the topic that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access topic_permissions outputs
topic_permissions_id = topic_permissions.id
topic_permissions_permissions = topic_permissions.permissions
topic_permissions_topic_arn = topic_permissions.topic_arn
topic_permissions_status = topic_permissions.status
topic_permissions_topic_id = topic_permissions.topic_id
topic_permissions_request_id = topic_permissions.request_id
```

---


### Analysis

Analysis resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account where you are creating an analysis.</p> |
| `name` | String | ✅ | <p>A descriptive name for the analysis that you're creating. This name displays for the
            analysis in the Amazon Quick Sight console. </p> |
| `permissions` | Vec<String> |  | <p>A structure that describes the principals and the resource-level permissions on an
            analysis. You can use the <code>Permissions</code> structure to grant permissions by
            providing a list of Identity and Access Management (IAM) action information for each
            principal listed by Amazon Resource Name (ARN). </p>
         <p>To specify no permissions, omit <code>Permissions</code>.</p> |
| `validation_strategy` | String |  | <p>The option to relax the validation needed to create an analysis with definition objects. This skips the validation step for specific errors.</p> |
| `folder_arns` | Vec<String> |  | <p>When you create the analysis, Amazon Quick Sight adds the analysis to these folders.</p> |
| `tags` | Vec<String> |  | <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the
            analysis.</p> |
| `analysis_id` | String | ✅ | <p>The ID for the analysis that you're creating. This ID displays in the URL of the
            analysis.</p> |
| `parameters` | String |  | <p>The parameter names and override values that you want to use. An analysis can have 
            any parameter type, and some parameters might accept multiple values. </p> |
| `source_entity` | String |  | <p>A source entity to use for the analysis that you're creating. This metadata structure
            contains details that describe a source template and one or more datasets.</p>
         <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in 
            order for the request to be valid.</p> |
| `theme_arn` | String |  | <p>The ARN for the theme to apply to the analysis that you're creating. To see the theme
            in the Amazon Quick Sight console, make sure that you have access to it.</p> |
| `definition` | String |  | <p>The definition of an analysis.</p>
         <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
         <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in 
            order for the request to be valid.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `analysis` | String | <p>A metadata structure that contains summary information for the analysis that you're
            describing.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create analysis
analysis = provider.quicksight.Analysis {
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account where you are creating an analysis.</p>
    name = "value"  # <p>A descriptive name for the analysis that you're creating. This name displays for the
            analysis in the Amazon Quick Sight console. </p>
    analysis_id = "value"  # <p>The ID for the analysis that you're creating. This ID displays in the URL of the
            analysis.</p>
}

# Access analysis outputs
analysis_id = analysis.id
analysis_request_id = analysis.request_id
analysis_analysis = analysis.analysis
analysis_status = analysis.status
```

---


### Flow_permissions

FlowPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the flow you are updating permissions against.</p> |
| `grant_permissions` | Vec<String> |  | <p>The permissions that you want to grant on this flow.</p> |
| `flow_id` | String | ✅ | <p>The unique identifier of the flow to update permissions for.</p> |
| `revoke_permissions` | Vec<String> |  | <p>The permissions that you want to revoke from this flow.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The Amazon Resource Name (ARN) of the flow you are getting permissions against.</p> |
| `permissions` | Vec<String> | <p>A structure that contains the permissions for the flow.</p> |
| `flow_id` | String | <p>The unique identifier of the flow with permissions.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flow_permissions outputs
flow_permissions_id = flow_permissions.id
flow_permissions_arn = flow_permissions.arn
flow_permissions_permissions = flow_permissions.permissions
flow_permissions_flow_id = flow_permissions.flow_id
flow_permissions_request_id = flow_permissions.request_id
flow_permissions_status = flow_permissions.status
```

---


### Folder

Folder resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Tags for the folder.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account where you want to create the folder.</p> |
| `folder_id` | String | ✅ | <p>The ID of the folder.</p> |
| `folder_type` | String |  | <p>The type of folder. By default, <code>folderType</code> is <code>SHARED</code>.</p> |
| `parent_folder_arn` | String |  | <p>The Amazon Resource Name (ARN) for the parent folder.</p>
         <p>
            <code>ParentFolderArn</code> can be null. An empty <code>parentFolderArn</code> creates a root-level folder.</p> |
| `sharing_model` | String |  | <p>An optional parameter that determines the sharing scope of the folder. The default value for this parameter is <code>ACCOUNT</code>.</p> |
| `name` | String |  | <p>The name of the folder.</p> |
| `permissions` | Vec<String> |  | <p>A structure that describes the principals and the resource-level permissions of a folder.</p>
         <p>To specify no permissions, omit <code>Permissions</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `folder` | String | <p>Information about the folder.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create folder
folder = provider.quicksight.Folder {
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account where you want to create the folder.</p>
    folder_id = "value"  # <p>The ID of the folder.</p>
}

# Access folder outputs
folder_id = folder.id
folder_request_id = folder.request_id
folder_status = folder.status
folder_folder = folder.folder
```

---


### Dashboard_snapshot_job_result

DashboardSnapshotJobResult resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_time` | String | <p>The time that a snapshot job was created.</p> |
| `error_info` | String | <p>Displays information for the error that caused a job to fail.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) for the snapshot job. The job ARN is generated when you start a new job with a <code>StartDashboardSnapshotJob</code> API call.</p> |
| `job_status` | String | <p>Indicates the status of a job after it has reached a terminal state. A finished snapshot job will retuen a <code>COMPLETED</code> or <code>FAILED</code> status.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request</p> |
| `last_updated_time` | String | <p>The time that a snapshot job status was last updated.</p> |
| `result` | String | <p>The result of the snapshot job. Jobs that have successfully completed will return the S3Uri where they are located. Jobs that have failedwill return information on the error that caused the job to fail.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dashboard_snapshot_job_result outputs
dashboard_snapshot_job_result_id = dashboard_snapshot_job_result.id
dashboard_snapshot_job_result_created_time = dashboard_snapshot_job_result.created_time
dashboard_snapshot_job_result_error_info = dashboard_snapshot_job_result.error_info
dashboard_snapshot_job_result_arn = dashboard_snapshot_job_result.arn
dashboard_snapshot_job_result_job_status = dashboard_snapshot_job_result.job_status
dashboard_snapshot_job_result_request_id = dashboard_snapshot_job_result.request_id
dashboard_snapshot_job_result_status = dashboard_snapshot_job_result.status
dashboard_snapshot_job_result_last_updated_time = dashboard_snapshot_job_result.last_updated_time
dashboard_snapshot_job_result_result = dashboard_snapshot_job_result.result
```

---


### Ip_restriction

IpRestriction resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the IP rules.</p> |
| `ip_restriction_rule_map` | HashMap<String, String> |  | <p>A map that describes the updated IP rules with CIDR ranges and descriptions.</p> |
| `vpc_id_restriction_rule_map` | HashMap<String, String> |  | <p>A map of VPC IDs and their corresponding rules. When you configure this parameter, traffic from all VPC endpoints that are present in the specified VPC is allowed.</p> |
| `vpc_endpoint_id_restriction_rule_map` | HashMap<String, String> |  | <p>A map of allowed VPC endpoint IDs and their corresponding rule descriptions.</p> |
| `enabled` | bool |  | <p>A value that specifies whether IP rules are turned on.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `ip_restriction_rule_map` | HashMap<String, String> | <p>A map that describes the IP rules with CIDR range and description.</p> |
| `status` | i64 | <p>The HTTP status of the request.
			</p> |
| `enabled` | bool | <p>A value that specifies whether IP rules are turned on.</p> |
| `vpc_id_restriction_rule_map` | HashMap<String, String> | <p>A map of allowed VPC IDs and their rule descriptions.</p> |
| `vpc_endpoint_id_restriction_rule_map` | HashMap<String, String> | <p>A map of allowed VPC endpoint IDs and their rule descriptions.</p> |
| `aws_account_id` | String | <p>The ID of the Amazon Web Services account that contains the IP rules.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ip_restriction outputs
ip_restriction_id = ip_restriction.id
ip_restriction_request_id = ip_restriction.request_id
ip_restriction_ip_restriction_rule_map = ip_restriction.ip_restriction_rule_map
ip_restriction_status = ip_restriction.status
ip_restriction_enabled = ip_restriction.enabled
ip_restriction_vpc_id_restriction_rule_map = ip_restriction.vpc_id_restriction_rule_map
ip_restriction_vpc_endpoint_id_restriction_rule_map = ip_restriction.vpc_endpoint_id_restriction_rule_map
ip_restriction_aws_account_id = ip_restriction.aws_account_id
```

---


### Session_embed_url

SessionEmbedUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `embed_url` | String | <p>A single-use URL that you can put into your server-side web page to embed your QuickSight session. This URL is valid for 5 minutes. The API operation provides the
            URL with an <code>auth_code</code> value that enables one (and only one) sign-on to a
            user session that is valid for 10 hours. </p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access session_embed_url outputs
session_embed_url_id = session_embed_url.id
session_embed_url_embed_url = session_embed_url.embed_url
session_embed_url_status = session_embed_url.status
session_embed_url_request_id = session_embed_url.request_id
```

---


### Group_membership

GroupMembership resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the 
			Amazon Web Services account that contains your Amazon Quick Sight account.</p> |
| `group_name` | String | ✅ | <p>The name of the group that you want to add the user to.</p> |
| `member_name` | String | ✅ | <p>The name of the user that you want to add to the group membership.</p> |
| `namespace` | String | ✅ | <p>The namespace that you want the user to be a part of.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `group_member` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group_membership
group_membership = provider.quicksight.Group_membership {
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the 
			Amazon Web Services account that contains your Amazon Quick Sight account.</p>
    group_name = "value"  # <p>The name of the group that you want to add the user to.</p>
    member_name = "value"  # <p>The name of the user that you want to add to the group membership.</p>
    namespace = "value"  # <p>The namespace that you want the user to be a part of.</p>
}

# Access group_membership outputs
group_membership_id = group_membership.id
group_membership_request_id = group_membership.request_id
group_membership_status = group_membership.status
group_membership_group_member = group_membership.group_member
```

---


### Template_definition

TemplateDefinition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `name` | String | <p>The descriptive name of the template.</p> |
| `resource_status` | String | <p>Status associated with the template.</p>
         <ul>
            <li>
               <p>
                  <code>CREATION_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATION_SUCCESSFUL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATION_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_SUCCESSFUL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETED</code>
               </p>
            </li>
         </ul> |
| `errors` | Vec<String> | <p>Errors associated with the template version.</p> |
| `theme_arn` | String | <p>The ARN of the theme of the template.</p> |
| `definition` | String | <p>The definition of the template.</p>
         <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p> |
| `template_id` | String | <p>The ID of the template described.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access template_definition outputs
template_definition_id = template_definition.id
template_definition_status = template_definition.status
template_definition_name = template_definition.name
template_definition_resource_status = template_definition.resource_status
template_definition_errors = template_definition.errors
template_definition_theme_arn = template_definition.theme_arn
template_definition_definition = template_definition.definition
template_definition_template_id = template_definition.template_id
template_definition_request_id = template_definition.request_id
```

---


### Dashboard_links

DashboardLinks resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the dashboard whose links you
            want to update.</p> |
| `dashboard_id` | String | ✅ | <p>The ID for the dashboard.</p> |
| `link_entities` | Vec<String> | ✅ | <p> list of analysis Amazon Resource Names (ARNs) to be linked to the dashboard.</p> |



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


### Asset_bundle_export_job

AssetBundleExportJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `errors` | Vec<String> | <p>An array of error records that describes any failures that occurred during the export
         job processing.</p>
         <p>Error records accumulate while the job runs. The complete set of error records is
         available after the job has completed and failed.</p> |
| `resource_arns` | Vec<String> | <p>A list of resource ARNs that exported with the job.</p> |
| `include_tags` | bool | <p>The include tags flag.</p> |
| `asset_bundle_export_job_id` | String | <p>The ID of the job. The job ID is set when you start a new job with a
            <code>StartAssetBundleExportJob</code> API call.</p> |
| `include_folder_memberships` | bool | <p>The include folder memberships flag.</p> |
| `cloud_formation_override_property_configuration` | String | <p>The CloudFormation override property configuration for the export job.</p> |
| `created_time` | String | <p>The time that the export job was created.</p> |
| `include_all_dependencies` | bool | <p>The include dependencies flag.</p> |
| `download_url` | String | <p>The URL to download the exported asset bundle data from.</p>
         <p>This URL is available only after the job has succeeded. This URL is valid for 5 minutes
         after issuance. Call <code>DescribeAssetBundleExportJob</code> again for a fresh URL if
         needed.</p>
         <p>The downloaded asset bundle is a zip file named <code>assetbundle-{jobId}.qs</code>. The
         file has a <code>.qs</code> extension.</p>
         <p>This URL can't be used in a <code>StartAssetBundleImportJob</code> API call and
         should only be used for download purposes.</p> |
| `status` | i64 | <p>The HTTP status of the response.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) for the export job.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `include_permissions` | bool | <p>The include permissions flag.</p> |
| `warnings` | Vec<String> | <p>An array of warning records that describe the analysis or dashboard that is exported.
         This array includes UI errors that can be skipped during the validation process.</p>
         <p>This property only appears if <code>StrictModeForAllResources</code> in
            <code>ValidationStrategy</code> is set to <code>FALSE</code>.</p> |
| `include_folder_members` | String | <p>A setting that determines whether folder members are included.</p> |
| `validation_strategy` | String | <p>The validation strategy that is used to export the analysis or dashboard.</p> |
| `aws_account_id` | String | <p>The ID of the Amazon Web Services account that the export job was executed in. </p> |
| `export_format` | String | <p>The format of the exported asset bundle. A <code>QUICKSIGHT_JSON</code> formatted file
         can be used to make a <code>StartAssetBundleImportJob</code> API call. A
            <code>CLOUDFORMATION_JSON</code> formatted file can be used in the CloudFormation
         console and with the CloudFormation APIs.</p> |
| `job_status` | String | <p>Indicates the status of a job through its queuing and execution.</p>
         <p>Poll this <code>DescribeAssetBundleExportApi</code> until <code>JobStatus</code> is
         either <code>SUCCESSFUL</code> or <code>FAILED</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_bundle_export_job outputs
asset_bundle_export_job_id = asset_bundle_export_job.id
asset_bundle_export_job_errors = asset_bundle_export_job.errors
asset_bundle_export_job_resource_arns = asset_bundle_export_job.resource_arns
asset_bundle_export_job_include_tags = asset_bundle_export_job.include_tags
asset_bundle_export_job_asset_bundle_export_job_id = asset_bundle_export_job.asset_bundle_export_job_id
asset_bundle_export_job_include_folder_memberships = asset_bundle_export_job.include_folder_memberships
asset_bundle_export_job_cloud_formation_override_property_configuration = asset_bundle_export_job.cloud_formation_override_property_configuration
asset_bundle_export_job_created_time = asset_bundle_export_job.created_time
asset_bundle_export_job_include_all_dependencies = asset_bundle_export_job.include_all_dependencies
asset_bundle_export_job_download_url = asset_bundle_export_job.download_url
asset_bundle_export_job_status = asset_bundle_export_job.status
asset_bundle_export_job_arn = asset_bundle_export_job.arn
asset_bundle_export_job_request_id = asset_bundle_export_job.request_id
asset_bundle_export_job_include_permissions = asset_bundle_export_job.include_permissions
asset_bundle_export_job_warnings = asset_bundle_export_job.warnings
asset_bundle_export_job_include_folder_members = asset_bundle_export_job.include_folder_members
asset_bundle_export_job_validation_strategy = asset_bundle_export_job.validation_strategy
asset_bundle_export_job_aws_account_id = asset_bundle_export_job.aws_account_id
asset_bundle_export_job_export_format = asset_bundle_export_job.export_format
asset_bundle_export_job_job_status = asset_bundle_export_job.job_status
```

---


### Folder_resolved_permissions

FolderResolvedPermissions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The Amazon Resource Name (ARN) of the folder.</p> |
| `permissions` | Vec<String> | <p>Information about the permissions for the folder.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `next_token` | String | <p>A pagination token for the next set of results, or null if there are no more results.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `folder_id` | String | <p>The ID of the folder.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access folder_resolved_permissions outputs
folder_resolved_permissions_id = folder_resolved_permissions.id
folder_resolved_permissions_arn = folder_resolved_permissions.arn
folder_resolved_permissions_permissions = folder_resolved_permissions.permissions
folder_resolved_permissions_request_id = folder_resolved_permissions.request_id
folder_resolved_permissions_next_token = folder_resolved_permissions.next_token
folder_resolved_permissions_status = folder_resolved_permissions.status
folder_resolved_permissions_folder_id = folder_resolved_permissions.folder_id
```

---


### Dashboard

Dashboard resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_description` | String |  | <p>A description for the first version of the dashboard being created.</p> |
| `theme_arn` | String |  | <p>The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. If
            you add a value for this field, it overrides the value that is used in the source
            entity. The theme ARN must exist in the same Amazon Web Services account where you create
            the dashboard.</p> |
| `folder_arns` | Vec<String> |  | <p>When you create the dashboard, Amazon Quick Sight adds the dashboard to these
            folders.</p> |
| `permissions` | Vec<String> |  | <p>A structure that contains the permissions of the dashboard. You can use this structure
            for granting permissions by providing a list of IAM action information
            for each principal ARN. </p>
         <p>To specify no permissions, omit the permissions list.</p> |
| `tags` | Vec<String> |  | <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the
            dashboard.</p> |
| `validation_strategy` | String |  | <p>The option to relax the validation needed to create a dashboard with definition
            objects. This option skips the validation step for specific errors.</p> |
| `name` | String | ✅ | <p>The display name of the dashboard.</p> |
| `parameters` | String |  | <p>The parameters for the creation of the dashboard, which you want to use to override
            the default settings. A dashboard can have any type of parameters, and some parameters
            might accept multiple values. </p> |
| `link_entities` | Vec<String> |  | <p>A list of analysis Amazon Resource Names (ARNs) to be linked to the dashboard.</p> |
| `dashboard_publish_options` | String |  | <p>Options for publishing the dashboard when you create it:</p>
         <ul>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This
                    status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is
                    set to <code>DISABLED</code>, Amazon Quick Sight disables the left filter pane on
                    the published dashboard, which can be used for ad hoc (one-time) filtering. This
                    option is <code>ENABLED</code> by default. </p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This
                    status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual
                    option to export data to .CSV format isn't enabled when this is set to
                        <code>DISABLED</code>. This option is <code>ENABLED</code> by default.
                </p>
            </li>
            <li>
               <p>
                  <code>VisibilityState</code> for <code>SheetControlsOption</code> - This
                    visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>.
                    This option is <code>COLLAPSED</code> by default. </p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>QuickSuiteActionsOption</code> -
                    This status can be either <code>ENABLED</code> or <code>DISABLED</code>.
                    Features related to Actions in Amazon Quick Suite on dashboards are disabled
                    when this is set to <code>DISABLED</code>. This option is <code>DISABLED</code>
                    by default.</p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>ExecutiveSummaryOption</code> - This
                    status can be either <code>ENABLED</code> or <code>DISABLED</code>. The option
                    to build an executive summary is disabled when this is set to
                        <code>DISABLED</code>. This option is <code>ENABLED</code> by
                    default.</p>
            </li>
            <li>
               <p>
                  <code>AvailabilityStatus</code> for <code>DataStoriesSharingOption</code> -
                    This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The
                    option to share a data story is disabled when this is set to
                        <code>DISABLED</code>. This option is <code>ENABLED</code> by
                    default.</p>
            </li>
         </ul> |
| `source_entity` | String |  | <p>The entity that you are using as a source when you create the dashboard. In
                <code>SourceEntity</code>, you specify the type of object you're using as source.
            You can only create a dashboard from a template, so you use a
                <code>SourceTemplate</code> entity. If you need to create a dashboard from an
            analysis, first convert the analysis to a template by using the <code>
               <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateTemplate.html">CreateTemplate</a>
            </code> API operation. For <code>SourceTemplate</code>,
            specify the Amazon Resource Name (ARN) of the source template. The
                <code>SourceTemplate</code>ARN can contain any Amazon Web Services account and any
                Amazon Quick Sight-supported Amazon Web Services Region. </p>
         <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> to
            list the replacement datasets for the placeholders listed in the original. The schema in
            each dataset must match its placeholder. </p>
         <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in
            order for the request to be valid.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account where you want to create the
            dashboard.</p> |
| `definition` | String |  | <p>The definition of a dashboard.</p>
         <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
         <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in
            order for the request to be valid.</p> |
| `dashboard_id` | String | ✅ | <p>The ID for the dashboard, also added to the IAM policy.</p> |
| `link_sharing_configuration` | String |  | <p>A structure that contains the permissions of a shareable link to the dashboard.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dashboard` | String | <p>Information about the dashboard.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dashboard
dashboard = provider.quicksight.Dashboard {
    name = "value"  # <p>The display name of the dashboard.</p>
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account where you want to create the
            dashboard.</p>
    dashboard_id = "value"  # <p>The ID for the dashboard, also added to the IAM policy.</p>
}

# Access dashboard outputs
dashboard_id = dashboard.id
dashboard_dashboard = dashboard.dashboard
dashboard_request_id = dashboard.request_id
dashboard_status = dashboard.status
```

---


### Role_membership

RoleMembership resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_name` | String | ✅ | <p>The name of the group that you want to add to the role.</p> |
| `namespace` | String | ✅ | <p>The namespace that the role belongs to.</p> |
| `role` | String | ✅ | <p>The role that you want to add a group to.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that you want to create a group in. The Amazon Web Services account ID that you provide must be the same Amazon Web Services account that contains your Amazon Quick Sight account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create role_membership
role_membership = provider.quicksight.Role_membership {
    member_name = "value"  # <p>The name of the group that you want to add to the role.</p>
    namespace = "value"  # <p>The namespace that the role belongs to.</p>
    role = "value"  # <p>The role that you want to add a group to.</p>
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account that you want to create a group in. The Amazon Web Services account ID that you provide must be the same Amazon Web Services account that contains your Amazon Quick Sight account.</p>
}

```

---


### Data_set_permissions

DataSetPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID.</p> |
| `grant_permissions` | Vec<String> |  | <p>The resource permissions that you want to grant to the dataset.</p> |
| `data_set_id` | String | ✅ | <p>The ID for the dataset whose permissions you want to update. This ID is unique per
				Amazon Web Services Region for each Amazon Web Services account.</p> |
| `revoke_permissions` | Vec<String> |  | <p>The resource permissions that you want to revoke from the dataset.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permissions` | Vec<String> | <p>A list of resource permissions on the dataset.</p> |
| `data_set_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset.</p> |
| `data_set_id` | String | <p>The ID for the dataset that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_set_permissions outputs
data_set_permissions_id = data_set_permissions.id
data_set_permissions_permissions = data_set_permissions.permissions
data_set_permissions_data_set_arn = data_set_permissions.data_set_arn
data_set_permissions_data_set_id = data_set_permissions.data_set_id
data_set_permissions_request_id = data_set_permissions.request_id
data_set_permissions_status = data_set_permissions.status
```

---


### Folder_permissions

FolderPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `folder_id` | String | ✅ | <p>The ID of the folder.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that contains the folder to update.</p> |
| `revoke_permissions` | Vec<String> |  | <p>The permissions that you want to revoke from a resource.  Namespace ARNs are not supported <code>Principal</code> values for folder permissions.</p> |
| `grant_permissions` | Vec<String> |  | <p>The permissions that you want to grant on a resource. Namespace ARNs are not supported <code>Principal</code> values for folder permissions.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permissions` | Vec<String> | <p>Information about the permissions on the folder.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `folder_id` | String | <p>The ID of the folder.</p> |
| `next_token` | String | <p>The pagination token for the next set of results, or null if there are no more results.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) for the folder.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access folder_permissions outputs
folder_permissions_id = folder_permissions.id
folder_permissions_permissions = folder_permissions.permissions
folder_permissions_request_id = folder_permissions.request_id
folder_permissions_folder_id = folder_permissions.folder_id
folder_permissions_next_token = folder_permissions.next_token
folder_permissions_arn = folder_permissions.arn
folder_permissions_status = folder_permissions.status
```

---


### Topic_refresh_schedule

TopicRefreshSchedule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the dataset.</p> |
| `topic_id` | String | ✅ | <p>The ID of the topic that you want to modify. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the topic
         you're creating a refresh schedule for.</p> |
| `refresh_schedule` | String | ✅ | <p>The definition of a refresh schedule.</p> |
| `dataset_name` | String |  | <p>The name of the dataset.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `topic_arn` | String | <p>The Amazon Resource Name (ARN) of the topic.</p> |
| `refresh_schedule` | String | <p>The definition of a refresh schedule.</p> |
| `topic_id` | String | <p>The ID of the topic that contains the refresh schedule that you want to describe. This
         ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create topic_refresh_schedule
topic_refresh_schedule = provider.quicksight.Topic_refresh_schedule {
    dataset_arn = "value"  # <p>The Amazon Resource Name (ARN) of the dataset.</p>
    topic_id = "value"  # <p>The ID of the topic that you want to modify. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account that contains the topic
         you're creating a refresh schedule for.</p>
    refresh_schedule = "value"  # <p>The definition of a refresh schedule.</p>
}

# Access topic_refresh_schedule outputs
topic_refresh_schedule_id = topic_refresh_schedule.id
topic_refresh_schedule_status = topic_refresh_schedule.status
topic_refresh_schedule_topic_arn = topic_refresh_schedule.topic_arn
topic_refresh_schedule_refresh_schedule = topic_refresh_schedule.refresh_schedule
topic_refresh_schedule_topic_id = topic_refresh_schedule.topic_id
topic_refresh_schedule_request_id = topic_refresh_schedule.request_id
topic_refresh_schedule_dataset_arn = topic_refresh_schedule.dataset_arn
```

---


### Template_alias

TemplateAlias resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_version_number` | i64 | ✅ | <p>The version number of the template.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the template that you creating an alias for.</p> |
| `template_id` | String | ✅ | <p>An ID for the template.</p> |
| `alias_name` | String | ✅ | <p>The name that you want to give to the template alias that you're creating. Don't start the
			alias name with the <code>$</code> character. Alias names that start with <code>$</code>
			are reserved by Quick Sight. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_alias` | String | <p>Information about the template alias.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create template_alias
template_alias = provider.quicksight.Template_alias {
    template_version_number = "value"  # <p>The version number of the template.</p>
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account that contains the template that you creating an alias for.</p>
    template_id = "value"  # <p>An ID for the template.</p>
    alias_name = "value"  # <p>The name that you want to give to the template alias that you're creating. Don't start the
			alias name with the <code>$</code> character. Alias names that start with <code>$</code>
			are reserved by Quick Sight. </p>
}

# Access template_alias outputs
template_alias_id = template_alias.id
template_alias_template_alias = template_alias.template_alias
template_alias_status = template_alias.status
template_alias_request_id = template_alias.request_id
```

---


### Theme

Theme resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | <p>A valid grouping of resource permissions to apply to the new theme.
			</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account where you want to store the new theme. </p> |
| `base_theme_id` | String | ✅ | <p>The ID of the theme that a custom theme will inherit from. All themes inherit from one of
			the starting themes defined by Amazon Quick Sight. For a list of the starting themes, use
				<code>ListThemes</code> or choose <b>Themes</b> from
			within an analysis. </p> |
| `configuration` | String | ✅ | <p>The theme configuration, which contains the theme display properties.</p> |
| `tags` | Vec<String> |  | <p>A map of the key-value pairs for the resource tag or tags that you want to add to the
			resource.</p> |
| `name` | String | ✅ | <p>A display name for the theme.</p> |
| `version_description` | String |  | <p>A description of the first version of the theme that you're creating. Every time
				<code>UpdateTheme</code> is called, a new version is created. Each version of the
			theme has a description of the version in the <code>VersionDescription</code>
			field.</p> |
| `theme_id` | String | ✅ | <p>An ID for the theme that you want to create. The theme ID is unique per Amazon Web Services Region in
			each Amazon Web Services account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `theme` | String | <p>The information about the theme that you are describing.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create theme
theme = provider.quicksight.Theme {
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account where you want to store the new theme. </p>
    base_theme_id = "value"  # <p>The ID of the theme that a custom theme will inherit from. All themes inherit from one of
			the starting themes defined by Amazon Quick Sight. For a list of the starting themes, use
				<code>ListThemes</code> or choose <b>Themes</b> from
			within an analysis. </p>
    configuration = "value"  # <p>The theme configuration, which contains the theme display properties.</p>
    name = "value"  # <p>A display name for the theme.</p>
    theme_id = "value"  # <p>An ID for the theme that you want to create. The theme ID is unique per Amazon Web Services Region in
			each Amazon Web Services account.</p>
}

# Access theme outputs
theme_id = theme.id
theme_request_id = theme.request_id
theme_theme = theme.theme
theme_status = theme.status
```

---


### Ingestion

Ingestion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ingestion_id` | String | ✅ | <p>An ID for the ingestion.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID.</p> |
| `data_set_id` | String | ✅ | <p>The ID of the dataset used in the ingestion.</p> |
| `ingestion_type` | String |  | <p>The type of ingestion that you want to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ingestion` | String | <p>Information about the ingestion.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ingestion
ingestion = provider.quicksight.Ingestion {
    ingestion_id = "value"  # <p>An ID for the ingestion.</p>
    aws_account_id = "value"  # <p>The Amazon Web Services account ID.</p>
    data_set_id = "value"  # <p>The ID of the dataset used in the ingestion.</p>
}

# Access ingestion outputs
ingestion_id = ingestion.id
ingestion_ingestion = ingestion.ingestion
ingestion_request_id = ingestion.request_id
ingestion_status = ingestion.status
```

---


### Spice_capacity_configuration

SPICECapacityConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the SPICE configuration that you want to update.</p> |
| `purchase_mode` | String | ✅ | <p>Determines how SPICE capacity can be purchased. The following options are available. </p>
         <ul>
            <li>
               <p>
                  <code>MANUAL</code>: SPICE capacity can only be purchased manually.</p>
            </li>
            <li>
               <p>
                  <code>AUTO_PURCHASE</code>: Extra SPICE capacity is automatically purchased on your behalf as needed. SPICE capacity can also be purchased manually with this option.</p>
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

```

---


### Action_connector

ActionConnector resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | <p>The permissions configuration that defines which users, groups, or namespaces can access this action connector and what operations they can perform.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to apply to the action connector for resource management and organization.</p> |
| `name` | String | ✅ | <p>A descriptive name for the action connector.</p> |
| `action_connector_id` | String | ✅ | <p>A unique identifier for the action connector. This ID must be unique within the Amazon Web Services account. The <code>ActionConnectorId</code> must not
	          start with the prefix <code>quicksuite-</code>
         </p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID associated with the action connector.</p> |
| `authentication_config` | String | ✅ | <p>The authentication configuration for connecting to the external service. This includes the authentication type, base URL, and authentication metadata such as client credentials or API keys.</p> |
| `vpc_connection_arn` | String |  | <p>The ARN of the VPC connection to use for secure connectivity to the external service.</p> |
| `description` | String |  | <p>An optional description of the action connector.</p> |
| `type` | String | ✅ | <p>The type of action connector.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `action_connector` | String | <p>The detailed information about the action connector, including its configuration and current state.</p> |
| `status` | i64 | <p>The HTTP status code of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create action_connector
action_connector = provider.quicksight.Action_connector {
    name = "value"  # <p>A descriptive name for the action connector.</p>
    action_connector_id = "value"  # <p>A unique identifier for the action connector. This ID must be unique within the Amazon Web Services account. The <code>ActionConnectorId</code> must not
	          start with the prefix <code>quicksuite-</code>
         </p>
    aws_account_id = "value"  # <p>The Amazon Web Services account ID associated with the action connector.</p>
    authentication_config = "value"  # <p>The authentication configuration for connecting to the external service. This includes the authentication type, base URL, and authentication metadata such as client credentials or API keys.</p>
    type = "value"  # <p>The type of action connector.</p>
}

# Access action_connector outputs
action_connector_id = action_connector.id
action_connector_request_id = action_connector.request_id
action_connector_action_connector = action_connector.action_connector
action_connector_status = action_connector.status
```

---


### Key_registration

KeyRegistration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_registration` | Vec<String> | ✅ | <p>A list of <code>RegisteredCustomerManagedKey</code> objects to be updated to the Quick Sight account.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the customer managed key registration that you want to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aws_account_id` | String | <p>The ID of the Amazon Web Services account that contains the customer managed key registration specified in the request.</p> |
| `key_registration` | Vec<String> | <p>A list of <code>RegisteredCustomerManagedKey</code> objects in a Quick Sight account.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `q_data_key` | String | <p>A list of <code>QDataKey</code> objects in a Quick Sight account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access key_registration outputs
key_registration_id = key_registration.id
key_registration_aws_account_id = key_registration.aws_account_id
key_registration_key_registration = key_registration.key_registration
key_registration_status = key_registration.status
key_registration_request_id = key_registration.request_id
key_registration_q_data_key = key_registration.q_data_key
```

---


### Custom_permissions

CustomPermissions resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that you want to create the custom permissions profile in.</p> |
| `tags` | Vec<String> |  | <p>The tags to associate with the custom permissions profile.</p> |
| `capabilities` | String |  | <p>A set of actions to include in the custom permissions profile.</p> |
| `custom_permissions_name` | String | ✅ | <p>The name of the custom permissions profile that you want to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `custom_permissions` | String | <p>The custom permissions profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_permissions
custom_permissions = provider.quicksight.Custom_permissions {
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account that you want to create the custom permissions profile in.</p>
    custom_permissions_name = "value"  # <p>The name of the custom permissions profile that you want to create.</p>
}

# Access custom_permissions outputs
custom_permissions_id = custom_permissions.id
custom_permissions_request_id = custom_permissions.request_id
custom_permissions_status = custom_permissions.status
custom_permissions_custom_permissions = custom_permissions.custom_permissions
```

---


### Identity_propagation_config

IdentityPropagationConfig resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorized_targets` | Vec<String> |  | <p>Specifies a list of application ARNs that represent the authorized targets for a service.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the identity propagation configuration that you want to update.</p> |
| `service` | String | ✅ | <p>The name of the Amazon Web Services service that contains the authorized targets that you want to add or update.</p> |



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


### Q_personalization_configuration

QPersonalizationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account account that contains the personalization configuration that the user wants to update.</p> |
| `personalization_mode` | String | ✅ | <p>An option to allow Amazon Quick Sight to customize data stories with user specific metadata, specifically location and job information, in your IAM Identity Center instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `personalization_mode` | String | <p>A value that indicates whether personalization is enabled or not.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access q_personalization_configuration outputs
q_personalization_configuration_id = q_personalization_configuration.id
q_personalization_configuration_request_id = q_personalization_configuration.request_id
q_personalization_configuration_status = q_personalization_configuration.status
q_personalization_configuration_personalization_mode = q_personalization_configuration.personalization_mode
```

---


### Public_sharing_settings

PublicSharingSettings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `public_sharing_enabled` | bool |  | <p>A Boolean value that indicates whether public sharing is turned on for an QuickSight account.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID associated with your Amazon Quick Sight
            subscription.</p> |



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


### Refresh_schedule

RefreshSchedule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_set_id` | String | ✅ | <p>The ID of the dataset.</p> |
| `schedule` | String | ✅ | <p>The refresh schedule.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) for the refresh schedule.</p> |
| `refresh_schedule` | String | <p>The refresh schedule.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create refresh_schedule
refresh_schedule = provider.quicksight.Refresh_schedule {
    data_set_id = "value"  # <p>The ID of the dataset.</p>
    schedule = "value"  # <p>The refresh schedule.</p>
    aws_account_id = "value"  # <p>The Amazon Web Services account ID.</p>
}

# Access refresh_schedule outputs
refresh_schedule_id = refresh_schedule.id
refresh_schedule_status = refresh_schedule.status
refresh_schedule_arn = refresh_schedule.arn
refresh_schedule_refresh_schedule = refresh_schedule.refresh_schedule
refresh_schedule_request_id = refresh_schedule.request_id
```

---


### Default_q_business_application

DefaultQBusinessApplication resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The ID of the Amazon Q Business application that you want to update.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Quick Sight account that is connected to the Amazon Q Business application that you want to update.</p> |
| `namespace` | String |  | <p>The Quick Sight namespace that contains the linked Amazon Q Business application. If this field is left blank, the default namespace is used. Currently, the default namespace is the only valid value for this parameter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_id` | String | <p>The ID of the Amazon Q Business application that is linked to the Quick Sight account.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_q_business_application outputs
default_q_business_application_id = default_q_business_application.id
default_q_business_application_application_id = default_q_business_application.application_id
default_q_business_application_request_id = default_q_business_application.request_id
default_q_business_application_status = default_q_business_application.status
```

---


### Dashboard_snapshot_job

DashboardSnapshotJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_status` | String | <p>Indicates the status of a job. The status updates as the job executes. This shows one of the following values.</p>
         <ul>
            <li>
               <p>
                  <code>COMPLETED</code> - The job was completed successfully.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The job failed to execute.</p>
            </li>
            <li>
               <p>
                  <code>QUEUED</code> - The job is queued and hasn't started yet.</p>
            </li>
            <li>
               <p>
                  <code>RUNNING</code> - The job is still running.</p>
            </li>
         </ul> |
| `created_time` | String | <p>
            The time that the snapshot job was created.
        </p> |
| `last_updated_time` | String | <p>
            The time that the snapshot job status was last updated.
        </p> |
| `aws_account_id` | String | <p>
            The ID of the Amazon Web Services account that the dashboard snapshot job is executed in.
        </p> |
| `request_id` | String | <p>
            The Amazon Web Services request ID for this operation.
        </p> |
| `snapshot_job_id` | String | <p>The ID of the job to be described. The job ID is set when you start a new job with a <code>StartDashboardSnapshotJob</code> API call.</p> |
| `snapshot_configuration` | String | <p>The snapshot configuration of the job. This information is provided when you make a <code>StartDashboardSnapshotJob</code> API call.</p> |
| `dashboard_id` | String | <p>The ID of the dashboard that you have started a snapshot job for.</p> |
| `status` | i64 | <p>The HTTP status of the request</p> |
| `user_configuration` | String | <p>The user configuration for the snapshot job. This information is provided when you make a <code>StartDashboardSnapshotJob</code> API call.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) for the snapshot job. The job ARN is generated when you start a new job with a <code>StartDashboardSnapshotJob</code> API call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dashboard_snapshot_job outputs
dashboard_snapshot_job_id = dashboard_snapshot_job.id
dashboard_snapshot_job_job_status = dashboard_snapshot_job.job_status
dashboard_snapshot_job_created_time = dashboard_snapshot_job.created_time
dashboard_snapshot_job_last_updated_time = dashboard_snapshot_job.last_updated_time
dashboard_snapshot_job_aws_account_id = dashboard_snapshot_job.aws_account_id
dashboard_snapshot_job_request_id = dashboard_snapshot_job.request_id
dashboard_snapshot_job_snapshot_job_id = dashboard_snapshot_job.snapshot_job_id
dashboard_snapshot_job_snapshot_configuration = dashboard_snapshot_job.snapshot_configuration
dashboard_snapshot_job_dashboard_id = dashboard_snapshot_job.dashboard_id
dashboard_snapshot_job_status = dashboard_snapshot_job.status
dashboard_snapshot_job_user_configuration = dashboard_snapshot_job.user_configuration
dashboard_snapshot_job_arn = dashboard_snapshot_job.arn
```

---


### Folder_membership

FolderMembership resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `folder_id` | String | ✅ | <p>The ID of the folder.</p> |
| `member_id` | String | ✅ | <p>The ID of the asset that you want to add to the folder.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that contains the folder.</p> |
| `member_type` | String | ✅ | <p>The member type of the asset that you want to add to a folder.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create folder_membership
folder_membership = provider.quicksight.Folder_membership {
    folder_id = "value"  # <p>The ID of the folder.</p>
    member_id = "value"  # <p>The ID of the asset that you want to add to the folder.</p>
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account that contains the folder.</p>
    member_type = "value"  # <p>The member type of the asset that you want to add to a folder.</p>
}

```

---


### Template

Template resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_id` | String | ✅ | <p>An ID for the template that you want to create. This template is unique per Amazon Web Services Region; in
			each Amazon Web Services account.</p> |
| `version_description` | String |  | <p>A description of the current template version being created. This API operation creates the
			first version of the template. Every time <code>UpdateTemplate</code> is called, a new
			version is created. Each version of the template maintains a description of the version
			in the <code>VersionDescription</code> field.</p> |
| `validation_strategy` | String |  | <p>TThe option to relax the validation needed to create a template with definition objects. This skips the validation step for specific errors.</p> |
| `tags` | Vec<String> |  | <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the resource.</p> |
| `source_entity` | String |  | <p>The entity that you are using as a source when you create the template. In
			<code>SourceEntity</code>, you specify the type of object you're using as source:
			<code>SourceTemplate</code> for a template or <code>SourceAnalysis</code> for an
			analysis. Both of these require an Amazon Resource Name (ARN). For
			<code>SourceTemplate</code>, specify the ARN of the source template. For
			<code>SourceAnalysis</code>, specify the ARN of the source analysis. The <code>SourceTemplate</code>
			ARN can contain any Amazon Web Services account and any Quick Sight-supported Amazon Web Services Region. </p>
         <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> or
			<code>SourceAnalysis</code> to list the replacement datasets for the placeholders listed
			in the original. The schema in each dataset must match its placeholder. </p>
         <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in 
			order for the request to be valid.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that the group is in. You use the ID for the Amazon Web Services account that contains your Amazon Quick Sight account.</p> |
| `permissions` | Vec<String> |  | <p>A list of resource permissions to be set on the template. </p> |
| `name` | String |  | <p>A display name for the template.</p> |
| `definition` | String |  | <p>The definition of a template.</p>
         <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
         <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in 
			order for the request to be valid.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `template` | String | <p>The template structure for the object you want to describe.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create template
template = provider.quicksight.Template {
    template_id = "value"  # <p>An ID for the template that you want to create. This template is unique per Amazon Web Services Region; in
			each Amazon Web Services account.</p>
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account that the group is in. You use the ID for the Amazon Web Services account that contains your Amazon Quick Sight account.</p>
}

# Access template outputs
template_id = template.id
template_status = template.status
template_template = template.template
template_request_id = template.request_id
```

---


### Asset_bundle_import_job

AssetBundleImportJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The Amazon Resource Name (ARN) for the import job.</p> |
| `asset_bundle_import_source` | String | <p>The source of the asset bundle zip file that contains the data that is imported by the
         job.</p> |
| `aws_account_id` | String | <p>The ID of the Amazon Web Services account the import job was executed in. </p> |
| `failure_action` | String | <p>The failure action for the import job.</p> |
| `created_time` | String | <p>The time that the import job was created.</p> |
| `warnings` | Vec<String> | <p>An array of warning records that describe all permitted errors that are encountered
         during the import job.</p> |
| `asset_bundle_import_job_id` | String | <p>The ID of the job. The job ID is set when you start a new job with a
            <code>StartAssetBundleImportJob</code> API call.</p> |
| `errors` | Vec<String> | <p>An array of error records that describes any failures that occurred during the export
         job processing.</p>
         <p>Error records accumulate while the job is still running. The complete set of error
         records is available after the job has completed and failed.</p> |
| `job_status` | String | <p>Indicates the status of a job through its queuing and execution.</p>
         <p>Poll the <code>DescribeAssetBundleImport</code> API until <code>JobStatus</code> returns
         one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>SUCCESSFUL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>FAILED_ROLLBACK_COMPLETED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>FAILED_ROLLBACK_ERROR</code>
               </p>
            </li>
         </ul> |
| `override_parameters` | String | <p>Optional overrides that are applied to the resource configuration before import.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the response.</p> |
| `override_tags` | String | <p>Optional tag overrides that are applied to the resource configuration before
         import.</p> |
| `override_validation_strategy` | String | <p>An optional validation strategy override for all analyses and dashboards to be applied
         to the resource configuration before import.</p> |
| `rollback_errors` | Vec<String> | <p>An array of error records that describes any failures that occurred while an import job
         was attempting a rollback.</p>
         <p>Error records accumulate while the job is still running. The complete set of error
         records is available after the job has completed and failed.</p> |
| `override_permissions` | String | <p>Optional permission overrides that are applied to the resource configuration before
         import.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_bundle_import_job outputs
asset_bundle_import_job_id = asset_bundle_import_job.id
asset_bundle_import_job_arn = asset_bundle_import_job.arn
asset_bundle_import_job_asset_bundle_import_source = asset_bundle_import_job.asset_bundle_import_source
asset_bundle_import_job_aws_account_id = asset_bundle_import_job.aws_account_id
asset_bundle_import_job_failure_action = asset_bundle_import_job.failure_action
asset_bundle_import_job_created_time = asset_bundle_import_job.created_time
asset_bundle_import_job_warnings = asset_bundle_import_job.warnings
asset_bundle_import_job_asset_bundle_import_job_id = asset_bundle_import_job.asset_bundle_import_job_id
asset_bundle_import_job_errors = asset_bundle_import_job.errors
asset_bundle_import_job_job_status = asset_bundle_import_job.job_status
asset_bundle_import_job_override_parameters = asset_bundle_import_job.override_parameters
asset_bundle_import_job_request_id = asset_bundle_import_job.request_id
asset_bundle_import_job_status = asset_bundle_import_job.status
asset_bundle_import_job_override_tags = asset_bundle_import_job.override_tags
asset_bundle_import_job_override_validation_strategy = asset_bundle_import_job.override_validation_strategy
asset_bundle_import_job_rollback_errors = asset_bundle_import_job.rollback_errors
asset_bundle_import_job_override_permissions = asset_bundle_import_job.override_permissions
```

---


### Brand

Brand resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `brand_id` | String | ✅ | <p>The ID of the QuickSight brand.</p> |
| `brand_definition` | String |  | <p>The definition of the brand.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that owns the brand.</p> |
| `tags` | Vec<String> |  | <p>A map of the key-value pairs that are assigned to the brand.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `brand_definition` | String | <p>The definition of the brand.</p> |
| `brand_detail` | String | <p>The details of the brand.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create brand
brand = provider.quicksight.Brand {
    brand_id = "value"  # <p>The ID of the QuickSight brand.</p>
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account that owns the brand.</p>
}

# Access brand outputs
brand_id = brand.id
brand_brand_definition = brand.brand_definition
brand_brand_detail = brand.brand_detail
brand_request_id = brand.request_id
```

---


### User_by_principal_id

UserByPrincipalId resource

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


### Vpc_connection

VPCConnection resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_group_ids` | Vec<String> | ✅ | <p>A list of security group IDs for the VPC connection.</p> |
| `role_arn` | String | ✅ | <p>The IAM role to associate with the VPC connection.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID of the account where you want to create a new VPC
			connection.</p> |
| `name` | String | ✅ | <p>The display name for the VPC connection.</p> |
| `vpc_connection_id` | String | ✅ | <p>The ID of the VPC connection that
			you're creating. This ID is a unique identifier for each Amazon Web Services Region in an
				Amazon Web Services account.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>A list of subnet IDs for the VPC connection.</p> |
| `tags` | Vec<String> |  | <p>A map of the key-value pairs for the resource tag or tags assigned to the VPC
			connection.</p> |
| `dns_resolvers` | Vec<String> |  | <p>A list of IP addresses of DNS resolver endpoints for the VPC connection.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_connection` | String | <p>A response object that provides information for the specified VPC connection.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_connection
vpc_connection = provider.quicksight.Vpc_connection {
    security_group_ids = "value"  # <p>A list of security group IDs for the VPC connection.</p>
    role_arn = "value"  # <p>The IAM role to associate with the VPC connection.</p>
    aws_account_id = "value"  # <p>The Amazon Web Services account ID of the account where you want to create a new VPC
			connection.</p>
    name = "value"  # <p>The display name for the VPC connection.</p>
    vpc_connection_id = "value"  # <p>The ID of the VPC connection that
			you're creating. This ID is a unique identifier for each Amazon Web Services Region in an
				Amazon Web Services account.</p>
    subnet_ids = "value"  # <p>A list of subnet IDs for the VPC connection.</p>
}

# Access vpc_connection outputs
vpc_connection_id = vpc_connection.id
vpc_connection_vpc_connection = vpc_connection.vpc_connection
vpc_connection_status = vpc_connection.status
vpc_connection_request_id = vpc_connection.request_id
```

---


### Data_source

DataSource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `credentials` | String |  | <p>The credentials Amazon Quick Sight that uses to connect to your underlying source.
			Currently, only credentials based on user name and password are supported.</p> |
| `permissions` | Vec<String> |  | <p>A list of resource permissions on the data source.</p> |
| `ssl_properties` | String |  | <p>Secure Socket Layer (SSL) properties that apply when Amazon Quick Sight connects to
			your underlying source.</p> |
| `tags` | Vec<String> |  | <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the
			data source.</p> |
| `vpc_connection_properties` | String |  | <p>Use this parameter only when you want Amazon Quick Sight to use a VPC connection when
			connecting to your underlying source.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID.</p> |
| `folder_arns` | Vec<String> |  | <p>When you create the data source, Amazon Quick Sight adds the data source to these
			folders.</p> |
| `type` | String | ✅ | <p>The type of the data source. To return a list of all data sources, use
				<code>ListDataSources</code>.</p>
         <p>Use <code>AMAZON_ELASTICSEARCH</code> for Amazon OpenSearch Service.</p> |
| `data_source_parameters` | String |  | <p>The parameters that Amazon Quick Sight uses to connect to your underlying
			source.</p> |
| `name` | String | ✅ | <p>A display name for the data source.</p> |
| `data_source_id` | String | ✅ | <p>An ID for the data source. This ID is unique per Amazon Web Services Region for each
				Amazon Web Services account. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_source` | String | <p>The information on the data source.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_source
data_source = provider.quicksight.Data_source {
    aws_account_id = "value"  # <p>The Amazon Web Services account ID.</p>
    type = "value"  # <p>The type of the data source. To return a list of all data sources, use
				<code>ListDataSources</code>.</p>
         <p>Use <code>AMAZON_ELASTICSEARCH</code> for Amazon OpenSearch Service.</p>
    name = "value"  # <p>A display name for the data source.</p>
    data_source_id = "value"  # <p>An ID for the data source. This ID is unique per Amazon Web Services Region for each
				Amazon Web Services account. </p>
}

# Access data_source outputs
data_source_id = data_source.id
data_source_data_source = data_source.data_source
data_source_status = data_source.status
data_source_request_id = data_source.request_id
```

---


### Theme_alias

ThemeAlias resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the theme for the new theme alias.</p> |
| `alias_name` | String | ✅ | <p>The name that you want to give to the theme alias that you are creating. The
			alias name can't begin with a <code>$</code>. Alias names that start with <code>$</code>
			are reserved by Amazon Quick Sight. </p> |
| `theme_version_number` | i64 | ✅ | <p>The version number of the theme.</p> |
| `theme_id` | String | ✅ | <p>An ID for the theme alias.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `theme_alias` | String | <p>Information about the theme alias.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create theme_alias
theme_alias = provider.quicksight.Theme_alias {
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account that contains the theme for the new theme alias.</p>
    alias_name = "value"  # <p>The name that you want to give to the theme alias that you are creating. The
			alias name can't begin with a <code>$</code>. Alias names that start with <code>$</code>
			are reserved by Amazon Quick Sight. </p>
    theme_version_number = "value"  # <p>The version number of the theme.</p>
    theme_id = "value"  # <p>An ID for the theme alias.</p>
}

# Access theme_alias outputs
theme_alias_id = theme_alias.id
theme_alias_status = theme_alias.status
theme_alias_theme_alias = theme_alias.theme_alias
theme_alias_request_id = theme_alias.request_id
```

---


### Dashboards_qa_configuration

DashboardsQAConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the dashboard QA configuration that you want to update.</p> |
| `dashboards_qa_status` | String | ✅ | <p>The status of dashboards QA configuration that you want to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `dashboards_qa_status` | String | <p>The status of dashboards QA configuration that you want described.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dashboards_qa_configuration outputs
dashboards_qa_configuration_id = dashboards_qa_configuration.id
dashboards_qa_configuration_request_id = dashboards_qa_configuration.request_id
dashboards_qa_configuration_dashboards_qa_status = dashboards_qa_configuration.dashboards_qa_status
dashboards_qa_configuration_status = dashboards_qa_configuration.status
```

---


### Data_set_refresh_properties

DataSetRefreshProperties resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_set_refresh_properties` | String | ✅ | <p>The dataset refresh properties.</p> |
| `data_set_id` | String | ✅ | <p>The ID of the dataset.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_set_refresh_properties` | String | <p>The dataset refresh properties.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_set_refresh_properties
data_set_refresh_properties = provider.quicksight.Data_set_refresh_properties {
    data_set_refresh_properties = "value"  # <p>The dataset refresh properties.</p>
    data_set_id = "value"  # <p>The ID of the dataset.</p>
    aws_account_id = "value"  # <p>The Amazon Web Services account ID.</p>
}

# Access data_set_refresh_properties outputs
data_set_refresh_properties_id = data_set_refresh_properties.id
data_set_refresh_properties_data_set_refresh_properties = data_set_refresh_properties.data_set_refresh_properties
data_set_refresh_properties_request_id = data_set_refresh_properties.request_id
data_set_refresh_properties_status = data_set_refresh_properties.status
```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_name` | String | ✅ | <p>A name for the group that you want to create.</p> |
| `description` | String |  | <p>A description for the group that you want to create.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the
			Amazon Web Services account that contains your Amazon Quick Sight account.</p> |
| `namespace` | String | ✅ | <p>The namespace that you want the group to be a part of.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group` | String | <p>The name of the group.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.quicksight.Group {
    group_name = "value"  # <p>A name for the group that you want to create.</p>
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the
			Amazon Web Services account that contains your Amazon Quick Sight account.</p>
    namespace = "value"  # <p>The namespace that you want the group to be a part of.</p>
}

# Access group outputs
group_id = group.id
group_group = group.group
group_request_id = group.request_id
group_status = group.status
```

---


### Account_custom_permission

AccountCustomPermission resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account for which you want to apply a custom permissions profile.</p> |
| `custom_permissions_name` | String | ✅ | <p>The name of the custom permissions profile that you want to apply to an account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `custom_permissions_name` | String | <p>The name of the custom permissions profile.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_custom_permission outputs
account_custom_permission_id = account_custom_permission.id
account_custom_permission_request_id = account_custom_permission.request_id
account_custom_permission_custom_permissions_name = account_custom_permission.custom_permissions_name
account_custom_permission_status = account_custom_permission.status
```

---


### Role_custom_permission

RoleCustomPermission resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_permissions_name` | String | ✅ | <p>The name of the custom permission that you want to update the role with.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that you want to create a group in. The Amazon Web Services account ID that you provide must be the same Amazon Web Services account that contains your Amazon Quick Sight account.</p> |
| `role` | String | ✅ | <p>The name of role tht you want to update.</p> |
| `namespace` | String | ✅ | <p>The namespace that contains the role that you want to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `custom_permissions_name` | String | <p>The name of the custom permission that is described.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access role_custom_permission outputs
role_custom_permission_id = role_custom_permission.id
role_custom_permission_status = role_custom_permission.status
role_custom_permission_custom_permissions_name = role_custom_permission.custom_permissions_name
role_custom_permission_request_id = role_custom_permission.request_id
```

---


### Brand_published_version

BrandPublishedVersion resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_id` | String | ✅ | <p>The ID of the published version.</p> |
| `brand_id` | String | ✅ | <p>The ID of the QuickSight brand.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that owns the brand.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `brand_definition` | String | <p>The definition of the brand.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `brand_detail` | String | <p>The details of the brand.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access brand_published_version outputs
brand_published_version_id = brand_published_version.id
brand_published_version_brand_definition = brand_published_version.brand_definition
brand_published_version_request_id = brand_published_version.request_id
brand_published_version_brand_detail = brand_published_version.brand_detail
```

---


### Template_permissions

TemplatePermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the template.</p> |
| `template_id` | String | ✅ | <p>The ID for the template.</p> |
| `grant_permissions` | Vec<String> |  | <p>A list of resource permissions to be granted on the template. </p> |
| `revoke_permissions` | Vec<String> |  | <p>A list of resource permissions to be revoked from the template. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `template_id` | String | <p>The ID for the template.</p> |
| `template_arn` | String | <p>The Amazon Resource Name (ARN) of the template.</p> |
| `permissions` | Vec<String> | <p>A list of resource permissions to be set on the template. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access template_permissions outputs
template_permissions_id = template_permissions.id
template_permissions_request_id = template_permissions.request_id
template_permissions_status = template_permissions.status
template_permissions_template_id = template_permissions.template_id
template_permissions_template_arn = template_permissions.template_arn
template_permissions_permissions = template_permissions.permissions
```

---


### Dashboard_embed_url

DashboardEmbedUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `embed_url` | String | <p>A single-use URL that you can put into your server-side webpage to embed your
            dashboard. This URL is valid for 5 minutes. The API operation provides the URL with an
                <code>auth_code</code> value that enables one (and only one) sign-on to a user
            session that is valid for 10 hours. </p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dashboard_embed_url outputs
dashboard_embed_url_id = dashboard_embed_url.id
dashboard_embed_url_embed_url = dashboard_embed_url.embed_url
dashboard_embed_url_request_id = dashboard_embed_url.request_id
dashboard_embed_url_status = dashboard_embed_url.status
```

---


### Topic_refresh

TopicRefresh resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `refresh_details` | String | <p>Details of the refresh, which is performed when the topic is created or updated.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access topic_refresh outputs
topic_refresh_id = topic_refresh.id
topic_refresh_refresh_details = topic_refresh.refresh_details
topic_refresh_request_id = topic_refresh.request_id
topic_refresh_status = topic_refresh.status
```

---


### Account_settings

AccountSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_namespace` | String | ✅ | <p>The default namespace for this Amazon Web Services account. Currently, the default is
                <code>default</code>. IAM users that
            register for the first time with Amazon Quick Sight provide an email address that becomes
            associated with the default namespace.
        </p> |
| `notification_email` | String |  | <p>The email address that you want Quick Sight to send notifications to regarding your
            Amazon Web Services account or Quick Sight subscription.</p> |
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that contains the Quick Sight settings that you want to
            list.</p> |
| `termination_protection_enabled` | bool |  | <p>A boolean value that determines whether or not an Quick Sight account can be deleted. A <code>True</code> value doesn't allow the account to be deleted and results in an error message if a user tries to make a <code>DeleteAccountSubscription</code> request. A <code>False</code> value will allow the account to be deleted.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_settings` | String | <p>The Amazon Quick Sight settings for this Amazon Web Services account. This information
            includes the edition of Amazon Quick Sight that you subscribed to (Standard or
            Enterprise) and the notification email for the Amazon Quick Sight subscription. </p>
         <p>In the Quick Sight console, the Amazon Quick Sight subscription is sometimes referred to
            as a Quick Sight "account" even though it's technically not an account by
            itself. Instead, it's a subscription to the Amazon Quick Sight service for your
                Amazon Web Services account. The edition that you subscribe to applies to QuickSight in every Amazon Web Services Region where you use it.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_settings outputs
account_settings_id = account_settings.id
account_settings_account_settings = account_settings.account_settings
account_settings_status = account_settings.status
account_settings_request_id = account_settings.request_id
```

---


### Data_source_permissions

DataSourcePermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grant_permissions` | Vec<String> |  | <p>A list of resource permissions that you want to grant on the data source.</p> |
| `revoke_permissions` | Vec<String> |  | <p>A list of resource permissions that you want to revoke on the data source.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID.</p> |
| `data_source_id` | String | ✅ | <p>The ID of the data source. This ID is unique per Amazon Web Services Region for each
				Amazon Web Services account. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `data_source_arn` | String | <p>The Amazon Resource Name (ARN) of the data source.</p> |
| `permissions` | Vec<String> | <p>A list of resource permissions on the data source.</p> |
| `data_source_id` | String | <p>The ID of the data source. This ID is unique per Amazon Web Services Region for each
				Amazon Web Services account.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_source_permissions outputs
data_source_permissions_id = data_source_permissions.id
data_source_permissions_status = data_source_permissions.status
data_source_permissions_data_source_arn = data_source_permissions.data_source_arn
data_source_permissions_permissions = data_source_permissions.permissions
data_source_permissions_data_source_id = data_source_permissions.data_source_id
data_source_permissions_request_id = data_source_permissions.request_id
```

---


### Theme_permissions

ThemePermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `theme_id` | String | ✅ | <p>The ID for the theme.</p> |
| `grant_permissions` | Vec<String> |  | <p>A list of resource permissions to be granted for the theme.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the theme.</p> |
| `revoke_permissions` | Vec<String> |  | <p>A list of resource permissions to be revoked from the theme.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `theme_id` | String | <p>The ID for the theme.</p> |
| `permissions` | Vec<String> | <p>A list of resource permissions set on the theme. </p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `theme_arn` | String | <p>The Amazon Resource Name (ARN) of the theme.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access theme_permissions outputs
theme_permissions_id = theme_permissions.id
theme_permissions_status = theme_permissions.status
theme_permissions_theme_id = theme_permissions.theme_id
theme_permissions_permissions = theme_permissions.permissions
theme_permissions_request_id = theme_permissions.request_id
theme_permissions_theme_arn = theme_permissions.theme_arn
```

---


### Analysis_definition

AnalysisDefinition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `theme_arn` | String | <p>The ARN of the theme of the analysis.</p> |
| `definition` | String | <p>The definition of an analysis.</p>
         <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `analysis_id` | String | <p>The ID of the analysis described.</p> |
| `errors` | Vec<String> | <p>Errors associated with the analysis.</p> |
| `resource_status` | String | <p>Status associated with the analysis.</p>
         <ul>
            <li>
               <p>
                  <code>CREATION_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATION_SUCCESSFUL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATION_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_SUCCESSFUL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETED</code>
               </p>
            </li>
         </ul> |
| `name` | String | <p>The descriptive name of the analysis.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access analysis_definition outputs
analysis_definition_id = analysis_definition.id
analysis_definition_theme_arn = analysis_definition.theme_arn
analysis_definition_definition = analysis_definition.definition
analysis_definition_status = analysis_definition.status
analysis_definition_request_id = analysis_definition.request_id
analysis_definition_analysis_id = analysis_definition.analysis_id
analysis_definition_errors = analysis_definition.errors
analysis_definition_resource_status = analysis_definition.resource_status
analysis_definition_name = analysis_definition.name
```

---


### Data_set

DataSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `row_level_permission_data_set` | String |  | <p>The row-level security configuration for the data that you want to create.</p> |
| `name` | String | ✅ | <p>The display name for the dataset.</p> |
| `dataset_parameters` | Vec<String> |  | <p>The parameter declarations of the dataset.</p> |
| `aws_account_id` | String | ✅ | <p>The Amazon Web Services account ID.</p> |
| `physical_table_map` | HashMap<String, String> | ✅ | <p>Declares the physical tables that are available in the underlying data sources.</p> |
| `use_as` | String |  | <p>The usage of the dataset. <code>RLS_RULES</code> must be specified for RLS permission
			datasets.</p> |
| `field_folders` | HashMap<String, String> |  | <p>The folder that contains fields and nested subfolders for your dataset.</p> |
| `data_set_usage_configuration` | String |  |  |
| `data_set_id` | String | ✅ | <p>An ID for the dataset that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `logical_table_map` | HashMap<String, String> |  | <p>Configures the combination and transformation of the data from the physical
			tables.</p> |
| `column_groups` | Vec<String> |  | <p>Groupings of columns that work together in certain Amazon Quick Sight features.
			Currently, only geospatial hierarchy is supported.</p> |
| `tags` | Vec<String> |  | <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the
			dataset.</p> |
| `column_level_permission_rules` | Vec<String> |  | <p>A set of one or more definitions of a <code>
               <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ColumnLevelPermissionRule.html">ColumnLevelPermissionRule</a>
            </code>.</p> |
| `folder_arns` | Vec<String> |  | <p>When you create the dataset, Amazon Quick Sight adds the dataset to these
			folders.</p> |
| `import_mode` | String | ✅ | <p>Indicates whether you want to import the data into SPICE.</p> |
| `permissions` | Vec<String> |  | <p>A list of resource permissions on the dataset.</p> |
| `row_level_permission_tag_configuration` | String |  | <p>The configuration of tags on a dataset to set row-level security. Row-level security
			tags are currently supported for anonymous embedding only.</p> |
| `performance_configuration` | String |  | <p>The configuration for the performance optimization of the dataset that contains a
				<code>UniqueKey</code> configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_set` | String | <p>Information on the dataset.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_set
data_set = provider.quicksight.Data_set {
    name = "value"  # <p>The display name for the dataset.</p>
    aws_account_id = "value"  # <p>The Amazon Web Services account ID.</p>
    physical_table_map = "value"  # <p>Declares the physical tables that are available in the underlying data sources.</p>
    data_set_id = "value"  # <p>An ID for the dataset that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    import_mode = "value"  # <p>Indicates whether you want to import the data into SPICE.</p>
}

# Access data_set outputs
data_set_id = data_set.id
data_set_data_set = data_set.data_set
data_set_status = data_set.status
data_set_request_id = data_set.request_id
```

---


### Topic

Topic resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `topic_id` | String | ✅ | <p>The ID for the topic that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `topic` | String | ✅ | <p>The definition of a topic to create.</p> |
| `tags` | Vec<String> |  | <p>Contains a map of the key-value pairs for the resource tag or tags that are assigned to
         the dataset.</p> |
| `custom_instructions` | String |  | <p>Custom instructions for the topic.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that you want to create a topic in.</p> |
| `folder_arns` | Vec<String> |  | <p>The Folder ARN of the folder that you want the topic to reside in.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topic_id` | String | <p>The ID of the topic that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the topic.</p> |
| `topic` | String | <p>The definition of a topic.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `custom_instructions` | String | <p>Custom instructions for the topic.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create topic
topic = provider.quicksight.Topic {
    topic_id = "value"  # <p>The ID for the topic that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    topic = "value"  # <p>The definition of a topic to create.</p>
    aws_account_id = "value"  # <p>The ID of the Amazon Web Services account that you want to create a topic in.</p>
}

# Access topic outputs
topic_id = topic.id
topic_topic_id = topic.topic_id
topic_status = topic.status
topic_arn = topic.arn
topic_topic = topic.topic
topic_request_id = topic.request_id
topic_custom_instructions = topic.custom_instructions
```

---


### Namespace

Namespace resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String | ✅ | <p>The ID for the Amazon Web Services account that you want to create the Quick Sight namespace in.</p> |
| `tags` | Vec<String> |  | <p>The tags that you want to associate with the namespace that you're creating.</p> |
| `namespace` | String | ✅ | <p>The name that you want to use to describe the new namespace.</p> |
| `identity_store` | String | ✅ | <p>Specifies the type of your user identity directory. Currently, this supports users
            with an identity type of <code>QUICKSIGHT</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `namespace` | String | <p>The information about the namespace that you're describing. The response includes 
        the namespace ARN, name, Amazon Web Services Region, creation status, and identity store. <code>DescribeNamespace</code> also
        works for namespaces that are in the process of being created. For incomplete namespaces,
        this API operation lists the namespace error types and messages associated with the creation process.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create namespace
namespace = provider.quicksight.Namespace {
    aws_account_id = "value"  # <p>The ID for the Amazon Web Services account that you want to create the Quick Sight namespace in.</p>
    namespace = "value"  # <p>The name that you want to use to describe the new namespace.</p>
    identity_store = "value"  # <p>Specifies the type of your user identity directory. Currently, this supports users
            with an identity type of <code>QUICKSIGHT</code>.</p>
}

# Access namespace outputs
namespace_id = namespace.id
namespace_namespace = namespace.namespace
namespace_status = namespace.status
namespace_request_id = namespace.request_id
```

---


### Flow_metadata

FlowMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publish_state` | String | <p>The publish state for the flow. Valid values are <code>DRAFT</code>, <code>PUBLISHED</code>, 
            or <code>PENDING_APPROVAL</code>.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the flow.</p> |
| `run_count` | i64 | <p>The number of runs done for the flow.</p> |
| `last_updated_time` | String | <p>The last time this flow was modified.</p> |
| `request_id` | String | <p>The Amazon Web Services request ID for this operation.</p> |
| `name` | String | <p>A display name for the flow.</p> |
| `status` | i64 | <p>The HTTP status of the request.</p> |
| `created_time` | String | <p>The time this flow was created.</p> |
| `flow_id` | String | <p>The unique identifier of the flow.</p> |
| `user_count` | i64 | <p>The number of users who have used the flow.</p> |
| `description` | String | <p>The description for the flow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flow_metadata outputs
flow_metadata_id = flow_metadata.id
flow_metadata_publish_state = flow_metadata.publish_state
flow_metadata_arn = flow_metadata.arn
flow_metadata_run_count = flow_metadata.run_count
flow_metadata_last_updated_time = flow_metadata.last_updated_time
flow_metadata_request_id = flow_metadata.request_id
flow_metadata_name = flow_metadata.name
flow_metadata_status = flow_metadata.status
flow_metadata_created_time = flow_metadata.created_time
flow_metadata_flow_id = flow_metadata.flow_id
flow_metadata_user_count = flow_metadata.user_count
flow_metadata_description = flow_metadata.description
```

---


### User_custom_permission

UserCustomPermission resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `namespace` | String | ✅ | <p>The namespace that the user belongs to.</p> |
| `custom_permissions_name` | String | ✅ | <p>The name of the custom permissions that you want to update.</p> |
| `aws_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that contains the custom permission configuration that you want to update.</p> |
| `user_name` | String | ✅ | <p>The username of the user that you want to update custom permissions for.</p> |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple account_subscription resources
account_subscription_0 = provider.quicksight.Account_subscription {
    authentication_method = "value-0"
    account_name = "value-0"
    aws_account_id = "value-0"
    notification_email = "value-0"
}
account_subscription_1 = provider.quicksight.Account_subscription {
    authentication_method = "value-1"
    account_name = "value-1"
    aws_account_id = "value-1"
    notification_email = "value-1"
}
account_subscription_2 = provider.quicksight.Account_subscription {
    authentication_method = "value-2"
    account_name = "value-2"
    aws_account_id = "value-2"
    notification_email = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_subscription = provider.quicksight.Account_subscription {
        authentication_method = "production-value"
        account_name = "production-value"
        aws_account_id = "production-value"
        notification_email = "production-value"
    }
```

---

## Related Documentation

- [AWS Quicksight Documentation](https://docs.aws.amazon.com/quicksight/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
