# Sts Service



**Resources**: 4

---

## Overview

The sts service provides access to 4 resource types:

- [Federation_token](#federation_token) [R]
- [Access_key_info](#access_key_info) [R]
- [Session_token](#session_token) [R]
- [Caller_identity](#caller_identity) [R]

---

## Resources


### Federation_token

FederationToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `federated_user` | String | <p>Identifiers for the federated user associated with the credentials (such as
            <code>arn:aws:sts::123456789012:federated-user/Bob</code> or
            <code>123456789012:Bob</code>). You can use the federated user's ARN in your
         resource-based policies, such as an Amazon S3 bucket policy. </p> |
| `packed_policy_size` | i64 | <p>A percentage value that indicates the packed size of the session policies and session 
      tags combined passed in the request. The request fails if the packed size is greater than 100 percent, 
      which means the policies and tags exceeded the allowed space.</p> |
| `credentials` | String | <p>The temporary security credentials, which include an access key ID, a secret access key,
         and a security (or session) token.</p>
         <note>
            <p>The size of the security token that STS API operations return is not fixed. We
        strongly recommend that you make no assumptions about the maximum size.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access federation_token outputs
federation_token_id = federation_token.id
federation_token_federated_user = federation_token.federated_user
federation_token_packed_policy_size = federation_token.packed_policy_size
federation_token_credentials = federation_token.credentials
```

---


### Access_key_info

AccessKeyInfo resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account` | String | <p>The number used to identify the Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_key_info outputs
access_key_info_id = access_key_info.id
access_key_info_account = access_key_info.account
```

---


### Session_token

SessionToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credentials` | String | <p>The temporary security credentials, which include an access key ID, a secret access key,
         and a security (or session) token.</p>
         <note>
            <p>The size of the security token that STS API operations return is not fixed. We
        strongly recommend that you make no assumptions about the maximum size.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access session_token outputs
session_token_id = session_token.id
session_token_credentials = session_token.credentials
```

---


### Caller_identity

CallerIdentity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_id` | String | <p>The unique identifier of the calling entity. The exact value depends on the type of
         entity that is making the call. The values returned are those listed in the <b>aws:userid</b> column in the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_variables.html#principaltable">Principal
            table</a> found on the <b>Policy Variables</b> reference
         page in the <i>IAM User Guide</i>.</p> |
| `account` | String | <p>The Amazon Web Services account ID number of the account that owns or contains the calling
         entity.</p> |
| `arn` | String | <p>The Amazon Web Services ARN associated with the calling entity.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access caller_identity outputs
caller_identity_id = caller_identity.id
caller_identity_user_id = caller_identity.user_id
caller_identity_account = caller_identity.account
caller_identity_arn = caller_identity.arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple federation_token resources
federation_token_0 = provider.sts.Federation_token {
}
federation_token_1 = provider.sts.Federation_token {
}
federation_token_2 = provider.sts.Federation_token {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    federation_token = provider.sts.Federation_token {
    }
```

---

## Related Documentation

- [AWS Sts Documentation](https://docs.aws.amazon.com/sts/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
