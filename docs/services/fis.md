# Fis Service



**Resources**: 8

---

## Overview

The fis service provides access to 8 resource types:

- [Experiment](#experiment) [R]
- [Safety_lever](#safety_lever) [R]
- [Experiment_target_account_configuration](#experiment_target_account_configuration) [R]
- [Target_resource_type](#target_resource_type) [R]
- [Target_account_configuration](#target_account_configuration) [CRUD]
- [Action](#action) [R]
- [Experiment_template](#experiment_template) [CRUD]
- [Safety_lever_state](#safety_lever_state) [U]

---

## Resources


### Experiment

Experiment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `experiment` | String | <p>Information about the experiment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access experiment outputs
experiment_id = experiment.id
experiment_experiment = experiment.experiment
```

---


### Safety_lever

SafetyLever resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `safety_lever` | String | <p>
      Information about the safety lever.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access safety_lever outputs
safety_lever_id = safety_lever.id
safety_lever_safety_lever = safety_lever.safety_lever
```

---


### Experiment_target_account_configuration

ExperimentTargetAccountConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_account_configuration` | String | <p>Information about the target account configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access experiment_target_account_configuration outputs
experiment_target_account_configuration_id = experiment_target_account_configuration.id
experiment_target_account_configuration_target_account_configuration = experiment_target_account_configuration.target_account_configuration
```

---


### Target_resource_type

TargetResourceType resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_resource_type` | String | <p>Information about the resource type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access target_resource_type outputs
target_resource_type_id = target_resource_type.id
target_resource_type_target_resource_type = target_resource_type.target_resource_type
```

---


### Target_account_configuration

TargetAccountConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `experiment_template_id` | String | ✅ | <p>The experiment template ID.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role for the target account.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |
| `account_id` | String | ✅ | <p>The Amazon Web Services account ID of the target account.</p> |
| `description` | String |  | <p>The description of the target account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_account_configuration` | String | <p>Information about the target account configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create target_account_configuration
target_account_configuration = provider.fis.Target_account_configuration {
    experiment_template_id = "value"  # <p>The experiment template ID.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role for the target account.</p>
    account_id = "value"  # <p>The Amazon Web Services account ID of the target account.</p>
}

# Access target_account_configuration outputs
target_account_configuration_id = target_account_configuration.id
target_account_configuration_target_account_configuration = target_account_configuration.target_account_configuration
```

---


### Action

Action resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action` | String | <p>Information about the action.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access action outputs
action_id = action.id
action_action = action.action
```

---


### Experiment_template

ExperimentTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `actions` | HashMap<String, String> | ✅ | <p>The actions for the experiment.</p> |
| `log_configuration` | String |  | <p>The configuration for experiment logging.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |
| `experiment_options` | String |  | <p>The experiment options for the experiment template.</p> |
| `experiment_report_configuration` | String |  | <p>The experiment report configuration for the experiment template.</p> |
| `stop_conditions` | Vec<String> | ✅ | <p>The stop conditions.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that grants the FIS service permission to perform service actions on your behalf.</p> |
| `description` | String | ✅ | <p>A description for the experiment template.</p> |
| `targets` | HashMap<String, String> |  | <p>The targets for the experiment.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to apply to the experiment template.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `experiment_template` | String | <p>Information about the experiment template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create experiment_template
experiment_template = provider.fis.Experiment_template {
    actions = "value"  # <p>The actions for the experiment.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    stop_conditions = "value"  # <p>The stop conditions.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that grants the FIS service permission to perform service actions on your behalf.</p>
    description = "value"  # <p>A description for the experiment template.</p>
}

# Access experiment_template outputs
experiment_template_id = experiment_template.id
experiment_template_experiment_template = experiment_template.experiment_template
```

---


### Safety_lever_state

SafetyLeverState resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>
         The ID of the safety lever.
      </p> |
| `state` | String | ✅ | <p>
       The state of the safety lever.
      </p> |



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

# Create multiple experiment resources
experiment_0 = provider.fis.Experiment {
}
experiment_1 = provider.fis.Experiment {
}
experiment_2 = provider.fis.Experiment {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    experiment = provider.fis.Experiment {
    }
```

---

## Related Documentation

- [AWS Fis Documentation](https://docs.aws.amazon.com/fis/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
