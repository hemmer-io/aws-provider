# Emr_containers Service



**Resources**: 6

---

## Overview

The emr_containers service provides access to 6 resource types:

- [Job_run](#job_run) [R]
- [Managed_endpoint](#managed_endpoint) [CRD]
- [Virtual_cluster](#virtual_cluster) [CRD]
- [Security_configuration](#security_configuration) [CR]
- [Managed_endpoint_session_credentials](#managed_endpoint_session_credentials) [R]
- [Job_template](#job_template) [CRD]

---

## Resources


### Job_run

JobRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_run` | String | <p>The output displays information about a job run.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_run outputs
job_run_id = job_run.id
job_run_job_run = job_run.job_run
```

---


### Managed_endpoint

ManagedEndpoint resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>The type of the managed endpoint.</p> |
| `name` | String | ✅ | <p>The name of the managed endpoint.</p> |
| `execution_role_arn` | String | ✅ | <p>The ARN of the execution role.</p> |
| `certificate_arn` | String |  | <p>The certificate ARN provided by users for the managed endpoint. This field is under
         deprecation and will be removed in future releases.</p> |
| `client_token` | String | ✅ | <p>The client idempotency token for this create call.</p> |
| `release_label` | String | ✅ | <p>The Amazon EMR release version.</p> |
| `virtual_cluster_id` | String | ✅ | <p>The ID of the virtual cluster for which a managed endpoint is created.</p> |
| `configuration_overrides` | String |  | <p>The configuration settings that will be used to override existing configurations.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags of the managed endpoint. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint` | String | <p>This output displays information about a managed endpoint.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create managed_endpoint
managed_endpoint = provider.emr_containers.Managed_endpoint {
    type = "value"  # <p>The type of the managed endpoint.</p>
    name = "value"  # <p>The name of the managed endpoint.</p>
    execution_role_arn = "value"  # <p>The ARN of the execution role.</p>
    client_token = "value"  # <p>The client idempotency token for this create call.</p>
    release_label = "value"  # <p>The Amazon EMR release version.</p>
    virtual_cluster_id = "value"  # <p>The ID of the virtual cluster for which a managed endpoint is created.</p>
}

# Access managed_endpoint outputs
managed_endpoint_id = managed_endpoint.id
managed_endpoint_endpoint = managed_endpoint.endpoint
```

---


### Virtual_cluster

VirtualCluster resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_configuration_id` | String |  | <p>The ID of the security configuration.</p> |
| `container_provider` | String | ✅ | <p>The container provider of the virtual cluster.</p> |
| `name` | String | ✅ | <p>The specified name of the virtual cluster.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags assigned to the virtual cluster.</p> |
| `client_token` | String | ✅ | <p>The client token of the virtual cluster.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `virtual_cluster` | String | <p>This output displays information about the specified virtual cluster.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create virtual_cluster
virtual_cluster = provider.emr_containers.Virtual_cluster {
    container_provider = "value"  # <p>The container provider of the virtual cluster.</p>
    name = "value"  # <p>The specified name of the virtual cluster.</p>
    client_token = "value"  # <p>The client token of the virtual cluster.</p>
}

# Access virtual_cluster outputs
virtual_cluster_id = virtual_cluster.id
virtual_cluster_virtual_cluster = virtual_cluster.virtual_cluster
```

---


### Security_configuration

SecurityConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_configuration_data` | String | ✅ | <p>Security configuration input for the request.</p> |
| `client_token` | String | ✅ | <p>The client idempotency token to use when creating the security configuration.</p> |
| `container_provider` | String |  | <p>The container provider associated with the security configuration.</p> |
| `name` | String | ✅ | <p>The name of the security configuration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to add to the security configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_configuration` | String | <p>Details of the security configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create security_configuration
security_configuration = provider.emr_containers.Security_configuration {
    security_configuration_data = "value"  # <p>Security configuration input for the request.</p>
    client_token = "value"  # <p>The client idempotency token to use when creating the security configuration.</p>
    name = "value"  # <p>The name of the security configuration.</p>
}

# Access security_configuration outputs
security_configuration_id = security_configuration.id
security_configuration_security_configuration = security_configuration.security_configuration
```

---


### Managed_endpoint_session_credentials

ManagedEndpointSessionCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expires_at` | String | <p>The date and time when the session token will expire.</p> |
| `id` | String | <p>The identifier of the session token returned.</p> |
| `credentials` | String | <p>The structure containing the session credentials.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_endpoint_session_credentials outputs
managed_endpoint_session_credentials_id = managed_endpoint_session_credentials.id
managed_endpoint_session_credentials_expires_at = managed_endpoint_session_credentials.expires_at
managed_endpoint_session_credentials_id = managed_endpoint_session_credentials.id
managed_endpoint_session_credentials_credentials = managed_endpoint_session_credentials.credentials
```

---


### Job_template

JobTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_arn` | String |  | <p>The KMS key ARN used to encrypt the job template.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags that are associated with the job template.</p> |
| `client_token` | String | ✅ | <p>The client token of the job template.</p> |
| `name` | String | ✅ | <p>The specified name of the job template.</p> |
| `job_template_data` | String | ✅ | <p>The job template data which holds values of StartJobRun API request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_template` | String | <p>This output displays information about the specified job template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job_template
job_template = provider.emr_containers.Job_template {
    client_token = "value"  # <p>The client token of the job template.</p>
    name = "value"  # <p>The specified name of the job template.</p>
    job_template_data = "value"  # <p>The job template data which holds values of StartJobRun API request.</p>
}

# Access job_template outputs
job_template_id = job_template.id
job_template_job_template = job_template.job_template
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple job_run resources
job_run_0 = provider.emr_containers.Job_run {
}
job_run_1 = provider.emr_containers.Job_run {
}
job_run_2 = provider.emr_containers.Job_run {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    job_run = provider.emr_containers.Job_run {
    }
```

---

## Related Documentation

- [AWS Emr_containers Documentation](https://docs.aws.amazon.com/emr_containers/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
