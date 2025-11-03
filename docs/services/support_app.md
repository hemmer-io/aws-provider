# Support_app Service



**Resources**: 3

---

## Overview

The support_app service provides access to 3 resource types:

- [Slack_channel_configuration](#slack_channel_configuration) [CUD]
- [Account_alias](#account_alias) [CRD]
- [Slack_workspace_configuration](#slack_workspace_configuration) [D]

---

## Resources


### Slack_channel_configuration

SlackChannelConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notify_on_create_or_reopen_case` | bool |  | <p>Whether you want to get notified when a support case is created or reopened.</p> |
| `channel_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that you want to
use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to
the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p> |
| `team_id` | String | ✅ | <p>The team ID in Slack. This ID uniquely identifies a Slack workspace, such as
<code>T012ABCDEFG</code>.</p> |
| `notify_on_add_correspondence_to_case` | bool |  | <p>Whether you want to get notified when a support case has a new correspondence.</p> |
| `channel_name` | String |  | <p>The name of the Slack channel that you configure for the Amazon Web Services Support App.</p> |
| `channel_id` | String | ✅ | <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p> |
| `notify_on_case_severity` | String | ✅ | <p>The case severity for a support case that you want to receive notifications.</p>
         <p>If you specify <code>high</code> or <code>all</code>, you must specify <code>true</code>
      for at least one of the following parameters:</p>
         <ul>
            <li>
               <p>
                  <code>notifyOnAddCorrespondenceToCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnCreateOrReopenCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnResolveCase</code>
               </p>
            </li>
         </ul>
         <p>If you specify <code>none</code>, the following parameters must be null or
        <code>false</code>:</p>
         <ul>
            <li>
               <p>
                  <code>notifyOnAddCorrespondenceToCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnCreateOrReopenCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnResolveCase</code>
               </p>
            </li>
         </ul>
         <note>
            <p>If you don't specify these parameters in your request, they default to
          <code>false</code>.</p>
         </note> |
| `notify_on_resolve_case` | bool |  | <p>Whether you want to get notified when a support case is resolved.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create slack_channel_configuration
slack_channel_configuration = provider.support_app.Slack_channel_configuration {
    channel_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that you want to
use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to
the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
    team_id = "value"  # <p>The team ID in Slack. This ID uniquely identifies a Slack workspace, such as
<code>T012ABCDEFG</code>.</p>
    channel_id = "value"  # <p>The channel ID in Slack. This ID identifies a channel within a Slack workspace.</p>
    notify_on_case_severity = "value"  # <p>The case severity for a support case that you want to receive notifications.</p>
         <p>If you specify <code>high</code> or <code>all</code>, you must specify <code>true</code>
      for at least one of the following parameters:</p>
         <ul>
            <li>
               <p>
                  <code>notifyOnAddCorrespondenceToCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnCreateOrReopenCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnResolveCase</code>
               </p>
            </li>
         </ul>
         <p>If you specify <code>none</code>, the following parameters must be null or
        <code>false</code>:</p>
         <ul>
            <li>
               <p>
                  <code>notifyOnAddCorrespondenceToCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnCreateOrReopenCase</code>
               </p>
            </li>
            <li>
               <p>
                  <code>notifyOnResolveCase</code>
               </p>
            </li>
         </ul>
         <note>
            <p>If you don't specify these parameters in your request, they default to
          <code>false</code>.</p>
         </note>
}

```

---


### Account_alias

AccountAlias resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_alias` | String | ✅ | <p>An alias or short name for an Amazon Web Services account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_alias` | String | <p>An alias or short name for an Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_alias
account_alias = provider.support_app.Account_alias {
    account_alias = "value"  # <p>An alias or short name for an Amazon Web Services account.</p>
}

# Access account_alias outputs
account_alias_id = account_alias.id
account_alias_account_alias = account_alias.account_alias
```

---


### Slack_workspace_configuration

SlackWorkspaceConfiguration resource

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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple slack_channel_configuration resources
slack_channel_configuration_0 = provider.support_app.Slack_channel_configuration {
    channel_role_arn = "value-0"
    team_id = "value-0"
    channel_id = "value-0"
    notify_on_case_severity = "value-0"
}
slack_channel_configuration_1 = provider.support_app.Slack_channel_configuration {
    channel_role_arn = "value-1"
    team_id = "value-1"
    channel_id = "value-1"
    notify_on_case_severity = "value-1"
}
slack_channel_configuration_2 = provider.support_app.Slack_channel_configuration {
    channel_role_arn = "value-2"
    team_id = "value-2"
    channel_id = "value-2"
    notify_on_case_severity = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    slack_channel_configuration = provider.support_app.Slack_channel_configuration {
        channel_role_arn = "production-value"
        team_id = "production-value"
        channel_id = "production-value"
        notify_on_case_severity = "production-value"
    }
```

---

## Related Documentation

- [AWS Support_app Documentation](https://docs.aws.amazon.com/support_app/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
