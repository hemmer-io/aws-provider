# Oam Service



**Resources**: 3

---

## Overview

The oam service provides access to 3 resource types:

- [Sink_policy](#sink_policy) [CR]
- [Link](#link) [CRUD]
- [Sink](#sink) [CRD]

---

## Resources


### Sink_policy

SinkPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sink_identifier` | String | ✅ | <p>The ARN of the sink to attach this policy to.</p> |
| `policy` | String | ✅ | <p>The JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.</p> <p>The policy must be in JSON string format with quotation marks escaped and no newlines.</p> <p>For examples of different types of policies, see the <b>Examples</b> section on this page.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The policy that you specified, in JSON format.</p> |
| `sink_id` | String | <p>The random ID string that Amazon Web Services generated as part of the sink ARN.</p> |
| `sink_arn` | String | <p>The ARN of the sink.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sink_policy
sink_policy = provider.oam.Sink_policy {
    sink_identifier = "value"  # <p>The ARN of the sink to attach this policy to.</p>
    policy = "value"  # <p>The JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.</p> <p>The policy must be in JSON string format with quotation marks escaped and no newlines.</p> <p>For examples of different types of policies, see the <b>Examples</b> section on this page.</p>
}

# Access sink_policy outputs
sink_policy_id = sink_policy.id
sink_policy_policy = sink_policy.policy
sink_policy_sink_id = sink_policy.sink_id
sink_policy_sink_arn = sink_policy.sink_arn
```

---


### Link

Link resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `link_configuration` | String |  | <p>Use this structure to optionally create filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account.</p> |
| `label_template` | String | ✅ | <p>Specify a friendly human-readable name to use to identify this source account when you are viewing data from it in the monitoring account.</p> <p>You can use a custom label or use the following variables:</p> <ul> <li> <p> <code>$AccountName</code> is the name of the account</p> </li> <li> <p> <code>$AccountEmail</code> is the globally unique email address of the account</p> </li> <li> <p> <code>$AccountEmailNoDomain</code> is the email address of the account without the domain name</p> </li> </ul> <note> <p>In the Amazon Web Services GovCloud (US-East) and Amazon Web Services GovCloud (US-West) Regions, the only supported option is to use custom labels, and the <code>$AccountName</code>, <code>$AccountEmail</code>, and <code>$AccountEmailNoDomain</code> variables all resolve as <i>account-id</i> instead of the specified variable.</p> </note> |
| `sink_identifier` | String | ✅ | <p>The ARN of the sink to use to create this link. You can use <a href="https://docs.aws.amazon.com/OAM/latest/APIReference/API_ListSinks.html">ListSinks</a> to find the ARNs of sinks.</p> <p>For more information about sinks, see <a href="https://docs.aws.amazon.com/OAM/latest/APIReference/API_CreateSink.html">CreateSink</a>.</p> |
| `tags` | HashMap<String, String> |  | <p>Assigns one or more tags (key-value pairs) to the link. </p> <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p> <p>For more information about using tags to control access, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html">Controlling access to Amazon Web Services resources using tags</a>.</p> |
| `resource_types` | Vec<String> | ✅ | <p>An array of strings that define which types of data that the source account shares with the monitoring account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `link_configuration` | String | <p>This structure includes filters that specify which metric namespaces and which log groups are shared from the source account to the monitoring account.</p> |
| `arn` | String | <p>The ARN of the link.</p> |
| `sink_arn` | String | <p>The ARN of the sink that is used for this link.</p> |
| `resource_types` | Vec<String> | <p>The resource types supported by this link.</p> |
| `tags` | HashMap<String, String> | <p>The tags assigned to the link.</p> |
| `id` | String | <p>The random ID string that Amazon Web Services generated as part of the link ARN.</p> |
| `label` | String | <p>The label that you assigned to this link, with the variables resolved to their actual values.</p> |
| `label_template` | String | <p>The exact label template that was specified when the link was created, with the template variables not resolved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create link
link = provider.oam.Link {
    label_template = "value"  # <p>Specify a friendly human-readable name to use to identify this source account when you are viewing data from it in the monitoring account.</p> <p>You can use a custom label or use the following variables:</p> <ul> <li> <p> <code>$AccountName</code> is the name of the account</p> </li> <li> <p> <code>$AccountEmail</code> is the globally unique email address of the account</p> </li> <li> <p> <code>$AccountEmailNoDomain</code> is the email address of the account without the domain name</p> </li> </ul> <note> <p>In the Amazon Web Services GovCloud (US-East) and Amazon Web Services GovCloud (US-West) Regions, the only supported option is to use custom labels, and the <code>$AccountName</code>, <code>$AccountEmail</code>, and <code>$AccountEmailNoDomain</code> variables all resolve as <i>account-id</i> instead of the specified variable.</p> </note>
    sink_identifier = "value"  # <p>The ARN of the sink to use to create this link. You can use <a href="https://docs.aws.amazon.com/OAM/latest/APIReference/API_ListSinks.html">ListSinks</a> to find the ARNs of sinks.</p> <p>For more information about sinks, see <a href="https://docs.aws.amazon.com/OAM/latest/APIReference/API_CreateSink.html">CreateSink</a>.</p>
    resource_types = "value"  # <p>An array of strings that define which types of data that the source account shares with the monitoring account.</p>
}

# Access link outputs
link_id = link.id
link_link_configuration = link.link_configuration
link_arn = link.arn
link_sink_arn = link.sink_arn
link_resource_types = link.resource_types
link_tags = link.tags
link_id = link.id
link_label = link.label
link_label_template = link.label_template
```

---


### Sink

Sink resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Assigns one or more tags (key-value pairs) to the link. </p> <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p> <p>For more information about using tags to control access, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html">Controlling access to Amazon Web Services resources using tags</a>.</p> |
| `name` | String | ✅ | <p>A name for the sink.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The random ID string that Amazon Web Services generated as part of the sink ARN.</p> |
| `arn` | String | <p>The ARN of the sink.</p> |
| `tags` | HashMap<String, String> | <p>The tags assigned to the sink.</p> |
| `name` | String | <p>The name of the sink.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sink
sink = provider.oam.Sink {
    name = "value"  # <p>A name for the sink.</p>
}

# Access sink outputs
sink_id = sink.id
sink_id = sink.id
sink_arn = sink.arn
sink_tags = sink.tags
sink_name = sink.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple sink_policy resources
sink_policy_0 = provider.oam.Sink_policy {
    sink_identifier = "value-0"
    policy = "value-0"
}
sink_policy_1 = provider.oam.Sink_policy {
    sink_identifier = "value-1"
    policy = "value-1"
}
sink_policy_2 = provider.oam.Sink_policy {
    sink_identifier = "value-2"
    policy = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    sink_policy = provider.oam.Sink_policy {
        sink_identifier = "production-value"
        policy = "production-value"
    }
```

---

## Related Documentation

- [AWS Oam Documentation](https://docs.aws.amazon.com/oam/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
