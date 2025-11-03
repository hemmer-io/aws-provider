# Redshift_serverless Service



**Resources**: 4

---

## Overview

The redshift_serverless service provides access to 4 resource types:

- [Track](#track) [R]
- [Resource_policy](#resource_policy) [CRD]
- [Credentials](#credentials) [R]
- [Custom_domain_association](#custom_domain_association) [CRUD]

---

## Resources


### Track

Track resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `track` | String | <p>The version of the specified track.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access track outputs
track_id = track.id
track_track = track.track
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the account to create or update a resource policy for.</p> |
| `policy` | String | ✅ | <p>The policy to create or update. For example, the following policy grants a user authorization to restore a snapshot.</p>
         <p>
            <code>"{\"Version\": \"2012-10-17\", \"Statement\" : 
            [{ \"Sid\": \"AllowUserRestoreFromSnapshot\", \"Principal\":{\"AWS\": 
            [\"739247239426\"]}, \"Action\": [\"redshift-serverless:RestoreFromSnapshot\"]
            , \"Effect\": \"Allow\" }]}"</code>
         </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_policy` | String | <p>The returned resource policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.redshift_serverless.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the account to create or update a resource policy for.</p>
    policy = "value"  # <p>The policy to create or update. For example, the following policy grants a user authorization to restore a snapshot.</p>
         <p>
            <code>"{\"Version\": \"2012-10-17\", \"Statement\" : 
            [{ \"Sid\": \"AllowUserRestoreFromSnapshot\", \"Principal\":{\"AWS\": 
            [\"739247239426\"]}, \"Action\": [\"redshift-serverless:RestoreFromSnapshot\"]
            , \"Effect\": \"Allow\" }]}"</code>
         </p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_resource_policy = resource_policy.resource_policy
```

---


### Credentials

Credentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `db_user` | String | <p>A database user name that is authorized to log on to the database <code>DbName</code>
         using the password <code>DbPassword</code>. If the specified <code>DbUser</code> exists in the database, 
         the new user name has the same database privileges as the the user named in 
         <code>DbUser</code>. By default, the user is added to PUBLIC.</p> |
| `db_password` | String | <p>A temporary password that authorizes the user name returned by 
         <code>DbUser</code> to log on to the database <code>DbName</code>.</p> |
| `expiration` | String | <p>The date and time the password in <code>DbPassword</code> expires.</p> |
| `next_refresh_time` | String | <p>The date and time of when the <code>DbUser</code> and <code>DbPassword</code> 
         authorization refreshes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access credentials outputs
credentials_id = credentials.id
credentials_db_user = credentials.db_user
credentials_db_password = credentials.db_password
credentials_expiration = credentials.expiration
credentials_next_refresh_time = credentials.next_refresh_time
```

---


### Custom_domain_association

CustomDomainAssociation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_domain_name` | String | ✅ | <p>The custom domain name to associate with the workgroup.</p> |
| `workgroup_name` | String | ✅ | <p>The name of the workgroup associated with the database.</p> |
| `custom_domain_certificate_arn` | String | ✅ | <p>The custom domain name’s certificate Amazon resource name (ARN).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_domain_name` | String | <p>The custom domain name associated with the workgroup.</p> |
| `custom_domain_certificate_arn` | String | <p>The custom domain name’s certificate Amazon resource name (ARN).</p> |
| `workgroup_name` | String | <p>The name of the workgroup associated with the database.</p> |
| `custom_domain_certificate_expiry_time` | String | <p>The expiration time for the certificate.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_domain_association
custom_domain_association = provider.redshift_serverless.Custom_domain_association {
    custom_domain_name = "value"  # <p>The custom domain name to associate with the workgroup.</p>
    workgroup_name = "value"  # <p>The name of the workgroup associated with the database.</p>
    custom_domain_certificate_arn = "value"  # <p>The custom domain name’s certificate Amazon resource name (ARN).</p>
}

# Access custom_domain_association outputs
custom_domain_association_id = custom_domain_association.id
custom_domain_association_custom_domain_name = custom_domain_association.custom_domain_name
custom_domain_association_custom_domain_certificate_arn = custom_domain_association.custom_domain_certificate_arn
custom_domain_association_workgroup_name = custom_domain_association.workgroup_name
custom_domain_association_custom_domain_certificate_expiry_time = custom_domain_association.custom_domain_certificate_expiry_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple track resources
track_0 = provider.redshift_serverless.Track {
}
track_1 = provider.redshift_serverless.Track {
}
track_2 = provider.redshift_serverless.Track {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    track = provider.redshift_serverless.Track {
    }
```

---

## Related Documentation

- [AWS Redshift_serverless Documentation](https://docs.aws.amazon.com/redshift_serverless/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
