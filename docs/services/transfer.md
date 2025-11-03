# Transfer Service



**Resources**: 5

---

## Overview

The transfer service provides access to 5 resource types:

- [Security_policy](#security_policy) [R]
- [Execution](#execution) [R]
- [Host_key](#host_key) [RUD]
- [Access](#access) [CRUD]
- [Ssh_public_key](#ssh_public_key) [D]

---

## Resources


### Security_policy

SecurityPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_policy` | String | <p>An array containing the properties of the security policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_policy outputs
security_policy_id = security_policy.id
security_policy_security_policy = security_policy.security_policy
```

---


### Execution

Execution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution` | String | <p>The structure that contains the details of the workflow' execution.</p> |
| `workflow_id` | String | <p>A unique identifier for the workflow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access execution outputs
execution_id = execution.id
execution_execution = execution.execution
execution_workflow_id = execution.workflow_id
```

---


### Host_key

HostKey resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>An updated description for the host key.</p> |
| `host_key_id` | String | ✅ | <p>The identifier of the host key that you are updating.</p> |
| `server_id` | String | ✅ | <p>The identifier of the server that contains the host key that you are updating.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `host_key` | String | <p>Returns the details for the specified host key.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access host_key outputs
host_key_id = host_key.id
host_key_host_key = host_key.host_key
```

---


### Access

Access resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | <p>A session policy for your user so that you can use the same Identity and Access Management (IAM) role across multiple users. This policy scopes down a user's access to portions of their Amazon S3 bucket. Variables that you can use inside this policy include <code>${Transfer:UserName}</code>, <code>${Transfer:HomeDirectory}</code>, and <code>${Transfer:HomeBucket}</code>.</p> <note> <p>This policy applies only when the domain of <code>ServerId</code> is Amazon S3. Amazon EFS does not use session policies.</p> <p>For session policies, Transfer Family stores the policy as a JSON blob, instead of the Amazon Resource Name (ARN) of the policy. You save the policy as a JSON blob and pass it in the <code>Policy</code> argument.</p> <p>For an example of a session policy, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/session-policy.html">Example session policy</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html">AssumeRole</a> in the <i>Security Token Service API Reference</i>.</p> </note> |
| `posix_profile` | String |  |  |
| `home_directory` | String |  | <p>The landing directory (folder) for a user when they log in to the server using the client.</p> <p>A <code>HomeDirectory</code> example is <code>/bucket_name/home/mydirectory</code>.</p> <note> <p>You can use the <code>HomeDirectory</code> parameter for <code>HomeDirectoryType</code> when it is set to either <code>PATH</code> or <code>LOGICAL</code>.</p> </note> |
| `home_directory_mappings` | Vec<String> |  | <p>Logical directory mappings that specify what Amazon S3 or Amazon EFS paths and keys should be visible to your user and how you want to make them visible. You must specify the <code>Entry</code> and <code>Target</code> pair, where <code>Entry</code> shows how the path is made visible and <code>Target</code> is the actual Amazon S3 or Amazon EFS path. If you only specify a target, it is displayed as is. You also must ensure that your Identity and Access Management (IAM) role provides access to paths in <code>Target</code>. This value can be set only when <code>HomeDirectoryType</code> is set to <i>LOGICAL</i>.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example.</p> <p> <code>[ { "Entry": "/directory1", "Target": "/bucket_name/home/mydirectory" } ]</code> </p> <p>In most cases, you can use this value instead of the session policy to lock down your user to the designated home directory ("<code>chroot</code>"). To do this, you can set <code>Entry</code> to <code>/</code> and set <code>Target</code> to the <code>HomeDirectory</code> parameter value.</p> <p>The following is an <code>Entry</code> and <code>Target</code> pair example for <code>chroot</code>.</p> <p> <code>[ { "Entry": "/", "Target": "/bucket_name/home/mydirectory" } ]</code> </p> |
| `role` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that controls your users' access to your Amazon S3 bucket or Amazon EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or Amazon EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p> |
| `server_id` | String | ✅ | <p>A system-assigned unique identifier for a server instance. This is the specific server that you added your user to.</p> |
| `external_id` | String | ✅ | <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regular expression used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p> |
| `home_directory_type` | String |  | <p>The type of landing directory (folder) that you want your users' home directory to be when they log in to the server. If you set it to <code>PATH</code>, the user will see the absolute Amazon S3 bucket or Amazon EFS path as is in their file transfer protocol clients. If you set it to <code>LOGICAL</code>, you need to provide mappings in the <code>HomeDirectoryMappings</code> for how you want to make Amazon S3 or Amazon EFS paths visible to your users.</p> <note> <p>If <code>HomeDirectoryType</code> is <code>LOGICAL</code>, you must provide mappings, using the <code>HomeDirectoryMappings</code> parameter. If, on the other hand, <code>HomeDirectoryType</code> is <code>PATH</code>, you provide an absolute path using the <code>HomeDirectory</code> parameter. You cannot have both <code>HomeDirectory</code> and <code>HomeDirectoryMappings</code> in your template.</p> </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `server_id` | String | <p>A system-assigned unique identifier for a server that has this access assigned.</p> |
| `access` | String | <p>The external identifier of the server that the access is attached to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access
access = provider.transfer.Access {
    role = "value"  # <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that controls your users' access to your Amazon S3 bucket or Amazon EFS file system. The policies attached to this role determine the level of access that you want to provide your users when transferring files into and out of your Amazon S3 bucket or Amazon EFS file system. The IAM role should also contain a trust relationship that allows the server to access your resources when servicing your users' transfer requests.</p>
    server_id = "value"  # <p>A system-assigned unique identifier for a server instance. This is the specific server that you added your user to.</p>
    external_id = "value"  # <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p> <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p> <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p> <p>The regular expression used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
}

# Access access outputs
access_id = access.id
access_server_id = access.server_id
access_access = access.access
```

---


### Ssh_public_key

SshPublicKey resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple security_policy resources
security_policy_0 = provider.transfer.Security_policy {
}
security_policy_1 = provider.transfer.Security_policy {
}
security_policy_2 = provider.transfer.Security_policy {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    security_policy = provider.transfer.Security_policy {
    }
```

---

## Related Documentation

- [AWS Transfer Documentation](https://docs.aws.amazon.com/transfer/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
