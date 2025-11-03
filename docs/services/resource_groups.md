# Resource_groups Service



**Resources**: 6

---

## Overview

The resource_groups service provides access to 6 resource types:

- [Account_settings](#account_settings) [RU]
- [Group_query](#group_query) [RU]
- [Tag_sync_task](#tag_sync_task) [R]
- [Group_configuration](#group_configuration) [CR]
- [Group](#group) [CRUD]
- [Tags](#tags) [R]

---

## Resources


### Account_settings

AccountSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_lifecycle_events_desired_status` | String |  | <p>Specifies whether you want to turn <a href="https://docs.aws.amazon.com/ARG/latest/userguide/monitor-groups.html">group lifecycle events</a> on or off.</p>
         <p>You can't turn on group lifecycle events if your resource groups quota is greater than 2,000. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_settings` | String | <p>The current settings for the optional features in Resource Groups.</p> |


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
```

---


### Group_query

GroupQuery resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group` | String |  | <p>The name or the Amazon resource name (ARN) of the resource group to query.</p> |
| `resource_query` | String | ✅ | <p>The resource query to determine which Amazon Web Services resources are members of this resource
            group.</p>
         <note>
            <p>A resource group can contain either a <code>Configuration</code> or a
                    <code>ResourceQuery</code>, but not both.</p>
         </note> |
| `group_name` | String |  | <p>Don't use this parameter. Use <code>Group</code> instead.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_query` | String | <p>The resource query associated with the specified group. For more information about
            resource queries, see <a href="https://docs.aws.amazon.com/ARG/latest/userguide/gettingstarted-query.html#gettingstarted-query-cli-tag">Create
                a tag-based group in Resource Groups</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access group_query outputs
group_query_id = group_query.id
group_query_group_query = group_query.group_query
```

---


### Tag_sync_task

TagSyncTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the tag-sync task. </p>
         <p>Valid values include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code> - The tag-sync task is actively managing resources in 
                    the application by adding or removing the <code>awsApplication</code> tag from resources 
                    when they are tagged or untagged with the specified tag key-value pair. 
                </p>
            </li>
            <li>
               <p>
                  <code>ERROR</code> - The tag-sync task is not actively managing resources 
                    in the application. Review the <code>ErrorMessage</code> for more information about 
                    resolving the error. 
                </p>
            </li>
         </ul> |
| `task_arn` | String | <p>The Amazon resource name (ARN) of the tag-sync task. </p> |
| `group_name` | String | <p>The name of the application group. </p> |
| `tag_key` | String | <p>The tag key. </p> |
| `error_message` | String | <p>The specific error message in cases where the tag-sync task status
            is <code>ERROR</code>. </p> |
| `group_arn` | String | <p>The Amazon resource name (ARN) of the application group. </p> |
| `role_arn` | String | <p>The Amazon resource name (ARN) of the role assumed by Resource Groups to tag and untag resources on your behalf. </p>
         <p>For more information about this role, review <a href="https://docs.aws.amazon.com/servicecatalog/latest/arguide/app-tag-sync.html#tag-sync-role">Tag-sync required permissions</a>. 
        </p> |
| `created_at` | String | <p>The timestamp of when the tag-sync task was created. </p> |
| `tag_value` | String | <p>The tag value. </p> |
| `resource_query` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tag_sync_task outputs
tag_sync_task_id = tag_sync_task.id
tag_sync_task_status = tag_sync_task.status
tag_sync_task_task_arn = tag_sync_task.task_arn
tag_sync_task_group_name = tag_sync_task.group_name
tag_sync_task_tag_key = tag_sync_task.tag_key
tag_sync_task_error_message = tag_sync_task.error_message
tag_sync_task_group_arn = tag_sync_task.group_arn
tag_sync_task_role_arn = tag_sync_task.role_arn
tag_sync_task_created_at = tag_sync_task.created_at
tag_sync_task_tag_value = tag_sync_task.tag_value
tag_sync_task_resource_query = tag_sync_task.resource_query
```

---


### Group_configuration

GroupConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group` | String |  | <p>The name or Amazon resource name (ARN) of the resource group with the configuration that you want to
            update.</p> |
| `configuration` | Vec<String> |  | <p>The new configuration to associate with the specified group. A configuration
            associates the resource group with an Amazon Web Services service and specifies how the service can
            interact with the resources in the group. A configuration is an array of <a>GroupConfigurationItem</a> elements.</p>
         <p>For information about the syntax of a service configuration, see <a href="https://docs.aws.amazon.com/ARG/latest/APIReference/about-slg.html">Service configurations for
                Resource Groups</a>.</p>
         <note>
            <p>A resource group can contain either a <code>Configuration</code> or a
                    <code>ResourceQuery</code>, but not both.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_configuration` | String | <p>A structure that describes the service configuration attached with the specified
            group. For details about the service configuration syntax, see <a href="https://docs.aws.amazon.com/ARG/latest/APIReference/about-slg.html">Service configurations for
                Resource Groups</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group_configuration
group_configuration = provider.resource_groups.Group_configuration {
}

# Access group_configuration outputs
group_configuration_id = group_configuration.id
group_configuration_group_configuration = group_configuration.group_configuration
```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p>The name of the application group, which you can change at any time. </p> |
| `name` | String | ✅ | <p>The name of the group, which is the identifier of the group in other operations. You
            can't change the name of a resource group after you create it. A resource group name can
            consist of letters, numbers, hyphens, periods, and underscores. The name cannot start
            with <code>AWS</code>, <code>aws</code>, or any other possible capitalization; these are
            reserved. A resource group name must be unique within each Amazon Web Services Region in your Amazon Web Services
            account.</p> |
| `resource_query` | String |  | <p>The resource query that determines which Amazon Web Services resources are members of this group.
            For more information about resource queries, see <a href="https://docs.aws.amazon.com/ARG/latest/userguide/gettingstarted-query.html#gettingstarted-query-cli-tag">Create
                a tag-based group in Resource Groups</a>. </p>
         <note>
            <p>A resource group can contain either a <code>ResourceQuery</code> or a
                    <code>Configuration</code>, but not both.</p>
         </note> |
| `configuration` | Vec<String> |  | <p>A configuration associates the resource group with an Amazon Web Services service and specifies how
            the service can interact with the resources in the group. A configuration is an array of
                <a>GroupConfigurationItem</a> elements. For details about the syntax of
            service configurations, see <a href="https://docs.aws.amazon.com/ARG/latest/APIReference/about-slg.html">Service configurations for Resource Groups</a>.</p>
         <note>
            <p>A resource group can contain either a <code>Configuration</code> or a
                    <code>ResourceQuery</code>, but not both.</p>
         </note> |
| `description` | String |  | <p>The description of the resource group. Descriptions can consist of letters, numbers,
            hyphens, underscores, periods, and spaces.</p> |
| `criticality` | i64 |  | <p>The critical rank of the application group on a scale of 1 to 10, with a 
            rank of 1 being the most critical, and a rank of 10 being least critical.</p> |
| `owner` | String |  | <p>A name, email address or other identifier for the person or group 
            who is considered as the owner of this application group within your organization. </p> |
| `tags` | HashMap<String, String> |  | <p>The tags to add to the group. A tag is key-value pair string.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group` | String | <p>A structure that contains the metadata details for the specified resource group. Use
                <a>GetGroupQuery</a> and <a>GetGroupConfiguration</a> to get
            those additional details of the resource group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.resource_groups.Group {
    name = "value"  # <p>The name of the group, which is the identifier of the group in other operations. You
            can't change the name of a resource group after you create it. A resource group name can
            consist of letters, numbers, hyphens, periods, and underscores. The name cannot start
            with <code>AWS</code>, <code>aws</code>, or any other possible capitalization; these are
            reserved. A resource group name must be unique within each Amazon Web Services Region in your Amazon Web Services
            account.</p>
}

# Access group outputs
group_id = group.id
group_group = group.group
```

---


### Tags

Tags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>TheAmazon resource name (ARN) of the tagged resource group.</p> |
| `tags` | HashMap<String, String> | <p>The tags associated with the specified resource group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tags outputs
tags_id = tags.id
tags_arn = tags.arn
tags_tags = tags.tags
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple account_settings resources
account_settings_0 = provider.resource_groups.Account_settings {
}
account_settings_1 = provider.resource_groups.Account_settings {
}
account_settings_2 = provider.resource_groups.Account_settings {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_settings = provider.resource_groups.Account_settings {
    }
```

---

## Related Documentation

- [AWS Resource_groups Documentation](https://docs.aws.amazon.com/resource_groups/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
