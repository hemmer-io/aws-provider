# Amplifyuibuilder Service



**Resources**: 2

---

## Overview

The amplifyuibuilder service provides access to 2 resource types:

- [Metadata_flag](#metadata_flag) [C]
- [Metadata](#metadata) [R]

---

## Resources


### Metadata_flag

MetadataFlag resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The unique ID for the Amplify app.</p> |
| `feature_name` | String | ✅ | <p>The name of the feature associated with the metadata.</p> |
| `environment_name` | String | ✅ | <p>The name of the backend environment that is part of the Amplify app.</p> |
| `body` | String | ✅ | <p>The metadata information to store.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metadata_flag
metadata_flag = provider.amplifyuibuilder.Metadata_flag {
    app_id = "value"  # <p>The unique ID for the Amplify app.</p>
    feature_name = "value"  # <p>The name of the feature associated with the metadata.</p>
    environment_name = "value"  # <p>The name of the backend environment that is part of the Amplify app.</p>
    body = "value"  # <p>The metadata information to store.</p>
}

```

---


### Metadata

Metadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `features` | HashMap<String, String> | <p>Represents the configuration settings for the features metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metadata outputs
metadata_id = metadata.id
metadata_features = metadata.features
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple metadata_flag resources
metadata_flag_0 = provider.amplifyuibuilder.Metadata_flag {
    app_id = "value-0"
    feature_name = "value-0"
    environment_name = "value-0"
    body = "value-0"
}
metadata_flag_1 = provider.amplifyuibuilder.Metadata_flag {
    app_id = "value-1"
    feature_name = "value-1"
    environment_name = "value-1"
    body = "value-1"
}
metadata_flag_2 = provider.amplifyuibuilder.Metadata_flag {
    app_id = "value-2"
    feature_name = "value-2"
    environment_name = "value-2"
    body = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    metadata_flag = provider.amplifyuibuilder.Metadata_flag {
        app_id = "production-value"
        feature_name = "production-value"
        environment_name = "production-value"
        body = "production-value"
    }
```

---

## Related Documentation

- [AWS Amplifyuibuilder Documentation](https://docs.aws.amazon.com/amplifyuibuilder/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
