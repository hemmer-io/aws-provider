# Iot_jobs_data_plane Service



**Resources**: 2

---

## Overview

The iot_jobs_data_plane service provides access to 2 resource types:

- [Pending_job_executions](#pending_job_executions) [R]
- [Job_execution](#job_execution) [RU]

---

## Resources


### Pending_job_executions

PendingJobExecutions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `in_progress_jobs` | Vec<String> | <p>A list of JobExecutionSummary objects with status IN_PROGRESS.</p> |
| `queued_jobs` | Vec<String> | <p>A list of JobExecutionSummary objects with status QUEUED.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pending_job_executions outputs
pending_job_executions_id = pending_job_executions.id
pending_job_executions_in_progress_jobs = pending_job_executions.in_progress_jobs
pending_job_executions_queued_jobs = pending_job_executions.queued_jobs
```

---


### Job_execution

JobExecution resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thing_name` | String | ✅ | <p>The name of the thing associated with the device.</p> |
| `status` | String | ✅ | <p>The new status for the job execution (IN_PROGRESS, FAILED, SUCCESS, or REJECTED).
         This must be specified on every update.</p> |
| `job_id` | String | ✅ | <p>The unique identifier assigned to this job when it was created.</p> |
| `execution_number` | i64 |  | <p>Optional. A number that identifies a particular job execution on a particular
         device.</p> |
| `expected_version` | i64 |  | <p>Optional. The expected current version of the job execution. Each time you update the
         job execution, its version is incremented. If the version of the job execution stored in
         Jobs does not match, the update is rejected with a VersionMismatch error, and an
         ErrorResponse that contains the current job execution status data is returned. (This makes
         it unnecessary to perform a separate DescribeJobExecution request in order to obtain the
         job execution status data.)</p> |
| `include_job_execution_state` | bool |  | <p>Optional. When included and set to true, the response contains the JobExecutionState
         data. The default is false.</p> |
| `include_job_document` | bool |  | <p>Optional. When set to true, the response contains the job document. The default is
         false.</p> |
| `step_timeout_in_minutes` | i64 |  | <p>Specifies the amount of time this device has to finish execution of this job. If the job
         execution status is not set to a terminal state before this timer expires, or before the
         timer is reset (by again calling <code>UpdateJobExecution</code>, setting the status to
            <code>IN_PROGRESS</code>, and specifying a new timeout value in this field) the job
         execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting or
         resetting the step timeout has no effect on the in progress timeout that may have been
         specified when the job was created (<code>CreateJob</code> using field
            <code>timeoutConfig</code>).</p>
         <p>Valid values for this parameter range from 1 to 10080 (1 minute to 7 days). A value of
         -1 is also valid and will cancel the current step timer (created by an earlier use of
            <code>UpdateJobExecutionRequest</code>).</p> |
| `status_details` | HashMap<String, String> |  | <p> Optional. A collection of name/value pairs that describe the status of the job
         execution. If not specified, the statusDetails are unchanged.</p>
         <p>The maximum length of the value in the name/value pair is 1,024 characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution` | String | <p>Contains data about a job execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_execution outputs
job_execution_id = job_execution.id
job_execution_execution = job_execution.execution
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple pending_job_executions resources
pending_job_executions_0 = provider.iot_jobs_data_plane.Pending_job_executions {
}
pending_job_executions_1 = provider.iot_jobs_data_plane.Pending_job_executions {
}
pending_job_executions_2 = provider.iot_jobs_data_plane.Pending_job_executions {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    pending_job_executions = provider.iot_jobs_data_plane.Pending_job_executions {
    }
```

---

## Related Documentation

- [AWS Iot_jobs_data_plane Documentation](https://docs.aws.amazon.com/iot_jobs_data_plane/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
