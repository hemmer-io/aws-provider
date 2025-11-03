# Bedrock_agentcore Service



**Resources**: 5

---

## Overview

The bedrock_agentcore service provides access to 5 resource types:

- [Resource_oauth2_token](#resource_oauth2_token) [R]
- [Resource_api_key](#resource_api_key) [R]
- [Workload_access_token_for_jwt](#workload_access_token_for_jwt) [R]
- [Workload_access_token_for_user_id](#workload_access_token_for_user_id) [R]
- [Workload_access_token](#workload_access_token) [R]

---

## Resources


### Resource_oauth2_token

ResourceOauth2Token resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorization_url` | String | <p>The URL to initiate the authorization process, provided when the access token requires user authorization.</p> |
| `session_status` | String | <p>Status indicating whether the user's authorization session is in progress or has failed. This helps determine the next steps in the OAuth2 authentication flow.</p> |
| `session_uri` | String | <p>Unique identifier for the user's authorization session for retrieving OAuth2 tokens. This matches the sessionId from the request and can be used to track the session state.</p> |
| `access_token` | String | <p>The OAuth 2.0 access token to use.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_oauth2_token outputs
resource_oauth2_token_id = resource_oauth2_token.id
resource_oauth2_token_authorization_url = resource_oauth2_token.authorization_url
resource_oauth2_token_session_status = resource_oauth2_token.session_status
resource_oauth2_token_session_uri = resource_oauth2_token.session_uri
resource_oauth2_token_access_token = resource_oauth2_token.access_token
```

---


### Resource_api_key

ResourceApiKey resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_key` | String | <p>The API key associated with the resource requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_api_key outputs
resource_api_key_id = resource_api_key.id
resource_api_key_api_key = resource_api_key.api_key
```

---


### Workload_access_token_for_jwt

WorkloadAccessTokenForJWT resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload_access_token` | String | <p>An opaque token representing the identity of both the workload and the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workload_access_token_for_jwt outputs
workload_access_token_for_jwt_id = workload_access_token_for_jwt.id
workload_access_token_for_jwt_workload_access_token = workload_access_token_for_jwt.workload_access_token
```

---


### Workload_access_token_for_user_id

WorkloadAccessTokenForUserId resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload_access_token` | String | <p>The access token for the specified workload.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workload_access_token_for_user_id outputs
workload_access_token_for_user_id_id = workload_access_token_for_user_id.id
workload_access_token_for_user_id_workload_access_token = workload_access_token_for_user_id.workload_access_token
```

---


### Workload_access_token

WorkloadAccessToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workload_access_token` | String | <p>An opaque token representing the identity of both the workload and the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workload_access_token outputs
workload_access_token_id = workload_access_token.id
workload_access_token_workload_access_token = workload_access_token.workload_access_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_oauth2_token resources
resource_oauth2_token_0 = provider.bedrock_agentcore.Resource_oauth2_token {
}
resource_oauth2_token_1 = provider.bedrock_agentcore.Resource_oauth2_token {
}
resource_oauth2_token_2 = provider.bedrock_agentcore.Resource_oauth2_token {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_oauth2_token = provider.bedrock_agentcore.Resource_oauth2_token {
    }
```

---

## Related Documentation

- [AWS Bedrock_agentcore Documentation](https://docs.aws.amazon.com/bedrock_agentcore/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
