# Detective Service



**Resources**: 6

---

## Overview

The detective service provides access to 6 resource types:

- [Investigation_state](#investigation_state) [U]
- [Datasource_packages](#datasource_packages) [U]
- [Organization_configuration](#organization_configuration) [RU]
- [Members](#members) [CRD]
- [Graph](#graph) [CD]
- [Investigation](#investigation) [R]

---

## Resources


### Investigation_state

InvestigationState resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `investigation_id` | String | ✅ | <p>The investigation ID of the investigation report.</p> |
| `state` | String | ✅ | <p>The current state of the investigation. An archived investigation indicates you have completed reviewing the investigation.</p> |
| `graph_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the behavior graph.</p> |



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


### Datasource_packages

DatasourcePackages resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `graph_arn` | String | ✅ | <p>The ARN of the behavior graph.</p> |
| `datasource_packages` | Vec<String> | ✅ | <p>The data source package to start for the behavior graph.</p> |



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


### Organization_configuration

OrganizationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_enable` | bool |  | <p>Indicates whether to automatically enable new organization accounts as member accounts
         in the organization behavior graph.</p> |
| `graph_arn` | String | ✅ | <p>The ARN of the organization behavior graph.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_enable` | bool | <p>Indicates whether to automatically enable new organization accounts as member accounts
         in the organization behavior graph.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_configuration outputs
organization_configuration_id = organization_configuration.id
organization_configuration_auto_enable = organization_configuration.auto_enable
```

---


### Members

Members resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disable_email_notification` | bool |  | <p>if set to <code>true</code>, then the invited accounts do not receive email
         notifications. By default, this is set to <code>false</code>, and the invited accounts
         receive email notifications.</p>
         <p>Organization accounts in the organization behavior graph do not receive email
         notifications.</p> |
| `accounts` | Vec<String> | ✅ | <p>The list of Amazon Web Services accounts to invite or to enable. You can invite or enable
         up to 50 accounts at a time. For each invited account, the account list contains the
         account identifier and the Amazon Web Services account root user email address. For
         organization accounts in the organization behavior graph, the email address is not
         required.</p> |
| `graph_arn` | String | ✅ | <p>The ARN of the behavior graph.</p> |
| `message` | String |  | <p>Customized message text to include in the invitation email message to the invited member
         accounts.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unprocessed_accounts` | Vec<String> | <p>The requested member accounts for which Detective was unable to return member
         details.</p>
         <p>For each account, provides the reason why the request could not be processed.</p> |
| `member_details` | Vec<String> | <p>The member account details that Detective is returning in response to the
         request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create members
members = provider.detective.Members {
    accounts = "value"  # <p>The list of Amazon Web Services accounts to invite or to enable. You can invite or enable
         up to 50 accounts at a time. For each invited account, the account list contains the
         account identifier and the Amazon Web Services account root user email address. For
         organization accounts in the organization behavior graph, the email address is not
         required.</p>
    graph_arn = "value"  # <p>The ARN of the behavior graph.</p>
}

# Access members outputs
members_id = members.id
members_unprocessed_accounts = members.unprocessed_accounts
members_member_details = members.member_details
```

---


### Graph

Graph resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags to assign to the new behavior graph. You can add up to 50 tags. For each tag,
         you provide the tag key and the tag value. Each tag key can contain up to 128 characters.
         Each tag value can contain up to 256 characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create graph
graph = provider.detective.Graph {
}

```

---


### Investigation

Investigation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scope_start_time` | String | <p>The start date and time used to set the scope time within which you want to generate the investigation report. The value is an UTC ISO8601 formatted
         string. For example, <code>2021-08-18T16:35:56.284Z</code>.</p> |
| `entity_arn` | String | <p>The unique Amazon Resource Name (ARN). Detective supports IAM user ARNs and IAM role ARNs.</p> |
| `scope_end_time` | String | <p>The data and time when the investigation began. The value is an UTC ISO8601 formatted
         string. For example, <code>2021-08-18T16:35:56.284Z</code>.</p> |
| `created_time` | String | <p>The creation time of the investigation report in UTC time stamp format.</p> |
| `graph_arn` | String | <p>The Amazon Resource Name (ARN) of the behavior graph.</p> |
| `status` | String | <p>The status based on the completion status of the investigation.</p> |
| `investigation_id` | String | <p>The investigation ID of the investigation report.</p> |
| `severity` | String | <p>The severity assigned is based on the likelihood and impact of the indicators of
         compromise discovered in the investigation.</p> |
| `entity_type` | String | <p>Type of entity. For example, Amazon Web Services accounts, such as an IAM user and/or IAM role.</p> |
| `state` | String | <p>The current state of the investigation. An archived investigation indicates that you
         have completed reviewing the investigation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access investigation outputs
investigation_id = investigation.id
investigation_scope_start_time = investigation.scope_start_time
investigation_entity_arn = investigation.entity_arn
investigation_scope_end_time = investigation.scope_end_time
investigation_created_time = investigation.created_time
investigation_graph_arn = investigation.graph_arn
investigation_status = investigation.status
investigation_investigation_id = investigation.investigation_id
investigation_severity = investigation.severity
investigation_entity_type = investigation.entity_type
investigation_state = investigation.state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple investigation_state resources
investigation_state_0 = provider.detective.Investigation_state {
    investigation_id = "value-0"
    state = "value-0"
    graph_arn = "value-0"
}
investigation_state_1 = provider.detective.Investigation_state {
    investigation_id = "value-1"
    state = "value-1"
    graph_arn = "value-1"
}
investigation_state_2 = provider.detective.Investigation_state {
    investigation_id = "value-2"
    state = "value-2"
    graph_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    investigation_state = provider.detective.Investigation_state {
        investigation_id = "production-value"
        state = "production-value"
        graph_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Detective Documentation](https://docs.aws.amazon.com/detective/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
