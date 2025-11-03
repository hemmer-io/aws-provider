# Route53profiles Service



**Resources**: 3

---

## Overview

The route53profiles service provides access to 3 resource types:

- [Profile_association](#profile_association) [R]
- [Profile_resource_association](#profile_resource_association) [RU]
- [Profile](#profile) [CRD]

---

## Resources


### Profile_association

ProfileAssociation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile_association` | String | <p>
  Information about the Profile association that you specified in a <code>GetProfileAssociation</code> request.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access profile_association outputs
profile_association_id = profile_association.id
profile_association_profile_association = profile_association.profile_association
```

---


### Profile_resource_association

ProfileResourceAssociation resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>
Name of the resource association.
</p> |
| `resource_properties` | String |  | <p>
  If you are adding a DNS Firewall rule group, include also a priority. The priority indicates the processing order for the rule groups, starting with the priority assinged the lowest value.
  </p>
         <p>The allowed values for priority are between 100 and 9900.</p> |
| `profile_resource_association_id` | String | ✅ | <p>
ID of the resource association.
</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile_resource_association` | String | <p>
  Information about the Profile resource association that you specified in a <code>GetProfileResourceAssociation</code> request.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access profile_resource_association outputs
profile_resource_association_id = profile_resource_association.id
profile_resource_association_profile_resource_association = profile_resource_association.profile_resource_association
```

---


### Profile

Profile resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String | ✅ | <p>
            <code>ClientToken</code> is an idempotency token that ensures a call to <code>CreateProfile</code> completes only once. You choose the value to pass. 
  For example, an issue might prevent you from getting a response from <code>CreateProfile</code>. 
  In this case, safely retry your call to <code>CreateProfile</code> by using the same <code>CreateProfile</code> parameter value.
</p> |
| `name` | String | ✅ | <p>
  A name for the Profile.
</p> |
| `tags` | Vec<String> |  | <p>
  A list of the tag keys and values that you want to associate with the Route 53 Profile.
</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile` | String | <p>
  Information about the Profile, including the status of the Profile.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile
profile = provider.route53profiles.Profile {
    client_token = "value"  # <p>
            <code>ClientToken</code> is an idempotency token that ensures a call to <code>CreateProfile</code> completes only once. You choose the value to pass. 
  For example, an issue might prevent you from getting a response from <code>CreateProfile</code>. 
  In this case, safely retry your call to <code>CreateProfile</code> by using the same <code>CreateProfile</code> parameter value.
</p>
    name = "value"  # <p>
  A name for the Profile.
</p>
}

# Access profile outputs
profile_id = profile.id
profile_profile = profile.profile
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple profile_association resources
profile_association_0 = provider.route53profiles.Profile_association {
}
profile_association_1 = provider.route53profiles.Profile_association {
}
profile_association_2 = provider.route53profiles.Profile_association {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    profile_association = provider.route53profiles.Profile_association {
    }
```

---

## Related Documentation

- [AWS Route53profiles Documentation](https://docs.aws.amazon.com/route53profiles/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
