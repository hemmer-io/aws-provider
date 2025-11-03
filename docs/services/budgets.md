# Budgets Service



**Resources**: 12

---

## Overview

The budgets service provides access to 12 resource types:

- [Notification](#notification) [CUD]
- [Budgets](#budgets) [R]
- [Budget_performance_history](#budget_performance_history) [R]
- [Subscribers_for_notification](#subscribers_for_notification) [R]
- [Budget_action_histories](#budget_action_histories) [R]
- [Subscriber](#subscriber) [CUD]
- [Budget_actions_for_account](#budget_actions_for_account) [R]
- [Notifications_for_budget](#notifications_for_budget) [R]
- [Budget](#budget) [CRUD]
- [Budget_actions_for_budget](#budget_actions_for_budget) [R]
- [Budget_action](#budget_action) [CRUD]
- [Budget_notifications_for_account](#budget_notifications_for_account) [R]

---

## Resources


### Notification

Notification resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification` | String | ✅ | <p>The notification that you want to create.</p> |
| `budget_name` | String | ✅ | <p>The name of the budget that you want Amazon Web Services to notify you about. Budget names must be unique within an account.</p> |
| `account_id` | String | ✅ | <p>The <code>accountId</code> that is associated with the budget that you want to create a notification for.</p> |
| `subscribers` | Vec<String> | ✅ | <p>A list of subscribers that you want to associate with the notification. Each notification can have one SNS subscriber and up to 10 email subscribers.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notification
notification = provider.budgets.Notification {
    notification = "value"  # <p>The notification that you want to create.</p>
    budget_name = "value"  # <p>The name of the budget that you want Amazon Web Services to notify you about. Budget names must be unique within an account.</p>
    account_id = "value"  # <p>The <code>accountId</code> that is associated with the budget that you want to create a notification for.</p>
    subscribers = "value"  # <p>A list of subscribers that you want to associate with the notification. Each notification can have one SNS subscriber and up to 10 email subscribers.</p>
}

```

---


### Budgets

Budgets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `budgets` | Vec<String> | <p>A list of budgets.</p> |
| `next_token` | String | <p>The pagination token in the service response that indicates the next set of results that you can retrieve.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access budgets outputs
budgets_id = budgets.id
budgets_budgets = budgets.budgets
budgets_next_token = budgets.next_token
```

---


### Budget_performance_history

BudgetPerformanceHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `budget_performance_history` | String | <p>The history of how often the budget has gone into an <code>ALARM</code> state.</p>
         <p>For <code>DAILY</code> budgets, the history saves the state of the budget for the last 60 days. For <code>MONTHLY</code> budgets, the history saves the state of the budget for the current month plus the last 12 months. For <code>QUARTERLY</code> budgets, the history saves the state of the budget for the last four quarters.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access budget_performance_history outputs
budget_performance_history_id = budget_performance_history.id
budget_performance_history_budget_performance_history = budget_performance_history.budget_performance_history
budget_performance_history_next_token = budget_performance_history.next_token
```

---


### Subscribers_for_notification

SubscribersForNotification resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token in the service response that indicates the next set of results that you can retrieve.</p> |
| `subscribers` | Vec<String> | <p>A list of subscribers that are associated with a notification.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subscribers_for_notification outputs
subscribers_for_notification_id = subscribers_for_notification.id
subscribers_for_notification_next_token = subscribers_for_notification.next_token
subscribers_for_notification_subscribers = subscribers_for_notification.subscribers
```

---


### Budget_action_histories

BudgetActionHistories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_histories` | Vec<String> | <p>
         The historical record of the budget action resource.
      </p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access budget_action_histories outputs
budget_action_histories_id = budget_action_histories.id
budget_action_histories_action_histories = budget_action_histories.action_histories
budget_action_histories_next_token = budget_action_histories.next_token
```

---


### Subscriber

Subscriber resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subscriber` | String | ✅ | <p>The subscriber that you want to associate with a budget notification.</p> |
| `account_id` | String | ✅ | <p>The <code>accountId</code> that is associated with the budget that you want to create a subscriber for.</p> |
| `notification` | String | ✅ | <p>The notification that you want to create a subscriber for.</p> |
| `budget_name` | String | ✅ | <p>The name of the budget that you want to subscribe to. Budget names must be unique within an account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscriber
subscriber = provider.budgets.Subscriber {
    subscriber = "value"  # <p>The subscriber that you want to associate with a budget notification.</p>
    account_id = "value"  # <p>The <code>accountId</code> that is associated with the budget that you want to create a subscriber for.</p>
    notification = "value"  # <p>The notification that you want to create a subscriber for.</p>
    budget_name = "value"  # <p>The name of the budget that you want to subscribe to. Budget names must be unique within an account.</p>
}

```

---


### Budget_actions_for_account

BudgetActionsForAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `actions` | Vec<String> | <p>
         A list of the budget action resources information.
      </p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access budget_actions_for_account outputs
budget_actions_for_account_id = budget_actions_for_account.id
budget_actions_for_account_actions = budget_actions_for_account.actions
budget_actions_for_account_next_token = budget_actions_for_account.next_token
```

---


### Notifications_for_budget

NotificationsForBudget resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token in the service response that indicates the next set of results that you can retrieve.</p> |
| `notifications` | Vec<String> | <p>A list of notifications that are associated with a budget.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access notifications_for_budget outputs
notifications_for_budget_id = notifications_for_budget.id
notifications_for_budget_next_token = notifications_for_budget.next_token
notifications_for_budget_notifications = notifications_for_budget.notifications
```

---


### Budget

Budget resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notifications_with_subscribers` | Vec<String> |  | <p>A notification that you want to associate with a budget. A budget can have up to five notifications, and each notification can have one SNS subscriber and up to 10 email subscribers. If you include notifications and subscribers in your <code>CreateBudget</code> call, Amazon Web Services creates the notifications and subscribers for you.</p> |
| `budget` | String | ✅ | <p>The budget object that you want to create.</p> |
| `resource_tags` | Vec<String> |  | <p>An optional list of tags to associate with the specified budget. Each tag consists of a
         key and a value, and each key must be unique for the resource.</p> |
| `account_id` | String | ✅ | <p>The <code>accountId</code> that is associated with the budget.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `budget` | String | <p>The description of the budget.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create budget
budget = provider.budgets.Budget {
    budget = "value"  # <p>The budget object that you want to create.</p>
    account_id = "value"  # <p>The <code>accountId</code> that is associated with the budget.</p>
}

# Access budget outputs
budget_id = budget.id
budget_budget = budget.budget
```

---


### Budget_actions_for_budget

BudgetActionsForBudget resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `actions` | Vec<String> | <p>
         A list of the budget action resources information.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access budget_actions_for_budget outputs
budget_actions_for_budget_id = budget_actions_for_budget.id
budget_actions_for_budget_next_token = budget_actions_for_budget.next_token
budget_actions_for_budget_actions = budget_actions_for_budget.actions
```

---


### Budget_action

BudgetAction resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_type` | String | ✅ | <p>
         The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition.
      </p> |
| `approval_model` | String | ✅ | <p>
         This specifies if the action needs manual or automatic approval.
      </p> |
| `notification_type` | String | ✅ |  |
| `definition` | String | ✅ |  |
| `budget_name` | String | ✅ |  |
| `resource_tags` | Vec<String> |  | <p>An optional list of tags to associate with the specified budget action. Each tag consists of a
         key and a value, and each key must be unique for the resource.</p> |
| `action_threshold` | String | ✅ |  |
| `execution_role_arn` | String | ✅ | <p>
         The role passed for action execution and reversion. Roles and actions must be in the same account.
      </p> |
| `subscribers` | Vec<String> | ✅ |  |
| `account_id` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String |  |
| `budget_name` | String |  |
| `action` | String | <p>
         A budget action resource.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create budget_action
budget_action = provider.budgets.Budget_action {
    action_type = "value"  # <p>
         The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition.
      </p>
    approval_model = "value"  # <p>
         This specifies if the action needs manual or automatic approval.
      </p>
    notification_type = "value"  # Required field
    definition = "value"  # Required field
    budget_name = "value"  # Required field
    action_threshold = "value"  # Required field
    execution_role_arn = "value"  # <p>
         The role passed for action execution and reversion. Roles and actions must be in the same account.
      </p>
    subscribers = "value"  # Required field
    account_id = "value"  # Required field
}

# Access budget_action outputs
budget_action_id = budget_action.id
budget_action_account_id = budget_action.account_id
budget_action_budget_name = budget_action.budget_name
budget_action_action = budget_action.action
```

---


### Budget_notifications_for_account

BudgetNotificationsForAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `budget_notifications_for_account` | Vec<String> | <p> A list of budget names and associated notifications for an account. </p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access budget_notifications_for_account outputs
budget_notifications_for_account_id = budget_notifications_for_account.id
budget_notifications_for_account_budget_notifications_for_account = budget_notifications_for_account.budget_notifications_for_account
budget_notifications_for_account_next_token = budget_notifications_for_account.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple notification resources
notification_0 = provider.budgets.Notification {
    notification = "value-0"
    budget_name = "value-0"
    account_id = "value-0"
    subscribers = "value-0"
}
notification_1 = provider.budgets.Notification {
    notification = "value-1"
    budget_name = "value-1"
    account_id = "value-1"
    subscribers = "value-1"
}
notification_2 = provider.budgets.Notification {
    notification = "value-2"
    budget_name = "value-2"
    account_id = "value-2"
    subscribers = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    notification = provider.budgets.Notification {
        notification = "production-value"
        budget_name = "production-value"
        account_id = "production-value"
        subscribers = "production-value"
    }
```

---

## Related Documentation

- [AWS Budgets Documentation](https://docs.aws.amazon.com/budgets/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
