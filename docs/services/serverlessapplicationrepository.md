# Serverlessapplicationrepository Service



**Resources**: 5

---

## Overview

The serverlessapplicationrepository service provides access to 5 resource types:

- [Cloud_formation_template](#cloud_formation_template) [CR]
- [Application_policy](#application_policy) [CR]
- [Application](#application) [CRUD]
- [Cloud_formation_change_set](#cloud_formation_change_set) [C]
- [Application_version](#application_version) [C]

---

## Resources


### Cloud_formation_template

CloudFormationTemplate resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The Amazon Resource Name (ARN) of the application.</p> |
| `semantic_version` | String |  | <p>The semantic version of the application:</p><p>
 <a href="https://semver.org/">https://semver.org/</a>
 </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The date and time this resource was created.</p> |
| `semantic_version` | String | <p>The semantic version of the application:</p><p>
 <a href="https://semver.org/">https://semver.org/</a>
 </p> |
| `expiration_time` | String | <p>The date and time this template expires. Templates
 expire 1 hour after creation.</p> |
| `template_url` | String | <p>A link to the template that can be used to deploy the application using
 AWS CloudFormation.</p> |
| `status` | String | <p>Status of the template creation workflow.</p><p>Possible values: PREPARING | ACTIVE | EXPIRED
 </p> |
| `application_id` | String | <p>The application Amazon Resource Name (ARN).</p> |
| `template_id` | String | <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cloud_formation_template
cloud_formation_template = provider.serverlessapplicationrepository.Cloud_formation_template {
    application_id = "value"  # <p>The Amazon Resource Name (ARN) of the application.</p>
}

# Access cloud_formation_template outputs
cloud_formation_template_id = cloud_formation_template.id
cloud_formation_template_creation_time = cloud_formation_template.creation_time
cloud_formation_template_semantic_version = cloud_formation_template.semantic_version
cloud_formation_template_expiration_time = cloud_formation_template.expiration_time
cloud_formation_template_template_url = cloud_formation_template.template_url
cloud_formation_template_status = cloud_formation_template.status
cloud_formation_template_application_id = cloud_formation_template.application_id
cloud_formation_template_template_id = cloud_formation_template.template_id
```

---


### Application_policy

ApplicationPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `statements` | Vec<String> | ✅ | <p>An array of policy statements applied to the application.</p> |
| `application_id` | String | ✅ | <p>The Amazon Resource Name (ARN) of the application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statements` | Vec<String> | <p>An array of policy statements applied to the application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_policy
application_policy = provider.serverlessapplicationrepository.Application_policy {
    statements = "value"  # <p>An array of policy statements applied to the application.</p>
    application_id = "value"  # <p>The Amazon Resource Name (ARN) of the application.</p>
}

# Access application_policy outputs
application_policy_id = application_policy.id
application_policy_statements = application_policy.statements
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `license_body` | String |  | <p>A local text file that contains the license of the app that matches the spdxLicenseID value of your application.
 The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p> |
| `labels` | Vec<String> |  | <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p> |
| `home_page_url` | String |  | <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p> |
| `template_body` | String |  | <p>The local raw packaged AWS SAM template file of your application.
 The file has the format file://&lt;path>/&lt;filename>.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p> |
| `source_code_archive_url` | String |  | <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p><p>Maximum size 50 MB</p> |
| `readme_body` | String |  | <p>A local text readme file in Markdown language that contains a more detailed description of the application and how it works.
 The file has the format file://&lt;path>/&lt;filename>.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p> |
| `source_code_url` | String |  | <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p> |
| `author` | String | ✅ | <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p> |
| `readme_url` | String |  | <p>A link to the S3 object in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p><p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p> |
| `description` | String | ✅ | <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p> |
| `semantic_version` | String |  | <p>The semantic version of the application:</p><p>
 <a href="https://semver.org/">https://semver.org/</a>
 </p> |
| `template_url` | String |  | <p>A link to the S3 object containing the packaged AWS SAM template of your application.</p><p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p> |
| `license_url` | String |  | <p>A link to the S3 object that contains the license of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p><p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p> |
| `name` | String | ✅ | <p>The name of the application that you want to publish.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p> |
| `spdx_license_id` | String |  | <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `readme_url` | String | <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p><p>Maximum size 5 MB</p> |
| `creation_time` | String | <p>The date and time this resource was created.</p> |
| `is_verified_author` | bool | <p>Whether the author of this application has been verified. This means means that AWS has made a good faith review, as a reasonable and prudent service provider, of the information provided by the requester and has confirmed that the requester's identity is as claimed.</p> |
| `spdx_license_id` | String | <p>A valid identifier from https://spdx.org/licenses/.</p> |
| `verified_author_url` | String | <p>The URL to the public profile of a verified author. This URL is submitted by the author.</p> |
| `description` | String | <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p> |
| `application_id` | String | <p>The application Amazon Resource Name (ARN).</p> |
| `labels` | Vec<String> | <p>Labels to improve discovery of apps in search results.</p><p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p><p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p> |
| `license_url` | String | <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p><p>Maximum size 5 MB</p> |
| `home_page_url` | String | <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p> |
| `author` | String | <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p> |
| `name` | String | <p>The name of the application.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p> |
| `version` | String | <p>Version information about the application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.serverlessapplicationrepository.Application {
    author = "value"  # <p>The name of the author publishing the app.</p><p>Minimum length=1. Maximum length=127.</p><p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    description = "value"  # <p>The description of the application.</p><p>Minimum length=1. Maximum length=256</p>
    name = "value"  # <p>The name of the application that you want to publish.</p><p>Minimum length=1. Maximum length=140</p><p>Pattern: "[a-zA-Z0-9\\-]+";</p>
}

# Access application outputs
application_id = application.id
application_readme_url = application.readme_url
application_creation_time = application.creation_time
application_is_verified_author = application.is_verified_author
application_spdx_license_id = application.spdx_license_id
application_verified_author_url = application.verified_author_url
application_description = application.description
application_application_id = application.application_id
application_labels = application.labels
application_license_url = application.license_url
application_home_page_url = application.home_page_url
application_author = application.author
application_name = application.name
application_version = application.version
```

---


### Cloud_formation_change_set

CloudFormationChangeSet resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |
| `application_id` | String | ✅ | <p>The Amazon Resource Name (ARN) of the application.</p> |
| `parameter_overrides` | Vec<String> |  | <p>A list of parameter values for the parameters of the application.</p> |
| `capabilities` | Vec<String> |  | <p>A list of values that you must specify before you can deploy certain applications.
 Some applications might include resources that can affect permissions in your AWS
 account, for example, by creating new AWS Identity and Access Management (IAM) users.
 For those applications, you must explicitly acknowledge their capabilities by
 specifying this parameter.</p><p>The only valid values are CAPABILITY_IAM, CAPABILITY_NAMED_IAM,
 CAPABILITY_RESOURCE_POLICY, and CAPABILITY_AUTO_EXPAND.</p><p>The following resources require you to specify CAPABILITY_IAM or
 CAPABILITY_NAMED_IAM:
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html">AWS::IAM::Group</a>,
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a>,
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM::Policy</a>, and
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html">AWS::IAM::Role</a>.
 If the application contains IAM resources, you can specify either CAPABILITY_IAM
 or CAPABILITY_NAMED_IAM. If the application contains IAM resources
 with custom names, you must specify CAPABILITY_NAMED_IAM.</p><p>The following resources require you to specify CAPABILITY_RESOURCE_POLICY:
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html">AWS::Lambda::Permission</a>,
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html">AWS::IAM:Policy</a>,
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html">AWS::ApplicationAutoScaling::ScalingPolicy</a>,
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html">AWS::S3::BucketPolicy</a>,
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sqs-policy.html">AWS::SQS::QueuePolicy</a>, and
 <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html">AWS::SNS:TopicPolicy</a>.</p><p>Applications that contain one or more nested applications require you to specify
 CAPABILITY_AUTO_EXPAND.</p><p>If your application template contains any of the above resources, we recommend that you review
 all permissions associated with the application before deploying. If you don't specify
 this parameter for an application that requires capabilities, the call will fail.</p> |
| `tags` | Vec<String> |  | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |
| `resource_types` | Vec<String> |  | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |
| `rollback_configuration` | String |  | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |
| `template_id` | String |  | <p>The UUID returned by CreateCloudFormationTemplate.</p><p>Pattern: [0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}</p> |
| `notification_arns` | Vec<String> |  | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |
| `change_set_name` | String |  | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |
| `description` | String |  | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |
| `semantic_version` | String |  | <p>The semantic version of the application:</p><p>
 <a href="https://semver.org/">https://semver.org/</a>
 </p> |
| `stack_name` | String | ✅ | <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cloud_formation_change_set
cloud_formation_change_set = provider.serverlessapplicationrepository.Cloud_formation_change_set {
    application_id = "value"  # <p>The Amazon Resource Name (ARN) of the application.</p>
    stack_name = "value"  # <p>This property corresponds to the parameter of the same name for the <i>AWS CloudFormation <a href="https://docs.aws.amazon.com/goto/WebAPI/cloudformation-2010-05-15/CreateChangeSet">CreateChangeSet</a>
 </i> API.</p>
}

```

---


### Application_version

ApplicationVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_code_url` | String |  | <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p> |
| `template_url` | String |  | <p>A link to the packaged AWS SAM template of your application.</p> |
| `application_id` | String | ✅ | <p>The Amazon Resource Name (ARN) of the application.</p> |
| `semantic_version` | String | ✅ | <p>The semantic version of the new version.</p> |
| `source_code_archive_url` | String |  | <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p><p>Maximum size 50 MB</p> |
| `template_body` | String |  | <p>The raw packaged AWS SAM template of your application.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_version
application_version = provider.serverlessapplicationrepository.Application_version {
    application_id = "value"  # <p>The Amazon Resource Name (ARN) of the application.</p>
    semantic_version = "value"  # <p>The semantic version of the new version.</p>
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

# Create multiple cloud_formation_template resources
cloud_formation_template_0 = provider.serverlessapplicationrepository.Cloud_formation_template {
    application_id = "value-0"
}
cloud_formation_template_1 = provider.serverlessapplicationrepository.Cloud_formation_template {
    application_id = "value-1"
}
cloud_formation_template_2 = provider.serverlessapplicationrepository.Cloud_formation_template {
    application_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cloud_formation_template = provider.serverlessapplicationrepository.Cloud_formation_template {
        application_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Serverlessapplicationrepository Documentation](https://docs.aws.amazon.com/serverlessapplicationrepository/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
