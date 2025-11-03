# Support_app Service



**Resources**: 3

---

## Overview

The support_app service provides access to 3 resource types:

- [Account_alias](#account_alias) [CRD]
- [Slack_channel_configuration](#slack_channel_configuration) [CUD]
- [Slack_workspace_configuration](#slack_workspace_configuration) [D]

---

## Resources


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


### Slack_channel_configuration

SlackChannelConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
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
| `channel_name` | String |  | <p>The name of the Slack channel that you configure for the Amazon Web Services Support App.</p> |
| `team_id` | String | ✅ | <p>The team ID in Slack. This ID uniquely identifies a Slack workspace, such as
<code>T012ABCDEFG</code>.</p> |
| `notify_on_resolve_case` | bool |  | <p>Whether you want to get notified when a support case is resolved.</p> |
| `notify_on_create_or_reopen_case` | bool |  | <p>Whether you want to get notified when a support case is created or reopened.</p> |
| `channel_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that you want to
use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to
the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p> |
| `notify_on_add_correspondence_to_case` | bool |  | <p>Whether you want to get notified when a support case has a new correspondence.</p> |



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
    team_id = "value"  # <p>The team ID in Slack. This ID uniquely identifies a Slack workspace, such as
<code>T012ABCDEFG</code>.</p>
    channel_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that you want to
use to perform operations on Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-permissions.html">Managing access to
the Amazon Web Services Support App</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
}

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

# Create multiple account_alias resources
account_alias_0 = provider.support_app.Account_alias {
    account_alias = "value-0"
}
account_alias_1 = provider.support_app.Account_alias {
    account_alias = "value-1"
}
account_alias_2 = provider.support_app.Account_alias {
    account_alias = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_alias = provider.support_app.Account_alias {
        account_alias = "production-value"
    }
```

---

## Related Documentation

- [AWS Support_app Documentation](https://docs.aws.amazon.com/support_app/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
