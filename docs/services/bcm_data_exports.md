# Bcm_data_exports Service



**Resources**: 3

---

## Overview

The bcm_data_exports service provides access to 3 resource types:

- [Export](#export) [CRUD]
- [Execution](#execution) [R]
- [Table](#table) [R]

---

## Resources


### Export

Export resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `export` | String | ✅ | <p>The details of the export, including data query, name, description, and destination
      configuration.</p> |
| `resource_tags` | Vec<String> |  | <p>An optional list of tags to associate with the specified export. Each tag consists of a
      key and a value, and each key must be unique for the resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export` | String | <p>The data for this specific export.</p> |
| `export_status` | String | <p>The status of this specific export.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create export
export = provider.bcm_data_exports.Export {
    export = "value"  # <p>The details of the export, including data query, name, description, and destination
      configuration.</p>
}

# Access export outputs
export_id = export.id
export_export = export.export
export_export_status = export.export_status
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
| `export` | String | <p>The export data for this specific execution. This export data is a snapshot from when the
      execution was generated. The data could be different from the current export data if the
      export was updated since the execution was generated.</p> |
| `execution_id` | String | <p>The ID for this specific execution.</p> |
| `execution_status` | String | <p>The status of this specific execution.</p> |


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
execution_export = execution.export
execution_execution_id = execution.execution_id
execution_execution_status = execution.execution_status
```

---


### Table

Table resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_name` | String | <p>The name of the table.</p> |
| `description` | String | <p>The table description.</p> |
| `schema` | Vec<String> | <p>The schema of the table.</p> |
| `table_properties` | HashMap<String, String> | <p>TableProperties are additional configurations you can provide to change the data and
      schema of a table. Each table can have different TableProperties. Tables are not required to
      have any TableProperties. Each table property has a default value that it assumes if not
      specified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table outputs
table_id = table.id
table_table_name = table.table_name
table_description = table.description
table_schema = table.schema
table_table_properties = table.table_properties
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple export resources
export_0 = provider.bcm_data_exports.Export {
    export = "value-0"
}
export_1 = provider.bcm_data_exports.Export {
    export = "value-1"
}
export_2 = provider.bcm_data_exports.Export {
    export = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    export = provider.bcm_data_exports.Export {
        export = "production-value"
    }
```

---

## Related Documentation

- [AWS Bcm_data_exports Documentation](https://docs.aws.amazon.com/bcm_data_exports/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
