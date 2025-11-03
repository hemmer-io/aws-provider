# Synthetics Service



**Resources**: 6

---

## Overview

The synthetics service provides access to 6 resource types:

- [Canaries_last_run](#canaries_last_run) [R]
- [Canaries](#canaries) [R]
- [Group](#group) [CRD]
- [Canary](#canary) [CRUD]
- [Runtime_versions](#runtime_versions) [R]
- [Canary_runs](#canary_runs) [R]

---

## Resources


### Canaries_last_run

CanariesLastRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates that there is more data
         available. You can use this token in a subsequent <code>DescribeCanariesLastRun</code> operation to retrieve the next 
         set of results.</p> |
| `canaries_last_run` | Vec<String> | <p>An array that contains the information from the most recent run of each
         canary.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access canaries_last_run outputs
canaries_last_run_id = canaries_last_run.id
canaries_last_run_next_token = canaries_last_run.next_token
canaries_last_run_canaries_last_run = canaries_last_run.canaries_last_run
```

---


### Canaries

Canaries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `canaries` | Vec<String> | <p>Returns an array. Each item in the array contains the full information about
         one canary.</p> |
| `next_token` | String | <p>A token that indicates that there is more data
         available. You can use this token in a subsequent <code>DescribeCanaries</code> operation to retrieve the next 
         set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access canaries outputs
canaries_id = canaries.id
canaries_canaries = canaries.canaries
canaries_next_token = canaries.next_token
```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name for the group. It can include any Unicode characters.</p>
         <p>The names for all groups in your account, across all Regions, must be unique.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs to associate with the group. 
         You can associate as many as 50 tags with a group.</p>
         <p>Tags can help you organize and categorize your
         resources. You can also use them to scope user permissions, by 
         granting a user permission to access or change only the resources that have
         certain tag values.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group` | String | <p>A structure that contains information about the group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.synthetics.Group {
    name = "value"  # <p>The name for the group. It can include any Unicode characters.</p>
         <p>The names for all groups in your account, across all Regions, must be unique.</p>
}

# Access group outputs
group_id = group.id
group_group = group.group
```

---


### Canary

Canary resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `browser_configs` | Vec<String> |  | <p>CloudWatch Synthetics now supports multibrowser canaries for <code>syn-nodejs-puppeteer-11.0</code> and <code>syn-nodejs-playwright-3.0</code> runtimes. This feature allows you to run your canaries on both 
         Firefox and Chrome browsers. To create a multibrowser canary, you need to specify the BrowserConfigs with a list of browsers you want to use.</p>
         <note>
            <p>If not specified, <code>browserConfigs</code> defaults to Chrome.</p>
         </note> |
| `execution_role_arn` | String | ✅ | <p>The ARN of the IAM role to be used to run the canary. This role must already exist, 
         and must include <code>lambda.amazonaws.com</code> as a principal in the trust
         policy. The role must also have the following permissions:</p>
         <ul>
            <li>
               <p>
                  <code>s3:PutObject</code>
               </p>
            </li>
            <li>
               <p>
                  <code>s3:GetBucketLocation</code>
               </p>
            </li>
            <li>
               <p>
                  <code>s3:ListAllMyBuckets</code>
               </p>
            </li>
            <li>
               <p>
                  <code>cloudwatch:PutMetricData</code>
               </p>
            </li>
            <li>
               <p>
                  <code>logs:CreateLogGroup</code>
               </p>
            </li>
            <li>
               <p>
                  <code>logs:CreateLogStream</code>
               </p>
            </li>
            <li>
               <p>
                  <code>logs:PutLogEvents</code>
               </p>
            </li>
         </ul> |
| `success_retention_period_in_days` | i64 |  | <p>The number of days to retain data about successful runs of this canary. If you omit 
         this field, the default of 31 days is used. The valid range is 1 to 455 days.</p>
         <p>This setting affects the range of information returned by <a href="https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_GetCanaryRuns.html">GetCanaryRuns</a>, as well as 
         the range of information displayed in the Synthetics console.
      </p> |
| `run_config` | String |  | <p>A structure that contains the configuration for individual canary runs, 
         such as timeout value and environment variables.</p>
         <important>
            <p>Environment variable keys and values are encrypted at rest using Amazon Web Services owned KMS keys. However, the environment variables 
         are not encrypted on the client side. Do not store sensitive information
         in them.</p>
         </important> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs to associate with the canary. 
         You can associate as many as 50 tags with a canary.</p>
         <p>Tags can help you organize and categorize your
         resources. You can also use them to scope user permissions, by 
         granting a user permission to access or change only the resources that have
         certain tag values.</p>
         <p>To have the tags that you apply to this canary also be applied to the Lambda function that
         the canary uses, specify this parameter with the value <code>lambda-function</code>.</p> |
| `artifact_s3_location` | String | ✅ | <p>The location in Amazon S3 where Synthetics stores artifacts from the test runs of this
         canary. Artifacts include the log file, screenshots, and HAR files.  The name of the 
         Amazon S3 bucket can't include a period (.).</p> |
| `failure_retention_period_in_days` | i64 |  | <p>The number of days to retain data about failed runs of this canary. If you omit 
         this field, the default of 31 days is used. The valid range is 1 to 455 days.</p>
         <p>This setting affects the range of information returned by <a href="https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_GetCanaryRuns.html">GetCanaryRuns</a>, as well as 
         the range of information displayed in the Synthetics console.
      </p> |
| `runtime_version` | String | ✅ | <p>Specifies the runtime version to use for the canary. For a list of valid
         runtime versions and more information about
         runtime versions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html">
            Canary Runtime Versions</a>.</p> |
| `vpc_config` | String |  | <p>If this canary is to test an endpoint in a VPC, this structure contains
      information about the subnet and security groups of the VPC endpoint. 
      For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_VPC.html">
         Running a Canary in a VPC</a>.</p> |
| `artifact_config` | String |  | <p>A structure that contains the configuration for canary artifacts, including 
         the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3.</p> |
| `provisioned_resource_cleanup` | String |  | <p>Specifies whether to also delete the Lambda functions and layers used by this canary
      when the canary is deleted. If you omit this parameter, the default of <code>AUTOMATIC</code> is used, which means
         that the Lambda functions and layers will be deleted when the canary is deleted.</p>
         <p>If the value of this parameter is <code>OFF</code>, then the value of the <code>DeleteLambda</code> parameter
         of the <a href="https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_DeleteCanary.html">DeleteCanary</a> operation
         determines whether the Lambda functions and layers will be deleted.</p> |
| `resources_to_replicate_tags` | Vec<String> |  | <p>To have the tags that you apply to this canary also be applied to the Lambda function that
         the canary uses, specify this parameter with the value <code>lambda-function</code>.</p>
         <p>If you specify this parameter and don't specify any tags in the <code>Tags</code>
         parameter, the canary creation fails.</p> |
| `name` | String | ✅ | <p>The name for this canary. Be sure to give it a descriptive name 
         that distinguishes it from other canaries in your account.</p>
         <p>Do not include secrets or proprietary information in your canary names. The canary name
         makes up part of the canary ARN, and the ARN is included in outbound calls over the
         internet. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/servicelens_canaries_security.html">Security
            Considerations for Synthetics Canaries</a>.</p> |
| `code` | String | ✅ | <p>A structure that includes the entry point from which the canary should start
         running your script. If the script is stored in 
         an Amazon S3 bucket, the bucket name, key, and version are also included.
      </p> |
| `schedule` | String | ✅ | <p>A structure that contains information about how often the canary is to run and when
         these test runs are to stop.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `canary` | String | <p>A structure that contains the full information about the canary.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create canary
canary = provider.synthetics.Canary {
    execution_role_arn = "value"  # <p>The ARN of the IAM role to be used to run the canary. This role must already exist, 
         and must include <code>lambda.amazonaws.com</code> as a principal in the trust
         policy. The role must also have the following permissions:</p>
         <ul>
            <li>
               <p>
                  <code>s3:PutObject</code>
               </p>
            </li>
            <li>
               <p>
                  <code>s3:GetBucketLocation</code>
               </p>
            </li>
            <li>
               <p>
                  <code>s3:ListAllMyBuckets</code>
               </p>
            </li>
            <li>
               <p>
                  <code>cloudwatch:PutMetricData</code>
               </p>
            </li>
            <li>
               <p>
                  <code>logs:CreateLogGroup</code>
               </p>
            </li>
            <li>
               <p>
                  <code>logs:CreateLogStream</code>
               </p>
            </li>
            <li>
               <p>
                  <code>logs:PutLogEvents</code>
               </p>
            </li>
         </ul>
    artifact_s3_location = "value"  # <p>The location in Amazon S3 where Synthetics stores artifacts from the test runs of this
         canary. Artifacts include the log file, screenshots, and HAR files.  The name of the 
         Amazon S3 bucket can't include a period (.).</p>
    runtime_version = "value"  # <p>Specifies the runtime version to use for the canary. For a list of valid
         runtime versions and more information about
         runtime versions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html">
            Canary Runtime Versions</a>.</p>
    name = "value"  # <p>The name for this canary. Be sure to give it a descriptive name 
         that distinguishes it from other canaries in your account.</p>
         <p>Do not include secrets or proprietary information in your canary names. The canary name
         makes up part of the canary ARN, and the ARN is included in outbound calls over the
         internet. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/servicelens_canaries_security.html">Security
            Considerations for Synthetics Canaries</a>.</p>
    code = "value"  # <p>A structure that includes the entry point from which the canary should start
         running your script. If the script is stored in 
         an Amazon S3 bucket, the bucket name, key, and version are also included.
      </p>
    schedule = "value"  # <p>A structure that contains information about how often the canary is to run and when
         these test runs are to stop.</p>
}

# Access canary outputs
canary_id = canary.id
canary_canary = canary.canary
```

---


### Runtime_versions

RuntimeVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that indicates that there is more data
         available. You can use this token in a subsequent <code>DescribeRuntimeVersions</code> operation to retrieve the next 
         set of results.</p> |
| `runtime_versions` | Vec<String> | <p>An array of objects that display the details about each Synthetics canary runtime
         version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access runtime_versions outputs
runtime_versions_id = runtime_versions.id
runtime_versions_next_token = runtime_versions.next_token
runtime_versions_runtime_versions = runtime_versions.runtime_versions
```

---


### Canary_runs

CanaryRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `canary_runs` | Vec<String> | <p>An array of structures. Each structure contains the details of one of the 
         retrieved canary runs.</p> |
| `next_token` | String | <p>A token that indicates that there is more data
         available. You can use this token in a subsequent <code>GetCanaryRuns</code> 
         operation to retrieve the next 
         set of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access canary_runs outputs
canary_runs_id = canary_runs.id
canary_runs_canary_runs = canary_runs.canary_runs
canary_runs_next_token = canary_runs.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple canaries_last_run resources
canaries_last_run_0 = provider.synthetics.Canaries_last_run {
}
canaries_last_run_1 = provider.synthetics.Canaries_last_run {
}
canaries_last_run_2 = provider.synthetics.Canaries_last_run {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    canaries_last_run = provider.synthetics.Canaries_last_run {
    }
```

---

## Related Documentation

- [AWS Synthetics Documentation](https://docs.aws.amazon.com/synthetics/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
