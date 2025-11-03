# Codedeploy Service



**Resources**: 11

---

## Overview

The codedeploy service provides access to 11 resource types:

- [Application_revision](#application_revision) [R]
- [Deployment_config](#deployment_config) [CRD]
- [Application](#application) [CRUD]
- [Deployment_target](#deployment_target) [R]
- [On_premises_instance](#on_premises_instance) [R]
- [Resources_by_external_id](#resources_by_external_id) [D]
- [Lifecycle_event_hook_execution_status](#lifecycle_event_hook_execution_status) [C]
- [Deployment_group](#deployment_group) [CRUD]
- [Git_hub_account_token](#git_hub_account_token) [D]
- [Deployment_instance](#deployment_instance) [R]
- [Deployment](#deployment) [CR]

---

## Resources


### Application_revision

ApplicationRevision resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `revision_info` | String | <p>General information about the revision.</p> |
| `application_name` | String | <p>The name of the application that corresponds to the revision.</p> |
| `revision` | String | <p>Additional information about the revision, including type and location.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_revision outputs
application_revision_id = application_revision.id
application_revision_revision_info = application_revision.revision_info
application_revision_application_name = application_revision.application_name
application_revision_revision = application_revision.revision
```

---


### Deployment_config

DeploymentConfig resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `traffic_routing_config` | String |  | <p>The configuration that specifies how the deployment traffic is routed.</p> |
| `minimum_healthy_hosts` | String |  | <p>The minimum number of healthy instances that should be available at any time during
            the deployment. There are two parameters expected in the input: type and value.</p>
         <p>The type parameter takes either of the following values:</p>
         <ul>
            <li>
               <p>HOST_COUNT: The value parameter represents the minimum number of healthy
                    instances as an absolute value.</p>
            </li>
            <li>
               <p>FLEET_PERCENT: The value parameter represents the minimum number of healthy
                    instances as a percentage of the total number of instances in the deployment. If
                    you specify FLEET_PERCENT, at the start of the deployment, CodeDeploy converts the percentage to the equivalent number of instances and rounds up
                    fractional instances.</p>
            </li>
         </ul>
         <p>The value parameter takes an integer.</p>
         <p>For example, to set a minimum of 95% healthy instance, specify a type of FLEET_PERCENT
            and a value of 95.</p> |
| `compute_platform` | String |  | <p>The destination platform type for the deployment (<code>Lambda</code>,
                <code>Server</code>, or <code>ECS</code>).</p> |
| `zonal_config` | String |  | <p>Configure the <code>ZonalConfig</code> object if you want CodeDeploy to
            deploy your application to one <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-availability-zones">Availability Zone</a> at a time, within an Amazon Web Services Region.</p>
         <p>For more information about the zonal configuration feature, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/deployment-configurations-create.html#zonal-config">zonal configuration</a> in the <i>CodeDeploy User
                Guide</i>.</p> |
| `deployment_config_name` | String | ✅ | <p>The name of the deployment configuration to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployment_config_info` | String | <p>Information about the deployment configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment_config
deployment_config = provider.codedeploy.Deployment_config {
    deployment_config_name = "value"  # <p>The name of the deployment configuration to create.</p>
}

# Access deployment_config outputs
deployment_config_id = deployment_config.id
deployment_config_deployment_config_info = deployment_config.deployment_config_info
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p> The metadata that you apply to CodeDeploy applications to help you organize and
            categorize them. Each tag consists of a key and an optional value, both of which you
            define. </p> |
| `application_name` | String | ✅ | <p>The name of the application. This name must be unique with the applicable user or
                Amazon Web Services account.</p> |
| `compute_platform` | String |  | <p> The destination platform type for the deployment (<code>Lambda</code>,
                <code>Server</code>, or <code>ECS</code>).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application` | String | <p>Information about the application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.codedeploy.Application {
    application_name = "value"  # <p>The name of the application. This name must be unique with the applicable user or
                Amazon Web Services account.</p>
}

# Access application outputs
application_id = application.id
application_application = application.application
```

---


### Deployment_target

DeploymentTarget resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployment_target` | String | <p> A deployment target that contains information about a deployment such as its status,
            lifecycle events, and when it was last updated. It also contains metadata about the
            deployment target. The deployment target metadata depends on the deployment target's
            type (<code>instanceTarget</code>, <code>lambdaTarget</code>, or
            <code>ecsTarget</code>). </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deployment_target outputs
deployment_target_id = deployment_target.id
deployment_target_deployment_target = deployment_target.deployment_target
```

---


### On_premises_instance

OnPremisesInstance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_info` | String | <p> Information about the on-premises instance. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access on_premises_instance outputs
on_premises_instance_id = on_premises_instance.id
on_premises_instance_instance_info = on_premises_instance.instance_info
```

---


### Resources_by_external_id

ResourcesByExternalId resource

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


### Lifecycle_event_hook_execution_status

LifecycleEventHookExecutionStatus resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_id` | String |  | <p> The unique ID of a deployment. Pass this ID to a Lambda function that
            validates a deployment lifecycle event. </p> |
| `status` | String |  | <p>The result of a Lambda function that validates a deployment lifecycle
            event. The values listed in <b>Valid Values</b> are valid for
            lifecycle statuses in general; however, only <code>Succeeded</code> and
                <code>Failed</code> can be passed successfully in your API call.</p> |
| `lifecycle_event_hook_execution_id` | String |  | <p> The execution ID of a deployment's lifecycle hook. A deployment lifecycle hook is
            specified in the <code>hooks</code> section of the AppSpec file. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_event_hook_execution_status
lifecycle_event_hook_execution_status = provider.codedeploy.Lifecycle_event_hook_execution_status {
}

```

---


### Deployment_group

DeploymentGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trigger_configurations` | Vec<String> |  | <p>Information about triggers to create when the deployment group is created. For
            examples, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/how-to-notify-sns.html">Create a Trigger for an
                    CodeDeploy Event</a> in the <i>CodeDeploy
                User Guide</i>.</p> |
| `alarm_configuration` | String |  | <p>Information to add about Amazon CloudWatch alarms when the deployment group is
            created.</p> |
| `load_balancer_info` | String |  | <p>Information about the load balancer used in a deployment.</p> |
| `ec2_tag_set` | String |  | <p>Information about groups of tags applied to Amazon EC2 instances. The
            deployment group includes only Amazon EC2 instances identified by all the tag
            groups. Cannot be used in the same call as <code>ec2TagFilters</code>.</p> |
| `blue_green_deployment_configuration` | String |  | <p>Information about blue/green deployment options for a deployment group.</p> |
| `on_premises_instance_tag_filters` | Vec<String> |  | <p>The on-premises instance tags on which to filter. The deployment group includes
            on-premises instances with any of the specified tags. Cannot be used in the same call as
                <code>OnPremisesTagSet</code>.</p> |
| `tags` | Vec<String> |  | <p> The metadata that you apply to CodeDeploy deployment groups to help you organize and
            categorize them. Each tag consists of a key and an optional value, both of which you
            define. </p> |
| `auto_scaling_groups` | Vec<String> |  | <p>A list of associated Amazon EC2 Auto Scaling groups.</p> |
| `termination_hook_enabled` | bool |  | <p>This parameter only applies if you are using CodeDeploy with Amazon EC2 Auto Scaling. For more information, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/integrations-aws-auto-scaling.html">Integrating
                    CodeDeploy with Amazon EC2 Auto Scaling</a> in the <i>CodeDeploy User Guide</i>.</p>
         <p>Set <code>terminationHookEnabled</code> to <code>true</code> to have CodeDeploy install a termination hook into your Auto Scaling group when you create a
            deployment group. When this hook is installed, CodeDeploy will perform
            termination deployments.</p>
         <p>For information about termination deployments, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/integrations-aws-auto-scaling.html#integrations-aws-auto-scaling-behaviors-hook-enable">Enabling termination deployments during Auto Scaling scale-in events</a> in the
                    <i>CodeDeploy User Guide</i>.</p>
         <p>For more information about Auto Scaling scale-in events, see the <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-lifecycle.html#as-lifecycle-scale-in">Scale in</a> topic in the <i>Amazon EC2 Auto Scaling User
            Guide</i>.</p> |
| `application_name` | String | ✅ | <p>The name of an CodeDeploy application associated with the user or Amazon Web Services account.</p> |
| `deployment_style` | String |  | <p>Information about the type of deployment, in-place or blue/green, that you want to run
            and whether to route deployment traffic behind a load balancer.</p> |
| `ec2_tag_filters` | Vec<String> |  | <p>The Amazon EC2 tags on which to filter. The deployment group includes Amazon EC2 instances with any of the specified tags. Cannot be used in the same call
            as ec2TagSet.</p> |
| `on_premises_tag_set` | String |  | <p>Information about groups of tags applied to on-premises instances. The deployment
            group includes only on-premises instances identified by all of the tag groups. Cannot be
            used in the same call as <code>onPremisesInstanceTagFilters</code>.</p> |
| `deployment_group_name` | String | ✅ | <p>The name of a new deployment group for the specified application.</p> |
| `deployment_config_name` | String |  | <p>If specified, the deployment configuration name can be either one of the predefined
            configurations provided with CodeDeploy or a custom deployment configuration
            that you create by calling the create deployment configuration operation.</p>
         <p>
            <code>CodeDeployDefault.OneAtATime</code> is the default deployment configuration. It
            is used if a configuration isn't specified for the deployment or deployment
            group.</p>
         <p>For more information about the predefined deployment configurations in CodeDeploy, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/deployment-configurations.html">Working with
                Deployment Configurations in CodeDeploy</a> in the <i>CodeDeploy User Guide</i>.</p> |
| `service_role_arn` | String | ✅ | <p>A service role Amazon Resource Name (ARN) that allows CodeDeploy to act on
            the user's behalf when interacting with Amazon Web Services services.</p> |
| `ecs_services` | Vec<String> |  | <p> The target Amazon ECS services in the deployment group. This applies only to
            deployment groups that use the Amazon ECS compute platform. A target Amazon ECS service is specified as an Amazon ECS cluster and service name
            pair using the format <code><clustername>:<servicename></code>. </p> |
| `outdated_instances_strategy` | String |  | <p>Indicates what happens when new Amazon EC2 instances are launched
            mid-deployment and do not receive the deployed application revision.</p>
         <p>If this option is set to <code>UPDATE</code> or is unspecified, CodeDeploy initiates
            one or more 'auto-update outdated instances' deployments to apply the deployed
            application revision to the new Amazon EC2 instances.</p>
         <p>If this option is set to <code>IGNORE</code>, CodeDeploy does not initiate a
            deployment to update the new Amazon EC2 instances. This may result in instances
            having different revisions.</p> |
| `auto_rollback_configuration` | String |  | <p>Configuration information for an automatic rollback that is added when a deployment
            group is created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployment_group_info` | String | <p>Information about the deployment group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment_group
deployment_group = provider.codedeploy.Deployment_group {
    application_name = "value"  # <p>The name of an CodeDeploy application associated with the user or Amazon Web Services account.</p>
    deployment_group_name = "value"  # <p>The name of a new deployment group for the specified application.</p>
    service_role_arn = "value"  # <p>A service role Amazon Resource Name (ARN) that allows CodeDeploy to act on
            the user's behalf when interacting with Amazon Web Services services.</p>
}

# Access deployment_group outputs
deployment_group_id = deployment_group.id
deployment_group_deployment_group_info = deployment_group.deployment_group_info
```

---


### Git_hub_account_token

GitHubAccountToken resource

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


### Deployment_instance

DeploymentInstance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_summary` | String | <p> Information about the instance. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deployment_instance outputs
deployment_instance_id = deployment_instance.id
deployment_instance_instance_summary = deployment_instance.instance_summary
```

---


### Deployment

Deployment resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_instances` | String |  | <p> Information about the instances that belong to the replacement environment in a
            blue/green deployment. </p> |
| `description` | String |  | <p>A comment about the deployment.</p> |
| `update_outdated_instances_only` | bool |  | <p> Indicates whether to deploy to all instances or only to instances that are not
            running the latest application revision. </p> |
| `override_alarm_configuration` | String |  | <p>Allows you to specify information about alarms associated with a deployment. The alarm
            configuration that you specify here will override the alarm configuration at the
            deployment group level. Consider overriding the alarm configuration if you have set up
            alarms at the deployment group level that are causing deployment failures. In this case,
            you would call <code>CreateDeployment</code> to create a new deployment that uses a
            previous application revision that is known to work, and set its alarm configuration to
            turn off alarm polling. Turning off alarm polling ensures that the new deployment
            proceeds without being blocked by the alarm that was generated by the previous, failed,
            deployment.</p>
         <note>
            <p>If you specify an <code>overrideAlarmConfiguration</code>, you need the
                    <code>UpdateDeploymentGroup</code>
                IAM permission when calling <code>CreateDeployment</code>.</p>
         </note> |
| `ignore_application_stop_failures` | bool |  | <p> If true, then if an <code>ApplicationStop</code>, <code>BeforeBlockTraffic</code>, or
                <code>AfterBlockTraffic</code> deployment lifecycle event to an instance fails, then
            the deployment continues to the next deployment lifecycle event. For example, if
                <code>ApplicationStop</code> fails, the deployment continues with
                <code>DownloadBundle</code>. If <code>BeforeBlockTraffic</code> fails, the
            deployment continues with <code>BlockTraffic</code>. If <code>AfterBlockTraffic</code>
            fails, the deployment continues with <code>ApplicationStop</code>. </p>
         <p> If false or not specified, then if a lifecycle event fails during a deployment to an
            instance, that deployment fails. If deployment to that instance is part of an overall
            deployment and the number of healthy hosts is not less than the minimum number of
            healthy hosts, then a deployment to the next instance is attempted. </p>
         <p> During a deployment, the CodeDeploy agent runs the scripts specified for
                <code>ApplicationStop</code>, <code>BeforeBlockTraffic</code>, and
                <code>AfterBlockTraffic</code> in the AppSpec file from the previous successful
            deployment. (All other scripts are run from the AppSpec file in the current deployment.)
            If one of these scripts contains an error and does not run successfully, the deployment
            can fail. </p>
         <p> If the cause of the failure is a script from the last successful deployment that will
            never run successfully, create a new deployment and use
                <code>ignoreApplicationStopFailures</code> to specify that the
                <code>ApplicationStop</code>, <code>BeforeBlockTraffic</code>, and
                <code>AfterBlockTraffic</code> failures should be ignored. </p> |
| `auto_rollback_configuration` | String |  | <p>Configuration information for an automatic rollback that is added when a deployment is
            created.</p> |
| `application_name` | String | ✅ | <p>The name of an CodeDeploy application associated with the user or Amazon Web Services account.</p> |
| `deployment_config_name` | String |  | <p>The name of a deployment configuration associated with the user or Amazon Web Services account.</p>
         <p>If not specified, the value configured in the deployment group is used as the default.
            If the deployment group does not have a deployment configuration associated with it,
                <code>CodeDeployDefault</code>.<code>OneAtATime</code> is used by default.</p> |
| `deployment_group_name` | String |  | <p>The name of the deployment group.</p> |
| `file_exists_behavior` | String |  | <p>Information about how CodeDeploy handles files that already exist in a
            deployment target location but weren't part of the previous successful
            deployment.</p>
         <p>The <code>fileExistsBehavior</code> parameter takes any of the following
            values:</p>
         <ul>
            <li>
               <p>DISALLOW: The deployment fails. This is also the default behavior if no option
                    is specified.</p>
            </li>
            <li>
               <p>OVERWRITE: The version of the file from the application revision currently
                    being deployed replaces the version already on the instance.</p>
            </li>
            <li>
               <p>RETAIN: The version of the file already on the instance is kept and used as
                    part of the new deployment.</p>
            </li>
         </ul> |
| `revision` | String |  | <p> The type and location of the revision to deploy. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployment_info` | String | <p>Information about the deployment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment
deployment = provider.codedeploy.Deployment {
    application_name = "value"  # <p>The name of an CodeDeploy application associated with the user or Amazon Web Services account.</p>
}

# Access deployment outputs
deployment_id = deployment.id
deployment_deployment_info = deployment.deployment_info
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple application_revision resources
application_revision_0 = provider.codedeploy.Application_revision {
}
application_revision_1 = provider.codedeploy.Application_revision {
}
application_revision_2 = provider.codedeploy.Application_revision {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    application_revision = provider.codedeploy.Application_revision {
    }
```

---

## Related Documentation

- [AWS Codedeploy Documentation](https://docs.aws.amazon.com/codedeploy/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
