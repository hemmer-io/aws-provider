# Service_catalog Service



**Resources**: 20

---

## Overview

The service_catalog service provides access to 20 resource types:

- [Product](#product) [CRUD]
- [Copy_product_status](#copy_product_status) [R]
- [Constraint](#constraint) [CRUD]
- [Service_action](#service_action) [CRUD]
- [Provisioning_artifact](#provisioning_artifact) [CRUD]
- [Provisioning_parameters](#provisioning_parameters) [R]
- [Portfolio_shares](#portfolio_shares) [R]
- [Provisioned_product_plan](#provisioned_product_plan) [CRD]
- [Portfolio](#portfolio) [CRUD]
- [Portfolio_share_status](#portfolio_share_status) [R]
- [Provisioned_product_properties](#provisioned_product_properties) [U]
- [Record](#record) [R]
- [Aws_organizations_access_status](#aws_organizations_access_status) [R]
- [Portfolio_share](#portfolio_share) [CUD]
- [Provisioned_product_outputs](#provisioned_product_outputs) [R]
- [Service_action_execution_parameters](#service_action_execution_parameters) [R]
- [Product_view](#product_view) [R]
- [Tag_option](#tag_option) [CRUD]
- [Provisioned_product](#provisioned_product) [RU]
- [Product_as_admin](#product_as_admin) [R]

---

## Resources


### Product

Product resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `support_email` | String |  | <p>The contact email for product support.</p> |
| `distributor` | String |  | <p>The distributor of the product.</p> |
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |
| `product_type` | String | ✅ | <p>The type of product.</p> |
| `provisioning_artifact_parameters` | String |  | <p>The configuration of the provisioning artifact. </p> |
| `tags` | Vec<String> |  | <p>One or more tags.</p> |
| `source_connection` | String |  | <p>Specifies connection details for the created product and syncs the product to the connection source
         artifact. This automatically manages the product's artifacts based on changes to the source.
         The <code>SourceConnection</code> parameter consists of the following sub-fields.</p>
         <ul>
            <li>
               <p>
                  <code>Type</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ConnectionParamters</code>
               </p>
            </li>
         </ul> |
| `idempotency_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p> |
| `owner` | String | ✅ | <p>The owner of the product.</p> |
| `support_description` | String |  | <p>The support information about the product.</p> |
| `name` | String | ✅ | <p>The name of the product.</p> |
| `support_url` | String |  | <p>The contact URL for product support.</p>
         <p>
            <code>^https?:\/\// </code>/ is the pattern used to validate SupportUrl.</p> |
| `description` | String |  | <p>The description of the product.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `budgets` | Vec<String> | <p>Information about the associated budgets.</p> |
| `launch_paths` | Vec<String> | <p>Information about the associated launch paths.</p> |
| `provisioning_artifacts` | Vec<String> | <p>Information about the provisioning artifacts for the specified product.</p> |
| `product_view_summary` | String | <p>Summary information about the product view.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create product
product = provider.service_catalog.Product {
    product_type = "value"  # <p>The type of product.</p>
    idempotency_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p>
    owner = "value"  # <p>The owner of the product.</p>
    name = "value"  # <p>The name of the product.</p>
}

# Access product outputs
product_id = product.id
product_budgets = product.budgets
product_launch_paths = product.launch_paths
product_provisioning_artifacts = product.provisioning_artifacts
product_product_view_summary = product.product_view_summary
```

---


### Copy_product_status

CopyProductStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `copy_product_status` | String | <p>The status of the copy product operation.</p> |
| `target_product_id` | String | <p>The identifier of the copied product.</p> |
| `status_detail` | String | <p>The status message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access copy_product_status outputs
copy_product_status_id = copy_product_status.id
copy_product_status_copy_product_status = copy_product_status.copy_product_status
copy_product_status_target_product_id = copy_product_status.target_product_id
copy_product_status_status_detail = copy_product_status.status_detail
```

---


### Constraint

Constraint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `portfolio_id` | String | ✅ | <p>The portfolio identifier.</p> |
| `product_id` | String | ✅ | <p>The product identifier.</p> |
| `parameters` | String | ✅ | <p>The constraint parameters, in JSON format. The syntax depends on the constraint type as follows:</p>
         <dl>
            <dt>LAUNCH</dt>
            <dd>
               <p>You are required to specify either the <code>RoleArn</code> or the <code>LocalRoleName</code> but can't use both.</p>
               <p>Specify the <code>RoleArn</code> property as follows:</p>
               <p>
                  <code>{"RoleArn" : "arn:aws:iam::123456789012:role/LaunchRole"}</code>
               </p>
               <p>Specify the <code>LocalRoleName</code> property as follows:</p>
               <p>
                  <code>{"LocalRoleName": "SCBasicLaunchRole"}</code>
               </p>
               <p>If you specify the <code>LocalRoleName</code> property, when an account uses the launch constraint, the IAM role with that name in the account will be used. This allows launch-role constraints to be 
               account-agnostic so the administrator can create fewer resources per shared account.</p>
               <note>
                  <p>The given role name must exist in the account used to create the launch constraint and the account of the user who launches a product with this launch constraint.</p>
               </note>
               <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p>
               <p>You also cannot have more than one <code>LAUNCH</code> constraint on a product and portfolio.</p>
            </dd>
            <dt>NOTIFICATION</dt>
            <dd>
               <p>Specify the <code>NotificationArns</code> property as follows:</p>
               <p>
                  <code>{"NotificationArns" : ["arn:aws:sns:us-east-1:123456789012:Topic"]}</code>
               </p>
            </dd>
            <dt>RESOURCE_UPDATE</dt>
            <dd>
               <p>Specify the <code>TagUpdatesOnProvisionedProduct</code> property as follows:</p>
               <p>
                  <code>{"Version":"2.0","Properties":{"TagUpdateOnProvisionedProduct":"String"}}</code>
               </p>
               <p>The <code>TagUpdatesOnProvisionedProduct</code> property accepts a string value of <code>ALLOWED</code> or <code>NOT_ALLOWED</code>.</p>
            </dd>
            <dt>STACKSET</dt>
            <dd>
               <p>Specify the <code>Parameters</code> property as follows:</p>
               <p>
                  <code>{"Version": "String", "Properties": {"AccountList": [ "String" ], "RegionList": [ "String" ], "AdminRole": "String", "ExecutionRole": "String"}}</code>
               </p>
               <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p>
               <p>You also cannot have more than one <code>STACKSET</code> constraint on a product and portfolio.</p>
               <p>Products with a <code>STACKSET</code> constraint will launch an CloudFormation stack set.</p>
            </dd>
            <dt>TEMPLATE</dt>
            <dd>
               <p>Specify the <code>Rules</code> property. For more information, see
                  <a href="http://docs.aws.amazon.com/servicecatalog/latest/adminguide/reference-template_constraint_rules.html">Template Constraint Rules</a>.</p>
            </dd>
         </dl> |
| `type` | String | ✅ | <p>The type of constraint.</p>
         <ul>
            <li>
               <p>
                  <code>LAUNCH</code>
               </p>
            </li>
            <li>
               <p>
                  <code>NOTIFICATION</code>
               </p>
            </li>
            <li>
               <p>
                  <code>RESOURCE_UPDATE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>STACKSET</code>
               </p>
            </li>
            <li>
               <p>
                  <code>TEMPLATE</code>
               </p>
            </li>
         </ul> |
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |
| `description` | String |  | <p>The description of the constraint.</p> |
| `idempotency_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `constraint_parameters` | String | <p>The constraint parameters.</p> |
| `status` | String | <p>The status of the current request.</p> |
| `constraint_detail` | String | <p>Information about the constraint.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create constraint
constraint = provider.service_catalog.Constraint {
    portfolio_id = "value"  # <p>The portfolio identifier.</p>
    product_id = "value"  # <p>The product identifier.</p>
    parameters = "value"  # <p>The constraint parameters, in JSON format. The syntax depends on the constraint type as follows:</p>
         <dl>
            <dt>LAUNCH</dt>
            <dd>
               <p>You are required to specify either the <code>RoleArn</code> or the <code>LocalRoleName</code> but can't use both.</p>
               <p>Specify the <code>RoleArn</code> property as follows:</p>
               <p>
                  <code>{"RoleArn" : "arn:aws:iam::123456789012:role/LaunchRole"}</code>
               </p>
               <p>Specify the <code>LocalRoleName</code> property as follows:</p>
               <p>
                  <code>{"LocalRoleName": "SCBasicLaunchRole"}</code>
               </p>
               <p>If you specify the <code>LocalRoleName</code> property, when an account uses the launch constraint, the IAM role with that name in the account will be used. This allows launch-role constraints to be 
               account-agnostic so the administrator can create fewer resources per shared account.</p>
               <note>
                  <p>The given role name must exist in the account used to create the launch constraint and the account of the user who launches a product with this launch constraint.</p>
               </note>
               <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p>
               <p>You also cannot have more than one <code>LAUNCH</code> constraint on a product and portfolio.</p>
            </dd>
            <dt>NOTIFICATION</dt>
            <dd>
               <p>Specify the <code>NotificationArns</code> property as follows:</p>
               <p>
                  <code>{"NotificationArns" : ["arn:aws:sns:us-east-1:123456789012:Topic"]}</code>
               </p>
            </dd>
            <dt>RESOURCE_UPDATE</dt>
            <dd>
               <p>Specify the <code>TagUpdatesOnProvisionedProduct</code> property as follows:</p>
               <p>
                  <code>{"Version":"2.0","Properties":{"TagUpdateOnProvisionedProduct":"String"}}</code>
               </p>
               <p>The <code>TagUpdatesOnProvisionedProduct</code> property accepts a string value of <code>ALLOWED</code> or <code>NOT_ALLOWED</code>.</p>
            </dd>
            <dt>STACKSET</dt>
            <dd>
               <p>Specify the <code>Parameters</code> property as follows:</p>
               <p>
                  <code>{"Version": "String", "Properties": {"AccountList": [ "String" ], "RegionList": [ "String" ], "AdminRole": "String", "ExecutionRole": "String"}}</code>
               </p>
               <p>You cannot have both a <code>LAUNCH</code> and a <code>STACKSET</code> constraint.</p>
               <p>You also cannot have more than one <code>STACKSET</code> constraint on a product and portfolio.</p>
               <p>Products with a <code>STACKSET</code> constraint will launch an CloudFormation stack set.</p>
            </dd>
            <dt>TEMPLATE</dt>
            <dd>
               <p>Specify the <code>Rules</code> property. For more information, see
                  <a href="http://docs.aws.amazon.com/servicecatalog/latest/adminguide/reference-template_constraint_rules.html">Template Constraint Rules</a>.</p>
            </dd>
         </dl>
    type = "value"  # <p>The type of constraint.</p>
         <ul>
            <li>
               <p>
                  <code>LAUNCH</code>
               </p>
            </li>
            <li>
               <p>
                  <code>NOTIFICATION</code>
               </p>
            </li>
            <li>
               <p>
                  <code>RESOURCE_UPDATE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>STACKSET</code>
               </p>
            </li>
            <li>
               <p>
                  <code>TEMPLATE</code>
               </p>
            </li>
         </ul>
    idempotency_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p>
}

# Access constraint outputs
constraint_id = constraint.id
constraint_constraint_parameters = constraint.constraint_parameters
constraint_status = constraint.status
constraint_constraint_detail = constraint.constraint_detail
```

---


### Service_action

ServiceAction resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `idempotency_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p> |
| `definition_type` | String | ✅ | <p>The service action definition type. For example, <code>SSM_AUTOMATION</code>.</p> |
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |
| `description` | String |  | <p>The self-service action description.</p> |
| `name` | String | ✅ | <p>The self-service action name.</p> |
| `definition` | HashMap<String, String> | ✅ | <p>The self-service action definition. Can be one of the following:</p>
         <dl>
            <dt>Name</dt>
            <dd>
               <p>The name of the Amazon Web Services Systems Manager document (SSM document). For example, <code>AWS-RestartEC2Instance</code>.</p>
               <p>If you are using a shared SSM document, you must provide the ARN instead of the name.</p>
            </dd>
            <dt>Version</dt>
            <dd>
               <p>The Amazon Web Services Systems Manager automation document version. For example, <code>"Version": "1"</code>
               </p>
            </dd>
            <dt>AssumeRole</dt>
            <dd>
               <p>The Amazon Resource Name (ARN) of the role that performs the self-service actions on your behalf. For example, <code>"AssumeRole": "arn:aws:iam::12345678910:role/ActionRole"</code>.</p>
               <p>To reuse the provisioned product launch role, set to <code>"AssumeRole": "LAUNCH_ROLE"</code>.</p>
            </dd>
            <dt>Parameters</dt>
            <dd>
               <p>The list of parameters in JSON format.</p>
               <p>For example: <code>[{\"Name\":\"InstanceId\",\"Type\":\"TARGET\"}]</code> or <code>[{\"Name\":\"InstanceId\",\"Type\":\"TEXT_VALUE\"}]</code>.</p>
            </dd>
         </dl> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_action_detail` | String | <p>Detailed information about the self-service action.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service_action
service_action = provider.service_catalog.Service_action {
    idempotency_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p>
    definition_type = "value"  # <p>The service action definition type. For example, <code>SSM_AUTOMATION</code>.</p>
    name = "value"  # <p>The self-service action name.</p>
    definition = "value"  # <p>The self-service action definition. Can be one of the following:</p>
         <dl>
            <dt>Name</dt>
            <dd>
               <p>The name of the Amazon Web Services Systems Manager document (SSM document). For example, <code>AWS-RestartEC2Instance</code>.</p>
               <p>If you are using a shared SSM document, you must provide the ARN instead of the name.</p>
            </dd>
            <dt>Version</dt>
            <dd>
               <p>The Amazon Web Services Systems Manager automation document version. For example, <code>"Version": "1"</code>
               </p>
            </dd>
            <dt>AssumeRole</dt>
            <dd>
               <p>The Amazon Resource Name (ARN) of the role that performs the self-service actions on your behalf. For example, <code>"AssumeRole": "arn:aws:iam::12345678910:role/ActionRole"</code>.</p>
               <p>To reuse the provisioned product launch role, set to <code>"AssumeRole": "LAUNCH_ROLE"</code>.</p>
            </dd>
            <dt>Parameters</dt>
            <dd>
               <p>The list of parameters in JSON format.</p>
               <p>For example: <code>[{\"Name\":\"InstanceId\",\"Type\":\"TARGET\"}]</code> or <code>[{\"Name\":\"InstanceId\",\"Type\":\"TEXT_VALUE\"}]</code>.</p>
            </dd>
         </dl>
}

# Access service_action outputs
service_action_id = service_action.id
service_action_service_action_detail = service_action.service_action_detail
```

---


### Provisioning_artifact

ProvisioningArtifact resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | String | ✅ | <p>The product identifier.</p> |
| `parameters` | String | ✅ | <p>The configuration for the provisioning artifact.</p> |
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |
| `idempotency_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provisioning_artifact_parameters` | Vec<String> | <p>Information about the parameters used to provision the product.  </p> |
| `provisioning_artifact_detail` | String | <p>Information about the provisioning artifact.</p> |
| `info` | HashMap<String, String> | <p>The URL of the CloudFormation template in Amazon S3 or GitHub in JSON format.</p> |
| `status` | String | <p>The status of the current request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create provisioning_artifact
provisioning_artifact = provider.service_catalog.Provisioning_artifact {
    product_id = "value"  # <p>The product identifier.</p>
    parameters = "value"  # <p>The configuration for the provisioning artifact.</p>
    idempotency_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p>
}

# Access provisioning_artifact outputs
provisioning_artifact_id = provisioning_artifact.id
provisioning_artifact_provisioning_artifact_parameters = provisioning_artifact.provisioning_artifact_parameters
provisioning_artifact_provisioning_artifact_detail = provisioning_artifact.provisioning_artifact_detail
provisioning_artifact_info = provisioning_artifact.info
provisioning_artifact_status = provisioning_artifact.status
```

---


### Provisioning_parameters

ProvisioningParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provisioning_artifact_preferences` | String | <p>An object that contains information about preferences, such as Regions and accounts, for the provisioning artifact.</p> |
| `tag_options` | Vec<String> | <p>Information about the TagOptions associated with the resource.</p> |
| `provisioning_artifact_outputs` | Vec<String> | <p>The output of the provisioning artifact.</p> |
| `provisioning_artifact_output_keys` | Vec<String> | <p>A list of the keys and descriptions of the outputs. These outputs can be referenced from a provisioned product launched from this provisioning artifact.</p> |
| `usage_instructions` | Vec<String> | <p>Any additional metadata specifically related to the provisioning of the product. For
         example, see the <code>Version</code> field of the CloudFormation template.</p> |
| `provisioning_artifact_parameters` | Vec<String> | <p>Information about the parameters used to provision the product.</p> |
| `constraint_summaries` | Vec<String> | <p>Information about the constraints used to provision the product.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access provisioning_parameters outputs
provisioning_parameters_id = provisioning_parameters.id
provisioning_parameters_provisioning_artifact_preferences = provisioning_parameters.provisioning_artifact_preferences
provisioning_parameters_tag_options = provisioning_parameters.tag_options
provisioning_parameters_provisioning_artifact_outputs = provisioning_parameters.provisioning_artifact_outputs
provisioning_parameters_provisioning_artifact_output_keys = provisioning_parameters.provisioning_artifact_output_keys
provisioning_parameters_usage_instructions = provisioning_parameters.usage_instructions
provisioning_parameters_provisioning_artifact_parameters = provisioning_parameters.provisioning_artifact_parameters
provisioning_parameters_constraint_summaries = provisioning_parameters.constraint_summaries
```

---


### Portfolio_shares

PortfolioShares resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `portfolio_share_details` | Vec<String> | <p>Summaries about each of the portfolio shares.</p> |
| `next_page_token` | String | <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access portfolio_shares outputs
portfolio_shares_id = portfolio_shares.id
portfolio_shares_portfolio_share_details = portfolio_shares.portfolio_share_details
portfolio_shares_next_page_token = portfolio_shares.next_page_token
```

---


### Provisioned_product_plan

ProvisionedProductPlan resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |
| `provisioning_artifact_id` | String | ✅ | <p>The identifier of the provisioning artifact.</p> |
| `notification_arns` | Vec<String> |  | <p>Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related
         events.</p> |
| `path_id` | String |  | <p>The path identifier of the product. This value is optional if the product 
         has a default path, and required if the product has more than one path.
         To list the paths for a product, use <a>ListLaunchPaths</a>.</p> |
| `provisioning_parameters` | Vec<String> |  | <p>Parameters specified by the administrator that are required for provisioning the
         product.</p> |
| `product_id` | String | ✅ | <p>The product identifier.</p> |
| `plan_type` | String | ✅ | <p>The plan type.</p> |
| `plan_name` | String | ✅ | <p>The name of the plan.</p> |
| `provisioned_product_name` | String | ✅ | <p>A user-friendly name for the provisioned product. This value must be
         unique for the Amazon Web Services account and cannot be updated after the product is provisioned.</p> |
| `idempotency_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p> |
| `tags` | Vec<String> |  | <p>One or more tags.</p>
         <p>If the plan is for an existing provisioned product, the product must have a <code>RESOURCE_UPDATE</code> constraint with <code>TagUpdatesOnProvisionedProduct</code> set to <code>ALLOWED</code> to allow tag updates.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provisioned_product_plan_details` | String | <p>Information about the plan.</p> |
| `next_page_token` | String | <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p> |
| `resource_changes` | Vec<String> | <p>Information about the resource changes that will occur when the plan is executed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create provisioned_product_plan
provisioned_product_plan = provider.service_catalog.Provisioned_product_plan {
    provisioning_artifact_id = "value"  # <p>The identifier of the provisioning artifact.</p>
    product_id = "value"  # <p>The product identifier.</p>
    plan_type = "value"  # <p>The plan type.</p>
    plan_name = "value"  # <p>The name of the plan.</p>
    provisioned_product_name = "value"  # <p>A user-friendly name for the provisioned product. This value must be
         unique for the Amazon Web Services account and cannot be updated after the product is provisioned.</p>
    idempotency_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p>
}

# Access provisioned_product_plan outputs
provisioned_product_plan_id = provisioned_product_plan.id
provisioned_product_plan_provisioned_product_plan_details = provisioned_product_plan.provisioned_product_plan_details
provisioned_product_plan_next_page_token = provisioned_product_plan.next_page_token
provisioned_product_plan_resource_changes = provisioned_product_plan.resource_changes
```

---


### Portfolio

Portfolio resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the portfolio.</p> |
| `display_name` | String | ✅ | <p>The name to use for display purposes.</p> |
| `provider_name` | String | ✅ | <p>The name of the portfolio provider.</p> |
| `idempotency_token` | String | ✅ | <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p> |
| `tags` | Vec<String> |  | <p>One or more tags.</p> |
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | Vec<String> | <p>Information about the tags associated with the portfolio.</p> |
| `tag_options` | Vec<String> | <p>Information about the TagOptions associated with the portfolio.</p> |
| `portfolio_detail` | String | <p>Information about the portfolio.</p> |
| `budgets` | Vec<String> | <p>Information about the associated budgets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create portfolio
portfolio = provider.service_catalog.Portfolio {
    display_name = "value"  # <p>The name to use for display purposes.</p>
    provider_name = "value"  # <p>The name of the portfolio provider.</p>
    idempotency_token = "value"  # <p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token,
  the same response is returned for each repeated request.</p>
}

# Access portfolio outputs
portfolio_id = portfolio.id
portfolio_tags = portfolio.tags
portfolio_tag_options = portfolio.tag_options
portfolio_portfolio_detail = portfolio.portfolio_detail
portfolio_budgets = portfolio.budgets
```

---


### Portfolio_share_status

PortfolioShareStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `share_details` | String | <p>Information about the portfolio share operation.</p> |
| `organization_node_value` | String | <p>Organization node identifier. It can be either account id, organizational unit id or organization id.</p> |
| `status` | String | <p>Status of the portfolio share operation.</p> |
| `portfolio_share_token` | String | <p>The token for the portfolio share operation. For example, <code>share-6v24abcdefghi</code>.</p> |
| `portfolio_id` | String | <p>The portfolio identifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access portfolio_share_status outputs
portfolio_share_status_id = portfolio_share_status.id
portfolio_share_status_share_details = portfolio_share_status.share_details
portfolio_share_status_organization_node_value = portfolio_share_status.organization_node_value
portfolio_share_status_status = portfolio_share_status.status
portfolio_share_status_portfolio_share_token = portfolio_share_status.portfolio_share_token
portfolio_share_status_portfolio_id = portfolio_share_status.portfolio_id
```

---


### Provisioned_product_properties

ProvisionedProductProperties resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provisioned_product_id` | String | ✅ | <p>The identifier of the provisioned product.</p> |
| `idempotency_token` | String | ✅ | <p>The idempotency token that uniquely identifies the provisioning product update request.</p> |
| `provisioned_product_properties` | HashMap<String, String> | ✅ | <p>A map that contains the provisioned product properties to be updated.</p>
         <p>The <code>LAUNCH_ROLE</code> key accepts role ARNs. This key allows an
         administrator to call <code>UpdateProvisionedProductProperties</code> to update the launch
         role that is associated with a provisioned product. This role is used when an end user
         calls a provisioning operation such as <code>UpdateProvisionedProduct</code>,
            <code>TerminateProvisionedProduct</code>, or
            <code>ExecuteProvisionedProductServiceAction</code>. Only a role ARN is valid. A user ARN is invalid. </p>
         <p>The <code>OWNER</code> key accepts user ARNs, IAM role ARNs, and STS 
         assumed-role ARNs. The owner is the user that has permission to see, update, terminate, and 
         execute service actions in the provisioned product.</p>
         <p>The administrator can change the owner of a provisioned product to another IAM or STS entity within the 
         same account. Both end user owners and administrators can see ownership history of the provisioned 
         product using the <code>ListRecordHistory</code> API. The new owner can describe all past records 
         for the provisioned product using the <code>DescribeRecord</code> API. The previous owner can no 
         longer use <code>DescribeRecord</code>, but can still see the product's history from when he was 
         an owner using <code>ListRecordHistory</code>.</p>
         <p>If a provisioned product ownership is assigned to an end user, they can see and perform any action through the API or 
         Service Catalog console such as update, terminate, and execute service actions. 
         If an end user provisions a product and the owner is updated to someone else, they will no longer be able to see or perform any actions through 
         API or the Service Catalog console on that provisioned product.</p> |
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
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

```

---


### Record

Record resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `record_detail` | String | <p>Information about the product.</p> |
| `next_page_token` | String | <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p> |
| `record_outputs` | Vec<String> | <p>Information about the product created as the result of a request. For example, the output for  
         a CloudFormation-backed product that creates an S3 bucket would include the S3 bucket URL.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access record outputs
record_id = record.id
record_record_detail = record.record_detail
record_next_page_token = record.next_page_token
record_record_outputs = record.record_outputs
```

---


### Aws_organizations_access_status

AWSOrganizationsAccessStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_status` | String | <p>The status of the portfolio share feature.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aws_organizations_access_status outputs
aws_organizations_access_status_id = aws_organizations_access_status.id
aws_organizations_access_status_access_status = aws_organizations_access_status.access_status
```

---


### Portfolio_share

PortfolioShare resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |
| `account_id` | String |  | <p>The Amazon Web Services account ID. For example, <code>123456789012</code>.</p> |
| `organization_node` | String |  | <p>The organization node to whom you are going to share. When you pass <code>OrganizationNode</code>, it creates <code>PortfolioShare</code> for all of the Amazon Web Services accounts that are associated to the <code>OrganizationNode</code>. 
      The output returns a <code>PortfolioShareToken</code>, which enables the administrator to monitor the status of the <code>PortfolioShare</code> creation process.</p> |
| `portfolio_id` | String | ✅ | <p>The portfolio identifier.</p> |
| `share_tag_options` | bool |  | <p>Enables or disables <code>TagOptions </code> sharing when creating the portfolio share. If this flag is not 
         provided, TagOptions sharing is disabled.</p> |
| `share_principals` | bool |  | <p>This parameter is only supported for portfolios with an <b>OrganizationalNode</b> 
      Type of <code>ORGANIZATION</code> or <code>ORGANIZATIONAL_UNIT</code>. </p>
         <p>Enables or disables <code>Principal</code> sharing when creating the portfolio share. If you do 
         <b>not</b> provide this flag, principal sharing is disabled. </p>
         <p>When you enable Principal Name Sharing for a portfolio share, the share recipient
         account end users with a principal that matches any of the associated IAM
         patterns can provision products from the portfolio. Once
         shared, the share recipient can view associations of <code>PrincipalType</code>: 
         <code>IAM_PATTERN</code> on their portfolio. You can create the principals in the recipient account before or 
         after creating the share. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create portfolio_share
portfolio_share = provider.service_catalog.Portfolio_share {
    portfolio_id = "value"  # <p>The portfolio identifier.</p>
}

```

---


### Provisioned_product_outputs

ProvisionedProductOutputs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `outputs` | Vec<String> | <p>Information about the product created as the result of a request. For example, the output for a CloudFormation-backed product that creates an S3 bucket would include the S3 bucket URL.
      </p> |
| `next_page_token` | String | <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access provisioned_product_outputs outputs
provisioned_product_outputs_id = provisioned_product_outputs.id
provisioned_product_outputs_outputs = provisioned_product_outputs.outputs
provisioned_product_outputs_next_page_token = provisioned_product_outputs.next_page_token
```

---


### Service_action_execution_parameters

ServiceActionExecutionParameters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_action_parameters` | Vec<String> | <p>The parameters of the self-service action.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_action_execution_parameters outputs
service_action_execution_parameters_id = service_action_execution_parameters.id
service_action_execution_parameters_service_action_parameters = service_action_execution_parameters.service_action_parameters
```

---


### Product_view

ProductView resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_view_summary` | String | <p>Summary information about the product.</p> |
| `provisioning_artifacts` | Vec<String> | <p>Information about the provisioning artifacts for the product.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access product_view outputs
product_view_id = product_view.id
product_view_product_view_summary = product_view.product_view_summary
product_view_provisioning_artifacts = product_view.provisioning_artifacts
```

---


### Tag_option

TagOption resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | String | ✅ | <p>The TagOption key.</p> |
| `value` | String | ✅ | <p>The TagOption value.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_option_detail` | String | <p>Information about the TagOption.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tag_option
tag_option = provider.service_catalog.Tag_option {
    key = "value"  # <p>The TagOption key.</p>
    value = "value"  # <p>The TagOption value.</p>
}

# Access tag_option outputs
tag_option_id = tag_option.id
tag_option_tag_option_detail = tag_option.tag_option_detail
```

---


### Provisioned_product

ProvisionedProduct resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provisioning_parameters` | Vec<String> |  | <p>The new parameters.</p> |
| `tags` | Vec<String> |  | <p>One or more tags. Requires the product to have <code>RESOURCE_UPDATE</code> constraint with <code>TagUpdatesOnProvisionedProduct</code> set to <code>ALLOWED</code> to allow tag updates.</p> |
| `update_token` | String | ✅ | <p>The idempotency token that uniquely identifies the provisioning update request.</p> |
| `path_id` | String |  | <p>The path identifier. This value is optional if the product 
         has a default path, and required if the product has more than one path. You must provide the name or ID, but not both.</p> |
| `path_name` | String |  | <p>The name of the path. You must provide the name or ID, but not both.</p> |
| `provisioned_product_name` | String |  | <p>The name of the provisioned product. You cannot specify both
         <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p> |
| `product_name` | String |  | <p>The name of the product. You must provide the name or ID, but not both.</p> |
| `accept_language` | String |  | <p>The language code.</p>
         <ul>
            <li>
               <p>
                  <code>jp</code> - Japanese</p>
            </li>
            <li>
               <p>
                  <code>zh</code> - Chinese</p>
            </li>
         </ul> |
| `provisioning_artifact_id` | String |  | <p>The identifier of the provisioning artifact.</p> |
| `provisioning_artifact_name` | String |  | <p>The name of the provisioning artifact. You must provide the name or ID, but not both.</p> |
| `provisioned_product_id` | String |  | <p>The identifier of the provisioned product. You must provide the name or ID, but not both.</p> |
| `provisioning_preferences` | String |  | <p>An object that contains information about the provisioning preferences for a stack set.</p> |
| `product_id` | String |  | <p>The identifier of the product. You must provide the name or ID, but not both.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provisioned_product_detail` | String | <p>Information about the provisioned product.</p> |
| `cloud_watch_dashboards` | Vec<String> | <p>Any CloudWatch dashboards that were created when provisioning the product.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access provisioned_product outputs
provisioned_product_id = provisioned_product.id
provisioned_product_provisioned_product_detail = provisioned_product.provisioned_product_detail
provisioned_product_cloud_watch_dashboards = provisioned_product.cloud_watch_dashboards
```

---


### Product_as_admin

ProductAsAdmin resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_view_detail` | String | <p>Information about the product view.</p> |
| `provisioning_artifact_summaries` | Vec<String> | <p>Information about the provisioning artifacts (also known as versions) for the specified product.</p> |
| `budgets` | Vec<String> | <p>Information about the associated budgets.</p> |
| `tags` | Vec<String> | <p>Information about the tags associated with the product.</p> |
| `tag_options` | Vec<String> | <p>Information about the TagOptions associated with the product.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access product_as_admin outputs
product_as_admin_id = product_as_admin.id
product_as_admin_product_view_detail = product_as_admin.product_view_detail
product_as_admin_provisioning_artifact_summaries = product_as_admin.provisioning_artifact_summaries
product_as_admin_budgets = product_as_admin.budgets
product_as_admin_tags = product_as_admin.tags
product_as_admin_tag_options = product_as_admin.tag_options
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple product resources
product_0 = provider.service_catalog.Product {
    product_type = "value-0"
    idempotency_token = "value-0"
    owner = "value-0"
    name = "value-0"
}
product_1 = provider.service_catalog.Product {
    product_type = "value-1"
    idempotency_token = "value-1"
    owner = "value-1"
    name = "value-1"
}
product_2 = provider.service_catalog.Product {
    product_type = "value-2"
    idempotency_token = "value-2"
    owner = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    product = provider.service_catalog.Product {
        product_type = "production-value"
        idempotency_token = "production-value"
        owner = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Service_catalog Documentation](https://docs.aws.amazon.com/service_catalog/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
