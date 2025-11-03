# Amplifybackend Service



**Resources**: 8

---

## Overview

The amplifybackend service provides access to 8 resource types:

- [Backend_api](#backend_api) [CRUD]
- [Backend_auth](#backend_auth) [CRUD]
- [Backend_config](#backend_config) [CU]
- [Token](#token) [CRD]
- [Backend_job](#backend_job) [RU]
- [Backend_storage](#backend_storage) [CRUD]
- [Backend_api_models](#backend_api_models) [R]
- [Backend](#backend) [CRD]

---

## Resources


### Backend_api

BackendAPI resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `backend_environment_name` | String | ✅ | <p>The name of the backend environment.</p> |
| `app_id` | String | ✅ | <p>The app ID.</p> |
| `resource_name` | String | ✅ | <p>The name of this resource.</p> |
| `resource_config` | String | ✅ | <p>The resource configuration for this request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backend_environment_name` | String | <p>The name of the backend environment.</p> |
| `error` | String | <p>If the request fails, this error is returned.</p> |
| `resource_config` | String | <p>The resource configuration for this response object.</p> |
| `app_id` | String | <p>The app ID.</p> |
| `resource_name` | String | <p>The name of this resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backend_api
backend_api = provider.amplifybackend.Backend_api {
    backend_environment_name = "value"  # <p>The name of the backend environment.</p>
    app_id = "value"  # <p>The app ID.</p>
    resource_name = "value"  # <p>The name of this resource.</p>
    resource_config = "value"  # <p>The resource configuration for this request.</p>
}

# Access backend_api outputs
backend_api_id = backend_api.id
backend_api_backend_environment_name = backend_api.backend_environment_name
backend_api_error = backend_api.error
backend_api_resource_config = backend_api.resource_config
backend_api_app_id = backend_api.app_id
backend_api_resource_name = backend_api.resource_name
```

---


### Backend_auth

BackendAuth resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_config` | String | ✅ | <p>The resource configuration for this request object.</p> |
| `app_id` | String | ✅ | <p>The app ID.</p> |
| `resource_name` | String | ✅ | <p>The name of this resource.</p> |
| `backend_environment_name` | String | ✅ | <p>The name of the backend environment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_config` | String | <p>The resource configuration for authorization requests to the backend of your Amplify project.</p> |
| `backend_environment_name` | String | <p>The name of the backend environment.</p> |
| `app_id` | String | <p>The app ID.</p> |
| `error` | String | <p>If the request fails, this error is returned.</p> |
| `resource_name` | String | <p>The name of this resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backend_auth
backend_auth = provider.amplifybackend.Backend_auth {
    resource_config = "value"  # <p>The resource configuration for this request object.</p>
    app_id = "value"  # <p>The app ID.</p>
    resource_name = "value"  # <p>The name of this resource.</p>
    backend_environment_name = "value"  # <p>The name of the backend environment.</p>
}

# Access backend_auth outputs
backend_auth_id = backend_auth.id
backend_auth_resource_config = backend_auth.resource_config
backend_auth_backend_environment_name = backend_auth.backend_environment_name
backend_auth_app_id = backend_auth.app_id
backend_auth_error = backend_auth.error
backend_auth_resource_name = backend_auth.resource_name
```

---


### Backend_config

BackendConfig resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The app ID.</p> |
| `backend_manager_app_id` | String |  | <p>The app ID for the backend manager.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backend_config
backend_config = provider.amplifybackend.Backend_config {
    app_id = "value"  # <p>The app ID.</p>
}

```

---


### Token

Token resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The app ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ttl` | String | <p>The expiry time for the one-time generated token code.</p> |
| `challenge_code` | String | <p>The one-time challenge code for authenticating into the Amplify Admin UI.</p> |
| `session_id` | String | <p>A unique ID provided when creating a new challenge token.</p> |
| `app_id` | String | <p>The app ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create token
token = provider.amplifybackend.Token {
    app_id = "value"  # <p>The app ID.</p>
}

# Access token outputs
token_id = token.id
token_ttl = token.ttl
token_challenge_code = token.challenge_code
token_session_id = token.session_id
token_app_id = token.app_id
```

---


### Backend_job

BackendJob resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The app ID.</p> |
| `backend_environment_name` | String | ✅ | <p>The name of the backend environment.</p> |
| `status` | String |  | <p>Filters the list of response objects to include only those with the specified status.</p> |
| `job_id` | String | ✅ | <p>The ID for the job.</p> |
| `operation` | String |  | <p>Filters the list of response objects to include only those with the specified operation name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_id` | String | <p>The app ID.</p> |
| `error` | String | <p>If the request fails, this error is returned.</p> |
| `operation` | String | <p>The name of the operation.</p> |
| `backend_environment_name` | String | <p>The name of the backend environment.</p> |
| `create_time` | String | <p>The time when the job was created.</p> |
| `status` | String | <p>The current status of the request.</p> |
| `job_id` | String | <p>The ID for the job.</p> |
| `update_time` | String | <p>The time when the job was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access backend_job outputs
backend_job_id = backend_job.id
backend_job_app_id = backend_job.app_id
backend_job_error = backend_job.error
backend_job_operation = backend_job.operation
backend_job_backend_environment_name = backend_job.backend_environment_name
backend_job_create_time = backend_job.create_time
backend_job_status = backend_job.status
backend_job_job_id = backend_job.job_id
backend_job_update_time = backend_job.update_time
```

---


### Backend_storage

BackendStorage resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The app ID.</p> |
| `resource_config` | String | ✅ | <p>The resource configuration for creating backend storage.</p> |
| `backend_environment_name` | String | ✅ | <p>The name of the backend environment.</p> |
| `resource_name` | String | ✅ | <p>The name of the storage resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_config` | String | <p>The resource configuration for the backend storage resource.</p> |
| `backend_environment_name` | String | <p>The name of the backend environment.</p> |
| `resource_name` | String | <p>The name of the storage resource.</p> |
| `app_id` | String | <p>The app ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backend_storage
backend_storage = provider.amplifybackend.Backend_storage {
    app_id = "value"  # <p>The app ID.</p>
    resource_config = "value"  # <p>The resource configuration for creating backend storage.</p>
    backend_environment_name = "value"  # <p>The name of the backend environment.</p>
    resource_name = "value"  # <p>The name of the storage resource.</p>
}

# Access backend_storage outputs
backend_storage_id = backend_storage.id
backend_storage_resource_config = backend_storage.resource_config
backend_storage_backend_environment_name = backend_storage.backend_environment_name
backend_storage_resource_name = backend_storage.resource_name
backend_storage_app_id = backend_storage.app_id
```

---


### Backend_api_models

BackendAPIModels resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `models` | String | <p>Stringified JSON of the datastore model.</p> |
| `status` | String | <p>The current status of the request.</p> |
| `model_introspection_schema` | String | <p>Stringified JSON of the model introspection schema for an existing backend API resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access backend_api_models outputs
backend_api_models_id = backend_api_models.id
backend_api_models_models = backend_api_models.models
backend_api_models_status = backend_api_models.status
backend_api_models_model_introspection_schema = backend_api_models.model_introspection_schema
```

---


### Backend

Backend resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The app ID.</p> |
| `app_name` | String | ✅ | <p>The name of the app.</p> |
| `resource_name` | String |  | <p>The name of the resource.</p> |
| `backend_environment_name` | String | ✅ | <p>The name of the backend environment.</p> |
| `resource_config` | String |  | <p>The resource configuration for creating a backend.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | <p>If the request failed, this is the returned error.</p> |
| `amplify_feature_flags` | String | <p>A stringified version of the cli.json file for your Amplify project.</p> |
| `backend_environment_name` | String | <p>The name of the backend environment.</p> |
| `app_id` | String | <p>The app ID.</p> |
| `amplify_meta_config` | String | <p>A stringified version of the current configs for your Amplify project.</p> |
| `app_name` | String | <p>The name of the app.</p> |
| `backend_environment_list` | Vec<String> | <p>A list of backend environments in an array.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backend
backend = provider.amplifybackend.Backend {
    app_id = "value"  # <p>The app ID.</p>
    app_name = "value"  # <p>The name of the app.</p>
    backend_environment_name = "value"  # <p>The name of the backend environment.</p>
}

# Access backend outputs
backend_id = backend.id
backend_error = backend.error
backend_amplify_feature_flags = backend.amplify_feature_flags
backend_backend_environment_name = backend.backend_environment_name
backend_app_id = backend.app_id
backend_amplify_meta_config = backend.amplify_meta_config
backend_app_name = backend.app_name
backend_backend_environment_list = backend.backend_environment_list
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple backend_api resources
backend_api_0 = provider.amplifybackend.Backend_api {
    backend_environment_name = "value-0"
    app_id = "value-0"
    resource_name = "value-0"
    resource_config = "value-0"
}
backend_api_1 = provider.amplifybackend.Backend_api {
    backend_environment_name = "value-1"
    app_id = "value-1"
    resource_name = "value-1"
    resource_config = "value-1"
}
backend_api_2 = provider.amplifybackend.Backend_api {
    backend_environment_name = "value-2"
    app_id = "value-2"
    resource_name = "value-2"
    resource_config = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backend_api = provider.amplifybackend.Backend_api {
        backend_environment_name = "production-value"
        app_id = "production-value"
        resource_name = "production-value"
        resource_config = "production-value"
    }
```

---

## Related Documentation

- [AWS Amplifybackend Documentation](https://docs.aws.amazon.com/amplifybackend/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
