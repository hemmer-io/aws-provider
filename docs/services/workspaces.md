# Workspaces Service



**Resources**: 37

---

## Overview

The workspaces service provides access to 37 resource types:

- [Workspaces_pools](#workspaces_pools) [R]
- [Workspace_bundle](#workspace_bundle) [CUD]
- [Workspace_directories](#workspace_directories) [R]
- [Standby_workspaces](#standby_workspaces) [C]
- [Updated_workspace_image](#updated_workspace_image) [C]
- [Account](#account) [R]
- [Connection_aliases](#connection_aliases) [R]
- [Ip_group](#ip_group) [CD]
- [Workspace_image](#workspace_image) [CD]
- [Connect_client_add_in](#connect_client_add_in) [CUD]
- [Connection_alias](#connection_alias) [CD]
- [Applications](#applications) [R]
- [Connection_alias_permissions](#connection_alias_permissions) [R]
- [Workspace_image_permission](#workspace_image_permission) [U]
- [Workspace_images](#workspace_images) [R]
- [Connect_client_add_ins](#connect_client_add_ins) [R]
- [Workspace_bundles](#workspace_bundles) [R]
- [Workspaces](#workspaces) [CR]
- [Rules_of_ip_group](#rules_of_ip_group) [U]
- [Connection_alias_permission](#connection_alias_permission) [U]
- [Application_associations](#application_associations) [R]
- [Workspace_associations](#workspace_associations) [R]
- [Workspaces_pool_sessions](#workspaces_pool_sessions) [R]
- [Client_branding](#client_branding) [RD]
- [Image_associations](#image_associations) [R]
- [Workspaces_connection_status](#workspaces_connection_status) [R]
- [Account_link_invitation](#account_link_invitation) [CD]
- [Workspace_snapshots](#workspace_snapshots) [R]
- [Ip_groups](#ip_groups) [R]
- [Workspaces_pool](#workspaces_pool) [CU]
- [Account_modifications](#account_modifications) [R]
- [Bundle_associations](#bundle_associations) [R]
- [Custom_workspace_image_import](#custom_workspace_image_import) [R]
- [Workspace_image_permissions](#workspace_image_permissions) [R]
- [Tags](#tags) [CRD]
- [Client_properties](#client_properties) [R]
- [Account_link](#account_link) [R]

---

## Resources


### Workspaces_pools

WorkspacesPools resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workspaces_pools` | Vec<String> | <p>Information about the WorkSpaces Pools.</p> |
| `next_token` | String | <p>If you received a <code>NextToken</code> from a previous call that was paginated, 
         provide this token to receive the next set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspaces_pools outputs
workspaces_pools_id = workspaces_pools.id
workspaces_pools_workspaces_pools = workspaces_pools.workspaces_pools
workspaces_pools_next_token = workspaces_pools.next_token
```

---


### Workspace_bundle

WorkspaceBundle resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_id` | String | ✅ | <p>The identifier of the image that is used to create the bundle.</p> |
| `bundle_description` | String | ✅ | <p>The description of the bundle.</p> |
| `compute_type` | String | ✅ |  |
| `user_storage` | String | ✅ |  |
| `root_storage` | String |  |  |
| `tags` | Vec<String> |  | <p>The tags associated with the bundle.</p>
         <note>
            <p>To add tags at the same time when you're creating the bundle, you must create an IAM policy that 
            grants your IAM user permissions to use <code>workspaces:CreateTags</code>. </p>
         </note> |
| `bundle_name` | String | ✅ | <p>The name of the bundle.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workspace_bundle
workspace_bundle = provider.workspaces.Workspace_bundle {
    image_id = "value"  # <p>The identifier of the image that is used to create the bundle.</p>
    bundle_description = "value"  # <p>The description of the bundle.</p>
    compute_type = "value"  # Required field
    user_storage = "value"  # Required field
    bundle_name = "value"  # <p>The name of the bundle.</p>
}

```

---


### Workspace_directories

WorkspaceDirectories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |
| `directories` | Vec<String> | <p>Information about the directories.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspace_directories outputs
workspace_directories_id = workspace_directories.id
workspace_directories_next_token = workspace_directories.next_token
workspace_directories_directories = workspace_directories.directories
```

---


### Standby_workspaces

StandbyWorkspaces resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `primary_region` | String | ✅ | <p>The Region of the primary WorkSpace.</p> |
| `standby_workspaces` | Vec<String> | ✅ | <p>Information about the standby WorkSpace to be created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create standby_workspaces
standby_workspaces = provider.workspaces.Standby_workspaces {
    primary_region = "value"  # <p>The Region of the primary WorkSpace.</p>
    standby_workspaces = "value"  # <p>Information about the standby WorkSpace to be created.</p>
}

```

---


### Updated_workspace_image

UpdatedWorkspaceImage resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags that you want to add to the new updated WorkSpace image.</p>
         <note>
            <p>To add tags at the same time when you're creating the updated image, you must create
            an IAM policy that grants your IAM user permissions to use
               <code>workspaces:CreateTags</code>. </p>
         </note> |
| `name` | String | ✅ | <p>The name of the new updated WorkSpace image.</p> |
| `description` | String | ✅ | <p>A description of whether updates for the WorkSpace image are available.</p> |
| `source_image_id` | String | ✅ | <p>The identifier of the source WorkSpace image.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create updated_workspace_image
updated_workspace_image = provider.workspaces.Updated_workspace_image {
    name = "value"  # <p>The name of the new updated WorkSpace image.</p>
    description = "value"  # <p>A description of whether updates for the WorkSpace image are available.</p>
    source_image_id = "value"  # <p>The identifier of the source WorkSpace image.</p>
}

```

---


### Account

Account resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dedicated_tenancy_account_type` | String | <p>The type of linked account.</p> |
| `message` | String | <p>The text message to describe the status of BYOL.</p> |
| `dedicated_tenancy_support` | String | <p>The status of BYOL (whether BYOL is enabled or disabled).</p> |
| `dedicated_tenancy_management_cidr_range` | String | <p>The IP address range, specified as an IPv4 CIDR block, used for the management network
         interface.</p>
         <p>The management network interface is connected to a secure Amazon WorkSpaces management
         network. It is used for interactive streaming of the WorkSpace desktop to Amazon WorkSpaces
         clients, and to allow Amazon WorkSpaces to manage the WorkSpace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account outputs
account_id = account.id
account_dedicated_tenancy_account_type = account.dedicated_tenancy_account_type
account_message = account.message
account_dedicated_tenancy_support = account.dedicated_tenancy_support
account_dedicated_tenancy_management_cidr_range = account.dedicated_tenancy_management_cidr_range
```

---


### Connection_aliases

ConnectionAliases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection_aliases` | Vec<String> | <p>Information about the specified connection aliases.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection_aliases outputs
connection_aliases_id = connection_aliases.id
connection_aliases_connection_aliases = connection_aliases.connection_aliases
connection_aliases_next_token = connection_aliases.next_token
```

---


### Ip_group

IpGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_desc` | String |  | <p>The description of the group.</p> |
| `group_name` | String | ✅ | <p>The name of the group.</p> |
| `tags` | Vec<String> |  | <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p> |
| `user_rules` | Vec<String> |  | <p>The rules to add to the group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ip_group
ip_group = provider.workspaces.Ip_group {
    group_name = "value"  # <p>The name of the group.</p>
}

```

---


### Workspace_image

WorkspaceImage resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the new WorkSpace image.</p> |
| `tags` | Vec<String> |  | <p>The tags that you want to add to the new WorkSpace image. 
         To add tags when you're creating the image, you must create an IAM policy that grants 
         your IAM user permission to use <code>workspaces:CreateTags</code>.</p> |
| `description` | String | ✅ | <p>The description of the new WorkSpace image.</p> |
| `workspace_id` | String | ✅ | <p>The identifier of the source WorkSpace</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workspace_image
workspace_image = provider.workspaces.Workspace_image {
    name = "value"  # <p>The name of the new WorkSpace image.</p>
    description = "value"  # <p>The description of the new WorkSpace image.</p>
    workspace_id = "value"  # <p>The identifier of the source WorkSpace</p>
}

```

---


### Connect_client_add_in

ConnectClientAddIn resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_id` | String | ✅ | <p>The directory identifier for which to configure the client add-in.</p> |
| `name` | String | ✅ | <p>The name of the client add-in.</p> |
| `url` | String | ✅ | <p>The endpoint URL of the Amazon Connect client add-in.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connect_client_add_in
connect_client_add_in = provider.workspaces.Connect_client_add_in {
    resource_id = "value"  # <p>The directory identifier for which to configure the client add-in.</p>
    name = "value"  # <p>The name of the client add-in.</p>
    url = "value"  # <p>The endpoint URL of the Amazon Connect client add-in.</p>
}

```

---


### Connection_alias

ConnectionAlias resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_string` | String | ✅ | <p>A connection string in the form of a fully qualified domain name (FQDN), such as
            <code>www.example.com</code>.</p>
         <important>
            <p>After you create a connection string, it is always associated to your Amazon Web Services account. You cannot recreate the same connection string with a different
            account, even if you delete all instances of it from the original account. The
            connection string is globally reserved for your account.</p>
         </important> |
| `tags` | Vec<String> |  | <p>The tags to associate with the connection alias.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection_alias
connection_alias = provider.workspaces.Connection_alias {
    connection_string = "value"  # <p>A connection string in the form of a fully qualified domain name (FQDN), such as
            <code>www.example.com</code>.</p>
         <important>
            <p>After you create a connection string, it is always associated to your Amazon Web Services account. You cannot recreate the same connection string with a different
            account, even if you delete all instances of it from the original account. The
            connection string is globally reserved for your account.</p>
         </important>
}

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
| `applications` | Vec<String> | <p>List of information about the specified applications.</p> |
| `next_token` | String | <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p> |


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


### Connection_alias_permissions

ConnectionAliasPermissions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection_alias_permissions` | Vec<String> | <p>The permissions associated with a connection alias.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |
| `alias_id` | String | <p>The identifier of the connection alias.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection_alias_permissions outputs
connection_alias_permissions_id = connection_alias_permissions.id
connection_alias_permissions_connection_alias_permissions = connection_alias_permissions.connection_alias_permissions
connection_alias_permissions_next_token = connection_alias_permissions.next_token
connection_alias_permissions_alias_id = connection_alias_permissions.alias_id
```

---


### Workspace_image_permission

WorkspaceImagePermission resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shared_account_id` | String | ✅ | <p>The identifier of the Amazon Web Services account to share or unshare the image
         with.</p>
         <important>
            <p>Before sharing the image, confirm that you are sharing to the correct Amazon Web Services account ID.</p>
         </important> |
| `image_id` | String | ✅ | <p>The identifier of the image.</p> |
| `allow_copy_image` | bool | ✅ | <p>The permission to copy the image. This permission can be revoked only after an image has
         been shared.</p> |



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


### Workspace_images

WorkspaceImages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `images` | Vec<String> | <p>Information about the images.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspace_images outputs
workspace_images_id = workspace_images.id
workspace_images_images = workspace_images.images
workspace_images_next_token = workspace_images.next_token
```

---


### Connect_client_add_ins

ConnectClientAddIns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `add_ins` | Vec<String> | <p>Information about client add-ins.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connect_client_add_ins outputs
connect_client_add_ins_id = connect_client_add_ins.id
connect_client_add_ins_add_ins = connect_client_add_ins.add_ins
connect_client_add_ins_next_token = connect_client_add_ins.next_token
```

---


### Workspace_bundles

WorkspaceBundles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are no more 
         results to return. This token is valid for one day and must be used within that time
         frame.</p> |
| `bundles` | Vec<String> | <p>Information about the bundles.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspace_bundles outputs
workspace_bundles_id = workspace_bundles.id
workspace_bundles_next_token = workspace_bundles.next_token
workspace_bundles_bundles = workspace_bundles.bundles
```

---


### Workspaces

Workspaces resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workspaces` | Vec<String> | ✅ | <p>The WorkSpaces to create. You can specify up to 25 WorkSpaces.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |
| `workspaces` | Vec<String> | <p>Information about the WorkSpaces.</p>
         <p>Because <a>CreateWorkspaces</a> is an asynchronous operation, some of the
         returned information could be incomplete.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workspaces
workspaces = provider.workspaces.Workspaces {
    workspaces = "value"  # <p>The WorkSpaces to create. You can specify up to 25 WorkSpaces.</p>
}

# Access workspaces outputs
workspaces_id = workspaces.id
workspaces_next_token = workspaces.next_token
workspaces_workspaces = workspaces.workspaces
```

---


### Rules_of_ip_group

RulesOfIpGroup resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_id` | String | ✅ | <p>The identifier of the group.</p> |
| `user_rules` | Vec<String> | ✅ | <p>One or more rules.</p> |



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


### Connection_alias_permission

ConnectionAliasPermission resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alias_id` | String | ✅ | <p>The identifier of the connection alias that you want to update permissions for.</p> |
| `connection_alias_permission` | String | ✅ | <p>Indicates whether to share or unshare the connection alias with the specified Amazon Web Services account.</p> |



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


### Application_associations

ApplicationAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>List of associations and information about them.</p> |
| `next_token` | String | <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_associations outputs
application_associations_id = application_associations.id
application_associations_associations = application_associations.associations
application_associations_next_token = application_associations.next_token
```

---


### Workspace_associations

WorkspaceAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>List of information about the specified associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspace_associations outputs
workspace_associations_id = workspace_associations.id
workspace_associations_associations = workspace_associations.associations
```

---


### Workspaces_pool_sessions

WorkspacesPoolSessions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If you received a <code>NextToken</code> from a previous call that was paginated, 
         provide this token to receive the next set of results.</p> |
| `sessions` | Vec<String> | <p>Describes the pool sessions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspaces_pool_sessions outputs
workspaces_pool_sessions_id = workspaces_pool_sessions.id
workspaces_pool_sessions_next_token = workspaces_pool_sessions.next_token
workspaces_pool_sessions_sessions = workspaces_pool_sessions.sessions
```

---


### Client_branding

ClientBranding resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_type_osx` | String | <p>The branding information for macOS devices.</p> |
| `device_type_android` | String | <p>The branding information for Android devices.</p> |
| `device_type_ios` | String | <p>The branding information for iOS devices.</p> |
| `device_type_linux` | String | <p>The branding information for Linux devices.</p> |
| `device_type_web` | String | <p>The branding information for Web access.</p> |
| `device_type_windows` | String | <p>The branding information for Windows devices.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_branding outputs
client_branding_id = client_branding.id
client_branding_device_type_osx = client_branding.device_type_osx
client_branding_device_type_android = client_branding.device_type_android
client_branding_device_type_ios = client_branding.device_type_ios
client_branding_device_type_linux = client_branding.device_type_linux
client_branding_device_type_web = client_branding.device_type_web
client_branding_device_type_windows = client_branding.device_type_windows
```

---


### Image_associations

ImageAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>List of information about the specified associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_associations outputs
image_associations_id = image_associations.id
image_associations_associations = image_associations.associations
```

---


### Workspaces_connection_status

WorkspacesConnectionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |
| `workspaces_connection_status` | Vec<String> | <p>Information about the connection status of the WorkSpace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspaces_connection_status outputs
workspaces_connection_status_id = workspaces_connection_status.id
workspaces_connection_status_next_token = workspaces_connection_status.next_token
workspaces_connection_status_workspaces_connection_status = workspaces_connection_status.workspaces_connection_status
```

---


### Account_link_invitation

AccountLinkInvitation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_account_id` | String | ✅ | <p>The identifier of the target account.</p> |
| `client_token` | String |  | <p>A string of up to 64 ASCII characters that Amazon WorkSpaces uses to ensure idempotent creation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_link_invitation
account_link_invitation = provider.workspaces.Account_link_invitation {
    target_account_id = "value"  # <p>The identifier of the target account.</p>
}

```

---


### Workspace_snapshots

WorkspaceSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rebuild_snapshots` | Vec<String> | <p>Information about the snapshots that can be used to rebuild a WorkSpace. These snapshots
         include the user volume.</p> |
| `restore_snapshots` | Vec<String> | <p>Information about the snapshots that can be used to restore a WorkSpace. These snapshots
         include both the root volume and the user volume.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspace_snapshots outputs
workspace_snapshots_id = workspace_snapshots.id
workspace_snapshots_rebuild_snapshots = workspace_snapshots.rebuild_snapshots
workspace_snapshots_restore_snapshots = workspace_snapshots.restore_snapshots
```

---


### Ip_groups

IpGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `result` | Vec<String> | <p>Information about the IP access control groups.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ip_groups outputs
ip_groups_id = ip_groups.id
ip_groups_result = ip_groups.result
ip_groups_next_token = ip_groups.next_token
```

---


### Workspaces_pool

WorkspacesPool resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pool_name` | String | ✅ | <p>The name of the pool.</p> |
| `tags` | Vec<String> |  | <p>The tags for the pool.</p> |
| `bundle_id` | String | ✅ | <p>The identifier of the bundle for the pool.</p> |
| `application_settings` | String |  | <p>Indicates the application settings of the pool.</p> |
| `directory_id` | String | ✅ | <p>The identifier of the directory for the pool.</p> |
| `timeout_settings` | String |  | <p>Indicates the timeout settings of the pool.</p> |
| `capacity` | String | ✅ | <p>The user capacity of the pool.</p> |
| `description` | String | ✅ | <p>The pool description.</p> |
| `running_mode` | String |  | <p>The running mode for the pool.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workspaces_pool
workspaces_pool = provider.workspaces.Workspaces_pool {
    pool_name = "value"  # <p>The name of the pool.</p>
    bundle_id = "value"  # <p>The identifier of the bundle for the pool.</p>
    directory_id = "value"  # <p>The identifier of the directory for the pool.</p>
    capacity = "value"  # <p>The user capacity of the pool.</p>
    description = "value"  # <p>The pool description.</p>
}

```

---


### Account_modifications

AccountModifications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_modifications` | Vec<String> | <p>The list of modifications to the configuration of BYOL.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_modifications outputs
account_modifications_id = account_modifications.id
account_modifications_account_modifications = account_modifications.account_modifications
account_modifications_next_token = account_modifications.next_token
```

---


### Bundle_associations

BundleAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>List of information about the specified associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bundle_associations outputs
bundle_associations_id = bundle_associations.id
bundle_associations_associations = bundle_associations.associations
```

---


### Custom_workspace_image_import

CustomWorkspaceImageImport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `infrastructure_configuration_arn` | String | <p>The infrastructure configuration ARN that specifies how the WorkSpace image is built.</p> |
| `error_details` | Vec<String> | <p>Describes in-depth details about the error. These details include the
         possible causes of the error and troubleshooting information.</p> |
| `image_source` | String | <p>Describes the image import source.</p> |
| `state` | String | <p>The state of the WorkSpace image.</p> |
| `created` | String | <p>The timestamp when the WorkSpace image import was created.</p> |
| `image_builder_instance_id` | String | <p>The image builder instance ID of the WorkSpace image.</p> |
| `last_updated_time` | String | <p>The timestamp when the WorkSpace image import was last updated.</p> |
| `image_id` | String | <p>The identifier of the WorkSpace image.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_workspace_image_import outputs
custom_workspace_image_import_id = custom_workspace_image_import.id
custom_workspace_image_import_infrastructure_configuration_arn = custom_workspace_image_import.infrastructure_configuration_arn
custom_workspace_image_import_error_details = custom_workspace_image_import.error_details
custom_workspace_image_import_image_source = custom_workspace_image_import.image_source
custom_workspace_image_import_state = custom_workspace_image_import.state
custom_workspace_image_import_created = custom_workspace_image_import.created
custom_workspace_image_import_image_builder_instance_id = custom_workspace_image_import.image_builder_instance_id
custom_workspace_image_import_last_updated_time = custom_workspace_image_import.last_updated_time
custom_workspace_image_import_image_id = custom_workspace_image_import.image_id
```

---


### Workspace_image_permissions

WorkspaceImagePermissions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_id` | String | <p>The identifier of the image.</p> |
| `image_permissions` | Vec<String> | <p>The identifiers of the Amazon Web Services accounts that the image has been shared
         with.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are
         no more results to return. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workspace_image_permissions outputs
workspace_image_permissions_id = workspace_image_permissions.id
workspace_image_permissions_image_id = workspace_image_permissions.image_id
workspace_image_permissions_image_permissions = workspace_image_permissions.image_permissions
workspace_image_permissions_next_token = workspace_image_permissions.next_token
```

---


### Tags

Tags resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> | ✅ | <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p> |
| `resource_id` | String | ✅ | <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces,
         registered directories, images, custom bundles, IP access control groups, and connection
         aliases.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_list` | Vec<String> | <p>The tags.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tags
tags = provider.workspaces.Tags {
    tags = "value"  # <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p>
    resource_id = "value"  # <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces,
         registered directories, images, custom bundles, IP access control groups, and connection
         aliases.</p>
}

# Access tags outputs
tags_id = tags.id
tags_tag_list = tags.tag_list
```

---


### Client_properties

ClientProperties resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_properties_list` | Vec<String> | <p>Information about the specified Amazon WorkSpaces clients.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_properties outputs
client_properties_id = client_properties.id
client_properties_client_properties_list = client_properties.client_properties_list
```

---


### Account_link

AccountLink resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_link` | String | <p>The account link of the account link to retrieve.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_link outputs
account_link_id = account_link.id
account_link_account_link = account_link.account_link
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple workspaces_pools resources
workspaces_pools_0 = provider.workspaces.Workspaces_pools {
}
workspaces_pools_1 = provider.workspaces.Workspaces_pools {
}
workspaces_pools_2 = provider.workspaces.Workspaces_pools {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    workspaces_pools = provider.workspaces.Workspaces_pools {
    }
```

---

## Related Documentation

- [AWS Workspaces Documentation](https://docs.aws.amazon.com/workspaces/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
