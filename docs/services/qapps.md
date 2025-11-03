# Qapps Service



**Resources**: 7

---

## Overview

The qapps service provides access to 7 resource types:

- [Library_item_metadata](#library_item_metadata) [U]
- [Q_app_session_metadata](#q_app_session_metadata) [RU]
- [Library_item](#library_item) [CRUD]
- [Q_app_permissions](#q_app_permissions) [RU]
- [Q_app](#q_app) [CRUD]
- [Q_app_session](#q_app_session) [RU]
- [Presigned_url](#presigned_url) [C]

---

## Resources


### Library_item_metadata

LibraryItemMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `library_item_id` | String | ✅ | <p>The unique identifier of the updated library item.</p> |
| `instance_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application environment instance.</p> |
| `is_verified` | bool |  | <p>The verification status of the library item</p> |



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


### Q_app_session_metadata

QAppSessionMetadata resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application environment instance.</p> |
| `session_name` | String |  | <p>The new name for the Q App session.</p> |
| `session_id` | String | ✅ | <p>The unique identifier of the Q App session to update configuration for.</p> |
| `sharing_configuration` | String | ✅ | <p>The new sharing configuration for the Q App data collection session.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_id` | String | <p>The unique identifier of the Q App session.</p> |
| `session_arn` | String | <p>The Amazon Resource Name (ARN) of the Q App session.</p> |
| `session_name` | String | <p>The name of the Q App session.</p> |
| `sharing_configuration` | String | <p>The sharing configuration of the Q App data collection session.</p> |
| `session_owner` | bool | <p>Indicates whether the current user is the owner of the Q App session.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access q_app_session_metadata outputs
q_app_session_metadata_id = q_app_session_metadata.id
q_app_session_metadata_session_id = q_app_session_metadata.session_id
q_app_session_metadata_session_arn = q_app_session_metadata.session_arn
q_app_session_metadata_session_name = q_app_session_metadata.session_name
q_app_session_metadata_sharing_configuration = q_app_session_metadata.sharing_configuration
q_app_session_metadata_session_owner = q_app_session_metadata.session_owner
```

---


### Library_item

LibraryItem resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `categories` | Vec<String> | ✅ | <p>The categories to associate with the library item for easier discovery.</p> |
| `instance_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application environment instance.</p> |
| `app_version` | i64 | ✅ | <p>The version of the Amazon Q App to publish to the library.</p> |
| `app_id` | String | ✅ | <p>The unique identifier of the Amazon Q App to publish to the library.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The date and time the library item was created.</p> |
| `created_by` | String | <p>The user who created the library item.</p> |
| `updated_by` | String | <p>The user who last updated the library item.</p> |
| `user_count` | i64 | <p>The number of users who have associated the Q App with their account.</p> |
| `app_id` | String | <p>The unique identifier of the Q App associated with the library item.</p> |
| `library_item_id` | String | <p>The unique identifier of the library item.</p> |
| `app_version` | i64 | <p>The version of the Q App associated with the library item.</p> |
| `is_rated_by_user` | bool | <p>Whether the current user has rated the library item.</p> |
| `rating_count` | i64 | <p>The number of ratings the library item has received from users.</p> |
| `is_verified` | bool | <p>Indicates whether the library item has been verified.</p> |
| `categories` | Vec<String> | <p>The categories associated with the library item for discovery.</p> |
| `updated_at` | String | <p>The date and time the library item was last updated.</p> |
| `status` | String | <p>The status of the library item, such as "Published".</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create library_item
library_item = provider.qapps.Library_item {
    categories = "value"  # <p>The categories to associate with the library item for easier discovery.</p>
    instance_id = "value"  # <p>The unique identifier of the Amazon Q Business application environment instance.</p>
    app_version = "value"  # <p>The version of the Amazon Q App to publish to the library.</p>
    app_id = "value"  # <p>The unique identifier of the Amazon Q App to publish to the library.</p>
}

# Access library_item outputs
library_item_id = library_item.id
library_item_created_at = library_item.created_at
library_item_created_by = library_item.created_by
library_item_updated_by = library_item.updated_by
library_item_user_count = library_item.user_count
library_item_app_id = library_item.app_id
library_item_library_item_id = library_item.library_item_id
library_item_app_version = library_item.app_version
library_item_is_rated_by_user = library_item.is_rated_by_user
library_item_rating_count = library_item.rating_count
library_item_is_verified = library_item.is_verified
library_item_categories = library_item.categories
library_item_updated_at = library_item.updated_at
library_item_status = library_item.status
```

---


### Q_app_permissions

QAppPermissions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The unique identifier of the Amazon Q App for which permissions are being updated.</p> |
| `revoke_permissions` | Vec<String> |  | <p>The list of permissions to revoke for the Amazon Q App.</p> |
| `instance_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application environment instance.</p> |
| `grant_permissions` | Vec<String> |  | <p>The list of permissions to grant for the Amazon Q App.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the Amazon Q App for which permissions are returned.</p> |
| `permissions` | Vec<String> | <p>The list of permissions granted for the Amazon Q App.</p> |
| `app_id` | String | <p>The unique identifier of the Amazon Q App for which permissions are returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access q_app_permissions outputs
q_app_permissions_id = q_app_permissions.id
q_app_permissions_resource_arn = q_app_permissions.resource_arn
q_app_permissions_permissions = q_app_permissions.permissions
q_app_permissions_app_id = q_app_permissions.app_id
```

---


### Q_app

QApp resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the new Q App.</p> |
| `app_definition` | String | ✅ | <p>The definition of the new Q App, specifying the cards and flow.</p> |
| `tags` | HashMap<String, String> |  | <p>Optional tags to associate with the new Q App.</p> |
| `instance_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application environment instance.</p> |
| `title` | String | ✅ | <p>The title of the new Q App.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The date and time the Q App was created.</p> |
| `created_by` | String | <p>The user who created the Q App.</p> |
| `updated_at` | String | <p>The date and time the Q App was last updated.</p> |
| `updated_by` | String | <p>The user who last updated the Q App.</p> |
| `required_capabilities` | Vec<String> | <p>The capabilities required to run the Q App, such as file upload or third-party integrations.</p> |
| `status` | String | <p>The status of the Q App.</p> |
| `app_definition` | String | <p>The full definition of the Q App, specifying the cards and flow.</p> |
| `app_version` | i64 | <p>The version of the Q App.</p> |
| `title` | String | <p>The title of the Q App.</p> |
| `app_id` | String | <p>The unique identifier of the Q App.</p> |
| `initial_prompt` | String | <p>The initial prompt displayed when the Q App is started.</p> |
| `app_arn` | String | <p>The Amazon Resource Name (ARN) of the Q App.</p> |
| `description` | String | <p>The description of the Q App.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create q_app
q_app = provider.qapps.Q_app {
    app_definition = "value"  # <p>The definition of the new Q App, specifying the cards and flow.</p>
    instance_id = "value"  # <p>The unique identifier of the Amazon Q Business application environment instance.</p>
    title = "value"  # <p>The title of the new Q App.</p>
}

# Access q_app outputs
q_app_id = q_app.id
q_app_created_at = q_app.created_at
q_app_created_by = q_app.created_by
q_app_updated_at = q_app.updated_at
q_app_updated_by = q_app.updated_by
q_app_required_capabilities = q_app.required_capabilities
q_app_status = q_app.status
q_app_app_definition = q_app.app_definition
q_app_app_version = q_app.app_version
q_app_title = q_app.title
q_app_app_id = q_app.app_id
q_app_initial_prompt = q_app.initial_prompt
q_app_app_arn = q_app.app_arn
q_app_description = q_app.description
```

---


### Q_app_session

QAppSession resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_id` | String | ✅ | <p>The unique identifier of the Q App session to provide input for.</p> |
| `values` | Vec<String> |  | <p>The input values to provide for the current state of the Q App session.</p> |
| `instance_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application environment instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_arn` | String | <p>The Amazon Resource Name (ARN) of the Q App session.</p> |
| `session_name` | String | <p>The name of the Q App session.</p> |
| `latest_published_app_version` | i64 | <p>The latest published version of the Q App used for the session.</p> |
| `user_is_host` | bool | <p>Indicates whether the current user is the owner of the Q App data collection session.</p> |
| `app_version` | i64 | <p>The version of the Q App used for the session.</p> |
| `status` | String | <p>The current status of the Q App session.</p> |
| `session_id` | String | <p>The unique identifier of the Q App session.</p> |
| `card_status` | HashMap<String, String> | <p>The current status for each card in the Q App session.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access q_app_session outputs
q_app_session_id = q_app_session.id
q_app_session_session_arn = q_app_session.session_arn
q_app_session_session_name = q_app_session.session_name
q_app_session_latest_published_app_version = q_app_session.latest_published_app_version
q_app_session_user_is_host = q_app_session.user_is_host
q_app_session_app_version = q_app_session.app_version
q_app_session_status = q_app_session.status
q_app_session_session_id = q_app_session.session_id
q_app_session_card_status = q_app_session.card_status
```

---


### Presigned_url

PresignedUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scope` | String | ✅ | <p>Whether the file is associated with a Q App definition or a specific Q App session.</p> |
| `file_contents_sha256` | String | ✅ | <p>The Base64-encoded SHA-256 digest of the contents of the file to be uploaded.</p> |
| `card_id` | String | ✅ | <p>The unique identifier of the card the file is associated with.</p> |
| `session_id` | String |  | <p>The unique identifier of the Q App session the file is associated with, if applicable.</p> |
| `instance_id` | String | ✅ | <p>The unique identifier of the Amazon Q Business application environment instance.</p> |
| `file_name` | String | ✅ | <p>The name of the file to be uploaded.</p> |
| `app_id` | String | ✅ | <p>The unique identifier of the Q App the file is associated with.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create presigned_url
presigned_url = provider.qapps.Presigned_url {
    scope = "value"  # <p>Whether the file is associated with a Q App definition or a specific Q App session.</p>
    file_contents_sha256 = "value"  # <p>The Base64-encoded SHA-256 digest of the contents of the file to be uploaded.</p>
    card_id = "value"  # <p>The unique identifier of the card the file is associated with.</p>
    instance_id = "value"  # <p>The unique identifier of the Amazon Q Business application environment instance.</p>
    file_name = "value"  # <p>The name of the file to be uploaded.</p>
    app_id = "value"  # <p>The unique identifier of the Q App the file is associated with.</p>
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

# Create multiple library_item_metadata resources
library_item_metadata_0 = provider.qapps.Library_item_metadata {
    library_item_id = "value-0"
    instance_id = "value-0"
}
library_item_metadata_1 = provider.qapps.Library_item_metadata {
    library_item_id = "value-1"
    instance_id = "value-1"
}
library_item_metadata_2 = provider.qapps.Library_item_metadata {
    library_item_id = "value-2"
    instance_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    library_item_metadata = provider.qapps.Library_item_metadata {
        library_item_id = "production-value"
        instance_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Qapps Documentation](https://docs.aws.amazon.com/qapps/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
