# Workdocs Service



**Resources**: 21

---

## Overview

The workdocs service provides access to 21 resource types:

- [Document_versions](#document_versions) [R]
- [Root_folders](#root_folders) [R]
- [Folder_path](#folder_path) [R]
- [Notification_subscription](#notification_subscription) [CD]
- [Document](#document) [RUD]
- [Document_path](#document_path) [R]
- [Users](#users) [R]
- [Resources](#resources) [R]
- [Groups](#groups) [R]
- [Folder](#folder) [CRUD]
- [Document_version](#document_version) [RUD]
- [Folder_contents](#folder_contents) [RD]
- [Custom_metadata](#custom_metadata) [CD]
- [Activities](#activities) [R]
- [Resource_permissions](#resource_permissions) [R]
- [Current_user](#current_user) [R]
- [Notification_subscriptions](#notification_subscriptions) [R]
- [Comment](#comment) [CD]
- [Comments](#comments) [R]
- [Labels](#labels) [CD]
- [User](#user) [CUD]

---

## Resources


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


### Notification_subscription

NotificationSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocol` | String | ✅ | <p>The protocol to use. The supported value is https, which delivers JSON-encoded
            messages using HTTPS POST.</p> |
| `subscription_type` | String | ✅ | <p>The notification type.</p> |
| `organization_id` | String | ✅ | <p>The ID of the organization.</p> |
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
    protocol = "value"  # <p>The protocol to use. The supported value is https, which delivers JSON-encoded
            messages using HTTPS POST.</p>
    subscription_type = "value"  # <p>The notification type.</p>
    organization_id = "value"  # <p>The ID of the organization.</p>
    endpoint = "value"  # <p>The endpoint to receive the notifications. If the protocol is HTTPS, the endpoint
            is a URL that begins with <code>https</code>.</p>
}

```

---


### Document

Document resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_folder_id` | String |  | <p>The ID of the parent folder.</p> |
| `resource_state` | String |  | <p>The resource state of the document. Only ACTIVE and RECYCLED are
            supported.</p> |
| `document_id` | String | ✅ | <p>The ID of the document.</p> |
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


### Users

Users resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_number_of_users` | i64 | <p>The total number of users included in the results.</p> |
| `users` | Vec<String> | <p>The users.</p> |
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
users_total_number_of_users = users.total_number_of_users
users_users = users.users
users_marker = users.marker
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
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p> |
| `folders` | Vec<String> | <p>The folders in the specified folder.</p> |
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
resources_marker = resources.marker
resources_folders = resources.folders
resources_documents = resources.documents
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


### Folder

Folder resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the new folder.</p> |
| `parent_folder_id` | String | ✅ | <p>The ID of the parent folder.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | <p>The metadata of the folder.</p> |
| `custom_metadata` | HashMap<String, String> | <p>The custom metadata on the folder.</p> |


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
folder_metadata = folder.metadata
folder_custom_metadata = folder.custom_metadata
```

---


### Document_version

DocumentVersion resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document_id` | String | ✅ | <p>The ID of the document.</p> |
| `version_status` | String |  | <p>The status of the version.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |
| `version_id` | String | ✅ | <p>The version ID of the document.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_metadata` | HashMap<String, String> | <p>The custom metadata on the document version.</p> |
| `metadata` | String | <p>The version metadata.</p> |


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
document_version_custom_metadata = document_version.custom_metadata
document_version_metadata = document_version.metadata
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
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no
            additional results, the string is empty.</p> |
| `folders` | Vec<String> | <p>The subfolders in the specified folder.</p> |
| `documents` | Vec<String> | <p>The documents in the specified folder.</p> |


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
folder_contents_marker = folder_contents.marker
folder_contents_folders = folder_contents.folders
folder_contents_documents = folder_contents.documents
```

---


### Custom_metadata

CustomMetadata resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_metadata` | HashMap<String, String> | ✅ | <p>Custom metadata in the form of name-value pairs.</p> |
| `version_id` | String |  | <p>The ID of the version, if the custom metadata is being added to a document
            version.</p> |
| `resource_id` | String | ✅ | <p>The ID of the resource.</p> |
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
    custom_metadata = "value"  # <p>Custom metadata in the form of name-value pairs.</p>
    resource_id = "value"  # <p>The ID of the resource.</p>
}

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
| `user_activities` | Vec<String> | <p>The list of activities for the specified user and time period.</p> |
| `marker` | String | <p>The marker for the next set of results.</p> |


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
activities_user_activities = activities.user_activities
activities_marker = activities.marker
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


### Notification_subscriptions

NotificationSubscriptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>The marker to use when requesting the next set of results. If there are no
            additional results, the string is empty.</p> |
| `subscriptions` | Vec<String> | <p>The subscriptions.</p> |


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
notification_subscriptions_marker = notification_subscriptions.marker
notification_subscriptions_subscriptions = notification_subscriptions.subscriptions
```

---


### Comment

Comment resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |
| `thread_id` | String |  | <p>The ID of the root comment in the thread.</p> |
| `notify_collaborators` | bool |  | <p>Set this parameter to TRUE to send an email out to the document collaborators after
            the comment is created.</p> |
| `text` | String | ✅ | <p>The text of the comment.</p> |
| `visibility` | String |  | <p>The visibility of the comment. Options are either PRIVATE, where the comment is
            visible only to the comment author and document owner and co-owners, or PUBLIC, where
            the comment is visible to document owners, co-owners, and contributors.</p> |
| `document_id` | String | ✅ | <p>The ID of the document.</p> |
| `parent_id` | String |  | <p>The ID of the parent comment.</p> |
| `version_id` | String | ✅ | <p>The ID of the document version.</p> |



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
    text = "value"  # <p>The text of the comment.</p>
    document_id = "value"  # <p>The ID of the document.</p>
    version_id = "value"  # <p>The ID of the document version.</p>
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
| `marker` | String | <p>The marker for the next set of results. This marker was received from a previous
            call.</p> |
| `comments` | Vec<String> | <p>The list of comments for the specified document version.</p> |


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
comments_marker = comments.marker
comments_comments = comments.comments
```

---


### Labels

Labels resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_id` | String | ✅ | <p>The ID of the resource.</p> |
| `labels` | Vec<String> | ✅ | <p>List of labels to add to the resource.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |



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
    resource_id = "value"  # <p>The ID of the resource.</p>
    labels = "value"  # <p>List of labels to add to the resource.</p>
}

```

---


### User

User resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `surname` | String | ✅ | <p>The surname of the user.</p> |
| `password` | String | ✅ | <p>The password of the user.</p> |
| `time_zone_id` | String |  | <p>The time zone ID of the user.</p> |
| `storage_rule` | String |  | <p>The amount of storage for the user.</p> |
| `username` | String | ✅ | <p>The login name of the user.</p> |
| `organization_id` | String |  | <p>The ID of the organization.</p> |
| `authentication_token` | String |  | <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p> |
| `email_address` | String |  | <p>The email address of the user.</p> |
| `given_name` | String | ✅ | <p>The given name of the user.</p> |



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
    surname = "value"  # <p>The surname of the user.</p>
    password = "value"  # <p>The password of the user.</p>
    username = "value"  # <p>The login name of the user.</p>
    given_name = "value"  # <p>The given name of the user.</p>
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

# Create multiple document_versions resources
document_versions_0 = provider.workdocs.Document_versions {
}
document_versions_1 = provider.workdocs.Document_versions {
}
document_versions_2 = provider.workdocs.Document_versions {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    document_versions = provider.workdocs.Document_versions {
    }
```

---

## Related Documentation

- [AWS Workdocs Documentation](https://docs.aws.amazon.com/workdocs/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
