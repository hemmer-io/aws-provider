# S3_control Service



**Resources**: 33

---

## Overview

The s3_control service provides access to 33 resource types:

- [Public_access_block](#public_access_block) [CRD]
- [Bucket_policy](#bucket_policy) [CRD]
- [Access_point_policy_for_object_lambda](#access_point_policy_for_object_lambda) [CRD]
- [Access_point_scope](#access_point_scope) [CRD]
- [Job_tagging](#job_tagging) [CRD]
- [Access_grants_instance](#access_grants_instance) [CRD]
- [Access_grants_location](#access_grants_location) [CRUD]
- [Job](#job) [CR]
- [Access_point_policy_status_for_object_lambda](#access_point_policy_status_for_object_lambda) [R]
- [Data_access](#data_access) [R]
- [Multi_region_access_point_policy_status](#multi_region_access_point_policy_status) [R]
- [Job_status](#job_status) [U]
- [Access_point](#access_point) [CRD]
- [Access_point_for_object_lambda](#access_point_for_object_lambda) [CRD]
- [Access_grants_instance_resource_policy](#access_grants_instance_resource_policy) [CRD]
- [Storage_lens_configuration](#storage_lens_configuration) [CRD]
- [Access_grant](#access_grant) [CRD]
- [Bucket_lifecycle_configuration](#bucket_lifecycle_configuration) [CRD]
- [Access_grants_instance_for_prefix](#access_grants_instance_for_prefix) [R]
- [Access_point_configuration_for_object_lambda](#access_point_configuration_for_object_lambda) [CR]
- [Multi_region_access_point](#multi_region_access_point) [CRD]
- [Bucket_replication](#bucket_replication) [CRD]
- [Bucket](#bucket) [CRD]
- [Storage_lens_group](#storage_lens_group) [CRUD]
- [Multi_region_access_point_operation](#multi_region_access_point_operation) [R]
- [Multi_region_access_point_policy](#multi_region_access_point_policy) [CR]
- [Bucket_tagging](#bucket_tagging) [CRD]
- [Job_priority](#job_priority) [U]
- [Multi_region_access_point_routes](#multi_region_access_point_routes) [R]
- [Storage_lens_configuration_tagging](#storage_lens_configuration_tagging) [CRD]
- [Access_point_policy_status](#access_point_policy_status) [R]
- [Access_point_policy](#access_point_policy) [CRD]
- [Bucket_versioning](#bucket_versioning) [CR]

---

## Resources


### Public_access_block

PublicAccessBlock resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `public_access_block_configuration` | String | ✅ | <p>The <code>PublicAccessBlock</code> configuration that you want to apply to the specified
         Amazon Web Services account.</p> |
| `account_id` | String | ✅ | <p>The account ID for the Amazon Web Services account whose <code>PublicAccessBlock</code> configuration
         you want to set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `public_access_block_configuration` | String | <p>The <code>PublicAccessBlock</code> configuration currently in effect for this
         Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create public_access_block
public_access_block = provider.s3_control.Public_access_block {
    public_access_block_configuration = "value"  # <p>The <code>PublicAccessBlock</code> configuration that you want to apply to the specified
         Amazon Web Services account.</p>
    account_id = "value"  # <p>The account ID for the Amazon Web Services account whose <code>PublicAccessBlock</code> configuration
         you want to set.</p>
}

# Access public_access_block outputs
public_access_block_id = public_access_block.id
public_access_block_public_access_block_configuration = public_access_block.public_access_block_configuration
```

---


### Bucket_policy

BucketPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `confirm_remove_self_bucket_access` | bool |  | <p>Set this parameter to true to confirm that you want to remove your permissions to change
         this bucket policy in the future.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `policy` | String | ✅ | <p>The bucket policy as a JSON document.</p> |
| `bucket` | String | ✅ | <p>Specifies the bucket.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the Outposts bucket.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The policy of the Outposts bucket.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket_policy
bucket_policy = provider.s3_control.Bucket_policy {
    policy = "value"  # <p>The bucket policy as a JSON document.</p>
    bucket = "value"  # <p>Specifies the bucket.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p>
    account_id = "value"  # <p>The Amazon Web Services account ID of the Outposts bucket.</p>
}

# Access bucket_policy outputs
bucket_policy_id = bucket_policy.id
bucket_policy_policy = bucket_policy.policy
```

---


### Access_point_policy_for_object_lambda

AccessPointPolicyForObjectLambda resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>Object Lambda Access Point resource policy document.</p> |
| `account_id` | String | ✅ | <p>The account ID for the account that owns the specified Object Lambda Access Point.</p> |
| `name` | String | ✅ | <p>The name of the Object Lambda Access Point.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>Object Lambda Access Point resource policy document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_point_policy_for_object_lambda
access_point_policy_for_object_lambda = provider.s3_control.Access_point_policy_for_object_lambda {
    policy = "value"  # <p>Object Lambda Access Point resource policy document.</p>
    account_id = "value"  # <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
    name = "value"  # <p>The name of the Object Lambda Access Point.</p>
}

# Access access_point_policy_for_object_lambda outputs
access_point_policy_for_object_lambda_id = access_point_policy_for_object_lambda.id
access_point_policy_for_object_lambda_policy = access_point_policy_for_object_lambda.policy
```

---


### Access_point_scope

AccessPointScope resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scope` | String | ✅ | <p>Object prefixes, API operations, or a combination of both.</p> |
| `name` | String | ✅ | <p>The name of the access point with the scope that you want to create or replace.</p> |
| `account_id` | String | ✅ | <p> The Amazon Web Services account ID that owns the access point with scope that you want to create or replace.
      </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scope` | String | <p>The contents of the access point scope.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_point_scope
access_point_scope = provider.s3_control.Access_point_scope {
    scope = "value"  # <p>Object prefixes, API operations, or a combination of both.</p>
    name = "value"  # <p>The name of the access point with the scope that you want to create or replace.</p>
    account_id = "value"  # <p> The Amazon Web Services account ID that owns the access point with scope that you want to create or replace.
      </p>
}

# Access access_point_scope outputs
access_point_scope_id = access_point_scope.id
access_point_scope_scope = access_point_scope.scope
```

---


### Job_tagging

JobTagging resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> | ✅ | <p>The set of tags to associate with the S3 Batch Operations job.</p> |
| `job_id` | String | ✅ | <p>The ID for the S3 Batch Operations job whose tags you want to replace.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | Vec<String> | <p>The set of tags associated with the S3 Batch Operations job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job_tagging
job_tagging = provider.s3_control.Job_tagging {
    tags = "value"  # <p>The set of tags to associate with the S3 Batch Operations job.</p>
    job_id = "value"  # <p>The ID for the S3 Batch Operations job whose tags you want to replace.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p>
}

# Access job_tagging outputs
job_tagging_id = job_tagging.id
job_tagging_tags = job_tagging.tags
```

---


### Access_grants_instance

AccessGrantsInstance resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `identity_center_arn` | String |  | <p>If you would like to associate your S3 Access Grants instance with an Amazon Web Services IAM Identity Center
         instance, use this field to pass the Amazon Resource Name (ARN) of the Amazon Web Services IAM Identity
         Center instance that you are associating with your S3 Access Grants instance. An IAM Identity
         Center instance is your corporate identity directory that you added to the IAM Identity
         Center. You can use the <a href="https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ListInstances.html">ListInstances</a> API
         operation to retrieve a list of your Identity Center instances and their ARNs. </p> |
| `tags` | Vec<String> |  | <p>The Amazon Web Services resource tags that you are adding to the S3 Access Grants instance. Each tag is a label
         consisting of a user-defined key and value. Tags can help you manage, identify, organize,
         search for, and filter resources. </p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_center_application_arn` | String | <p>If you associated your S3 Access Grants instance with an Amazon Web Services IAM Identity Center instance,
         this field returns the Amazon Resource Name (ARN) of the IAM Identity Center instance
         application; a subresource of the original Identity Center instance. S3 Access Grants creates this
         Identity Center application for the specific S3 Access Grants instance. </p> |
| `identity_center_arn` | String | <p>If you associated your S3 Access Grants instance with an Amazon Web Services IAM Identity Center instance,
         this field returns the Amazon Resource Name (ARN) of the IAM Identity Center instance
         application; a subresource of the original Identity Center instance. S3 Access Grants creates this
         Identity Center application for the specific S3 Access Grants instance. </p> |
| `access_grants_instance_id` | String | <p>The ID of the S3 Access Grants instance. The ID is <code>default</code>. You can have one S3 Access Grants
         instance per Region per account. </p> |
| `access_grants_instance_arn` | String | <p>The Amazon Resource Name (ARN) of the S3 Access Grants instance. </p> |
| `created_at` | String | <p>The date and time when you created the S3 Access Grants instance. </p> |
| `identity_center_instance_arn` | String | <p>The Amazon Resource Name (ARN) of the Amazon Web Services IAM Identity Center instance that you are
         associating with your S3 Access Grants instance. An IAM Identity Center instance is your corporate
         identity directory that you added to the IAM Identity Center. You can use the <a href="https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ListInstances.html">ListInstances</a> API operation to retrieve a list of your Identity Center
         instances and their ARNs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_grants_instance
access_grants_instance = provider.s3_control.Access_grants_instance {
    account_id = "value"  # <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p>
}

# Access access_grants_instance outputs
access_grants_instance_id = access_grants_instance.id
access_grants_instance_identity_center_application_arn = access_grants_instance.identity_center_application_arn
access_grants_instance_identity_center_arn = access_grants_instance.identity_center_arn
access_grants_instance_access_grants_instance_id = access_grants_instance.access_grants_instance_id
access_grants_instance_access_grants_instance_arn = access_grants_instance.access_grants_instance_arn
access_grants_instance_created_at = access_grants_instance.created_at
access_grants_instance_identity_center_instance_arn = access_grants_instance.identity_center_instance_arn
```

---


### Access_grants_location

AccessGrantsLocation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location_scope` | String | ✅ | <p>The S3 path to the location that you are registering. The location scope can be the
         default S3 location <code>s3://</code>, the S3 path to a bucket
            <code>s3://<bucket></code>, or the S3 path to a bucket and prefix
            <code>s3://<bucket>/<prefix></code>. A prefix in S3 is a string of
         characters at the beginning of an object key name used to organize the objects that you
         store in your S3 buckets. For example, object key names that start with the
            <code>engineering/</code> prefix or object key names that start with the
            <code>marketing/campaigns/</code> prefix.</p> |
| `iam_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role for the registered location. S3 Access Grants
         assumes this role to manage access to the registered location. </p> |
| `tags` | Vec<String> |  | <p>The Amazon Web Services resource tags that you are adding to the S3 Access Grants location. Each tag is a label
         consisting of a user-defined key and value. Tags can help you manage, identify, organize,
         search for, and filter resources.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_grants_location_id` | String | <p>The ID of the registered location to which you are granting access. S3 Access Grants assigns this
         ID when you register the location. S3 Access Grants assigns the ID <code>default</code> to the
         default location <code>s3://</code> and assigns an auto-generated ID to other locations
         that you register. </p> |
| `access_grants_location_arn` | String | <p>The Amazon Resource Name (ARN) of the registered location. </p> |
| `location_scope` | String | <p>The S3 URI path to the registered location. The location scope can be the default S3
         location <code>s3://</code>, the S3 path to a bucket, or the S3 path to a bucket and
         prefix. A prefix in S3 is a string of characters at the beginning of an object key name
         used to organize the objects that you store in your S3 buckets. For example, object key
         names that start with the <code>engineering/</code> prefix or object key names that start
         with the <code>marketing/campaigns/</code> prefix.</p> |
| `created_at` | String | <p>The date and time when you registered the location. </p> |
| `iam_role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role for the registered location. S3 Access Grants
         assumes this role to manage access to the registered location. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_grants_location
access_grants_location = provider.s3_control.Access_grants_location {
    location_scope = "value"  # <p>The S3 path to the location that you are registering. The location scope can be the
         default S3 location <code>s3://</code>, the S3 path to a bucket
            <code>s3://<bucket></code>, or the S3 path to a bucket and prefix
            <code>s3://<bucket>/<prefix></code>. A prefix in S3 is a string of
         characters at the beginning of an object key name used to organize the objects that you
         store in your S3 buckets. For example, object key names that start with the
            <code>engineering/</code> prefix or object key names that start with the
            <code>marketing/campaigns/</code> prefix.</p>
    iam_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role for the registered location. S3 Access Grants
         assumes this role to manage access to the registered location. </p>
    account_id = "value"  # <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p>
}

# Access access_grants_location outputs
access_grants_location_id = access_grants_location.id
access_grants_location_access_grants_location_id = access_grants_location.access_grants_location_id
access_grants_location_access_grants_location_arn = access_grants_location.access_grants_location_arn
access_grants_location_location_scope = access_grants_location.location_scope
access_grants_location_created_at = access_grants_location.created_at
access_grants_location_iam_role_arn = access_grants_location.iam_role_arn
```

---


### Job

Job resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `confirmation_required` | bool |  | <p>Indicates whether confirmation is required before Amazon S3 runs the job. Confirmation is
         only required for jobs created through the Amazon S3 console.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for the Identity and Access Management (IAM) role that Batch Operations will
         use to run this job's action on every object in the manifest.</p> |
| `operation` | String | ✅ | <p>The action that you want this job to perform on every object listed in the manifest. For
         more information about the available actions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-operations.html">Operations</a> in the
            <i>Amazon S3 User Guide</i>.</p> |
| `report` | String | ✅ | <p>Configuration parameters for the optional job-completion report.</p> |
| `client_request_token` | String | ✅ | <p>An idempotency token to ensure that you don't accidentally submit the same request
         twice. You can use any string up to the maximum length.</p> |
| `manifest` | String |  | <p>Configuration parameters for the manifest.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID that creates the job.</p> |
| `description` | String |  | <p>A description for this job. You can use any string within the permitted length.
         Descriptions don't need to be unique and can be used for multiple jobs.</p> |
| `priority` | i64 | ✅ | <p>The numerical priority for this job. Higher numbers indicate higher priority.</p> |
| `manifest_generator` | String |  | <p>The attribute container for the ManifestGenerator details. Jobs must be created with
         either a manifest file or a ManifestGenerator, but not both.</p> |
| `tags` | Vec<String> |  | <p>A set of tags to associate with the S3 Batch Operations job. This is an optional parameter.
      </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job` | String | <p>Contains the configuration parameters and status for the job specified in the
            <code>Describe Job</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job
job = provider.s3_control.Job {
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) for the Identity and Access Management (IAM) role that Batch Operations will
         use to run this job's action on every object in the manifest.</p>
    operation = "value"  # <p>The action that you want this job to perform on every object listed in the manifest. For
         more information about the available actions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-operations.html">Operations</a> in the
            <i>Amazon S3 User Guide</i>.</p>
    report = "value"  # <p>Configuration parameters for the optional job-completion report.</p>
    client_request_token = "value"  # <p>An idempotency token to ensure that you don't accidentally submit the same request
         twice. You can use any string up to the maximum length.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID that creates the job.</p>
    priority = "value"  # <p>The numerical priority for this job. Higher numbers indicate higher priority.</p>
}

# Access job outputs
job_id = job.id
job_job = job.job
```

---


### Access_point_policy_status_for_object_lambda

AccessPointPolicyStatusForObjectLambda resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_status` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_point_policy_status_for_object_lambda outputs
access_point_policy_status_for_object_lambda_id = access_point_policy_status_for_object_lambda.id
access_point_policy_status_for_object_lambda_policy_status = access_point_policy_status_for_object_lambda.policy_status
```

---


### Data_access

DataAccess resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credentials` | String | <p>The temporary credential token that S3 Access Grants vends.</p> |
| `matched_grant_target` | String | <p>The S3 URI path of the data to which you are being granted temporary access credentials.
      </p> |
| `grantee` | String | <p>The user, group, or role that was granted access to the S3 location scope. For directory
         identities, this API also returns the grants of the IAM role used for the identity-aware
         request. For more information on identity-aware sessions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_sts-setcontext.html">Granting permissions to use identity-aware console sessions</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_access outputs
data_access_id = data_access.id
data_access_credentials = data_access.credentials
data_access_matched_grant_target = data_access.matched_grant_target
data_access_grantee = data_access.grantee
```

---


### Multi_region_access_point_policy_status

MultiRegionAccessPointPolicyStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `established` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access multi_region_access_point_policy_status outputs
multi_region_access_point_policy_status_id = multi_region_access_point_policy_status.id
multi_region_access_point_policy_status_established = multi_region_access_point_policy_status.established
```

---


### Job_status

JobStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status_update_reason` | String |  | <p>A description of the reason why you want to change the specified job's status. This
         field can be any string up to the maximum length.</p> |
| `requested_job_status` | String | ✅ | <p>The status that you want to move the specified job to.</p> |
| `job_id` | String | ✅ | <p>The ID of the job whose status you want to update.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p> |



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


### Access_point

AccessPoint resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID for the account that owns the specified access point.</p> |
| `vpc_configuration` | String |  | <p>If you include this field, Amazon S3 restricts access to this access point to requests from the
         specified virtual private cloud (VPC).</p>
         <note>
            <p>This is required for creating an access point for Amazon S3 on Outposts buckets.</p>
         </note> |
| `bucket_account_id` | String |  | <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p>
         <p>For same account access point when your bucket and access point belong to the same account owner, the
            <code>BucketAccountId</code> is not required. For cross-account access point when your bucket
         and access point are not in the same account, the <code>BucketAccountId</code> is required. </p> |
| `scope` | String |  | <p>For directory buckets, you can filter access control to specific prefixes, API
         operations, or a combination of both. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-directory-buckets.html">Managing access to shared datasets in directory buckets with
            access points</a> in the <i>Amazon S3 User Guide</i>.</p>
         <note>
            <p>Scope is only supported for access points attached to directory buckets.</p>
         </note> |
| `name` | String | ✅ | <p>The name you want to assign to this access point.</p>
         <p>For directory buckets, the access point name must consist of a base name that you provide and
         suffix that includes the <code>ZoneID</code> (Amazon Web Services Availability Zone or Local Zone) of your bucket location,
         followed by <code>--xa-s3</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-directory-buckets.html">Managing access to shared datasets in directory buckets with
            access points</a> in the <i>Amazon S3 User Guide</i>.</p> |
| `bucket` | String | ✅ | <p>The name of the bucket that you want to associate this access point with.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p> |
| `public_access_block_configuration` | String |  | <p> The <code>PublicAccessBlock</code> configuration that you want to apply to the access point.
      </p> |
| `tags` | Vec<String> |  | <p>An array of tags that you can apply to an access point. Tags are key-value pairs of metadata used to control access to your access points. For more information about tags, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/tagging.html">Using tags with Amazon S3</a>. For information about tagging access points, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/tagging.html#using-tags-for-abac">Using tags for attribute-based access control (ABAC)</a>.</p>
         <note>
            <ul>
               <li>
                  <p>You must have the <code>s3:TagResource</code> permission to create an access point with tags for a general purpose bucket. </p>
               </li>
               <li>
                  <p>You must have the <code>s3express:TagResource</code> permission to create an access point with tags for a directory bucket.</p>
               </li>
            </ul>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_origin` | String | <p>Indicates whether this access point allows access from the public internet. If
            <code>VpcConfiguration</code> is specified for this access point, then
            <code>NetworkOrigin</code> is <code>VPC</code>, and the access point doesn't allow access from
         the public internet. Otherwise, <code>NetworkOrigin</code> is <code>Internet</code>, and
         the access point allows access from the public internet, subject to the access point and bucket access
         policies.</p>
         <p>This will always be true for an Amazon S3 on Outposts access point</p> |
| `vpc_configuration` | String | <p>Contains the virtual private cloud (VPC) configuration for the specified access point.</p>
         <note>
            <p>This element is empty if this access point is an Amazon S3 on Outposts access point that is used by other
               Amazon Web Services services.</p>
         </note> |
| `creation_date` | String | <p>The date and time when the specified access point was created.</p> |
| `data_source_type` | String | <p>The type of the data source that the access point is attached to.</p> |
| `name` | String | <p>The name of the specified access point.</p> |
| `alias` | String | <p>The name or alias of the access point.</p> |
| `bucket_account_id` | String | <p>The Amazon Web Services account ID associated with the S3 bucket associated with this access point.</p> |
| `bucket` | String | <p>The name of the bucket associated with the specified access point.</p> |
| `public_access_block_configuration` | String |  |
| `access_point_arn` | String | <p>The ARN of the access point.</p> |
| `endpoints` | HashMap<String, String> | <p>The VPC endpoint for the access point.</p> |
| `data_source_id` | String | <p>The unique identifier for the data source of the access point.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_point
access_point = provider.s3_control.Access_point {
    account_id = "value"  # <p>The Amazon Web Services account ID for the account that owns the specified access point.</p>
    name = "value"  # <p>The name you want to assign to this access point.</p>
         <p>For directory buckets, the access point name must consist of a base name that you provide and
         suffix that includes the <code>ZoneID</code> (Amazon Web Services Availability Zone or Local Zone) of your bucket location,
         followed by <code>--xa-s3</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-directory-buckets.html">Managing access to shared datasets in directory buckets with
            access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    bucket = "value"  # <p>The name of the bucket that you want to associate this access point with.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p>
}

# Access access_point outputs
access_point_id = access_point.id
access_point_network_origin = access_point.network_origin
access_point_vpc_configuration = access_point.vpc_configuration
access_point_creation_date = access_point.creation_date
access_point_data_source_type = access_point.data_source_type
access_point_name = access_point.name
access_point_alias = access_point.alias
access_point_bucket_account_id = access_point.bucket_account_id
access_point_bucket = access_point.bucket
access_point_public_access_block_configuration = access_point.public_access_block_configuration
access_point_access_point_arn = access_point.access_point_arn
access_point_endpoints = access_point.endpoints
access_point_data_source_id = access_point.data_source_id
```

---


### Access_point_for_object_lambda

AccessPointForObjectLambda resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name you want to assign to this Object Lambda Access Point.</p> |
| `configuration` | String | ✅ | <p>Object Lambda Access Point configuration as a JSON document.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID for owner of the specified Object Lambda Access Point.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>The date and time when the specified Object Lambda Access Point was created.</p> |
| `public_access_block_configuration` | String | <p>Configuration to block all public access. This setting is turned on and can not be
         edited. </p> |
| `alias` | String | <p>The alias of the Object Lambda Access Point.</p> |
| `name` | String | <p>The name of the Object Lambda Access Point.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_point_for_object_lambda
access_point_for_object_lambda = provider.s3_control.Access_point_for_object_lambda {
    name = "value"  # <p>The name you want to assign to this Object Lambda Access Point.</p>
    configuration = "value"  # <p>Object Lambda Access Point configuration as a JSON document.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID for owner of the specified Object Lambda Access Point.</p>
}

# Access access_point_for_object_lambda outputs
access_point_for_object_lambda_id = access_point_for_object_lambda.id
access_point_for_object_lambda_creation_date = access_point_for_object_lambda.creation_date
access_point_for_object_lambda_public_access_block_configuration = access_point_for_object_lambda.public_access_block_configuration
access_point_for_object_lambda_alias = access_point_for_object_lambda.alias
access_point_for_object_lambda_name = access_point_for_object_lambda.name
```

---


### Access_grants_instance_resource_policy

AccessGrantsInstanceResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The resource policy of the S3 Access Grants instance that you are updating.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p> |
| `organization` | String |  | <p>The Organization of the resource policy of the S3 Access Grants instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The date and time when you created the S3 Access Grants instance resource policy. </p> |
| `policy` | String | <p>The resource policy of the S3 Access Grants instance.</p> |
| `organization` | String | <p>The Organization of the resource policy of the S3 Access Grants instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_grants_instance_resource_policy
access_grants_instance_resource_policy = provider.s3_control.Access_grants_instance_resource_policy {
    policy = "value"  # <p>The resource policy of the S3 Access Grants instance that you are updating.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p>
}

# Access access_grants_instance_resource_policy outputs
access_grants_instance_resource_policy_id = access_grants_instance_resource_policy.id
access_grants_instance_resource_policy_created_at = access_grants_instance_resource_policy.created_at
access_grants_instance_resource_policy_policy = access_grants_instance_resource_policy.policy
access_grants_instance_resource_policy_organization = access_grants_instance_resource_policy.organization
```

---


### Storage_lens_configuration

StorageLensConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The account ID of the requester.</p> |
| `storage_lens_configuration` | String | ✅ | <p>The S3 Storage Lens configuration.</p> |
| `config_id` | String | ✅ | <p>The ID of the S3 Storage Lens configuration.</p> |
| `tags` | Vec<String> |  | <p>The tag set of the S3 Storage Lens configuration.</p>
         <note>
            <p>You can set up to a maximum of 50 tags.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `storage_lens_configuration` | String | <p>The S3 Storage Lens configuration requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create storage_lens_configuration
storage_lens_configuration = provider.s3_control.Storage_lens_configuration {
    account_id = "value"  # <p>The account ID of the requester.</p>
    storage_lens_configuration = "value"  # <p>The S3 Storage Lens configuration.</p>
    config_id = "value"  # <p>The ID of the S3 Storage Lens configuration.</p>
}

# Access storage_lens_configuration outputs
storage_lens_configuration_id = storage_lens_configuration.id
storage_lens_configuration_storage_lens_configuration = storage_lens_configuration.storage_lens_configuration
```

---


### Access_grant

AccessGrant resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_prefix_type` | String |  | <p>The type of <code>S3SubPrefix</code>. The only possible value is <code>Object</code>.
         Pass this value if the access grant scope is an object. Do not pass this value if the
         access grant scope is a bucket or a bucket and a prefix. </p> |
| `application_arn` | String |  | <p>The Amazon Resource Name (ARN) of an Amazon Web Services IAM Identity Center application associated
         with your Identity Center instance. If an application ARN is included in the request to
         create an access grant, the grantee can only access the S3 data through this application.
      </p> |
| `access_grants_location_id` | String | ✅ | <p>The ID of the registered location to which you are granting access. S3 Access Grants assigns this
         ID when you register the location. S3 Access Grants assigns the ID <code>default</code> to the
         default location <code>s3://</code> and assigns an auto-generated ID to other locations
         that you register. </p>
         <p>If you are passing the <code>default</code> location, you cannot create an access grant
         for the entire default location. You must also specify a bucket or a bucket and prefix in
         the <code>Subprefix</code> field. </p> |
| `access_grants_location_configuration` | String |  | <p>The configuration options of the grant location. The grant location is the S3 path to
         the data to which you are granting access. It contains the <code>S3SubPrefix</code> field.
         The grant scope is the result of appending the subprefix to the location scope of the
         registered location.</p> |
| `grantee` | String | ✅ | <p>The user, group, or role to which you are granting access. You can grant access to an
         IAM user or role. If you have added your corporate directory to Amazon Web Services IAM Identity
         Center and associated your Identity Center instance with your S3 Access Grants instance, the grantee
         can also be a corporate directory user or group.</p> |
| `permission` | String | ✅ | <p>The type of access that you are granting to your S3 data, which can be set to one of the
         following values:</p>
         <ul>
            <li>
               <p>
                  <code>READ</code> – Grant read-only access to the S3 data.</p>
            </li>
            <li>
               <p>
                  <code>WRITE</code> – Grant write-only access to the S3 data.</p>
            </li>
            <li>
               <p>
                  <code>READWRITE</code> – Grant both read and write access to the S3 data.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>The Amazon Web Services resource tags that you are adding to the access grant. Each tag is a label
         consisting of a user-defined key and value. Tags can help you manage, identify, organize,
         search for, and filter resources. </p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_grant_id` | String | <p>The ID of the access grant. S3 Access Grants auto-generates this ID when you create the access
         grant.</p> |
| `access_grants_location_configuration` | String | <p>The configuration options of the grant location. The grant location is the S3 path to
         the data to which you are granting access. </p> |
| `grantee` | String | <p>The user, group, or role to which you are granting access. You can grant access to an
         IAM user or role. If you have added a corporate directory to Amazon Web Services IAM Identity Center
         and associated this Identity Center instance with the S3 Access Grants instance, the grantee can also
         be a corporate directory user or group.</p> |
| `grant_scope` | String | <p>The S3 path of the data to which you are granting access. It is the result of appending
         the <code>Subprefix</code> to the location scope.</p> |
| `access_grants_location_id` | String | <p>The ID of the registered location to which you are granting access. S3 Access Grants assigns this
         ID when you register the location. S3 Access Grants assigns the ID <code>default</code> to the
         default location <code>s3://</code> and assigns an auto-generated ID to other locations
         that you register. </p> |
| `created_at` | String | <p>The date and time when you created the access grant. </p> |
| `permission` | String | <p>The type of permission that was granted in the access grant. Can be one of the following
         values:</p>
         <ul>
            <li>
               <p>
                  <code>READ</code> – Grant read-only access to the S3 data.</p>
            </li>
            <li>
               <p>
                  <code>WRITE</code> – Grant write-only access to the S3 data.</p>
            </li>
            <li>
               <p>
                  <code>READWRITE</code> – Grant both read and write access to the S3 data.</p>
            </li>
         </ul> |
| `application_arn` | String | <p>The Amazon Resource Name (ARN) of an Amazon Web Services IAM Identity Center application associated
         with your Identity Center instance. If the grant includes an application ARN, the grantee
         can only access the S3 data through this application. </p> |
| `access_grant_arn` | String | <p>The Amazon Resource Name (ARN) of the access grant. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_grant
access_grant = provider.s3_control.Access_grant {
    access_grants_location_id = "value"  # <p>The ID of the registered location to which you are granting access. S3 Access Grants assigns this
         ID when you register the location. S3 Access Grants assigns the ID <code>default</code> to the
         default location <code>s3://</code> and assigns an auto-generated ID to other locations
         that you register. </p>
         <p>If you are passing the <code>default</code> location, you cannot create an access grant
         for the entire default location. You must also specify a bucket or a bucket and prefix in
         the <code>Subprefix</code> field. </p>
    grantee = "value"  # <p>The user, group, or role to which you are granting access. You can grant access to an
         IAM user or role. If you have added your corporate directory to Amazon Web Services IAM Identity
         Center and associated your Identity Center instance with your S3 Access Grants instance, the grantee
         can also be a corporate directory user or group.</p>
    permission = "value"  # <p>The type of access that you are granting to your S3 data, which can be set to one of the
         following values:</p>
         <ul>
            <li>
               <p>
                  <code>READ</code> – Grant read-only access to the S3 data.</p>
            </li>
            <li>
               <p>
                  <code>WRITE</code> – Grant write-only access to the S3 data.</p>
            </li>
            <li>
               <p>
                  <code>READWRITE</code> – Grant both read and write access to the S3 data.</p>
            </li>
         </ul>
    account_id = "value"  # <p>The Amazon Web Services account ID of the S3 Access Grants instance.</p>
}

# Access access_grant outputs
access_grant_id = access_grant.id
access_grant_access_grant_id = access_grant.access_grant_id
access_grant_access_grants_location_configuration = access_grant.access_grants_location_configuration
access_grant_grantee = access_grant.grantee
access_grant_grant_scope = access_grant.grant_scope
access_grant_access_grants_location_id = access_grant.access_grants_location_id
access_grant_created_at = access_grant.created_at
access_grant_permission = access_grant.permission
access_grant_application_arn = access_grant.application_arn
access_grant_access_grant_arn = access_grant.access_grant_arn
```

---


### Bucket_lifecycle_configuration

BucketLifecycleConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the Outposts bucket.</p> |
| `lifecycle_configuration` | String |  | <p>Container for lifecycle rules. You can add as many as 1,000 rules.</p> |
| `bucket` | String | ✅ | <p>The name of the bucket for which to set the configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rules` | Vec<String> | <p>Container for the lifecycle rule of the Outposts bucket.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket_lifecycle_configuration
bucket_lifecycle_configuration = provider.s3_control.Bucket_lifecycle_configuration {
    account_id = "value"  # <p>The Amazon Web Services account ID of the Outposts bucket.</p>
    bucket = "value"  # <p>The name of the bucket for which to set the configuration.</p>
}

# Access bucket_lifecycle_configuration outputs
bucket_lifecycle_configuration_id = bucket_lifecycle_configuration.id
bucket_lifecycle_configuration_rules = bucket_lifecycle_configuration.rules
```

---


### Access_grants_instance_for_prefix

AccessGrantsInstanceForPrefix resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_grants_instance_arn` | String | <p>The Amazon Resource Name (ARN) of the S3 Access Grants instance. </p> |
| `access_grants_instance_id` | String | <p>The ID of the S3 Access Grants instance. The ID is <code>default</code>. You can have one S3 Access Grants
         instance per Region per account. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_grants_instance_for_prefix outputs
access_grants_instance_for_prefix_id = access_grants_instance_for_prefix.id
access_grants_instance_for_prefix_access_grants_instance_arn = access_grants_instance_for_prefix.access_grants_instance_arn
access_grants_instance_for_prefix_access_grants_instance_id = access_grants_instance_for_prefix.access_grants_instance_id
```

---


### Access_point_configuration_for_object_lambda

AccessPointConfigurationForObjectLambda resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration` | String | ✅ | <p>Object Lambda Access Point configuration document.</p> |
| `name` | String | ✅ | <p>The name of the Object Lambda Access Point.</p> |
| `account_id` | String | ✅ | <p>The account ID for the account that owns the specified Object Lambda Access Point.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration` | String | <p>Object Lambda Access Point configuration document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_point_configuration_for_object_lambda
access_point_configuration_for_object_lambda = provider.s3_control.Access_point_configuration_for_object_lambda {
    configuration = "value"  # <p>Object Lambda Access Point configuration document.</p>
    name = "value"  # <p>The name of the Object Lambda Access Point.</p>
    account_id = "value"  # <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
}

# Access access_point_configuration_for_object_lambda outputs
access_point_configuration_for_object_lambda_id = access_point_configuration_for_object_lambda.id
access_point_configuration_for_object_lambda_configuration = access_point_configuration_for_object_lambda.configuration
```

---


### Multi_region_access_point

MultiRegionAccessPoint resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String | ✅ | <p>An idempotency token used to identify the request and guarantee that requests are
         unique.</p> |
| `details` | String | ✅ | <p>A container element containing details about the Multi-Region Access Point.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point. The owner of the Multi-Region Access Point also must own
         the underlying buckets.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_point` | String | <p>A container element containing the details of the requested Multi-Region Access Point.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multi_region_access_point
multi_region_access_point = provider.s3_control.Multi_region_access_point {
    client_token = "value"  # <p>An idempotency token used to identify the request and guarantee that requests are
         unique.</p>
    details = "value"  # <p>A container element containing details about the Multi-Region Access Point.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point. The owner of the Multi-Region Access Point also must own
         the underlying buckets.</p>
}

# Access multi_region_access_point outputs
multi_region_access_point_id = multi_region_access_point.id
multi_region_access_point_access_point = multi_region_access_point.access_point
```

---


### Bucket_replication

BucketReplication resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the Outposts bucket.</p> |
| `replication_configuration` | String | ✅ | <p></p> |
| `bucket` | String | ✅ | <p>Specifies the S3 on Outposts bucket to set the configuration for.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_configuration` | String | <p>A container for one or more replication rules. A replication configuration must have at
         least one rule and you can add up to 100 rules. The maximum size of a replication
         configuration is 128 KB.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket_replication
bucket_replication = provider.s3_control.Bucket_replication {
    account_id = "value"  # <p>The Amazon Web Services account ID of the Outposts bucket.</p>
    replication_configuration = "value"  # <p></p>
    bucket = "value"  # <p>Specifies the S3 on Outposts bucket to set the configuration for.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p>
}

# Access bucket_replication outputs
bucket_replication_id = bucket_replication.id
bucket_replication_replication_configuration = bucket_replication.replication_configuration
```

---


### Bucket

Bucket resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `outpost_id` | String |  | <p>The ID of the Outposts where the bucket is being created.</p>
         <note>
            <p>This ID is required by Amazon S3 on Outposts buckets.</p>
         </note> |
| `grant_read_acp` | String |  | <p>Allows grantee to read the bucket ACL.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `acl` | String |  | <p>The canned ACL to apply to the bucket.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `grant_read` | String |  | <p>Allows grantee to list the objects in the bucket.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `grant_full_control` | String |  | <p>Allows grantee the read, write, read ACP, and write ACP permissions on the
         bucket.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `grant_write_acp` | String |  | <p>Allows grantee to write the ACL for the applicable bucket.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `object_lock_enabled_for_bucket` | bool |  | <p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `bucket` | String | ✅ | <p>The name of the bucket.</p> |
| `create_bucket_configuration` | String |  | <p>The configuration information for the bucket.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |
| `grant_write` | String |  | <p>Allows grantee to create, overwrite, and delete any object in the bucket.</p>
         <note>
            <p>This is not supported by Amazon S3 on Outposts buckets.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `public_access_block_enabled` | bool | <p></p> |
| `creation_date` | String | <p>The creation date of the Outposts bucket.</p> |
| `bucket` | String | <p>The Outposts bucket requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket
bucket = provider.s3_control.Bucket {
    bucket = "value"  # <p>The name of the bucket.</p>
}

# Access bucket outputs
bucket_id = bucket.id
bucket_public_access_block_enabled = bucket.public_access_block_enabled
bucket_creation_date = bucket.creation_date
bucket_bucket = bucket.bucket
```

---


### Storage_lens_group

StorageLensGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>
The Amazon Web Services account ID that the Storage Lens group is created from and associated with.
</p> |
| `storage_lens_group` | String | ✅ | <p>
The Storage Lens group configuration.
</p> |
| `tags` | Vec<String> |  | <p>
The Amazon Web Services resource tags that you're adding to your Storage Lens group. This parameter is optional.
</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `storage_lens_group` | String | <p>
The name of the Storage Lens group that you're trying to retrieve the configuration details for.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create storage_lens_group
storage_lens_group = provider.s3_control.Storage_lens_group {
    account_id = "value"  # <p>
The Amazon Web Services account ID that the Storage Lens group is created from and associated with.
</p>
    storage_lens_group = "value"  # <p>
The Storage Lens group configuration.
</p>
}

# Access storage_lens_group outputs
storage_lens_group_id = storage_lens_group.id
storage_lens_group_storage_lens_group = storage_lens_group.storage_lens_group
```

---


### Multi_region_access_point_operation

MultiRegionAccessPointOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `async_operation` | String | <p>A container element containing the details of the asynchronous operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access multi_region_access_point_operation outputs
multi_region_access_point_operation_id = multi_region_access_point_operation.id
multi_region_access_point_operation_async_operation = multi_region_access_point_operation.async_operation
```

---


### Multi_region_access_point_policy

MultiRegionAccessPointPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `details` | String | ✅ | <p>A container element containing the details of the policy for the Multi-Region Access Point.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p> |
| `client_token` | String | ✅ | <p>An idempotency token used to identify the request and guarantee that requests are
         unique.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The policy associated with the specified Multi-Region Access Point.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multi_region_access_point_policy
multi_region_access_point_policy = provider.s3_control.Multi_region_access_point_policy {
    details = "value"  # <p>A container element containing the details of the policy for the Multi-Region Access Point.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    client_token = "value"  # <p>An idempotency token used to identify the request and guarantee that requests are
         unique.</p>
}

# Access multi_region_access_point_policy outputs
multi_region_access_point_policy_id = multi_region_access_point_policy.id
multi_region_access_point_policy_policy = multi_region_access_point_policy.policy
```

---


### Bucket_tagging

BucketTagging resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the Outposts bucket.</p> |
| `bucket` | String | ✅ | <p>The Amazon Resource Name (ARN) of the bucket.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p> |
| `tagging` | String | ✅ | <p></p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_set` | Vec<String> | <p>The tags set of the Outposts bucket.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket_tagging
bucket_tagging = provider.s3_control.Bucket_tagging {
    account_id = "value"  # <p>The Amazon Web Services account ID of the Outposts bucket.</p>
    bucket = "value"  # <p>The Amazon Resource Name (ARN) of the bucket.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/bucket/<my-bucket-name></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded.  </p>
    tagging = "value"  # <p></p>
}

# Access bucket_tagging outputs
bucket_tagging_id = bucket_tagging.id
bucket_tagging_tag_set = bucket_tagging.tag_set
```

---


### Job_priority

JobPriority resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p> |
| `job_id` | String | ✅ | <p>The ID for the job whose priority you want to update.</p> |
| `priority` | i64 | ✅ | <p>The priority you want to assign to this job.</p> |



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


### Multi_region_access_point_routes

MultiRegionAccessPointRoutes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mrap` | String | <p>The Multi-Region Access Point ARN.</p> |
| `routes` | Vec<String> | <p>The different routes that make up the route configuration. Active routes return a value
         of <code>100</code>, and passive routes return a value of <code>0</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access multi_region_access_point_routes outputs
multi_region_access_point_routes_id = multi_region_access_point_routes.id
multi_region_access_point_routes_mrap = multi_region_access_point_routes.mrap
multi_region_access_point_routes_routes = multi_region_access_point_routes.routes
```

---


### Storage_lens_configuration_tagging

StorageLensConfigurationTagging resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | <p>The account ID of the requester.</p> |
| `tags` | Vec<String> | ✅ | <p>The tag set of the S3 Storage Lens configuration.</p>
         <note>
            <p>You can set up to a maximum of 50 tags.</p>
         </note> |
| `config_id` | String | ✅ | <p>The ID of the S3 Storage Lens configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | Vec<String> | <p>The tags of S3 Storage Lens configuration requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create storage_lens_configuration_tagging
storage_lens_configuration_tagging = provider.s3_control.Storage_lens_configuration_tagging {
    account_id = "value"  # <p>The account ID of the requester.</p>
    tags = "value"  # <p>The tag set of the S3 Storage Lens configuration.</p>
         <note>
            <p>You can set up to a maximum of 50 tags.</p>
         </note>
    config_id = "value"  # <p>The ID of the S3 Storage Lens configuration.</p>
}

# Access storage_lens_configuration_tagging outputs
storage_lens_configuration_tagging_id = storage_lens_configuration_tagging.id
storage_lens_configuration_tagging_tags = storage_lens_configuration_tagging.tags
```

---


### Access_point_policy_status

AccessPointPolicyStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_status` | String | <p>Indicates the current policy status of the specified access point.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_point_policy_status outputs
access_point_policy_status_id = access_point_policy_status.id
access_point_policy_status_policy_status = access_point_policy_status.policy_status
```

---


### Access_point_policy

AccessPointPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The policy that you want to apply to the specified access point. For more information about access point
         policies, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html">Managing data access with Amazon S3
            access points</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-directory-buckets.html">Managing access to
            shared datasets in directory buckets with access points</a> in the
            <i>Amazon S3 User Guide</i>.</p> |
| `name` | String | ✅ | <p>The name of the access point that you want to associate with the specified policy.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/accesspoint/<my-accesspoint-name></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID for owner of the bucket associated with the specified access point.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The access point policy associated with the specified access point.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_point_policy
access_point_policy = provider.s3_control.Access_point_policy {
    policy = "value"  # <p>The policy that you want to apply to the specified access point. For more information about access point
         policies, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html">Managing data access with Amazon S3
            access points</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points-directory-buckets.html">Managing access to
            shared datasets in directory buckets with access points</a> in the
            <i>Amazon S3 User Guide</i>.</p>
    name = "value"  # <p>The name of the access point that you want to associate with the specified policy.</p>
         <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
         <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must  specify the ARN of the access point accessed in the format <code>arn:aws:s3-outposts:<Region>:<account-id>:outpost/<outpost-id>/accesspoint/<my-accesspoint-name></code>. For example, to access the access point <code>reports-ap</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/accesspoint/reports-ap</code>. The value must be URL encoded. </p>
    account_id = "value"  # <p>The Amazon Web Services account ID for owner of the bucket associated with the specified access point.</p>
}

# Access access_point_policy outputs
access_point_policy_id = access_point_policy.id
access_point_policy_policy = access_point_policy.policy
```

---


### Bucket_versioning

BucketVersioning resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket` | String | ✅ | <p>The S3 on Outposts bucket to set the versioning state for.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the S3 on Outposts bucket.</p> |
| `versioning_configuration` | String | ✅ | <p>The root-level tag for the <code>VersioningConfiguration</code> parameters.</p> |
| `mfa` | String |  | <p>The concatenation of the authentication device's serial number, a space, and the value
         that is displayed on your authentication device.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The versioning state of the S3 on Outposts bucket.</p> |
| `mfa_delete` | String | <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This
         element is returned only if the bucket has been configured with MFA delete. If MFA delete
         has never been configured for the bucket, this element is not returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket_versioning
bucket_versioning = provider.s3_control.Bucket_versioning {
    bucket = "value"  # <p>The S3 on Outposts bucket to set the versioning state for.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID of the S3 on Outposts bucket.</p>
    versioning_configuration = "value"  # <p>The root-level tag for the <code>VersioningConfiguration</code> parameters.</p>
}

# Access bucket_versioning outputs
bucket_versioning_id = bucket_versioning.id
bucket_versioning_status = bucket_versioning.status
bucket_versioning_mfa_delete = bucket_versioning.mfa_delete
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple public_access_block resources
public_access_block_0 = provider.s3_control.Public_access_block {
    public_access_block_configuration = "value-0"
    account_id = "value-0"
}
public_access_block_1 = provider.s3_control.Public_access_block {
    public_access_block_configuration = "value-1"
    account_id = "value-1"
}
public_access_block_2 = provider.s3_control.Public_access_block {
    public_access_block_configuration = "value-2"
    account_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    public_access_block = provider.s3_control.Public_access_block {
        public_access_block_configuration = "production-value"
        account_id = "production-value"
    }
```

---

## Related Documentation

- [AWS S3_control Documentation](https://docs.aws.amazon.com/s3_control/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
