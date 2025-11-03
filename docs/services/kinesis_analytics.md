# Kinesis_analytics Service



**Resources**: 16

---

## Overview

The kinesis_analytics service provides access to 16 resource types:

- [Application_maintenance_configuration](#application_maintenance_configuration) [U]
- [Application_presigned_url](#application_presigned_url) [C]
- [Application_output](#application_output) [D]
- [Application_version](#application_version) [R]
- [Application_operation](#application_operation) [R]
- [Application_cloud_watch_logging_option](#application_cloud_watch_logging_option) [D]
- [Application_reference_data_source](#application_reference_data_source) [D]
- [Application_input_processing_configuration](#application_input_processing_configuration) [D]
- [Application_snapshot](#application_snapshot) [CRD]
- [Application_vpc_configuration](#application_vpc_configuration) [D]
- [Application](#application) [CRUD]
- [Application_output](#application_output) [D]
- [Application_reference_data_source](#application_reference_data_source) [D]
- [Application](#application) [CRUD]
- [Application_cloud_watch_logging_option](#application_cloud_watch_logging_option) [D]
- [Application_input_processing_configuration](#application_input_processing_configuration) [D]

---

## Resources


### Application_maintenance_configuration

ApplicationMaintenanceConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_name` | String | ✅ | <p>The name of the application for which you want to update the maintenance configuration.</p> |
| `application_maintenance_configuration_update` | String | ✅ | <p>Describes the application maintenance configuration update.</p> |



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


### Application_presigned_url

ApplicationPresignedUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_name` | String | ✅ | <p>The name of the application.</p> |
| `url_type` | String | ✅ | <p>The type of the extension for which to create and return a URL. Currently, the only valid
            extension URL type is <code>FLINK_DASHBOARD_URL</code>. </p> |
| `session_expiration_duration_in_seconds` | i64 |  | <p>The duration in seconds for which the returned URL will be valid.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_presigned_url
application_presigned_url = provider.kinesis_analytics.Application_presigned_url {
    application_name = "value"  # <p>The name of the application.</p>
    url_type = "value"  # <p>The type of the extension for which to create and return a URL. Currently, the only valid
            extension URL type is <code>FLINK_DASHBOARD_URL</code>. </p>
}

```

---


### Application_output

ApplicationOutput resource

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


### Application_version

ApplicationVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_version_detail` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_version outputs
application_version_id = application_version.id
application_version_application_version_detail = application_version.application_version_detail
```

---


### Application_operation

ApplicationOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_operation_info_details` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_operation outputs
application_operation_id = application_operation.id
application_operation_application_operation_info_details = application_operation.application_operation_info_details
```

---


### Application_cloud_watch_logging_option

ApplicationCloudWatchLoggingOption resource

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


### Application_reference_data_source

ApplicationReferenceDataSource resource

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


### Application_input_processing_configuration

ApplicationInputProcessingConfiguration resource

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


### Application_snapshot

ApplicationSnapshot resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_name` | String | ✅ | <p>The name of an existing application</p> |
| `snapshot_name` | String | ✅ | <p>An identifier for the application snapshot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshot_details` | String | <p>An object containing information about the application snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_snapshot
application_snapshot = provider.kinesis_analytics.Application_snapshot {
    application_name = "value"  # <p>The name of an existing application</p>
    snapshot_name = "value"  # <p>An identifier for the application snapshot.</p>
}

# Access application_snapshot outputs
application_snapshot_id = application_snapshot.id
application_snapshot_snapshot_details = application_snapshot.snapshot_details
```

---


### Application_vpc_configuration

ApplicationVpcConfiguration resource

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


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `runtime_environment` | String | ✅ | <p>The runtime environment for the application.</p> |
| `application_configuration` | String |  | <p>Use this parameter to configure the application.</p> |
| `tags` | Vec<String> |  | <p>A list of one or more tags to assign to the application. A tag is a key-value pair that identifies an 
        application. Note that the maximum number of application tags includes system tags. The maximum number of 
        user-defined application tags is 50.
        For more information, see 
        <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p> |
| `application_mode` | String |  | <p>Use the <code>STREAMING</code> mode to create a Managed Service for Apache Flink application. To create a Managed Service for Apache Flink Studio notebook, use the 
    <code>INTERACTIVE</code> mode.</p> |
| `cloud_watch_logging_options` | Vec<String> |  | <p>Use this parameter to configure an Amazon CloudWatch log stream to monitor application
      configuration errors.
      </p> |
| `application_description` | String |  | <p>A summary description of the application.</p> |
| `application_name` | String | ✅ | <p>The name of your application (for example, <code>sample-app</code>).</p> |
| `service_execution_role` | String | ✅ | <p>The IAM role used by the application to access Kinesis data streams, Kinesis Data Firehose
      delivery streams, Amazon S3 objects, and other external resources.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_detail` | String | <p>Provides a description of the application, such as the application's Amazon Resource Name
      (ARN), status, and latest version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.kinesis_analytics.Application {
    runtime_environment = "value"  # <p>The runtime environment for the application.</p>
    application_name = "value"  # <p>The name of your application (for example, <code>sample-app</code>).</p>
    service_execution_role = "value"  # <p>The IAM role used by the application to access Kinesis data streams, Kinesis Data Firehose
      delivery streams, Amazon S3 objects, and other external resources.</p>
}

# Access application outputs
application_id = application.id
application_application_detail = application.application_detail
```

---


### Application_output

ApplicationOutput resource

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


### Application_reference_data_source

ApplicationReferenceDataSource resource

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


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_watch_logging_options` | Vec<String> |  | <p>Use this parameter to configure a CloudWatch log stream to monitor application
            configuration errors. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon
                CloudWatch Logs</a>.</p> |
| `application_name` | String | ✅ | <p>Name of your Amazon Kinesis Analytics application (for example, <code>sample-app</code>).</p> |
| `outputs` | Vec<String> |  | <p>You can configure application output to write data from any of the in-application streams to up to three destinations.</p>
        <p>These destinations can be Amazon Kinesis streams, Amazon Kinesis Firehose delivery
            streams, AWS Lambda destinations, or any combination of the three.</p>
        <p>In the configuration, you specify the in-application stream name, the destination stream or Lambda function Amazon Resource Name (ARN), and the format to use when writing data. You must also provide an IAM role that Amazon Kinesis Analytics can assume to write to the 
            destination stream or Lambda function on your behalf.</p>
        <p>In the output configuration, you also provide the output stream or Lambda function ARN. For stream destinations, you provide the format
            of data in the stream (for example, JSON, CSV). You also must provide an IAM role that
            Amazon Kinesis Analytics can assume to write to the stream or Lambda function on your behalf.</p> |
| `application_description` | String |  | <p>Summary description of the application.</p> |
| `inputs` | Vec<String> |  | <p>Use this parameter to configure the application input.</p>
        <p>You can configure your application to receive input from a single streaming source. In this configuration, you map this streaming source to an in-application stream that is created. Your application code can then query the in-application stream like a table (you can think of it as a constantly updating table).</p>
        <p>For the streaming source, you provide its Amazon Resource Name (ARN) and format of
            data on the stream (for example, JSON, CSV, etc.). You also must provide an IAM role
            that Amazon Kinesis Analytics can assume to read this stream on your behalf.</p>
        <p>To create the in-application stream, you need to specify a schema to transform your data into a schematized version used in SQL. In the schema, you provide the necessary mapping of the data elements in the streaming source to record columns in the in-app stream.</p> |
| `application_code` | String |  | <p>One or more SQL statements that read input data, transform it, and generate output.
            For example, you can write a SQL statement that reads data from one in-application
            stream, generates a running average of the number of advertisement clicks by vendor, and
            insert resulting rows in another in-application stream using pumps. For more information
            about the typical pattern, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-app-code.html">Application
                Code</a>. </p>
        
        <p>You can provide such series of SQL statements, 
            where output of one statement can be
            used as the input for the next statement. You store 
            intermediate results by creating in-application streams and pumps.</p>
        
        <p>Note that the application code must create the streams with names 
            specified in the <code>Outputs</code>. For example, 
            if your <code>Outputs</code> defines output streams named 
            <code>ExampleOutputStream1</code> and <code>ExampleOutputStream2</code>,
            then your application code must create these streams. </p> |
| `tags` | Vec<String> |  | <p>A list of one or more tags to assign to the application. A tag is a key-value pair that identifies an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.
        For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-tagging.html">Using Tagging</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_detail` | String | <p>Provides a description of the application, such as the application Amazon Resource Name (ARN), status, latest version, and input and output configuration details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.kinesis_analytics.Application {
    application_name = "value"  # <p>Name of your Amazon Kinesis Analytics application (for example, <code>sample-app</code>).</p>
}

# Access application outputs
application_id = application.id
application_application_detail = application.application_detail
```

---


### Application_cloud_watch_logging_option

ApplicationCloudWatchLoggingOption resource

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


### Application_input_processing_configuration

ApplicationInputProcessingConfiguration resource

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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple application_maintenance_configuration resources
application_maintenance_configuration_0 = provider.kinesis_analytics.Application_maintenance_configuration {
    application_name = "value-0"
    application_maintenance_configuration_update = "value-0"
}
application_maintenance_configuration_1 = provider.kinesis_analytics.Application_maintenance_configuration {
    application_name = "value-1"
    application_maintenance_configuration_update = "value-1"
}
application_maintenance_configuration_2 = provider.kinesis_analytics.Application_maintenance_configuration {
    application_name = "value-2"
    application_maintenance_configuration_update = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    application_maintenance_configuration = provider.kinesis_analytics.Application_maintenance_configuration {
        application_name = "production-value"
        application_maintenance_configuration_update = "production-value"
    }
```

---

## Related Documentation

- [AWS Kinesis_analytics Documentation](https://docs.aws.amazon.com/kinesis_analytics/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
