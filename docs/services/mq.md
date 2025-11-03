# Mq Service



**Resources**: 7

---

## Overview

The mq service provides access to 7 resource types:

- [Tags](#tags) [CD]
- [User](#user) [CRUD]
- [Configuration](#configuration) [CRUD]
- [Configuration_revision](#configuration_revision) [R]
- [Broker](#broker) [CRUD]
- [Broker_instance_options](#broker_instance_options) [R]
- [Broker_engine_types](#broker_engine_types) [R]

---

## Resources


### Tags

Tags resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource tag.</p> |
| `tags` | HashMap<String, String> |  | <p>The key-value pair for the resource tag.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tags
tags = provider.mq.Tags {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the resource tag.</p>
}

```

---


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `groups` | Vec<String> |  | <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p> |
| `password` | String | ✅ | <p>Required. The password of the user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas, colons, or equal signs (,:=).</p> |
| `replication_user` | bool |  | <p>Defines if this user is intended for CRDR replication purposes.</p> |
| `username` | String | ✅ | <p>The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p> |
| `console_access` | bool |  | <p>Enables access to the ActiveMQ Web Console for the ActiveMQ user.</p> |
| `broker_id` | String | ✅ | <p>The unique ID that Amazon MQ generates for the broker.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pending` | String | <p>The status of the changes pending for the ActiveMQ user.</p> |
| `groups` | Vec<String> | <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p> |
| `username` | String | <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p> |
| `broker_id` | String | <p>Required. The unique ID that Amazon MQ generates for the broker.</p> |
| `console_access` | bool | <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p> |
| `replication_user` | bool | <p>Describes whether the user is intended for data replication</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.mq.User {
    password = "value"  # <p>Required. The password of the user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas, colons, or equal signs (,:=).</p>
    username = "value"  # <p>The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    broker_id = "value"  # <p>The unique ID that Amazon MQ generates for the broker.</p>
}

# Access user outputs
user_id = user.id
user_pending = user.pending
user_groups = user.groups
user_username = user.username
user_broker_id = user.broker_id
user_console_access = user.console_access
user_replication_user = user.replication_user
```

---


### Configuration

Configuration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `engine_type` | String | ✅ | <p>Required. The type of broker engine. Currently, Amazon MQ supports ACTIVEMQ and RABBITMQ.</p> |
| `engine_version` | String |  | <p>The broker engine version. Defaults to the latest available version for the specified broker engine type. For more information, see the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/activemq-version-management.html">ActiveMQ version management</a> and the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/rabbitmq-version-management.html">RabbitMQ version management</a> sections in the Amazon MQ Developer Guide.</p> |
| `tags` | HashMap<String, String> |  | <p>Create tags when creating the configuration.</p> |
| `authentication_strategy` | String |  | <p>Optional. The authentication strategy associated with the configuration. The default is SIMPLE.</p> |
| `name` | String | ✅ | <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `engine_type` | String | <p>Required. The type of broker engine. Currently, Amazon MQ supports ACTIVEMQ and RABBITMQ.</p> |
| `name` | String | <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p> |
| `arn` | String | <p>Required. The ARN of the configuration.</p> |
| `latest_revision` | String | <p>Required. The latest revision of the configuration.</p> |
| `engine_version` | String | <p>The broker engine version. Defaults to the latest available version for the specified broker engine type. For a list of supported engine versions, see the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/activemq-version-management.html">ActiveMQ version management</a> and the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/rabbitmq-version-management.html">RabbitMQ version management</a> sections in the Amazon MQ Developer Guide.</p> |
| `tags` | HashMap<String, String> | <p>The list of all tags associated with this configuration.</p> |
| `id` | String | <p>Required. The unique ID that Amazon MQ generates for the configuration.</p> |
| `authentication_strategy` | String | <p>Optional. The authentication strategy associated with the configuration. The default is SIMPLE.</p> |
| `description` | String | <p>Required. The description of the configuration.</p> |
| `created` | String | <p>Required. The date and time of the configuration revision.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration
configuration = provider.mq.Configuration {
    engine_type = "value"  # <p>Required. The type of broker engine. Currently, Amazon MQ supports ACTIVEMQ and RABBITMQ.</p>
    name = "value"  # <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
}

# Access configuration outputs
configuration_id = configuration.id
configuration_engine_type = configuration.engine_type
configuration_name = configuration.name
configuration_arn = configuration.arn
configuration_latest_revision = configuration.latest_revision
configuration_engine_version = configuration.engine_version
configuration_tags = configuration.tags
configuration_id = configuration.id
configuration_authentication_strategy = configuration.authentication_strategy
configuration_description = configuration.description
configuration_created = configuration.created
```

---


### Configuration_revision

ConfigurationRevision resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | <p>Amazon MQ for ActiveMQ: the base64-encoded XML configuration. Amazon MQ for RabbitMQ: base64-encoded Cuttlefish.</p> |
| `created` | String | <p>Required. The date and time of the configuration.</p> |
| `description` | String | <p>The description of the configuration.</p> |
| `configuration_id` | String | <p>Required. The unique ID that Amazon MQ generates for the configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_revision outputs
configuration_revision_id = configuration_revision.id
configuration_revision_data = configuration_revision.data
configuration_revision_created = configuration_revision.created
configuration_revision_description = configuration_revision.description
configuration_revision_configuration_id = configuration_revision.configuration_id
```

---


### Broker

Broker resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `broker_name` | String | ✅ | <p>Required. The broker's name. This value must be unique in your Amazon Web Services account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain white spaces, brackets, wildcard characters, or special characters.</p> <important><p>Do not add personally identifiable information (PII) or other confidential or sensitive information in broker names. Broker names are accessible to other Amazon Web Services services, including CloudWatch Logs. Broker names are not intended to be used for private or sensitive data.</p></important> |
| `authentication_strategy` | String |  | <p>Optional. The authentication strategy used to secure the broker. The default is SIMPLE.</p> |
| `creator_request_id` | String |  | <p>The unique ID that the requester receives for the created broker. Amazon MQ passes your ID with the API action.</p> <note><p>We recommend using a Universally Unique Identifier (UUID) for the creatorRequestId. You may omit the creatorRequestId if your application doesn't require idempotency.</p></note> |
| `publicly_accessible` | bool | ✅ | <p>Enables connections from applications outside of the VPC that hosts the broker's subnets. Set to false by default, if no value is provided.</p> |
| `encryption_options` | String |  | <p>Encryption options for the broker.</p> |
| `subnet_ids` | Vec<String> |  | <p>The list of groups that define which subnets and IP ranges the broker can use from different Availability Zones. If you specify more than one subnet, the subnets must be in different Availability Zones. Amazon MQ will not be able to create VPC endpoints for your broker with multiple subnets in the same Availability Zone. A SINGLE_INSTANCE deployment requires one subnet (for example, the default subnet). An ACTIVE_STANDBY_MULTI_AZ Amazon MQ for ActiveMQ deployment requires two subnets. A CLUSTER_MULTI_AZ Amazon MQ for RabbitMQ deployment has no subnet requirements when deployed with public accessibility. Deployment without public accessibility requires at least one subnet.</p> <important><p>If you specify subnets in a <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-sharing.html">shared VPC</a> for a RabbitMQ broker, the associated VPC to which the specified subnets belong must be owned by your Amazon Web Services account. Amazon MQ will not be able to create VPC endpoints in VPCs that are not owned by your Amazon Web Services account.</p></important> |
| `tags` | HashMap<String, String> |  | <p>Create tags when creating the broker.</p> |
| `data_replication_primary_broker_arn` | String |  | <p>The Amazon Resource Name (ARN) of the primary broker that is used to replicate data from in a data replication pair, and is applied to the replica broker. Must be set when dataReplicationMode is set to CRDR.</p> |
| `logs` | String |  | <p>Enables Amazon CloudWatch logging for brokers.</p> |
| `engine_type` | String | ✅ | <p>Required. The type of broker engine. Currently, Amazon MQ supports ACTIVEMQ and RABBITMQ.</p> |
| `ldap_server_metadata` | String |  | <p>Optional. The metadata of the LDAP server used to authenticate and authorize connections to the broker. Does not apply to RabbitMQ brokers.</p> |
| `security_groups` | Vec<String> |  | <p>The list of rules (1 minimum, 125 maximum) that authorize connections to brokers.</p> |
| `storage_type` | String |  | <p>The broker's storage type.</p> |
| `deployment_mode` | String | ✅ | <p>Required. The broker's deployment mode.</p> |
| `host_instance_type` | String | ✅ | <p>Required. The broker's instance type.</p> |
| `users` | Vec<String> |  | <p>The list of broker users (persons or applications) who can access queues and topics. For Amazon MQ for RabbitMQ brokers, an administrative user is required if using simple authentication and authorization. For brokers using OAuth2, this user is optional. When provided, one and only one administrative user is accepted and created when a broker is first provisioned. All subsequent broker users are created by making RabbitMQ API calls directly to brokers or via the RabbitMQ web console.</p> |
| `engine_version` | String |  | <p>The broker engine version. Defaults to the latest available version for the specified broker engine type. For more information, see the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/activemq-version-management.html">ActiveMQ version management</a> and the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/rabbitmq-version-management.html">RabbitMQ version management</a> sections in the Amazon MQ Developer Guide.</p> |
| `configuration` | String |  | <p>A list of information about the configuration.</p> |
| `auto_minor_version_upgrade` | bool |  | <p>Enables automatic upgrades to new patch versions for brokers as new versions are released and supported by Amazon MQ. Automatic upgrades occur during the scheduled maintenance window or after a manual broker reboot. Set to true by default, if no value is specified.</p> <note><p>Must be set to true for ActiveMQ brokers version 5.18 and above and for RabbitMQ brokers version 3.13 and above.</p></note> |
| `maintenance_window_start_time` | String |  | <p>The parameters that determine the WeeklyStartTime.</p> |
| `data_replication_mode` | String |  | <p>Defines whether this broker is a part of a data replication pair.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `storage_type` | String | <p>The broker's storage type.</p> |
| `pending_data_replication_metadata` | String | <p>The pending replication details of the data replication-enabled broker. Only returned if pendingDataReplicationMode is set to CRDR.</p> |
| `data_replication_metadata` | String | <p>The replication details of the data replication-enabled broker. Only returned if dataReplicationMode is set to CRDR.</p> |
| `tags` | HashMap<String, String> | <p>The list of all tags associated with this broker.</p> |
| `broker_state` | String | <p>The broker's status.</p> |
| `configurations` | String | <p>The list of all revisions for the specified configuration.</p> |
| `broker_name` | String | <p>The broker's name. This value must be unique in your Amazon Web Services account account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain white spaces, brackets, wildcard characters, or special characters.</p> |
| `logs` | String | <p>The list of information about logs currently enabled and pending to be deployed for the specified broker.</p> |
| `deployment_mode` | String | <p>The broker's deployment mode.</p> |
| `engine_type` | String | <p>The type of broker engine. Currently, Amazon MQ supports ACTIVEMQ and RABBITMQ.</p> |
| `data_replication_mode` | String | <p>Describes whether this broker is a part of a data replication pair.</p> |
| `pending_authentication_strategy` | String | <p>The authentication strategy that will be applied when the broker is rebooted. The default is SIMPLE.</p> |
| `broker_id` | String | <p>The unique ID that Amazon MQ generates for the broker.</p> |
| `broker_arn` | String | <p>The broker's Amazon Resource Name (ARN).</p> |
| `security_groups` | Vec<String> | <p>The list of rules (1 minimum, 125 maximum) that authorize connections to brokers.</p> |
| `users` | Vec<String> | <p>The list of all broker usernames for the specified broker.</p> |
| `pending_data_replication_mode` | String | <p>Describes whether this broker will be a part of a data replication pair after reboot.</p> |
| `authentication_strategy` | String | <p>The authentication strategy used to secure the broker. The default is SIMPLE.</p> |
| `pending_security_groups` | Vec<String> | <p>The list of pending security groups to authorize connections to brokers.</p> |
| `subnet_ids` | Vec<String> | <p>The list of groups that define which subnets and IP ranges the broker can use from different Availability Zones.</p> |
| `created` | String | <p>The time when the broker was created.</p> |
| `pending_host_instance_type` | String | <p>The broker's host instance type to upgrade to. For a list of supported instance types, see <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/broker.html#broker-instance-types">Broker instance types</a>.</p> |
| `encryption_options` | String | <p>Encryption options for the broker.</p> |
| `maintenance_window_start_time` | String | <p>The parameters that determine the WeeklyStartTime.</p> |
| `auto_minor_version_upgrade` | bool | <p>Enables automatic upgrades to new patch versions for brokers as new versions are released and supported by Amazon MQ. Automatic upgrades occur during the scheduled maintenance window or after a manual broker reboot.</p> |
| `broker_instances` | Vec<String> | <p>A list of information about allocated brokers.</p> |
| `engine_version` | String | <p>The broker engine version. For more information, see the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/activemq-version-management.html">ActiveMQ version management</a> and the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/rabbitmq-version-management.html">RabbitMQ version management</a> sections in the Amazon MQ Developer Guide.</p> |
| `ldap_server_metadata` | String | <p>The metadata of the LDAP server used to authenticate and authorize connections to the broker.</p> |
| `actions_required` | Vec<String> | <p>Actions required for a broker.</p> |
| `publicly_accessible` | bool | <p>Enables connections from applications outside of the VPC that hosts the broker's subnets.</p> |
| `pending_engine_version` | String | <p>The broker engine version to upgrade to. For more information, see the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/activemq-version-management.html">ActiveMQ version management</a> and the <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/rabbitmq-version-management.html">RabbitMQ version management</a> sections in the Amazon MQ Developer Guide.</p> |
| `pending_ldap_server_metadata` | String | <p>The metadata of the LDAP server that will be used to authenticate and authorize connections to the broker after it is rebooted.</p> |
| `host_instance_type` | String | <p>The broker's instance type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create broker
broker = provider.mq.Broker {
    broker_name = "value"  # <p>Required. The broker's name. This value must be unique in your Amazon Web Services account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain white spaces, brackets, wildcard characters, or special characters.</p> <important><p>Do not add personally identifiable information (PII) or other confidential or sensitive information in broker names. Broker names are accessible to other Amazon Web Services services, including CloudWatch Logs. Broker names are not intended to be used for private or sensitive data.</p></important>
    publicly_accessible = "value"  # <p>Enables connections from applications outside of the VPC that hosts the broker's subnets. Set to false by default, if no value is provided.</p>
    engine_type = "value"  # <p>Required. The type of broker engine. Currently, Amazon MQ supports ACTIVEMQ and RABBITMQ.</p>
    deployment_mode = "value"  # <p>Required. The broker's deployment mode.</p>
    host_instance_type = "value"  # <p>Required. The broker's instance type.</p>
}

# Access broker outputs
broker_id = broker.id
broker_storage_type = broker.storage_type
broker_pending_data_replication_metadata = broker.pending_data_replication_metadata
broker_data_replication_metadata = broker.data_replication_metadata
broker_tags = broker.tags
broker_broker_state = broker.broker_state
broker_configurations = broker.configurations
broker_broker_name = broker.broker_name
broker_logs = broker.logs
broker_deployment_mode = broker.deployment_mode
broker_engine_type = broker.engine_type
broker_data_replication_mode = broker.data_replication_mode
broker_pending_authentication_strategy = broker.pending_authentication_strategy
broker_broker_id = broker.broker_id
broker_broker_arn = broker.broker_arn
broker_security_groups = broker.security_groups
broker_users = broker.users
broker_pending_data_replication_mode = broker.pending_data_replication_mode
broker_authentication_strategy = broker.authentication_strategy
broker_pending_security_groups = broker.pending_security_groups
broker_subnet_ids = broker.subnet_ids
broker_created = broker.created
broker_pending_host_instance_type = broker.pending_host_instance_type
broker_encryption_options = broker.encryption_options
broker_maintenance_window_start_time = broker.maintenance_window_start_time
broker_auto_minor_version_upgrade = broker.auto_minor_version_upgrade
broker_broker_instances = broker.broker_instances
broker_engine_version = broker.engine_version
broker_ldap_server_metadata = broker.ldap_server_metadata
broker_actions_required = broker.actions_required
broker_publicly_accessible = broker.publicly_accessible
broker_pending_engine_version = broker.pending_engine_version
broker_pending_ldap_server_metadata = broker.pending_ldap_server_metadata
broker_host_instance_type = broker.host_instance_type
```

---


### Broker_instance_options

BrokerInstanceOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `broker_instance_options` | Vec<String> | <p>List of available broker instance options.</p> |
| `next_token` | String | <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p> |
| `max_results` | i64 | <p>Required. The maximum number of instance options that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access broker_instance_options outputs
broker_instance_options_id = broker_instance_options.id
broker_instance_options_broker_instance_options = broker_instance_options.broker_instance_options
broker_instance_options_next_token = broker_instance_options.next_token
broker_instance_options_max_results = broker_instance_options.max_results
```

---


### Broker_engine_types

BrokerEngineTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `broker_engine_types` | Vec<String> | <p>List of available engine types and versions.</p> |
| `max_results` | i64 | <p>Required. The maximum number of engine types that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p> |
| `next_token` | String | <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access broker_engine_types outputs
broker_engine_types_id = broker_engine_types.id
broker_engine_types_broker_engine_types = broker_engine_types.broker_engine_types
broker_engine_types_max_results = broker_engine_types.max_results
broker_engine_types_next_token = broker_engine_types.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple tags resources
tags_0 = provider.mq.Tags {
    resource_arn = "value-0"
}
tags_1 = provider.mq.Tags {
    resource_arn = "value-1"
}
tags_2 = provider.mq.Tags {
    resource_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tags = provider.mq.Tags {
        resource_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Mq Documentation](https://docs.aws.amazon.com/mq/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
