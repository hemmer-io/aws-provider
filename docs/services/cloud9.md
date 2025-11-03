# Cloud9 Service



**Resources**: 6

---

## Overview

The cloud9 service provides access to 6 resource types:

- [Environments](#environments) [R]
- [Environment_status](#environment_status) [R]
- [Environment](#environment) [UD]
- [Environment_memberships](#environment_memberships) [R]
- [Environment_ec2](#environment_ec2) [C]
- [Environment_membership](#environment_membership) [CUD]

---

## Resources


### Environments

Environments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environments` | Vec<String> | <p>Information about the environments that are returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environments outputs
environments_id = environments.id
environments_environments = environments.environments
```

---


### Environment_status

EnvironmentStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message` | String | <p>Any informational message about the status of the environment.</p> |
| `status` | String | <p>The status of the environment. Available values include:</p>
         <ul>
            <li>
               <p>
                  <code>connecting</code>: The environment is connecting.</p>
            </li>
            <li>
               <p>
                  <code>creating</code>: The environment is being created.</p>
            </li>
            <li>
               <p>
                  <code>deleting</code>: The environment is being deleted.</p>
            </li>
            <li>
               <p>
                  <code>error</code>: The environment is in an error state.</p>
            </li>
            <li>
               <p>
                  <code>ready</code>: The environment is ready.</p>
            </li>
            <li>
               <p>
                  <code>stopped</code>: The environment is stopped.</p>
            </li>
            <li>
               <p>
                  <code>stopping</code>: The environment is stopping.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environment_status outputs
environment_status_id = environment_status.id
environment_status_message = environment_status.message
environment_status_status = environment_status.status
```

---


### Environment

Environment resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>A replacement name for the environment.</p> |
| `managed_credentials_action` | String |  | <p>Allows the environment owner to turn on or turn off the Amazon Web Services managed temporary
      credentials for an Cloud9 environment by using one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>ENABLE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DISABLE</code>
               </p>
            </li>
         </ul>
         <note>
            <p>Only the environment owner can change the status of managed temporary credentials. An <code>AccessDeniedException</code> is thrown if an attempt to turn on or turn off managed temporary credentials is made by an account that's not the environment
      owner.</p>
         </note> |
| `environment_id` | String | ✅ | <p>The ID of the environment to change settings.</p> |
| `description` | String |  | <p>Any new or replacement description for the environment.</p> |



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


### Environment_memberships

EnvironmentMemberships resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `memberships` | Vec<String> | <p>Information about the environment members for the environment.</p> |
| `next_token` | String | <p>If there are more than 25 items in the list, only the first 25 items are returned, along
      with a unique string called a <i>next token</i>. To get the next batch of items
      in the list, call this operation again, adding the next token to the call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environment_memberships outputs
environment_memberships_id = environment_memberships.id
environment_memberships_memberships = environment_memberships.memberships
environment_memberships_next_token = environment_memberships.next_token
```

---


### Environment_ec2

EnvironmentEC2 resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_type` | String | ✅ | <p>The type of instance to connect to the environment (for example,
      <code>t2.micro</code>).</p> |
| `connection_type` | String |  | <p>The connection type used for connecting to an Amazon EC2 environment. Valid values are
        <code>CONNECT_SSH</code> (default) and <code>CONNECT_SSM</code> (connected through
      Amazon EC2 Systems Manager).</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cloud9/latest/user-guide/ec2-ssm.html">Accessing no-ingress EC2 instances with
        Amazon EC2 Systems Manager</a> in the <i>Cloud9 User Guide</i>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `automatic_stop_time_minutes` | i64 |  | <p>The number of minutes until the running instance is shut down after the environment has
      last been used.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive string that helps Cloud9 to ensure this operation completes no
      more than one time.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Client Tokens</a> in the
        <i>Amazon EC2 API Reference</i>.</p> |
| `description` | String |  | <p>The description of the environment to create.</p> |
| `name` | String | ✅ | <p>The name of the environment to create.</p>
         <p>This name is visible to other IAM users in the same Amazon Web Services account.</p> |
| `subnet_id` | String |  | <p>The ID of the subnet in Amazon VPC that Cloud9 will use to communicate with the Amazon EC2
      instance.</p> |
| `owner_arn` | String |  | <p>The Amazon Resource Name (ARN) of the environment owner. This ARN can be the ARN of any
      IAM principal. If this value is not specified, the ARN defaults to this environment's
      creator.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs that will be associated with the new Cloud9 development
      environment.</p> |
| `image_id` | String | ✅ | <p>The identifier for the Amazon Machine Image (AMI) that's used to create the EC2 instance.
      To choose an AMI for the instance, you must specify a valid AMI alias or a valid Amazon EC2 Systems Manager (SSM)
      path.</p>
         <p></p>
         <p>We recommend using Amazon Linux 2023 as the AMI to create your environment as it is fully
      supported.</p>
         <p>From December 16, 2024, Ubuntu 18.04 will be removed from the list of available
        <code>imageIds</code> for Cloud9. This change is necessary as Ubuntu 18.04 has ended standard
      support on May 31, 2023. This change will only affect direct API consumers, and not Cloud9
      console users.</p>
         <p>Since Ubuntu 18.04 has ended standard support as of May 31, 2023, we recommend you choose
      Ubuntu 22.04.</p>
         <p>
            <b>AMI aliases </b>
         </p>
         <ul>
            <li>
               <p>Amazon Linux 2: <code>amazonlinux-2-x86_64</code>
               </p>
            </li>
            <li>
               <p>Amazon Linux 2023 (recommended): <code>amazonlinux-2023-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 18.04: <code>ubuntu-18.04-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 22.04: <code>ubuntu-22.04-x86_64</code>
               </p>
            </li>
         </ul>
         <p>
            <b>SSM paths</b>
         </p>
         <ul>
            <li>
               <p>Amazon Linux 2:
          <code>resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2-x86_64</code>
               </p>
            </li>
            <li>
               <p>Amazon Linux 2023 (recommended):
            <code>resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2023-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 18.04:
          <code>resolve:ssm:/aws/service/cloud9/amis/ubuntu-18.04-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 22.04:
          <code>resolve:ssm:/aws/service/cloud9/amis/ubuntu-22.04-x86_64</code>
               </p>
            </li>
         </ul> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment_ec2
environment_ec2 = provider.cloud9.Environment_ec2 {
    instance_type = "value"  # <p>The type of instance to connect to the environment (for example,
      <code>t2.micro</code>).</p>
    name = "value"  # <p>The name of the environment to create.</p>
         <p>This name is visible to other IAM users in the same Amazon Web Services account.</p>
    image_id = "value"  # <p>The identifier for the Amazon Machine Image (AMI) that's used to create the EC2 instance.
      To choose an AMI for the instance, you must specify a valid AMI alias or a valid Amazon EC2 Systems Manager (SSM)
      path.</p>
         <p></p>
         <p>We recommend using Amazon Linux 2023 as the AMI to create your environment as it is fully
      supported.</p>
         <p>From December 16, 2024, Ubuntu 18.04 will be removed from the list of available
        <code>imageIds</code> for Cloud9. This change is necessary as Ubuntu 18.04 has ended standard
      support on May 31, 2023. This change will only affect direct API consumers, and not Cloud9
      console users.</p>
         <p>Since Ubuntu 18.04 has ended standard support as of May 31, 2023, we recommend you choose
      Ubuntu 22.04.</p>
         <p>
            <b>AMI aliases </b>
         </p>
         <ul>
            <li>
               <p>Amazon Linux 2: <code>amazonlinux-2-x86_64</code>
               </p>
            </li>
            <li>
               <p>Amazon Linux 2023 (recommended): <code>amazonlinux-2023-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 18.04: <code>ubuntu-18.04-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 22.04: <code>ubuntu-22.04-x86_64</code>
               </p>
            </li>
         </ul>
         <p>
            <b>SSM paths</b>
         </p>
         <ul>
            <li>
               <p>Amazon Linux 2:
          <code>resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2-x86_64</code>
               </p>
            </li>
            <li>
               <p>Amazon Linux 2023 (recommended):
            <code>resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2023-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 18.04:
          <code>resolve:ssm:/aws/service/cloud9/amis/ubuntu-18.04-x86_64</code>
               </p>
            </li>
            <li>
               <p>Ubuntu 22.04:
          <code>resolve:ssm:/aws/service/cloud9/amis/ubuntu-22.04-x86_64</code>
               </p>
            </li>
         </ul>
}

```

---


### Environment_membership

EnvironmentMembership resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `environment_id` | String | ✅ | <p>The ID of the environment that contains the environment member you want to add.</p> |
| `user_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the environment member you want to add.</p> |
| `permissions` | String | ✅ | <p>The type of environment member permissions you want to associate with this environment
      member. Available values include:</p>
         <ul>
            <li>
               <p>
                  <code>read-only</code>: Has read-only access to the environment.</p>
            </li>
            <li>
               <p>
                  <code>read-write</code>: Has read-write access to the environment.</p>
            </li>
         </ul> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment_membership
environment_membership = provider.cloud9.Environment_membership {
    environment_id = "value"  # <p>The ID of the environment that contains the environment member you want to add.</p>
    user_arn = "value"  # <p>The Amazon Resource Name (ARN) of the environment member you want to add.</p>
    permissions = "value"  # <p>The type of environment member permissions you want to associate with this environment
      member. Available values include:</p>
         <ul>
            <li>
               <p>
                  <code>read-only</code>: Has read-only access to the environment.</p>
            </li>
            <li>
               <p>
                  <code>read-write</code>: Has read-write access to the environment.</p>
            </li>
         </ul>
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

# Create multiple environments resources
environments_0 = provider.cloud9.Environments {
}
environments_1 = provider.cloud9.Environments {
}
environments_2 = provider.cloud9.Environments {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    environments = provider.cloud9.Environments {
    }
```

---

## Related Documentation

- [AWS Cloud9 Documentation](https://docs.aws.amazon.com/cloud9/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
