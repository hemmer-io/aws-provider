# Appconfig Service



**Resources**: 10

---

## Overview

The appconfig service provides access to 10 resource types:

- [Configuration](#configuration) [R]
- [Deployment_strategy](#deployment_strategy) [CRUD]
- [Configuration_profile](#configuration_profile) [CRUD]
- [Extension_association](#extension_association) [CRUD]
- [Application](#application) [CRUD]
- [Environment](#environment) [CRUD]
- [Extension](#extension) [CRUD]
- [Account_settings](#account_settings) [RU]
- [Deployment](#deployment) [R]
- [Hosted_configuration_version](#hosted_configuration_version) [CRD]

---

## Resources


### Configuration

Configuration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | <p>The content of the configuration or the configuration data.</p>
         <important>
            <p>The <code>Content</code> attribute only contains data if the system finds new or
            updated configuration data. If there is no new or updated data and
               <code>ClientConfigurationVersion</code> matches the version of the current
            configuration, AppConfig returns a <code>204 No Content</code> HTTP response
            code and the <code>Content</code> value will be empty.</p>
         </important> |
| `configuration_version` | String | <p>The configuration version.</p> |
| `content_type` | String | <p>A standard MIME type describing the format of the configuration content. For more
         information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">Content-Type</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration outputs
configuration_id = configuration.id
configuration_content = configuration.content
configuration_configuration_version = configuration.configuration_version
configuration_content_type = configuration.content_type
```

---


### Deployment_strategy

DeploymentStrategy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Metadata to assign to the deployment strategy. Tags help organize and categorize your
            AppConfig resources. Each tag consists of a key and an optional value, both of
         which you define.</p> |
| `final_bake_time_in_minutes` | i64 |  | <p>Specifies the amount of time AppConfig monitors for Amazon CloudWatch alarms after the
         configuration has been deployed to 100% of its targets, before considering the deployment
         to be complete. If an alarm is triggered during this time, AppConfig rolls back
         the deployment. You must configure permissions for AppConfig to roll back based
         on CloudWatch alarms. For more information, see <a href="https://docs.aws.amazon.com/appconfig/latest/userguide/getting-started-with-appconfig-cloudwatch-alarms-permissions.html">Configuring permissions for rollback based on Amazon CloudWatch alarms</a> in the
            <i>AppConfig User Guide</i>.</p> |
| `deployment_duration_in_minutes` | i64 | ✅ | <p>Total amount of time for a deployment to last.</p> |
| `growth_factor` | String | ✅ | <p>The percentage of targets to receive a deployed configuration during each
         interval.</p> |
| `growth_type` | String |  | <p>The algorithm used to define how percentage grows over time. AppConfig
         supports the following growth types:</p>
         <p>
            <b>Linear</b>: For this type, AppConfig processes
         the deployment by dividing the total number of targets by the value specified for
            <code>Step percentage</code>. For example, a linear deployment that uses a <code>Step
            percentage</code> of 10 deploys the configuration to 10 percent of the hosts. After
         those deployments are complete, the system deploys the configuration to the next 10
         percent. This continues until 100% of the targets have successfully received the
         configuration.</p>
         <p>
            <b>Exponential</b>: For this type, AppConfig
         processes the deployment exponentially using the following formula: <code>G*(2^N)</code>.
         In this formula, <code>G</code> is the growth factor specified by the user and
            <code>N</code> is the number of steps until the configuration is deployed to all
         targets. For example, if you specify a growth factor of 2, then the system rolls out the
         configuration as follows:</p>
         <p>
            <code>2*(2^0)</code>
         </p>
         <p>
            <code>2*(2^1)</code>
         </p>
         <p>
            <code>2*(2^2)</code>
         </p>
         <p>Expressed numerically, the deployment rolls out as follows: 2% of the targets, 4% of the
         targets, 8% of the targets, and continues until the configuration has been deployed to all
         targets.</p> |
| `description` | String |  | <p>A description of the deployment strategy.</p> |
| `name` | String | ✅ | <p>A name for the deployment strategy.</p> |
| `replicate_to` | String |  | <p>Save the deployment strategy to a Systems Manager (SSM) document.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the deployment strategy.</p> |
| `replicate_to` | String | <p>Save the deployment strategy to a Systems Manager (SSM) document.</p> |
| `growth_factor` | String | <p>The percentage of targets that received a deployed configuration during each
         interval.</p> |
| `id` | String | <p>The deployment strategy ID.</p> |
| `description` | String | <p>The description of the deployment strategy.</p> |
| `deployment_duration_in_minutes` | i64 | <p>Total amount of time the deployment lasted.</p> |
| `growth_type` | String | <p>The algorithm used to define how percentage grew over time.</p> |
| `final_bake_time_in_minutes` | i64 | <p>The amount of time that AppConfig monitored for alarms before considering the
         deployment to be complete and no longer eligible for automatic rollback.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment_strategy
deployment_strategy = provider.appconfig.Deployment_strategy {
    deployment_duration_in_minutes = "value"  # <p>Total amount of time for a deployment to last.</p>
    growth_factor = "value"  # <p>The percentage of targets to receive a deployed configuration during each
         interval.</p>
    name = "value"  # <p>A name for the deployment strategy.</p>
}

# Access deployment_strategy outputs
deployment_strategy_id = deployment_strategy.id
deployment_strategy_name = deployment_strategy.name
deployment_strategy_replicate_to = deployment_strategy.replicate_to
deployment_strategy_growth_factor = deployment_strategy.growth_factor
deployment_strategy_id = deployment_strategy.id
deployment_strategy_description = deployment_strategy.description
deployment_strategy_deployment_duration_in_minutes = deployment_strategy.deployment_duration_in_minutes
deployment_strategy_growth_type = deployment_strategy.growth_type
deployment_strategy_final_bake_time_in_minutes = deployment_strategy.final_bake_time_in_minutes
```

---


### Configuration_profile

ConfigurationProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | <p>The application ID.</p> |
| `location_uri` | String | ✅ | <p>A URI to locate the configuration. You can specify the following:</p>
         <ul>
            <li>
               <p>For the AppConfig hosted configuration store and for feature flags,
               specify <code>hosted</code>.</p>
            </li>
            <li>
               <p>For an Amazon Web Services Systems Manager Parameter Store parameter, specify either the parameter name in
               the format <code>ssm-parameter://<parameter name></code> or the ARN.</p>
            </li>
            <li>
               <p>For an Amazon Web Services
               CodePipeline pipeline, specify the URI in the following format:
                  <code>codepipeline</code>://<pipeline name>.</p>
            </li>
            <li>
               <p>For an Secrets Manager secret, specify the URI in the following format:
                  <code>secretsmanager</code>://<secret name>.</p>
            </li>
            <li>
               <p>For an Amazon S3 object, specify the URI in the following format:
                  <code>s3://<bucket>/<objectKey> </code>. Here is an example:
                     <code>s3://amzn-s3-demo-bucket/my-app/us-east-1/my-config.json</code>
               </p>
            </li>
            <li>
               <p>For an SSM document, specify either the document name in the format
                  <code>ssm-document://<document name></code> or the Amazon Resource Name
               (ARN).</p>
            </li>
         </ul> |
| `description` | String |  | <p>A description of the configuration profile.</p> |
| `kms_key_identifier` | String |  | <p>The identifier for an Key Management Service key to encrypt new configuration data
         versions in the AppConfig hosted configuration store. This attribute is only used
         for <code>hosted</code> configuration types. The identifier can be an KMS
         key ID, alias, or the Amazon Resource Name (ARN) of the key ID or alias. To encrypt data
         managed in other configuration stores, see the documentation for how to specify an KMS key for that particular service.</p> |
| `validators` | Vec<String> |  | <p>A list of methods for validating the configuration.</p> |
| `name` | i64 | ✅ | <p>A name for the configuration profile.</p> |
| `retrieval_role_arn` | String |  | <p>The ARN of an IAM role with permission to access the configuration at the specified
            <code>LocationUri</code>.</p>
         <important>
            <p>A retrieval role ARN is not required for configurations stored in CodePipeline or the AppConfig hosted configuration store. It is required for all other sources that
            store your configuration. </p>
         </important> |
| `tags` | HashMap<String, String> |  | <p>Metadata to assign to the configuration profile. Tags help organize and categorize your
            AppConfig resources. Each tag consists of a key and an optional value, both of
         which you define.</p> |
| `type` | String |  | <p>The type of configurations contained in the profile. AppConfig supports
            <code>feature flags</code> and <code>freeform</code> configurations. We recommend you
         create feature flag configurations to enable or disable new features and freeform
         configurations to distribute configurations to an application. When calling this API, enter
         one of the following values for <code>Type</code>:</p>
         <p>
            <code>AWS.AppConfig.FeatureFlags</code>
         </p>
         <p>
            <code>AWS.Freeform</code>
         </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The type of configurations contained in the profile. AppConfig supports
            <code>feature flags</code> and <code>freeform</code> configurations. We recommend you
         create feature flag configurations to enable or disable new features and freeform
         configurations to distribute configurations to an application. When calling this API, enter
         one of the following values for <code>Type</code>:</p>
         <p>
            <code>AWS.AppConfig.FeatureFlags</code>
         </p>
         <p>
            <code>AWS.Freeform</code>
         </p> |
| `name` | i64 | <p>The name of the configuration profile.</p> |
| `id` | String | <p>The configuration profile ID.</p> |
| `location_uri` | String | <p>The URI location of the configuration.</p> |
| `description` | String | <p>The configuration profile description.</p> |
| `kms_key_identifier` | String | <p>The Key Management Service key identifier (key ID, key alias, or key ARN) provided when
         the resource was created or updated.</p> |
| `kms_key_arn` | String | <p>The Amazon Resource Name of the Key Management Service key to encrypt new configuration
         data versions in the AppConfig hosted configuration store. This attribute is only
         used for <code>hosted</code> configuration types. To encrypt data managed in other
         configuration stores, see the documentation for how to specify an KMS key
         for that particular service.</p> |
| `validators` | Vec<String> | <p>A list of methods for validating the configuration.</p> |
| `retrieval_role_arn` | String | <p>The ARN of an IAM role with permission to access the configuration at the specified
            <code>LocationUri</code>.</p> |
| `application_id` | String | <p>The application ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_profile
configuration_profile = provider.appconfig.Configuration_profile {
    application_id = "value"  # <p>The application ID.</p>
    location_uri = "value"  # <p>A URI to locate the configuration. You can specify the following:</p>
         <ul>
            <li>
               <p>For the AppConfig hosted configuration store and for feature flags,
               specify <code>hosted</code>.</p>
            </li>
            <li>
               <p>For an Amazon Web Services Systems Manager Parameter Store parameter, specify either the parameter name in
               the format <code>ssm-parameter://<parameter name></code> or the ARN.</p>
            </li>
            <li>
               <p>For an Amazon Web Services
               CodePipeline pipeline, specify the URI in the following format:
                  <code>codepipeline</code>://<pipeline name>.</p>
            </li>
            <li>
               <p>For an Secrets Manager secret, specify the URI in the following format:
                  <code>secretsmanager</code>://<secret name>.</p>
            </li>
            <li>
               <p>For an Amazon S3 object, specify the URI in the following format:
                  <code>s3://<bucket>/<objectKey> </code>. Here is an example:
                     <code>s3://amzn-s3-demo-bucket/my-app/us-east-1/my-config.json</code>
               </p>
            </li>
            <li>
               <p>For an SSM document, specify either the document name in the format
                  <code>ssm-document://<document name></code> or the Amazon Resource Name
               (ARN).</p>
            </li>
         </ul>
    name = "value"  # <p>A name for the configuration profile.</p>
}

# Access configuration_profile outputs
configuration_profile_id = configuration_profile.id
configuration_profile_type = configuration_profile.type
configuration_profile_name = configuration_profile.name
configuration_profile_id = configuration_profile.id
configuration_profile_location_uri = configuration_profile.location_uri
configuration_profile_description = configuration_profile.description
configuration_profile_kms_key_identifier = configuration_profile.kms_key_identifier
configuration_profile_kms_key_arn = configuration_profile.kms_key_arn
configuration_profile_validators = configuration_profile.validators
configuration_profile_retrieval_role_arn = configuration_profile.retrieval_role_arn
configuration_profile_application_id = configuration_profile.application_id
```

---


### Extension_association

ExtensionAssociation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Adds one or more tags for the specified extension association. Tags are metadata that
         help you categorize resources in different ways, for example, by purpose, owner, or
         environment. Each tag consists of a key and an optional value, both of which you define.
      </p> |
| `resource_identifier` | String | ✅ | <p>The ARN of an application, configuration profile, or environment.</p> |
| `parameters` | HashMap<String, String> |  | <p>The parameter names and values defined in the extensions. Extension parameters marked
            <code>Required</code> must be entered for this field.</p> |
| `extension_version_number` | i64 |  | <p>The version number of the extension. If not specified, AppConfig uses the
         maximum version of the extension.</p> |
| `extension_identifier` | String | ✅ | <p>The name, the ID, or the Amazon Resource Name (ARN) of the extension.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The system-generated Amazon Resource Name (ARN) for the extension.</p> |
| `id` | String | <p>The system-generated ID for the association.</p> |
| `parameters` | HashMap<String, String> | <p>The parameter names and values defined in the association.</p> |
| `extension_arn` | String | <p>The ARN of the extension defined in the association.</p> |
| `extension_version_number` | i64 | <p>The version number for the extension defined in the association.</p> |
| `resource_arn` | String | <p>The ARNs of applications, configuration profiles, or environments defined in the
         association.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create extension_association
extension_association = provider.appconfig.Extension_association {
    resource_identifier = "value"  # <p>The ARN of an application, configuration profile, or environment.</p>
    extension_identifier = "value"  # <p>The name, the ID, or the Amazon Resource Name (ARN) of the extension.</p>
}

# Access extension_association outputs
extension_association_id = extension_association.id
extension_association_arn = extension_association.arn
extension_association_id = extension_association.id
extension_association_parameters = extension_association.parameters
extension_association_extension_arn = extension_association.extension_arn
extension_association_extension_version_number = extension_association.extension_version_number
extension_association_resource_arn = extension_association.resource_arn
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the application.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata to assign to the application. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which
         you define.</p> |
| `name` | String | ✅ | <p>A name for the application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The application name.</p> |
| `description` | String | <p>The description of the application.</p> |
| `id` | String | <p>The application ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.appconfig.Application {
    name = "value"  # <p>A name for the application.</p>
}

# Access application outputs
application_id = application.id
application_name = application.name
application_description = application.description
application_id = application.id
```

---


### Environment

Environment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the environment.</p> |
| `name` | String | ✅ | <p>A name for the environment.</p> |
| `monitors` | Vec<String> |  | <p>Amazon CloudWatch alarms to monitor during the deployment process.</p> |
| `application_id` | String | ✅ | <p>The application ID.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata to assign to the environment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which
         you define.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | <p>The state of the environment. An environment can be in one of the following states:
            <code>READY_FOR_DEPLOYMENT</code>, <code>DEPLOYING</code>, <code>ROLLING_BACK</code>, or
            <code>ROLLED_BACK</code>
         </p> |
| `application_id` | String | <p>The application ID.</p> |
| `name` | String | <p>The name of the environment.</p> |
| `description` | String | <p>The description of the environment.</p> |
| `monitors` | Vec<String> | <p>Amazon CloudWatch alarms monitored during the deployment.</p> |
| `id` | String | <p>The environment ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment
environment = provider.appconfig.Environment {
    name = "value"  # <p>A name for the environment.</p>
    application_id = "value"  # <p>The application ID.</p>
}

# Access environment outputs
environment_id = environment.id
environment_state = environment.state
environment_application_id = environment.application_id
environment_name = environment.name
environment_description = environment.description
environment_monitors = environment.monitors
environment_id = environment.id
```

---


### Extension

Extension resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Adds one or more tags for the specified extension. Tags are metadata that help you
         categorize resources in different ways, for example, by purpose, owner, or environment.
         Each tag consists of a key and an optional value, both of which you define. </p> |
| `latest_version_number` | i64 |  | <p>You can omit this field when you create an extension. When you create a new version,
         specify the most recent current version number. For example, you create version 3, enter 2
         for this field.</p> |
| `name` | String | ✅ | <p>A name for the extension. Each extension name in your account must be unique. Extension
         versions use the same name.</p> |
| `actions` | HashMap<String, Vec<String>> | ✅ | <p>The actions defined in the extension.</p> |
| `parameters` | HashMap<String, String> |  | <p>The parameters accepted by the extension. You specify parameter values when you
         associate the extension to an AppConfig resource by using the
            <code>CreateExtensionAssociation</code> API action. For Lambda extension
         actions, these parameters are included in the Lambda request object.</p> |
| `description` | String |  | <p>Information about the extension.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The system-generated Amazon Resource Name (ARN) for the extension.</p> |
| `parameters` | HashMap<String, String> | <p>The parameters accepted by the extension. You specify parameter values when you
         associate the extension to an AppConfig resource by using the
            <code>CreateExtensionAssociation</code> API action. For Lambda extension
         actions, these parameters are included in the Lambda request object.</p> |
| `id` | String | <p>The system-generated ID of the extension.</p> |
| `description` | String | <p>Information about the extension.</p> |
| `name` | String | <p>The extension name.</p> |
| `actions` | HashMap<String, Vec<String>> | <p>The actions defined in the extension.</p> |
| `version_number` | i64 | <p>The extension version number.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create extension
extension = provider.appconfig.Extension {
    name = "value"  # <p>A name for the extension. Each extension name in your account must be unique. Extension
         versions use the same name.</p>
    actions = "value"  # <p>The actions defined in the extension.</p>
}

# Access extension outputs
extension_id = extension.id
extension_arn = extension.arn
extension_parameters = extension.parameters
extension_id = extension.id
extension_description = extension.description
extension_name = extension.name
extension_actions = extension.actions
extension_version_number = extension.version_number
```

---


### Account_settings

AccountSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deletion_protection` | String |  | <p>A parameter to configure deletion protection. Deletion protection prevents a user from
         deleting a configuration profile or an environment if AppConfig has called either
            <a href="https://docs.aws.amazon.com/appconfig/2019-10-09/APIReference/API_appconfigdata_GetLatestConfiguration.html">GetLatestConfiguration</a> or  for the
         configuration profile or from the environment during the specified interval. The default
         interval for <code>ProtectionPeriodInMinutes</code> is 60.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deletion_protection` | String | <p>A parameter to configure deletion protection. Deletion protection prevents a user from
         deleting a configuration profile or an environment if AppConfig has called either
            <a href="https://docs.aws.amazon.com/appconfig/2019-10-09/APIReference/API_appconfigdata_GetLatestConfiguration.html">GetLatestConfiguration</a> or  for the
         configuration profile or from the environment during the specified interval. The default
         interval for <code>ProtectionPeriodInMinutes</code> is 60.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_settings outputs
account_settings_id = account_settings.id
account_settings_deletion_protection = account_settings.deletion_protection
```

---


### Deployment

Deployment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployment_number` | i64 | <p>The sequence number of the deployment.</p> |
| `percentage_complete` | String | <p>The percentage of targets for which the deployment is available.</p> |
| `kms_key_arn` | String | <p>The Amazon Resource Name of the Key Management Service key used to encrypt configuration
         data. You can encrypt secrets stored in Secrets Manager, Amazon Simple Storage Service
         (Amazon S3) objects encrypted with SSE-KMS, or secure string parameters stored in Amazon Web Services Systems Manager
         Parameter Store. </p> |
| `started_at` | String | <p>The time the deployment started.</p> |
| `deployment_duration_in_minutes` | i64 | <p>Total amount of time the deployment lasted.</p> |
| `final_bake_time_in_minutes` | i64 | <p>The amount of time that AppConfig monitored for alarms before considering the
         deployment to be complete and no longer eligible for automatic rollback.</p> |
| `growth_type` | String | <p>The algorithm used to define how percentage grew over time.</p> |
| `state` | String | <p>The state of the deployment.</p> |
| `application_id` | String | <p>The ID of the application that was deployed.</p> |
| `completed_at` | String | <p>The time the deployment completed. </p> |
| `description` | String | <p>The description of the deployment.</p> |
| `deployment_strategy_id` | String | <p>The ID of the deployment strategy that was deployed.</p> |
| `event_log` | Vec<String> | <p>A list containing all events related to a deployment. The most recent events are
         displayed first.</p> |
| `configuration_name` | String | <p>The name of the configuration.</p> |
| `configuration_profile_id` | String | <p>The ID of the configuration profile that was deployed.</p> |
| `applied_extensions` | Vec<String> | <p>A list of extensions that were processed as part of the deployment. The extensions that
         were previously associated to the configuration profile, environment, or the application
         when <code>StartDeployment</code> was called.</p> |
| `kms_key_identifier` | String | <p>The Key Management Service key identifier (key ID, key alias, or key ARN) provided when
         the resource was created or updated.</p> |
| `environment_id` | String | <p>The ID of the environment that was deployed.</p> |
| `version_label` | String | <p>A user-defined label for an AppConfig hosted configuration version.</p> |
| `configuration_location_uri` | String | <p>Information about the source location of the configuration.</p> |
| `configuration_version` | String | <p>The configuration version that was deployed.</p> |
| `growth_factor` | String | <p>The percentage of targets to receive a deployed configuration during each
         interval.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deployment outputs
deployment_id = deployment.id
deployment_deployment_number = deployment.deployment_number
deployment_percentage_complete = deployment.percentage_complete
deployment_kms_key_arn = deployment.kms_key_arn
deployment_started_at = deployment.started_at
deployment_deployment_duration_in_minutes = deployment.deployment_duration_in_minutes
deployment_final_bake_time_in_minutes = deployment.final_bake_time_in_minutes
deployment_growth_type = deployment.growth_type
deployment_state = deployment.state
deployment_application_id = deployment.application_id
deployment_completed_at = deployment.completed_at
deployment_description = deployment.description
deployment_deployment_strategy_id = deployment.deployment_strategy_id
deployment_event_log = deployment.event_log
deployment_configuration_name = deployment.configuration_name
deployment_configuration_profile_id = deployment.configuration_profile_id
deployment_applied_extensions = deployment.applied_extensions
deployment_kms_key_identifier = deployment.kms_key_identifier
deployment_environment_id = deployment.environment_id
deployment_version_label = deployment.version_label
deployment_configuration_location_uri = deployment.configuration_location_uri
deployment_configuration_version = deployment.configuration_version
deployment_growth_factor = deployment.growth_factor
```

---


### Hosted_configuration_version

HostedConfigurationVersion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `latest_version_number` | i64 |  | <p>An optional locking token used to prevent race conditions from overwriting configuration
         updates when creating a new version. To ensure your data is not overwritten when creating
         multiple hosted configuration versions in rapid succession, specify the version number of
         the latest hosted configuration version.</p> |
| `content` | String | ✅ | <p>The configuration data, as bytes.</p>
         <note>
            <p>AppConfig accepts any type of data, including text formats like JSON or
            TOML, or binary formats like protocol buffers or compressed data.</p>
         </note> |
| `content_type` | String | ✅ | <p>A standard MIME type describing the format of the configuration content. For more
         information, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">Content-Type</a>.</p> |
| `version_label` | String |  | <p>An optional, user-defined label for the AppConfig hosted configuration
         version. This value must contain at least one non-numeric character. For example,
         "v2.2.0".</p> |
| `configuration_profile_id` | String | ✅ | <p>The configuration profile ID.</p> |
| `description` | String |  | <p>A description of the configuration.</p> |
| `application_id` | String | ✅ | <p>The application ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_profile_id` | String | <p>The configuration profile ID.</p> |
| `version_label` | String | <p>A user-defined label for an AppConfig hosted configuration version.</p> |
| `content_type` | String | <p>A standard MIME type describing the format of the configuration content. For more
         information, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">Content-Type</a>.</p> |
| `kms_key_arn` | String | <p>The Amazon Resource Name of the Key Management Service key that was used to encrypt this
         specific version of the configuration data in the AppConfig hosted configuration
         store.</p> |
| `content` | String | <p>The content of the configuration or the configuration data.</p> |
| `version_number` | i64 | <p>The configuration version.</p> |
| `description` | String | <p>A description of the configuration.</p> |
| `application_id` | String | <p>The application ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hosted_configuration_version
hosted_configuration_version = provider.appconfig.Hosted_configuration_version {
    content = "value"  # <p>The configuration data, as bytes.</p>
         <note>
            <p>AppConfig accepts any type of data, including text formats like JSON or
            TOML, or binary formats like protocol buffers or compressed data.</p>
         </note>
    content_type = "value"  # <p>A standard MIME type describing the format of the configuration content. For more
         information, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">Content-Type</a>.</p>
    configuration_profile_id = "value"  # <p>The configuration profile ID.</p>
    application_id = "value"  # <p>The application ID.</p>
}

# Access hosted_configuration_version outputs
hosted_configuration_version_id = hosted_configuration_version.id
hosted_configuration_version_configuration_profile_id = hosted_configuration_version.configuration_profile_id
hosted_configuration_version_version_label = hosted_configuration_version.version_label
hosted_configuration_version_content_type = hosted_configuration_version.content_type
hosted_configuration_version_kms_key_arn = hosted_configuration_version.kms_key_arn
hosted_configuration_version_content = hosted_configuration_version.content
hosted_configuration_version_version_number = hosted_configuration_version.version_number
hosted_configuration_version_description = hosted_configuration_version.description
hosted_configuration_version_application_id = hosted_configuration_version.application_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple configuration resources
configuration_0 = provider.appconfig.Configuration {
}
configuration_1 = provider.appconfig.Configuration {
}
configuration_2 = provider.appconfig.Configuration {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    configuration = provider.appconfig.Configuration {
    }
```

---

## Related Documentation

- [AWS Appconfig Documentation](https://docs.aws.amazon.com/appconfig/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
