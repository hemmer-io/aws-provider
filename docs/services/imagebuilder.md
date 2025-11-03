# Imagebuilder Service



**Resources**: 17

---

## Overview

The imagebuilder service provides access to 17 resource types:

- [Image_recipe_policy](#image_recipe_policy) [CR]
- [Container_recipe_policy](#container_recipe_policy) [CR]
- [Distribution_configuration](#distribution_configuration) [CRUD]
- [Workflow_step_execution](#workflow_step_execution) [R]
- [Marketplace_resource](#marketplace_resource) [R]
- [Image_recipe](#image_recipe) [CRD]
- [Image_policy](#image_policy) [CR]
- [Image_pipeline](#image_pipeline) [CRUD]
- [Workflow](#workflow) [CRD]
- [Infrastructure_configuration](#infrastructure_configuration) [CRUD]
- [Lifecycle_policy](#lifecycle_policy) [CRUD]
- [Component_policy](#component_policy) [CR]
- [Workflow_execution](#workflow_execution) [R]
- [Image](#image) [CRD]
- [Component](#component) [CRD]
- [Container_recipe](#container_recipe) [CRD]
- [Lifecycle_execution](#lifecycle_execution) [R]

---

## Resources


### Image_recipe_policy

ImageRecipePolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The policy to apply.</p> |
| `image_recipe_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the image recipe that this policy should be applied
			to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The image recipe policy object.</p> |
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_recipe_policy
image_recipe_policy = provider.imagebuilder.Image_recipe_policy {
    policy = "value"  # <p>The policy to apply.</p>
    image_recipe_arn = "value"  # <p>The Amazon Resource Name (ARN) of the image recipe that this policy should be applied
			to.</p>
}

# Access image_recipe_policy outputs
image_recipe_policy_id = image_recipe_policy.id
image_recipe_policy_policy = image_recipe_policy.policy
image_recipe_policy_request_id = image_recipe_policy.request_id
```

---


### Container_recipe_policy

ContainerRecipePolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container_recipe_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the container recipe that this policy should be
			applied to.</p> |
| `policy` | String | ✅ | <p>The policy to apply to the container recipe.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |
| `policy` | String | <p>The container recipe policy object that is returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create container_recipe_policy
container_recipe_policy = provider.imagebuilder.Container_recipe_policy {
    container_recipe_arn = "value"  # <p>The Amazon Resource Name (ARN) of the container recipe that this policy should be
			applied to.</p>
    policy = "value"  # <p>The policy to apply to the container recipe.</p>
}

# Access container_recipe_policy outputs
container_recipe_policy_id = container_recipe_policy.id
container_recipe_policy_request_id = container_recipe_policy.request_id
container_recipe_policy_policy = container_recipe_policy.policy
```

---


### Distribution_configuration

DistributionConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `distributions` | Vec<String> | ✅ | <p>The distributions of the distribution configuration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags of the distribution configuration.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `name` | String | ✅ | <p>The name of the distribution configuration.</p> |
| `description` | String |  | <p>The description of the distribution configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |
| `distribution_configuration` | String | <p>The distribution configuration object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create distribution_configuration
distribution_configuration = provider.imagebuilder.Distribution_configuration {
    distributions = "value"  # <p>The distributions of the distribution configuration.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
    name = "value"  # <p>The name of the distribution configuration.</p>
}

# Access distribution_configuration outputs
distribution_configuration_id = distribution_configuration.id
distribution_configuration_request_id = distribution_configuration.request_id
distribution_configuration_distribution_configuration = distribution_configuration.distribution_configuration
```

---


### Workflow_step_execution

WorkflowStepExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | <p>The timestamp when the specified runtime version of the workflow step started.</p> |
| `on_failure` | String | <p>The action to perform if the workflow step fails.</p> |
| `message` | String | <p>The output message from the specified runtime instance of the workflow step, if applicable.</p> |
| `status` | String | <p>The current status for the specified runtime version of the workflow step.</p> |
| `timeout_seconds` | i64 | <p>The maximum duration in seconds for this step to complete its action.</p> |
| `inputs` | String | <p>Input parameters that Image Builder provided for the specified runtime instance of 
			the workflow step.</p> |
| `action` | String | <p>The name of the action that the specified step performs.</p> |
| `workflow_execution_id` | String | <p>The unique identifier that Image Builder assigned to keep track of runtime details
			when it ran the workflow.</p> |
| `image_build_version_arn` | String | <p>The Amazon Resource Name (ARN) of the image resource build version that the specified 
			runtime instance of the workflow step creates.</p> |
| `workflow_build_version_arn` | String | <p>The Amazon Resource Name (ARN) of the build version for the Image Builder workflow resource
			that defines this workflow step.</p> |
| `rollback_status` | String | <p>Reports on the rollback status of the specified runtime version of the workflow step, 
			if applicable.</p> |
| `outputs` | String | <p>The file names that the specified runtime version of the workflow step created as output.</p> |
| `name` | String | <p>The name of the specified runtime instance of the workflow step.</p> |
| `step_execution_id` | String | <p>The unique identifier for the runtime version of the workflow step that you specified 
			in the request.</p> |
| `description` | String | <p>Describes the specified workflow step.</p> |
| `end_time` | String | <p>The timestamp when the specified runtime instance of the workflow step finished.</p> |
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_step_execution outputs
workflow_step_execution_id = workflow_step_execution.id
workflow_step_execution_start_time = workflow_step_execution.start_time
workflow_step_execution_on_failure = workflow_step_execution.on_failure
workflow_step_execution_message = workflow_step_execution.message
workflow_step_execution_status = workflow_step_execution.status
workflow_step_execution_timeout_seconds = workflow_step_execution.timeout_seconds
workflow_step_execution_inputs = workflow_step_execution.inputs
workflow_step_execution_action = workflow_step_execution.action
workflow_step_execution_workflow_execution_id = workflow_step_execution.workflow_execution_id
workflow_step_execution_image_build_version_arn = workflow_step_execution.image_build_version_arn
workflow_step_execution_workflow_build_version_arn = workflow_step_execution.workflow_build_version_arn
workflow_step_execution_rollback_status = workflow_step_execution.rollback_status
workflow_step_execution_outputs = workflow_step_execution.outputs
workflow_step_execution_name = workflow_step_execution.name
workflow_step_execution_step_execution_id = workflow_step_execution.step_execution_id
workflow_step_execution_description = workflow_step_execution.description
workflow_step_execution_end_time = workflow_step_execution.end_time
workflow_step_execution_request_id = workflow_step_execution.request_id
```

---


### Marketplace_resource

MarketplaceResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) for the Amazon Web Services Marketplace resource that was requested.</p> |
| `url` | String | <p>The obfuscated S3 URL to download the component artifact from.</p> |
| `data` | String | <p>Returns obfuscated data that contains the YAML content of the component.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access marketplace_resource outputs
marketplace_resource_id = marketplace_resource.id
marketplace_resource_resource_arn = marketplace_resource.resource_arn
marketplace_resource_url = marketplace_resource.url
marketplace_resource_data = marketplace_resource.data
```

---


### Image_recipe

ImageRecipe resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `components` | Vec<String> | ✅ | <p>The components included in the image recipe.</p> |
| `working_directory` | String |  | <p>The working directory used during build and test workflows.</p> |
| `additional_instance_configuration` | String |  | <p>Specify additional settings and launch scripts for your build instances.</p> |
| `semantic_version` | String | ✅ | <p>The semantic version of the image recipe. This version follows the semantic version
			syntax.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note> |
| `block_device_mappings` | Vec<String> |  | <p>The block device mappings of the image recipe.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags of the image recipe.</p> |
| `ami_tags` | HashMap<String, String> |  | <p>Tags that are applied to the AMI that Image Builder creates during the Build phase 
			prior to image distribution.</p> |
| `name` | String | ✅ | <p>The name of the image recipe.</p> |
| `description` | String |  | <p>The description of the image recipe.</p> |
| `parent_image` | String | ✅ | <p>The base image for customizations specified in the image recipe. You can specify the 
			parent image using one of the following options:</p>
         <ul>
            <li>
               <p>AMI ID</p>
            </li>
            <li>
               <p>Image Builder image Amazon Resource Name (ARN)</p>
            </li>
            <li>
               <p>Amazon Web Services Systems Manager (SSM) Parameter Store Parameter, prefixed by <code>ssm:</code>, 
					followed by the parameter name or ARN.</p>
            </li>
            <li>
               <p>Amazon Web Services Marketplace product ID</p>
            </li>
         </ul>
         <p>If you enter an AMI ID or an SSM parameter that contains the AMI ID, you must have access 
			to the AMI, and the AMI must be in the source Region.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |
| `image_recipe` | String | <p>The image recipe object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_recipe
image_recipe = provider.imagebuilder.Image_recipe {
    components = "value"  # <p>The components included in the image recipe.</p>
    semantic_version = "value"  # <p>The semantic version of the image recipe. This version follows the semantic version
			syntax.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note>
    name = "value"  # <p>The name of the image recipe.</p>
    parent_image = "value"  # <p>The base image for customizations specified in the image recipe. You can specify the 
			parent image using one of the following options:</p>
         <ul>
            <li>
               <p>AMI ID</p>
            </li>
            <li>
               <p>Image Builder image Amazon Resource Name (ARN)</p>
            </li>
            <li>
               <p>Amazon Web Services Systems Manager (SSM) Parameter Store Parameter, prefixed by <code>ssm:</code>, 
					followed by the parameter name or ARN.</p>
            </li>
            <li>
               <p>Amazon Web Services Marketplace product ID</p>
            </li>
         </ul>
         <p>If you enter an AMI ID or an SSM parameter that contains the AMI ID, you must have access 
			to the AMI, and the AMI must be in the source Region.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
}

# Access image_recipe outputs
image_recipe_id = image_recipe.id
image_recipe_request_id = image_recipe.request_id
image_recipe_image_recipe = image_recipe.image_recipe
```

---


### Image_policy

ImagePolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the image that this policy should be applied
			to.</p> |
| `policy` | String | ✅ | <p>The policy to apply.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The image policy object.</p> |
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_policy
image_policy = provider.imagebuilder.Image_policy {
    image_arn = "value"  # <p>The Amazon Resource Name (ARN) of the image that this policy should be applied
			to.</p>
    policy = "value"  # <p>The policy to apply.</p>
}

# Access image_policy outputs
image_policy_id = image_policy.id
image_policy_policy = image_policy.policy
image_policy_request_id = image_policy.request_id
```

---


### Image_pipeline

ImagePipeline resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `distribution_configuration_arn` | String |  | <p>The Amazon Resource Name (ARN) of the distribution configuration that will be used to
			configure and distribute images created by this image pipeline.</p> |
| `workflows` | Vec<String> |  | <p>Contains an array of workflow configuration objects.</p> |
| `container_recipe_arn` | String |  | <p>The Amazon Resource Name (ARN) of the container recipe that is used to configure
			images created by this container pipeline.</p> |
| `schedule` | String |  | <p>The schedule of the image pipeline.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags of the image pipeline.</p> |
| `logging_configuration` | String |  | <p>Define logging configuration for the image build process.</p> |
| `infrastructure_configuration_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the infrastructure configuration that will be used
			to build images created by this image pipeline.</p> |
| `image_tests_configuration` | String |  | <p>The image test configuration of the image pipeline.</p> |
| `execution_role` | String |  | <p>The name or Amazon Resource Name (ARN) for the IAM role you create that grants 
			Image Builder access to perform workflow actions.</p> |
| `status` | String |  | <p>The status of the image pipeline.</p> |
| `enhanced_image_metadata_enabled` | bool |  | <p>Collects additional information about the image being created, including the operating
			system (OS) version and package list. This information is used to enhance the overall
			experience of using EC2 Image Builder. Enabled by default.</p> |
| `image_scanning_configuration` | String |  | <p>Contains settings for vulnerability scans.</p> |
| `description` | String |  | <p>The description of the image pipeline.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `image_recipe_arn` | String |  | <p>The Amazon Resource Name (ARN) of the image recipe that will be used to configure
			images created by this image pipeline.</p> |
| `name` | String | ✅ | <p>The name of the image pipeline.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_pipeline` | String | <p>The image pipeline object.</p> |
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_pipeline
image_pipeline = provider.imagebuilder.Image_pipeline {
    infrastructure_configuration_arn = "value"  # <p>The Amazon Resource Name (ARN) of the infrastructure configuration that will be used
			to build images created by this image pipeline.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
    name = "value"  # <p>The name of the image pipeline.</p>
}

# Access image_pipeline outputs
image_pipeline_id = image_pipeline.id
image_pipeline_image_pipeline = image_pipeline.image_pipeline
image_pipeline_request_id = image_pipeline.request_id
```

---


### Workflow

Workflow resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `data` | String |  | <p>Contains the UTF-8 encoded YAML document content for the workflow. 
			Alternatively, you can specify the <code>uri</code> of a YAML document file stored in
			Amazon S3. However, you cannot specify both properties.</p> |
| `semantic_version` | String | ✅ | <p>The semantic version of this workflow resource. The semantic version syntax 
			adheres to the following rules.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note> |
| `kms_key_id` | String |  | <p>The Amazon Resource Name (ARN) that uniquely identifies the KMS key used to encrypt this workflow resource. 
			This can be either the Key ARN or the Alias ARN. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">Key identifiers (KeyId)</a> 
			in the <i>Key Management Service Developer Guide</i>.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags that apply to the workflow resource.</p> |
| `change_description` | String |  | <p>Describes what change has been made in this version of the workflow, or 
			what makes this version different from other versions of the workflow.</p> |
| `description` | String |  | <p>Describes the workflow.</p> |
| `name` | String | ✅ | <p>The name of the workflow to create.</p> |
| `uri` | String |  | <p>The <code>uri</code> of a YAML component document file. This must be an S3 URL
			(<code>s3://bucket/key</code>), and the requester must have permission to access the
			S3 bucket it points to. If you use Amazon S3, you can specify component content up to your
			service quota.</p>
         <p>Alternatively, you can specify the YAML document inline, using the component
			<code>data</code> property. You cannot specify both properties.</p> |
| `type` | String | ✅ | <p>The phase in the image build process for which the workflow resource 
			is responsible.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workflow` | String | <p>The workflow resource specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workflow
workflow = provider.imagebuilder.Workflow {
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
    semantic_version = "value"  # <p>The semantic version of this workflow resource. The semantic version syntax 
			adheres to the following rules.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note>
    name = "value"  # <p>The name of the workflow to create.</p>
    type = "value"  # <p>The phase in the image build process for which the workflow resource 
			is responsible.</p>
}

# Access workflow outputs
workflow_id = workflow.id
workflow_workflow = workflow.workflow
```

---


### Infrastructure_configuration

InfrastructureConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sns_topic_arn` | String |  | <p>The Amazon Resource Name (ARN) for the SNS topic to which we send image build event
			notifications.</p>
         <note>
            <p>EC2 Image Builder is unable to send notifications to SNS topics that are encrypted using keys 
				from other accounts. The key that is used to encrypt the SNS topic must reside in the 
				account that the Image Builder service runs under.</p>
         </note> |
| `tags` | HashMap<String, String> |  | <p>The metadata tags to assign to the infrastructure configuration resource that Image Builder 
			creates as output. Tags are formatted as key value pairs.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `logging` | String |  | <p>The logging configuration of the infrastructure configuration.</p> |
| `key_pair` | String |  | <p>The key pair of the infrastructure configuration. You can use this to log on to and
			debug the instance used to create your image.</p> |
| `name` | String | ✅ | <p>The name of the infrastructure configuration.</p> |
| `terminate_instance_on_failure` | bool |  | <p>The terminate instance on failure setting of the infrastructure configuration. Set to
			false if you want Image Builder to retain the instance used to configure your AMI if the build or
			test phase of your workflow fails.</p> |
| `instance_types` | Vec<String> |  | <p>The instance types of the infrastructure configuration. You can specify one or more
			instance types to use for this build. The service will pick one of these instance types
			based on availability.</p> |
| `resource_tags` | HashMap<String, String> |  | <p>The metadata tags to assign to the Amazon EC2 instance that Image Builder launches during the build process. 
			Tags are formatted as key value pairs.</p> |
| `security_group_ids` | Vec<String> |  | <p>The security group IDs to associate with the instance used to customize your Amazon EC2
			AMI.</p> |
| `placement` | String |  | <p>The instance placement settings that define where the instances that are launched 
			from your image will run.</p> |
| `description` | String |  | <p>The description of the infrastructure configuration.</p> |
| `instance_profile_name` | String | ✅ | <p>The instance profile to associate with the instance used to customize your Amazon EC2
			AMI.</p> |
| `instance_metadata_options` | String |  | <p>The instance metadata options that you can set for the HTTP requests that pipeline
			builds use to launch EC2 build and test instances.</p> |
| `subnet_id` | String |  | <p>The subnet ID in which to place the instance used to customize your Amazon EC2 AMI.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |
| `infrastructure_configuration` | String | <p>The infrastructure configuration object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create infrastructure_configuration
infrastructure_configuration = provider.imagebuilder.Infrastructure_configuration {
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
    name = "value"  # <p>The name of the infrastructure configuration.</p>
    instance_profile_name = "value"  # <p>The instance profile to associate with the instance used to customize your Amazon EC2
			AMI.</p>
}

# Access infrastructure_configuration outputs
infrastructure_configuration_id = infrastructure_configuration.id
infrastructure_configuration_request_id = infrastructure_configuration.request_id
infrastructure_configuration_infrastructure_configuration = infrastructure_configuration.infrastructure_configuration
```

---


### Lifecycle_policy

LifecyclePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>Optional description for the lifecycle policy.</p> |
| `policy_details` | Vec<String> | ✅ | <p>Configuration details for the lifecycle policy rules.</p> |
| `status` | String |  | <p>Indicates whether the lifecycle policy resource is enabled.</p> |
| `resource_type` | String | ✅ | <p>The type of Image Builder resource that the lifecycle policy applies to.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `execution_role` | String | ✅ | <p>The name or Amazon Resource Name (ARN) for the IAM role you create that grants 
			Image Builder access to run lifecycle actions.</p> |
| `name` | String | ✅ | <p>The name of the  lifecycle policy to create.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to apply to the lifecycle policy resource.</p> |
| `resource_selection` | String | ✅ | <p>Selection criteria for the resources that the lifecycle policy applies to. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_policy` | String | <p>The ARN of the image lifecycle policy resource that was returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_policy
lifecycle_policy = provider.imagebuilder.Lifecycle_policy {
    policy_details = "value"  # <p>Configuration details for the lifecycle policy rules.</p>
    resource_type = "value"  # <p>The type of Image Builder resource that the lifecycle policy applies to.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
    execution_role = "value"  # <p>The name or Amazon Resource Name (ARN) for the IAM role you create that grants 
			Image Builder access to run lifecycle actions.</p>
    name = "value"  # <p>The name of the  lifecycle policy to create.</p>
    resource_selection = "value"  # <p>Selection criteria for the resources that the lifecycle policy applies to. </p>
}

# Access lifecycle_policy outputs
lifecycle_policy_id = lifecycle_policy.id
lifecycle_policy_lifecycle_policy = lifecycle_policy.lifecycle_policy
```

---


### Component_policy

ComponentPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The policy to apply.</p> |
| `component_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the component that this policy should be applied
			to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The component policy.</p> |
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create component_policy
component_policy = provider.imagebuilder.Component_policy {
    policy = "value"  # <p>The policy to apply.</p>
    component_arn = "value"  # <p>The Amazon Resource Name (ARN) of the component that this policy should be applied
			to.</p>
}

# Access component_policy outputs
component_policy_id = component_policy.id
component_policy_policy = component_policy.policy
component_policy_request_id = component_policy.request_id
```

---


### Workflow_execution

WorkflowExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | <p>The timestamp when the specified runtime instance of the workflow started.</p> |
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |
| `workflow_build_version_arn` | String | <p>The Amazon Resource Name (ARN) of the build version for the Image Builder workflow resource 
			that defines the specified runtime instance of the workflow.</p> |
| `workflow_execution_id` | String | <p>The unique identifier that Image Builder assigned to keep track of runtime details
			when it ran the workflow.</p> |
| `image_build_version_arn` | String | <p>The Amazon Resource Name (ARN) of the image resource build version that the specified 
			runtime instance of the workflow created.</p> |
| `message` | String | <p>The output message from the specified runtime instance of the workflow, if applicable.</p> |
| `status` | String | <p>The current runtime status for the specified runtime instance of the workflow.</p> |
| `type` | String | <p>The type of workflow that Image Builder ran for the specified runtime instance of the workflow.</p> |
| `total_step_count` | i64 | <p>The total number of steps in the specified runtime instance of the workflow that ran. 
			This number should equal the sum of the step counts for steps that succeeded, were skipped, 
			and failed.</p> |
| `total_steps_succeeded` | i64 | <p>A runtime count for the number of steps that ran successfully in the specified runtime 
			instance of the workflow.</p> |
| `total_steps_skipped` | i64 | <p>A runtime count for the number of steps that were skipped in the specified runtime 
			instance of the workflow.</p> |
| `end_time` | String | <p>The timestamp when the specified runtime instance of the workflow finished.</p> |
| `total_steps_failed` | i64 | <p>A runtime count for the number of steps that failed in the specified runtime instance 
			of the workflow.</p> |
| `parallel_group` | String | <p>Test workflows are defined within named runtime groups. The parallel group 
			is a named group that contains one or more test workflows.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_execution outputs
workflow_execution_id = workflow_execution.id
workflow_execution_start_time = workflow_execution.start_time
workflow_execution_request_id = workflow_execution.request_id
workflow_execution_workflow_build_version_arn = workflow_execution.workflow_build_version_arn
workflow_execution_workflow_execution_id = workflow_execution.workflow_execution_id
workflow_execution_image_build_version_arn = workflow_execution.image_build_version_arn
workflow_execution_message = workflow_execution.message
workflow_execution_status = workflow_execution.status
workflow_execution_type = workflow_execution.type
workflow_execution_total_step_count = workflow_execution.total_step_count
workflow_execution_total_steps_succeeded = workflow_execution.total_steps_succeeded
workflow_execution_total_steps_skipped = workflow_execution.total_steps_skipped
workflow_execution_end_time = workflow_execution.end_time
workflow_execution_total_steps_failed = workflow_execution.total_steps_failed
workflow_execution_parallel_group = workflow_execution.parallel_group
```

---


### Image

Image resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `infrastructure_configuration_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the infrastructure configuration that defines the
			environment in which your image will be built and tested.</p> |
| `container_recipe_arn` | String |  | <p>The Amazon Resource Name (ARN) of the container recipe that defines how images are
			configured and tested.</p> |
| `image_scanning_configuration` | String |  | <p>Contains settings for vulnerability scans.</p> |
| `image_tests_configuration` | String |  | <p>The image tests configuration of the image.</p> |
| `enhanced_image_metadata_enabled` | bool |  | <p>Collects additional information about the image being created, including the operating
			system (OS) version and package list. This information is used to enhance the overall
			experience of using EC2 Image Builder. Enabled by default.</p> |
| `logging_configuration` | String |  | <p>Define logging configuration for the image build process.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags of the image.</p> |
| `execution_role` | String |  | <p>The name or Amazon Resource Name (ARN) for the IAM role you create that grants 
			Image Builder access to perform workflow actions.</p> |
| `image_recipe_arn` | String |  | <p>The Amazon Resource Name (ARN) of the image recipe that defines how images are
			configured, tested, and assessed.</p> |
| `workflows` | Vec<String> |  | <p>Contains an array of workflow configuration objects.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `distribution_configuration_arn` | String |  | <p>The Amazon Resource Name (ARN) of the distribution configuration that defines and
			configures the outputs of your pipeline.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |
| `image` | String | <p>The image object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image
image = provider.imagebuilder.Image {
    infrastructure_configuration_arn = "value"  # <p>The Amazon Resource Name (ARN) of the infrastructure configuration that defines the
			environment in which your image will be built and tested.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
}

# Access image outputs
image_id = image.id
image_request_id = image.request_id
image_image = image.image
```

---


### Component

Component resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `change_description` | String |  | <p>The change description of the component. Describes what change has been made in this
			version, or what makes this version different from other versions of the
			component.</p> |
| `name` | String | ✅ | <p>The name of the component.</p> |
| `supported_os_versions` | Vec<String> |  | <p>The operating system (OS) version supported by the component. If the OS information is
			available, a prefix match is performed against the base image OS version during image
			recipe creation.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `uri` | String |  | <p>The <code>uri</code> of a YAML component document file. This must be an S3 URL
				(<code>s3://bucket/key</code>), and the requester must have permission to access the
			S3 bucket it points to. If you use Amazon S3, you can specify component content up to your
			service quota.</p>
         <p>Alternatively, you can specify the YAML document inline, using the component
				<code>data</code> property. You cannot specify both properties.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags that apply to the component.</p> |
| `semantic_version` | String | ✅ | <p>The semantic version of the component. This version follows the semantic version
			syntax.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note> |
| `kms_key_id` | String |  | <p>The Amazon Resource Name (ARN) that uniquely identifies the KMS key used to encrypt this component. This can be either the Key ARN or the Alias ARN. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">Key identifiers (KeyId)</a> 
			in the <i>Key Management Service Developer Guide</i>.</p> |
| `data` | String |  | <p>Component <code>data</code> contains inline YAML document content for the component.
			Alternatively, you can specify the <code>uri</code> of a YAML document file stored in
			Amazon S3. However, you cannot specify both properties.</p> |
| `platform` | String | ✅ | <p>The operating system platform of the component.</p> |
| `description` | String |  | <p>Describes the contents of the component.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `component` | String | <p>The component object specified in the request.</p> |
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create component
component = provider.imagebuilder.Component {
    name = "value"  # <p>The name of the component.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
    semantic_version = "value"  # <p>The semantic version of the component. This version follows the semantic version
			syntax.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note>
    platform = "value"  # <p>The operating system platform of the component.</p>
}

# Access component outputs
component_id = component.id
component_component = component.component
component_request_id = component.request_id
```

---


### Container_recipe

ContainerRecipe resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the container recipe.</p> |
| `image_os_version_override` | String |  | <p>Specifies the operating system version for the base image.</p> |
| `working_directory` | String |  | <p>The working directory for use during build and test workflows.</p> |
| `parent_image` | String | ✅ | <p>The base image for the container recipe.</p> |
| `kms_key_id` | String |  | <p>The Amazon Resource Name (ARN) that uniquely identifies which KMS key is used to encrypt the Dockerfile 
			template. This can be either the Key ARN or the Alias ARN. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">Key identifiers (KeyId)</a> 
			in the <i>Key Management Service Developer Guide</i>.</p> |
| `dockerfile_template_uri` | String |  | <p>The Amazon S3 URI for the Dockerfile that will be used to build your container
			image.</p> |
| `components` | Vec<String> | ✅ | <p>Components for build and test that are included in the container recipe.
			Recipes require a minimum of one build component, and can 
			have a maximum of 20 build and test components in any combination.</p> |
| `container_type` | String | ✅ | <p>The type of container to create.</p> |
| `semantic_version` | String | ✅ | <p>The semantic version of the container recipe. This version follows the semantic
			version syntax.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note> |
| `target_repository` | String | ✅ | <p>The destination repository for the container image.</p> |
| `name` | String | ✅ | <p>The name of the container recipe.</p> |
| `dockerfile_template_data` | String |  | <p>The Dockerfile template used to build your image as an inline data blob.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags that are attached to the container recipe.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p> |
| `instance_configuration` | String |  | <p>A group of options that can be used to configure an instance for building and testing
			container images.</p> |
| `platform_override` | String |  | <p>Specifies the operating system platform when you use a custom base image.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | <p>The request ID that uniquely identifies this request.</p> |
| `container_recipe` | String | <p>The container recipe object that is returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create container_recipe
container_recipe = provider.imagebuilder.Container_recipe {
    parent_image = "value"  # <p>The base image for the container recipe.</p>
    components = "value"  # <p>Components for build and test that are included in the container recipe.
			Recipes require a minimum of one build component, and can 
			have a maximum of 20 build and test components in any combination.</p>
    container_type = "value"  # <p>The type of container to create.</p>
    semantic_version = "value"  # <p>The semantic version of the container recipe. This version follows the semantic
			version syntax.</p>
         <note>
            <p>The semantic version has four nodes: <major>.<minor>.<patch>/<build>. 
	You can assign values for the first three, and can filter on all of them.</p>
            <p>
               <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including 
	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 
	build number to the fourth node.</p>
            <p>
               <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for 
	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 
	a date, such as 2021.01.01.</p>
         </note>
    target_repository = "value"  # <p>The destination repository for the container image.</p>
    name = "value"  # <p>The name of the container recipe.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure
       idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> 
       in the <i>Amazon EC2 API Reference</i>.</p>
}

# Access container_recipe outputs
container_recipe_id = container_recipe.id
container_recipe_request_id = container_recipe.request_id
container_recipe_container_recipe = container_recipe.container_recipe
```

---


### Lifecycle_execution

LifecycleExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_execution` | String | <p>Runtime details for the specified runtime instance of the lifecycle policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lifecycle_execution outputs
lifecycle_execution_id = lifecycle_execution.id
lifecycle_execution_lifecycle_execution = lifecycle_execution.lifecycle_execution
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple image_recipe_policy resources
image_recipe_policy_0 = provider.imagebuilder.Image_recipe_policy {
    policy = "value-0"
    image_recipe_arn = "value-0"
}
image_recipe_policy_1 = provider.imagebuilder.Image_recipe_policy {
    policy = "value-1"
    image_recipe_arn = "value-1"
}
image_recipe_policy_2 = provider.imagebuilder.Image_recipe_policy {
    policy = "value-2"
    image_recipe_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    image_recipe_policy = provider.imagebuilder.Image_recipe_policy {
        policy = "production-value"
        image_recipe_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Imagebuilder Documentation](https://docs.aws.amazon.com/imagebuilder/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
