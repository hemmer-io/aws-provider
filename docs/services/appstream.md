# Appstream Service



**Resources**: 34

---

## Overview

The appstream service provides access to 34 resource types:

- [Application](#application) [CUD]
- [User](#user) [CD]
- [App_block_builders](#app_block_builders) [R]
- [Images](#images) [R]
- [Updated_image](#updated_image) [C]
- [Fleets](#fleets) [R]
- [User_stack_associations](#user_stack_associations) [R]
- [App_block_builder_app_block_associations](#app_block_builder_app_block_associations) [R]
- [Image_permissions](#image_permissions) [RUD]
- [Theme_for_stack](#theme_for_stack) [CRUD]
- [App_block_builder_streaming_url](#app_block_builder_streaming_url) [C]
- [App_license_usage](#app_license_usage) [R]
- [Entitlements](#entitlements) [R]
- [Stacks](#stacks) [R]
- [Applications](#applications) [R]
- [Streaming_url](#streaming_url) [C]
- [Application_fleet_associations](#application_fleet_associations) [R]
- [Sessions](#sessions) [R]
- [App_block](#app_block) [CD]
- [Usage_report_subscription](#usage_report_subscription) [CD]
- [App_blocks](#app_blocks) [R]
- [Image_builders](#image_builders) [R]
- [Directory_configs](#directory_configs) [R]
- [Fleet](#fleet) [CUD]
- [Image_builder_streaming_url](#image_builder_streaming_url) [C]
- [Entitlement](#entitlement) [CUD]
- [Stack](#stack) [CUD]
- [Users](#users) [R]
- [Image](#image) [D]
- [Software_associations](#software_associations) [R]
- [Directory_config](#directory_config) [CUD]
- [Usage_report_subscriptions](#usage_report_subscriptions) [R]
- [App_block_builder](#app_block_builder) [CUD]
- [Image_builder](#image_builder) [CD]

---

## Resources


### Application

Application resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `icon_s3_location` | String | ✅ | <p>The location in S3 of the application icon.</p> |
| `launch_path` | String | ✅ | <p>The launch path of the application.</p> |
| `platforms` | Vec<String> | ✅ | <p>The platforms the application supports. WINDOWS_SERVER_2019 and AMAZON_LINUX2 are supported for Elastic fleets.</p> |
| `display_name` | String |  | <p>The display name of the application. This name is visible to users in the application catalog.</p> |
| `launch_parameters` | String |  | <p>The launch parameters of the application.</p> |
| `instance_families` | String | ✅ | <p>The instance families the application supports. Valid values are GENERAL_PURPOSE and GRAPHICS_G4.</p> |
| `name` | String | ✅ | <p>The name of the application. This name is visible to users when display name is not specified.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags assigned to the application.</p> |
| `working_directory` | String |  | <p>The working directory of the application.</p> |
| `app_block_arn` | String | ✅ | <p>The app block ARN to which the application should be associated</p> |
| `description` | String |  | <p>The description of the application.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.appstream.Application {
    icon_s3_location = "value"  # <p>The location in S3 of the application icon.</p>
    launch_path = "value"  # <p>The launch path of the application.</p>
    platforms = "value"  # <p>The platforms the application supports. WINDOWS_SERVER_2019 and AMAZON_LINUX2 are supported for Elastic fleets.</p>
    instance_families = "value"  # <p>The instance families the application supports. Valid values are GENERAL_PURPOSE and GRAPHICS_G4.</p>
    name = "value"  # <p>The name of the application. This name is visible to users when display name is not specified.</p>
    app_block_arn = "value"  # <p>The app block ARN to which the application should be associated</p>
}

```

---


### User

User resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authentication_type` | String | ✅ | <p>The authentication type for the user. You must specify USERPOOL. </p> |
| `first_name` | String |  | <p>The first name, or given name, of the user.</p> |
| `user_name` | String | ✅ | <p>The email address of the user.</p>
         <note>
            <p>Users' email addresses are case-sensitive. During login, if they specify an email address that doesn't use the same capitalization as the email address specified when their user pool account was created, a "user does not exist" error message displays.</p>
         </note> |
| `last_name` | String |  | <p>The last name, or surname, of the user.</p> |
| `message_action` | String |  | <p>The action to take for the welcome email that is sent to a user after the user is created in the user pool. If you specify SUPPRESS, no email is sent. If you specify RESEND, do not specify the first name or last name of the user. If the value is null, the email is sent. </p>
         <note>
            <p>The temporary password in the welcome email is valid for only 7 days. If users don’t set their passwords within 7 days, you must send them a new welcome email.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.appstream.User {
    authentication_type = "value"  # <p>The authentication type for the user. You must specify USERPOOL. </p>
    user_name = "value"  # <p>The email address of the user.</p>
         <note>
            <p>Users' email addresses are case-sensitive. During login, if they specify an email address that doesn't use the same capitalization as the email address specified when their user pool account was created, a "user does not exist" error message displays.</p>
         </note>
}

```

---


### App_block_builders

AppBlockBuilders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_block_builders` | Vec<String> | <p>The list that describes one or more app block builders.</p> |
| `next_token` | String | <p>The pagination token used to retrieve the next page of results for this
            operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_block_builders outputs
app_block_builders_id = app_block_builders.id
app_block_builders_app_block_builders = app_block_builders.app_block_builders
app_block_builders_next_token = app_block_builders.next_token
```

---


### Images

Images resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |
| `images` | Vec<String> | <p>Information about the images.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access images outputs
images_id = images.id
images_next_token = images.next_token
images_images = images.images
```

---


### Updated_image

UpdatedImage resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Indicates whether to display the status of image update availability before AppStream 2.0 initiates the process of creating a new updated image. If this value is set to <code>true</code>, AppStream 2.0 displays whether image updates are available. If this value is set to <code>false</code>, AppStream 2.0 initiates the process of creating a new updated image without displaying whether image updates are available.</p> |
| `new_image_display_name` | String |  | <p>The name to display for the new image.</p> |
| `new_image_name` | String | ✅ | <p>The name of the new image. The name must be unique within the AWS account and Region.</p> |
| `existing_image_name` | String | ✅ | <p>The name of the image to update.</p> |
| `new_image_description` | String |  | <p>The description to display for the new image.</p> |
| `new_image_tags` | HashMap<String, String> |  | <p>The tags to associate with the new image. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p>
         <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p>
         <p>_ . : / = + \ - @</p>
         <p>If you do not specify a value, the value is set to an empty string.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create updated_image
updated_image = provider.appstream.Updated_image {
    new_image_name = "value"  # <p>The name of the new image. The name must be unique within the AWS account and Region.</p>
    existing_image_name = "value"  # <p>The name of the image to update.</p>
}

```

---


### Fleets

Fleets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleets` | Vec<String> | <p>Information about the fleets.</p> |
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleets outputs
fleets_id = fleets.id
fleets_fleets = fleets.fleets
fleets_next_token = fleets.next_token
```

---


### User_stack_associations

UserStackAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |
| `user_stack_associations` | Vec<String> | <p>The UserStackAssociation objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_stack_associations outputs
user_stack_associations_id = user_stack_associations.id
user_stack_associations_next_token = user_stack_associations.next_token
user_stack_associations_user_stack_associations = user_stack_associations.user_stack_associations
```

---


### App_block_builder_app_block_associations

AppBlockBuilderAppBlockAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_block_builder_app_block_associations` | Vec<String> | <p>This list of app block builders associated with app blocks.</p> |
| `next_token` | String | <p>The pagination token used to retrieve the next page of results for this
            operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_block_builder_app_block_associations outputs
app_block_builder_app_block_associations_id = app_block_builder_app_block_associations.id
app_block_builder_app_block_associations_app_block_builder_app_block_associations = app_block_builder_app_block_associations.app_block_builder_app_block_associations
app_block_builder_app_block_associations_next_token = app_block_builder_app_block_associations.next_token
```

---


### Image_permissions

ImagePermissions resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_permissions` | String | ✅ | <p>The permissions for the image.</p> |
| `shared_account_id` | String | ✅ | <p>The 12-digit identifier of the AWS account for which you want add or update image permissions.</p> |
| `name` | String | ✅ | <p>The name of the private image.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the private image.</p> |
| `shared_image_permissions_list` | Vec<String> | <p>The permissions for a private image that you own. </p> |
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_permissions outputs
image_permissions_id = image_permissions.id
image_permissions_name = image_permissions.name
image_permissions_shared_image_permissions_list = image_permissions.shared_image_permissions_list
image_permissions_next_token = image_permissions.next_token
```

---


### Theme_for_stack

ThemeForStack resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stack_name` | String | ✅ | <p>The name of the stack for the theme.</p> |
| `title_text` | String | ✅ | <p>The title that is displayed at the top of the browser tab during users' application streaming sessions.</p> |
| `theme_styling` | String | ✅ | <p>The color theme that is applied to website links, text, and buttons. These colors are also applied as accents in the background for the streaming application catalog page.</p> |
| `organization_logo_s3_location` | String | ✅ | <p>The organization logo that appears on the streaming application catalog page.</p> |
| `favicon_s3_location` | String | ✅ | <p>The S3 location of the favicon. The favicon enables users to recognize their application streaming site in a browser full of tabs or bookmarks. It is displayed at the top of the browser tab for the application streaming site during users' streaming sessions.</p> |
| `footer_links` | Vec<String> |  | <p>The links that are displayed in the footer of the streaming application catalog page. These links are helpful resources for users, such as the organization's IT support and product marketing sites.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `theme` | String | <p> The theme object that contains the metadata of the custom branding.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create theme_for_stack
theme_for_stack = provider.appstream.Theme_for_stack {
    stack_name = "value"  # <p>The name of the stack for the theme.</p>
    title_text = "value"  # <p>The title that is displayed at the top of the browser tab during users' application streaming sessions.</p>
    theme_styling = "value"  # <p>The color theme that is applied to website links, text, and buttons. These colors are also applied as accents in the background for the streaming application catalog page.</p>
    organization_logo_s3_location = "value"  # <p>The organization logo that appears on the streaming application catalog page.</p>
    favicon_s3_location = "value"  # <p>The S3 location of the favicon. The favicon enables users to recognize their application streaming site in a browser full of tabs or bookmarks. It is displayed at the top of the browser tab for the application streaming site during users' streaming sessions.</p>
}

# Access theme_for_stack outputs
theme_for_stack_id = theme_for_stack.id
theme_for_stack_theme = theme_for_stack.theme
```

---


### App_block_builder_streaming_url

AppBlockBuilderStreamingURL resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_block_builder_name` | String | ✅ | <p>The name of the app block builder.</p> |
| `validity` | i64 |  | <p>The time that the streaming URL will be valid, in seconds. 
            Specify a value between 1 and 604800 seconds. The default is 3600 seconds.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_block_builder_streaming_url
app_block_builder_streaming_url = provider.appstream.App_block_builder_streaming_url {
    app_block_builder_name = "value"  # <p>The name of the app block builder.</p>
}

```

---


### App_license_usage

AppLicenseUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Token for pagination of results.</p> |
| `app_license_usages` | Vec<String> | <p>Collection of license usage records.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_license_usage outputs
app_license_usage_id = app_license_usage.id
app_license_usage_next_token = app_license_usage.next_token
app_license_usage_app_license_usages = app_license_usage.app_license_usages
```

---


### Entitlements

Entitlements resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entitlements` | Vec<String> | <p>The entitlements.</p> |
| `next_token` | String | <p>The pagination token used to retrieve the next page of results for this
            operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entitlements outputs
entitlements_id = entitlements.id
entitlements_entitlements = entitlements.entitlements
entitlements_next_token = entitlements.next_token
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
| `stacks` | Vec<String> | <p>Information about the stacks.</p> |
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |


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
stacks_stacks = stacks.stacks
stacks_next_token = stacks.next_token
```

---


### Applications

Applications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `applications` | Vec<String> | <p>The applications in the list.</p> |
| `next_token` | String | <p>The pagination token used to retrieve the next page of results for this
            operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access applications outputs
applications_id = applications.id
applications_applications = applications.applications
applications_next_token = applications.next_token
```

---


### Streaming_url

StreamingURL resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stack_name` | String | ✅ | <p>The name of the stack.</p> |
| `user_id` | String | ✅ | <p>The identifier of the user.</p> |
| `application_id` | String |  | <p>The name of the application to launch after the session starts. This is the name that you specified
            as <b>Name</b> in the Image Assistant. If your fleet is enabled for the <b>Desktop</b> stream view, you can also choose to launch directly to the operating system desktop. To do so, specify <b>Desktop</b>.</p> |
| `fleet_name` | String | ✅ | <p>The name of the fleet.</p> |
| `validity` | i64 |  | <p>The time that the streaming URL will be valid, in seconds.
            Specify a value between 1 and 604800 seconds. The default is 60 seconds.</p> |
| `session_context` | String |  | <p>The session context. For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/managing-stacks-fleets.html#managing-stacks-fleets-parameters">Session Context</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create streaming_url
streaming_url = provider.appstream.Streaming_url {
    stack_name = "value"  # <p>The name of the stack.</p>
    user_id = "value"  # <p>The identifier of the user.</p>
    fleet_name = "value"  # <p>The name of the fleet.</p>
}

```

---


### Application_fleet_associations

ApplicationFleetAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_fleet_associations` | Vec<String> | <p>The application fleet associations in the list.</p> |
| `next_token` | String | <p>The pagination token used to retrieve the next page of results for this
            operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_fleet_associations outputs
application_fleet_associations_id = application_fleet_associations.id
application_fleet_associations_application_fleet_associations = application_fleet_associations.application_fleet_associations
application_fleet_associations_next_token = application_fleet_associations.next_token
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
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |
| `sessions` | Vec<String> | <p>Information about the streaming sessions.</p> |


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


### App_block

AppBlock resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the app block.</p> |
| `display_name` | String |  | <p>The display name of the app block. This is not displayed to the user.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags assigned to the app block.</p> |
| `post_setup_script_details` | String |  | <p>The post setup script details of the app block. This can only be provided for the
                <code>APPSTREAM2</code> PackagingType.</p> |
| `setup_script_details` | String |  | <p>The setup script details of the app block. This must be provided for the
                <code>CUSTOM</code> PackagingType.</p> |
| `packaging_type` | String |  | <p>The packaging type of the app block.</p> |
| `name` | String | ✅ | <p>The name of the app block.</p> |
| `source_s3_location` | String | ✅ | <p>The source S3 location of the app block.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_block
app_block = provider.appstream.App_block {
    name = "value"  # <p>The name of the app block.</p>
    source_s3_location = "value"  # <p>The source S3 location of the app block.</p>
}

```

---


### Usage_report_subscription

UsageReportSubscription resource

**Operations**: ✅ Create ✅ Delete

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

# Create usage_report_subscription
usage_report_subscription = provider.appstream.Usage_report_subscription {
}

```

---


### App_blocks

AppBlocks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_blocks` | Vec<String> | <p>The app blocks in the list.</p> |
| `next_token` | String | <p>The pagination token used to retrieve the next page of results for this
            operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_blocks outputs
app_blocks_id = app_blocks.id
app_blocks_app_blocks = app_blocks.app_blocks
app_blocks_next_token = app_blocks.next_token
```

---


### Image_builders

ImageBuilders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_builders` | Vec<String> | <p>Information about the image builders.</p> |
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_builders outputs
image_builders_id = image_builders.id
image_builders_image_builders = image_builders.image_builders
image_builders_next_token = image_builders.next_token
```

---


### Directory_configs

DirectoryConfigs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `directory_configs` | Vec<String> | <p>Information about the directory configurations. Note that although the response syntax in this topic includes the account password, this password is not returned in the actual response. </p> |
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access directory_configs outputs
directory_configs_id = directory_configs.id
directory_configs_directory_configs = directory_configs.directory_configs
directory_configs_next_token = directory_configs.next_token
```

---


### Fleet

Fleet resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_name` | String |  | <p>The name of the image used to create the fleet.</p> |
| `usb_device_filter_strings` | Vec<String> |  | <p>The USB device filter strings that specify which USB devices a user can redirect to the fleet streaming session, when using the Windows native client. This is allowed but not required for Elastic fleets.</p> |
| `stream_view` | String |  | <p>The AppStream 2.0 view that is displayed to your users when they stream from the fleet. When <code>APP</code> is specified, only the windows of applications opened by users display. When <code>DESKTOP</code> is specified, the standard desktop that is provided by the operating system displays.</p>
         <p>The default value is <code>APP</code>.</p> |
| `iam_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role to apply to the fleet. To assume a role, a fleet instance calls the AWS Security Token Service (STS) <code>AssumeRole</code> API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |
| `max_user_duration_in_seconds` | i64 |  | <p>The maximum amount of time that a streaming session can remain active, in seconds. If users are still connected to a streaming instance five minutes before this limit is reached, they are prompted to save any open documents before being disconnected. After this time elapses, the instance is terminated and replaced by a new instance.</p>
         <p>Specify a value between 600 and 432000.</p> |
| `idle_disconnect_timeout_in_seconds` | i64 |  | <p>The amount of time that users can be idle (inactive) before they are disconnected
            from their streaming session and the <code>DisconnectTimeoutInSeconds</code> time
            interval begins. Users are notified before they are disconnected due to inactivity. If
            they try to reconnect to the streaming session before the time interval specified in
            <code>DisconnectTimeoutInSeconds</code> elapses, they are connected to their
            previous session. Users are considered idle when they stop providing keyboard or mouse
            input during their streaming session. File uploads and downloads, audio in, audio out,
            and pixels changing do not qualify as user activity. If users continue to be idle after
            the time interval in <code>IdleDisconnectTimeoutInSeconds</code> elapses, they are
            disconnected.</p>
         <p>To prevent users from being disconnected due to inactivity, specify a value of 0. Otherwise, specify a value between 60 and 36000. The default value is 0.</p>
         <note>
            <p>If you enable this feature, we recommend that you specify a value that corresponds exactly to a whole number of minutes (for example, 60, 120, and 180). If you don't do this, the value is rounded to the nearest minute. For example, if you specify a value of 70, users are disconnected after 1 minute of inactivity. If you specify a value that is at the midpoint between two different minutes, the value is rounded up. For example, if you specify a value of 90, users are disconnected after 2 minutes of inactivity. </p>
         </note> |
| `domain_join_info` | String |  | <p>The name of the directory and organizational unit (OU) to use to join the fleet to a Microsoft Active Directory domain. This is not allowed for Elastic fleets. </p> |
| `session_script_s3_location` | String |  | <p>The S3 location of the session scripts configuration zip file. This only applies to Elastic fleets.</p> |
| `vpc_config` | String |  | <p>The VPC configuration for the fleet. This is required for Elastic fleets, but not required for other fleet types. Elastic fleets require that you specify at least two subnets in different availability zones.</p> |
| `disconnect_timeout_in_seconds` | i64 |  | <p>The amount of time that a streaming session remains active after users disconnect. If users try to reconnect to the streaming session after a disconnection or network interruption within this time interval, they are connected to their previous session. Otherwise, they are connected to a new session with a new streaming instance. </p>
         <p>Specify a value between 60 and 36000.</p> |
| `fleet_type` | String |  | <p>The fleet type.</p>
         <dl>
            <dt>ALWAYS_ON</dt>
            <dd>
               <p>Provides users with instant-on access to their apps.
                        You are charged for all running instances in your fleet, even if no users are streaming apps.</p>
            </dd>
            <dt>ON_DEMAND</dt>
            <dd>
               <p>Provide users with access to applications after they connect, which takes one to two minutes.
                        You are charged for instance streaming when users are connected and a
                        small hourly fee for instances that are not streaming apps.</p>
            </dd>
         </dl> |
| `compute_capacity` | String |  | <p>The desired capacity for the fleet. This is not allowed for Elastic fleets. For Elastic fleets, specify MaxConcurrentSessions instead.</p> |
| `max_concurrent_sessions` | i64 |  | <p>The maximum concurrent sessions of the Elastic fleet. This is required for Elastic
            fleets, and not allowed for other fleet types.</p> |
| `max_sessions_per_instance` | i64 |  | <p>The maximum number of user sessions on an instance. This only applies to multi-session fleets.</p> |
| `image_arn` | String |  | <p>The ARN of the public, private, or shared image to use.</p> |
| `instance_type` | String | ✅ | <p>The instance type to use when launching fleet instances. The following instance types are available:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.standard.xlarge</p>
            </li>
            <li>
               <p>stream.standard.2xlarge</p>
            </li>
            <li>
               <p>stream.compute.large</p>
            </li>
            <li>
               <p>stream.compute.xlarge</p>
            </li>
            <li>
               <p>stream.compute.2xlarge</p>
            </li>
            <li>
               <p>stream.compute.4xlarge</p>
            </li>
            <li>
               <p>stream.compute.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.large</p>
            </li>
            <li>
               <p>stream.memory.xlarge</p>
            </li>
            <li>
               <p>stream.memory.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.4xlarge</p>
            </li>
            <li>
               <p>stream.memory.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.large</p>
            </li>
            <li>
               <p>stream.memory.z1d.xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.3xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.6xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.large</p>
            </li>
            <li>
               <p>stream.graphics-design.xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-desktop.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.large</p>
            </li>
            <li>
               <p>stream.graphics.g6f.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6f.4xlarge</p>
            </li>
         </ul>
         <p>The following instance types are available for Elastic fleets:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.standard.xlarge</p>
            </li>
            <li>
               <p>stream.standard.2xlarge</p>
            </li>
         </ul> |
| `description` | String |  | <p>The description to display.</p> |
| `display_name` | String |  | <p>The fleet name to display.</p> |
| `name` | String | ✅ | <p>A unique name for the fleet.</p> |
| `enable_default_internet_access` | bool |  | <p>Enables or disables default internet access for the fleet.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to associate with the fleet. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p>
         <p>If you do not specify a value, the value is set to an empty string.</p>
         <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p>
         <p>_ . : / = + \ - @</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |
| `platform` | String |  | <p>The fleet platform. WINDOWS_SERVER_2019 and AMAZON_LINUX2 are supported for Elastic
            fleets. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fleet
fleet = provider.appstream.Fleet {
    instance_type = "value"  # <p>The instance type to use when launching fleet instances. The following instance types are available:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.standard.xlarge</p>
            </li>
            <li>
               <p>stream.standard.2xlarge</p>
            </li>
            <li>
               <p>stream.compute.large</p>
            </li>
            <li>
               <p>stream.compute.xlarge</p>
            </li>
            <li>
               <p>stream.compute.2xlarge</p>
            </li>
            <li>
               <p>stream.compute.4xlarge</p>
            </li>
            <li>
               <p>stream.compute.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.large</p>
            </li>
            <li>
               <p>stream.memory.xlarge</p>
            </li>
            <li>
               <p>stream.memory.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.4xlarge</p>
            </li>
            <li>
               <p>stream.memory.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.large</p>
            </li>
            <li>
               <p>stream.memory.z1d.xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.3xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.6xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.large</p>
            </li>
            <li>
               <p>stream.graphics-design.xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-desktop.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.large</p>
            </li>
            <li>
               <p>stream.graphics.g6f.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6f.4xlarge</p>
            </li>
         </ul>
         <p>The following instance types are available for Elastic fleets:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.standard.xlarge</p>
            </li>
            <li>
               <p>stream.standard.2xlarge</p>
            </li>
         </ul>
    name = "value"  # <p>A unique name for the fleet.</p>
}

```

---


### Image_builder_streaming_url

ImageBuilderStreamingURL resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the image builder.</p> |
| `validity` | i64 |  | <p>The time that the streaming URL will be valid, in seconds. 
            Specify a value between 1 and 604800 seconds. The default is 3600 seconds.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_builder_streaming_url
image_builder_streaming_url = provider.appstream.Image_builder_streaming_url {
    name = "value"  # <p>The name of the image builder.</p>
}

```

---


### Entitlement

Entitlement resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_visibility` | String | ✅ | <p>Specifies whether all or selected apps are entitled.</p> |
| `stack_name` | String | ✅ | <p>The name of the stack with which the entitlement is associated.</p> |
| `description` | String |  | <p>The description of the entitlement.</p> |
| `attributes` | Vec<String> | ✅ | <p>The attributes of the entitlement.</p> |
| `name` | String | ✅ | <p>The name of the entitlement.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create entitlement
entitlement = provider.appstream.Entitlement {
    app_visibility = "value"  # <p>Specifies whether all or selected apps are entitled.</p>
    stack_name = "value"  # <p>The name of the stack with which the entitlement is associated.</p>
    attributes = "value"  # <p>The attributes of the entitlement.</p>
    name = "value"  # <p>The name of the entitlement.</p>
}

```

---


### Stack

Stack resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_settings` | Vec<String> |  | <p>The actions that are enabled or disabled for users during their streaming sessions. By default, these actions are enabled. </p> |
| `display_name` | String |  | <p>The stack name to display.</p> |
| `feedback_url` | String |  | <p>The URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed.</p> |
| `name` | String | ✅ | <p>The name of the stack.</p> |
| `access_endpoints` | Vec<String> |  | <p>The list of interface VPC endpoint (interface endpoint) objects. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.</p> |
| `embed_host_domains` | Vec<String> |  | <p>The domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions. </p> |
| `redirect_url` | String |  | <p>The URL that users are redirected to after their streaming session ends.</p> |
| `storage_connectors` | Vec<String> |  | <p>The storage connectors to enable.</p> |
| `application_settings` | String |  | <p>The persistent application settings for users of a stack. When these settings are enabled, changes that users make to applications and Windows settings are automatically saved after each session and applied to the next session.</p> |
| `description` | String |  | <p>The description to display.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to associate with the stack. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p>
         <p>If you do not specify a value, the value is set to an empty string.</p>
         <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p>
         <p>_ . : / = + \ - @</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |
| `streaming_experience_settings` | String |  | <p>The streaming protocol you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stack
stack = provider.appstream.Stack {
    name = "value"  # <p>The name of the stack.</p>
}

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
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |
| `users` | Vec<String> | <p>Information about users in the user pool.</p> |


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
users_next_token = users.next_token
users_users = users.users
```

---


### Image

Image resource

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


### Software_associations

SoftwareAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `software_associations` | Vec<String> | <p>Collection of license included applications association details including:</p>
         <ul>
            <li>
               <p>License included application name and version information</p>
            </li>
            <li>
               <p>Deployment status (SoftwareDeploymentStatus enum)</p>
            </li>
            <li>
               <p>Error details for failed deployments</p>
            </li>
            <li>
               <p>Association timestamps</p>
            </li>
         </ul> |
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation.</p> |
| `associated_resource` | String | <p>The ARN of the resource to describe software associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access software_associations outputs
software_associations_id = software_associations.id
software_associations_software_associations = software_associations.software_associations
software_associations_next_token = software_associations.next_token
software_associations_associated_resource = software_associations.associated_resource
```

---


### Directory_config

DirectoryConfig resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `directory_name` | String | ✅ | <p>The fully qualified name of the directory (for example, corp.example.com).</p> |
| `service_account_credentials` | String |  | <p>The credentials for the service account used by the fleet or image builder to connect to the directory.</p> |
| `certificate_based_auth_properties` | String |  | <p>The certificate-based authentication properties used to authenticate SAML 2.0 Identity
            Provider (IdP) user identities to Active Directory domain-joined streaming instances.
            Fallback is turned on by default when certificate-based authentication is <b>Enabled</b> . Fallback allows users to log in using their AD
            domain password if certificate-based authentication is unsuccessful, or to unlock a
            desktop lock screen. <b>Enabled_no_directory_login_fallback</b> enables certificate-based
            authentication, but does not allow users to log in using their AD domain password. Users
            will be disconnected to re-authenticate using certificates.</p> |
| `organizational_unit_distinguished_names` | Vec<String> | ✅ | <p>The distinguished names of the organizational units for computer accounts.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create directory_config
directory_config = provider.appstream.Directory_config {
    directory_name = "value"  # <p>The fully qualified name of the directory (for example, corp.example.com).</p>
    organizational_unit_distinguished_names = "value"  # <p>The distinguished names of the organizational units for computer accounts.</p>
}

```

---


### Usage_report_subscriptions

UsageReportSubscriptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `usage_report_subscriptions` | Vec<String> | <p>Information about the usage report subscription.</p> |
| `next_token` | String | <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_report_subscriptions outputs
usage_report_subscriptions_id = usage_report_subscriptions.id
usage_report_subscriptions_usage_report_subscriptions = usage_report_subscriptions.usage_report_subscriptions
usage_report_subscriptions_next_token = usage_report_subscriptions.next_token
```

---


### App_block_builder

AppBlockBuilder resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_endpoints` | Vec<String> |  | <p>The list of interface VPC endpoint (interface endpoint) objects. Administrators can connect to the app block builder only through the specified endpoints.</p> |
| `display_name` | String |  | <p>The display name of the app block builder.</p> |
| `platform` | String | ✅ | <p>The platform of the app block builder.</p>
         <p>
            <code>WINDOWS_SERVER_2019</code> is the only valid value.</p> |
| `enable_default_internet_access` | bool |  | <p>Enables or disables default internet access for the app block builder.</p> |
| `vpc_config` | String | ✅ | <p>The VPC configuration for the app block builder.</p>
         <p>App block builders require that you specify at least two subnets in different availability
            zones.</p> |
| `description` | String |  | <p>The description of the app block builder.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to associate with the app block builder. A tag is a key-value pair, and the
            value is optional. For example, Environment=Test. If you do not specify a value,
            Environment=. </p>
         <p>If you do not specify a value, the value is set to an empty string.</p>
         <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p>
         <p>_ . : / = + \ - @</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |
| `instance_type` | String | ✅ | <p>The instance type to use when launching the app block builder. The following instance
            types are available:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.standard.xlarge</p>
            </li>
            <li>
               <p>stream.standard.2xlarge</p>
            </li>
         </ul> |
| `name` | String | ✅ | <p>The unique name for the app block builder.</p> |
| `iam_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role to apply to the app block builder. To
            assume a role, the app block builder calls the AWS Security Token Service (STS)
                <code>AssumeRole</code> API operation and passes the ARN of the role to use. The
            operation creates a new session with temporary credentials. AppStream 2.0 retrieves the
            temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_block_builder
app_block_builder = provider.appstream.App_block_builder {
    platform = "value"  # <p>The platform of the app block builder.</p>
         <p>
            <code>WINDOWS_SERVER_2019</code> is the only valid value.</p>
    vpc_config = "value"  # <p>The VPC configuration for the app block builder.</p>
         <p>App block builders require that you specify at least two subnets in different availability
            zones.</p>
    instance_type = "value"  # <p>The instance type to use when launching the app block builder. The following instance
            types are available:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.standard.xlarge</p>
            </li>
            <li>
               <p>stream.standard.2xlarge</p>
            </li>
         </ul>
    name = "value"  # <p>The unique name for the app block builder.</p>
}

```

---


### Image_builder

ImageBuilder resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_name` | String |  | <p>The name of the image used to create the image builder.</p> |
| `softwares_to_install` | String |  | <p>The list of license included applications to install on the image builder during creation.</p>
         <p>Possible values include the following:</p>
         <ul>
            <li>
               <p>Microsoft_Office_2021_LTSC_Professional_Plus_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2021_LTSC_Professional_Plus_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Professional_Plus_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Professional_Plus_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2021_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2021_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Standard_64Bit</p>
            </li>
         </ul> |
| `description` | String |  | <p>The description to display.</p> |
| `image_arn` | String |  | <p>The ARN of the public, private, or shared image to use.</p> |
| `display_name` | String |  | <p>The image builder name to display.</p> |
| `domain_join_info` | String |  | <p>The name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain. </p> |
| `appstream_agent_version` | String |  | <p>The version of the AppStream 2.0 agent to use for this image builder. To use the latest version of the AppStream 2.0 agent, specify [LATEST]. </p> |
| `enable_default_internet_access` | bool |  | <p>Enables or disables default internet access for the image builder.</p> |
| `instance_type` | String | ✅ | <p>The instance type to use when launching the image builder. The following instance types are available:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.compute.large</p>
            </li>
            <li>
               <p>stream.compute.xlarge</p>
            </li>
            <li>
               <p>stream.compute.2xlarge</p>
            </li>
            <li>
               <p>stream.compute.4xlarge</p>
            </li>
            <li>
               <p>stream.compute.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.large</p>
            </li>
            <li>
               <p>stream.memory.xlarge</p>
            </li>
            <li>
               <p>stream.memory.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.4xlarge</p>
            </li>
            <li>
               <p>stream.memory.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.large</p>
            </li>
            <li>
               <p>stream.memory.z1d.xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.3xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.6xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.large</p>
            </li>
            <li>
               <p>stream.graphics-design.xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-desktop.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.large</p>
            </li>
            <li>
               <p>stream.graphics.g6f.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6f.4xlarge</p>
            </li>
         </ul> |
| `access_endpoints` | Vec<String> |  | <p>The list of interface VPC endpoint (interface endpoint) objects. Administrators can connect to the image builder only through the specified endpoints.</p> |
| `softwares_to_uninstall` | String |  | <p>The list of license included applications to uninstall from the image builder during creation.</p>
         <p>Possible values include the following:</p>
         <ul>
            <li>
               <p>Microsoft_Office_2021_LTSC_Professional_Plus_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2021_LTSC_Professional_Plus_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Professional_Plus_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Professional_Plus_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Professional_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Professional_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2021_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2021_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Office_2024_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2021_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Visio_2024_LTSC_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2021_Standard_64Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Standard_32Bit</p>
            </li>
            <li>
               <p>Microsoft_Project_2024_Standard_64Bit</p>
            </li>
         </ul> |
| `vpc_config` | String |  | <p>The VPC configuration for the image builder. You can specify only one subnet.</p> |
| `name` | String | ✅ | <p>A unique name for the image builder.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to associate with the image builder. A tag is a key-value pair, and the value is optional. For example, Environment=Test. If you do not specify a value, Environment=. </p>
         <p>Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following special characters: </p>
         <p>_ . : / = + \ - @</p>
         <p>If you do not specify a value, the value is set to an empty string.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/tagging-basic.html">Tagging Your Resources</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |
| `iam_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role to apply to the image builder. To assume a role, the image builder calls the AWS Security Token Service (STS) <code>AssumeRole</code> API operation and passes the ARN of the role to use. The operation creates a new session with temporary credentials. AppStream 2.0 retrieves the temporary credentials and creates the <b>appstream_machine_role</b> credential profile on the instance.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/using-iam-roles-to-grant-permissions-to-applications-scripts-streaming-instances.html">Using an IAM Role to Grant Permissions to Applications and Scripts Running on AppStream 2.0 Streaming Instances</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_builder
image_builder = provider.appstream.Image_builder {
    instance_type = "value"  # <p>The instance type to use when launching the image builder. The following instance types are available:</p>
         <ul>
            <li>
               <p>stream.standard.small</p>
            </li>
            <li>
               <p>stream.standard.medium</p>
            </li>
            <li>
               <p>stream.standard.large</p>
            </li>
            <li>
               <p>stream.compute.large</p>
            </li>
            <li>
               <p>stream.compute.xlarge</p>
            </li>
            <li>
               <p>stream.compute.2xlarge</p>
            </li>
            <li>
               <p>stream.compute.4xlarge</p>
            </li>
            <li>
               <p>stream.compute.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.large</p>
            </li>
            <li>
               <p>stream.memory.xlarge</p>
            </li>
            <li>
               <p>stream.memory.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.4xlarge</p>
            </li>
            <li>
               <p>stream.memory.8xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.large</p>
            </li>
            <li>
               <p>stream.memory.z1d.xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.2xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.3xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.6xlarge</p>
            </li>
            <li>
               <p>stream.memory.z1d.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.large</p>
            </li>
            <li>
               <p>stream.graphics-design.xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics-design.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-desktop.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g4dn.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics-pro.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g5.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.16xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.12xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6.24xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6.8xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.large</p>
            </li>
            <li>
               <p>stream.graphics.g6f.xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.2xlarge</p>
            </li>
            <li>
               <p>stream.graphics.g6f.4xlarge</p>
            </li>
            <li>
               <p>stream.graphics.gr6f.4xlarge</p>
            </li>
         </ul>
    name = "value"  # <p>A unique name for the image builder.</p>
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

# Create multiple application resources
application_0 = provider.appstream.Application {
    icon_s3_location = "value-0"
    launch_path = "value-0"
    platforms = "value-0"
    instance_families = "value-0"
    name = "value-0"
    app_block_arn = "value-0"
}
application_1 = provider.appstream.Application {
    icon_s3_location = "value-1"
    launch_path = "value-1"
    platforms = "value-1"
    instance_families = "value-1"
    name = "value-1"
    app_block_arn = "value-1"
}
application_2 = provider.appstream.Application {
    icon_s3_location = "value-2"
    launch_path = "value-2"
    platforms = "value-2"
    instance_families = "value-2"
    name = "value-2"
    app_block_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    application = provider.appstream.Application {
        icon_s3_location = "production-value"
        launch_path = "production-value"
        platforms = "production-value"
        instance_families = "production-value"
        name = "production-value"
        app_block_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Appstream Documentation](https://docs.aws.amazon.com/appstream/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
