# Inspector_scan Service



**Resources**: 0

---

## Overview

The inspector_scan service provides access to 0 resource types:


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

- [AWS Inspector_scan Documentation](https://docs.aws.amazon.com/inspector_scan/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
