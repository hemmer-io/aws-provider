# Finspace_data Service



**Resources**: 8

---

## Overview

The finspace_data service provides access to 8 resource types:

- [Working_location](#working_location) [R]
- [Dataset](#dataset) [CRUD]
- [Permission_group](#permission_group) [CRUD]
- [Data_view](#data_view) [CR]
- [External_data_view_access_details](#external_data_view_access_details) [R]
- [User](#user) [CRU]
- [Changeset](#changeset) [CRU]
- [Programmatic_access_credentials](#programmatic_access_credentials) [R]

---

## Resources


### Working_location

WorkingLocation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `s3_uri` | String | <p>Returns the Amazon S3 URI for the working location.</p> |
| `s3_path` | String | <p>Returns the Amazon S3 Path for the working location.</p> |
| `s3_bucket` | String | <p>Returns the Amazon S3 bucket name for the working location.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access working_location outputs
working_location_id = working_location.id
working_location_s3_uri = working_location.s3_uri
working_location_s3_path = working_location.s3_path
working_location_s3_bucket = working_location.s3_bucket
```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset_title` | String | ✅ | <p>Display title for a FinSpace Dataset.</p> |
| `owner_info` | String |  | <p>Contact information for a Dataset owner.</p> |
| `kind` | String | ✅ | <p>The format in which Dataset data is structured.</p>
         <ul>
            <li>
               <p>
                  <code>TABULAR</code> – Data is structured in a tabular format.</p>
            </li>
            <li>
               <p>
                  <code>NON_TABULAR</code> – Data is structured in a non-tabular format.</p>
            </li>
         </ul> |
| `alias` | String |  | <p>The unique resource identifier for a Dataset.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `permission_group_params` | String | ✅ | <p>Permission group parameters for Dataset permissions.</p> |
| `schema_definition` | String |  | <p>Definition for a schema on a tabular Dataset.</p> |
| `dataset_description` | String |  | <p>Description of a Dataset.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_description` | String | <p>A description of the Dataset.</p> |
| `last_modified_time` | String | <p>The last time that the Dataset was modified. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `alias` | String | <p>The unique resource identifier for a Dataset.</p> |
| `dataset_arn` | String | <p>The ARN identifier of the Dataset.</p> |
| `create_time` | String | <p>The timestamp at which the Dataset was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `dataset_id` | String | <p>The unique identifier for a Dataset.</p> |
| `kind` | String | <p>The format in which Dataset data is structured.</p>
         <ul>
            <li>
               <p>
                  <code>TABULAR</code> – Data is structured in a tabular format.</p>
            </li>
            <li>
               <p>
                  <code>NON_TABULAR</code> – Data is structured in a non-tabular format.</p>
            </li>
         </ul> |
| `dataset_title` | String | <p>Display title for a Dataset.</p> |
| `schema_definition` | String | <p>Definition for a schema on a tabular Dataset.</p> |
| `status` | String | <p>Status of the Dataset creation.</p>
         <ul>
            <li>
               <p>
                  <code>PENDING</code> – Dataset is pending creation.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> – Dataset creation has failed.</p>
            </li>
            <li>
               <p>
                  <code>SUCCESS</code> – Dataset creation has succeeded.</p>
            </li>
            <li>
               <p>
                  <code>RUNNING</code> – Dataset creation is running.</p>
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

# Create dataset
dataset = provider.finspace_data.Dataset {
    dataset_title = "value"  # <p>Display title for a FinSpace Dataset.</p>
    kind = "value"  # <p>The format in which Dataset data is structured.</p>
         <ul>
            <li>
               <p>
                  <code>TABULAR</code> – Data is structured in a tabular format.</p>
            </li>
            <li>
               <p>
                  <code>NON_TABULAR</code> – Data is structured in a non-tabular format.</p>
            </li>
         </ul>
    permission_group_params = "value"  # <p>Permission group parameters for Dataset permissions.</p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset_description = dataset.dataset_description
dataset_last_modified_time = dataset.last_modified_time
dataset_alias = dataset.alias
dataset_dataset_arn = dataset.dataset_arn
dataset_create_time = dataset.create_time
dataset_dataset_id = dataset.dataset_id
dataset_kind = dataset.kind
dataset_dataset_title = dataset.dataset_title
dataset_schema_definition = dataset.schema_definition
dataset_status = dataset.status
```

---


### Permission_group

PermissionGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `application_permissions` | Vec<String> | ✅ | <p>The option to indicate FinSpace application permissions that are granted to a specific group.</p>
         <important>
            <p>When assigning application permissions, be aware that the permission <code>ManageUsersAndGroups</code> allows users to grant themselves or others access to any functionality in their FinSpace environment's application. It should only be granted to trusted users.</p>
         </important>
         <ul>
            <li>
               <p>
                  <code>CreateDataset</code> – Group members can create new datasets.</p>
            </li>
            <li>
               <p>
                  <code>ManageClusters</code> – Group members can manage Apache Spark clusters from FinSpace notebooks.</p>
            </li>
            <li>
               <p>
                  <code>ManageUsersAndGroups</code> – Group members can manage users and permission groups. This is a privileged permission that allows users to grant themselves or others access to any functionality in the application. It should only be granted to trusted users.</p>
            </li>
            <li>
               <p>
                  <code>ManageAttributeSets</code> – Group members can manage attribute sets.</p>
            </li>
            <li>
               <p>
                  <code>ViewAuditData</code> – Group members can view audit data.</p>
            </li>
            <li>
               <p>
                  <code>AccessNotebooks</code> – Group members will have access to FinSpace notebooks.</p>
            </li>
            <li>
               <p>
                  <code>GetTemporaryCredentials</code> – Group members can get temporary API credentials.</p>
            </li>
         </ul> |
| `description` | String |  | <p>A brief description for the permission group.</p> |
| `name` | String | ✅ | <p>The name of the permission group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permission_group` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission_group
permission_group = provider.finspace_data.Permission_group {
    application_permissions = "value"  # <p>The option to indicate FinSpace application permissions that are granted to a specific group.</p>
         <important>
            <p>When assigning application permissions, be aware that the permission <code>ManageUsersAndGroups</code> allows users to grant themselves or others access to any functionality in their FinSpace environment's application. It should only be granted to trusted users.</p>
         </important>
         <ul>
            <li>
               <p>
                  <code>CreateDataset</code> – Group members can create new datasets.</p>
            </li>
            <li>
               <p>
                  <code>ManageClusters</code> – Group members can manage Apache Spark clusters from FinSpace notebooks.</p>
            </li>
            <li>
               <p>
                  <code>ManageUsersAndGroups</code> – Group members can manage users and permission groups. This is a privileged permission that allows users to grant themselves or others access to any functionality in the application. It should only be granted to trusted users.</p>
            </li>
            <li>
               <p>
                  <code>ManageAttributeSets</code> – Group members can manage attribute sets.</p>
            </li>
            <li>
               <p>
                  <code>ViewAuditData</code> – Group members can view audit data.</p>
            </li>
            <li>
               <p>
                  <code>AccessNotebooks</code> – Group members will have access to FinSpace notebooks.</p>
            </li>
            <li>
               <p>
                  <code>GetTemporaryCredentials</code> – Group members can get temporary API credentials.</p>
            </li>
         </ul>
    name = "value"  # <p>The name of the permission group.</p>
}

# Access permission_group outputs
permission_group_id = permission_group.id
permission_group_permission_group = permission_group.permission_group
```

---


### Data_view

DataView resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partition_columns` | Vec<String> |  | <p>Ordered set of column names used to partition data.</p> |
| `sort_columns` | Vec<String> |  | <p>Columns to be used for sorting the data.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `dataset_id` | String | ✅ | <p>The unique Dataset identifier that is used to create a Dataview.</p> |
| `as_of_timestamp` | String |  | <p>Beginning time to use for the Dataview. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `destination_type_params` | String | ✅ | <p>Options that define the destination type for the Dataview.</p> |
| `auto_update` | bool |  | <p>Flag to indicate Dataview should be updated automatically.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>The last time that a Dataview was modified. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `sort_columns` | Vec<String> | <p>Columns to be used for sorting the data.</p> |
| `create_time` | String | <p>The timestamp at which the Dataview was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `status` | String | <p>The status of a Dataview creation.</p>
         <ul>
            <li>
               <p>
                  <code>RUNNING</code> – Dataview creation is running.</p>
            </li>
            <li>
               <p>
                  <code>STARTING</code> – Dataview creation is starting.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> – Dataview creation has failed.</p>
            </li>
            <li>
               <p>
                  <code>CANCELLED</code> – Dataview creation has been cancelled.</p>
            </li>
            <li>
               <p>
                  <code>TIMEOUT</code> – Dataview creation has timed out.</p>
            </li>
            <li>
               <p>
                  <code>SUCCESS</code> – Dataview creation has succeeded.</p>
            </li>
            <li>
               <p>
                  <code>PENDING</code> – Dataview creation is pending.</p>
            </li>
            <li>
               <p>
                  <code>FAILED_CLEANUP_FAILED</code> – Dataview creation failed and resource cleanup failed.</p>
            </li>
         </ul> |
| `error_info` | String | <p>Information about an error that occurred for the Dataview.</p> |
| `auto_update` | bool | <p>Flag to indicate Dataview should be updated automatically.</p> |
| `partition_columns` | Vec<String> | <p>Ordered set of column names used to partition data.</p> |
| `dataset_id` | String | <p>The unique identifier for the Dataset used in the Dataview.</p> |
| `as_of_timestamp` | String | <p>Time range to use for the Dataview. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `data_view_id` | String | <p>The unique identifier for the Dataview.</p> |
| `data_view_arn` | String | <p>The ARN identifier of the Dataview.</p> |
| `destination_type_params` | String | <p>Options that define the destination type for the Dataview.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_view
data_view = provider.finspace_data.Data_view {
    dataset_id = "value"  # <p>The unique Dataset identifier that is used to create a Dataview.</p>
    destination_type_params = "value"  # <p>Options that define the destination type for the Dataview.</p>
}

# Access data_view outputs
data_view_id = data_view.id
data_view_last_modified_time = data_view.last_modified_time
data_view_sort_columns = data_view.sort_columns
data_view_create_time = data_view.create_time
data_view_status = data_view.status
data_view_error_info = data_view.error_info
data_view_auto_update = data_view.auto_update
data_view_partition_columns = data_view.partition_columns
data_view_dataset_id = data_view.dataset_id
data_view_as_of_timestamp = data_view.as_of_timestamp
data_view_data_view_id = data_view.data_view_id
data_view_data_view_arn = data_view.data_view_arn
data_view_destination_type_params = data_view.destination_type_params
```

---


### External_data_view_access_details

ExternalDataViewAccessDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credentials` | String | <p>The credentials required to access the external Dataview from the S3 location.</p> |
| `s3_location` | String | <p>The location where the external Dataview is stored.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access external_data_view_access_details outputs
external_data_view_access_details_id = external_data_view_access_details.id
external_data_view_access_details_credentials = external_data_view_access_details.credentials
external_data_view_access_details_s3_location = external_data_view_access_details.s3_location
```

---


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `first_name` | String |  | <p>The first name of the user that you want to register.</p> |
| `last_name` | String |  | <p>The last name of the user that you want to register.</p> |
| `api_access_principal_arn` | String |  | <p>The ARN identifier of an AWS user or role that is allowed to call the <code>GetProgrammaticAccessCredentials</code> API to obtain a credentials token for a specific FinSpace user. This must be an IAM role within your FinSpace account.</p> |
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `email_address` | String | ✅ | <p>The email address of the user that you want to register. The email address serves as a uniquer identifier for each user and cannot be changed after it's created.</p> |
| `type` | String | ✅ | <p>The option to indicate the type of user. Use one of the following options to specify this parameter:</p>
         <ul>
            <li>
               <p>
                  <code>SUPER_USER</code> – A user with permission to all the functionality and data in FinSpace.</p>
            </li>
            <li>
               <p>
                  <code>APP_USER</code> – A user with specific permissions in FinSpace. The users are assigned permissions by adding them to a permission group.</p>
            </li>
         </ul> |
| `api_access` | String |  | <p>The option to indicate whether the user can use the <code>GetProgrammaticAccessCredentials</code> API to obtain credentials that can then be used to access other FinSpace Data API operations.</p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> – The user has permissions to use the APIs.</p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> – The user does not have permissions to use any APIs.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>Describes the last time the user details were updated. The value is determined as epoch time in milliseconds.</p> |
| `user_id` | String | <p>The unique identifier for the user that is retrieved.</p> |
| `last_name` | String | <p>The last name of the user.</p> |
| `first_name` | String | <p>The first name of the user.</p> |
| `status` | String | <p>The current status of the user. </p>
         <ul>
            <li>
               <p>
                  <code>CREATING</code> – The creation is in progress.</p>
            </li>
            <li>
               <p>
                  <code>ENABLED</code> – The user is created and is currently active.</p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> – The user is currently inactive.</p>
            </li>
         </ul> |
| `email_address` | String | <p>The email address that is associated with the user.</p> |
| `api_access` | String | <p>Indicates whether the user can use the <code>GetProgrammaticAccessCredentials</code> API to obtain credentials that can then be used to access other FinSpace Data API operations. </p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> – The user has permissions to use the APIs.</p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> – The user does not have permissions to use any APIs.</p>
            </li>
         </ul> |
| `api_access_principal_arn` | String | <p>The ARN identifier of an AWS user or role that is allowed to call the <code>GetProgrammaticAccessCredentials</code> API to obtain a credentials token for a specific FinSpace user. This must be an IAM role within your FinSpace account.</p> |
| `create_time` | String | <p>The timestamp at which the user was created in FinSpace. The value is determined as epoch time in milliseconds. </p> |
| `last_enabled_time` | String | <p>Describes the last time the user was activated. The value is determined as epoch time in milliseconds.</p> |
| `last_disabled_time` | String | <p>Describes the last time the user was deactivated. The value is determined as epoch time in milliseconds.</p> |
| `last_login_time` | String | <p>Describes the last time that the user logged into their account. The value is determined as epoch time in milliseconds.</p> |
| `type` | String | <p>Indicates the type of user.  </p>
         <ul>
            <li>
               <p>
                  <code>SUPER_USER</code> – A user with permission to all the functionality and data in FinSpace.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>
                  <code>APP_USER</code> – A user with specific permissions in FinSpace. The users are assigned permissions by adding them to a permission group.</p>
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

# Create user
user = provider.finspace_data.User {
    email_address = "value"  # <p>The email address of the user that you want to register. The email address serves as a uniquer identifier for each user and cannot be changed after it's created.</p>
    type = "value"  # <p>The option to indicate the type of user. Use one of the following options to specify this parameter:</p>
         <ul>
            <li>
               <p>
                  <code>SUPER_USER</code> – A user with permission to all the functionality and data in FinSpace.</p>
            </li>
            <li>
               <p>
                  <code>APP_USER</code> – A user with specific permissions in FinSpace. The users are assigned permissions by adding them to a permission group.</p>
            </li>
         </ul>
}

# Access user outputs
user_id = user.id
user_last_modified_time = user.last_modified_time
user_user_id = user.user_id
user_last_name = user.last_name
user_first_name = user.first_name
user_status = user.status
user_email_address = user.email_address
user_api_access = user.api_access
user_api_access_principal_arn = user.api_access_principal_arn
user_create_time = user.create_time
user_last_enabled_time = user.last_enabled_time
user_last_disabled_time = user.last_disabled_time
user_last_login_time = user.last_login_time
user_type = user.type
```

---


### Changeset

Changeset resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A token that ensures idempotency. This token expires in 10 minutes.</p> |
| `change_type` | String | ✅ | <p>The option to indicate how a Changeset will be applied to a Dataset.</p>
         <ul>
            <li>
               <p>
                  <code>REPLACE</code> – Changeset will be considered as a replacement to all prior
          loaded Changesets.</p>
            </li>
            <li>
               <p>
                  <code>APPEND</code> – Changeset will be considered as an addition to the end of all
          prior loaded Changesets.</p>
            </li>
            <li>
               <p>
                  <code>MODIFY</code> – Changeset is considered as a replacement to a specific prior ingested Changeset.</p>
            </li>
         </ul> |
| `source_params` | HashMap<String, String> | ✅ | <p>Options that define the location of the data being ingested (<code>s3SourcePath</code>) and the source of the changeset (<code>sourceType</code>).</p>
         <p>Both <code>s3SourcePath</code> and <code>sourceType</code> are required attributes.</p>
         <p>Here is an example of how you could specify the <code>sourceParams</code>:</p>
         <p>
            <code>
        "sourceParams": 
        {
        "s3SourcePath": "s3://finspace-landing-us-east-2-bk7gcfvitndqa6ebnvys4d/scratch/wr5hh8pwkpqqkxa4sxrmcw/ingestion/equity.csv",
        "sourceType": "S3"
        }
      </code>
         </p>
         <p>The S3 path that you specify must allow the FinSpace role access. To do that, you first need to configure the IAM policy on S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/finspace/latest/data-api/fs-using-the-finspace-api.html#access-s3-buckets">Loading data from an Amazon S3 Bucket using the FinSpace API</a> section.</p> |
| `format_params` | HashMap<String, String> | ✅ | <p>Options that define the structure of the source file(s) including the format type (<code>formatType</code>), header row (<code>withHeader</code>), data separation character (<code>separator</code>) and the type of compression (<code>compression</code>).
    </p>
         <p>
            <code>formatType</code> is a required attribute and can have the following values:
    </p>
         <ul>
            <li>
               <p>
                  <code>PARQUET</code> – Parquet source file format.</p>
            </li>
            <li>
               <p>
                  <code>CSV</code> – CSV source file format.</p>
            </li>
            <li>
               <p>
                  <code>JSON</code> – JSON source file format.</p>
            </li>
            <li>
               <p>
                  <code>XML</code> – XML source file format.</p>
            </li>
         </ul>
         <p>Here is an example of how you could specify the <code>formatParams</code>:</p>
         <p>
            <code>
          "formatParams": 
        {
         "formatType": "CSV",
         "withHeader": "true",
         "separator": ",",
         "compression":"None"
         } 
      </code>
         </p>
         <p>Note that if you only provide <code>formatType</code> as <code>CSV</code>, the rest of the attributes will automatically default to CSV values as following:</p>
         <p>
            <code>
          {
          "withHeader": "true",
          "separator": ","
           }
        </code>
         </p>
         <p> For more information about supported file formats, see <a href="https://docs.aws.amazon.com/finspace/latest/userguide/supported-data-types.html">Supported Data Types and File Formats</a> in the FinSpace User Guide.</p> |
| `dataset_id` | String | ✅ | <p>The unique identifier for the FinSpace Dataset where the Changeset will be created.
    </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active_from_timestamp` | String | <p>Beginning time from which the Changeset is active. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `updates_changeset_id` | String | <p>The unique identifier of the Changeset that is being updated.</p> |
| `updated_by_changeset_id` | String | <p>The unique identifier of the updated Changeset.</p> |
| `changeset_arn` | String | <p>The ARN identifier of the Changeset.</p> |
| `create_time` | String | <p>The timestamp at which the Changeset was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `error_info` | String | <p>The structure with error messages.</p> |
| `change_type` | String | <p>Type that indicates how a Changeset is applied to a Dataset.</p>
         <ul>
            <li>
               <p>
                  <code>REPLACE</code> – Changeset is considered as a replacement to all prior loaded Changesets.</p>
            </li>
            <li>
               <p>
                  <code>APPEND</code> – Changeset is considered as an addition to the end of all prior loaded Changesets.</p>
            </li>
            <li>
               <p>
                  <code>MODIFY</code> – Changeset is considered as a replacement to a specific prior ingested Changeset.</p>
            </li>
         </ul> |
| `source_params` | HashMap<String, String> | <p>Options that define the location of the data being ingested.</p> |
| `active_until_timestamp` | String | <p>Time until which the Changeset is active. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.</p> |
| `format_params` | HashMap<String, String> | <p>Structure of the source file(s).</p> |
| `dataset_id` | String | <p>The unique identifier for the FinSpace Dataset where the Changeset is created.</p> |
| `changeset_id` | String | <p>The unique identifier for a Changeset.</p> |
| `status` | String | <p>The status of Changeset creation operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create changeset
changeset = provider.finspace_data.Changeset {
    change_type = "value"  # <p>The option to indicate how a Changeset will be applied to a Dataset.</p>
         <ul>
            <li>
               <p>
                  <code>REPLACE</code> – Changeset will be considered as a replacement to all prior
          loaded Changesets.</p>
            </li>
            <li>
               <p>
                  <code>APPEND</code> – Changeset will be considered as an addition to the end of all
          prior loaded Changesets.</p>
            </li>
            <li>
               <p>
                  <code>MODIFY</code> – Changeset is considered as a replacement to a specific prior ingested Changeset.</p>
            </li>
         </ul>
    source_params = "value"  # <p>Options that define the location of the data being ingested (<code>s3SourcePath</code>) and the source of the changeset (<code>sourceType</code>).</p>
         <p>Both <code>s3SourcePath</code> and <code>sourceType</code> are required attributes.</p>
         <p>Here is an example of how you could specify the <code>sourceParams</code>:</p>
         <p>
            <code>
        "sourceParams": 
        {
        "s3SourcePath": "s3://finspace-landing-us-east-2-bk7gcfvitndqa6ebnvys4d/scratch/wr5hh8pwkpqqkxa4sxrmcw/ingestion/equity.csv",
        "sourceType": "S3"
        }
      </code>
         </p>
         <p>The S3 path that you specify must allow the FinSpace role access. To do that, you first need to configure the IAM policy on S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/finspace/latest/data-api/fs-using-the-finspace-api.html#access-s3-buckets">Loading data from an Amazon S3 Bucket using the FinSpace API</a> section.</p>
    format_params = "value"  # <p>Options that define the structure of the source file(s) including the format type (<code>formatType</code>), header row (<code>withHeader</code>), data separation character (<code>separator</code>) and the type of compression (<code>compression</code>).
    </p>
         <p>
            <code>formatType</code> is a required attribute and can have the following values:
    </p>
         <ul>
            <li>
               <p>
                  <code>PARQUET</code> – Parquet source file format.</p>
            </li>
            <li>
               <p>
                  <code>CSV</code> – CSV source file format.</p>
            </li>
            <li>
               <p>
                  <code>JSON</code> – JSON source file format.</p>
            </li>
            <li>
               <p>
                  <code>XML</code> – XML source file format.</p>
            </li>
         </ul>
         <p>Here is an example of how you could specify the <code>formatParams</code>:</p>
         <p>
            <code>
          "formatParams": 
        {
         "formatType": "CSV",
         "withHeader": "true",
         "separator": ",",
         "compression":"None"
         } 
      </code>
         </p>
         <p>Note that if you only provide <code>formatType</code> as <code>CSV</code>, the rest of the attributes will automatically default to CSV values as following:</p>
         <p>
            <code>
          {
          "withHeader": "true",
          "separator": ","
           }
        </code>
         </p>
         <p> For more information about supported file formats, see <a href="https://docs.aws.amazon.com/finspace/latest/userguide/supported-data-types.html">Supported Data Types and File Formats</a> in the FinSpace User Guide.</p>
    dataset_id = "value"  # <p>The unique identifier for the FinSpace Dataset where the Changeset will be created.
    </p>
}

# Access changeset outputs
changeset_id = changeset.id
changeset_active_from_timestamp = changeset.active_from_timestamp
changeset_updates_changeset_id = changeset.updates_changeset_id
changeset_updated_by_changeset_id = changeset.updated_by_changeset_id
changeset_changeset_arn = changeset.changeset_arn
changeset_create_time = changeset.create_time
changeset_error_info = changeset.error_info
changeset_change_type = changeset.change_type
changeset_source_params = changeset.source_params
changeset_active_until_timestamp = changeset.active_until_timestamp
changeset_format_params = changeset.format_params
changeset_dataset_id = changeset.dataset_id
changeset_changeset_id = changeset.changeset_id
changeset_status = changeset.status
```

---


### Programmatic_access_credentials

ProgrammaticAccessCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credentials` | String | <p>Returns the programmatic credentials.</p> |
| `duration_in_minutes` | i64 | <p>Returns the duration in which the credentials will remain valid.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access programmatic_access_credentials outputs
programmatic_access_credentials_id = programmatic_access_credentials.id
programmatic_access_credentials_credentials = programmatic_access_credentials.credentials
programmatic_access_credentials_duration_in_minutes = programmatic_access_credentials.duration_in_minutes
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple working_location resources
working_location_0 = provider.finspace_data.Working_location {
}
working_location_1 = provider.finspace_data.Working_location {
}
working_location_2 = provider.finspace_data.Working_location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    working_location = provider.finspace_data.Working_location {
    }
```

---

## Related Documentation

- [AWS Finspace_data Documentation](https://docs.aws.amazon.com/finspace_data/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
