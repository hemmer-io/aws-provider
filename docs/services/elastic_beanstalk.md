# Elastic_beanstalk Service



**Resources**: 21

---

## Overview

The elastic_beanstalk service provides access to 21 resource types:

- [Environment](#environment) [CU]
- [Environment_health](#environment_health) [R]
- [Environment_managed_actions](#environment_managed_actions) [R]
- [Instances_health](#instances_health) [R]
- [Environments](#environments) [R]
- [Application_resource_lifecycle](#application_resource_lifecycle) [U]
- [Configuration_settings](#configuration_settings) [R]
- [Application](#application) [CUD]
- [Environment_managed_action_history](#environment_managed_action_history) [R]
- [Environment_resources](#environment_resources) [R]
- [Environment_configuration](#environment_configuration) [D]
- [Events](#events) [R]
- [Applications](#applications) [R]
- [Configuration_options](#configuration_options) [R]
- [Application_version](#application_version) [CUD]
- [Account_attributes](#account_attributes) [R]
- [Storage_location](#storage_location) [C]
- [Tags_for_resource](#tags_for_resource) [U]
- [Platform_version](#platform_version) [CRD]
- [Application_versions](#application_versions) [R]
- [Configuration_template](#configuration_template) [CUD]

---

## Resources


### Environment

Environment resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_name` | String |  | <p>The name of the Elastic Beanstalk configuration template to use with the environment.</p>
         <note>
            <p>If you specify <code>TemplateName</code>, then don't specify 
          <code>SolutionStackName</code>.</p>
         </note> |
| `option_settings` | Vec<String> |  | <p>If specified, AWS Elastic Beanstalk sets the specified configuration options to the
      requested value in the configuration set for the new environment. These override the values
      obtained from the solution stack or the configuration template.</p> |
| `tags` | Vec<String> |  | <p>Specifies the tags applied to resources in the environment.</p> |
| `solution_stack_name` | String |  | <p>The name of an Elastic Beanstalk solution stack (platform version) to use with the environment. If
      specified, Elastic Beanstalk sets the configuration values to the default values associated with the
      specified solution stack. For a list of current solution stacks, see <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/platforms/platforms-supported.html">Elastic Beanstalk Supported Platforms</a> in the <i>AWS Elastic Beanstalk
        Platforms</i> guide.</p>
         <note>
            <p>If you specify <code>SolutionStackName</code>, don't specify <code>PlatformArn</code> or
          <code>TemplateName</code>.</p>
         </note> |
| `group_name` | String |  | <p>The name of the group to which the target environment belongs. Specify a group name
      only if the environment's name is specified in an environment manifest and not with the
      environment name parameter. See <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-cfg-manifest.html">Environment Manifest
        (env.yaml)</a> for details.</p> |
| `description` | String |  | <p>Your description for this environment.</p> |
| `cname_prefix` | String |  | <p>If specified, the environment attempts to use this value as the prefix for the CNAME in
      your Elastic Beanstalk environment URL. If not specified, the CNAME is generated automatically by
      appending a random alphanumeric string to the environment name.</p> |
| `environment_name` | String |  | <p>A unique name for the environment.</p>
         <p>Constraint: Must be from 4 to 40 characters in length. The name can contain only
      letters, numbers, and hyphens. It can't start or end with a hyphen. This name must be unique
      within a region in your account. If the specified name already exists in the region, Elastic Beanstalk returns an
        <code>InvalidParameterValue</code> error. </p>
         <p>If you don't specify the <code>CNAMEPrefix</code> parameter, the environment name becomes part of
      the CNAME, and therefore part of the visible URL for your application.</p> |
| `version_label` | String |  | <p>The name of the application version to deploy.</p>
         <p>Default: If not specified, Elastic Beanstalk attempts to deploy the sample application.</p> |
| `operations_role` | String |  | <p>The Amazon Resource Name (ARN) of an existing IAM role to be used as the environment's
      operations role. If specified, Elastic Beanstalk uses the operations role for permissions to downstream
      services during this call and during subsequent calls acting on this environment. To specify
      an operations role, you must have the <code>iam:PassRole</code> permission for the role. For
      more information, see <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/iam-operationsrole.html">Operations roles</a> in the
        <i>AWS Elastic Beanstalk Developer Guide</i>.</p> |
| `application_name` | String | ✅ | <p>The name of the application that is associated with this environment.</p> |
| `tier` | String |  | <p>Specifies the tier to use in creating this environment. The environment tier that you
      choose determines whether Elastic Beanstalk provisions resources to support a web application that handles
      HTTP(S) requests or a web application that handles background-processing tasks.</p> |
| `options_to_remove` | Vec<String> |  | <p>A list of custom user-defined configuration options to remove from the configuration
      set for this new environment.</p> |
| `platform_arn` | String |  | <p>The Amazon Resource Name (ARN) of the custom platform to use with the environment. For
      more information, see <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/custom-platforms.html">Custom Platforms</a> in the
        <i>AWS Elastic Beanstalk Developer Guide</i>.</p>
         <note>
            <p>If you specify <code>PlatformArn</code>, don't specify
        <code>SolutionStackName</code>.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment
environment = provider.elastic_beanstalk.Environment {
    application_name = "value"  # <p>The name of the application that is associated with this environment.</p>
}

```

---


### Environment_health

EnvironmentHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `causes` | Vec<String> | <p>Descriptions of the data that contributed to the environment's current health
      status.</p> |
| `instances_health` | String | <p>Summary health information for the instances in the environment.</p> |
| `health_status` | String | <p>The <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">health status</a> of the
      environment. For example, <code>Ok</code>.</p> |
| `status` | String | <p>The environment's operational status. <code>Ready</code>, <code>Launching</code>,
        <code>Updating</code>, <code>Terminating</code>, or <code>Terminated</code>.</p> |
| `color` | String | <p>The <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">health color</a> of the
      environment.</p> |
| `application_metrics` | String | <p>Application request metrics for the environment.</p> |
| `environment_name` | String | <p>The environment's name.</p> |
| `refreshed_at` | String | <p>The date and time that the health information was retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environment_health outputs
environment_health_id = environment_health.id
environment_health_causes = environment_health.causes
environment_health_instances_health = environment_health.instances_health
environment_health_health_status = environment_health.health_status
environment_health_status = environment_health.status
environment_health_color = environment_health.color
environment_health_application_metrics = environment_health.application_metrics
environment_health_environment_name = environment_health.environment_name
environment_health_refreshed_at = environment_health.refreshed_at
```

---


### Environment_managed_actions

EnvironmentManagedActions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_actions` | Vec<String> | <p>A list of upcoming and in-progress managed actions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environment_managed_actions outputs
environment_managed_actions_id = environment_managed_actions.id
environment_managed_actions_managed_actions = environment_managed_actions.managed_actions
```

---


### Instances_health

InstancesHealth resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `refreshed_at` | String | <p>The date and time that the health information was retrieved.</p> |
| `next_token` | String | <p>Pagination token for the next page of results, if available.</p> |
| `instance_health_list` | Vec<String> | <p>Detailed health information about each instance.</p>
         <p>The output differs slightly between Linux and Windows environments. There is a difference
      in the members that are supported under the <code><CPUUtilization></code> type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instances_health outputs
instances_health_id = instances_health.id
instances_health_refreshed_at = instances_health.refreshed_at
instances_health_next_token = instances_health.next_token
instances_health_instance_health_list = instances_health.instance_health_list
```

---


### Environments

Environments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>In a paginated request, the token that you can pass in a subsequent request to get the
      next response page.</p> |
| `environments` | Vec<String> | <p> Returns an <a>EnvironmentDescription</a> list. </p> |


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
environments_next_token = environments.next_token
environments_environments = environments.environments
```

---


### Application_resource_lifecycle

ApplicationResourceLifecycle resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_name` | String | ✅ | <p>The name of the application.</p> |
| `resource_lifecycle_config` | String | ✅ | <p>The lifecycle configuration.</p> |



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


### Configuration_settings

ConfigurationSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_settings` | Vec<String> | <p> A list of <a>ConfigurationSettingsDescription</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_settings outputs
configuration_settings_id = configuration_settings.id
configuration_settings_configuration_settings = configuration_settings.configuration_settings
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>Your description of the application.</p> |
| `resource_lifecycle_config` | String |  | <p>Specifies an application resource lifecycle configuration to prevent your application
      from accumulating too many versions.</p> |
| `application_name` | String | ✅ | <p>The name of the application. Must be unique within your account.</p> |
| `tags` | Vec<String> |  | <p>Specifies the tags applied to the application.</p>
         <p>Elastic Beanstalk applies these tags only to the application. Environments that you create in the
      application don't inherit the tags.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.elastic_beanstalk.Application {
    application_name = "value"  # <p>The name of the application. Must be unique within your account.</p>
}

```

---


### Environment_managed_action_history

EnvironmentManagedActionHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_action_history_items` | Vec<String> | <p>A list of completed and failed managed actions.</p> |
| `next_token` | String | <p>A pagination token that you pass to <a>DescribeEnvironmentManagedActionHistory</a> to get the next page of
      results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environment_managed_action_history outputs
environment_managed_action_history_id = environment_managed_action_history.id
environment_managed_action_history_managed_action_history_items = environment_managed_action_history.managed_action_history_items
environment_managed_action_history_next_token = environment_managed_action_history.next_token
```

---


### Environment_resources

EnvironmentResources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment_resources` | String | <p> A list of <a>EnvironmentResourceDescription</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environment_resources outputs
environment_resources_id = environment_resources.id
environment_resources_environment_resources = environment_resources.environment_resources
```

---


### Environment_configuration

EnvironmentConfiguration resource

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


### Events

Events resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p> If returned, this indicates that there are more results to obtain. Use this token in
      the next <a>DescribeEvents</a> call to get the next batch of events. </p> |
| `events` | Vec<String> | <p> A list of <a>EventDescription</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access events outputs
events_id = events.id
events_next_token = events.next_token
events_events = events.events
```

---


### Applications

Applications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `applications` | Vec<String> | <p>This parameter contains a list of <a>ApplicationDescription</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access applications outputs
applications_id = applications.id
applications_applications = applications.applications
```

---


### Configuration_options

ConfigurationOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `solution_stack_name` | String | <p>The name of the solution stack these configuration options belong to.</p> |
| `platform_arn` | String | <p>The ARN of the platform version.</p> |
| `options` | Vec<String> | <p> A list of <a>ConfigurationOptionDescription</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_options outputs
configuration_options_id = configuration_options.id
configuration_options_solution_stack_name = configuration_options.solution_stack_name
configuration_options_platform_arn = configuration_options.platform_arn
configuration_options_options = configuration_options.options
```

---


### Application_version

ApplicationVersion resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of this application version.</p> |
| `tags` | Vec<String> |  | <p>Specifies the tags applied to the application version.</p>
         <p>Elastic Beanstalk applies these tags only to the application version. Environments that use the
      application version don't inherit the tags.</p> |
| `process` | bool |  | <p>Pre-processes and validates the environment manifest (<code>env.yaml</code>) and
      configuration files (<code>*.config</code> files in the <code>.ebextensions</code> folder) in
      the source bundle. Validating configuration files can identify issues prior to deploying the
      application version to an environment.</p>
         <p>You must turn processing on for application versions that you create using AWS
      CodeBuild or AWS CodeCommit. For application versions built from a source bundle in Amazon S3,
      processing is optional.</p>
         <note>
            <p>The <code>Process</code> option validates Elastic Beanstalk configuration files. It
      doesn't validate your application's configuration files, like proxy server or Docker
      configuration.</p>
         </note> |
| `auto_create_application` | bool |  | <p>Set to <code>true</code> to create an application with the specified name if it doesn't
      already exist.</p> |
| `version_label` | String | ✅ | <p>A label identifying this version.</p>
         <p>Constraint: Must be unique per application. If an application version already exists
      with this label for the specified application, AWS Elastic Beanstalk returns an
        <code>InvalidParameterValue</code> error. </p> |
| `build_configuration` | String |  | <p>Settings for an AWS CodeBuild build.</p> |
| `source_bundle` | String |  | <p>The Amazon S3 bucket and key that identify the location of the source bundle for this
      version.</p>
         <note>
            <p>The Amazon S3 bucket must be in the same region as the
      environment.</p>
         </note>
         <p>Specify a source bundle in S3 or a commit in an AWS CodeCommit repository (with
        <code>SourceBuildInformation</code>), but not both. If neither <code>SourceBundle</code> nor
        <code>SourceBuildInformation</code> are provided, Elastic Beanstalk uses a sample
      application.</p> |
| `application_name` | String | ✅ | <p> The name of the application. If no application is found with this name, and
        <code>AutoCreateApplication</code> is <code>false</code>, returns an
        <code>InvalidParameterValue</code> error. </p> |
| `source_build_information` | String |  | <p>Specify a commit in an AWS CodeCommit Git repository to use as the source code for the
      application version.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_version
application_version = provider.elastic_beanstalk.Application_version {
    version_label = "value"  # <p>A label identifying this version.</p>
         <p>Constraint: Must be unique per application. If an application version already exists
      with this label for the specified application, AWS Elastic Beanstalk returns an
        <code>InvalidParameterValue</code> error. </p>
    application_name = "value"  # <p> The name of the application. If no application is found with this name, and
        <code>AutoCreateApplication</code> is <code>false</code>, returns an
        <code>InvalidParameterValue</code> error. </p>
}

```

---


### Account_attributes

AccountAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_quotas` | String | <p>The Elastic Beanstalk resource quotas associated with the calling AWS account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_attributes outputs
account_attributes_id = account_attributes.id
account_attributes_resource_quotas = account_attributes.resource_quotas
```

---


### Storage_location

StorageLocation resource

**Operations**: ✅ Create

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

# Create storage_location
storage_location = provider.elastic_beanstalk.Storage_location {
}

```

---


### Tags_for_resource

TagsForResource resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags_to_remove` | Vec<String> |  | <p>A list of tag keys to remove. If a tag key doesn't exist, it is silently ignored.</p>
         <p>Specify at least one of these parameters: <code>TagsToAdd</code>,
      <code>TagsToRemove</code>.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resouce to be updated.</p>
         <p>Must be the ARN of an Elastic Beanstalk resource.</p> |
| `tags_to_add` | Vec<String> |  | <p>A list of tags to add or update. If a key of an existing tag is added, the tag's value is
      updated.</p>
         <p>Specify at least one of these parameters: <code>TagsToAdd</code>,
        <code>TagsToRemove</code>.</p> |



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


### Platform_version

PlatformVersion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Specifies the tags applied to the new platform version.</p>
         <p>Elastic Beanstalk applies these tags only to the platform version. Environments that you create using
      the platform version don't inherit the tags.</p> |
| `platform_version` | String | ✅ | <p>The number, such as 1.0.2, for the new platform version.</p> |
| `platform_definition_bundle` | String | ✅ | <p>The location of the platform definition archive in Amazon S3.</p> |
| `environment_name` | String |  | <p>The name of the builder environment.</p> |
| `platform_name` | String | ✅ | <p>The name of your custom platform.</p> |
| `option_settings` | Vec<String> |  | <p>The configuration option settings to apply to the builder environment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `platform_description` | String | <p>Detailed information about the platform version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create platform_version
platform_version = provider.elastic_beanstalk.Platform_version {
    platform_version = "value"  # <p>The number, such as 1.0.2, for the new platform version.</p>
    platform_definition_bundle = "value"  # <p>The location of the platform definition archive in Amazon S3.</p>
    platform_name = "value"  # <p>The name of your custom platform.</p>
}

# Access platform_version outputs
platform_version_id = platform_version.id
platform_version_platform_description = platform_version.platform_description
```

---


### Application_versions

ApplicationVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_versions` | Vec<String> | <p>List of <code>ApplicationVersionDescription</code> objects sorted in order of
      creation.</p> |
| `next_token` | String | <p>In a paginated request, the token that you can pass in a subsequent request to get the
      next response page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_versions outputs
application_versions_id = application_versions.id
application_versions_application_versions = application_versions.application_versions
application_versions_next_token = application_versions.next_token
```

---


### Configuration_template

ConfigurationTemplate resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_name` | String | ✅ | <p>The name of the Elastic Beanstalk application to associate with this configuration
      template.</p> |
| `template_name` | String | ✅ | <p>The name of the configuration template.</p>
         <p>Constraint: This name must be unique per application.</p> |
| `platform_arn` | String |  | <p>The Amazon Resource Name (ARN) of the custom platform. For more information, see <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/custom-platforms.html"> Custom
        Platforms</a> in the <i>AWS Elastic Beanstalk Developer Guide</i>.</p>
         <note>
            <p>If you specify <code>PlatformArn</code>, then don't specify
          <code>SolutionStackName</code>.</p>
         </note> |
| `source_configuration` | String |  | <p>An Elastic Beanstalk configuration template to base this one on. If specified, Elastic Beanstalk uses the configuration values from the specified
      configuration template to create a new configuration.</p>
         <p>Values specified in <code>OptionSettings</code> override any values obtained from the
        <code>SourceConfiguration</code>.</p>
         <p>You must specify <code>SourceConfiguration</code> if you don't specify
        <code>PlatformArn</code>, <code>EnvironmentId</code>, or
      <code>SolutionStackName</code>.</p>
         <p>Constraint: If both solution stack name and source configuration are specified, the
      solution stack of the source configuration template must match the specified solution stack
      name.</p> |
| `solution_stack_name` | String |  | <p>The name of an Elastic Beanstalk solution stack (platform version) that this configuration uses. For
      example, <code>64bit Amazon Linux 2013.09 running Tomcat 7 Java 7</code>. A solution stack
      specifies the operating system, runtime, and application server for a configuration template.
      It also determines the set of configuration options as well as the possible and default
      values. For more information, see <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/concepts.platforms.html">Supported Platforms</a> in the
        <i>AWS Elastic Beanstalk Developer Guide</i>.</p>
         <p>You must specify <code>SolutionStackName</code> if you don't specify
        <code>PlatformArn</code>, <code>EnvironmentId</code>, or
      <code>SourceConfiguration</code>.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/api/API_ListAvailableSolutionStacks.html">
               <code>ListAvailableSolutionStacks</code>
            </a> API to obtain a list of available
      solution stacks.</p> |
| `tags` | Vec<String> |  | <p>Specifies the tags applied to the configuration template.</p> |
| `environment_id` | String |  | <p>The ID of an environment whose settings you want to use to create the configuration
      template. You must specify <code>EnvironmentId</code> if you don't specify
        <code>PlatformArn</code>, <code>SolutionStackName</code>, or
        <code>SourceConfiguration</code>.</p> |
| `description` | String |  | <p>An optional description for this configuration.</p> |
| `option_settings` | Vec<String> |  | <p>Option values for the Elastic Beanstalk configuration, such as the instance type. If specified, these
      values override the values obtained from the solution stack or the source configuration
      template. For a complete list of Elastic Beanstalk configuration options, see <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/command-options.html">Option Values</a> in the
        <i>AWS Elastic Beanstalk Developer Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_template
configuration_template = provider.elastic_beanstalk.Configuration_template {
    application_name = "value"  # <p>The name of the Elastic Beanstalk application to associate with this configuration
      template.</p>
    template_name = "value"  # <p>The name of the configuration template.</p>
         <p>Constraint: This name must be unique per application.</p>
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

# Create multiple environment resources
environment_0 = provider.elastic_beanstalk.Environment {
    application_name = "value-0"
}
environment_1 = provider.elastic_beanstalk.Environment {
    application_name = "value-1"
}
environment_2 = provider.elastic_beanstalk.Environment {
    application_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    environment = provider.elastic_beanstalk.Environment {
        application_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Elastic_beanstalk Documentation](https://docs.aws.amazon.com/elastic_beanstalk/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
