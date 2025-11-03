# Dynamodb Service



**Resources**: 15

---

## Overview

The dynamodb service provides access to 15 resource types:

- [Resource_policy](#resource_policy) [CRD]
- [Global_table_settings](#global_table_settings) [RU]
- [Export](#export) [R]
- [Item](#item) [CRUD]
- [Limits](#limits) [R]
- [Contributor_insights](#contributor_insights) [RU]
- [Table_replica_auto_scaling](#table_replica_auto_scaling) [RU]
- [Global_table](#global_table) [CRU]
- [Endpoints](#endpoints) [R]
- [Import](#import) [R]
- [Kinesis_streaming_destination](#kinesis_streaming_destination) [RU]
- [Continuous_backups](#continuous_backups) [RU]
- [Backup](#backup) [CRD]
- [Table](#table) [CRUD]
- [Time_to_live](#time_to_live) [RU]

---

## Resources


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached.
            The resources you can specify include tables and streams.</p>
         <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p> |
| `policy` | String | ✅ | <p>An Amazon Web Services resource-based policy document in JSON format.</p>
         <ul>
            <li>
               <p>The maximum size supported for a resource-based policy document is 20 KB.
                        DynamoDB counts whitespaces when calculating the size of a policy
                    against this limit.</p>
            </li>
            <li>
               <p>Within a resource-based policy, if the action for a DynamoDB
                    service-linked role (SLR) to replicate data for a global table is denied, adding
                    or deleting a replica will fail with an error.</p>
            </li>
         </ul>
         <p>For a full list of all considerations that apply while attaching a resource-based
            policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based
                policy considerations</a>.</p> |
| `expected_revision_id` | String |  | <p>A string value that you can use to conditionally update your policy. You can provide
            the revision ID of your existing policy to make mutating requests against that
            policy.</p>
         <note>
            <p>When you provide an expected revision ID, if the revision ID of the existing
                policy on the resource doesn't match or if there's no policy attached to the
                resource, your request will be rejected with a
                <code>PolicyNotFoundException</code>.</p>
         </note>
         <p>To conditionally attach a policy when no policy exists for the resource, specify
                <code>NO_POLICY</code> for the revision ID.</p> |
| `confirm_remove_self_resource_access` | bool |  | <p>Set this parameter to <code>true</code> to confirm that you want to remove your
            permissions to change the policy of this resource in the future.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The resource-based policy document attached to the resource, which can be a table or
            stream, in JSON format.</p> |
| `revision_id` | String | <p>A unique string that represents the revision ID of the policy. If you're comparing revision IDs, make sure to always use string comparison logic.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.dynamodb.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached.
            The resources you can specify include tables and streams.</p>
         <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p>
    policy = "value"  # <p>An Amazon Web Services resource-based policy document in JSON format.</p>
         <ul>
            <li>
               <p>The maximum size supported for a resource-based policy document is 20 KB.
                        DynamoDB counts whitespaces when calculating the size of a policy
                    against this limit.</p>
            </li>
            <li>
               <p>Within a resource-based policy, if the action for a DynamoDB
                    service-linked role (SLR) to replicate data for a global table is denied, adding
                    or deleting a replica will fail with an error.</p>
            </li>
         </ul>
         <p>For a full list of all considerations that apply while attaching a resource-based
            policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based
                policy considerations</a>.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
resource_policy_revision_id = resource_policy.revision_id
```

---


### Global_table_settings

GlobalTableSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `global_table_provisioned_write_capacity_auto_scaling_settings_update` | String |  | <p>Auto scaling settings for managing provisioned write capacity for the global
            table.</p> |
| `global_table_provisioned_write_capacity_units` | i64 |  | <p>The maximum number of writes consumed per second before DynamoDB returns a
                <code>ThrottlingException.</code>
         </p> |
| `replica_settings_update` | Vec<String> |  | <p>Represents the settings for a global table in a Region that will be modified.</p> |
| `global_table_global_secondary_index_settings_update` | Vec<String> |  | <p>Represents the settings of a global secondary index for a global table that will be
            modified.</p> |
| `global_table_billing_mode` | String |  | <p>The billing mode of the global table. If <code>GlobalTableBillingMode</code> is not
            specified, the global table defaults to <code>PROVISIONED</code> capacity billing
            mode.</p>
         <ul>
            <li>
               <p>
                  <code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for
                    predictable workloads. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/provisioned-capacity-mode.html">Provisioned capacity mode</a>.</p>
            </li>
            <li>
               <p>
                  <code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code>
                    for unpredictable workloads. <code>PAY_PER_REQUEST</code> sets the billing mode
                    to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/on-demand-capacity-mode.html">On-demand capacity mode</a>. </p>
            </li>
         </ul> |
| `global_table_name` | String | ✅ | <p>The name of the global table</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_table_name` | String | <p>The name of the global table.</p> |
| `replica_settings` | Vec<String> | <p>The Region-specific settings for the global table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access global_table_settings outputs
global_table_settings_id = global_table_settings.id
global_table_settings_global_table_name = global_table_settings.global_table_name
global_table_settings_replica_settings = global_table_settings.replica_settings
```

---


### Export

Export resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_description` | String | <p>Represents the properties of the export.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export outputs
export_id = export.id
export_export_description = export.export_description
```

---


### Item

Item resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `return_values` | String |  | <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared
            before they were updated with the <code>PutItem</code> request. For
            <code>PutItem</code>, the valid values are:</p>
         <ul>
            <li>
               <p>
                  <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its
                    value is <code>NONE</code>, then nothing is returned. (This setting is the
                    default for <code>ReturnValues</code>.)</p>
            </li>
            <li>
               <p>
                  <code>ALL_OLD</code> - If <code>PutItem</code> overwrote an attribute name-value
                    pair, then the content of the old item is returned.</p>
            </li>
         </ul>
         <p>The values returned are strongly consistent.</p>
         <p>There is no additional cost associated with requesting a return value aside from the
            small network and processing overhead of receiving a larger response. No read capacity
            units are consumed.</p>
         <note>
            <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations;
                however, <code>PutItem</code> does not recognize any values other than
                    <code>NONE</code> or <code>ALL_OLD</code>.</p>
         </note> |
| `table_name` | String | ✅ | <p>The name of the table to contain the item. You can also provide the Amazon Resource Name (ARN) of the
            table in this parameter.</p> |
| `item` | HashMap<String, String> | ✅ | <p>A map of attribute name/value pairs, one for each attribute. Only the primary key
            attributes are required; you can optionally provide other attribute name-value pairs for
            the item.</p>
         <p>You must provide all of the attributes for the primary key. For example, with a simple
            primary key, you only need to provide a value for the partition key. For a composite
            primary key, you must provide both values for both the partition key and the sort
            key.</p>
         <p>If you specify any attributes that are part of an index key, then the data types for
            those attributes must match those of the schema in the table's attribute
            definition.</p>
         <p>Empty String and Binary attribute values are allowed. Attribute values of type String
            and Binary must have a length greater than zero if the attribute is used as a key
            attribute for a table or index.</p>
         <p>For more information about primary keys, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.CoreComponents.html#HowItWorks.CoreComponents.PrimaryKey">Primary Key</a> in the <i>Amazon DynamoDB Developer
            Guide</i>.</p>
         <p>Each element in the <code>Item</code> map is an <code>AttributeValue</code>
            object.</p> |
| `condition_expression` | String |  | <p>A condition that must be satisfied in order for a conditional <code>PutItem</code>
            operation to succeed.</p>
         <p>An expression can contain any of the following:</p>
         <ul>
            <li>
               <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type |
                        contains | begins_with | size</code>
               </p>
               <p>These function names are case-sensitive.</p>
            </li>
            <li>
               <p>Comparison operators: <code>= | <> |
            < | > | <= | >= |
            BETWEEN | IN </code>
               </p>
            </li>
            <li>
               <p> Logical operators: <code>AND | OR | NOT</code>
               </p>
            </li>
         </ul>
         <p>For more information on condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer
                Guide</i>.</p> |
| `expression_attribute_names` | HashMap<String, String> |  | <p>One or more substitution tokens for attribute names in an expression. The following
            are some use cases for using <code>ExpressionAttributeNames</code>:</p>
         <ul>
            <li>
               <p>To access an attribute whose name conflicts with a DynamoDB reserved
                    word.</p>
            </li>
            <li>
               <p>To create a placeholder for repeating occurrences of an attribute name in an
                    expression.</p>
            </li>
            <li>
               <p>To prevent special characters in an attribute name from being misinterpreted
                    in an expression.</p>
            </li>
         </ul>
         <p>Use the <b>#</b> character in an expression to dereference
            an attribute name. For example, consider the following attribute name:</p>
         <ul>
            <li>
               <p>
                  <code>Percentile</code>
               </p>
            </li>
         </ul>
         <p>The name of this attribute conflicts with a reserved word, so it cannot be used
            directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer
            Guide</i>). To work around this, you could specify the following for
                <code>ExpressionAttributeNames</code>:</p>
         <ul>
            <li>
               <p>
                  <code>{"#P":"Percentile"}</code>
               </p>
            </li>
         </ul>
         <p>You could then use this substitution in an expression, as in this example:</p>
         <ul>
            <li>
               <p>
                  <code>#P = :val</code>
               </p>
            </li>
         </ul>
         <note>
            <p>Tokens that begin with the <b>:</b> character are
                    <i>expression attribute values</i>, which are placeholders for the
                actual value at runtime.</p>
         </note>
         <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer
                Guide</i>.</p> |
| `expression_attribute_values` | HashMap<String, String> |  | <p>One or more values that can be substituted in an expression.</p>
         <p>Use the <b>:</b> (colon) character in an expression to
            dereference an attribute value. For example, suppose that you wanted to check whether
            the value of the <i>ProductStatus</i> attribute was one of the following: </p>
         <p>
            <code>Available | Backordered | Discontinued</code>
         </p>
         <p>You would first need to specify <code>ExpressionAttributeValues</code> as
            follows:</p>
         <p>
            <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"},
                ":disc":{"S":"Discontinued"} }</code>
         </p>
         <p>You could then use these values in an expression, such as this:</p>
         <p>
            <code>ProductStatus IN (:avail, :back, :disc)</code>
         </p>
         <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer
                Guide</i>.</p> |
| `return_values_on_condition_check_failure` | String |  | <p>An optional parameter that returns the item attributes for a <code>PutItem</code>
            operation that failed a condition check.</p>
         <p>There is no additional cost associated with requesting a return value aside from the
            small network and processing overhead of receiving a larger response. No read capacity
            units are consumed.</p> |
| `expected` | HashMap<String, String> |  | <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more
            information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer
            Guide</i>.</p> |
| `return_consumed_capacity` | String |  |  |
| `return_item_collection_metrics` | String |  | <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>,
            the response includes statistics about item collections, if any, that were modified
            during the operation are returned in the response. If set to <code>NONE</code> (the
            default), no statistics are returned.</p> |
| `conditional_operator` | String |  | <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more
            information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer
                Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `item` | HashMap<String, String> | <p>A map of attribute names to <code>AttributeValue</code> objects, as specified by
                <code>ProjectionExpression</code>.</p> |
| `consumed_capacity` | String | <p>The capacity units consumed by the <code>GetItem</code> operation. The data returned
            includes the total provisioned throughput consumed, along with statistics for the table
            and any indexes involved in the operation. <code>ConsumedCapacity</code> is only
            returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more
            information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/read-write-operations.html#read-operation-consumption">Capacity unit consumption for read operations</a> in the <i>Amazon
                DynamoDB Developer Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create item
item = provider.dynamodb.Item {
    table_name = "value"  # <p>The name of the table to contain the item. You can also provide the Amazon Resource Name (ARN) of the
            table in this parameter.</p>
    item = "value"  # <p>A map of attribute name/value pairs, one for each attribute. Only the primary key
            attributes are required; you can optionally provide other attribute name-value pairs for
            the item.</p>
         <p>You must provide all of the attributes for the primary key. For example, with a simple
            primary key, you only need to provide a value for the partition key. For a composite
            primary key, you must provide both values for both the partition key and the sort
            key.</p>
         <p>If you specify any attributes that are part of an index key, then the data types for
            those attributes must match those of the schema in the table's attribute
            definition.</p>
         <p>Empty String and Binary attribute values are allowed. Attribute values of type String
            and Binary must have a length greater than zero if the attribute is used as a key
            attribute for a table or index.</p>
         <p>For more information about primary keys, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.CoreComponents.html#HowItWorks.CoreComponents.PrimaryKey">Primary Key</a> in the <i>Amazon DynamoDB Developer
            Guide</i>.</p>
         <p>Each element in the <code>Item</code> map is an <code>AttributeValue</code>
            object.</p>
}

# Access item outputs
item_id = item.id
item_item = item.item
item_consumed_capacity = item.consumed_capacity
```

---


### Limits

Limits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_max_write_capacity_units` | i64 | <p>The maximum total write capacity units that your account allows you to provision
            across all of your tables in this Region.</p> |
| `table_max_read_capacity_units` | i64 | <p>The maximum read capacity units that your account allows you to provision for a new
            table that you are creating in this Region, including the read capacity units
            provisioned for its global secondary indexes (GSIs).</p> |
| `table_max_write_capacity_units` | i64 | <p>The maximum write capacity units that your account allows you to provision for a new
            table that you are creating in this Region, including the write capacity units
            provisioned for its global secondary indexes (GSIs).</p> |
| `account_max_read_capacity_units` | i64 | <p>The maximum total read capacity units that your account allows you to provision across
            all of your tables in this Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access limits outputs
limits_id = limits.id
limits_account_max_write_capacity_units = limits.account_max_write_capacity_units
limits_table_max_read_capacity_units = limits.table_max_read_capacity_units
limits_table_max_write_capacity_units = limits.table_max_write_capacity_units
limits_account_max_read_capacity_units = limits.account_max_read_capacity_units
```

---


### Contributor_insights

ContributorInsights resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `table_name` | String | ✅ | <p>The name of the table. You can also provide the Amazon Resource Name (ARN) of the table in this
            parameter.</p> |
| `contributor_insights_mode` | String |  | <p>Specifies whether to track all access and throttled events or throttled events only for
            the DynamoDB table or index.</p> |
| `index_name` | String |  | <p>The global secondary index name, if applicable.</p> |
| `contributor_insights_action` | String | ✅ | <p>Represents the contributor insights action.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_name` | String | <p>The name of the global secondary index being described.</p> |
| `failure_exception` | String | <p>Returns information about the last failure that was encountered.</p>
         <p>The most common exceptions for a FAILED status are:</p>
         <ul>
            <li>
               <p>LimitExceededException - Per-account Amazon CloudWatch Contributor Insights
                    rule limit reached. Please disable Contributor Insights for other tables/indexes
                    OR disable Contributor Insights rules before retrying.</p>
            </li>
            <li>
               <p>AccessDeniedException - Amazon CloudWatch Contributor Insights rules cannot be
                    modified due to insufficient permissions.</p>
            </li>
            <li>
               <p>AccessDeniedException - Failed to create service-linked role for Contributor
                    Insights due to insufficient permissions.</p>
            </li>
            <li>
               <p>InternalServerError - Failed to create Amazon CloudWatch Contributor Insights
                    rules. Please retry request.</p>
            </li>
         </ul> |
| `contributor_insights_mode` | String | <p>The mode of CloudWatch Contributor Insights for DynamoDB that determines
            which events are emitted. Can be set to track all access and throttled events or throttled
            events only.</p> |
| `contributor_insights_rule_list` | Vec<String> | <p>List of names of the associated contributor insights rules.</p> |
| `table_name` | String | <p>The name of the table being described.</p> |
| `contributor_insights_status` | String | <p>Current status of contributor insights.</p> |
| `last_update_date_time` | String | <p>Timestamp of the last time the status was changed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access contributor_insights outputs
contributor_insights_id = contributor_insights.id
contributor_insights_index_name = contributor_insights.index_name
contributor_insights_failure_exception = contributor_insights.failure_exception
contributor_insights_contributor_insights_mode = contributor_insights.contributor_insights_mode
contributor_insights_contributor_insights_rule_list = contributor_insights.contributor_insights_rule_list
contributor_insights_table_name = contributor_insights.table_name
contributor_insights_contributor_insights_status = contributor_insights.contributor_insights_status
contributor_insights_last_update_date_time = contributor_insights.last_update_date_time
```

---


### Table_replica_auto_scaling

TableReplicaAutoScaling resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replica_updates` | Vec<String> |  | <p>Represents the auto scaling settings of replicas of the table that will be
            modified.</p> |
| `provisioned_write_capacity_auto_scaling_update` | String |  |  |
| `global_secondary_index_updates` | Vec<String> |  | <p>Represents the auto scaling settings of the global secondary indexes of the replica to
            be updated.</p> |
| `table_name` | String | ✅ | <p>The name of the global table to be updated. You can also provide the Amazon Resource Name (ARN) of the
            table in this parameter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_auto_scaling_description` | String | <p>Represents the auto scaling properties of the table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_replica_auto_scaling outputs
table_replica_auto_scaling_id = table_replica_auto_scaling.id
table_replica_auto_scaling_table_auto_scaling_description = table_replica_auto_scaling.table_auto_scaling_description
```

---


### Global_table

GlobalTable resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replication_group` | Vec<String> | ✅ | <p>The Regions where the global table needs to be created.</p> |
| `global_table_name` | String | ✅ | <p>The global table name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_table_description` | String | <p>Contains the details of the global table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create global_table
global_table = provider.dynamodb.Global_table {
    replication_group = "value"  # <p>The Regions where the global table needs to be created.</p>
    global_table_name = "value"  # <p>The global table name.</p>
}

# Access global_table outputs
global_table_id = global_table.id
global_table_global_table_description = global_table.global_table_description
```

---


### Endpoints

Endpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoints` | Vec<String> | <p>List of endpoints.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoints outputs
endpoints_id = endpoints.id
endpoints_endpoints = endpoints.endpoints
```

---


### Import

Import resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_table_description` | String | <p> Represents the properties of the table created for the import, and parameters of the
            import. The import parameters include import status, how many items were processed, and
            how many errors were encountered. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import outputs
import_id = import.id
import_import_table_description = import.import_table_description
```

---


### Kinesis_streaming_destination

KinesisStreamingDestination resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_kinesis_streaming_configuration` | String |  | <p>The command to update the Kinesis stream configuration.</p> |
| `table_name` | String | ✅ | <p>The table name for the Kinesis streaming destination input. You can also provide the
            ARN of the table in this parameter.</p> |
| `stream_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for the Kinesis stream input.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kinesis_data_stream_destinations` | Vec<String> | <p>The list of replica structures for the table being described.</p> |
| `table_name` | String | <p>The name of the table being described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access kinesis_streaming_destination outputs
kinesis_streaming_destination_id = kinesis_streaming_destination.id
kinesis_streaming_destination_kinesis_data_stream_destinations = kinesis_streaming_destination.kinesis_data_stream_destinations
kinesis_streaming_destination_table_name = kinesis_streaming_destination.table_name
```

---


### Continuous_backups

ContinuousBackups resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `table_name` | String | ✅ | <p>The name of the table. You can also provide the Amazon Resource Name (ARN) of the table in this
            parameter.</p> |
| `point_in_time_recovery_specification` | String | ✅ | <p>Represents the settings used to enable point in time recovery.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `continuous_backups_description` | String | <p>Represents the continuous backups and point in time recovery settings on the
            table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access continuous_backups outputs
continuous_backups_id = continuous_backups.id
continuous_backups_continuous_backups_description = continuous_backups.continuous_backups_description
```

---


### Backup

Backup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `table_name` | String | ✅ | <p>The name of the table. You can also provide the Amazon Resource Name (ARN) of the table in this
            parameter.</p> |
| `backup_name` | String | ✅ | <p>Specified name for the backup.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backup_description` | String | <p>Contains the description of the backup created for the table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backup
backup = provider.dynamodb.Backup {
    table_name = "value"  # <p>The name of the table. You can also provide the Amazon Resource Name (ARN) of the table in this
            parameter.</p>
    backup_name = "value"  # <p>Specified name for the backup.</p>
}

# Access backup outputs
backup_id = backup.id
backup_backup_description = backup.backup_description
```

---


### Table

Table resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `billing_mode` | String |  | <p>Controls how you are charged for read and write throughput and how you manage
            capacity. This setting can be changed later.</p>
         <ul>
            <li>
               <p>
                  <code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code>
                    for most DynamoDB workloads. <code>PAY_PER_REQUEST</code> sets the billing mode
                    to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/on-demand-capacity-mode.html">On-demand capacity mode</a>. </p>
            </li>
            <li>
               <p>
                  <code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for
                    steady workloads with predictable growth where capacity requirements can be
                    reliably forecasted. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/provisioned-capacity-mode.html">Provisioned capacity mode</a>.</p>
            </li>
         </ul> |
| `stream_specification` | String |  | <p>The settings for DynamoDB Streams on the table. These settings consist of:</p>
         <ul>
            <li>
               <p>
                  <code>StreamEnabled</code> - Indicates whether DynamoDB Streams is to be enabled
                    (true) or disabled (false).</p>
            </li>
            <li>
               <p>
                  <code>StreamViewType</code> - When an item in the table is modified,
                        <code>StreamViewType</code> determines what information is written to the
                    table's stream. Valid values for <code>StreamViewType</code> are:</p>
               <ul>
                  <li>
                     <p>
                        <code>KEYS_ONLY</code> - Only the key attributes of the modified item
                            are written to the stream.</p>
                  </li>
                  <li>
                     <p>
                        <code>NEW_IMAGE</code> - The entire item, as it appears after it was
                            modified, is written to the stream.</p>
                  </li>
                  <li>
                     <p>
                        <code>OLD_IMAGE</code> - The entire item, as it appeared before it was
                            modified, is written to the stream.</p>
                  </li>
                  <li>
                     <p>
                        <code>NEW_AND_OLD_IMAGES</code> - Both the new and the old item images
                            of the item are written to the stream.</p>
                  </li>
               </ul>
            </li>
         </ul> |
| `table_class` | String |  | <p>The table class of the new table. Valid values are <code>STANDARD</code> and
                <code>STANDARD_INFREQUENT_ACCESS</code>.</p> |
| `key_schema` | Vec<String> | ✅ | <p>Specifies the attributes that make up the primary key for a table or an index. The
            attributes in <code>KeySchema</code> must also be defined in the
                <code>AttributeDefinitions</code> array. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html">Data
                Model</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
         <p>Each <code>KeySchemaElement</code> in the array is composed of:</p>
         <ul>
            <li>
               <p>
                  <code>AttributeName</code> - The name of this key attribute.</p>
            </li>
            <li>
               <p>
                  <code>KeyType</code> - The role that the key attribute will assume:</p>
               <ul>
                  <li>
                     <p>
                        <code>HASH</code> - partition key</p>
                  </li>
                  <li>
                     <p>
                        <code>RANGE</code> - sort key</p>
                  </li>
               </ul>
            </li>
         </ul>
         <note>
            <p>The partition key of an item is also known as its <i>hash
                    attribute</i>. The term "hash attribute" derives from the DynamoDB usage
                of an internal hash function to evenly distribute data items across partitions,
                based on their partition key values.</p>
            <p>The sort key of an item is also known as its <i>range attribute</i>.
                The term "range attribute" derives from the way DynamoDB stores items with the same
                partition key physically close together, in sorted order by the sort key
                value.</p>
         </note>
         <p>For a simple primary key (partition key), you must provide exactly one element with a
                <code>KeyType</code> of <code>HASH</code>.</p>
         <p>For a composite primary key (partition key and sort key), you must provide exactly two
            elements, in this order: The first element must have a <code>KeyType</code> of
                <code>HASH</code>, and the second element must have a <code>KeyType</code> of
                <code>RANGE</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key">Working with Tables</a> in the <i>Amazon DynamoDB Developer
                Guide</i>.</p> |
| `attribute_definitions` | Vec<String> | ✅ | <p>An array of attributes that describe the key schema for the table and indexes.</p> |
| `global_secondary_indexes` | Vec<String> |  | <p>One or more global secondary indexes (the maximum is 20) to be created on the table.
            Each global secondary index in the array includes the following:</p>
         <ul>
            <li>
               <p>
                  <code>IndexName</code> - The name of the global secondary index. Must be unique
                    only for this table.</p>
               <p></p>
            </li>
            <li>
               <p>
                  <code>KeySchema</code> - Specifies the key schema for the global secondary
                    index.</p>
            </li>
            <li>
               <p>
                  <code>Projection</code> - Specifies attributes that are copied (projected) from
                    the table into the index. These are in addition to the primary key attributes
                    and index key attributes, which are automatically projected. Each attribute
                    specification is composed of:</p>
               <ul>
                  <li>
                     <p>
                        <code>ProjectionType</code> - One of the following:</p>
                     <ul>
                        <li>
                           <p>
                              <code>KEYS_ONLY</code> - Only the index and primary keys are
                                    projected into the index.</p>
                        </li>
                        <li>
                           <p>
                              <code>INCLUDE</code> - Only the specified table attributes are
                                    projected into the index. The list of projected attributes is in
                                        <code>NonKeyAttributes</code>.</p>
                        </li>
                        <li>
                           <p>
                              <code>ALL</code> - All of the table attributes are projected
                                    into the index.</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <code>NonKeyAttributes</code> - A list of one or more non-key attribute
                            names that are projected into the secondary index. The total count of
                            attributes provided in <code>NonKeyAttributes</code>, summed across all
                            of the secondary indexes, must not exceed 100. If you project the same
                            attribute into two different indexes, this counts as two distinct
                            attributes when determining the total. This limit only applies when you
                            specify the ProjectionType of <code>INCLUDE</code>. You still can
                            specify the ProjectionType of <code>ALL</code> to project all attributes
                            from the source table, even if the table has more than 100
                            attributes.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <code>ProvisionedThroughput</code> - The provisioned throughput settings for the
                    global secondary index, consisting of read and write capacity units.</p>
            </li>
         </ul> |
| `sse_specification` | String |  | <p>Represents the settings used to enable server-side encryption.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs to label the table. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging
                for DynamoDB</a>.</p> |
| `deletion_protection_enabled` | bool |  | <p>Indicates whether deletion protection is to be enabled (true) or disabled (false) on
            the table.</p> |
| `warm_throughput` | String |  | <p>Represents the warm throughput (in read units per second and write units per second)
            for creating a table.</p> |
| `resource_policy` | String |  | <p>An Amazon Web Services resource-based policy document in JSON format that will be
            attached to the table.</p>
         <p>When you attach a resource-based policy while creating a table, the policy application
            is <i>strongly consistent</i>.</p>
         <p>The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this
            limit. For a full list of all considerations that apply for resource-based policies, see
                <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based
                policy considerations</a>.</p>
         <note>
            <p>You need to specify the <code>CreateTable</code> and
                    <code>PutResourcePolicy</code>
                IAM actions for authorizing a user to create a table with a
                resource-based policy.</p>
         </note> |
| `on_demand_throughput` | String |  | <p>Sets the maximum number of read and write units for the specified table in on-demand
            capacity mode. If you use this parameter, you must specify
                <code>MaxReadRequestUnits</code>, <code>MaxWriteRequestUnits</code>, or both.</p> |
| `table_name` | String | ✅ | <p>The name of the table to create. You can also provide the Amazon Resource Name (ARN) of the table in
            this parameter.</p> |
| `local_secondary_indexes` | Vec<String> |  | <p>One or more local secondary indexes (the maximum is 5) to be created on the table.
            Each index is scoped to a given partition key value. There is a 10 GB size limit per
            partition key value; otherwise, the size of a local secondary index is
            unconstrained.</p>
         <p>Each local secondary index in the array includes the following:</p>
         <ul>
            <li>
               <p>
                  <code>IndexName</code> - The name of the local secondary index. Must be unique
                    only for this table.</p>
               <p></p>
            </li>
            <li>
               <p>
                  <code>KeySchema</code> - Specifies the key schema for the local secondary index.
                    The key schema must begin with the same partition key as the table.</p>
            </li>
            <li>
               <p>
                  <code>Projection</code> - Specifies attributes that are copied (projected) from
                    the table into the index. These are in addition to the primary key attributes
                    and index key attributes, which are automatically projected. Each attribute
                    specification is composed of:</p>
               <ul>
                  <li>
                     <p>
                        <code>ProjectionType</code> - One of the following:</p>
                     <ul>
                        <li>
                           <p>
                              <code>KEYS_ONLY</code> - Only the index and primary keys are
                                    projected into the index.</p>
                        </li>
                        <li>
                           <p>
                              <code>INCLUDE</code> - Only the specified table attributes are
                                    projected into the index. The list of projected attributes is in
                                        <code>NonKeyAttributes</code>.</p>
                        </li>
                        <li>
                           <p>
                              <code>ALL</code> - All of the table attributes are projected
                                    into the index.</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <code>NonKeyAttributes</code> - A list of one or more non-key attribute
                            names that are projected into the secondary index. The total count of
                            attributes provided in <code>NonKeyAttributes</code>, summed across all
                            of the secondary indexes, must not exceed 100. If you project the same
                            attribute into two different indexes, this counts as two distinct
                            attributes when determining the total. This limit only applies when you
                            specify the ProjectionType of <code>INCLUDE</code>. You still can
                            specify the ProjectionType of <code>ALL</code> to project all attributes
                            from the source table, even if the table has more than 100
                            attributes.</p>
                  </li>
               </ul>
            </li>
         </ul> |
| `provisioned_throughput` | String |  | <p>Represents the provisioned throughput settings for a specified table or index. The
            settings can be modified using the <code>UpdateTable</code> operation.</p>
         <p> If you set BillingMode as <code>PROVISIONED</code>, you must specify this property.
            If you set BillingMode as <code>PAY_PER_REQUEST</code>, you cannot specify this
            property.</p>
         <p>For current minimum and maximum provisioned throughput values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service,
                Account, and Table Quotas</a> in the <i>Amazon DynamoDB Developer
                Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table` | String | <p>The properties of the table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create table
table = provider.dynamodb.Table {
    key_schema = "value"  # <p>Specifies the attributes that make up the primary key for a table or an index. The
            attributes in <code>KeySchema</code> must also be defined in the
                <code>AttributeDefinitions</code> array. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html">Data
                Model</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
         <p>Each <code>KeySchemaElement</code> in the array is composed of:</p>
         <ul>
            <li>
               <p>
                  <code>AttributeName</code> - The name of this key attribute.</p>
            </li>
            <li>
               <p>
                  <code>KeyType</code> - The role that the key attribute will assume:</p>
               <ul>
                  <li>
                     <p>
                        <code>HASH</code> - partition key</p>
                  </li>
                  <li>
                     <p>
                        <code>RANGE</code> - sort key</p>
                  </li>
               </ul>
            </li>
         </ul>
         <note>
            <p>The partition key of an item is also known as its <i>hash
                    attribute</i>. The term "hash attribute" derives from the DynamoDB usage
                of an internal hash function to evenly distribute data items across partitions,
                based on their partition key values.</p>
            <p>The sort key of an item is also known as its <i>range attribute</i>.
                The term "range attribute" derives from the way DynamoDB stores items with the same
                partition key physically close together, in sorted order by the sort key
                value.</p>
         </note>
         <p>For a simple primary key (partition key), you must provide exactly one element with a
                <code>KeyType</code> of <code>HASH</code>.</p>
         <p>For a composite primary key (partition key and sort key), you must provide exactly two
            elements, in this order: The first element must have a <code>KeyType</code> of
                <code>HASH</code>, and the second element must have a <code>KeyType</code> of
                <code>RANGE</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key">Working with Tables</a> in the <i>Amazon DynamoDB Developer
                Guide</i>.</p>
    attribute_definitions = "value"  # <p>An array of attributes that describe the key schema for the table and indexes.</p>
    table_name = "value"  # <p>The name of the table to create. You can also provide the Amazon Resource Name (ARN) of the table in
            this parameter.</p>
}

# Access table outputs
table_id = table.id
table_table = table.table
```

---


### Time_to_live

TimeToLive resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_to_live_specification` | String | ✅ | <p>Represents the settings used to enable or disable Time to Live for the specified
            table.</p> |
| `table_name` | String | ✅ | <p>The name of the table to be configured. You can also provide the Amazon Resource Name (ARN) of the
            table in this parameter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_to_live_description` | String | <p></p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access time_to_live outputs
time_to_live_id = time_to_live.id
time_to_live_time_to_live_description = time_to_live.time_to_live_description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_policy resources
resource_policy_0 = provider.dynamodb.Resource_policy {
    resource_arn = "value-0"
    policy = "value-0"
}
resource_policy_1 = provider.dynamodb.Resource_policy {
    resource_arn = "value-1"
    policy = "value-1"
}
resource_policy_2 = provider.dynamodb.Resource_policy {
    resource_arn = "value-2"
    policy = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_policy = provider.dynamodb.Resource_policy {
        resource_arn = "production-value"
        policy = "production-value"
    }
```

---

## Related Documentation

- [AWS Dynamodb Documentation](https://docs.aws.amazon.com/dynamodb/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
