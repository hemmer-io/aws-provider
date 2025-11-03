# Clouddirectory Service



**Resources**: 13

---

## Overview

The clouddirectory service provides access to 13 resource types:

- [Schema_as_json](#schema_as_json) [R]
- [Object_attributes](#object_attributes) [RU]
- [Applied_schema_version](#applied_schema_version) [R]
- [Index](#index) [C]
- [Object_information](#object_information) [R]
- [Schema_from_json](#schema_from_json) [C]
- [Directory](#directory) [CRD]
- [Schema](#schema) [CUD]
- [Typed_link_facet](#typed_link_facet) [CUD]
- [Object](#object) [CD]
- [Link_attributes](#link_attributes) [RU]
- [Typed_link_facet_information](#typed_link_facet_information) [R]
- [Facet](#facet) [CRUD]

---

## Resources


### Schema_as_json

SchemaAsJson resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the retrieved schema.</p> |
| `document` | String | <p>The JSON representation of the schema document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access schema_as_json outputs
schema_as_json_id = schema_as_json.id
schema_as_json_name = schema_as_json.name
schema_as_json_document = schema_as_json.document
```

---


### Object_attributes

ObjectAttributes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `object_reference` | String | ✅ | <p>The reference that identifies the object.</p> |
| `directory_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>
      where the object resides. For more information, see <a>arns</a>.</p> |
| `attribute_updates` | Vec<String> | ✅ | <p>The attributes update structure.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | Vec<String> | <p>The attributes that are associated with the object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access object_attributes outputs
object_attributes_id = object_attributes.id
object_attributes_attributes = object_attributes.attributes
```

---


### Applied_schema_version

AppliedSchemaVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `applied_schema_arn` | String | <p>Current applied schema ARN, including the minor version in use if one was provided.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access applied_schema_version outputs
applied_schema_version_id = applied_schema_version.id
applied_schema_version_applied_schema_arn = applied_schema_version.applied_schema_arn
```

---


### Index

Index resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `directory_arn` | String | ✅ | <p>The ARN of the directory where the index should be created.</p> |
| `is_unique` | bool | ✅ | <p>Indicates whether the attribute that is being indexed has unique values or
      not.</p> |
| `ordered_indexed_attribute_list` | Vec<String> | ✅ | <p>Specifies the attributes that should be indexed on. Currently only a single attribute
      is supported.</p> |
| `parent_reference` | String |  | <p>A reference to the parent object that contains the index object.</p> |
| `link_name` | String |  | <p>The name of the link between the parent object and the index object.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create index
index = provider.clouddirectory.Index {
    directory_arn = "value"  # <p>The ARN of the directory where the index should be created.</p>
    is_unique = "value"  # <p>Indicates whether the attribute that is being indexed has unique values or
      not.</p>
    ordered_indexed_attribute_list = "value"  # <p>Specifies the attributes that should be indexed on. Currently only a single attribute
      is supported.</p>
}

```

---


### Object_information

ObjectInformation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_facets` | Vec<String> | <p>The facets attached to the specified object. Although the response does not include minor version information, the most recently applied minor version of each Facet is in effect. See <a>GetAppliedSchemaVersion</a> for details.</p> |
| `object_identifier` | String | <p>The <code>ObjectIdentifier</code> of the specified object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access object_information outputs
object_information_id = object_information.id
object_information_schema_facets = object_information.schema_facets
object_information_object_identifier = object_information.object_identifier
```

---


### Schema_from_json

SchemaFromJson resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_arn` | String | ✅ | <p>The ARN of the schema to update.</p> |
| `document` | String | ✅ | <p>The replacement JSON schema.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schema_from_json
schema_from_json = provider.clouddirectory.Schema_from_json {
    schema_arn = "value"  # <p>The ARN of the schema to update.</p>
    document = "value"  # <p>The replacement JSON schema.</p>
}

```

---


### Directory

Directory resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the published schema that will be copied into the
      data <a>Directory</a>. For more information, see <a>arns</a>.</p> |
| `name` | String | ✅ | <p>The name of the <a>Directory</a>. Should be unique per account, per
      region.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `directory` | String | <p>Metadata about the directory.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create directory
directory = provider.clouddirectory.Directory {
    schema_arn = "value"  # <p>The Amazon Resource Name (ARN) of the published schema that will be copied into the
      data <a>Directory</a>. For more information, see <a>arns</a>.</p>
    name = "value"  # <p>The name of the <a>Directory</a>. Should be unique per account, per
      region.</p>
}

# Access directory outputs
directory_id = directory.id
directory_directory = directory.directory
```

---


### Schema

Schema resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name that is associated with the schema. This is unique to each account and in each
      region.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schema
schema = provider.clouddirectory.Schema {
    name = "value"  # <p>The name that is associated with the schema. This is unique to each account and in each
      region.</p>
}

```

---


### Typed_link_facet

TypedLinkFacet resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that is associated with the schema. For more
      information, see <a>arns</a>.</p> |
| `facet` | String | ✅ | <p>
            <a>Facet</a> structure that is associated with the typed link
      facet.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create typed_link_facet
typed_link_facet = provider.clouddirectory.Typed_link_facet {
    schema_arn = "value"  # <p>The Amazon Resource Name (ARN) that is associated with the schema. For more
      information, see <a>arns</a>.</p>
    facet = "value"  # <p>
            <a>Facet</a> structure that is associated with the typed link
      facet.</p>
}

```

---


### Object

Object resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_reference` | String |  | <p>If specified, the parent reference to which this object will be attached.</p> |
| `directory_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>
      in which the object will be created. For more information, see <a>arns</a>.</p> |
| `object_attribute_list` | Vec<String> |  | <p>The attribute map whose attribute ARN contains the key and attribute value as the map
      value.</p> |
| `schema_facets` | Vec<String> | ✅ | <p>A list of schema facets to be associated with the object. Do not provide minor version components. See <a>SchemaFacet</a> for details.</p> |
| `link_name` | String |  | <p>The name of link that is used to attach this object to a parent.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create object
object = provider.clouddirectory.Object {
    directory_arn = "value"  # <p>The Amazon Resource Name (ARN) that is associated with the <a>Directory</a>
      in which the object will be created. For more information, see <a>arns</a>.</p>
    schema_facets = "value"  # <p>A list of schema facets to be associated with the object. Do not provide minor version components. See <a>SchemaFacet</a> for details.</p>
}

```

---


### Link_attributes

LinkAttributes resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `typed_link_specifier` | String | ✅ | <p>Allows a typed link specifier to be accepted as input.</p> |
| `directory_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that is associated with the Directory where the updated typed link resides. For more information, see <a>arns</a> or <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p> |
| `attribute_updates` | Vec<String> | ✅ | <p>The attributes update structure.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | Vec<String> | <p>The attributes that are associated with the typed link.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access link_attributes outputs
link_attributes_id = link_attributes.id
link_attributes_attributes = link_attributes.attributes
```

---


### Typed_link_facet_information

TypedLinkFacetInformation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_attribute_order` | Vec<String> | <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed
      links considers the order that the attributes are defined on the typed link facet. When
      providing ranges to typed link selection, any inexact ranges must be specified at the end. Any
      attributes that do not have a range specified are presumed to match the entire range. Filters
      are interpreted in the order of the attributes on the typed link facet, not the order in which
      they are supplied to any API calls. For more information about identity attributes, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access typed_link_facet_information outputs
typed_link_facet_information_id = typed_link_facet_information.id
typed_link_facet_information_identity_attribute_order = typed_link_facet_information.identity_attribute_order
```

---


### Facet

Facet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the <a>Facet</a>, which is unique for a given schema.</p> |
| `attributes` | Vec<String> |  | <p>The attributes that are associated with the <a>Facet</a>.</p> |
| `object_type` | String |  | <p>Specifies whether a given object created from this facet is of type node, leaf node,
      policy or index.</p>
         <ul>
            <li>
               <p>Node: Can have multiple children but one parent.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>Leaf node: Cannot have children but can have multiple parents.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>Policy: Allows you to store a policy document and policy type. For more
        information, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_directory.html#key_concepts_policies">Policies</a>.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>Index: Can be created with the Index API.</p>
            </li>
         </ul> |
| `facet_style` | String |  | <p>There are two different styles that you can define on any given facet, <code>Static</code> and <code>Dynamic</code>. For static facets, all attributes must be defined in the schema. For dynamic facets, attributes can be defined during data plane operations.</p> |
| `schema_arn` | String | ✅ | <p>The schema ARN in which the new <a>Facet</a> will be created. For more
      information, see <a>arns</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `facet` | String | <p>The <a>Facet</a> structure that is associated with the facet.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create facet
facet = provider.clouddirectory.Facet {
    name = "value"  # <p>The name of the <a>Facet</a>, which is unique for a given schema.</p>
    schema_arn = "value"  # <p>The schema ARN in which the new <a>Facet</a> will be created. For more
      information, see <a>arns</a>.</p>
}

# Access facet outputs
facet_id = facet.id
facet_facet = facet.facet
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple schema_as_json resources
schema_as_json_0 = provider.clouddirectory.Schema_as_json {
}
schema_as_json_1 = provider.clouddirectory.Schema_as_json {
}
schema_as_json_2 = provider.clouddirectory.Schema_as_json {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    schema_as_json = provider.clouddirectory.Schema_as_json {
    }
```

---

## Related Documentation

- [AWS Clouddirectory Documentation](https://docs.aws.amazon.com/clouddirectory/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
