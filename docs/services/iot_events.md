# Iot_events Service



**Resources**: 6

---

## Overview

The iot_events service provides access to 6 resource types:

- [Detector_model_analysis_results](#detector_model_analysis_results) [R]
- [Alarm_model](#alarm_model) [CRUD]
- [Detector_model](#detector_model) [CRUD]
- [Logging_options](#logging_options) [CR]
- [Input](#input) [CRUD]
- [Detector_model_analysis](#detector_model_analysis) [R]

---

## Resources


### Detector_model_analysis_results

DetectorModelAnalysisResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token that you can use to return the next set of results, 
or <code>null</code> if there are no more results.</p> |
| `analysis_results` | Vec<String> | <p>Contains information about one or more analysis results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access detector_model_analysis_results outputs
detector_model_analysis_results_id = detector_model_analysis_results.id
detector_model_analysis_results_next_token = detector_model_analysis_results.next_token
detector_model_analysis_results_analysis_results = detector_model_analysis_results.analysis_results
```

---


### Alarm_model

AlarmModel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alarm_event_actions` | String |  | <p>Contains information about one or more alarm actions.</p> |
| `severity` | i64 |  | <p>A non-negative integer that reflects the severity level of the alarm.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs that contain metadata for the alarm model. The tags help you
      manage the alarm model. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/tagging-iotevents.html">Tagging your AWS IoT Events
        resources</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
         <p>You can create up to 50 tags for one alarm model.</p> |
| `key` | String |  | <p>An input attribute used as a key to create an alarm. 
AWS IoT Events routes <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Input.html">inputs</a> 
associated with this key to the alarm.</p> |
| `alarm_model_description` | String |  | <p>A description that tells you what the alarm model detects.</p> |
| `alarm_rule` | String | ✅ | <p>Defines when your alarm is invoked.</p> |
| `alarm_model_name` | String | ✅ | <p>A unique name that helps you identify the alarm model. You can't change this name after
      you create the alarm model.</p> |
| `alarm_capabilities` | String |  | <p>Contains the configuration information of alarm state changes.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p> |
| `alarm_notification` | String |  | <p>Contains information about one or more notification actions.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_message` | String | <p>
      Contains information about the status of the alarm model.
    </p> |
| `alarm_notification` | String | <p>Contains information about one or more notification actions.</p> |
| `alarm_model_name` | String | <p>The name of the alarm model.</p> |
| `key` | String | <p>An input attribute used as a key to create an alarm. 
AWS IoT Events routes <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Input.html">inputs</a> 
associated with this key to the alarm.</p> |
| `severity` | i64 | <p>A non-negative integer that reflects the severity level of the alarm.</p> |
| `alarm_event_actions` | String | <p>Contains information about one or more alarm actions.</p> |
| `alarm_rule` | String | <p>Defines when your alarm is invoked.</p> |
| `creation_time` | String | <p>The time the alarm model was created, in the Unix epoch format.</p> |
| `alarm_model_arn` | String | <p>The ARN of the alarm model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p> |
| `last_update_time` | String | <p>The time the alarm model was last updated, in the Unix epoch format.</p> |
| `alarm_capabilities` | String | <p>Contains the configuration information of alarm state changes.</p> |
| `status` | String | <p>The status of the alarm model. The status can be one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code> - The alarm model is active and it's ready to evaluate data.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVATING</code> - AWS IoT Events is activating your alarm model. 
        Activating an alarm model can take up to a few minutes.</p>
            </li>
            <li>
               <p>
                  <code>INACTIVE</code> - The alarm model is inactive, so it isn't ready to evaluate data. 
	  Check your alarm model information and update the alarm model.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - You couldn't create or update the alarm model. Check your alarm model information 
        and try again.</p>
            </li>
         </ul> |
| `role_arn` | String | <p>The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p> |
| `alarm_model_description` | String | <p>The description of the alarm model.</p> |
| `alarm_model_version` | String | <p>The version of the alarm model.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create alarm_model
alarm_model = provider.iot_events.Alarm_model {
    alarm_rule = "value"  # <p>Defines when your alarm is invoked.</p>
    alarm_model_name = "value"  # <p>A unique name that helps you identify the alarm model. You can't change this name after
      you create the alarm model.</p>
    role_arn = "value"  # <p>The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
}

# Access alarm_model outputs
alarm_model_id = alarm_model.id
alarm_model_status_message = alarm_model.status_message
alarm_model_alarm_notification = alarm_model.alarm_notification
alarm_model_alarm_model_name = alarm_model.alarm_model_name
alarm_model_key = alarm_model.key
alarm_model_severity = alarm_model.severity
alarm_model_alarm_event_actions = alarm_model.alarm_event_actions
alarm_model_alarm_rule = alarm_model.alarm_rule
alarm_model_creation_time = alarm_model.creation_time
alarm_model_alarm_model_arn = alarm_model.alarm_model_arn
alarm_model_last_update_time = alarm_model.last_update_time
alarm_model_alarm_capabilities = alarm_model.alarm_capabilities
alarm_model_status = alarm_model.status
alarm_model_role_arn = alarm_model.role_arn
alarm_model_alarm_model_description = alarm_model.alarm_model_description
alarm_model_alarm_model_version = alarm_model.alarm_model_version
```

---


### Detector_model

DetectorModel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Metadata that can be used to manage the detector model.</p> |
| `evaluation_method` | String |  | <p>Information about the order in which events are evaluated and how actions are executed.
    </p> |
| `detector_model_definition` | String | ✅ | <p>Information that defines how the detectors operate.</p> |
| `key` | String |  | <p>The input attribute key used to identify a device or system to create a detector (an
      instance of the detector model) and then to route each input received to the appropriate
      detector (instance). This parameter uses a JSON-path expression in the message payload of each
      input to specify the attribute-value pair that is used to identify the device associated with
      the input.</p> |
| `detector_model_name` | String | ✅ | <p>The name of the detector model.</p> |
| `detector_model_description` | String |  | <p>A brief description of the detector model.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `detector_model` | String | <p>Information about the detector model.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create detector_model
detector_model = provider.iot_events.Detector_model {
    detector_model_definition = "value"  # <p>Information that defines how the detectors operate.</p>
    detector_model_name = "value"  # <p>The name of the detector model.</p>
    role_arn = "value"  # <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
}

# Access detector_model outputs
detector_model_id = detector_model.id
detector_model_detector_model = detector_model.detector_model
```

---


### Logging_options

LoggingOptions resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_options` | String | ✅ | <p>The new values of the AWS IoT Events logging options.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_options` | String | <p>The current settings of the AWS IoT Events logging options.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logging_options
logging_options = provider.iot_events.Logging_options {
    logging_options = "value"  # <p>The new values of the AWS IoT Events logging options.</p>
}

# Access logging_options outputs
logging_options_id = logging_options.id
logging_options_logging_options = logging_options.logging_options
```

---


### Input

Input resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_name` | String | ✅ | <p>The name you want to give to the input.</p> |
| `input_description` | String |  | <p>A brief description of the input.</p> |
| `tags` | Vec<String> |  | <p>Metadata that can be used to manage the input.</p> |
| `input_definition` | String | ✅ | <p>The definition of the input.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `input` | String | <p>Information about the input.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create input
input = provider.iot_events.Input {
    input_name = "value"  # <p>The name you want to give to the input.</p>
    input_definition = "value"  # <p>The definition of the input.</p>
}

# Access input outputs
input_id = input.id
input_input = input.input
```

---


### Detector_model_analysis

DetectorModelAnalysis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the analysis activity. The status can be one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>RUNNING</code> - AWS IoT Events is analyzing your detector model. This process can take
          several minutes to complete.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETE</code> - AWS IoT Events finished analyzing your detector model.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - AWS IoT Events couldn't analyze your detector model. Try again
          later.</p>
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

# Access detector_model_analysis outputs
detector_model_analysis_id = detector_model_analysis.id
detector_model_analysis_status = detector_model_analysis.status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple detector_model_analysis_results resources
detector_model_analysis_results_0 = provider.iot_events.Detector_model_analysis_results {
}
detector_model_analysis_results_1 = provider.iot_events.Detector_model_analysis_results {
}
detector_model_analysis_results_2 = provider.iot_events.Detector_model_analysis_results {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    detector_model_analysis_results = provider.iot_events.Detector_model_analysis_results {
    }
```

---

## Related Documentation

- [AWS Iot_events Documentation](https://docs.aws.amazon.com/iot_events/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
