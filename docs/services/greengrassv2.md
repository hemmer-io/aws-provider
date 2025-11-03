# Greengrassv2 Service



**Resources**: 7

---

## Overview

The greengrassv2 service provides access to 7 resource types:

- [Deployment](#deployment) [CRD]
- [Core_device](#core_device) [RD]
- [Connectivity_info](#connectivity_info) [RU]
- [Component](#component) [RD]
- [Component_version_artifact](#component_version_artifact) [R]
- [Component_version](#component_version) [C]
- [Service_role_for_account](#service_role_for_account) [R]

---

## Resources


### Deployment

Deployment resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_name` | String |  | <p>The name of the deployment.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you can provide to ensure that the request is idempotent. 
    Idempotency means that the request is successfully processed only once, even if you send the request multiple times. 
    When a request succeeds, and you specify the same client token for subsequent successful requests, the IoT Greengrass V2 service 
    returns the successful response that it caches from the previous request. IoT Greengrass V2 caches successful responses for 
    idempotent requests for up to 8 hours.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the resource. For more
      information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/tag-resources.html">Tag your
        resources</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p> |
| `components` | HashMap<String, String> |  | <p>The components to deploy. This is a dictionary, where each key is the name of a component,
        and each key's value is the version and configuration to deploy for that component.</p> |
| `target_arn` | String | ✅ | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the target IoT thing or thing group. When creating a subdeployment, the targetARN can only be a thing group.</p> |
| `iot_job_configuration` | String |  | <p>The job configuration for the deployment configuration. The job configuration specifies
      the rollout, timeout, and stop configurations for the deployment configuration.</p> |
| `parent_target_arn` | String |  | <p>The parent deployment's target <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> within a subdeployment.</p> |
| `deployment_policies` | String |  | <p>The deployment policies for the deployment. These policies define how the deployment
      updates components and handles failure.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployment_id` | String | <p>The ID of the deployment.</p> |
| `iot_job_id` | String | <p>The ID of the IoT job that applies the deployment to target devices.</p> |
| `parent_target_arn` | String | <p>The parent deployment's target <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> within a subdeployment.</p> |
| `revision_id` | String | <p>The revision number of the deployment.</p> |
| `deployment_policies` | String | <p>The deployment policies for the deployment. These policies define how the deployment
      updates components and handles failure.</p> |
| `iot_job_configuration` | String | <p>The job configuration for the deployment configuration. The job configuration specifies
      the rollout, timeout, and stop configurations for the deployment configuration.</p> |
| `deployment_status` | String | <p>The status of the deployment.</p> |
| `creation_timestamp` | String | <p>The time at which the deployment was created, expressed in ISO 8601 format.</p> |
| `target_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the target IoT thing or thing group.</p> |
| `tags` | HashMap<String, String> | <p>A list of key-value pairs that contain metadata for the resource. For more
      information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/tag-resources.html">Tag your
        resources</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p> |
| `deployment_name` | String | <p>The name of the deployment.</p> |
| `iot_job_arn` | String | <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the IoT job that applies the deployment to target devices.</p> |
| `components` | HashMap<String, String> | <p>The components to deploy. This is a dictionary, where each key is the name of a component,
        and each key's value is the version and configuration to deploy for that component.</p> |
| `is_latest_for_target` | bool | <p>Whether or not the deployment is the latest revision for its target.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment
deployment = provider.greengrassv2.Deployment {
    target_arn = "value"  # <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the target IoT thing or thing group. When creating a subdeployment, the targetARN can only be a thing group.</p>
}

# Access deployment outputs
deployment_id = deployment.id
deployment_deployment_id = deployment.deployment_id
deployment_iot_job_id = deployment.iot_job_id
deployment_parent_target_arn = deployment.parent_target_arn
deployment_revision_id = deployment.revision_id
deployment_deployment_policies = deployment.deployment_policies
deployment_iot_job_configuration = deployment.iot_job_configuration
deployment_deployment_status = deployment.deployment_status
deployment_creation_timestamp = deployment.creation_timestamp
deployment_target_arn = deployment.target_arn
deployment_tags = deployment.tags
deployment_deployment_name = deployment.deployment_name
deployment_iot_job_arn = deployment.iot_job_arn
deployment_components = deployment.components
deployment_is_latest_for_target = deployment.is_latest_for_target
```

---


### Core_device

CoreDevice resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `architecture` | String | <p>The computer architecture of the core device.</p> |
| `status` | String | <p>The status of the core device. The core device status can be:</p>
         <ul>
            <li>
               <p>
                  <code>HEALTHY</code> – The IoT Greengrass Core software and all components run on the core device without issue.</p>
            </li>
            <li>
               <p>
                  <code>UNHEALTHY</code> – The IoT Greengrass Core software or a component is in a failed state
          on the core device.</p>
            </li>
         </ul> |
| `runtime` | String | <p>The runtime for the core device. The runtime can be:</p>
         <ul>
            <li>
               <p>
                  <code>aws_nucleus_classic</code>
               </p>
            </li>
            <li>
               <p>
                  <code>aws_nucleus_lite</code>
               </p>
            </li>
         </ul> |
| `last_status_update_timestamp` | String | <p>The time at which the core device's status last updated, expressed in ISO 8601
      format.</p> |
| `tags` | HashMap<String, String> | <p>A list of key-value pairs that contain metadata for the resource. For more
      information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/tag-resources.html">Tag your
        resources</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p> |
| `platform` | String | <p>The operating system platform that the core device runs.</p> |
| `core_version` | String | <p>The version of the IoT Greengrass Core software that the core device runs. This version is equivalent to
      the version of the Greengrass nucleus component that runs on the core device. For more information,
      see the <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/greengrass-nucleus-component.html">Greengrass nucleus
        component</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p> |
| `core_device_thing_name` | String | <p>The name of the core device. This is also the name of the IoT thing.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access core_device outputs
core_device_id = core_device.id
core_device_architecture = core_device.architecture
core_device_status = core_device.status
core_device_runtime = core_device.runtime
core_device_last_status_update_timestamp = core_device.last_status_update_timestamp
core_device_tags = core_device.tags
core_device_platform = core_device.platform
core_device_core_version = core_device.core_version
core_device_core_device_thing_name = core_device.core_device_thing_name
```

---


### Connectivity_info

ConnectivityInfo resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thing_name` | String | ✅ | <p>The name of the core device. This is also the name of the IoT thing.</p> |
| `connectivity_info` | Vec<String> | ✅ | <p>The connectivity information for the core device.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message` | String | <p>A message about the connectivity information request.</p> |
| `connectivity_info` | Vec<String> | <p>The connectivity information for the core device.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connectivity_info outputs
connectivity_info_id = connectivity_info.id
connectivity_info_message = connectivity_info.message
connectivity_info_connectivity_info = connectivity_info.connectivity_info
```

---


### Component

Component resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recipe_output_format` | String | <p>The format of the recipe.</p> |
| `recipe` | String | <p>The recipe of the component version.</p> |
| `tags` | HashMap<String, String> | <p>A list of key-value pairs that contain metadata for the resource. For more
      information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/tag-resources.html">Tag your
        resources</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access component outputs
component_id = component.id
component_recipe_output_format = component.recipe_output_format
component_recipe = component.recipe
component_tags = component.tags
```

---


### Component_version_artifact

ComponentVersionArtifact resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pre_signed_url` | String | <p>The URL of the artifact.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access component_version_artifact outputs
component_version_artifact_id = component_version_artifact.id
component_version_artifact_pre_signed_url = component_version_artifact.pre_signed_url
```

---


### Component_version

ComponentVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inline_recipe` | String |  | <p>The recipe to use to create the component. The recipe defines the component's metadata,
      parameters, dependencies, lifecycle, artifacts, and platform compatibility.</p>
         <p>You must specify either <code>inlineRecipe</code> or <code>lambdaFunction</code>.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you can provide to ensure that the request is idempotent. 
    Idempotency means that the request is successfully processed only once, even if you send the request multiple times. 
    When a request succeeds, and you specify the same client token for subsequent successful requests, the IoT Greengrass V2 service 
    returns the successful response that it caches from the previous request. IoT Greengrass V2 caches successful responses for 
    idempotent requests for up to 8 hours.</p> |
| `lambda_function` | String |  | <p>The parameters to create a component from a Lambda function.</p>
         <p>You must specify either <code>inlineRecipe</code> or <code>lambdaFunction</code>.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs that contain metadata for the resource. For more
      information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/tag-resources.html">Tag your
        resources</a> in the <i>IoT Greengrass V2 Developer Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create component_version
component_version = provider.greengrassv2.Component_version {
}

```

---


### Service_role_for_account

ServiceRoleForAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associated_at` | String | <p>The time when the service role was associated with IoT Greengrass for your Amazon Web Services account in this
      Amazon Web Services Region.</p> |
| `role_arn` | String | <p>The ARN of the service role that is associated with IoT Greengrass for your Amazon Web Services account in this
      Amazon Web Services Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_role_for_account outputs
service_role_for_account_id = service_role_for_account.id
service_role_for_account_associated_at = service_role_for_account.associated_at
service_role_for_account_role_arn = service_role_for_account.role_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple deployment resources
deployment_0 = provider.greengrassv2.Deployment {
    target_arn = "value-0"
}
deployment_1 = provider.greengrassv2.Deployment {
    target_arn = "value-1"
}
deployment_2 = provider.greengrassv2.Deployment {
    target_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    deployment = provider.greengrassv2.Deployment {
        target_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Greengrassv2 Documentation](https://docs.aws.amazon.com/greengrassv2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
