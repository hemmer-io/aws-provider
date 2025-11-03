# S3outposts Service



**Resources**: 1

---

## Overview

The s3outposts service provides access to 1 resource type:

- [Endpoint](#endpoint) [CD]

---

## Resources


### Endpoint

Endpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_group_id` | String | ✅ | <p>The ID of the security group to use with the endpoint.</p> |
| `customer_owned_ipv4_pool` | String |  | <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses
            are allocated from this pool for the endpoint.</p> |
| `outpost_id` | String | ✅ | <p>The ID of the Outposts. </p> |
| `access_type` | String |  | <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use
            the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises
            network, choose <code>CustomerOwnedIp</code>.  If you choose
                <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address
            pool (CoIP pool).</p>
         <note>
            <p>
               <code>Private</code> is the default access type value.</p>
         </note> |
| `subnet_id` | String | ✅ | <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost
            that has Amazon S3 on Outposts provisioned.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint
endpoint = provider.s3outposts.Endpoint {
    security_group_id = "value"  # <p>The ID of the security group to use with the endpoint.</p>
    outpost_id = "value"  # <p>The ID of the Outposts. </p>
    subnet_id = "value"  # <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost
            that has Amazon S3 on Outposts provisioned.</p>
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

# Create multiple endpoint resources
endpoint_0 = provider.s3outposts.Endpoint {
    security_group_id = "value-0"
    outpost_id = "value-0"
    subnet_id = "value-0"
}
endpoint_1 = provider.s3outposts.Endpoint {
    security_group_id = "value-1"
    outpost_id = "value-1"
    subnet_id = "value-1"
}
endpoint_2 = provider.s3outposts.Endpoint {
    security_group_id = "value-2"
    outpost_id = "value-2"
    subnet_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    endpoint = provider.s3outposts.Endpoint {
        security_group_id = "production-value"
        outpost_id = "production-value"
        subnet_id = "production-value"
    }
```

---

## Related Documentation

- [AWS S3outposts Documentation](https://docs.aws.amazon.com/s3outposts/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
