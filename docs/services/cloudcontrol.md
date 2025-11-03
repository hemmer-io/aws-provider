# Cloudcontrol Service



**Resources**: 2

---

## Overview

The cloudcontrol service provides access to 2 resource types:

- [Resource_request_status](#resource_request_status) [R]
- [Resource](#resource) [CRUD]

---

## Resources


### Resource_request_status

ResourceRequestStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hooks_progress_event` | Vec<String> | <p>Lists Hook invocations for the specified target in the request. This is a list since the same target can invoke multiple Hooks.</p> |
| `progress_event` | String | <p>Represents the current status of the resource operation request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_request_status outputs
resource_request_status_id = resource_request_status.id
resource_request_status_hooks_progress_event = resource_request_status.hooks_progress_event
resource_request_status_progress_event = resource_request_status.progress_event
```

---


### Resource

Resource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique identifier to ensure the idempotency of the resource request. As a
                                     best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can
                                     accurately distinguish between request retries and new resource requests. You
                                     might retry a resource request to ensure that it was successfully received.</p>
         <p>A client token is valid for 36 hours once used. After that, a resource
                                     request with the same client token is treated as a new request.</p>
         <p>If you
                                     do not specify a client token, one is generated for inclusion in the request.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring
                                     resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User
                                     Guide</i>.</p> |
| `desired_state` | String | ✅ | <p>Structured data format representing the desired state of the resource, consisting of that
      resource's properties and their desired values.</p>
         <note>
            <p>Cloud Control API currently supports JSON as a structured data format.</p>
         </note>
         <p>Specify the desired state as one of the following:</p>
         <ul>
            <li>
               <p>A JSON blob</p>
            </li>
            <li>
               <p>A local path containing the desired state in JSON data format</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-create.html#resource-operations-create-desiredstate">Composing the desired state of the resource</a> in the <i>Amazon Web Services Cloud Control API User
        Guide</i>.</p>
         <p>For more information about the properties of a specific resource, refer to the related
      topic for the resource in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Resource and property types reference</a> in the <i>CloudFormation Users Guide</i>.</p> |
| `type_name` | String | ✅ | <p>The name of the resource type.</p> |
| `type_version_id` | String |  | <p>For private resource types, the type version to use in this resource
                                     operation. If you do not specify a resource version, CloudFormation
                                     uses the default version.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Identity and Access Management
                                    (IAM) role for Cloud Control API to use when performing this resource
                                    operation. The role specified must have the permissions required for this
                                    operation. The necessary permissions for each event handler are defined in the
                                    <code>
               <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a>
            </code>
                                    section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type
                                    definition schema</a>.</p>
         <p>If you do not specify a role, Cloud Control API uses a temporary session created using
                                    your Amazon Web Services user credentials.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying
                                    credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type_name` | String | <p>The name of the resource type.</p> |
| `resource_description` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource
resource = provider.cloudcontrol.Resource {
    desired_state = "value"  # <p>Structured data format representing the desired state of the resource, consisting of that
      resource's properties and their desired values.</p>
         <note>
            <p>Cloud Control API currently supports JSON as a structured data format.</p>
         </note>
         <p>Specify the desired state as one of the following:</p>
         <ul>
            <li>
               <p>A JSON blob</p>
            </li>
            <li>
               <p>A local path containing the desired state in JSON data format</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-create.html#resource-operations-create-desiredstate">Composing the desired state of the resource</a> in the <i>Amazon Web Services Cloud Control API User
        Guide</i>.</p>
         <p>For more information about the properties of a specific resource, refer to the related
      topic for the resource in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Resource and property types reference</a> in the <i>CloudFormation Users Guide</i>.</p>
    type_name = "value"  # <p>The name of the resource type.</p>
}

# Access resource outputs
resource_id = resource.id
resource_type_name = resource.type_name
resource_resource_description = resource.resource_description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_request_status resources
resource_request_status_0 = provider.cloudcontrol.Resource_request_status {
}
resource_request_status_1 = provider.cloudcontrol.Resource_request_status {
}
resource_request_status_2 = provider.cloudcontrol.Resource_request_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_request_status = provider.cloudcontrol.Resource_request_status {
    }
```

---

## Related Documentation

- [AWS Cloudcontrol Documentation](https://docs.aws.amazon.com/cloudcontrol/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
