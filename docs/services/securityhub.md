# Securityhub Service



**Resources**: 36

---

## Overview

The securityhub service provides access to 36 resource types:

- [Action_target](#action_target) [CUD]
- [Master_account](#master_account) [R]
- [Hub](#hub) [R]
- [Automation_rule_v2](#automation_rule_v2) [CRUD]
- [Insight](#insight) [CUD]
- [Invitations_count](#invitations_count) [R]
- [Security_control](#security_control) [U]
- [Findings](#findings) [RU]
- [Products](#products) [R]
- [Connector_v2](#connector_v2) [CRUD]
- [Organization_configuration](#organization_configuration) [RU]
- [Insight_results](#insight_results) [R]
- [Finding_history](#finding_history) [R]
- [Security_control_definition](#security_control_definition) [R]
- [Standards](#standards) [R]
- [Insights](#insights) [R]
- [Resources_v2](#resources_v2) [R]
- [Standards_control](#standards_control) [U]
- [Standards_controls](#standards_controls) [R]
- [Administrator_account](#administrator_account) [R]
- [Resources_statistics_v2](#resources_statistics_v2) [R]
- [Ticket_v2](#ticket_v2) [C]
- [Action_targets](#action_targets) [R]
- [Enabled_standards](#enabled_standards) [R]
- [Finding_statistics_v2](#finding_statistics_v2) [R]
- [Aggregator_v2](#aggregator_v2) [CRUD]
- [Findings_v2](#findings_v2) [R]
- [Invitations](#invitations) [D]
- [Products_v2](#products_v2) [R]
- [Configuration_policy](#configuration_policy) [CRUD]
- [Security_hub_configuration](#security_hub_configuration) [U]
- [Finding_aggregator](#finding_aggregator) [CRUD]
- [Automation_rule](#automation_rule) [C]
- [Members](#members) [CRD]
- [Security_hub_v2](#security_hub_v2) [R]
- [Configuration_policy_association](#configuration_policy_association) [R]

---

## Resources


### Action_target

ActionTarget resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>The description for the custom action target.</p> |
| `name` | String | ✅ | <p>The name of the custom action target. Can contain up to 20 characters.</p> |
| `id` | String | ✅ | <p>The ID for the custom action target. Can contain up to 20 alphanumeric characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create action_target
action_target = provider.securityhub.Action_target {
    description = "value"  # <p>The description for the custom action target.</p>
    name = "value"  # <p>The name of the custom action target. Can contain up to 20 characters.</p>
    id = "value"  # <p>The ID for the custom action target. Can contain up to 20 alphanumeric characters.</p>
}

```

---


### Master_account

MasterAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `master` | String | <p>A list of details about the Security Hub administrator account for the current member account.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access master_account outputs
master_account_id = master_account.id
master_account_master = master_account.master
```

---


### Hub

Hub resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_enable_controls` | bool | <p>Whether to automatically enable new controls when they are added to standards that are
         enabled.</p>
         <p>If set to <code>true</code>, then new controls for enabled standards are enabled
         automatically. If set to <code>false</code>, then new controls are not enabled.</p>
         <p>When you automatically enable new controls, you can interact with the controls in 
           the console and programmatically immediately after release. However, automatically enabled controls have a temporary default status of 
           <code>DISABLED</code>. It can take up to several days for Security Hub to process the control release and designate the 
           control as <code>ENABLED</code> in your account. During the processing period, you can manually enable or disable a 
           control, and Security Hub will maintain that designation regardless of whether you have <code>AutoEnableControls</code> set to 
           <code>true</code>.</p> |
| `control_finding_generator` | String | <p>Specifies whether the calling account has consolidated control findings turned on. If the value for this field is set to 
         <code>SECURITY_CONTROL</code>, Security Hub generates a single finding for a control check even when the check 
         applies to multiple enabled standards.</p>
         <p>If the value for this field is set to <code>STANDARD_CONTROL</code>, Security Hub generates separate findings 
         for a control check when the check applies to multiple enabled standards.</p>
         <p>The value for this field in a member account matches the value in the administrator
         account. For accounts that aren't part of an organization, the default value of this field
         is <code>SECURITY_CONTROL</code> if you enabled Security Hub on or after February 23,
         2023.</p> |
| `hub_arn` | String | <p>The ARN of the Hub resource that was retrieved.</p> |
| `subscribed_at` | String | <p>The date and time when Security Hub was enabled in the account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hub outputs
hub_id = hub.id
hub_auto_enable_controls = hub.auto_enable_controls
hub_control_finding_generator = hub.control_finding_generator
hub_hub_arn = hub.hub_arn
hub_subscribed_at = hub.subscribed_at
```

---


### Automation_rule_v2

AutomationRuleV2 resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `criteria` | String | ✅ | <p>The filtering type and configuration of the automation rule.</p> |
| `client_token` | String |  | <p>A unique identifier used to ensure idempotency.</p> |
| `rule_status` | String |  | <p>The status of the V2 automation rule.</p> |
| `rule_order` | String | ✅ | <p>The value for the rule priority.</p> |
| `description` | String | ✅ | <p>A description of the V2 automation rule.</p> |
| `actions` | Vec<String> | ✅ | <p>A list of actions to be performed when the rule criteria is met.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs associated with the V2 automation rule.</p> |
| `rule_name` | String | ✅ | <p>The name of the V2 automation rule.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp when the V2 automation rule was created.</p> |
| `rule_status` | String | <p>The status of the V2 automation automation rule.</p> |
| `criteria` | String | <p>The filtering type and configuration of the V2 automation rule.</p> |
| `updated_at` | String | <p>The timestamp when the V2 automation rule was updated.</p> |
| `actions` | Vec<String> | <p>A list of actions performed when the rule criteria is met.</p> |
| `rule_order` | String | <p>The value for the rule priority.</p> |
| `rule_id` | String | <p>The ID of the V2 automation rule.</p> |
| `rule_arn` | String | <p>The ARN of the V2 automation rule.</p> |
| `rule_name` | String | <p>The name of the V2 automation rule.</p> |
| `description` | String | <p>A description of the automation rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create automation_rule_v2
automation_rule_v2 = provider.securityhub.Automation_rule_v2 {
    criteria = "value"  # <p>The filtering type and configuration of the automation rule.</p>
    rule_order = "value"  # <p>The value for the rule priority.</p>
    description = "value"  # <p>A description of the V2 automation rule.</p>
    actions = "value"  # <p>A list of actions to be performed when the rule criteria is met.</p>
    rule_name = "value"  # <p>The name of the V2 automation rule.</p>
}

# Access automation_rule_v2 outputs
automation_rule_v2_id = automation_rule_v2.id
automation_rule_v2_created_at = automation_rule_v2.created_at
automation_rule_v2_rule_status = automation_rule_v2.rule_status
automation_rule_v2_criteria = automation_rule_v2.criteria
automation_rule_v2_updated_at = automation_rule_v2.updated_at
automation_rule_v2_actions = automation_rule_v2.actions
automation_rule_v2_rule_order = automation_rule_v2.rule_order
automation_rule_v2_rule_id = automation_rule_v2.rule_id
automation_rule_v2_rule_arn = automation_rule_v2.rule_arn
automation_rule_v2_rule_name = automation_rule_v2.rule_name
automation_rule_v2_description = automation_rule_v2.description
```

---


### Insight

Insight resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the custom insight to create.</p> |
| `group_by_attribute` | String | ✅ | <p>The attribute used to group the findings for the insight. The grouping attribute
         identifies the type of item that the insight applies to. For example, if an insight is
         grouped by resource identifier, then the insight produces a list of resource
         identifiers.</p> |
| `filters` | String | ✅ | <p>One or more attributes used to filter the findings included in the insight. The insight
         only includes findings that match the criteria defined in the filters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create insight
insight = provider.securityhub.Insight {
    name = "value"  # <p>The name of the custom insight to create.</p>
    group_by_attribute = "value"  # <p>The attribute used to group the findings for the insight. The grouping attribute
         identifies the type of item that the insight applies to. For example, if an insight is
         grouped by resource identifier, then the insight produces a list of resource
         identifiers.</p>
    filters = "value"  # <p>One or more attributes used to filter the findings included in the insight. The insight
         only includes findings that match the criteria defined in the filters.</p>
}

```

---


### Invitations_count

InvitationsCount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invitations_count` | i64 | <p>The number of all membership invitations sent to this Security Hub member account, not
         including the currently accepted invitation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access invitations_count outputs
invitations_count_id = invitations_count.id
invitations_count_invitations_count = invitations_count.invitations_count
```

---


### Security_control

SecurityControl resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_update_reason` | String |  | <p>
            The most recent reason for updating the properties of the security control. This field accepts alphanumeric 
characters in addition to white spaces, dashes, and underscores.
        </p> |
| `security_control_id` | String | ✅ | <p>
            The Amazon Resource Name (ARN) or ID of the control to update. 
        </p> |
| `parameters` | HashMap<String, String> | ✅ | <p>
            An object that specifies which security control parameters to update.
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


### Findings

Findings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `record_state` | String |  | <p>The updated record state for the finding.</p> |
| `filters` | String | ✅ | <p>A collection of attributes that specify which findings you want to update.</p> |
| `note` | String |  | <p>The updated note for the finding.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token to use to request the next page of results.</p> |
| `findings` | Vec<String> | <p>The findings that matched the filters specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings outputs
findings_id = findings.id
findings_next_token = findings.next_token
findings_findings = findings.findings
```

---


### Products

Products resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `products` | Vec<String> | <p>A list of products, including details for each product.</p> |
| `next_token` | String | <p>The pagination token to use to request the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access products outputs
products_id = products.id
products_products = products.products
products_next_token = products.next_token
```

---


### Connector_v2

ConnectorV2 resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The unique name of the connectorV2.</p> |
| `provider` | String | ✅ | <p>The third-party provider’s service configuration.</p> |
| `kms_key_arn` | String |  | <p>The Amazon Resource Name (ARN) of KMS key used to encrypt secrets for the connectorV2.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to add to the connectorV2 when you create.</p> |
| `client_token` | String |  | <p>A unique identifier used to ensure idempotency.</p> |
| `description` | String |  | <p>The description of the connectorV2.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connector_arn` | String | <p>The Amazon Resource Name (ARN) of the connectorV2.</p> |
| `created_at` | String | <p>ISO 8601 UTC timestamp for the time create the connectorV2.</p> |
| `name` | String | <p>The name of the connectorV2.</p> |
| `provider_detail` | String | <p>The third-party provider detail for a service configuration.</p> |
| `health` | String | <p>The current health status for connectorV2</p> |
| `description` | String | <p>The description of the connectorV2.</p> |
| `kms_key_arn` | String | <p>The Amazon Resource Name (ARN) of KMS key used for the connectorV2.</p> |
| `last_updated_at` | String | <p>ISO 8601 UTC timestamp for the time update the connectorV2 connectorStatus.</p> |
| `connector_id` | String | <p>The UUID of the connectorV2 to identify connectorV2 resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connector_v2
connector_v2 = provider.securityhub.Connector_v2 {
    name = "value"  # <p>The unique name of the connectorV2.</p>
    provider = "value"  # <p>The third-party provider’s service configuration.</p>
}

# Access connector_v2 outputs
connector_v2_id = connector_v2.id
connector_v2_connector_arn = connector_v2.connector_arn
connector_v2_created_at = connector_v2.created_at
connector_v2_name = connector_v2.name
connector_v2_provider_detail = connector_v2.provider_detail
connector_v2_health = connector_v2.health
connector_v2_description = connector_v2.description
connector_v2_kms_key_arn = connector_v2.kms_key_arn
connector_v2_last_updated_at = connector_v2.last_updated_at
connector_v2_connector_id = connector_v2.connector_id
```

---


### Organization_configuration

OrganizationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_configuration` | String |  |  |
| `auto_enable` | bool | ✅ | <p>Whether to automatically enable Security Hub in new member accounts when they join the organization.</p>
         <p>If set to <code>true</code>, then Security Hub is automatically enabled in new accounts. If set to <code>false</code>,
           then Security Hub isn't enabled in new accounts automatically. The default value is <code>false</code>.</p>
         <p>If the <code>ConfigurationType</code> of your organization is set to <code>CENTRAL</code>, then this field is set 
           to <code>false</code> and can't be changed in the home Region and linked Regions. However, in that case, the delegated administrator can create a configuration 
            policy in which Security Hub is enabled and associate the policy with new organization accounts.</p> |
| `auto_enable_standards` | String |  | <p>Whether to automatically enable Security Hub <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-enable-disable.html">default standards</a> 
           in new member accounts when they join the organization.</p>
         <p>The default value of this parameter is equal to <code>DEFAULT</code>.</p>
         <p>If equal to <code>DEFAULT</code>, then Security Hub default standards are automatically enabled for new member 
           accounts. If equal to <code>NONE</code>, then default standards are not automatically enabled for new member 
           accounts.</p>
         <p>If the <code>ConfigurationType</code> of your organization is set to <code>CENTRAL</code>, then this field is set 
           to <code>NONE</code> and can't be changed in the home Region and linked Regions. However, in that case, the delegated administrator can create a configuration 
            policy in which specific security standards are enabled and associate the policy with new organization accounts.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_configuration` | String |  |
| `auto_enable_standards` | String | <p>Whether to automatically enable Security Hub <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-enable-disable.html">default standards</a> 
         in new member accounts when they join the organization.</p>
         <p>If equal to <code>DEFAULT</code>, then Security Hub default standards are automatically enabled for new member 
         accounts. If equal to <code>NONE</code>, then default standards are not automatically enabled for new member 
          accounts. The default value of this parameter is equal to <code>DEFAULT</code>.</p>
         <p>If the <code>ConfigurationType</code> of your organization is set to <code>CENTRAL</code>, then this field is set 
           to <code>NONE</code> and can't be changed in the home Region and linked Regions. However, in that case, the delegated administrator can create a configuration 
            policy in which specific security standards are enabled and associate the policy with new organization accounts.</p> |
| `auto_enable` | bool | <p>Whether to automatically enable Security Hub in new member accounts when they join the organization.</p>
         <p>If set to <code>true</code>, then Security Hub is automatically enabled in new accounts. If set to <code>false</code>,
           then Security Hub isn't enabled in new accounts automatically. The default value is <code>false</code>.</p>
         <p>If the <code>ConfigurationType</code> of your organization is set to <code>CENTRAL</code>, then this field is set 
           to <code>false</code> and can't be changed in the home Region and linked Regions. However, in that case, the delegated administrator can create a configuration 
            policy in which Security Hub is enabled and associate the policy with new organization accounts.</p> |
| `member_account_limit_reached` | bool | <p>Whether the maximum number of allowed member accounts are already associated with the
         Security Hub administrator account.</p> |


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
organization_configuration_organization_configuration = organization_configuration.organization_configuration
organization_configuration_auto_enable_standards = organization_configuration.auto_enable_standards
organization_configuration_auto_enable = organization_configuration.auto_enable
organization_configuration_member_account_limit_reached = organization_configuration.member_account_limit_reached
```

---


### Insight_results

InsightResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insight_results` | String | <p>The insight results returned by the operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight_results outputs
insight_results_id = insight_results.id
insight_results_insight_results = insight_results.insight_results
```

---


### Finding_history

FindingHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>
         A token for pagination purposes. Provide this token in the subsequent request to <code>GetFindingsHistory</code> to 
         get up to an additional 100 results of history for the same finding that you specified in your initial request.
      </p> |
| `records` | Vec<String> | <p>
         A list of events that altered the specified finding during the specified time period. 
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access finding_history outputs
finding_history_id = finding_history.id
finding_history_next_token = finding_history.next_token
finding_history_records = finding_history.records
```

---


### Security_control_definition

SecurityControlDefinition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_control_definition` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_control_definition outputs
security_control_definition_id = security_control_definition.id
security_control_definition_security_control_definition = security_control_definition.security_control_definition
```

---


### Standards

Standards resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token to use to request the next page of results.</p> |
| `standards` | Vec<String> | <p>A list of available standards.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access standards outputs
standards_id = standards.id
standards_next_token = standards.next_token
standards_standards = standards.standards
```

---


### Insights

Insights resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token to use to request the next page of results.</p> |
| `insights` | Vec<String> | <p>The insights returned by the operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insights outputs
insights_id = insights.id
insights_next_token = insights.next_token
insights_insights = insights.insights
```

---


### Resources_v2

ResourcesV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resources` | Vec<String> | <p>Filters resources based on a set of criteria.</p> |
| `next_token` | String | <p>The pagination token to use to request the next page of results. 
         Otherwise, this parameter is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resources_v2 outputs
resources_v2_id = resources_v2.id
resources_v2_resources = resources_v2.resources
resources_v2_next_token = resources_v2.next_token
```

---


### Standards_control

StandardsControl resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `control_status` | String |  | <p>The updated status of the security standard control.</p> |
| `standards_control_arn` | String | ✅ | <p>The ARN of the security standard control to enable or disable.</p> |
| `disabled_reason` | String |  | <p>A description of the reason why you are disabling a security standard control. If you
         are disabling a control, then this is required.</p> |



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


### Standards_controls

StandardsControls resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token to use to request the next page of results.</p> |
| `controls` | Vec<String> | <p>A list of security standards controls.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access standards_controls outputs
standards_controls_id = standards_controls.id
standards_controls_next_token = standards_controls.next_token
standards_controls_controls = standards_controls.controls
```

---


### Administrator_account

AdministratorAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `administrator` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access administrator_account outputs
administrator_account_id = administrator_account.id
administrator_account_administrator = administrator_account.administrator
```

---


### Resources_statistics_v2

ResourcesStatisticsV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_by_results` | Vec<String> | <p>The aggregated statistics about resources based on the specified grouping rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resources_statistics_v2 outputs
resources_statistics_v2_id = resources_statistics_v2.id
resources_statistics_v2_group_by_results = resources_statistics_v2.group_by_results
```

---


### Ticket_v2

TicketV2 resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>The client idempotency token.</p> |
| `finding_metadata_uid` | String | ✅ | <p>The the unique ID for the finding.</p> |
| `connector_id` | String | ✅ | <p>The UUID of the connectorV2 to identify connectorV2 resource.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ticket_v2
ticket_v2 = provider.securityhub.Ticket_v2 {
    finding_metadata_uid = "value"  # <p>The the unique ID for the finding.</p>
    connector_id = "value"  # <p>The UUID of the connectorV2 to identify connectorV2 resource.</p>
}

```

---


### Action_targets

ActionTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_targets` | Vec<String> | <p>A list of <code>ActionTarget</code> objects. Each object includes the <code>ActionTargetArn</code>,
            <code>Description</code>, and <code>Name</code> of a custom action target available in
         Security Hub.</p> |
| `next_token` | String | <p>The pagination token to use to request the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access action_targets outputs
action_targets_id = action_targets.id
action_targets_action_targets = action_targets.action_targets
action_targets_next_token = action_targets.next_token
```

---


### Enabled_standards

EnabledStandards resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `standards_subscriptions` | Vec<String> | <p>The list of <code>StandardsSubscriptions</code> objects that include information about
         the enabled standards.</p> |
| `next_token` | String | <p>The pagination token to use to request the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access enabled_standards outputs
enabled_standards_id = enabled_standards.id
enabled_standards_standards_subscriptions = enabled_standards.standards_subscriptions
enabled_standards_next_token = enabled_standards.next_token
```

---


### Finding_statistics_v2

FindingStatisticsV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group_by_results` | Vec<String> | <p>Aggregated statistics about security findings based on specified grouping criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access finding_statistics_v2 outputs
finding_statistics_v2_id = finding_statistics_v2.id
finding_statistics_v2_group_by_results = finding_statistics_v2.group_by_results
```

---


### Aggregator_v2

AggregatorV2 resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `region_linking_mode` | String | ✅ | <p>Determines how Regions are linked to an Aggregator V2.</p> |
| `client_token` | String |  | <p>A unique identifier used to ensure idempotency.</p> |
| `linked_regions` | String |  | <p>The list of Regions that are linked to the aggregation Region.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of key-value pairs to be applied to the AggregatorV2.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aggregator_v2_arn` | String | <p>The ARN of the Aggregator V2.</p> |
| `aggregation_region` | String | <p>The Amazon Web Services Region where data is aggregated.</p> |
| `linked_regions` | String | <p>The list of Regions that are linked to the aggregation Region.</p> |
| `region_linking_mode` | String | <p>Determines how Regions are linked to an Aggregator V2.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create aggregator_v2
aggregator_v2 = provider.securityhub.Aggregator_v2 {
    region_linking_mode = "value"  # <p>Determines how Regions are linked to an Aggregator V2.</p>
}

# Access aggregator_v2 outputs
aggregator_v2_id = aggregator_v2.id
aggregator_v2_aggregator_v2_arn = aggregator_v2.aggregator_v2_arn
aggregator_v2_aggregation_region = aggregator_v2.aggregation_region
aggregator_v2_linked_regions = aggregator_v2.linked_regions
aggregator_v2_region_linking_mode = aggregator_v2.region_linking_mode
```

---


### Findings_v2

FindingsV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token to use to request the next page of results. 
         Otherwise, this parameter is null.</p> |
| `findings` | Vec<String> | <p>An array of security findings returned by the operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings_v2 outputs
findings_v2_id = findings_v2.id
findings_v2_next_token = findings_v2.next_token
findings_v2_findings = findings_v2.findings
```

---


### Invitations

Invitations resource

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


### Products_v2

ProductsV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `products_v2` | Vec<String> | <p>Gets information about the product integration.</p> |
| `next_token` | String | <p>The pagination token to use to request the next page of results. 
      Otherwise, this parameter is null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access products_v2 outputs
products_v2_id = products_v2.id
products_v2_products_v2 = products_v2.products_v2
products_v2_next_token = products_v2.next_token
```

---


### Configuration_policy

ConfigurationPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_policy` | String | ✅ | <p>
            An object that defines how Security Hub is configured. It includes whether Security Hub is enabled or 
            disabled, a list of enabled security standards, a list of enabled or disabled security controls, and a list of custom parameter values for specified controls. 
            If you provide a list of security controls that are enabled in the configuration policy, Security Hub disables all other controls (including newly 
            released controls). If you provide a list of security controls that are disabled in the configuration policy, Security Hub 
            enables all other controls (including newly released controls).
        </p> |
| `description` | String |  | <p>
            The description of the configuration policy.
        </p> |
| `tags` | HashMap<String, String> |  | <p>
            User-defined tags associated with a configuration policy. For more information, see 
            <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/tagging-resources.html">Tagging Security Hub resources</a> 
            in the <i>Security Hub user guide</i>.
        </p> |
| `name` | String | ✅ | <p>
            The name of the configuration policy. Alphanumeric characters and the following ASCII characters are permitted: 
            <code>-, ., !, *, /</code>.
        </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_policy` | String | <p>
            An object that defines how Security Hub is configured. It includes whether Security Hub is enabled or 
            disabled, a list of enabled security standards, a list of enabled or disabled security controls, and a list of custom parameter values for specified controls. 
            If the policy includes a list of security controls that are enabled, Security Hub disables all other controls (including newly released controls). 
            If the policy includes a list of security controls that are disabled, Security Hub enables all other controls (including 
            newly released controls).
        </p> |
| `name` | String | <p>
            The name of the configuration policy.
        </p> |
| `arn` | String | <p>
            The ARN of the configuration policy.
        </p> |
| `description` | String | <p>
            The description of the configuration policy.
        </p> |
| `created_at` | String | <p>
            The date and time, in UTC and ISO 8601 format, that the configuration policy was created.
        </p> |
| `updated_at` | String | <p>
            The date and time, in UTC and ISO 8601 format, that the configuration policy was last updated.
        </p> |
| `id` | String | <p>
            The UUID of the configuration policy.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_policy
configuration_policy = provider.securityhub.Configuration_policy {
    configuration_policy = "value"  # <p>
            An object that defines how Security Hub is configured. It includes whether Security Hub is enabled or 
            disabled, a list of enabled security standards, a list of enabled or disabled security controls, and a list of custom parameter values for specified controls. 
            If you provide a list of security controls that are enabled in the configuration policy, Security Hub disables all other controls (including newly 
            released controls). If you provide a list of security controls that are disabled in the configuration policy, Security Hub 
            enables all other controls (including newly released controls).
        </p>
    name = "value"  # <p>
            The name of the configuration policy. Alphanumeric characters and the following ASCII characters are permitted: 
            <code>-, ., !, *, /</code>.
        </p>
}

# Access configuration_policy outputs
configuration_policy_id = configuration_policy.id
configuration_policy_configuration_policy = configuration_policy.configuration_policy
configuration_policy_name = configuration_policy.name
configuration_policy_arn = configuration_policy.arn
configuration_policy_description = configuration_policy.description
configuration_policy_created_at = configuration_policy.created_at
configuration_policy_updated_at = configuration_policy.updated_at
configuration_policy_id = configuration_policy.id
```

---


### Security_hub_configuration

SecurityHubConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_enable_controls` | bool |  | <p>Whether to automatically enable new controls when they are added to standards that are
         enabled.</p>
         <p>By default, this is set to <code>true</code>, and new controls are enabled
         automatically. To not automatically enable new controls, set this to <code>false</code>.
      </p>
         <p>When you automatically enable new controls, you can interact with the controls in 
           the console and programmatically immediately after release. However, automatically enabled controls have a temporary default status of 
           <code>DISABLED</code>. It can take up to several days for Security Hub to process the control release and designate the 
           control as <code>ENABLED</code> in your account. During the processing period, you can manually enable or disable a 
           control, and Security Hub will maintain that designation regardless of whether you have <code>AutoEnableControls</code> set to 
           <code>true</code>.</p> |
| `control_finding_generator` | String |  | <p>Updates whether the calling account has consolidated control findings turned on. 
      If the value for this field is set to 
      <code>SECURITY_CONTROL</code>, Security Hub generates a single finding for a control check even when the check 
      applies to multiple enabled standards.</p>
         <p>If the value for this field is set to <code>STANDARD_CONTROL</code>, Security Hub generates separate findings 
      for a control check when the check applies to multiple enabled standards.</p>
         <p>For accounts that are part of an organization, this value can only be updated in the administrator account.</p> |



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


### Finding_aggregator

FindingAggregator resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `regions` | String |  | <p>If <code>RegionLinkingMode</code> is <code>ALL_REGIONS_EXCEPT_SPECIFIED</code>, then this is a space-separated list of Regions that don't replicate and send findings to the home Region.</p>
         <p>If <code>RegionLinkingMode</code> is <code>SPECIFIED_REGIONS</code>, then this is a space-separated list of Regions that do replicate and send findings to the home Region.
      </p>
         <p>An <code>InvalidInputException</code> error results if you populate this field while <code>RegionLinkingMode</code> is 
           <code>NO_REGIONS</code>.</p> |
| `region_linking_mode` | String | ✅ | <p>Indicates whether to aggregate findings from all of the available Regions in the current partition. Also determines whether to automatically aggregate findings from new Regions as Security Hub supports them and you opt into them.</p>
         <p>The selected option also determines how to use the Regions provided in the Regions list.</p>
         <p>The options are as follows:</p>
         <ul>
            <li>
               <p>
                  <code>ALL_REGIONS</code> - Aggregates findings from all of the Regions where Security Hub is enabled. When you choose this option, Security Hub also automatically aggregates findings from new Regions as Security Hub supports them and you opt into them.
         </p>
            </li>
            <li>
               <p>
                  <code>ALL_REGIONS_EXCEPT_SPECIFIED</code> - Aggregates findings from all of the Regions where Security Hub is enabled, except for the Regions listed in the <code>Regions</code> parameter. When you choose this option, Security Hub also automatically aggregates findings from new Regions as Security Hub supports them and you opt into them.
         </p>
            </li>
            <li>
               <p>
                  <code>SPECIFIED_REGIONS</code> - Aggregates findings only from the Regions listed in the <code>Regions</code> parameter. Security Hub does not automatically aggregate findings from new Regions.
         </p>
            </li>
            <li>
               <p>
                  <code>NO_REGIONS</code> - Aggregates no data because no Regions are selected as linked Regions.
          </p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `finding_aggregator_arn` | String | <p>The ARN of the finding aggregator.</p> |
| `finding_aggregation_region` | String | <p>The home Region. Findings generated in linked Regions are replicated and sent to the home Region.</p> |
| `region_linking_mode` | String | <p>Indicates whether to link all Regions, all Regions except for a list of excluded Regions, or a list of included Regions.</p> |
| `regions` | String | <p>The list of excluded Regions or included Regions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create finding_aggregator
finding_aggregator = provider.securityhub.Finding_aggregator {
    region_linking_mode = "value"  # <p>Indicates whether to aggregate findings from all of the available Regions in the current partition. Also determines whether to automatically aggregate findings from new Regions as Security Hub supports them and you opt into them.</p>
         <p>The selected option also determines how to use the Regions provided in the Regions list.</p>
         <p>The options are as follows:</p>
         <ul>
            <li>
               <p>
                  <code>ALL_REGIONS</code> - Aggregates findings from all of the Regions where Security Hub is enabled. When you choose this option, Security Hub also automatically aggregates findings from new Regions as Security Hub supports them and you opt into them.
         </p>
            </li>
            <li>
               <p>
                  <code>ALL_REGIONS_EXCEPT_SPECIFIED</code> - Aggregates findings from all of the Regions where Security Hub is enabled, except for the Regions listed in the <code>Regions</code> parameter. When you choose this option, Security Hub also automatically aggregates findings from new Regions as Security Hub supports them and you opt into them.
         </p>
            </li>
            <li>
               <p>
                  <code>SPECIFIED_REGIONS</code> - Aggregates findings only from the Regions listed in the <code>Regions</code> parameter. Security Hub does not automatically aggregate findings from new Regions.
         </p>
            </li>
            <li>
               <p>
                  <code>NO_REGIONS</code> - Aggregates no data because no Regions are selected as linked Regions.
          </p>
            </li>
         </ul>
}

# Access finding_aggregator outputs
finding_aggregator_id = finding_aggregator.id
finding_aggregator_finding_aggregator_arn = finding_aggregator.finding_aggregator_arn
finding_aggregator_finding_aggregation_region = finding_aggregator.finding_aggregation_region
finding_aggregator_region_linking_mode = finding_aggregator.region_linking_mode
finding_aggregator_regions = finding_aggregator.regions
```

---


### Automation_rule

AutomationRule resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `actions` | Vec<String> | ✅ | <p>
         One or more actions to update finding fields if a finding matches the conditions 
         specified in <code>Criteria</code>. 
      </p> |
| `tags` | HashMap<String, String> |  | <p>
            User-defined tags associated with an automation rule. 
        </p> |
| `rule_order` | i64 | ✅ | <p>An integer ranging from 1 to 1000 that represents the order in which the rule action is
         applied to findings. Security Hub applies rules with lower values for this parameter
         first. </p> |
| `criteria` | String | ✅ | <p>
         A set of ASFF finding field attributes and corresponding expected values that 
         Security Hub uses to filter findings. If a rule is enabled and a finding matches the conditions specified in
         this parameter, Security Hub applies the rule action to the finding.
      </p> |
| `is_terminal` | bool |  | <p>Specifies whether a rule is the last to be applied with respect to a finding that matches the rule criteria. This is useful when a finding 
            matches the criteria for multiple rules, and each rule has different actions. If a rule is terminal, Security Hub applies the rule action to a finding that matches 
            the rule criteria and doesn't evaluate other rules for the finding. By default, a rule isn't terminal.
        </p> |
| `description` | String | ✅ | <p>
         A description of the rule.
      </p> |
| `rule_name` | String | ✅ | <p>
         The name of the rule.
      </p> |
| `rule_status` | String |  | <p>
         Whether the rule is active after it is created. If 
         this parameter is equal to <code>ENABLED</code>, Security Hub starts applying the rule to findings 
         and finding updates after the rule is created. To change the value of this
         parameter after creating a rule, use <a href="https://docs.aws.amazon.com/securityhub/1.0/APIReference/API_BatchUpdateAutomationRules.html">
               <code>BatchUpdateAutomationRules</code>
            </a>.
      </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create automation_rule
automation_rule = provider.securityhub.Automation_rule {
    actions = "value"  # <p>
         One or more actions to update finding fields if a finding matches the conditions 
         specified in <code>Criteria</code>. 
      </p>
    rule_order = "value"  # <p>An integer ranging from 1 to 1000 that represents the order in which the rule action is
         applied to findings. Security Hub applies rules with lower values for this parameter
         first. </p>
    criteria = "value"  # <p>
         A set of ASFF finding field attributes and corresponding expected values that 
         Security Hub uses to filter findings. If a rule is enabled and a finding matches the conditions specified in
         this parameter, Security Hub applies the rule action to the finding.
      </p>
    description = "value"  # <p>
         A description of the rule.
      </p>
    rule_name = "value"  # <p>
         The name of the rule.
      </p>
}

```

---


### Members

Members resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_details` | Vec<String> | ✅ | <p>The list of accounts to associate with the Security Hub administrator account. For each account, the
         list includes the account ID and optionally the email address.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `members` | Vec<String> | <p>The list of details about the Security Hub member accounts.</p> |
| `unprocessed_accounts` | Vec<String> | <p>The list of Amazon Web Services accounts that could not be processed. For each account, the list
         includes the account ID and the email address.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create members
members = provider.securityhub.Members {
    account_details = "value"  # <p>The list of accounts to associate with the Security Hub administrator account. For each account, the
         list includes the account ID and optionally the email address.</p>
}

# Access members outputs
members_id = members.id
members_members = members.members
members_unprocessed_accounts = members.unprocessed_accounts
```

---


### Security_hub_v2

SecurityHubV2 resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscribed_at` | String | <p>The date and time when the service was enabled in the account.</p> |
| `hub_v2_arn` | String | <p>The ARN of the service resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_hub_v2 outputs
security_hub_v2_id = security_hub_v2.id
security_hub_v2_subscribed_at = security_hub_v2.subscribed_at
security_hub_v2_hub_v2_arn = security_hub_v2.hub_v2_arn
```

---


### Configuration_policy_association

ConfigurationPolicyAssociation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `association_status_message` | String | <p>
            The explanation for a <code>FAILED</code> value for <code>AssociationStatus</code>.
        </p> |
| `association_type` | String | <p>
            Indicates whether the association between the specified target and the configuration was directly applied by the 
            Security Hub delegated administrator or inherited from a parent.
        </p> |
| `target_id` | String | <p>
            The target account ID, organizational unit ID, or the root ID for which the association is retrieved.
        </p> |
| `association_status` | String | <p>
            The current status of the association between the specified target and the configuration.
        </p> |
| `configuration_policy_id` | String | <p>
            The universally unique identifier (UUID) of a configuration policy. For self-managed behavior, the value is 
            <code>SELF_MANAGED_SECURITY_HUB</code>.
        </p> |
| `target_type` | String | <p>
            Specifies whether the target is an Amazon Web Services account, organizational unit, or the organization root.
        </p> |
| `updated_at` | String | <p>
            The date and time, in UTC and ISO 8601 format, that the configuration policy association was last updated.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_policy_association outputs
configuration_policy_association_id = configuration_policy_association.id
configuration_policy_association_association_status_message = configuration_policy_association.association_status_message
configuration_policy_association_association_type = configuration_policy_association.association_type
configuration_policy_association_target_id = configuration_policy_association.target_id
configuration_policy_association_association_status = configuration_policy_association.association_status
configuration_policy_association_configuration_policy_id = configuration_policy_association.configuration_policy_id
configuration_policy_association_target_type = configuration_policy_association.target_type
configuration_policy_association_updated_at = configuration_policy_association.updated_at
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple action_target resources
action_target_0 = provider.securityhub.Action_target {
    description = "value-0"
    name = "value-0"
    id = "value-0"
}
action_target_1 = provider.securityhub.Action_target {
    description = "value-1"
    name = "value-1"
    id = "value-1"
}
action_target_2 = provider.securityhub.Action_target {
    description = "value-2"
    name = "value-2"
    id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    action_target = provider.securityhub.Action_target {
        description = "production-value"
        name = "production-value"
        id = "production-value"
    }
```

---

## Related Documentation

- [AWS Securityhub Documentation](https://docs.aws.amazon.com/securityhub/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
