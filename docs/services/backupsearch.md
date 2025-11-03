# Backupsearch Service



**Resources**: 0

---

## Overview

The backupsearch service provides access to 0 resource types:


---

## Resources



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
```

---

## Related Documentation

- [AWS Backupsearch Documentation](https://docs.aws.amazon.com/backupsearch/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
