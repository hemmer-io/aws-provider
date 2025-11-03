# Chatbot Service



**Resources**: 12

---

## Overview

The chatbot service provides access to 12 resource types:

- [Account_preferences](#account_preferences) [RU]
- [Microsoft_teams_user_identity](#microsoft_teams_user_identity) [D]
- [Slack_user_identities](#slack_user_identities) [R]
- [Slack_channel_configuration](#slack_channel_configuration) [CUD]
- [Slack_workspace_authorization](#slack_workspace_authorization) [D]
- [Microsoft_teams_channel_configuration](#microsoft_teams_channel_configuration) [CRUD]
- [Chime_webhook_configurations](#chime_webhook_configurations) [R]
- [Microsoft_teams_configured_team](#microsoft_teams_configured_team) [D]
- [Chime_webhook_configuration](#chime_webhook_configuration) [CUD]
- [Slack_user_identity](#slack_user_identity) [D]
- [Slack_channel_configurations](#slack_channel_configurations) [R]
- [Slack_workspaces](#slack_workspaces) [R]

---

## Resources


### Account_preferences

AccountPreferences resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_authorization_required` | bool |  | <p>Enables use of a user role requirement in your chat configuration.</p> |
| `training_data_collection_enabled` | bool |  | <p>Turns on training data collection.</p>
         <p>This helps improve the AWS Chatbot experience by allowing AWS Chatbot to store and use your customer information, such as AWS Chatbot configurations, notifications, user inputs, AWS Chatbot generated responses, and interaction data. This data helps us to continuously improve and develop Artificial Intelligence (AI) technologies. Your data is not shared with any third parties and is protected using sophisticated controls to prevent unauthorized access and misuse. AWS Chatbot does not store or use interactions in chat channels with Amazon Q for training AI technologies for AWS Chatbot.
  </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_preferences` | String | <p>The preferences related to AWS Chatbot usage in the calling AWS account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_preferences outputs
account_preferences_id = account_preferences.id
account_preferences_account_preferences = account_preferences.account_preferences
```

---


### Microsoft_teams_user_identity

MicrosoftTeamsUserIdentity resource

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


### Slack_user_identities

SlackUserIdentities resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p> An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by MaxResults.
  </p> |
| `slack_user_identities` | Vec<String> | <p>A list of Slack User Identities.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access slack_user_identities outputs
slack_user_identities_id = slack_user_identities.id
slack_user_identities_next_token = slack_user_identities.next_token
slack_user_identities_slack_user_identities = slack_user_identities.slack_user_identities
```

---


### Slack_channel_configuration

SlackChannelConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_level` | String |  | <p>Logging levels include <code>ERROR</code>, <code>INFO</code>, or <code>NONE</code>.</p> |
| `configuration_name` | String | ✅ | <p>The name of the configuration.</p> |
| `guardrail_policy_arns` | Vec<String> |  | <p>The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed <code>AdministratorAccess</code> policy is applied by default if this is not set.
  </p> |
| `slack_channel_id` | String | ✅ | <p>The ID of the Slack channel.</p>
         <p>To get this ID, open Slack, right click on the channel name in the left pane, then choose Copy Link. The channel ID is the 9-character string at the end of the URL. For example, ABCBBLZZZ.
  </p> |
| `slack_channel_name` | String |  | <p>The name of the Slack channel.</p> |
| `sns_topic_arns` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) of the SNS topics that deliver notifications to AWS Chatbot.</p> |
| `tags` | Vec<String> |  | <p>A map of tags assigned to a resource. A tag is a string-to-string map of key-value pairs.</p> |
| `iam_role_arn` | String | ✅ | <p>A user-defined role that AWS Chatbot assumes. This is not the service-linked role.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chatbot-iam-policies.html">IAM policies for AWS Chatbot</a> in the <i> AWS Chatbot Administrator Guide</i>.
  </p> |
| `slack_team_id` | String | ✅ | <p>The ID of the Slack workspace authorized with AWS Chatbot.</p> |
| `user_authorization_required` | bool |  | <p>Enables use of a user role requirement in your chat configuration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create slack_channel_configuration
slack_channel_configuration = provider.chatbot.Slack_channel_configuration {
    configuration_name = "value"  # <p>The name of the configuration.</p>
    slack_channel_id = "value"  # <p>The ID of the Slack channel.</p>
         <p>To get this ID, open Slack, right click on the channel name in the left pane, then choose Copy Link. The channel ID is the 9-character string at the end of the URL. For example, ABCBBLZZZ.
  </p>
    iam_role_arn = "value"  # <p>A user-defined role that AWS Chatbot assumes. This is not the service-linked role.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chatbot-iam-policies.html">IAM policies for AWS Chatbot</a> in the <i> AWS Chatbot Administrator Guide</i>.
  </p>
    slack_team_id = "value"  # <p>The ID of the Slack workspace authorized with AWS Chatbot.</p>
}

```

---


### Slack_workspace_authorization

SlackWorkspaceAuthorization resource

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


### Microsoft_teams_channel_configuration

MicrosoftTeamsChannelConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_level` | String |  | <p>Logging levels include <code>ERROR</code>, <code>INFO</code>, or <code>NONE</code>.</p> |
| `channel_name` | String |  | <p>The name of the Microsoft Teams channel.</p> |
| `sns_topic_arns` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) of the SNS topics that deliver notifications to AWS Chatbot.</p> |
| `guardrail_policy_arns` | Vec<String> |  | <p>The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed <code>AdministratorAccess</code> policy is applied by default if this is not set.
  </p> |
| `iam_role_arn` | String | ✅ | <p>A user-defined role that AWS Chatbot assumes. This is not the service-linked role.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chatbot-iam-policies.html">IAM policies for AWS Chatbot</a> in the <i> AWS Chatbot Administrator Guide</i>.
  </p> |
| `tenant_id` | String | ✅ | <p>The ID of the Microsoft Teams tenant.</p> |
| `configuration_name` | String | ✅ | <p>The name of the configuration.</p> |
| `tags` | Vec<String> |  | <p>A map of tags assigned to a resource. A tag is a string-to-string map of key-value pairs.</p> |
| `channel_id` | String | ✅ | <p>The ID of the Microsoft Teams channel.</p> |
| `team_id` | String | ✅ | <p> The ID of the Microsoft Teams authorized with AWS Chatbot.</p>
         <p>To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/teams-setup.html#teams-client-setup">Step 1: Configure a Microsoft Teams client</a> in the <i> AWS Chatbot Administrator Guide</i>.
</p> |
| `team_name` | String |  | <p>The name of the Microsoft Teams Team.</p> |
| `user_authorization_required` | bool |  | <p>Enables use of a user role requirement in your chat configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_configuration` | String | <p>The configuration for a Microsoft Teams channel configured with AWS Chatbot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create microsoft_teams_channel_configuration
microsoft_teams_channel_configuration = provider.chatbot.Microsoft_teams_channel_configuration {
    iam_role_arn = "value"  # <p>A user-defined role that AWS Chatbot assumes. This is not the service-linked role.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chatbot-iam-policies.html">IAM policies for AWS Chatbot</a> in the <i> AWS Chatbot Administrator Guide</i>.
  </p>
    tenant_id = "value"  # <p>The ID of the Microsoft Teams tenant.</p>
    configuration_name = "value"  # <p>The name of the configuration.</p>
    channel_id = "value"  # <p>The ID of the Microsoft Teams channel.</p>
    team_id = "value"  # <p> The ID of the Microsoft Teams authorized with AWS Chatbot.</p>
         <p>To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/teams-setup.html#teams-client-setup">Step 1: Configure a Microsoft Teams client</a> in the <i> AWS Chatbot Administrator Guide</i>.
</p>
}

# Access microsoft_teams_channel_configuration outputs
microsoft_teams_channel_configuration_id = microsoft_teams_channel_configuration.id
microsoft_teams_channel_configuration_channel_configuration = microsoft_teams_channel_configuration.channel_configuration
```

---


### Chime_webhook_configurations

ChimeWebhookConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by MaxResults.
 </p> |
| `webhook_configurations` | Vec<String> | <p>A list of Amazon Chime webhooks associated with the account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access chime_webhook_configurations outputs
chime_webhook_configurations_id = chime_webhook_configurations.id
chime_webhook_configurations_next_token = chime_webhook_configurations.next_token
chime_webhook_configurations_webhook_configurations = chime_webhook_configurations.webhook_configurations
```

---


### Microsoft_teams_configured_team

MicrosoftTeamsConfiguredTeam resource

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


### Chime_webhook_configuration

ChimeWebhookConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sns_topic_arns` | Vec<String> | ✅ | <p>The Amazon Resource Names (ARNs) of the SNS topics that deliver notifications to AWS Chatbot.</p> |
| `iam_role_arn` | String | ✅ | <p>A user-defined role that AWS Chatbot assumes. This is not the service-linked role.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chatbot-iam-policies.html">IAM policies for AWS Chatbot</a> in the <i> AWS Chatbot Administrator Guide</i>.
  </p> |
| `webhook_url` | String | ✅ | <p>The URL for the Amazon Chime webhook.</p> |
| `configuration_name` | String | ✅ | <p>The name of the configuration.</p> |
| `logging_level` | String |  | <p>Logging levels include <code>ERROR</code>, <code>INFO</code>, or <code>NONE</code>.</p> |
| `tags` | Vec<String> |  | <p>A map of tags assigned to a resource. A tag is a string-to-string map of key-value pairs.</p> |
| `webhook_description` | String | ✅ | <p>A description of the webhook. We recommend using the convention <code>RoomName/WebhookName</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chime-setup.html">Tutorial: Get started with Amazon Chime</a> in the <i> AWS Chatbot Administrator Guide</i>.
 </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create chime_webhook_configuration
chime_webhook_configuration = provider.chatbot.Chime_webhook_configuration {
    sns_topic_arns = "value"  # <p>The Amazon Resource Names (ARNs) of the SNS topics that deliver notifications to AWS Chatbot.</p>
    iam_role_arn = "value"  # <p>A user-defined role that AWS Chatbot assumes. This is not the service-linked role.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chatbot-iam-policies.html">IAM policies for AWS Chatbot</a> in the <i> AWS Chatbot Administrator Guide</i>.
  </p>
    webhook_url = "value"  # <p>The URL for the Amazon Chime webhook.</p>
    configuration_name = "value"  # <p>The name of the configuration.</p>
    webhook_description = "value"  # <p>A description of the webhook. We recommend using the convention <code>RoomName/WebhookName</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/chime-setup.html">Tutorial: Get started with Amazon Chime</a> in the <i> AWS Chatbot Administrator Guide</i>.
 </p>
}

```

---


### Slack_user_identity

SlackUserIdentity resource

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


### Slack_channel_configurations

SlackChannelConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `slack_channel_configurations` | Vec<String> | <p>A list of Slack channel configurations.</p> |
| `next_token` | String | <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by MaxResults.
  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access slack_channel_configurations outputs
slack_channel_configurations_id = slack_channel_configurations.id
slack_channel_configurations_slack_channel_configurations = slack_channel_configurations.slack_channel_configurations
slack_channel_configurations_next_token = slack_channel_configurations.next_token
```

---


### Slack_workspaces

SlackWorkspaces resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `slack_workspaces` | Vec<String> | <p>A list of Slack workspaces registered with AWS Chatbot.</p> |
| `next_token` | String | <p> An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by MaxResults.
  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access slack_workspaces outputs
slack_workspaces_id = slack_workspaces.id
slack_workspaces_slack_workspaces = slack_workspaces.slack_workspaces
slack_workspaces_next_token = slack_workspaces.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple account_preferences resources
account_preferences_0 = provider.chatbot.Account_preferences {
}
account_preferences_1 = provider.chatbot.Account_preferences {
}
account_preferences_2 = provider.chatbot.Account_preferences {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_preferences = provider.chatbot.Account_preferences {
    }
```

---

## Related Documentation

- [AWS Chatbot Documentation](https://docs.aws.amazon.com/chatbot/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
