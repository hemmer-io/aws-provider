# Migrationhub_config Service



**Resources**: 3

---

## Overview

The migrationhub_config service provides access to 3 resource types:

- [Home_region](#home_region) [R]
- [Home_region_controls](#home_region_controls) [R]
- [Home_region_control](#home_region_control) [CD]

---

## Resources


### Home_region

HomeRegion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `home_region` | String | <p>The name of the home region of the calling account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access home_region outputs
home_region_id = home_region.id
home_region_home_region = home_region.home_region
```

---


### Home_region_controls

HomeRegionControls resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If a <code>NextToken</code> was returned by a previous call, more results are available.
      To retrieve the next page of results, make the call again using the returned token in
        <code>NextToken</code>.</p> |
| `home_region_controls` | Vec<String> | <p>An array that contains your <code>HomeRegionControl</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access home_region_controls outputs
home_region_controls_id = home_region_controls.id
home_region_controls_next_token = home_region_controls.next_token
home_region_controls_home_region_controls = home_region_controls.home_region_controls
```

---


### Home_region_control

HomeRegionControl resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether
      the caller has permission to make the call.</p> |
| `home_region` | String | ✅ | <p>The name of the home region of the calling account.</p> |
| `target` | String | ✅ | <p>The account for which this command sets up a home region control. The <code>Target</code>
      is always of type <code>ACCOUNT</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create home_region_control
home_region_control = provider.migrationhub_config.Home_region_control {
    home_region = "value"  # <p>The name of the home region of the calling account.</p>
    target = "value"  # <p>The account for which this command sets up a home region control. The <code>Target</code>
      is always of type <code>ACCOUNT</code>.</p>
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

# Create multiple home_region resources
home_region_0 = provider.migrationhub_config.Home_region {
}
home_region_1 = provider.migrationhub_config.Home_region {
}
home_region_2 = provider.migrationhub_config.Home_region {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    home_region = provider.migrationhub_config.Home_region {
    }
```

---

## Related Documentation

- [AWS Migrationhub_config Documentation](https://docs.aws.amazon.com/migrationhub_config/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
