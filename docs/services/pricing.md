# Pricing Service



**Resources**: 4

---

## Overview

The pricing service provides access to 4 resource types:

- [Products](#products) [R]
- [Price_list_file_url](#price_list_file_url) [R]
- [Services](#services) [R]
- [Attribute_values](#attribute_values) [R]

---

## Resources


### Products

Products resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token that indicates the next set of results to retrieve.</p> |
| `format_version` | String | <p>The format version of the response. For example, aws_v1.</p> |
| `price_list` | Vec<String> | <p>The list of products that match your filters. The list contains both the product metadata and 
         the price information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access products outputs
products_id = products.id
products_next_token = products.next_token
products_format_version = products.format_version
products_price_list = products.price_list
```

---


### Price_list_file_url

PriceListFileUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | <p>The URL to download your Price List file from. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access price_list_file_url outputs
price_list_file_url_id = price_list_file_url.id
price_list_file_url_url = price_list_file_url.url
```

---


### Services

Services resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `format_version` | String | <p>The format version of the response. For example, <code>aws_v1</code>.</p> |
| `next_token` | String | <p>The pagination token for the next set of retrievable results.</p> |
| `services` | Vec<String> | <p>The service metadata for the service or services in the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access services outputs
services_id = services.id
services_format_version = services.format_version
services_next_token = services.next_token
services_services = services.services
```

---


### Attribute_values

AttributeValues resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attribute_values` | Vec<String> | <p>The list of values for an attribute. For example, <code>Throughput Optimized HDD</code> and 
      <code>Provisioned IOPS</code> are two available values for the <code>AmazonEC2</code>
            <code>volumeType</code>.</p> |
| `next_token` | String | <p>The pagination token that indicates the next set of results to retrieve.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access attribute_values outputs
attribute_values_id = attribute_values.id
attribute_values_attribute_values = attribute_values.attribute_values
attribute_values_next_token = attribute_values.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple products resources
products_0 = provider.pricing.Products {
}
products_1 = provider.pricing.Products {
}
products_2 = provider.pricing.Products {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    products = provider.pricing.Products {
    }
```

---

## Related Documentation

- [AWS Pricing Documentation](https://docs.aws.amazon.com/pricing/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
