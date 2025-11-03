# Amp Service



**Resources**: 1

---

## Overview

The amp service provides access to 1 resource type:

- [Default_scraper_configuration](#default_scraper_configuration) [R]

---

## Resources


### Default_scraper_configuration

DefaultScraperConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration` | String | <p>The configuration file. Base 64 encoded. For more information, see <a href="https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html#AMP-collector-configuration">Scraper configuration</a>in the <i>Amazon Managed Service for Prometheus User Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_scraper_configuration outputs
default_scraper_configuration_id = default_scraper_configuration.id
default_scraper_configuration_configuration = default_scraper_configuration.configuration
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple default_scraper_configuration resources
default_scraper_configuration_0 = provider.amp.Default_scraper_configuration {
}
default_scraper_configuration_1 = provider.amp.Default_scraper_configuration {
}
default_scraper_configuration_2 = provider.amp.Default_scraper_configuration {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    default_scraper_configuration = provider.amp.Default_scraper_configuration {
    }
```

---

## Related Documentation

- [AWS Amp Documentation](https://docs.aws.amazon.com/amp/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
