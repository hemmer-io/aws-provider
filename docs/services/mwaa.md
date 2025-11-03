# Mwaa Service



**Resources**: 3

---

## Overview

The mwaa service provides access to 3 resource types:

- [Web_login_token](#web_login_token) [C]
- [Cli_token](#cli_token) [C]
- [Environment](#environment) [CRUD]

---

## Resources


### Web_login_token

WebLoginToken resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the Amazon MWAA environment. For example, <code>MyMWAAEnvironment</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create web_login_token
web_login_token = provider.mwaa.Web_login_token {
    name = "value"  # <p>The name of the Amazon MWAA environment. For example, <code>MyMWAAEnvironment</code>.</p>
}

```

---


### Cli_token

CliToken resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the Amazon MWAA environment. For example, <code>MyMWAAEnvironment</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cli_token
cli_token = provider.mwaa.Cli_token {
    name = "value"  # <p>The name of the Amazon MWAA environment. For example, <code>MyMWAAEnvironment</code>.</p>
}

```

---


### Environment

Environment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `startup_script_s3_object_version` | String |  | <p>The version of the startup shell script in your Amazon S3 bucket. You must specify the <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/versioning-workflows.html">version ID</a> that Amazon S3 assigns to the file
            every time you update the script.
        </p>
         <p>
            Version IDs are Unicode, UTF-8 encoded, URL-ready, opaque strings that are no more than 1,024 bytes long. The following is an example:
        </p>
         <p>
            <code>3sL4kqtJlcpXroDTDmJ+rmSpXd3dIbrHY+MTRCxf3vjVBH40Nr8X8gdRQBpUMLUo</code>
         </p>
         <p>
            For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/using-startup-script.html">Using a startup script</a>.
        </p> |
| `environment_class` | String |  | <p>The environment class type. Valid values: <code>mw1.micro</code>, <code>mw1.small</code>, <code>mw1.medium</code>, <code>mw1.large</code>, <code>mw1.xlarge</code>, and <code>mw1.2xlarge</code>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/environment-class.html">Amazon MWAA environment class</a>.</p> |
| `tags` | HashMap<String, String> |  | <p>The key-value tag pairs you want to associate to your environment. For example, <code>"Environment": "Staging"</code>. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p> |
| `min_workers` | i64 |  | <p>The minimum number of workers that you want to run in your environment. MWAA scales the number of Apache Airflow workers up to the number you specify in the <code>MaxWorkers</code> field. When there are no more tasks running, and no more in the queue, MWAA disposes of the extra workers leaving the worker count you specify in the <code>MinWorkers</code> field. For example, <code>2</code>.</p> |
| `schedulers` | i64 |  | <p>The number of Apache Airflow schedulers to run in your environment. Valid values:</p>
         <ul>
            <li>
               <p>v2 - For environments larger than mw1.micro, accepts values from
                        <code>2</code> to <code>5</code>. Defaults to <code>2</code> for all
                    environment sizes except mw1.micro, which defaults to <code>1</code>.</p>
            </li>
            <li>
               <p>v1 - Accepts <code>1</code>.</p>
            </li>
         </ul> |
| `plugins_s3_object_version` | String |  | <p>The version of the plugins.zip file on your Amazon S3 bucket. You must specify a version each time a plugins.zip file is updated. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/versioning-workflows.html">How S3 Versioning works</a>.</p> |
| `airflow_configuration_options` | HashMap<String, String> |  | <p>A list of key-value pairs containing the Apache Airflow configuration options you want to attach to your environment. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-env-variables.html">Apache Airflow configuration options</a>.</p> |
| `webserver_access_mode` | String |  | <p>Defines the access mode for the Apache Airflow <i>web server</i>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-networking.html">Apache Airflow access modes</a>.</p> |
| `requirements_s3_object_version` | String |  | <p>The version of the <code>requirements.txt</code> file on your Amazon S3 bucket. You must specify a version each time a requirements.txt file is updated. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/versioning-workflows.html">How S3 Versioning works</a>.</p> |
| `weekly_maintenance_window_start` | String |  | <p>The day and time of the week in Coordinated Universal Time (UTC) 24-hour standard time to start weekly maintenance updates of your environment in the following format: <code>DAY:HH:MM</code>. For example: <code>TUE:03:30</code>. You can specify a start time in 30 minute increments only.</p> |
| `min_webservers` | i64 |  | <p>
            The minimum number of web servers that you want to run in your environment.
            Amazon MWAA scales the number of Apache Airflow web servers up to the number you specify for <code>MaxWebservers</code>
            when you interact with your Apache Airflow environment using Apache Airflow REST API, or
            the Apache Airflow CLI. As the transaction-per-second rate, and the network load, decrease,
            Amazon MWAA disposes of the additional web servers, and scales down to the number set in <code>MinxWebserers</code>.
        </p>
         <p>Valid values: For environments larger than mw1.micro, accepts values from
                <code>2</code> to <code>5</code>. Defaults to <code>2</code> for all environment
            sizes except mw1.micro, which defaults to <code>1</code>.</p> |
| `airflow_version` | String |  | <p>The Apache Airflow version for your environment. If no value is specified, it defaults to the latest version.
            For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/airflow-versions.html">Apache Airflow versions on Amazon Managed Workflows for Apache Airflow (Amazon MWAA)</a>.</p>
         <p>Valid values: <code>1.10.12</code>, <code>2.0.2</code>, <code>2.2.2</code>,
                <code>2.4.3</code>, <code>2.5.1</code>, <code>2.6.3</code>, <code>2.7.2</code>,
            <code>2.8.1</code>, <code>2.9.2</code>, <code>2.10.1</code>, and <code>2.10.3</code>.</p> |
| `max_workers` | i64 |  | <p>The maximum number of workers that you want to run in your environment. MWAA scales the number of Apache Airflow workers up to the number you specify in the <code>MaxWorkers</code> field. For example, <code>20</code>. When there are no more tasks running, and no more in the queue, MWAA disposes of the extra workers leaving the one worker that is included with your environment, or the number you specify in <code>MinWorkers</code>.</p> |
| `logging_configuration` | String |  | <p>Defines the Apache Airflow logs to send to CloudWatch Logs.</p> |
| `source_bucket_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Amazon S3 bucket where your DAG code and supporting files are stored. For example, <code>arn:aws:s3:::my-airflow-bucket-unique-name</code>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/mwaa-s3-bucket.html">Create an Amazon S3 bucket for Amazon MWAA</a>.</p> |
| `startup_script_s3_path` | String |  | <p>The relative path to the startup shell script in your Amazon S3 bucket. For example, <code>s3://mwaa-environment/startup.sh</code>.</p>
         <p>
            Amazon MWAA runs the script as your environment starts, and before running the Apache Airflow process.
            You can use this script to install dependencies, modify Apache Airflow configuration options, and set environment variables. For more information, see
            <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/using-startup-script.html">Using a startup script</a>.
        </p> |
| `max_webservers` | i64 |  | <p>
            The maximum number of web servers that you want to run in your environment.
            Amazon MWAA scales the number of Apache Airflow web servers up to the number you specify for <code>MaxWebservers</code>
            when you interact with your Apache Airflow environment using Apache Airflow REST API, or
            the Apache Airflow CLI. For example, in scenarios where your workload requires network calls to the Apache Airflow REST API with a high transaction-per-second (TPS)
            rate, Amazon MWAA will increase the number of web servers up to the number set in <code>MaxWebserers</code>. As TPS rates decrease
            Amazon MWAA disposes of the additional web servers, and scales down to the number set in <code>MinxWebserers</code>.
        </p>
         <p>Valid values: For environments larger than mw1.micro, accepts values from
                <code>2</code> to <code>5</code>. Defaults to <code>2</code> for all environment
            sizes except mw1.micro, which defaults to <code>1</code>.</p> |
| `requirements_s3_path` | String |  | <p>The relative path to the <code>requirements.txt</code> file on your Amazon S3 bucket. For example, <code>requirements.txt</code>. If specified, then a version is required. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/working-dags-dependencies.html">Installing Python dependencies</a>.</p> |
| `name` | String | ✅ | <p>The name of the Amazon MWAA environment. For example, <code>MyMWAAEnvironment</code>.</p> |
| `plugins_s3_path` | String |  | <p>The relative path to the <code>plugins.zip</code> file on your Amazon S3 bucket. For example, <code>plugins.zip</code>. If specified, then the <code>plugins.zip</code> version is required. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-import-plugins.html">Installing custom plugins</a>.</p> |
| `execution_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the execution role for your environment. An execution role is an Amazon Web Services Identity and Access Management (IAM) role that grants MWAA permission to access Amazon Web Services services and resources used by your environment. For example, <code>arn:aws:iam::123456789:role/my-execution-role</code>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/mwaa-create-role.html">Amazon MWAA Execution role</a>.</p> |
| `endpoint_management` | String |  | <p>Defines whether the VPC endpoints configured for the environment are created, and managed, by the customer or by Amazon MWAA. If set to <code>SERVICE</code>, Amazon MWAA will create and manage the required VPC endpoints in
        your VPC. If set to <code>CUSTOMER</code>, you must create, and manage, the VPC endpoints for your VPC. If you choose to create an environment in a shared VPC, you must set this value to <code>CUSTOMER</code>.
        In a shared VPC deployment, the environment will remain in <code>PENDING</code> status until you create the VPC endpoints. If you do not take action to
            create the endpoints within 72 hours, the status will change to <code>CREATE_FAILED</code>. You can delete the failed environment and create a new one.</p> |
| `network_configuration` | String | ✅ | <p>The VPC networking components used to secure and enable network traffic between the Amazon Web Services resources for your environment. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/networking-about.html">About networking on Amazon MWAA</a>.</p> |
| `kms_key` | String |  | <p>The Amazon Web Services Key Management Service (KMS) key to encrypt the data in your environment. You can use an Amazon Web Services owned CMK, or a Customer managed CMK (advanced). For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/create-environment.html">Create an Amazon MWAA environment</a>.</p> |
| `dag_s3_path` | String | ✅ | <p>The relative path to the DAGs folder on your Amazon S3 bucket. For example, <code>dags</code>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-folder.html">Adding or updating DAGs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment` | String | <p>An object containing all available details about the environment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment
environment = provider.mwaa.Environment {
    source_bucket_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Amazon S3 bucket where your DAG code and supporting files are stored. For example, <code>arn:aws:s3:::my-airflow-bucket-unique-name</code>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/mwaa-s3-bucket.html">Create an Amazon S3 bucket for Amazon MWAA</a>.</p>
    name = "value"  # <p>The name of the Amazon MWAA environment. For example, <code>MyMWAAEnvironment</code>.</p>
    execution_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the execution role for your environment. An execution role is an Amazon Web Services Identity and Access Management (IAM) role that grants MWAA permission to access Amazon Web Services services and resources used by your environment. For example, <code>arn:aws:iam::123456789:role/my-execution-role</code>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/mwaa-create-role.html">Amazon MWAA Execution role</a>.</p>
    network_configuration = "value"  # <p>The VPC networking components used to secure and enable network traffic between the Amazon Web Services resources for your environment. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/networking-about.html">About networking on Amazon MWAA</a>.</p>
    dag_s3_path = "value"  # <p>The relative path to the DAGs folder on your Amazon S3 bucket. For example, <code>dags</code>. For more information, see <a href="https://docs.aws.amazon.com/mwaa/latest/userguide/configuring-dag-folder.html">Adding or updating DAGs</a>.</p>
}

# Access environment outputs
environment_id = environment.id
environment_environment = environment.environment
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple web_login_token resources
web_login_token_0 = provider.mwaa.Web_login_token {
    name = "value-0"
}
web_login_token_1 = provider.mwaa.Web_login_token {
    name = "value-1"
}
web_login_token_2 = provider.mwaa.Web_login_token {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    web_login_token = provider.mwaa.Web_login_token {
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Mwaa Documentation](https://docs.aws.amazon.com/mwaa/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
