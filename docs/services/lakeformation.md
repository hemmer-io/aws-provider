# Lakeformation Service



**Resources**: 20

---

## Overview

The lakeformation service provides access to 20 resource types:

- [Data_cells_filter](#data_cells_filter) [CRUD]
- [Query_state](#query_state) [R]
- [Table_objects](#table_objects) [RU]
- [Lf_tag_expression](#lf_tag_expression) [CRUD]
- [Work_unit_results](#work_unit_results) [R]
- [Lf_tag](#lf_tag) [CRUD]
- [Lake_formation_opt_in](#lake_formation_opt_in) [CD]
- [Data_lake_settings](#data_lake_settings) [CR]
- [Query_statistics](#query_statistics) [R]
- [Transaction](#transaction) [R]
- [Temporary_glue_partition_credentials](#temporary_glue_partition_credentials) [R]
- [Resource](#resource) [RU]
- [Table_storage_optimizer](#table_storage_optimizer) [U]
- [Work_units](#work_units) [R]
- [Lake_formation_identity_center_configuration](#lake_formation_identity_center_configuration) [CRUD]
- [Temporary_glue_table_credentials](#temporary_glue_table_credentials) [R]
- [Resource_lf_tags](#resource_lf_tags) [R]
- [Effective_permissions_for_path](#effective_permissions_for_path) [R]
- [Objects_on_cancel](#objects_on_cancel) [D]
- [Data_lake_principal](#data_lake_principal) [R]

---

## Resources


### Data_cells_filter

DataCellsFilter resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `table_data` | String | ✅ | <p>A <code>DataCellsFilter</code> structure containing information about the data cells filter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_cells_filter` | String | <p>A structure that describes certain columns on certain rows.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_cells_filter
data_cells_filter = provider.lakeformation.Data_cells_filter {
    table_data = "value"  # <p>A <code>DataCellsFilter</code> structure containing information about the data cells filter.</p>
}

# Access data_cells_filter outputs
data_cells_filter_id = data_cells_filter.id
data_cells_filter_data_cells_filter = data_cells_filter.data_cells_filter
```

---


### Query_state

QueryState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | <p>An error message when the operation fails.</p> |
| `state` | String | <p>The state of a query previously submitted. The possible states are:</p>
         <ul>
            <li>
               <p>PENDING: the query is pending.</p>
            </li>
            <li>
               <p>WORKUNITS_AVAILABLE: some work units are ready for retrieval and execution.</p>
            </li>
            <li>
               <p>FINISHED: the query planning finished successfully, and all work units are ready for retrieval and execution.</p>
            </li>
            <li>
               <p>ERROR: an error occurred with the query, such as an invalid query ID or a backend error.</p>
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

# Access query_state outputs
query_state_id = query_state.id
query_state_error = query_state.error
query_state_state = query_state.state
```

---


### Table_objects

TableObjects resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `write_operations` | Vec<String> | ✅ | <p>A list of <code>WriteOperation</code> objects that define an object to add to or delete from the manifest for a governed table.</p> |
| `transaction_id` | String |  | <p>The transaction at which to do the write.</p> |
| `database_name` | String | ✅ | <p>The database containing the governed table to update.</p> |
| `catalog_id` | String |  | <p>The catalog containing the governed table to update. Defaults to the caller’s account ID.</p> |
| `table_name` | String | ✅ | <p>The governed table to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `objects` | Vec<String> | <p>A list of objects organized by partition keys.</p> |
| `next_token` | String | <p>A continuation token indicating whether additional data is available.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_objects outputs
table_objects_id = table_objects.id
table_objects_objects = table_objects.objects
table_objects_next_token = table_objects.next_token
```

---


### Lf_tag_expression

LFTagExpression resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description with information about the LF-Tag expression.</p> |
| `catalog_id` | String |  | <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p> |
| `expression` | Vec<String> | ✅ | <p>A list of LF-Tag conditions (key-value pairs).</p> |
| `name` | String | ✅ | <p>A name for the expression.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description with information about the LF-Tag expression.</p> |
| `expression` | Vec<String> | <p>The body of the LF-Tag expression. It is composed of one or more LF-Tag key-value pairs.</p> |
| `catalog_id` | String | <p>The identifier for the Data Catalog. By default, the account ID in which the LF-Tag expression is saved.</p> |
| `name` | String | <p>The name for the LF-Tag expression. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lf_tag_expression
lf_tag_expression = provider.lakeformation.Lf_tag_expression {
    expression = "value"  # <p>A list of LF-Tag conditions (key-value pairs).</p>
    name = "value"  # <p>A name for the expression.</p>
}

# Access lf_tag_expression outputs
lf_tag_expression_id = lf_tag_expression.id
lf_tag_expression_description = lf_tag_expression.description
lf_tag_expression_expression = lf_tag_expression.expression
lf_tag_expression_catalog_id = lf_tag_expression.catalog_id
lf_tag_expression_name = lf_tag_expression.name
```

---


### Work_unit_results

WorkUnitResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `result_stream` | String | <p>Rows returned from the <code>GetWorkUnitResults</code> operation as a stream of Apache Arrow v1.0 messages.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access work_unit_results outputs
work_unit_results_id = work_unit_results.id
work_unit_results_result_stream = work_unit_results.result_stream
```

---


### Lf_tag

LFTag resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_id` | String |  | <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p> |
| `tag_values` | Vec<String> | ✅ | <p>A list of possible values an attribute can take.</p> |
| `tag_key` | String | ✅ | <p>The key-name for the LF-tag.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalog_id` | String | <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p> |
| `tag_key` | String | <p>The key-name for the LF-tag.</p> |
| `tag_values` | Vec<String> | <p>A list of possible values an attribute can take.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lf_tag
lf_tag = provider.lakeformation.Lf_tag {
    tag_values = "value"  # <p>A list of possible values an attribute can take.</p>
    tag_key = "value"  # <p>The key-name for the LF-tag.</p>
}

# Access lf_tag outputs
lf_tag_id = lf_tag.id
lf_tag_catalog_id = lf_tag.catalog_id
lf_tag_tag_key = lf_tag.tag_key
lf_tag_tag_values = lf_tag.tag_values
```

---


### Lake_formation_opt_in

LakeFormationOptIn resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal` | String | ✅ |  |
| `condition` | String |  |  |
| `resource` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lake_formation_opt_in
lake_formation_opt_in = provider.lakeformation.Lake_formation_opt_in {
    principal = "value"  # Required field
    resource = "value"  # Required field
}

```

---


### Data_lake_settings

DataLakeSettings resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_id` | String |  | <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p> |
| `data_lake_settings` | String | ✅ | <p>A structure representing a list of Lake Formation principals designated as data lake administrators.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_lake_settings` | String | <p>A structure representing a list of Lake Formation principals designated as data lake administrators.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_lake_settings
data_lake_settings = provider.lakeformation.Data_lake_settings {
    data_lake_settings = "value"  # <p>A structure representing a list of Lake Formation principals designated as data lake administrators.</p>
}

# Access data_lake_settings outputs
data_lake_settings_id = data_lake_settings.id
data_lake_settings_data_lake_settings = data_lake_settings.data_lake_settings
```

---


### Query_statistics

QueryStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_submission_time` | String | <p>The time that the query was submitted.</p> |
| `execution_statistics` | String | <p>An <code>ExecutionStatistics</code> structure containing execution statistics.</p> |
| `planning_statistics` | String | <p>A <code>PlanningStatistics</code> structure containing query planning statistics.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_statistics outputs
query_statistics_id = query_statistics.id
query_statistics_query_submission_time = query_statistics.query_submission_time
query_statistics_execution_statistics = query_statistics.execution_statistics
query_statistics_planning_statistics = query_statistics.planning_statistics
```

---


### Transaction

Transaction resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transaction_description` | String | <p>Returns a <code>TransactionDescription</code> object containing information about the transaction.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transaction outputs
transaction_id = transaction.id
transaction_transaction_description = transaction.transaction_description
```

---


### Temporary_glue_partition_credentials

TemporaryGluePartitionCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `secret_access_key` | String | <p>The secret key for the temporary credentials.</p> |
| `session_token` | String | <p>The session token for the temporary credentials.</p> |
| `access_key_id` | String | <p>The access key ID for the temporary credentials.</p> |
| `expiration` | String | <p>The date and time when the temporary credentials expire.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access temporary_glue_partition_credentials outputs
temporary_glue_partition_credentials_id = temporary_glue_partition_credentials.id
temporary_glue_partition_credentials_secret_access_key = temporary_glue_partition_credentials.secret_access_key
temporary_glue_partition_credentials_session_token = temporary_glue_partition_credentials.session_token
temporary_glue_partition_credentials_access_key_id = temporary_glue_partition_credentials.access_key_id
temporary_glue_partition_credentials_expiration = temporary_glue_partition_credentials.expiration
```

---


### Resource

Resource resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_arn` | String | ✅ | <p>The new role to use for the given resource registered in Lake Formation.</p> |
| `with_federation` | bool |  | <p>Whether or not the resource is a federated resource.</p> |
| `resource_arn` | String | ✅ | <p>The resource ARN.</p> |
| `hybrid_access_enabled` | bool |  | <p>
      Specifies whether the data access of tables pointing to the location can be managed by both Lake Formation permissions as well as Amazon S3 bucket policies.
    </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_info` | String | <p>A structure containing information about an Lake Formation resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource outputs
resource_id = resource.id
resource_resource_info = resource.resource_info
```

---


### Table_storage_optimizer

TableStorageOptimizer resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `storage_optimizer_config` | HashMap<String, HashMap<String, String>> | ✅ | <p>Name of the configuration for the storage optimizer.</p> |
| `catalog_id` | String |  | <p>The Catalog ID of the table.</p> |
| `database_name` | String | ✅ | <p>Name of the database where the table is present.</p> |
| `table_name` | String | ✅ | <p>Name of the table for which to enable the storage optimizer.</p> |



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


### Work_units

WorkUnits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_id` | String | <p>The ID of the plan query operation.</p> |
| `work_unit_ranges` | Vec<String> | <p>A <code>WorkUnitRangeList</code> object that specifies the valid range of work unit IDs for querying the execution service.</p> |
| `next_token` | String | <p>A continuation token for paginating the returned list of tokens, returned if the current segment of the list is not the last.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access work_units outputs
work_units_id = work_units.id
work_units_query_id = work_units.query_id
work_units_work_unit_ranges = work_units.work_unit_ranges
work_units_next_token = work_units.next_token
```

---


### Lake_formation_identity_center_configuration

LakeFormationIdentityCenterConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_id` | String |  | <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the
         persistent metadata store. It contains database definitions, table definitions, view
         definitions, and other control information to manage your Lake Formation
         environment.</p> |
| `external_filtering` | String |  | <p>A list of the account IDs of Amazon Web Services accounts of third-party applications
         that are allowed to access data managed by Lake Formation.</p> |
| `share_recipients` | Vec<String> |  | <p>A list of Amazon Web Services account IDs and/or Amazon Web Services organization/organizational unit ARNs
         that are allowed to access data managed by Lake Formation. </p>
         <p>If the <code>ShareRecipients</code> list includes valid values, a resource share is created with the principals you want to have access to the resources.</p>
         <p>If the <code>ShareRecipients</code> value is null or the list is empty, no resource share is created.</p> |
| `instance_arn` | String |  | <p>The ARN of the IAM Identity Center instance for which the operation will be executed.
         For more information about ARNs, see Amazon Resource Names (ARNs) and Amazon Web Services Service
         Namespaces in the Amazon Web Services General Reference.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `share_recipients` | Vec<String> | <p>A list of Amazon Web Services account IDs or Amazon Web Services organization/organizational unit ARNs that
         are allowed to access data managed by Lake Formation. </p>
         <p>If the <code>ShareRecipients</code> list includes valid values, a resource share is created with the principals you want to have access to the resources as the <code>ShareRecipients</code>.</p>
         <p>If the <code>ShareRecipients</code> value is null or the list is empty, no resource share is created.</p> |
| `catalog_id` | String | <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.</p> |
| `application_arn` | String | <p>The Amazon Resource Name (ARN) of the Lake Formation application integrated with IAM Identity Center.</p> |
| `external_filtering` | String | <p>Indicates if external filtering is enabled.</p> |
| `resource_share` | String | <p>The Amazon Resource Name (ARN) of the RAM share.</p> |
| `instance_arn` | String | <p>The Amazon Resource Name (ARN) of the connection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lake_formation_identity_center_configuration
lake_formation_identity_center_configuration = provider.lakeformation.Lake_formation_identity_center_configuration {
}

# Access lake_formation_identity_center_configuration outputs
lake_formation_identity_center_configuration_id = lake_formation_identity_center_configuration.id
lake_formation_identity_center_configuration_share_recipients = lake_formation_identity_center_configuration.share_recipients
lake_formation_identity_center_configuration_catalog_id = lake_formation_identity_center_configuration.catalog_id
lake_formation_identity_center_configuration_application_arn = lake_formation_identity_center_configuration.application_arn
lake_formation_identity_center_configuration_external_filtering = lake_formation_identity_center_configuration.external_filtering
lake_formation_identity_center_configuration_resource_share = lake_formation_identity_center_configuration.resource_share
lake_formation_identity_center_configuration_instance_arn = lake_formation_identity_center_configuration.instance_arn
```

---


### Temporary_glue_table_credentials

TemporaryGlueTableCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_key_id` | String | <p>The access key ID for the temporary credentials.</p> |
| `session_token` | String | <p>The session token for the temporary credentials.</p> |
| `secret_access_key` | String | <p>The secret key for the temporary credentials.</p> |
| `expiration` | String | <p>The date and time when the temporary credentials expire.</p> |
| `vended_s3_path` | Vec<String> | <p>The Amazon S3 path for the temporary credentials.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access temporary_glue_table_credentials outputs
temporary_glue_table_credentials_id = temporary_glue_table_credentials.id
temporary_glue_table_credentials_access_key_id = temporary_glue_table_credentials.access_key_id
temporary_glue_table_credentials_session_token = temporary_glue_table_credentials.session_token
temporary_glue_table_credentials_secret_access_key = temporary_glue_table_credentials.secret_access_key
temporary_glue_table_credentials_expiration = temporary_glue_table_credentials.expiration
temporary_glue_table_credentials_vended_s3_path = temporary_glue_table_credentials.vended_s3_path
```

---


### Resource_lf_tags

ResourceLFTags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lf_tags_on_table` | Vec<String> | <p>A list of LF-tags applied to a table resource.</p> |
| `lf_tags_on_columns` | Vec<String> | <p>A list of LF-tags applied to a column resource.</p> |
| `lf_tag_on_database` | Vec<String> | <p>A list of LF-tags applied to a database resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_lf_tags outputs
resource_lf_tags_id = resource_lf_tags.id
resource_lf_tags_lf_tags_on_table = resource_lf_tags.lf_tags_on_table
resource_lf_tags_lf_tags_on_columns = resource_lf_tags.lf_tags_on_columns
resource_lf_tags_lf_tag_on_database = resource_lf_tags.lf_tag_on_database
```

---


### Effective_permissions_for_path

EffectivePermissionsForPath resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if this is not the first call to retrieve this list.</p> |
| `permissions` | Vec<String> | <p>A list of the permissions for the specified table or database resource located at the path in Amazon S3.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access effective_permissions_for_path outputs
effective_permissions_for_path_id = effective_permissions_for_path.id
effective_permissions_for_path_next_token = effective_permissions_for_path.next_token
effective_permissions_for_path_permissions = effective_permissions_for_path.permissions
```

---


### Objects_on_cancel

ObjectsOnCancel resource

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


### Data_lake_principal

DataLakePrincipal resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity` | String | <p>A unique identifier of the invoking principal.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_lake_principal outputs
data_lake_principal_id = data_lake_principal.id
data_lake_principal_identity = data_lake_principal.identity
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple data_cells_filter resources
data_cells_filter_0 = provider.lakeformation.Data_cells_filter {
    table_data = "value-0"
}
data_cells_filter_1 = provider.lakeformation.Data_cells_filter {
    table_data = "value-1"
}
data_cells_filter_2 = provider.lakeformation.Data_cells_filter {
    table_data = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    data_cells_filter = provider.lakeformation.Data_cells_filter {
        table_data = "production-value"
    }
```

---

## Related Documentation

- [AWS Lakeformation Documentation](https://docs.aws.amazon.com/lakeformation/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
