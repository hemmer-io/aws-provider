# Mediastore_data Service



**Resources**: 1

---

## Overview

The mediastore_data service provides access to 1 resource type:

- [Object](#object) [CRD]

---

## Resources


### Object

Object resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | String | ✅ | <p>The path (including the file name) where the object is stored in the container.
         Format: <folder name>/<folder name>/<file name></p>
         <p>For example, to upload the file <code>mlaw.avi</code> to the folder path
            <code>premium\canada</code> in the container <code>movies</code>, enter the path
            <code>premium/canada/mlaw.avi</code>.</p>
         <p>Do not include the container name in this path.</p>
         <p>If the path includes any folders that don't exist yet, the service creates them. For
         example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify
            <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the
            <code>premium</code> folder. You then have two subfolders, <code>usa</code> and
            <code>canada</code>, in the <code>premium</code> folder. </p>
         <p>There is no correlation between the path to the source and the path (folders) in the
         container in AWS Elemental MediaStore.</p>
         <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User
            Guide</a>.</p>
         <p>The file name is the name that is assigned to the file that you upload. The file can
         have the same name inside and outside of AWS Elemental MediaStore, or it can have the same
         name. The file name can include or omit an extension. </p> |
| `upload_availability` | String |  | <p>Indicates the availability of an object while it is still uploading. If the value is set to <code>streaming</code>, the object is available for
            downloading after some initial buffering but before the object is uploaded completely. If the value is set to <code>standard</code>, the object is
            available for downloading only when it is uploaded completely. The default value for this header is <code>standard</code>.</p>
        <p>To use this header, you must also set the HTTP <code>Transfer-Encoding</code> header to <code>chunked</code>.</p> |
| `cache_control` | String |  | <p>An optional <code>CacheControl</code> header that allows the caller to control the
         object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
         <p>Headers with a custom user-defined value are also accepted.</p> |
| `storage_class` | String |  | <p>Indicates the storage class of a <code>Put</code> request. Defaults to
         high-performance temporal storage class, and objects are persisted into durable storage
         shortly after being received.</p> |
| `content_type` | String |  | <p>The content type of the object.</p> |
| `body` | String | ✅ | <p>The bytes to be stored. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The ETag that represents a unique instance of the object.</p> |
| `last_modified` | String | <p>The date and time that the object was last modified.</p> |
| `cache_control` | String | <p>An optional <code>CacheControl</code> header that allows the caller to control the
         object's cache behavior. Headers can be passed in as specified in the HTTP spec at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
         <p>Headers with a custom user-defined value are also accepted.</p> |
| `content_length` | i64 | <p>The length of the object in bytes.</p> |
| `content_range` | String | <p>The range of bytes to retrieve.</p> |
| `body` | String | <p>The bytes of the object. </p> |
| `status_code` | i64 | <p>The HTML status code of the request. Status codes ranging from 200 to 299 indicate
         success. All other status codes indicate the type of error that occurred.</p> |
| `content_type` | String | <p>The content type of the object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create object
object = provider.mediastore_data.Object {
    path = "value"  # <p>The path (including the file name) where the object is stored in the container.
         Format: <folder name>/<folder name>/<file name></p>
         <p>For example, to upload the file <code>mlaw.avi</code> to the folder path
            <code>premium\canada</code> in the container <code>movies</code>, enter the path
            <code>premium/canada/mlaw.avi</code>.</p>
         <p>Do not include the container name in this path.</p>
         <p>If the path includes any folders that don't exist yet, the service creates them. For
         example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify
            <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the
            <code>premium</code> folder. You then have two subfolders, <code>usa</code> and
            <code>canada</code>, in the <code>premium</code> folder. </p>
         <p>There is no correlation between the path to the source and the path (folders) in the
         container in AWS Elemental MediaStore.</p>
         <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User
            Guide</a>.</p>
         <p>The file name is the name that is assigned to the file that you upload. The file can
         have the same name inside and outside of AWS Elemental MediaStore, or it can have the same
         name. The file name can include or omit an extension. </p>
    body = "value"  # <p>The bytes to be stored. </p>
}

# Access object outputs
object_id = object.id
object_e_tag = object.e_tag
object_last_modified = object.last_modified
object_cache_control = object.cache_control
object_content_length = object.content_length
object_content_range = object.content_range
object_body = object.body
object_status_code = object.status_code
object_content_type = object.content_type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple object resources
object_0 = provider.mediastore_data.Object {
    path = "value-0"
    body = "value-0"
}
object_1 = provider.mediastore_data.Object {
    path = "value-1"
    body = "value-1"
}
object_2 = provider.mediastore_data.Object {
    path = "value-2"
    body = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    object = provider.mediastore_data.Object {
        path = "production-value"
        body = "production-value"
    }
```

---

## Related Documentation

- [AWS Mediastore_data Documentation](https://docs.aws.amazon.com/mediastore_data/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
