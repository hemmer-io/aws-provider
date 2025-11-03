# Mediaconvert Service



**Resources**: 8

---

## Overview

The mediaconvert service provides access to 8 resource types:

- [Endpoints](#endpoints) [R]
- [Jobs_query_results](#jobs_query_results) [R]
- [Resource_share](#resource_share) [C]
- [Policy](#policy) [CRD]
- [Job_template](#job_template) [CRUD]
- [Job](#job) [CR]
- [Queue](#queue) [CRUD]
- [Preset](#preset) [CRUD]

---

## Resources


### Endpoints

Endpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoints` | Vec<String> | List of endpoints |
| `next_token` | String | Use this string to request the next batch of endpoints. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoints outputs
endpoints_id = endpoints.id
endpoints_endpoints = endpoints.endpoints
endpoints_next_token = endpoints.next_token
```

---


### Jobs_query_results

JobsQueryResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `jobs` | Vec<String> | List of jobs. |
| `next_token` | String | Use this string to request the next batch of jobs via the StartJobsQuery API. |
| `status` | String | The status of the jobs query. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access jobs_query_results outputs
jobs_query_results_id = jobs_query_results.id
jobs_query_results_jobs = jobs_query_results.jobs
jobs_query_results_next_token = jobs_query_results.next_token
jobs_query_results_status = jobs_query_results.status
```

---


### Resource_share

ResourceShare resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `support_case_id` | String | ✅ | AWS Support case identifier |
| `job_id` | String | ✅ | Specify MediaConvert Job ID or ARN to share |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_share
resource_share = provider.mediaconvert.Resource_share {
    support_case_id = "value"  # AWS Support case identifier
    job_id = "value"  # Specify MediaConvert Job ID or ARN to share
}

```

---


### Policy

Policy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | A policy configures behavior that you allow or disallow for your account. For information about MediaConvert policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | A policy configures behavior that you allow or disallow for your account. For information about MediaConvert policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy
policy = provider.mediaconvert.Policy {
    policy = "value"  # A policy configures behavior that you allow or disallow for your account. For information about MediaConvert policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html
}

# Access policy outputs
policy_id = policy.id
policy_policy = policy.policy
```

---


### Job_template

JobTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `queue` | String |  | Optional. The queue that jobs created from this template are assigned to. If you don't specify this, jobs will go to the default queue. |
| `hop_destinations` | Vec<String> |  | Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide. |
| `category` | String |  | Optional. A category for the job template you are creating |
| `settings` | String | ✅ | JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it. |
| `acceleration_settings` | String |  | Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide. |
| `status_update_interval` | String |  | Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error. |
| `description` | String |  | Optional. A description of the job template you are creating. |
| `priority` | i64 |  | Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0. |
| `name` | String | ✅ | The name of the job template you are creating. |
| `tags` | HashMap<String, String> |  | The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_template` | String | A job template is a pre-made set of encoding instructions that you can use to quickly create a job. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job_template
job_template = provider.mediaconvert.Job_template {
    settings = "value"  # JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.
    name = "value"  # The name of the job template you are creating.
}

# Access job_template outputs
job_template_id = job_template.id
job_template_job_template = job_template.job_template
```

---


### Job

Job resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_metadata` | HashMap<String, String> |  | Optional. User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs.  Use only for existing integrations or workflows that rely on job metadata tags. Otherwise, we recommend that you use standard AWS tags. |
| `hop_destinations` | Vec<String> |  | Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide. |
| `status_update_interval` | String |  | Optional. Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error. |
| `priority` | i64 |  | Optional. Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0. |
| `job_engine_version` | String |  | Use Job engine versions to run jobs for your production workflow on one version, while you test and validate the latest version. Job engine versions represent periodically grouped MediaConvert releases with new features, updates, improvements, and fixes. Job engine versions are in a YYYY-MM-DD format. Note that the Job engine version feature is not publicly available at this time. To request access, contact AWS support. |
| `job_template` | String |  | Optional. When you create a job, you can either specify a job template or specify the transcoding settings individually. |
| `client_request_token` | String |  | Prevent duplicate jobs from being created and ensure idempotency for your requests. A client request token can be any string that includes up to 64 ASCII characters. If you reuse a client request token within one minute of a successful request, the API returns the job details of the original request instead. For more information see https://docs.aws.amazon.com/mediaconvert/latest/apireference/idempotency.html. |
| `role` | String | ✅ | Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html. |
| `queue` | String |  | Optional. When you create a job, you can specify a queue to send it to. If you don't specify, the job will go to the default queue. For more about queues, see the User Guide topic at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html. |
| `simulate_reserved_queue` | String |  | Optional. Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default. |
| `tags` | HashMap<String, String> |  | Optional. The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.  Use standard AWS tags on your job for automatic integration with AWS services and for custom integrations and workflows. |
| `acceleration_settings` | String |  | Optional. Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide. |
| `billing_tags_source` | String |  | Optionally choose a Billing tags source that AWS Billing and Cost Management will use to display tags for individual output costs on any billing report that you set up. Leave blank to use the default value, Job. |
| `settings` | String | ✅ | JobSettings contains all the transcode settings for a job. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job` | String | Each job converts an input file into an output file or files. For more information, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job
job = provider.mediaconvert.Job {
    role = "value"  # Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.
    settings = "value"  # JobSettings contains all the transcode settings for a job.
}

# Access job outputs
job_id = job.id
job_job = job.job
```

---


### Queue

Queue resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reservation_plan_settings` | String |  | Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues. |
| `description` | String |  | Optional. A description of the queue that you are creating. |
| `pricing_plan` | String |  | Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand. |
| `name` | String | ✅ | The name of the queue that you are creating. |
| `status` | String |  | Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin. |
| `concurrent_jobs` | i64 |  | Specify the maximum number of jobs your queue can process concurrently. For on-demand queues, the value you enter is constrained by your service quotas for Maximum concurrent jobs, per on-demand queue and Maximum concurrent jobs, per account. For reserved queues, specify the number of jobs you can process concurrently in your reservation plan instead. |
| `tags` | HashMap<String, String> |  | The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `queue` | String | You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don't specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create queue
queue = provider.mediaconvert.Queue {
    name = "value"  # The name of the queue that you are creating.
}

# Access queue outputs
queue_id = queue.id
queue_queue = queue.queue
```

---


### Preset

Preset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the preset you are creating. |
| `description` | String |  | Optional. A description of the preset you are creating. |
| `settings` | String | ✅ | Settings for preset |
| `tags` | HashMap<String, String> |  | The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key. |
| `category` | String |  | Optional. A category for the preset you are creating. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `preset` | String | A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process. |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create preset
preset = provider.mediaconvert.Preset {
    name = "value"  # The name of the preset you are creating.
    settings = "value"  # Settings for preset
}

# Access preset outputs
preset_id = preset.id
preset_preset = preset.preset
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple endpoints resources
endpoints_0 = provider.mediaconvert.Endpoints {
}
endpoints_1 = provider.mediaconvert.Endpoints {
}
endpoints_2 = provider.mediaconvert.Endpoints {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    endpoints = provider.mediaconvert.Endpoints {
    }
```

---

## Related Documentation

- [AWS Mediaconvert Documentation](https://docs.aws.amazon.com/mediaconvert/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
