# Machine_learning Service



**Resources**: 13

---

## Overview

The machine_learning service provides access to 13 resource types:

- [Data_source_from_redshift](#data_source_from_redshift) [C]
- [Data_source](#data_source) [RUD]
- [Tags](#tags) [RD]
- [Data_source_from_rds](#data_source_from_rds) [C]
- [Batch_predictions](#batch_predictions) [R]
- [Data_source_from_s3](#data_source_from_s3) [C]
- [Ml_models](#ml_models) [R]
- [Evaluation](#evaluation) [CRUD]
- [Batch_prediction](#batch_prediction) [CRUD]
- [Realtime_endpoint](#realtime_endpoint) [CD]
- [Evaluations](#evaluations) [R]
- [Data_sources](#data_sources) [R]
- [Ml_model](#ml_model) [CRUD]

---

## Resources


### Data_source_from_redshift

DataSourceFromRedshift resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_id` | String | ✅ | <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>.</p> |
| `role_arn` | String | ✅ | <p>A fully specified role Amazon Resource Name (ARN). Amazon ML assumes the role on behalf of the user to create the following:</p>
        
           <ul>
            <li>
               <p>A security group to allow Amazon ML to execute the <code>SelectSqlQuery</code> query on an Amazon Redshift cluster</p>
            </li>
            <li>
               <p>An Amazon S3 bucket policy to grant Amazon ML read/write permissions on the <code>S3StagingLocation</code>
               </p>
            </li>
         </ul> |
| `data_source_name` | String |  | <p>A user-supplied name or description of the <code>DataSource</code>. </p> |
| `compute_statistics` | bool |  | <p>The compute statistics for a <code>DataSource</code>. The statistics are generated from the observation data referenced by 
            a <code>DataSource</code>. Amazon ML uses the statistics internally during <code>MLModel</code> training.
           This parameter must be set to <code>true</code> if the <code>DataSource</code> needs to
          be used for <code>MLModel</code> training.</p> |
| `data_spec` | String | ✅ | <p>The data specification of an Amazon Redshift <code>DataSource</code>:</p>
        <ul>
            <li>
               <p>DatabaseInformation -</p> 
               <ul>
                  <li>
                     <p>
                        <code>DatabaseName</code> - The name of the Amazon Redshift database.</p>
                  </li>
                  <li>
                     <p>
                        <code> ClusterIdentifier</code> - The unique ID for the Amazon Redshift cluster.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>DatabaseCredentials - The AWS Identity and Access Management (IAM) credentials that are used to connect to the Amazon Redshift database.</p>
            </li>
            <li>
               <p>SelectSqlQuery - The query that is used to retrieve the observation data for the 
			<code>Datasource</code>.</p>
            </li>
            <li>
               <p>S3StagingLocation - The Amazon Simple Storage Service (Amazon S3) location for staging Amazon
                    Redshift data. The data retrieved from Amazon Redshift using
                        the <code>SelectSqlQuery</code> query is stored in this location.</p>
            </li>
            <li>
               <p>DataSchemaUri - The Amazon S3 location of the <code>DataSchema</code>.</p>
            </li>
            <li>
               <p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p>
            </li>
            <li>
               <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>DataSource</code>.</p>
               <p> Sample - 
            <code> "{\"splitting\":{\"percentBegin\":10,\"percentEnd\":60}}"</code>
               </p>
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

# Create data_source_from_redshift
data_source_from_redshift = provider.machine_learning.Data_source_from_redshift {
    data_source_id = "value"  # <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>.</p>
    role_arn = "value"  # <p>A fully specified role Amazon Resource Name (ARN). Amazon ML assumes the role on behalf of the user to create the following:</p>
        
           <ul>
            <li>
               <p>A security group to allow Amazon ML to execute the <code>SelectSqlQuery</code> query on an Amazon Redshift cluster</p>
            </li>
            <li>
               <p>An Amazon S3 bucket policy to grant Amazon ML read/write permissions on the <code>S3StagingLocation</code>
               </p>
            </li>
         </ul>
    data_spec = "value"  # <p>The data specification of an Amazon Redshift <code>DataSource</code>:</p>
        <ul>
            <li>
               <p>DatabaseInformation -</p> 
               <ul>
                  <li>
                     <p>
                        <code>DatabaseName</code> - The name of the Amazon Redshift database.</p>
                  </li>
                  <li>
                     <p>
                        <code> ClusterIdentifier</code> - The unique ID for the Amazon Redshift cluster.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>DatabaseCredentials - The AWS Identity and Access Management (IAM) credentials that are used to connect to the Amazon Redshift database.</p>
            </li>
            <li>
               <p>SelectSqlQuery - The query that is used to retrieve the observation data for the 
			<code>Datasource</code>.</p>
            </li>
            <li>
               <p>S3StagingLocation - The Amazon Simple Storage Service (Amazon S3) location for staging Amazon
                    Redshift data. The data retrieved from Amazon Redshift using
                        the <code>SelectSqlQuery</code> query is stored in this location.</p>
            </li>
            <li>
               <p>DataSchemaUri - The Amazon S3 location of the <code>DataSchema</code>.</p>
            </li>
            <li>
               <p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p>
            </li>
            <li>
               <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>DataSource</code>.</p>
               <p> Sample - 
            <code> "{\"splitting\":{\"percentBegin\":10,\"percentEnd\":60}}"</code>
               </p>
            </li>
         </ul>
}

```

---


### Data_source

DataSource resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_id` | String | ✅ | <p>The ID assigned to the <code>DataSource</code> during creation.</p> |
| `data_source_name` | String | ✅ | <p>A new user-supplied name or description of the <code>DataSource</code> that will replace the current description. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The current status of the <code>DataSource</code>. This element can have one of the following values:</p>
        <ul>
            <li>
               <p>
                  <code>PENDING</code> - Amazon ML submitted a request to create a <code>DataSource</code>.</p>
            </li>
            <li>
               <p>
                  <code>INPROGRESS</code> - The creation process is underway.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The request to create a <code>DataSource</code> did not run to completion. It is not usable.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED</code> - The creation process completed successfully.</p>
            </li>
            <li>
               <p>
                  <code>DELETED</code> - The <code>DataSource</code> is marked as deleted. It is not usable.</p>
            </li>
         </ul> |
| `data_source_schema` | String | <p>The schema used by all of the data files of this <code>DataSource</code>.</p>
        <p>
            <b>Note:</b> This parameter is provided as part of the verbose format.</p> |
| `role_arn` | String |  |
| `data_rearrangement` | String | <p>A JSON string that represents the splitting and rearrangement requirement used when this <code>DataSource</code>
            was created.</p> |
| `data_source_id` | String | <p>The ID assigned to the <code>DataSource</code> at creation.  This value should be identical to the value of the <code>DataSourceId</code> in the request.</p> |
| `started_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>DataSource</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>DataSource</code> is in the <code>PENDING</code> state.</p> |
| `log_uri` | String | <p>A link to the file containing logs of <code>CreateDataSourceFrom*</code> operations.</p> |
| `finished_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>DataSource</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>DataSource</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p> |
| `created_by_iam_user` | String | <p>The AWS user account from which the <code>DataSource</code> was created. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p> |
| `created_at` | String | <p>The time that the <code>DataSource</code> was created. The time is expressed in epoch time.</p> |
| `data_location_s3` | String | <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p> |
| `message` | String | <p>The user-supplied description of the most recent details about creating the <code>DataSource</code>.</p> |
| `name` | String | <p>A user-supplied name or description of the <code>DataSource</code>.</p> |
| `redshift_metadata` | String |  |
| `compute_time` | i64 | <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>DataSource</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>DataSource</code> is in the <code>COMPLETED</code> state and the <code>ComputeStatistics</code> is set to true.</p> |
| `data_size_in_bytes` | i64 | <p>The total size of observations in the data files.</p> |
| `last_updated_at` | String | <p>The time of the most recent edit to the <code>DataSource</code>. The time is expressed in epoch time.</p> |
| `number_of_files` | i64 | <p>The number of data files referenced by the <code>DataSource</code>.</p> |
| `compute_statistics` | bool | <p>
            The parameter is <code>true</code> if statistics need to be generated from the observation data.
        </p> |
| `rds_metadata` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_source outputs
data_source_id = data_source.id
data_source_status = data_source.status
data_source_data_source_schema = data_source.data_source_schema
data_source_role_arn = data_source.role_arn
data_source_data_rearrangement = data_source.data_rearrangement
data_source_data_source_id = data_source.data_source_id
data_source_started_at = data_source.started_at
data_source_log_uri = data_source.log_uri
data_source_finished_at = data_source.finished_at
data_source_created_by_iam_user = data_source.created_by_iam_user
data_source_created_at = data_source.created_at
data_source_data_location_s3 = data_source.data_location_s3
data_source_message = data_source.message
data_source_name = data_source.name
data_source_redshift_metadata = data_source.redshift_metadata
data_source_compute_time = data_source.compute_time
data_source_data_size_in_bytes = data_source.data_size_in_bytes
data_source_last_updated_at = data_source.last_updated_at
data_source_number_of_files = data_source.number_of_files
data_source_compute_statistics = data_source.compute_statistics
data_source_rds_metadata = data_source.rds_metadata
```

---


### Tags

Tags resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_type` | String | <p>The type of the tagged ML object.</p> |
| `resource_id` | String | <p>The ID of the tagged ML object.</p> |
| `tags` | Vec<String> | <p>A list of tags associated with the ML object.</p> |


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
tags_resource_type = tags.resource_type
tags_resource_id = tags.resource_id
tags_tags = tags.tags
```

---


### Data_source_from_rds

DataSourceFromRDS resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_id` | String | ✅ | <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>. Typically, an Amazon Resource Number (ARN) 
            becomes the ID for a <code>DataSource</code>.</p> |
| `data_source_name` | String |  | <p>A user-supplied name or description of the <code>DataSource</code>.</p> |
| `compute_statistics` | bool |  | <p>The compute statistics for a <code>DataSource</code>. The statistics are generated from the observation data referenced by 
            a <code>DataSource</code>. Amazon ML uses the statistics internally during <code>MLModel</code> training.
            This parameter must be set to <code>true</code> if the <code></code>DataSource<code></code> needs to be used for <code>MLModel</code> training.
            </p> |
| `rds_data` | String | ✅ | <p>The data specification of an Amazon RDS <code>DataSource</code>:</p>
        <ul>
            <li>
               <p>DatabaseInformation -</p>
                <ul>
                  <li>
                     <p>
                        <code>DatabaseName</code> - The name of the Amazon RDS database.</p>
                  </li>
                  <li>
                     <p>
                        <code>InstanceIdentifier </code> - A unique identifier for the Amazon RDS database instance.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>DatabaseCredentials - AWS Identity and Access Management (IAM) credentials that are used to connect to the Amazon RDS database.</p>
            </li>
            <li>
               <p>ResourceRole - A role (DataPipelineDefaultResourceRole) assumed by an EC2 instance to carry out the copy task from Amazon RDS to Amazon 
			Simple Storage Service (Amazon S3). For more information, see <a href="https://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
            </li>
            <li>
               <p>ServiceRole - A role (DataPipelineDefaultRole) assumed by the AWS Data Pipeline service to monitor the progress of the copy task from Amazon RDS
			to Amazon S3. For more information, see <a href="https://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
            </li>
            <li>
               <p>SecurityInfo - The security information to use to access an RDS DB instance. You need to set up appropriate ingress rules for the security entity IDs provided to allow access to the Amazon RDS instance. Specify a [<code>SubnetId</code>, <code>SecurityGroupIds</code>] pair for a VPC-based RDS DB instance.</p>
            </li>
            <li>
               <p>SelectSqlQuery - A query that is used to retrieve the observation data for the <code>Datasource</code>.</p>
            </li>
            <li>
               <p>S3StagingLocation - The Amazon S3 location for staging Amazon RDS data. The data retrieved from Amazon RDS using <code>SelectSqlQuery</code> is stored in this location.</p>
            </li>
            <li>
               <p>DataSchemaUri - The Amazon S3 location of the <code>DataSchema</code>.</p>
            </li>
            <li>
               <p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p>
            </li>
            <li>
               <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>Datasource</code>. </p>
               <p> Sample - 
            <code> "{\"splitting\":{\"percentBegin\":10,\"percentEnd\":60}}"</code>
               </p>
            </li>
         </ul> |
| `role_arn` | String | ✅ | <p>The role that Amazon ML assumes on behalf of the user to create and activate a data
          pipeline in the user's account and copy data using the <code>SelectSqlQuery</code> query from Amazon RDS to Amazon S3.</p>
        <p></p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_source_from_rds
data_source_from_rds = provider.machine_learning.Data_source_from_rds {
    data_source_id = "value"  # <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>. Typically, an Amazon Resource Number (ARN) 
            becomes the ID for a <code>DataSource</code>.</p>
    rds_data = "value"  # <p>The data specification of an Amazon RDS <code>DataSource</code>:</p>
        <ul>
            <li>
               <p>DatabaseInformation -</p>
                <ul>
                  <li>
                     <p>
                        <code>DatabaseName</code> - The name of the Amazon RDS database.</p>
                  </li>
                  <li>
                     <p>
                        <code>InstanceIdentifier </code> - A unique identifier for the Amazon RDS database instance.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>DatabaseCredentials - AWS Identity and Access Management (IAM) credentials that are used to connect to the Amazon RDS database.</p>
            </li>
            <li>
               <p>ResourceRole - A role (DataPipelineDefaultResourceRole) assumed by an EC2 instance to carry out the copy task from Amazon RDS to Amazon 
			Simple Storage Service (Amazon S3). For more information, see <a href="https://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
            </li>
            <li>
               <p>ServiceRole - A role (DataPipelineDefaultRole) assumed by the AWS Data Pipeline service to monitor the progress of the copy task from Amazon RDS
			to Amazon S3. For more information, see <a href="https://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
            </li>
            <li>
               <p>SecurityInfo - The security information to use to access an RDS DB instance. You need to set up appropriate ingress rules for the security entity IDs provided to allow access to the Amazon RDS instance. Specify a [<code>SubnetId</code>, <code>SecurityGroupIds</code>] pair for a VPC-based RDS DB instance.</p>
            </li>
            <li>
               <p>SelectSqlQuery - A query that is used to retrieve the observation data for the <code>Datasource</code>.</p>
            </li>
            <li>
               <p>S3StagingLocation - The Amazon S3 location for staging Amazon RDS data. The data retrieved from Amazon RDS using <code>SelectSqlQuery</code> is stored in this location.</p>
            </li>
            <li>
               <p>DataSchemaUri - The Amazon S3 location of the <code>DataSchema</code>.</p>
            </li>
            <li>
               <p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p>
            </li>
            <li>
               <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>Datasource</code>. </p>
               <p> Sample - 
            <code> "{\"splitting\":{\"percentBegin\":10,\"percentEnd\":60}}"</code>
               </p>
            </li>
         </ul>
    role_arn = "value"  # <p>The role that Amazon ML assumes on behalf of the user to create and activate a data
          pipeline in the user's account and copy data using the <code>SelectSqlQuery</code> query from Amazon RDS to Amazon S3.</p>
        <p></p>
}

```

---


### Batch_predictions

BatchPredictions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `results` | Vec<String> | <p>A list of <code>BatchPrediction</code> objects that meet the search criteria.
        </p> |
| `next_token` | String | <p>The ID of the next page in the paginated results that indicates at least one more page follows.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access batch_predictions outputs
batch_predictions_id = batch_predictions.id
batch_predictions_results = batch_predictions.results
batch_predictions_next_token = batch_predictions.next_token
```

---


### Data_source_from_s3

DataSourceFromS3 resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `compute_statistics` | bool |  | <p>The compute statistics for a <code>DataSource</code>. The statistics are generated from the observation data referenced by 
            a <code>DataSource</code>. Amazon ML uses the statistics internally during <code>MLModel</code> training.
            This parameter must be set to <code>true</code> if the <code></code>DataSource<code></code> needs to be used for <code>MLModel</code> training.</p> |
| `data_source_id` | String | ✅ | <p>A user-supplied identifier that uniquely identifies the <code>DataSource</code>. </p> |
| `data_source_name` | String |  | <p>A user-supplied name or description of the <code>DataSource</code>. </p> |
| `data_spec` | String | ✅ | <p>The data specification of a <code>DataSource</code>:</p>
        <ul>
            <li>
               <p>DataLocationS3 - The Amazon S3 location of the observation data.</p>
            </li>
            <li>
               <p>DataSchemaLocationS3 - The Amazon S3 location of the <code>DataSchema</code>.</p>
            </li>
            <li>
               <p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p>
            </li>
            <li>
               <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>Datasource</code>. </p>
               <p> Sample - 
            <code> "{\"splitting\":{\"percentBegin\":10,\"percentEnd\":60}}"</code>
               </p>
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

# Create data_source_from_s3
data_source_from_s3 = provider.machine_learning.Data_source_from_s3 {
    data_source_id = "value"  # <p>A user-supplied identifier that uniquely identifies the <code>DataSource</code>. </p>
    data_spec = "value"  # <p>The data specification of a <code>DataSource</code>:</p>
        <ul>
            <li>
               <p>DataLocationS3 - The Amazon S3 location of the observation data.</p>
            </li>
            <li>
               <p>DataSchemaLocationS3 - The Amazon S3 location of the <code>DataSchema</code>.</p>
            </li>
            <li>
               <p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p>
            </li>
            <li>
               <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>Datasource</code>. </p>
               <p> Sample - 
            <code> "{\"splitting\":{\"percentBegin\":10,\"percentEnd\":60}}"</code>
               </p>
            </li>
         </ul>
}

```

---


### Ml_models

MLModels resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `results` | Vec<String> | <p>A list of <code>MLModel</code> that meet the search criteria.</p> |
| `next_token` | String | <p>The ID of the next page in the paginated results that indicates at least one more page follows.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ml_models outputs
ml_models_id = ml_models.id
ml_models_results = ml_models.results
ml_models_next_token = ml_models.next_token
```

---


### Evaluation

Evaluation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluation_id` | String | ✅ | <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code>.</p> |
| `evaluation_data_source_id` | String | ✅ | <p>The ID of the <code>DataSource</code> for the evaluation. The schema of the <code>DataSource</code> 
            must match the schema used to create the <code>MLModel</code>.</p> |
| `ml_model_id` | String | ✅ | <p>The ID of the <code>MLModel</code> to evaluate.</p> 
        <p>The schema used in creating the <code>MLModel</code> must match the schema of the <code>DataSource</code> used in the <code>Evaluation</code>.</p> |
| `evaluation_name` | String |  | <p>A user-supplied name or description of the <code>Evaluation</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_at` | String | <p>The time of the most recent edit to the <code>Evaluation</code>. The time is expressed in epoch time.</p> |
| `ml_model_id` | String | <p>The ID of the <code>MLModel</code> that was the focus of the evaluation.</p> |
| `evaluation_data_source_id` | String | <p>The <code>DataSource</code> used for this evaluation.</p> |
| `status` | String | <p>The status of the evaluation. This element can have one of the following values:</p>
        <ul>
            <li>
               <p>
                  <code>PENDING</code> - Amazon Machine Language (Amazon ML) submitted a request to evaluate an <code>MLModel</code>.</p>
            </li>
            <li>
               <p>
                  <code>INPROGRESS</code> - The evaluation is underway.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The request to evaluate an <code>MLModel</code> did not run to completion. It is not usable.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED</code> - The evaluation process completed successfully.</p>
            </li>
            <li>
               <p>
                  <code>DELETED</code> - The <code>Evaluation</code> is marked as deleted. It is not usable.</p>
            </li>
         </ul> |
| `input_data_location_s3` | String | <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p> |
| `finished_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>Evaluation</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>Evaluation</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p> |
| `performance_metrics` | String | <p>Measurements of how well the <code>MLModel</code> performed using observations referenced by the <code>DataSource</code>. One of the following metric is returned based on the type of the <code>MLModel</code>: 
        </p>
        <ul>
            <li>
               <p>BinaryAUC: A binary <code>MLModel</code> uses the Area Under the Curve (AUC) technique to measure performance. </p>
            </li>
            <li>
               <p>RegressionRMSE: A regression <code>MLModel</code> uses the Root Mean Square Error (RMSE) technique to measure performance. RMSE measures the difference between predicted and actual values for a single variable.</p> 
            </li>
            <li>
               <p>MulticlassAvgFScore: A multiclass <code>MLModel</code> uses the F1 score technique to measure performance. </p>
            </li>
         </ul>
        <p>
                        For more information about performance metrics, please see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.
        </p> |
| `created_at` | String | <p>The time that the <code>Evaluation</code> was created. The time is expressed in epoch time.</p> |
| `compute_time` | i64 | <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>Evaluation</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>Evaluation</code> is in the <code>COMPLETED</code> state.</p> |
| `name` | String | <p>A user-supplied name or description of the <code>Evaluation</code>. </p> |
| `message` | String | <p>A description of the most recent details about evaluating the <code>MLModel</code>.</p> |
| `started_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>Evaluation</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>Evaluation</code> is in the <code>PENDING</code> state.</p> |
| `evaluation_id` | String | <p>The evaluation ID which is same as the <code>EvaluationId</code> in the request.</p> |
| `created_by_iam_user` | String | <p>The AWS user account that invoked the evaluation. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p> |
| `log_uri` | String | <p>A link to the file that contains logs of the <code>CreateEvaluation</code> operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create evaluation
evaluation = provider.machine_learning.Evaluation {
    evaluation_id = "value"  # <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code>.</p>
    evaluation_data_source_id = "value"  # <p>The ID of the <code>DataSource</code> for the evaluation. The schema of the <code>DataSource</code> 
            must match the schema used to create the <code>MLModel</code>.</p>
    ml_model_id = "value"  # <p>The ID of the <code>MLModel</code> to evaluate.</p> 
        <p>The schema used in creating the <code>MLModel</code> must match the schema of the <code>DataSource</code> used in the <code>Evaluation</code>.</p>
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_last_updated_at = evaluation.last_updated_at
evaluation_ml_model_id = evaluation.ml_model_id
evaluation_evaluation_data_source_id = evaluation.evaluation_data_source_id
evaluation_status = evaluation.status
evaluation_input_data_location_s3 = evaluation.input_data_location_s3
evaluation_finished_at = evaluation.finished_at
evaluation_performance_metrics = evaluation.performance_metrics
evaluation_created_at = evaluation.created_at
evaluation_compute_time = evaluation.compute_time
evaluation_name = evaluation.name
evaluation_message = evaluation.message
evaluation_started_at = evaluation.started_at
evaluation_evaluation_id = evaluation.evaluation_id
evaluation_created_by_iam_user = evaluation.created_by_iam_user
evaluation_log_uri = evaluation.log_uri
```

---


### Batch_prediction

BatchPrediction resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_uri` | String | ✅ | <p>The location of an Amazon Simple Storage Service (Amazon S3) bucket or directory to store the batch prediction results. The following substrings are not allowed in the <code>s3 key</code> portion of the <code>outputURI</code> field: ':', '//', '/./', '/../'.</p> 
        <p>Amazon ML needs permissions to store and retrieve the logs on your behalf. For information about how to set permissions, see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p> |
| `batch_prediction_id` | String | ✅ | <p>A user-supplied ID that uniquely identifies the
                <code>BatchPrediction</code>.</p> |
| `ml_model_id` | String | ✅ | <p>The ID of the <code>MLModel</code> that will generate predictions for the group of observations. </p> |
| `batch_prediction_data_source_id` | String | ✅ | <p>The ID of the <code>DataSource</code> that points to the group of observations to predict.</p> |
| `batch_prediction_name` | String |  | <p>A user-supplied name or description of the <code>BatchPrediction</code>. <code>BatchPredictionName</code> can only use the UTF-8 character set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `output_uri` | String | <p>The location of an Amazon S3 bucket or directory to receive the operation results.</p> |
| `compute_time` | i64 | <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>BatchPrediction</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>BatchPrediction</code> is in the <code>COMPLETED</code> state.</p> |
| `batch_prediction_id` | String | <p>An ID assigned to the <code>BatchPrediction</code> at creation. This value should be identical to the value of the <code>BatchPredictionID</code> 
            in the request.</p> |
| `name` | String | <p>A user-supplied name or description of the <code>BatchPrediction</code>.</p> |
| `created_at` | String | <p>The time when the <code>BatchPrediction</code> was created. The time is expressed in epoch time.</p> |
| `status` | String | <p>The status of the <code>BatchPrediction</code>, which can be one of the following values:</p>
        <ul>
            <li>
               <p>
                  <code>PENDING</code> - Amazon Machine Learning (Amazon ML) submitted a request to generate batch predictions.</p>
            </li>
            <li>
               <p>
                  <code>INPROGRESS</code> - The batch predictions are in progress.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The request to perform a batch prediction did not run to completion. It is not usable.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED</code> - The batch prediction process completed successfully.</p>
            </li>
            <li>
               <p>
                  <code>DELETED</code> - The <code>BatchPrediction</code> is marked as deleted. It is not usable.</p>
            </li>
         </ul> |
| `ml_model_id` | String | <p>The ID of the <code>MLModel</code> that generated predictions for the <code>BatchPrediction</code> request.</p> |
| `batch_prediction_data_source_id` | String | <p>The ID of the <code>DataSource</code> that was used to create the <code>BatchPrediction</code>.
        </p> |
| `input_data_location_s3` | String | <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p> |
| `log_uri` | String | <p>A link to the file that contains logs of the <code>CreateBatchPrediction</code> operation.</p> |
| `finished_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>BatchPrediction</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>BatchPrediction</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p> |
| `started_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>BatchPrediction</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>BatchPrediction</code> is in the <code>PENDING</code> state.</p> |
| `total_record_count` | i64 | <p>The number of total records that Amazon Machine Learning saw while processing the <code>BatchPrediction</code>.</p> |
| `created_by_iam_user` | String | <p>The AWS user account that invoked the <code>BatchPrediction</code>. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p> |
| `last_updated_at` | String | <p>The time of the most recent edit to <code>BatchPrediction</code>. The time is expressed in epoch time.</p> |
| `invalid_record_count` | i64 | <p>The number of invalid records that Amazon Machine Learning saw while processing the <code>BatchPrediction</code>.</p> |
| `message` | String | <p>A description of the most recent details about processing the batch prediction request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create batch_prediction
batch_prediction = provider.machine_learning.Batch_prediction {
    output_uri = "value"  # <p>The location of an Amazon Simple Storage Service (Amazon S3) bucket or directory to store the batch prediction results. The following substrings are not allowed in the <code>s3 key</code> portion of the <code>outputURI</code> field: ':', '//', '/./', '/../'.</p> 
        <p>Amazon ML needs permissions to store and retrieve the logs on your behalf. For information about how to set permissions, see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p>
    batch_prediction_id = "value"  # <p>A user-supplied ID that uniquely identifies the
                <code>BatchPrediction</code>.</p>
    ml_model_id = "value"  # <p>The ID of the <code>MLModel</code> that will generate predictions for the group of observations. </p>
    batch_prediction_data_source_id = "value"  # <p>The ID of the <code>DataSource</code> that points to the group of observations to predict.</p>
}

# Access batch_prediction outputs
batch_prediction_id = batch_prediction.id
batch_prediction_output_uri = batch_prediction.output_uri
batch_prediction_compute_time = batch_prediction.compute_time
batch_prediction_batch_prediction_id = batch_prediction.batch_prediction_id
batch_prediction_name = batch_prediction.name
batch_prediction_created_at = batch_prediction.created_at
batch_prediction_status = batch_prediction.status
batch_prediction_ml_model_id = batch_prediction.ml_model_id
batch_prediction_batch_prediction_data_source_id = batch_prediction.batch_prediction_data_source_id
batch_prediction_input_data_location_s3 = batch_prediction.input_data_location_s3
batch_prediction_log_uri = batch_prediction.log_uri
batch_prediction_finished_at = batch_prediction.finished_at
batch_prediction_started_at = batch_prediction.started_at
batch_prediction_total_record_count = batch_prediction.total_record_count
batch_prediction_created_by_iam_user = batch_prediction.created_by_iam_user
batch_prediction_last_updated_at = batch_prediction.last_updated_at
batch_prediction_invalid_record_count = batch_prediction.invalid_record_count
batch_prediction_message = batch_prediction.message
```

---


### Realtime_endpoint

RealtimeEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ml_model_id` | String | ✅ | <p>The ID assigned to the <code>MLModel</code> during creation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create realtime_endpoint
realtime_endpoint = provider.machine_learning.Realtime_endpoint {
    ml_model_id = "value"  # <p>The ID assigned to the <code>MLModel</code> during creation.</p>
}

```

---


### Evaluations

Evaluations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `results` | Vec<String> | <p>A list of <code>Evaluation</code> that meet the search criteria.
        </p> |
| `next_token` | String | <p>The ID of the next page in the paginated results that indicates at least one more page follows.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access evaluations outputs
evaluations_id = evaluations.id
evaluations_results = evaluations.results
evaluations_next_token = evaluations.next_token
```

---


### Data_sources

DataSources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `results` | Vec<String> | <p>A list of <code>DataSource</code> that meet the search criteria.
         </p> |
| `next_token` | String | <p>An ID of the next page in the paginated results that indicates at least one more page follows.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_sources outputs
data_sources_id = data_sources.id
data_sources_results = data_sources.results
data_sources_next_token = data_sources.next_token
```

---


### Ml_model

MLModel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `training_data_source_id` | String | ✅ | <p>The <code>DataSource</code> that points to the training data.</p> |
| `ml_model_id` | String | ✅ | <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>.</p> |
| `parameters` | HashMap<String, String> |  | <p>A list of the training parameters in the <code>MLModel</code>. The list is implemented as
            a map of key-value pairs.</p>
        <p>The following is the current set of training parameters:</p>
        <ul>
            <li>
               <p>
                  <code>sgd.maxMLModelSizeInBytes</code> - The maximum allowed size of the model. Depending on the
                    input data, the size of the model might affect its performance.</p> 
                <p> The value is an integer that ranges from <code>100000</code> to <code>2147483648</code>. The default value is <code>33554432</code>.</p>
            </li>
            <li>
               <p>
                  <code>sgd.maxPasses</code> - The number of times that the training process traverses the
                    observations to build the <code>MLModel</code>. The value is an integer that
                    ranges from <code>1</code> to <code>10000</code>. The default value is
                        <code>10</code>.</p>
            </li>
            <li>
                <p>
                  <code>sgd.shuffleType</code> - Whether Amazon ML shuffles the training data. Shuffling
                    the data improves a model's ability to find the optimal solution for a variety
                    of data types. The valid values are <code>auto</code> and <code>none</code>. The
                    default value is <code>none</code>. We strongly recommend that you shuffle your data.</p>
            </li>
            <li>
               <p>
                  <code>sgd.l1RegularizationAmount</code> - The coefficient regularization L1 norm. It controls
                    overfitting the data by penalizing large coefficients. This tends to drive
                    coefficients to zero, resulting in a sparse feature set. If you use this
                    parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p>
                <p>The value is a double that ranges from <code>0</code> to <code>MAX_DOUBLE</code>.
                    The default is to not use L1 normalization. This parameter can't be used when
                        <code>L2</code> is specified. Use this parameter sparingly.</p>
            </li>
            <li>
               <p>
                  <code>sgd.l2RegularizationAmount</code> - The coefficient regularization L2 norm. It controls
                    overfitting the data by penalizing large coefficients. This tends to drive
                    coefficients to small, nonzero values. If you use this parameter, start by
                    specifying a small value, such as <code>1.0E-08</code>.</p>
                <p>The value is a double that ranges from <code>0</code> to <code>MAX_DOUBLE</code>.
                    The default is to not use L2 normalization. This parameter can't be used when
                        <code>L1</code> is specified. Use this parameter sparingly.</p>
            </li>
         </ul> |
| `recipe` | String |  | <p>The data recipe for creating the <code>MLModel</code>. You must specify either the recipe
            or its URI. If you don't specify a recipe or its URI, Amazon ML creates a default.</p> |
| `recipe_uri` | String |  | <p>The Amazon Simple Storage Service (Amazon S3) location and file name that contains the <code>MLModel</code> recipe. You must specify either the recipe or its URI. If you don't specify a recipe or its URI, Amazon ML creates a default.</p> |
| `ml_model_name` | String |  | <p>A user-supplied name or description of the <code>MLModel</code>.</p> |
| `ml_model_type` | String | ✅ | <p>The category of supervised learning that this <code>MLModel</code> will address. Choose from the following types:</p>
         <ul>
            <li>
               <p>Choose <code>REGRESSION</code> if the <code>MLModel</code> will be used to predict a numeric value.</p>
            </li>
            <li>
               <p>Choose <code>BINARY</code> if the <code>MLModel</code> result has two possible values.</p>
            </li>
            <li>
               <p>Choose <code>MULTICLASS</code> if the <code>MLModel</code> result has a limited number of values.</p> 
            </li>
         </ul>
        <p> For more information, see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The time that the <code>MLModel</code> was created. The time is expressed in epoch time.</p> |
| `message` | String | <p>A description of the most recent details about accessing the <code>MLModel</code>.</p> |
| `training_data_source_id` | String | <p>The ID of the training <code>DataSource</code>.</p> |
| `created_by_iam_user` | String | <p>The AWS user account from which the <code>MLModel</code> was created. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p> |
| `ml_model_id` | String | <p>The MLModel ID, 
            which is same as the <code>MLModelId</code> in the request.</p> |
| `score_threshold_last_updated_at` | String | <p>The time of the most recent edit to the <code>ScoreThreshold</code>. The time is expressed in epoch time.</p> |
| `schema` | String | <p>The schema used by all of the data files referenced by the <code>DataSource</code>.</p>
        <p>
            <b>Note:</b> This parameter is provided as part of the verbose format.</p> |
| `ml_model_type` | String | <p>Identifies the <code>MLModel</code> category. The following are the available types: </p>
        <ul>
            <li>
               <p>REGRESSION -- Produces a numeric result. For example, "What price should a house be listed at?"</p>
            </li>
            <li>
               <p>BINARY -- Produces one of two possible results. For example, "Is this an e-commerce website?"</p>
            </li>
            <li>
               <p>MULTICLASS -- Produces one of several possible results. For example, "Is this a HIGH, LOW or MEDIUM risk trade?"</p>
            </li>
         </ul> |
| `last_updated_at` | String | <p>The time of the most recent edit to the <code>MLModel</code>. The time is expressed in epoch time.</p> |
| `compute_time` | i64 | <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>MLModel</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>MLModel</code> is in the <code>COMPLETED</code> state.</p> |
| `finished_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>MLModel</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>MLModel</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p> |
| `training_parameters` | HashMap<String, String> | <p>A list of the training parameters in the <code>MLModel</code>. The list is implemented as
            a map of key-value pairs.</p>
        <p>The following is the current set of training parameters:</p>
        <ul>
            <li>
               <p>
                  <code>sgd.maxMLModelSizeInBytes</code> - The maximum allowed size of the model. Depending on the
                    input data, the size of the model might affect its performance.</p> 
                <p> The value is an integer that ranges from <code>100000</code> to <code>2147483648</code>. The default value is <code>33554432</code>.</p>
            </li>
            <li>
               <p>
                  <code>sgd.maxPasses</code> - The number of times that the training process traverses the
                    observations to build the <code>MLModel</code>. The value is an integer that
                    ranges from <code>1</code> to <code>10000</code>. The default value is
                        <code>10</code>.</p>
            </li>
            <li>
               <p>
                  <code>sgd.shuffleType</code> - Whether Amazon ML shuffles the training data. Shuffling data improves a
                    model's ability to find the optimal solution for a variety of data types. The
                    valid values are <code>auto</code> and <code>none</code>. The default value is
                        <code>none</code>. We strongly recommend that you shuffle your data.</p>
            </li>
            <li>
               <p>
                  <code>sgd.l1RegularizationAmount</code> - The coefficient regularization L1 norm. It controls
                    overfitting the data by penalizing large coefficients. This tends to drive
                    coefficients to zero, resulting in a sparse feature set. If you use this
                    parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p>
                <p>The value is a double that ranges from <code>0</code> to <code>MAX_DOUBLE</code>.
                    The default is to not use L1 normalization. This parameter can't be used when
                        <code>L2</code> is specified. Use this parameter sparingly.</p>
            </li>
            <li>
               <p>
                  <code>sgd.l2RegularizationAmount</code> - The coefficient regularization L2 norm. It controls
                    overfitting the data by penalizing large coefficients. This tends to drive
                    coefficients to small, nonzero values. If you use this parameter, start by
                    specifying a small value, such as <code>1.0E-08</code>.</p>
                <p>The value is a double that ranges from <code>0</code> to <code>MAX_DOUBLE</code>.
                    The default is to not use L2 normalization. This parameter can't be used when
                        <code>L1</code> is specified. Use this parameter sparingly.</p>
            </li>
         </ul> |
| `started_at` | String | <p>The epoch time when Amazon Machine Learning marked the <code>MLModel</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>MLModel</code> is in the <code>PENDING</code> state.</p> |
| `size_in_bytes` | i64 |  |
| `name` | String | <p>A user-supplied name or description of the <code>MLModel</code>.</p> |
| `endpoint_info` | String | <p>The current endpoint of the <code>MLModel</code>
         </p> |
| `input_data_location_s3` | String | <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p> |
| `score_threshold` | String | <p>The scoring threshold is used in binary classification <code>MLModel</code>
            models. It marks the boundary between a positive prediction and a
            negative prediction.</p>
        <p>Output values greater than or equal to the threshold receive a positive result from the MLModel, such as 
            <code>true</code>. Output values less than the threshold receive a negative response from the MLModel, 
            such as <code>false</code>.</p> |
| `log_uri` | String | <p>A link to the file that contains logs of the <code>CreateMLModel</code> operation.</p> |
| `status` | String | <p>The current status of the <code>MLModel</code>. This element can have one of the following values:</p>
        <ul>
            <li>
               <p>
                  <code>PENDING</code> - Amazon Machine Learning (Amazon ML) submitted a request to describe a <code>MLModel</code>.</p>
            </li>
            <li>
               <p>
                  <code>INPROGRESS</code> - The request is processing.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The request did not run to completion. The ML model isn't
                usable.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETED</code> - The request completed successfully.</p>
            </li>
            <li>
               <p>
                  <code>DELETED</code> - The <code>MLModel</code> is marked as deleted. It isn't
                usable.</p>
            </li>
         </ul> |
| `recipe` | String | <p>The recipe to use when training the <code>MLModel</code>. The <code>Recipe</code>
            provides detailed information about the observation data to use during training, and
            manipulations to perform on the observation data during training.</p>
        <p>
            <b>Note:</b> This parameter is provided as part of the verbose format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ml_model
ml_model = provider.machine_learning.Ml_model {
    training_data_source_id = "value"  # <p>The <code>DataSource</code> that points to the training data.</p>
    ml_model_id = "value"  # <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>.</p>
    ml_model_type = "value"  # <p>The category of supervised learning that this <code>MLModel</code> will address. Choose from the following types:</p>
         <ul>
            <li>
               <p>Choose <code>REGRESSION</code> if the <code>MLModel</code> will be used to predict a numeric value.</p>
            </li>
            <li>
               <p>Choose <code>BINARY</code> if the <code>MLModel</code> result has two possible values.</p>
            </li>
            <li>
               <p>Choose <code>MULTICLASS</code> if the <code>MLModel</code> result has a limited number of values.</p> 
            </li>
         </ul>
        <p> For more information, see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p>
}

# Access ml_model outputs
ml_model_id = ml_model.id
ml_model_created_at = ml_model.created_at
ml_model_message = ml_model.message
ml_model_training_data_source_id = ml_model.training_data_source_id
ml_model_created_by_iam_user = ml_model.created_by_iam_user
ml_model_ml_model_id = ml_model.ml_model_id
ml_model_score_threshold_last_updated_at = ml_model.score_threshold_last_updated_at
ml_model_schema = ml_model.schema
ml_model_ml_model_type = ml_model.ml_model_type
ml_model_last_updated_at = ml_model.last_updated_at
ml_model_compute_time = ml_model.compute_time
ml_model_finished_at = ml_model.finished_at
ml_model_training_parameters = ml_model.training_parameters
ml_model_started_at = ml_model.started_at
ml_model_size_in_bytes = ml_model.size_in_bytes
ml_model_name = ml_model.name
ml_model_endpoint_info = ml_model.endpoint_info
ml_model_input_data_location_s3 = ml_model.input_data_location_s3
ml_model_score_threshold = ml_model.score_threshold
ml_model_log_uri = ml_model.log_uri
ml_model_status = ml_model.status
ml_model_recipe = ml_model.recipe
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple data_source_from_redshift resources
data_source_from_redshift_0 = provider.machine_learning.Data_source_from_redshift {
    data_source_id = "value-0"
    role_arn = "value-0"
    data_spec = "value-0"
}
data_source_from_redshift_1 = provider.machine_learning.Data_source_from_redshift {
    data_source_id = "value-1"
    role_arn = "value-1"
    data_spec = "value-1"
}
data_source_from_redshift_2 = provider.machine_learning.Data_source_from_redshift {
    data_source_id = "value-2"
    role_arn = "value-2"
    data_spec = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    data_source_from_redshift = provider.machine_learning.Data_source_from_redshift {
        data_source_id = "production-value"
        role_arn = "production-value"
        data_spec = "production-value"
    }
```

---

## Related Documentation

- [AWS Machine_learning Documentation](https://docs.aws.amazon.com/machine_learning/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
