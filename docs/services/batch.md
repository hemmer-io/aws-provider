# Batch Service



**Resources**: 13

---

## Overview

The batch service provides access to 13 resource types:

- [Job_queue_snapshot](#job_queue_snapshot) [R]
- [Scheduling_policy](#scheduling_policy) [CUD]
- [Service_environments](#service_environments) [R]
- [Scheduling_policies](#scheduling_policies) [R]
- [Service_environment](#service_environment) [CUD]
- [Job_definitions](#job_definitions) [R]
- [Jobs](#jobs) [R]
- [Job_queues](#job_queues) [R]
- [Job_queue](#job_queue) [CUD]
- [Consumable_resource](#consumable_resource) [CRUD]
- [Service_job](#service_job) [R]
- [Compute_environments](#compute_environments) [R]
- [Compute_environment](#compute_environment) [CUD]

---

## Resources


### Job_queue_snapshot

JobQueueSnapshot resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `front_of_queue` | String | <p>The list of the first 100 <code>RUNNABLE</code> jobs in each job queue. For first-in-first-out (FIFO) job queues, jobs are ordered based on their submission time. For fair-share scheduling (FSS) job queues, jobs are ordered based on their job priority and share usage.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_queue_snapshot outputs
job_queue_snapshot_id = job_queue_snapshot.id
job_queue_snapshot_front_of_queue = job_queue_snapshot.front_of_queue
```

---


### Scheduling_policy

SchedulingPolicy resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fairshare_policy` | String |  | <p>The fair-share scheduling policy details.</p> |
| `name` | String | ✅ | <p>The name of the fair-share scheduling policy. It can be up to 128 letters long. It can contain
      uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p> |
| `tags` | HashMap<String, String> |  | <p>The tags that you apply to the scheduling policy to help you categorize and organize your
      resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services
        Resources</a> in <i>Amazon Web Services General Reference</i>.</p>
         <p>These tags can be updated or removed using the <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_TagResource.html">TagResource</a> and <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_UntagResource.html">UntagResource</a> API operations.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scheduling_policy
scheduling_policy = provider.batch.Scheduling_policy {
    name = "value"  # <p>The name of the fair-share scheduling policy. It can be up to 128 letters long. It can contain
      uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
}

```

---


### Service_environments

ServiceEnvironments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_environments` | Vec<String> | <p>The list of service environments that match the request.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future <code>DescribeServiceEnvironments</code> request. When the results of a <code>DescribeServiceEnvironments</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_environments outputs
service_environments_id = service_environments.id
service_environments_service_environments = service_environments.service_environments
service_environments_next_token = service_environments.next_token
```

---


### Scheduling_policies

SchedulingPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scheduling_policies` | Vec<String> | <p>The list of scheduling policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scheduling_policies outputs
scheduling_policies_id = scheduling_policies.id
scheduling_policies_scheduling_policies = scheduling_policies.scheduling_policies
```

---


### Service_environment

ServiceEnvironment resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags that you apply to the service environment to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p> |
| `service_environment_name` | String | ✅ | <p>The name for the service environment. It can be up to 128 characters long and can contain letters, numbers, hyphens (-), and underscores (_).</p> |
| `service_environment_type` | String | ✅ | <p>The type of service environment. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p> |
| `state` | String |  | <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>. The default value is <code>ENABLED</code>.</p> |
| `capacity_limits` | Vec<String> | ✅ | <p>The capacity limits for the service environment. The number of instances a job consumes is the total number of instances requested in the submit training job request resource configuration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service_environment
service_environment = provider.batch.Service_environment {
    service_environment_name = "value"  # <p>The name for the service environment. It can be up to 128 characters long and can contain letters, numbers, hyphens (-), and underscores (_).</p>
    service_environment_type = "value"  # <p>The type of service environment. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    capacity_limits = "value"  # <p>The capacity limits for the service environment. The number of instances a job consumes is the total number of instances requested in the submit training job request resource configuration.</p>
}

```

---


### Job_definitions

JobDefinitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
        <code>DescribeJobDefinitions</code> request. When the results of a
        <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can
      be used to retrieve the next page of results. This value is <code>null</code> when there are
      no more results to return.</p> |
| `job_definitions` | Vec<String> | <p>The list of job definitions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_definitions outputs
job_definitions_id = job_definitions.id
job_definitions_next_token = job_definitions.next_token
job_definitions_job_definitions = job_definitions.job_definitions
```

---


### Jobs

Jobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `jobs` | Vec<String> | <p>The list of jobs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access jobs outputs
jobs_id = jobs.id
jobs_jobs = jobs.jobs
```

---


### Job_queues

JobQueues resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future <code>DescribeJobQueues</code>
      request. When the results of a <code>DescribeJobQueues</code> request exceed
        <code>maxResults</code>, this value can be used to retrieve the next page of results. This
      value is <code>null</code> when there are no more results to return.</p> |
| `job_queues` | Vec<String> | <p>The list of job queues.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_queues outputs
job_queues_id = job_queues.id
job_queues_next_token = job_queues.next_token
job_queues_job_queues = job_queues.job_queues
```

---


### Job_queue

JobQueue resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_state_time_limit_actions` | Vec<String> |  | <p>The set of actions that Batch performs on jobs that remain at the head of the job queue in the specified state longer than specified times. Batch will perform each action after <code>maxTimeSeconds</code> has passed. (<b>Note</b>: The minimum value for maxTimeSeconds is 600 (10 minutes) and its maximum value is 86,400 (24 hours).)</p> |
| `service_environment_order` | Vec<String> |  | <p>A list of service environments that this job queue can use to allocate jobs. All serviceEnvironments must have the same type. A job queue can't have both a serviceEnvironmentOrder and a computeEnvironmentOrder field.</p> |
| `scheduling_policy_arn` | String |  | <p>The Amazon Resource Name (ARN) of the fair-share scheduling policy. Job queues that don't have a fair-share scheduling policy are scheduled in a first-in, first-out (FIFO) model.  After a job queue has a fair-share scheduling policy, it can be replaced but can't be removed.</p>
         <p>The format is
          <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i>
            </code>.</p>
         <p>An example is
        <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
         <p>A job queue without a fair-share scheduling policy is scheduled as a FIFO job queue and can't have a fair-share scheduling policy added. Jobs queues with a fair-share scheduling policy can have a maximum of 500 active share identifiers. When the limit has been reached, submissions of any jobs that add a new share identifier fail.</p> |
| `compute_environment_order` | Vec<String> |  | <p>The set of compute environments mapped to a job queue and their order relative to each
      other. The job scheduler uses this parameter to determine which compute environment runs a
      specific job. Compute environments must be in the <code>VALID</code> state before you can
      associate them with a job queue. You can associate up to three compute environments with a job
      queue. All of the compute environments must be either EC2 (<code>EC2</code> or
        <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and
      Fargate compute environments can't be mixed.</p>
         <note>
            <p>All compute environments that are associated with a job queue must share the same
        architecture. Batch doesn't support mixing compute environment architecture types in a
        single job queue.</p>
         </note> |
| `priority` | i64 | ✅ | <p>The priority of the job queue. Job queues with a higher priority (or a higher integer
      value for the <code>priority</code> parameter) are evaluated first when associated with the
      same compute environment. Priority is determined in descending order. For example, a job queue
      with a priority value of <code>10</code> is given scheduling preference over a job queue with
      a priority value of <code>1</code>. All of the compute environments must be either EC2
        (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or
        <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p> |
| `job_queue_type` | String |  | <p>The type of job queue. For service jobs that run on SageMaker Training, this value is <code>SAGEMAKER_TRAINING</code>. For regular container jobs, this value is <code>EKS</code>, <code>ECS</code>, or <code>ECS_FARGATE</code> depending on the compute environment.</p> |
| `state` | String |  | <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to
      accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the
      queue, but jobs already in the queue can finish.</p> |
| `job_queue_name` | String | ✅ | <p>The name of the job queue. It can be up to 128 letters long. It can contain uppercase and
      lowercase letters, numbers, hyphens (-), and underscores (_).</p> |
| `tags` | HashMap<String, String> |  | <p>The tags that you apply to the job queue to help you categorize and organize your
      resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>
      in <i>Batch User Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job_queue
job_queue = provider.batch.Job_queue {
    priority = "value"  # <p>The priority of the job queue. Job queues with a higher priority (or a higher integer
      value for the <code>priority</code> parameter) are evaluated first when associated with the
      same compute environment. Priority is determined in descending order. For example, a job queue
      with a priority value of <code>10</code> is given scheduling preference over a job queue with
      a priority value of <code>1</code>. All of the compute environments must be either EC2
        (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or
        <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p>
    job_queue_name = "value"  # <p>The name of the job queue. It can be up to 128 letters long. It can contain uppercase and
      lowercase letters, numbers, hyphens (-), and underscores (_).</p>
}

```

---


### Consumable_resource

ConsumableResource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumable_resource_name` | String | ✅ | <p>The name of the consumable resource. Must be unique.</p> |
| `resource_type` | String |  | <p>Indicates whether the resource is available to be re-used after a job completes. Can be 
            one of: </p>
         <ul>
            <li>
               <p>
                  <code>REPLENISHABLE</code> (default)</p>
            </li>
            <li>
               <p>
                  <code>NON_REPLENISHABLE</code>
               </p>
            </li>
         </ul> |
| `tags` | HashMap<String, String> |  | <p>The tags that you apply to the consumable resource to help you categorize and organize your
            resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p> |
| `total_quantity` | i64 |  | <p>The total amount of the consumable resource that is available. Must be non-negative.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `in_use_quantity` | i64 | <p>The amount of the consumable resource that is currently in use.</p> |
| `resource_type` | String | <p>Indicates whether the resource is available to be re-used after a job completes. Can be 
            one of: </p>
         <ul>
            <li>
               <p>
                  <code>REPLENISHABLE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>NON_REPLENISHABLE</code>
               </p>
            </li>
         </ul> |
| `created_at` | i64 | <p>The Unix timestamp (in milliseconds) for when the consumable resource was created.</p> |
| `tags` | HashMap<String, String> | <p>The tags that you apply to the consumable resource to help you categorize and organize your
            resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p> |
| `consumable_resource_arn` | String | <p>The Amazon Resource Name (ARN) of the consumable resource.</p> |
| `total_quantity` | i64 | <p>The total amount of the consumable resource that is available.</p> |
| `available_quantity` | i64 | <p>The amount of the consumable resource that is currently available to use.</p> |
| `consumable_resource_name` | String | <p>The name of the consumable resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create consumable_resource
consumable_resource = provider.batch.Consumable_resource {
    consumable_resource_name = "value"  # <p>The name of the consumable resource. Must be unique.</p>
}

# Access consumable_resource outputs
consumable_resource_id = consumable_resource.id
consumable_resource_in_use_quantity = consumable_resource.in_use_quantity
consumable_resource_resource_type = consumable_resource.resource_type
consumable_resource_created_at = consumable_resource.created_at
consumable_resource_tags = consumable_resource.tags
consumable_resource_consumable_resource_arn = consumable_resource.consumable_resource_arn
consumable_resource_total_quantity = consumable_resource.total_quantity
consumable_resource_available_quantity = consumable_resource.available_quantity
consumable_resource_consumable_resource_name = consumable_resource.consumable_resource_name
```

---


### Service_job

ServiceJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_arn` | String | <p>The Amazon Resource Name (ARN) of the service job.</p> |
| `job_queue` | String | <p>The ARN of the job queue that the service job is associated with.</p> |
| `job_name` | String | <p>The name of the service job.</p> |
| `attempts` | Vec<String> | <p>A list of job attempts associated with the service job.</p> |
| `latest_attempt` | String | <p>The latest attempt associated with the service job.</p> |
| `service_job_type` | String | <p>The type of service job. For SageMaker Training jobs, this value is <code>SAGEMAKER_TRAINING</code>.</p> |
| `stopped_at` | i64 | <p>The Unix timestamp (in milliseconds) for when the service job stopped running.</p> |
| `is_terminated` | bool | <p>Indicates whether the service job has been terminated.</p> |
| `job_id` | String | <p>The job ID for the service job.</p> |
| `started_at` | i64 | <p>The Unix timestamp (in milliseconds) for when the service job was started.</p> |
| `tags` | HashMap<String, String> | <p>The tags that are associated with the service job. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p> |
| `created_at` | i64 | <p>The Unix timestamp (in milliseconds) for when the service job was created.</p> |
| `status` | String | <p>The current status of the service job. </p> |
| `status_reason` | String | <p>A short, human-readable string to provide more details for the current status of the service job.</p> |
| `service_request_payload` | String | <p>The request, in JSON, for the service that the <code>SubmitServiceJob</code> operation is queueing. </p> |
| `share_identifier` | String | <p>The share identifier for the service job. This is used for fair-share scheduling.</p> |
| `retry_strategy` | String | <p>The retry strategy to use for failed service jobs that are submitted with this service job.</p> |
| `scheduling_priority` | i64 | <p>The scheduling priority of the service job. </p> |
| `timeout_config` | String | <p>The timeout configuration for the service job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_job outputs
service_job_id = service_job.id
service_job_job_arn = service_job.job_arn
service_job_job_queue = service_job.job_queue
service_job_job_name = service_job.job_name
service_job_attempts = service_job.attempts
service_job_latest_attempt = service_job.latest_attempt
service_job_service_job_type = service_job.service_job_type
service_job_stopped_at = service_job.stopped_at
service_job_is_terminated = service_job.is_terminated
service_job_job_id = service_job.job_id
service_job_started_at = service_job.started_at
service_job_tags = service_job.tags
service_job_created_at = service_job.created_at
service_job_status = service_job.status
service_job_status_reason = service_job.status_reason
service_job_service_request_payload = service_job.service_request_payload
service_job_share_identifier = service_job.share_identifier
service_job_retry_strategy = service_job.retry_strategy
service_job_scheduling_priority = service_job.scheduling_priority
service_job_timeout_config = service_job.timeout_config
```

---


### Compute_environments

ComputeEnvironments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compute_environments` | Vec<String> | <p>The list of compute environments.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
        <code>DescribeComputeEnvironments</code> request. When the results of a
        <code>DescribeComputeEnvironments</code> request exceed <code>maxResults</code>, this value
      can be used to retrieve the next page of results. This value is <code>null</code> when there
      are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compute_environments outputs
compute_environments_id = compute_environments.id
compute_environments_compute_environments = compute_environments.compute_environments
compute_environments_next_token = compute_environments.next_token
```

---


### Compute_environment

ComputeEnvironment resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `eks_configuration` | String |  | <p>The details for the Amazon EKS cluster that supports the compute environment.</p>
         <note>
            <p>To create a compute environment that uses EKS resources, the caller must have
        permissions to call <code>eks:DescribeCluster</code>.</p>
         </note> |
| `compute_environment_name` | String | ✅ | <p>The name for your compute environment. It can be up to 128 characters long. It can contain uppercase and
 lowercase letters, numbers, hyphens (-), and underscores (_).</p> |
| `context` | String |  | <p>Reserved.</p> |
| `service_role` | String |  | <p>The full Amazon Resource Name (ARN) of the IAM role that allows Batch to make calls to other Amazon Web Services
      services on your behalf. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/service_IAM_role.html">Batch service IAM role</a> in the
        <i>Batch User Guide</i>.</p>
         <important>
            <p>If your account already created the Batch service-linked role, that role is used by
        default for your compute environment unless you specify a different role here. If the
        Batch service-linked role doesn't exist in your account, and no role is specified here,
        the service attempts to create the Batch service-linked role in your account.</p>
         </important>
         <p>If your specified role has a path other than <code>/</code>, then you must specify either
      the full role ARN (recommended) or prefix the role name with the path. For example, if a
      role with the name <code>bar</code> has a path of <code>/foo/</code>, specify
        <code>/foo/bar</code> as the role name. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names">Friendly
        names and paths</a> in the <i>IAM User Guide</i>.</p>
         <note>
            <p>Depending on how you created your Batch service role, its ARN might contain the
          <code>service-role</code> path prefix. When you only specify the name of the service role,
        Batch assumes that your ARN doesn't use the <code>service-role</code> path prefix. Because
        of this, we recommend that you specify the full ARN of your service role when you create
        compute environments.</p>
         </note> |
| `state` | String |  | <p>The state of the compute environment. If the state is <code>ENABLED</code>, then the
      compute environment accepts jobs from a queue and can scale out automatically based on
      queues.</p>
         <p>If the state is <code>ENABLED</code>, then the Batch scheduler can attempt to place jobs
      from an associated job queue on the compute resources within the environment. If the compute
      environment is managed, then it can scale its instances out or in automatically, based on the
      job queue demand.</p>
         <p>If the state is <code>DISABLED</code>, then the Batch scheduler doesn't attempt to place
      jobs within the environment. Jobs in a <code>STARTING</code> or <code>RUNNING</code> state
      continue to progress normally. Managed compute environments in the <code>DISABLED</code> state
      don't scale out. </p>
         <note>
            <p>Compute environments in a <code>DISABLED</code> state may continue to incur billing
        charges. To prevent additional charges, turn off and then delete the compute environment.
        For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environment_parameters.html#compute_environment_state">State</a> in the <i>Batch User Guide</i>.</p>
         </note>
         <p>When an instance is idle, the instance scales down to the <code>minvCpus</code> value.
      However, the instance size doesn't change. For example, consider a <code>c5.8xlarge</code>
      instance with a <code>minvCpus</code> value of <code>4</code> and a <code>desiredvCpus</code>
      value of <code>36</code>. This instance doesn't scale down to a <code>c5.large</code>
      instance.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags that you apply to the compute environment to help you categorize and organize
      your resources. Each tag consists of a key and an optional value. For more information, see
        <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services
        Resources</a> in <i>Amazon Web Services General Reference</i>.</p>
         <p>These tags can be updated or removed using the <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_TagResource.html">TagResource</a> and <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_UntagResource.html">UntagResource</a> API operations. These tags don't propagate to the underlying compute
      resources.</p> |
| `type` | String | ✅ | <p>The type of the compute environment: <code>MANAGED</code> or <code>UNMANAGED</code>. For
      more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>Batch User Guide</i>.</p> |
| `unmanagedv_cpus` | i64 |  | <p>The maximum number of vCPUs for an unmanaged compute environment. This parameter is only
      used for fair-share scheduling to reserve vCPU capacity for new share identifiers. If this
      parameter isn't provided for a fair-share job queue, no vCPU capacity is reserved.</p>
         <note>
            <p>This parameter is only supported when the <code>type</code> parameter is set to
          <code>UNMANAGED</code>.</p>
         </note> |
| `compute_resources` | String |  | <p>Details about the compute resources managed by the compute environment. This parameter is
      required for managed compute environments. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a>
      in the <i>Batch User Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create compute_environment
compute_environment = provider.batch.Compute_environment {
    compute_environment_name = "value"  # <p>The name for your compute environment. It can be up to 128 characters long. It can contain uppercase and
 lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    type = "value"  # <p>The type of the compute environment: <code>MANAGED</code> or <code>UNMANAGED</code>. For
      more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>Batch User Guide</i>.</p>
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

# Create multiple job_queue_snapshot resources
job_queue_snapshot_0 = provider.batch.Job_queue_snapshot {
}
job_queue_snapshot_1 = provider.batch.Job_queue_snapshot {
}
job_queue_snapshot_2 = provider.batch.Job_queue_snapshot {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    job_queue_snapshot = provider.batch.Job_queue_snapshot {
    }
```

---

## Related Documentation

- [AWS Batch Documentation](https://docs.aws.amazon.com/batch/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
