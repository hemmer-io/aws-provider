# Directory_service_data Service



**Resources**: 2

---

## Overview

The directory_service_data service provides access to 2 resource types:

- [Group](#group) [CRUD]
- [User](#user) [CRUD]

---

## Resources


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sam_account_name` | String | ✅ | <p> The name of the group. </p> |
| `group_type` | String |  | <p> The AD group type. For details, see <a href="https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/understand-security-groups#how-active-directory-security-groups-work">Active Directory security group type</a>.</p> |
| `group_scope` | String |  | <p> The scope of the AD group. For details, see <a href="https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/understand-security-groups#group-scope">Active Directory security group scope</a>. </p> |
| `directory_id` | String | ✅ | <p> The identifier (ID) of the directory that's associated with the group. </p> |
| `other_attributes` | HashMap<String, String> |  | <p> An expression that defines one or more attributes with the data type and value of each
      attribute. </p> |
| `client_token` | String |  | <p> A unique and case-sensitive identifier that you provide to make sure the idempotency of
      the request, so multiple identical calls have the same effect as one single call. </p>
         <p> A client token is valid for 8 hours after the first request that uses it completes. After
      8 hours, any request with the same client token is treated as a new request. If the request
      succeeds, any future uses of that token will be idempotent for another 8 hours. </p>
         <p> If you submit a request with the same client token but change one of the other parameters
      within the 8-hour idempotency window, Directory Service Data returns an <code>ConflictException</code>. </p>
         <note>
            <p> This parameter is optional when using the CLI or SDK. </p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sam_account_name` | String | <p> The name of the group. </p> |
| `realm` | String | <p> The domain name that's associated with the group. </p> |
| `group_scope` | String | <p> The scope of the AD group. For details, see <a href="https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/understand-security-groups#group-scope">Active Directory security groups</a>. </p> |
| `other_attributes` | HashMap<String, String> | <p> The attribute values that are returned for the attribute names that are included in the
      request. </p> |
| `distinguished_name` | String | <p> The <a href="https://learn.microsoft.com/en-us/windows/win32/ad/object-names-and-identities#distinguished-name">distinguished name</a> of the object. </p> |
| `group_type` | String | <p> The AD group type. For details, see <a href="https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/understand-security-groups#how-active-directory-security-groups-work">Active Directory security group type</a>. </p> |
| `directory_id` | String | <p> The identifier (ID) of the directory that's associated with the group. </p> |
| `sid` | String | <p> The unique security identifier (SID) of the group. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.directory_service_data.Group {
    sam_account_name = "value"  # <p> The name of the group. </p>
    directory_id = "value"  # <p> The identifier (ID) of the directory that's associated with the group. </p>
}

# Access group outputs
group_id = group.id
group_sam_account_name = group.sam_account_name
group_realm = group.realm
group_group_scope = group.group_scope
group_other_attributes = group.other_attributes
group_distinguished_name = group.distinguished_name
group_group_type = group.group_type
group_directory_id = group.directory_id
group_sid = group.sid
```

---


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `surname` | String |  | <p> The last name of the user. </p> |
| `email_address` | String |  | <p> The email address of the user. </p> |
| `client_token` | String |  | <p> A unique and case-sensitive identifier that you provide to make sure the idempotency of
      the request, so multiple identical calls have the same effect as one single call. </p>
         <p> A client token is valid for 8 hours after the first request that uses it completes. After
      8 hours, any request with the same client token is treated as a new request. If the request
      succeeds, any future uses of that token will be idempotent for another 8 hours. </p>
         <p> If you submit a request with the same client token but change one of the other parameters
      within the 8-hour idempotency window, Directory Service Data returns an <code>ConflictException</code>. </p>
         <note>
            <p> This parameter is optional when using the CLI or SDK. </p>
         </note> |
| `other_attributes` | HashMap<String, String> |  | <p> An expression that defines one or more attribute names with the data type and value of
      each attribute. A key is an attribute name, and the value is a list of maps. For a list of
      supported attributes, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ad_data_attributes.html">Directory Service Data Attributes</a>. </p>
         <note>
            <p> Attribute names are case insensitive. </p>
         </note> |
| `directory_id` | String | ✅ | <p> The identifier (ID) of the directory that’s associated with the user. </p> |
| `sam_account_name` | String | ✅ | <p> The name of the user. </p> |
| `given_name` | String |  | <p> The first name of the user. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `other_attributes` | HashMap<String, String> | <p> The attribute values that are returned for the attribute names that are included in the
      request. </p>
         <note>
            <p> Attribute names are case insensitive. </p>
         </note> |
| `surname` | String | <p> The last name of the user. </p> |
| `email_address` | String | <p> The email address of the user. </p> |
| `directory_id` | String | <p> The identifier (ID) of the directory that's associated with the user. </p> |
| `user_principal_name` | String | <p> The UPN that is an Internet-style login name for a user and is based on the Internet
      standard <a href="https://datatracker.ietf.org/doc/html/rfc822">RFC 822</a>. The UPN is shorter
      than the distinguished name and easier to remember. </p> |
| `realm` | String | <p> The domain name that's associated with the user. </p> |
| `sid` | String | <p> The unique security identifier (SID) of the user. </p> |
| `sam_account_name` | String | <p> The name of the user. </p> |
| `distinguished_name` | String | <p> The <a href="https://learn.microsoft.com/en-us/windows/win32/ad/object-names-and-identities#distinguished-name">distinguished name</a> of the object. </p> |
| `given_name` | String | <p> The first name of the user. </p> |
| `enabled` | bool | <p> Indicates whether the user account is active. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.directory_service_data.User {
    directory_id = "value"  # <p> The identifier (ID) of the directory that’s associated with the user. </p>
    sam_account_name = "value"  # <p> The name of the user. </p>
}

# Access user outputs
user_id = user.id
user_other_attributes = user.other_attributes
user_surname = user.surname
user_email_address = user.email_address
user_directory_id = user.directory_id
user_user_principal_name = user.user_principal_name
user_realm = user.realm
user_sid = user.sid
user_sam_account_name = user.sam_account_name
user_distinguished_name = user.distinguished_name
user_given_name = user.given_name
user_enabled = user.enabled
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple group resources
group_0 = provider.directory_service_data.Group {
    sam_account_name = "value-0"
    directory_id = "value-0"
}
group_1 = provider.directory_service_data.Group {
    sam_account_name = "value-1"
    directory_id = "value-1"
}
group_2 = provider.directory_service_data.Group {
    sam_account_name = "value-2"
    directory_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    group = provider.directory_service_data.Group {
        sam_account_name = "production-value"
        directory_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Directory_service_data Documentation](https://docs.aws.amazon.com/directory_service_data/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
