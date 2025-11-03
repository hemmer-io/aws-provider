# Sfn Service



**Resources**: 9

---

## Overview

The sfn service provides access to 9 resource types:

- [State_machine_alias](#state_machine_alias) [CRUD]
- [Execution](#execution) [R]
- [Execution_history](#execution_history) [R]
- [State_machine_for_execution](#state_machine_for_execution) [R]
- [Activity_task](#activity_task) [R]
- [State_machine](#state_machine) [CRUD]
- [State_machine_version](#state_machine_version) [D]
- [Map_run](#map_run) [RU]
- [Activity](#activity) [CRD]

---

## Resources


### State_machine_alias

StateMachineAlias resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the state machine alias.</p>
         <p>To avoid conflict with version ARNs, don't use an integer in the name of the alias.</p> |
| `description` | String |  | <p>A description for the state machine alias.</p> |
| `routing_configuration` | Vec<String> | ✅ | <p>The routing configuration of a state machine alias. The routing configuration shifts
      execution traffic between two state machine versions. <code>routingConfiguration</code>
      contains an array of <code>RoutingConfig</code> objects that specify up to two state machine
      versions. Step Functions then randomly choses which version to run an execution with based
      on the weight assigned to each <code>RoutingConfig</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_machine_alias_arn` | String | <p>The Amazon Resource Name (ARN) of the state machine alias.</p> |
| `name` | String | <p>The name of the state machine alias.</p> |
| `description` | String | <p>A description of the alias.</p> |
| `creation_date` | String | <p>The date the state machine alias was created.</p> |
| `update_date` | String | <p>The date the state machine alias was last updated.</p>
         <p>For a newly created state machine, this is the same as the creation date.</p> |
| `routing_configuration` | Vec<String> | <p>The routing configuration of the alias.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create state_machine_alias
state_machine_alias = provider.sfn.State_machine_alias {
    name = "value"  # <p>The name of the state machine alias.</p>
         <p>To avoid conflict with version ARNs, don't use an integer in the name of the alias.</p>
    routing_configuration = "value"  # <p>The routing configuration of a state machine alias. The routing configuration shifts
      execution traffic between two state machine versions. <code>routingConfiguration</code>
      contains an array of <code>RoutingConfig</code> objects that specify up to two state machine
      versions. Step Functions then randomly choses which version to run an execution with based
      on the weight assigned to each <code>RoutingConfig</code>.</p>
}

# Access state_machine_alias outputs
state_machine_alias_id = state_machine_alias.id
state_machine_alias_state_machine_alias_arn = state_machine_alias.state_machine_alias_arn
state_machine_alias_name = state_machine_alias.name
state_machine_alias_description = state_machine_alias.description
state_machine_alias_creation_date = state_machine_alias.creation_date
state_machine_alias_update_date = state_machine_alias.update_date
state_machine_alias_routing_configuration = state_machine_alias.routing_configuration
```

---


### Execution

Execution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_machine_arn` | String | <p>The Amazon Resource Name (ARN) of the executed stated machine.</p> |
| `trace_header` | String | <p>The X-Ray trace header that was passed to the execution.</p>
         <note>
            <p>
                For X-Ray traces, all Amazon Web Services services use the <code>X-Amzn-Trace-Id</code> header from the HTTP request. Using the header is the preferred mechanism to identify a trace. <code>StartExecution</code> and <code>StartSyncExecution</code> API operations can also use <code>traceHeader</code> from the body of the request payload. If <b>both</b> sources are provided, Step Functions will use the <b>header value</b> (preferred) over the value in the request body.
            </p>
         </note> |
| `cause` | String | <p>The cause string if the state machine execution failed.</p> |
| `redrive_count` | i64 | <p>The number of times you've redriven an execution. If you have not yet redriven an execution, the <code>redriveCount</code> is 0. This count is only updated if you successfully redrive an execution.</p> |
| `redrive_status` | String | <p>Indicates whether or not an execution can be redriven at a given point in time.</p>
         <ul>
            <li>
               <p>For executions of type <code>STANDARD</code>, <code>redriveStatus</code> is <code>NOT_REDRIVABLE</code> if calling the <a>RedriveExecution</a> API action would return the <code>ExecutionNotRedrivable</code> error.</p>
            </li>
            <li>
               <p>For a Distributed Map that includes child workflows of type <code>STANDARD</code>, <code>redriveStatus</code> indicates whether or not the Map Run can redrive child workflow executions.</p>
            </li>
            <li>
               <p>For a Distributed Map that includes child workflows of type <code>EXPRESS</code>, <code>redriveStatus</code> indicates whether or not the Map Run can redrive child workflow executions.</p>
               <p>You can redrive failed or timed out <code>EXPRESS</code> workflows <i>only if</i> they're a part of a Map Run. When you <a href="https://docs.aws.amazon.com/step-functions/latest/dg/redrive-map-run.html">redrive</a> the Map Run, these workflows are restarted using the <a>StartExecution</a> API action.</p>
            </li>
         </ul> |
| `name` | String | <p>The name of the execution.</p>
         <p>A name must <i>not</i> contain:</p>
         <ul>
            <li>
               <p>white space</p>
            </li>
            <li>
               <p>brackets <code>< > { } [ ]</code>
               </p>
            </li>
            <li>
               <p>wildcard characters <code>? *</code>
               </p>
            </li>
            <li>
               <p>special characters <code>" # % \ ^ | ~ ` $ & , ; : /</code>
               </p>
            </li>
            <li>
               <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p>
            </li>
            <li>
               <p>surrogates (<code>U+D800-DFFF</code>)</p>
            </li>
            <li>
               <p>invalid characters (<code> U+10FFFF</code>)</p>
            </li>
         </ul>
         <p>To enable logging with CloudWatch Logs, the name should only contain  0-9, A-Z, a-z, - and _.</p> |
| `error` | String | <p>The error string if the state machine execution failed.</p> |
| `state_machine_alias_arn` | String | <p>The Amazon Resource Name (ARN) of the state machine alias associated with the execution. The alias ARN is a combination of state machine ARN and the alias name separated by a colon (:). For example, <code>stateMachineARN:PROD</code>.</p>
         <p>If you start an execution from a <code>StartExecution</code> request with a
      state machine version ARN, this field will be null.</p> |
| `state_machine_version_arn` | String | <p>The Amazon Resource Name (ARN) of the state machine version associated with the execution. The version ARN is a combination of state machine ARN and the version number separated by a colon (:). For example, <code>stateMachineARN:1</code>.</p>
         <p>If you start an execution from a <code>StartExecution</code> request without specifying a
      state machine version or alias ARN, Step Functions returns a null value.</p> |
| `redrive_status_reason` | String | <p>When <code>redriveStatus</code> is <code>NOT_REDRIVABLE</code>, <code>redriveStatusReason</code> specifies the reason why an execution cannot be redriven.</p>
         <ul>
            <li>
               <p>For executions of type <code>STANDARD</code>, or for a Distributed Map that includes child workflows of type <code>STANDARD</code>, <code>redriveStatusReason</code> can include one of the following reasons:</p>
               <ul>
                  <li>
                     <p>
                        <code>State machine is in DELETING status</code>.</p>
                  </li>
                  <li>
                     <p>
                        <code>Execution is RUNNING and cannot be redriven</code>.</p>
                  </li>
                  <li>
                     <p>
                        <code>Execution is SUCCEEDED and cannot be redriven</code>.</p>
                  </li>
                  <li>
                     <p>
                        <code>Execution was started before the launch of RedriveExecution</code>.</p>
                  </li>
                  <li>
                     <p>
                        <code>Execution history event limit exceeded</code>.</p>
                  </li>
                  <li>
                     <p>
                        <code>Execution has exceeded the max execution time</code>.</p>
                  </li>
                  <li>
                     <p>
                        <code>Execution redrivable period exceeded</code>.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>For a Distributed Map that includes child workflows of type <code>EXPRESS</code>, <code>redriveStatusReason</code> is only returned if the child workflows are not redrivable. This happens when the child workflow executions have completed successfully.</p>
            </li>
         </ul> |
| `start_date` | String | <p>The date the execution is started.</p> |
| `output_details` | String |  |
| `output` | String | <p>The JSON output data of the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p>
         <note>
            <p>This field is set only if the execution succeeds. If the execution fails, this field is
        null.</p>
         </note> |
| `status` | String | <p>The current status of the execution.</p> |
| `stop_date` | String | <p>If the execution ended, the date the execution stopped.</p> |
| `map_run_arn` | i64 | <p>The Amazon Resource Name (ARN) that identifies a Map Run, which dispatched this execution.</p> |
| `redrive_date` | String | <p>The date the execution was last redriven. If you have not yet redriven an execution, the <code>redriveDate</code> is null.</p>
         <p>The <code>redriveDate</code> is unavailable if you redrive a Map Run that starts child workflow executions of type <code>EXPRESS</code>.</p> |
| `input_details` | String |  |
| `execution_arn` | String | <p>The Amazon Resource Name (ARN) that identifies the execution.</p> |
| `input` | String | <p>The string that contains the JSON input data of the execution. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access execution outputs
execution_id = execution.id
execution_state_machine_arn = execution.state_machine_arn
execution_trace_header = execution.trace_header
execution_cause = execution.cause
execution_redrive_count = execution.redrive_count
execution_redrive_status = execution.redrive_status
execution_name = execution.name
execution_error = execution.error
execution_state_machine_alias_arn = execution.state_machine_alias_arn
execution_state_machine_version_arn = execution.state_machine_version_arn
execution_redrive_status_reason = execution.redrive_status_reason
execution_start_date = execution.start_date
execution_output_details = execution.output_details
execution_output = execution.output
execution_status = execution.status
execution_stop_date = execution.stop_date
execution_map_run_arn = execution.map_run_arn
execution_redrive_date = execution.redrive_date
execution_input_details = execution.input_details
execution_execution_arn = execution.execution_arn
execution_input = execution.input
```

---


### Execution_history

ExecutionHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events` | Vec<String> | <p>The list of events that occurred in the execution.</p> |
| `next_token` | String | <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page.
    Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access execution_history outputs
execution_history_id = execution_history.id
execution_history_events = execution_history.events
execution_history_next_token = execution_history.next_token
```

---


### State_machine_for_execution

StateMachineForExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `variable_references` | HashMap<String, Vec<String>> | <p>A map of <b>state name</b> to a list of variables referenced by that state. States that do not use variable references will not be shown in the response.</p> |
| `definition` | String | <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p> |
| `label` | String | <p>A user-defined or an auto-generated string that identifies a <code>Map</code> state. This field is returned only if the <code>executionArn</code> is a child workflow execution that was started by a Distributed Map state.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role of the State Machine for the execution. </p> |
| `map_run_arn` | i64 | <p>The Amazon Resource Name (ARN) of the Map Run that started the child workflow execution. This field is returned only if the <code>executionArn</code> is a child workflow execution that was started by a Distributed Map state.</p> |
| `revision_id` | String | <p>The revision identifier for the state machine. The first revision ID when you create the state machine is null.</p>
         <p>Use the state machine <code>revisionId</code> parameter to compare the revision of a state machine with the configuration of the state machine used for executions without performing a diff of the properties, such as <code>definition</code> and <code>roleArn</code>.</p> |
| `encryption_configuration` | String | <p>Settings to configure server-side encryption. </p> |
| `tracing_configuration` | String | <p>Selects whether X-Ray tracing is enabled.</p> |
| `state_machine_arn` | String | <p>The Amazon Resource Name (ARN) of the state machine associated with the execution.</p> |
| `name` | String | <p>The name of the state machine associated with the execution.</p> |
| `update_date` | String | <p>The date and time the state machine associated with an execution was updated. For a newly
      created state machine, this is the creation date.</p> |
| `logging_configuration` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access state_machine_for_execution outputs
state_machine_for_execution_id = state_machine_for_execution.id
state_machine_for_execution_variable_references = state_machine_for_execution.variable_references
state_machine_for_execution_definition = state_machine_for_execution.definition
state_machine_for_execution_label = state_machine_for_execution.label
state_machine_for_execution_role_arn = state_machine_for_execution.role_arn
state_machine_for_execution_map_run_arn = state_machine_for_execution.map_run_arn
state_machine_for_execution_revision_id = state_machine_for_execution.revision_id
state_machine_for_execution_encryption_configuration = state_machine_for_execution.encryption_configuration
state_machine_for_execution_tracing_configuration = state_machine_for_execution.tracing_configuration
state_machine_for_execution_state_machine_arn = state_machine_for_execution.state_machine_arn
state_machine_for_execution_name = state_machine_for_execution.name
state_machine_for_execution_update_date = state_machine_for_execution.update_date
state_machine_for_execution_logging_configuration = state_machine_for_execution.logging_configuration
```

---


### Activity_task

ActivityTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `input` | String | <p>The string that contains the JSON input data for the task. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p> |
| `task_token` | String | <p>A token that identifies the scheduled task. This token must be copied and included in
      subsequent calls to <a>SendTaskHeartbeat</a>, <a>SendTaskSuccess</a> or
        <a>SendTaskFailure</a> in order to report the progress or completion of the
      task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access activity_task outputs
activity_task_id = activity_task.id
activity_task_input = activity_task.input
activity_task_task_token = activity_task.task_token
```

---


### State_machine

StateMachine resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `publish` | bool |  | <p>Set to <code>true</code> to publish the first version of the state machine during creation. The default is <code>false</code>.</p> |
| `encryption_configuration` | String |  | <p>Settings to configure server-side encryption.</p> |
| `name` | String | ✅ | <p>The name of the state machine. </p>
         <p>A name must <i>not</i> contain:</p>
         <ul>
            <li>
               <p>white space</p>
            </li>
            <li>
               <p>brackets <code>< > { } [ ]</code>
               </p>
            </li>
            <li>
               <p>wildcard characters <code>? *</code>
               </p>
            </li>
            <li>
               <p>special characters <code>" # % \ ^ | ~ ` $ & , ; : /</code>
               </p>
            </li>
            <li>
               <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p>
            </li>
            <li>
               <p>surrogates (<code>U+D800-DFFF</code>)</p>
            </li>
            <li>
               <p>invalid characters (<code> U+10FFFF</code>)</p>
            </li>
         </ul>
         <p>To enable logging with CloudWatch Logs, the name should only contain  0-9, A-Z, a-z, - and _.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role to use for this state machine.</p> |
| `type` | String |  | <p>Determines whether a Standard or Express state machine is created. The default is
        <code>STANDARD</code>. You cannot update the <code>type</code> of a state machine once it
      has been created.</p> |
| `version_description` | String |  | <p>Sets description about the state machine version. You can only set the description if the <code>publish</code> parameter is set to <code>true</code>. Otherwise, if you set <code>versionDescription</code>, but <code>publish</code> to <code>false</code>, this API action throws <code>ValidationException</code>.</p> |
| `tags` | Vec<String> |  | <p>Tags to be added when creating a state machine.</p>
         <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using
      Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User
        Guide</i>, and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM
          Tags</a>.</p>
         <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p> |
| `logging_configuration` | String |  | <p>Defines what execution history events are logged and where they are logged.</p>
         <note>
            <p>By default, the <code>level</code> is set to <code>OFF</code>. For more information see
          <a href="https://docs.aws.amazon.com/step-functions/latest/dg/cloudwatch-log-level.html">Log
          Levels</a> in the Step Functions User Guide.</p>
         </note> |
| `definition` | String | ✅ | <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p> |
| `tracing_configuration` | String |  | <p>Selects whether X-Ray tracing is enabled.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_configuration` | String |  |
| `tracing_configuration` | String | <p>Selects whether X-Ray tracing is enabled.</p> |
| `definition` | String | <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
         <p>If called with <code>includedData = METADATA_ONLY</code>, the returned definition will be <code>{}</code>.</p> |
| `description` | String | <p>The description of the state machine version.</p> |
| `creation_date` | String | <p>The date the state machine is created.</p>
         <p>For a state machine version, <code>creationDate</code> is the date the version was created.</p> |
| `variable_references` | HashMap<String, Vec<String>> | <p>A map of <b>state name</b> to a list of variables referenced by that state. States that do not use variable references will not be shown in the response.</p> |
| `encryption_configuration` | String | <p>Settings to configure server-side encryption. </p> |
| `revision_id` | String | <p>The revision identifier for the state machine.</p>
         <p>Use the <code>revisionId</code> parameter to compare between versions of a state machine
      configuration used for executions without performing a diff of the properties, such as
        <code>definition</code> and <code>roleArn</code>.</p> |
| `label` | String | <p>A user-defined or an auto-generated string that identifies a <code>Map</code> state. This parameter is present only if the <code>stateMachineArn</code> specified in input is a qualified state machine ARN.</p> |
| `status` | String | <p>The current status of the state machine.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role used when creating this state machine. (The IAM role
      maintains security by granting Step Functions access to Amazon Web Services resources.)</p> |
| `name` | String | <p>The name of the state machine.</p>
         <p>A name must <i>not</i> contain:</p>
         <ul>
            <li>
               <p>white space</p>
            </li>
            <li>
               <p>brackets <code>< > { } [ ]</code>
               </p>
            </li>
            <li>
               <p>wildcard characters <code>? *</code>
               </p>
            </li>
            <li>
               <p>special characters <code>" # % \ ^ | ~ ` $ & , ; : /</code>
               </p>
            </li>
            <li>
               <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p>
            </li>
            <li>
               <p>surrogates (<code>U+D800-DFFF</code>)</p>
            </li>
            <li>
               <p>invalid characters (<code> U+10FFFF</code>)</p>
            </li>
         </ul>
         <p>To enable logging with CloudWatch Logs, the name should only contain  0-9, A-Z, a-z, - and _.</p> |
| `state_machine_arn` | String | <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
         <p>If you specified a state machine version ARN in your request, the API returns the version ARN. The version ARN is a combination of state machine ARN and the version number separated by a colon (:). For example, <code>stateMachineARN:1</code>.</p> |
| `type` | String | <p>The <code>type</code> of the state machine (<code>STANDARD</code> or
      <code>EXPRESS</code>).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create state_machine
state_machine = provider.sfn.State_machine {
    name = "value"  # <p>The name of the state machine. </p>
         <p>A name must <i>not</i> contain:</p>
         <ul>
            <li>
               <p>white space</p>
            </li>
            <li>
               <p>brackets <code>< > { } [ ]</code>
               </p>
            </li>
            <li>
               <p>wildcard characters <code>? *</code>
               </p>
            </li>
            <li>
               <p>special characters <code>" # % \ ^ | ~ ` $ & , ; : /</code>
               </p>
            </li>
            <li>
               <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p>
            </li>
            <li>
               <p>surrogates (<code>U+D800-DFFF</code>)</p>
            </li>
            <li>
               <p>invalid characters (<code> U+10FFFF</code>)</p>
            </li>
         </ul>
         <p>To enable logging with CloudWatch Logs, the name should only contain  0-9, A-Z, a-z, - and _.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role to use for this state machine.</p>
    definition = "value"  # <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
}

# Access state_machine outputs
state_machine_id = state_machine.id
state_machine_logging_configuration = state_machine.logging_configuration
state_machine_tracing_configuration = state_machine.tracing_configuration
state_machine_definition = state_machine.definition
state_machine_description = state_machine.description
state_machine_creation_date = state_machine.creation_date
state_machine_variable_references = state_machine.variable_references
state_machine_encryption_configuration = state_machine.encryption_configuration
state_machine_revision_id = state_machine.revision_id
state_machine_label = state_machine.label
state_machine_status = state_machine.status
state_machine_role_arn = state_machine.role_arn
state_machine_name = state_machine.name
state_machine_state_machine_arn = state_machine.state_machine_arn
state_machine_type = state_machine.type
```

---


### State_machine_version

StateMachineVersion resource

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


### Map_run

MapRun resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `map_run_arn` | i64 | ✅ | <p>The Amazon Resource Name (ARN) of a Map Run.</p> |
| `tolerated_failure_percentage` | String |  | <p>The maximum percentage of failed items before the Map Run fails.</p> |
| `tolerated_failure_count` | i64 |  | <p>The maximum number of failed items before the Map Run fails.</p> |
| `max_concurrency` | i64 |  | <p>The maximum number of child workflow executions that can be specified to run in parallel for the Map Run at the same time.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_arn` | String | <p>The Amazon Resource Name (ARN) that identifies the execution in which the Map Run was started.</p> |
| `start_date` | String | <p>The date when the Map Run was started.</p> |
| `tolerated_failure_count` | i64 | <p>The maximum number of failed child workflow executions before the Map Run fails.</p> |
| `map_run_arn` | i64 | <p>The Amazon Resource Name (ARN) that identifies a Map Run.</p> |
| `item_counts` | String | <p>A JSON object that contains information about the total number of items, and the item count for each processing status, such as <code>pending</code> and <code>failed</code>.</p> |
| `status` | String | <p>The current status of the Map Run.</p> |
| `tolerated_failure_percentage` | String | <p>The maximum percentage of failed child workflow executions before the Map Run fails.</p> |
| `execution_counts` | String | <p>A JSON object that contains information about the total number of child workflow executions for the Map Run, and the count of child workflow executions for each status, such as <code>failed</code> and <code>succeeded</code>.</p> |
| `redrive_count` | i64 | <p>The number of times you've redriven a Map Run. If you have not yet redriven a Map Run, the <code>redriveCount</code> is 0. This count is only updated if you successfully redrive a Map Run.</p> |
| `stop_date` | String | <p>The date when the Map Run was stopped.</p> |
| `redrive_date` | String | <p>The date a Map Run was last redriven. If you have not yet redriven a Map Run, the <code>redriveDate</code> is null.</p> |
| `max_concurrency` | i64 | <p>The maximum number of child workflow executions configured to run in parallel for the Map Run at the same time.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access map_run outputs
map_run_id = map_run.id
map_run_execution_arn = map_run.execution_arn
map_run_start_date = map_run.start_date
map_run_tolerated_failure_count = map_run.tolerated_failure_count
map_run_map_run_arn = map_run.map_run_arn
map_run_item_counts = map_run.item_counts
map_run_status = map_run.status
map_run_tolerated_failure_percentage = map_run.tolerated_failure_percentage
map_run_execution_counts = map_run.execution_counts
map_run_redrive_count = map_run.redrive_count
map_run_stop_date = map_run.stop_date
map_run_redrive_date = map_run.redrive_date
map_run_max_concurrency = map_run.max_concurrency
```

---


### Activity

Activity resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the activity to create. This name must be unique for your Amazon Web Services account and region for 90 days. For more information,
    see <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions">
    Limits Related to State Machine Executions</a> in the <i>Step Functions Developer Guide</i>.</p>
         <p>A name must <i>not</i> contain:</p>
         <ul>
            <li>
               <p>white space</p>
            </li>
            <li>
               <p>brackets <code>< > { } [ ]</code>
               </p>
            </li>
            <li>
               <p>wildcard characters <code>? *</code>
               </p>
            </li>
            <li>
               <p>special characters <code>" # % \ ^ | ~ ` $ & , ; : /</code>
               </p>
            </li>
            <li>
               <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p>
            </li>
            <li>
               <p>surrogates (<code>U+D800-DFFF</code>)</p>
            </li>
            <li>
               <p>invalid characters (<code> U+10FFFF</code>)</p>
            </li>
         </ul>
         <p>To enable logging with CloudWatch Logs, the name should only contain  0-9, A-Z, a-z, - and _.</p> |
| `tags` | Vec<String> |  | <p>The list of tags to add to a resource.</p>
         <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using
      Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User
        Guide</i>, and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM
          Tags</a>.</p>
         <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p> |
| `encryption_configuration` | String |  | <p>Settings to configure server-side encryption.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the activity.</p>
         <p>A name must <i>not</i> contain:</p>
         <ul>
            <li>
               <p>white space</p>
            </li>
            <li>
               <p>brackets <code>< > { } [ ]</code>
               </p>
            </li>
            <li>
               <p>wildcard characters <code>? *</code>
               </p>
            </li>
            <li>
               <p>special characters <code>" # % \ ^ | ~ ` $ & , ; : /</code>
               </p>
            </li>
            <li>
               <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p>
            </li>
            <li>
               <p>surrogates (<code>U+D800-DFFF</code>)</p>
            </li>
            <li>
               <p>invalid characters (<code> U+10FFFF</code>)</p>
            </li>
         </ul>
         <p>To enable logging with CloudWatch Logs, the name should only contain  0-9, A-Z, a-z, - and _.</p> |
| `encryption_configuration` | String | <p>Settings for configured server-side encryption.</p> |
| `activity_arn` | String | <p>The Amazon Resource Name (ARN) that identifies the activity.</p> |
| `creation_date` | String | <p>The date the activity is created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create activity
activity = provider.sfn.Activity {
    name = "value"  # <p>The name of the activity to create. This name must be unique for your Amazon Web Services account and region for 90 days. For more information,
    see <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions">
    Limits Related to State Machine Executions</a> in the <i>Step Functions Developer Guide</i>.</p>
         <p>A name must <i>not</i> contain:</p>
         <ul>
            <li>
               <p>white space</p>
            </li>
            <li>
               <p>brackets <code>< > { } [ ]</code>
               </p>
            </li>
            <li>
               <p>wildcard characters <code>? *</code>
               </p>
            </li>
            <li>
               <p>special characters <code>" # % \ ^ | ~ ` $ & , ; : /</code>
               </p>
            </li>
            <li>
               <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p>
            </li>
            <li>
               <p>surrogates (<code>U+D800-DFFF</code>)</p>
            </li>
            <li>
               <p>invalid characters (<code> U+10FFFF</code>)</p>
            </li>
         </ul>
         <p>To enable logging with CloudWatch Logs, the name should only contain  0-9, A-Z, a-z, - and _.</p>
}

# Access activity outputs
activity_id = activity.id
activity_name = activity.name
activity_encryption_configuration = activity.encryption_configuration
activity_activity_arn = activity.activity_arn
activity_creation_date = activity.creation_date
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple state_machine_alias resources
state_machine_alias_0 = provider.sfn.State_machine_alias {
    name = "value-0"
    routing_configuration = "value-0"
}
state_machine_alias_1 = provider.sfn.State_machine_alias {
    name = "value-1"
    routing_configuration = "value-1"
}
state_machine_alias_2 = provider.sfn.State_machine_alias {
    name = "value-2"
    routing_configuration = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    state_machine_alias = provider.sfn.State_machine_alias {
        name = "production-value"
        routing_configuration = "production-value"
    }
```

---

## Related Documentation

- [AWS Sfn Documentation](https://docs.aws.amazon.com/sfn/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
