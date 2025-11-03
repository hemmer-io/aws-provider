# Workdocs Service



**Resources**: 21

---

## Overview

The workdocs service provides access to 21 resource types:

- [Current_user](#current_user) [R]
- [Resources](#resources) [R]
- [Document_version](#document_version) [RUD]
- [Groups](#groups) [R]
- [Labels](#labels) [CD]
- [Document_versions](#document_versions) [R]
- [Document](#document) [RUD]
- [Users](#users) [R]
- [User](#user) [CUD]
- [Comments](#comments) [R]
- [Activities](#activities) [R]
- [Folder](#folder) [CRUD]
- [Comment](#comment) [CD]
- [Notification_subscription](#notification_subscription) [CD]
- [Folder_contents](#folder_contents) [RD]
- [Notification_subscriptions](#notification_subscriptions) [R]
- [Resource_permissions](#resource_permissions) [R]
- [Document_path](#document_path) [R]
- [Folder_path](#folder_path) [R]
- [Custom_metadata](#custom_metadata) [CD]
- [Root_folders](#root_folders) [R]

---

## Resources


### Current_user

CurrentUser resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | String | <p>Metadata of the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access current_user outputs
current_user_id = current_user.id
current_user_user = current_user.user
```

---


### Resources

Resources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `folders` | Vec<String> | <p>The folders in the specified folder.</p> |
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p> |
| `documents` | Vec<String> | <p>The documents in the specified collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resources outputs
resources_id = resources.id
resources_folders = resources.folders
resources_marker = resources.marker
resources_documents = resources.documents
```

---


### Document_version

DocumentVersion resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_id` | String | ✅ | <p>The version ID of the document.</p> |
| `document_id` | String | ✅ | <p>The ID of the document.</p> |
| `version_status` | String |  | <p>The status of the version.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | <p>The version metadata.</p> |
| `custom_metadata` | HashMap<String, String> | <p>The custom metadata on the document version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_version outputs
document_version_id = document_version.id
document_version_metadata = document_version.metadata
document_version_custom_metadata = document_version.custom_metadata
```

---


### Groups

Groups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `groups` | Vec<String> | <p>The list of groups.</p> |
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no additional
            results, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access groups outputs
groups_id = groups.id
groups_groups = groups.groups
groups_marker = groups.marker
```

---


### Labels

Labels resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |
| `labels` | Vec<String> | ✅ | <p>List of labels to add to the resource.</p> |
| `resource_id` | String | ✅ | <p>The ID of the resource.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create labels
labels = provider.workdocs.Labels {
    labels = "value"  # <p>List of labels to add to the resource.</p>
    resource_id = "value"  # <p>The ID of the resource.</p>
}

```

---


### Document_versions

DocumentVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_versions` | Vec<String> | <p>The document versions.</p> |
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no
            additional results, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_versions outputs
document_versions_id = document_versions.id
document_versions_document_versions = document_versions.document_versions
document_versions_marker = document_versions.marker
```

---


### Document

Document resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document_id` | String | ✅ | <p>The ID of the document.</p> |
| `parent_folder_id` | String |  | <p>The ID of the parent folder.</p> |
| `resource_state` | String |  | <p>The resource state of the document. Only ACTIVE and RECYCLED are
            supported.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |
| `name` | String |  | <p>The name of the document.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | <p>The metadata details of the document.</p> |
| `custom_metadata` | HashMap<String, String> | <p>The custom metadata on the document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document outputs
document_id = document.id
document_metadata = document.metadata
document_custom_metadata = document.custom_metadata
```

---


### Users

Users resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `users` | Vec<String> | <p>The users.</p> |
| `total_number_of_users` | i64 | <p>The total number of users included in the results.</p> |
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no
            additional results, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access users outputs
users_id = users.id
users_users = users.users
users_total_number_of_users = users.total_number_of_users
users_marker = users.marker
```

---


### User

User resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `given_name` | String | ✅ | <p>The given name of the user.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |
| `storage_rule` | String |  | <p>The amount of storage for the user.</p> |
| `password` | String | ✅ | <p>The password of the user.</p> |
| `time_zone_id` | String |  | <p>The time zone ID of the user.</p> |
| `surname` | String | ✅ | <p>The surname of the user.</p> |
| `email_address` | String |  | <p>The email address of the user.</p> |
| `username` | String | ✅ | <p>The login name of the user.</p> |
| `organization_id` | String |  | <p>The ID of the organization.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.workdocs.User {
    given_name = "value"  # <p>The given name of the user.</p>
    password = "value"  # <p>The password of the user.</p>
    surname = "value"  # <p>The surname of the user.</p>
    username = "value"  # <p>The login name of the user.</p>
}

```

---


### Comments

Comments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comments` | Vec<String> | <p>The list of comments for the specified document version.</p> |
| `marker` | String | <p>The marker for the next set of results. This marker was received from a previous
            call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access comments outputs
comments_id = comments.id
comments_comments = comments.comments
comments_marker = comments.marker
```

---


### Activities

Activities resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>The marker for the next set of results.</p> |
| `user_activities` | Vec<String> | <p>The list of activities for the specified user and time period.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access activities outputs
activities_id = activities.id
activities_marker = activities.marker
activities_user_activities = activities.user_activities
```

---


### Folder

Folder resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_folder_id` | String | ✅ | <p>The ID of the parent folder.</p> |
| `name` | String |  | <p>The name of the new folder.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_metadata` | HashMap<String, String> | <p>The custom metadata on the folder.</p> |
| `metadata` | String | <p>The metadata of the folder.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create folder
folder = provider.workdocs.Folder {
    parent_folder_id = "value"  # <p>The ID of the parent folder.</p>
}

# Access folder outputs
folder_id = folder.id
folder_custom_metadata = folder.custom_metadata
folder_metadata = folder.metadata
```

---


### Comment

Comment resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thread_id` | String |  | <p>The ID of the root comment in the thread.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |
| `notify_collaborators` | bool |  | <p>Set this parameter to TRUE to send an email out to the document collaborators after
            the comment is created.</p> |
| `parent_id` | String |  | <p>The ID of the parent comment.</p> |
| `version_id` | String | ✅ | <p>The ID of the document version.</p> |
| `visibility` | String |  | <p>The visibility of the comment. Options are either PRIVATE, where the comment is
            visible only to the comment author and document owner and co-owners, or PUBLIC, where
            the comment is visible to document owners, co-owners, and contributors.</p> |
| `document_id` | String | ✅ | <p>The ID of the document.</p> |
| `text` | String | ✅ | <p>The text of the comment.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create comment
comment = provider.workdocs.Comment {
    version_id = "value"  # <p>The ID of the document version.</p>
    document_id = "value"  # <p>The ID of the document.</p>
    text = "value"  # <p>The text of the comment.</p>
}

```

---


### Notification_subscription

NotificationSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subscription_type` | String | ✅ | <p>The notification type.</p> |
| `organization_id` | String | ✅ | <p>The ID of the organization.</p> |
| `protocol` | String | ✅ | <p>The protocol to use. The supported value is https, which delivers JSON-encoded
            messages using HTTPS POST.</p> |
| `endpoint` | String | ✅ | <p>The endpoint to receive the notifications. If the protocol is HTTPS, the endpoint
            is a URL that begins with <code>https</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notification_subscription
notification_subscription = provider.workdocs.Notification_subscription {
    subscription_type = "value"  # <p>The notification type.</p>
    organization_id = "value"  # <p>The ID of the organization.</p>
    protocol = "value"  # <p>The protocol to use. The supported value is https, which delivers JSON-encoded
            messages using HTTPS POST.</p>
    endpoint = "value"  # <p>The endpoint to receive the notifications. If the protocol is HTTPS, the endpoint
            is a URL that begins with <code>https</code>.</p>
}

```

---


### Folder_contents

FolderContents resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documents` | Vec<String> | <p>The documents in the specified folder.</p> |
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no
            additional results, the string is empty.</p> |
| `folders` | Vec<String> | <p>The subfolders in the specified folder.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access folder_contents outputs
folder_contents_id = folder_contents.id
folder_contents_documents = folder_contents.documents
folder_contents_marker = folder_contents.marker
folder_contents_folders = folder_contents.folders
```

---


### Notification_subscriptions

NotificationSubscriptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscriptions` | Vec<String> | <p>The subscriptions.</p> |
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no
            additional results, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access notification_subscriptions outputs
notification_subscriptions_id = notification_subscriptions.id
notification_subscriptions_subscriptions = notification_subscriptions.subscriptions
notification_subscriptions_marker = notification_subscriptions.marker
```

---


### Resource_permissions

ResourcePermissions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `principals` | Vec<String> | <p>The principals.</p> |
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no
            additional results, the string is empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_permissions outputs
resource_permissions_id = resource_permissions.id
resource_permissions_principals = resource_permissions.principals
resource_permissions_marker = resource_permissions.marker
```

---


### Document_path

DocumentPath resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `path` | String | <p>The path information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_path outputs
document_path_id = document_path.id
document_path_path = document_path.path
```

---


### Folder_path

FolderPath resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `path` | String | <p>The path information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access folder_path outputs
folder_path_id = folder_path.id
folder_path_path = folder_path.path
```

---


### Custom_metadata

CustomMetadata resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_id` | String |  | <p>The ID of the version, if the custom metadata is being added to a document
            version.</p> |
| `resource_id` | String | ✅ | <p>The ID of the resource.</p> |
| `custom_metadata` | HashMap<String, String> | ✅ | <p>Custom metadata in the form of name-value pairs.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_metadata
custom_metadata = provider.workdocs.Custom_metadata {
    resource_id = "value"  # <p>The ID of the resource.</p>
    custom_metadata = "value"  # <p>Custom metadata in the form of name-value pairs.</p>
}

```

---


### Root_folders

RootFolders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `folders` | Vec<String> | <p>The user's special folders.</p> |
| `marker` | String | <p>The marker for the next set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access root_folders outputs
root_folders_id = root_folders.id
root_folders_folders = root_folders.folders
root_folders_marker = root_folders.marker
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple current_user resources
current_user_0 = provider.workdocs.Current_user {
}
current_user_1 = provider.workdocs.Current_user {
}
current_user_2 = provider.workdocs.Current_user {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    current_user = provider.workdocs.Current_user {
    }
```

---

## Related Documentation

- [AWS Workdocs Documentation](https://docs.aws.amazon.com/workdocs/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
