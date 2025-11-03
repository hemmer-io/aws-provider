# Athena Service



**Resources**: 19

---

## Overview

The athena service provides access to 19 resource types:

- [Capacity_reservation](#capacity_reservation) [CRUD]
- [Query_execution](#query_execution) [R]
- [Query_runtime_statistics](#query_runtime_statistics) [R]
- [Named_query](#named_query) [CRUD]
- [Presigned_notebook_url](#presigned_notebook_url) [C]
- [Notebook_metadata](#notebook_metadata) [RU]
- [Capacity_assignment_configuration](#capacity_assignment_configuration) [CR]
- [Prepared_statement](#prepared_statement) [CRUD]
- [Calculation_execution](#calculation_execution) [R]
- [Work_group](#work_group) [CRUD]
- [Query_results](#query_results) [R]
- [Session](#session) [R]
- [Session_status](#session_status) [R]
- [Calculation_execution_code](#calculation_execution_code) [R]
- [Notebook](#notebook) [CUD]
- [Database](#database) [R]
- [Table_metadata](#table_metadata) [R]
- [Data_catalog](#data_catalog) [CRUD]
- [Calculation_execution_status](#calculation_execution_status) [R]

---

## Resources


### Capacity_reservation

CapacityReservation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the capacity reservation to create.</p> |
| `tags` | Vec<String> |  | <p>The tags for the capacity reservation.</p> |
| `target_dpus` | i64 | ✅ | <p>The number of requested data processing units.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_reservation` | String | <p>The requested capacity reservation structure.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create capacity_reservation
capacity_reservation = provider.athena.Capacity_reservation {
    name = "value"  # <p>The name of the capacity reservation to create.</p>
    target_dpus = "value"  # <p>The number of requested data processing units.</p>
}

# Access capacity_reservation outputs
capacity_reservation_id = capacity_reservation.id
capacity_reservation_capacity_reservation = capacity_reservation.capacity_reservation
```

---


### Query_execution

QueryExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_execution` | String | <p>Information about the query execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_execution outputs
query_execution_id = query_execution.id
query_execution_query_execution = query_execution.query_execution
```

---


### Query_runtime_statistics

QueryRuntimeStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_runtime_statistics` | String | <p>Runtime statistics about the query execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_runtime_statistics outputs
query_runtime_statistics_id = query_runtime_statistics.id
query_runtime_statistics_query_runtime_statistics = query_runtime_statistics.query_runtime_statistics
```

---


### Named_query

NamedQuery resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `work_group` | String |  | <p>The name of the workgroup in which the named query is being created.</p> |
| `description` | String |  | <p>The query description.</p> |
| `name` | String | ✅ | <p>The query name.</p> |
| `database` | String | ✅ | <p>The database to which the query belongs.</p> |
| `query_string` | String | ✅ | <p>The contents of the query with all query statements.</p> |
| `client_request_token` | String |  | <p>A unique case-sensitive string used to ensure the request to create the query is
            idempotent (executes only once). If another <code>CreateNamedQuery</code> request is
            received, the same response is returned and another query is not created. If a parameter
            has changed, for example, the <code>QueryString</code>, an error is returned.</p>
         <important>
            <p>This token is listed as not required because Amazon Web Services SDKs (for example
                the Amazon Web Services SDK for Java) auto-generate the token for users. If you are
                not using the Amazon Web Services SDK or the Amazon Web Services CLI, you must provide
                this token or the action will fail.</p>
         </important> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `named_query` | String | <p>Information about the query.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create named_query
named_query = provider.athena.Named_query {
    name = "value"  # <p>The query name.</p>
    database = "value"  # <p>The database to which the query belongs.</p>
    query_string = "value"  # <p>The contents of the query with all query statements.</p>
}

# Access named_query outputs
named_query_id = named_query.id
named_query_named_query = named_query.named_query
```

---


### Presigned_notebook_url

PresignedNotebookUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_id` | String | ✅ | <p>The session ID.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create presigned_notebook_url
presigned_notebook_url = provider.athena.Presigned_notebook_url {
    session_id = "value"  # <p>The session ID.</p>
}

```

---


### Notebook_metadata

NotebookMetadata resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>A unique case-sensitive string used to ensure the request to create the notebook is
            idempotent (executes only once).</p>
         <important>
            <p>This token is listed as not required because Amazon Web Services SDKs (for example
                the Amazon Web Services SDK for Java) auto-generate the token for you. If you are not
                using the Amazon Web Services SDK or the Amazon Web Services CLI, you must provide
                this token or the action will fail.</p>
         </important> |
| `notebook_id` | String | ✅ | <p>The ID of the notebook to update the metadata for.</p> |
| `name` | String | ✅ | <p>The name to update the notebook to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notebook_metadata` | String | <p>The metadata that is returned for the specified notebook ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access notebook_metadata outputs
notebook_metadata_id = notebook_metadata.id
notebook_metadata_notebook_metadata = notebook_metadata.notebook_metadata
```

---


### Capacity_assignment_configuration

CapacityAssignmentConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `capacity_reservation_name` | String | ✅ | <p>The name of the capacity reservation to put a capacity assignment configuration
            for.</p> |
| `capacity_assignments` | Vec<String> | ✅ | <p>The list of assignments for the capacity assignment configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_assignment_configuration` | String | <p>The requested capacity assignment configuration for the specified capacity
            reservation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create capacity_assignment_configuration
capacity_assignment_configuration = provider.athena.Capacity_assignment_configuration {
    capacity_reservation_name = "value"  # <p>The name of the capacity reservation to put a capacity assignment configuration
            for.</p>
    capacity_assignments = "value"  # <p>The list of assignments for the capacity assignment configuration.</p>
}

# Access capacity_assignment_configuration outputs
capacity_assignment_configuration_id = capacity_assignment_configuration.id
capacity_assignment_configuration_capacity_assignment_configuration = capacity_assignment_configuration.capacity_assignment_configuration
```

---


### Prepared_statement

PreparedStatement resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `statement_name` | String | ✅ | <p>The name of the prepared statement.</p> |
| `query_statement` | String | ✅ | <p>The query string for the prepared statement.</p> |
| `work_group` | String | ✅ | <p>The name of the workgroup to which the prepared statement belongs.</p> |
| `description` | String |  | <p>The description of the prepared statement.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prepared_statement` | String | <p>The name of the prepared statement that was retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create prepared_statement
prepared_statement = provider.athena.Prepared_statement {
    statement_name = "value"  # <p>The name of the prepared statement.</p>
    query_statement = "value"  # <p>The query string for the prepared statement.</p>
    work_group = "value"  # <p>The name of the workgroup to which the prepared statement belongs.</p>
}

# Access prepared_statement outputs
prepared_statement_id = prepared_statement.id
prepared_statement_prepared_statement = prepared_statement.prepared_statement
```

---


### Calculation_execution

CalculationExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `working_directory` | String | <p>The Amazon S3 location in which calculation results are stored.</p> |
| `status` | String | <p>Contains information about the status of the calculation.</p> |
| `description` | String | <p>The description of the calculation execution.</p> |
| `result` | String | <p>Contains result information. This field is populated only if the calculation is
            completed.</p> |
| `calculation_execution_id` | String | <p>The calculation execution UUID.</p> |
| `statistics` | String | <p>Contains information about the data processing unit (DPU) execution time and progress.
            This field is populated only when statistics are available.</p> |
| `session_id` | String | <p>The session ID that the calculation ran in.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access calculation_execution outputs
calculation_execution_id = calculation_execution.id
calculation_execution_working_directory = calculation_execution.working_directory
calculation_execution_status = calculation_execution.status
calculation_execution_description = calculation_execution.description
calculation_execution_result = calculation_execution.result
calculation_execution_calculation_execution_id = calculation_execution.calculation_execution_id
calculation_execution_statistics = calculation_execution.statistics
calculation_execution_session_id = calculation_execution.session_id
```

---


### Work_group

WorkGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The workgroup name.</p> |
| `tags` | Vec<String> |  | <p>A list of comma separated tags to add to the workgroup that is created.</p> |
| `configuration` | String |  | <p>Contains configuration information for creating an Athena SQL workgroup or
            Spark enabled Athena workgroup. Athena SQL workgroup
            configuration includes the location in Amazon S3 where query and calculation
            results are stored, the encryption configuration, if any, used for encrypting query
            results, whether the Amazon CloudWatch Metrics are enabled for the workgroup, the
            limit for the amount of bytes scanned (cutoff) per query, if it is specified, and
            whether workgroup's settings (specified with <code>EnforceWorkGroupConfiguration</code>)
            in the <code>WorkGroupConfiguration</code> override client-side settings. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p> |
| `description` | String |  | <p>The workgroup description.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `work_group` | String | <p>Information about the workgroup.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create work_group
work_group = provider.athena.Work_group {
    name = "value"  # <p>The workgroup name.</p>
}

# Access work_group outputs
work_group_id = work_group.id
work_group_work_group = work_group.work_group
```

---


### Query_results

QueryResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `result_set` | String | <p>The results of the query execution.</p> |
| `next_token` | String | <p>A token generated by the Athena service that specifies where to continue
            pagination if a previous request was truncated. To obtain the next set of pages, pass in
            the <code>NextToken</code> from the response object of the previous page call.</p> |
| `update_count` | i64 | <p>The number of rows inserted with a <code>CREATE TABLE AS SELECT</code>, <code>INSERT
                INTO</code>, or <code>UPDATE</code> statement. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_results outputs
query_results_id = query_results.id
query_results_result_set = query_results.result_set
query_results_next_token = query_results.next_token
query_results_update_count = query_results.update_count
```

---


### Session

Session resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `work_group` | String | <p>The workgroup to which the session belongs.</p> |
| `notebook_version` | String | <p>The notebook version.</p> |
| `description` | String | <p>The session description.</p> |
| `session_id` | String | <p>The session ID.</p> |
| `engine_version` | String | <p>The engine version used by the session (for example, <code>PySpark engine version
                3</code>). You can get a list of engine versions by calling <a>ListEngineVersions</a>.</p> |
| `engine_configuration` | String | <p>Contains engine configuration information like DPU usage.</p> |
| `statistics` | String | <p>Contains the DPU execution time.</p> |
| `session_configuration` | String | <p>Contains the workgroup configuration information used by the session.</p> |
| `status` | String | <p>Contains information about the status of the session.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access session outputs
session_id = session.id
session_work_group = session.work_group
session_notebook_version = session.notebook_version
session_description = session.description
session_session_id = session.session_id
session_engine_version = session.engine_version
session_engine_configuration = session.engine_configuration
session_statistics = session.statistics
session_session_configuration = session.session_configuration
session_status = session.status
```

---


### Session_status

SessionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Contains information about the status of the session.</p> |
| `session_id` | String | <p>The session ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access session_status outputs
session_status_id = session_status.id
session_status_status = session_status.status
session_status_session_id = session_status.session_id
```

---


### Calculation_execution_code

CalculationExecutionCode resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `code_block` | String | <p>The unencrypted code that was executed for the calculation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access calculation_execution_code outputs
calculation_execution_code_id = calculation_execution_code.id
calculation_execution_code_code_block = calculation_execution_code.code_block
```

---


### Notebook

Notebook resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `work_group` | String | ✅ | <p>The name of the Spark enabled workgroup in which the notebook will be created.</p> |
| `client_request_token` | String |  | <p>A unique case-sensitive string used to ensure the request to create the notebook is
            idempotent (executes only once).</p>
         <important>
            <p>This token is listed as not required because Amazon Web Services SDKs (for example
                the Amazon Web Services SDK for Java) auto-generate the token for you. If you are not
                using the Amazon Web Services SDK or the Amazon Web Services CLI, you must provide
                this token or the action will fail.</p>
         </important> |
| `name` | String | ✅ | <p>The name of the <code>ipynb</code> file to be created in the Spark workgroup, without
            the <code>.ipynb</code> extension.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notebook
notebook = provider.athena.Notebook {
    work_group = "value"  # <p>The name of the Spark enabled workgroup in which the notebook will be created.</p>
    name = "value"  # <p>The name of the <code>ipynb</code> file to be created in the Spark workgroup, without
            the <code>.ipynb</code> extension.</p>
}

```

---


### Database

Database resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `database` | String | <p>The database returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access database outputs
database_id = database.id
database_database = database.database
```

---


### Table_metadata

TableMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_metadata` | String | <p>An object that contains table metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_metadata outputs
table_metadata_id = table_metadata.id
table_metadata_table_metadata = table_metadata.table_metadata
```

---


### Data_catalog

DataCatalog resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | <p>Specifies the Lambda function or functions to use for creating the data
            catalog. This is a mapping whose values depend on the catalog type. </p>
         <ul>
            <li>
               <p>For the <code>HIVE</code> data catalog type, use the following syntax. The
                        <code>metadata-function</code> parameter is required. <code>The
                        sdk-version</code> parameter is optional and defaults to the currently
                    supported version.</p>
               <p>
                  <code>metadata-function=<i>lambda_arn</i>,
                            sdk-version=<i>version_number</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For the <code>LAMBDA</code> data catalog type, use one of the following sets
                    of required parameters, but not both.</p>
               <ul>
                  <li>
                     <p>If you have one Lambda function that processes metadata
                            and another for reading the actual data, use the following syntax. Both
                            parameters are required.</p>
                     <p>
                        <code>metadata-function=<i>lambda_arn</i>,
                                    record-function=<i>lambda_arn</i>
                        </code>
                     </p>
                  </li>
                  <li>
                     <p> If you have a composite Lambda function that processes
                            both metadata and data, use the following syntax to specify your Lambda function.</p>
                     <p>
                        <code>function=<i>lambda_arn</i>
                        </code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>The <code>GLUE</code> type takes a catalog ID parameter and is required. The
                            <code>
                     <i>catalog_id</i>
                  </code> is the account ID of the
                        Amazon Web Services account to which the Glue Data Catalog
                    belongs.</p>
               <p>
                  <code>catalog-id=<i>catalog_id</i>
                  </code>
               </p>
               <ul>
                  <li>
                     <p>The <code>GLUE</code> data catalog type also applies to the default
                                <code>AwsDataCatalog</code> that already exists in your account, of
                            which you can have only one and cannot modify.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>The <code>FEDERATED</code> data catalog type uses one of the following
                    parameters, but not both. Use <code>connection-arn</code> for an existing
                        Glue connection. Use <code>connection-type</code> and
                        <code>connection-properties</code> to specify the configuration setting for
                    a new connection.</p>
               <ul>
                  <li>
                     <p>
                        <code>connection-arn:<i><glue_connection_arn_to_reuse></i>
                        </code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>lambda-role-arn</code> (optional): The execution role to use for the
                            Lambda function. If not provided, one is created.</p>
                  </li>
                  <li>
                     <p>
                        <code>connection-type:MYSQL|REDSHIFT|....,
                                    connection-properties:"<i><json_string></i>"</code>
                     </p>
                     <p>For <i>
                           <code><json_string></code>
                        </i>, use escaped
                            JSON text, as in the following example.</p>
                     <p>
                        <code>"{\"spill_bucket\":\"my_spill\",\"spill_prefix\":\"athena-spill\",\"host\":\"abc12345.snowflakecomputing.com\",\"port\":\"1234\",\"warehouse\":\"DEV_WH\",\"database\":\"TEST\",\"schema\":\"PUBLIC\",\"SecretArn\":\"arn:aws:secretsmanager:ap-south-1:111122223333:secret:snowflake-XHb67j\"}"</code>
                     </p>
                  </li>
               </ul>
            </li>
         </ul> |
| `description` | String |  | <p>A description of the data catalog to be created.</p> |
| `tags` | Vec<String> |  | <p>A list of comma separated tags to add to the data catalog that is created. All the
            resources that are created by the <code>CreateDataCatalog</code> API operation with
                <code>FEDERATED</code> type will have the tag
                <code>federated_athena_datacatalog="true"</code>. This includes the CFN Stack, Glue
            Connection, Athena DataCatalog, and all the resources created as part of the CFN Stack
            (Lambda Function, IAM policies/roles).</p> |
| `type` | String | ✅ | <p>The type of data catalog to create: <code>LAMBDA</code> for a federated catalog,
                <code>GLUE</code> for an Glue Data Catalog, and <code>HIVE</code> for an
            external Apache Hive metastore. <code>FEDERATED</code> is a federated catalog for which
                Athena creates the connection and the Lambda function for
            you based on the parameters that you pass.</p>
         <p>For <code>FEDERATED</code> type, we do not support IAM identity center.</p> |
| `name` | String | ✅ | <p>The name of the data catalog to create. The catalog name must be unique for the
                Amazon Web Services account and can use a maximum of 127 alphanumeric, underscore, at
            sign, or hyphen characters. The remainder of the length constraint of 256 is reserved
            for use by Athena.</p>
         <p>For <code>FEDERATED</code> type the catalog name has following considerations and
            limits:</p>
         <ul>
            <li>
               <p>The catalog name allows special characters such as <code>_ , @ , \ , -
                    </code>. These characters are replaced with a hyphen (-) when creating the CFN
                    Stack Name and with an underscore (_) when creating the Lambda Function and Glue
                    Connection Name.</p>
            </li>
            <li>
               <p>The catalog name has a theoretical limit of 128 characters. However, since we
                    use it to create other resources that allow less characters and we prepend a
                    prefix to it, the actual catalog name limit for <code>FEDERATED</code> catalog
                    is 64 - 23 = 41 characters.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_catalog` | String | <p>The data catalog returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_catalog
data_catalog = provider.athena.Data_catalog {
    type = "value"  # <p>The type of data catalog to create: <code>LAMBDA</code> for a federated catalog,
                <code>GLUE</code> for an Glue Data Catalog, and <code>HIVE</code> for an
            external Apache Hive metastore. <code>FEDERATED</code> is a federated catalog for which
                Athena creates the connection and the Lambda function for
            you based on the parameters that you pass.</p>
         <p>For <code>FEDERATED</code> type, we do not support IAM identity center.</p>
    name = "value"  # <p>The name of the data catalog to create. The catalog name must be unique for the
                Amazon Web Services account and can use a maximum of 127 alphanumeric, underscore, at
            sign, or hyphen characters. The remainder of the length constraint of 256 is reserved
            for use by Athena.</p>
         <p>For <code>FEDERATED</code> type the catalog name has following considerations and
            limits:</p>
         <ul>
            <li>
               <p>The catalog name allows special characters such as <code>_ , @ , \ , -
                    </code>. These characters are replaced with a hyphen (-) when creating the CFN
                    Stack Name and with an underscore (_) when creating the Lambda Function and Glue
                    Connection Name.</p>
            </li>
            <li>
               <p>The catalog name has a theoretical limit of 128 characters. However, since we
                    use it to create other resources that allow less characters and we prepend a
                    prefix to it, the actual catalog name limit for <code>FEDERATED</code> catalog
                    is 64 - 23 = 41 characters.</p>
            </li>
         </ul>
}

# Access data_catalog outputs
data_catalog_id = data_catalog.id
data_catalog_data_catalog = data_catalog.data_catalog
```

---


### Calculation_execution_status

CalculationExecutionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statistics` | String | <p>Contains information about the DPU execution time and progress.</p> |
| `status` | String | <p>Contains information about the calculation execution status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access calculation_execution_status outputs
calculation_execution_status_id = calculation_execution_status.id
calculation_execution_status_statistics = calculation_execution_status.statistics
calculation_execution_status_status = calculation_execution_status.status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple capacity_reservation resources
capacity_reservation_0 = provider.athena.Capacity_reservation {
    name = "value-0"
    target_dpus = "value-0"
}
capacity_reservation_1 = provider.athena.Capacity_reservation {
    name = "value-1"
    target_dpus = "value-1"
}
capacity_reservation_2 = provider.athena.Capacity_reservation {
    name = "value-2"
    target_dpus = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    capacity_reservation = provider.athena.Capacity_reservation {
        name = "production-value"
        target_dpus = "production-value"
    }
```

---

## Related Documentation

- [AWS Athena Documentation](https://docs.aws.amazon.com/athena/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
