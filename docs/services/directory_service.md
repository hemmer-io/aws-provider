# Directory_service Service



**Resources**: 31

---

## Overview

The directory_service service provides access to 31 resource types:

- [Directory](#directory) [CD]
- [Certificate](#certificate) [R]
- [Client_authentication_settings](#client_authentication_settings) [R]
- [Event_topics](#event_topics) [R]
- [Regions](#regions) [R]
- [Directory_data_access](#directory_data_access) [R]
- [Number_of_domain_controllers](#number_of_domain_controllers) [U]
- [Microsoft_ad](#microsoft_ad) [C]
- [Shared_directories](#shared_directories) [R]
- [Domain_controllers](#domain_controllers) [R]
- [Conditional_forwarder](#conditional_forwarder) [CUD]
- [Directory_setup](#directory_setup) [U]
- [Update_directory](#update_directory) [R]
- [Computer](#computer) [C]
- [Directories](#directories) [R]
- [Alias](#alias) [C]
- [Ldaps_settings](#ldaps_settings) [R]
- [Hybrid_ad](#hybrid_ad) [CU]
- [Radius](#radius) [U]
- [Snapshot](#snapshot) [CD]
- [Snapshot_limits](#snapshot_limits) [R]
- [Snapshots](#snapshots) [R]
- [Trust](#trust) [CUD]
- [Ad_assessment](#ad_assessment) [RD]
- [Settings](#settings) [RU]
- [Log_subscription](#log_subscription) [CD]
- [Trusts](#trusts) [R]
- [Directory_limits](#directory_limits) [R]
- [Hybrid_ad_update](#hybrid_ad_update) [R]
- [Conditional_forwarders](#conditional_forwarders) [R]
- [Ca_enrollment_policy](#ca_enrollment_policy) [R]

---

## Resources


### Directory

Directory resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p> |
| `network_type` | String |  | <p>The network type for your directory. Simple AD supports IPv4 and Dual-stack only.</p> |
| `password` | String | ✅ | <p>The password for the directory administrator. The directory creation process creates a
      directory administrator account with the user name <code>Administrator</code> and this
      password.</p>
         <p>If you need to change the password for the administrator account, you can use the <a>ResetUserPassword</a> API call.</p>
         <p>The regex pattern for this string is made up of the following conditions:</p>
         <ul>
            <li>
               <p>Length (?=^.{8,64}$) – Must be between 8 and 64 characters</p>
            </li>
         </ul>
         <p>AND any 3 of the following password complexity rules required by Active Directory:</p>
         <ul>
            <li>
               <p>Numbers and upper case and lowercase (?=.*\d)(?=.*[A-Z])(?=.*[a-z])</p>
            </li>
            <li>
               <p>Numbers and special characters and lower case
          (?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])</p>
            </li>
            <li>
               <p>Special characters and upper case and lower case
          (?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])</p>
            </li>
            <li>
               <p>Numbers and upper case and special characters
          (?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s])</p>
            </li>
         </ul>
         <p>For additional information about how Active Directory passwords are enforced, see <a href="https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/password-must-meet-complexity-requirements">Password must meet complexity requirements</a> on the Microsoft website.</p> |
| `description` | String |  | <p>A description for the directory.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the Simple AD directory.</p> |
| `short_name` | String |  | <p>The NetBIOS name of the directory, such as <code>CORP</code>.</p> |
| `size` | String | ✅ | <p>The size of the directory.</p> |
| `vpc_settings` | String |  | <p>A <a>DirectoryVpcSettings</a> object that contains additional information for
      the operation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create directory
directory = provider.directory_service.Directory {
    name = "value"  # <p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p>
    password = "value"  # <p>The password for the directory administrator. The directory creation process creates a
      directory administrator account with the user name <code>Administrator</code> and this
      password.</p>
         <p>If you need to change the password for the administrator account, you can use the <a>ResetUserPassword</a> API call.</p>
         <p>The regex pattern for this string is made up of the following conditions:</p>
         <ul>
            <li>
               <p>Length (?=^.{8,64}$) – Must be between 8 and 64 characters</p>
            </li>
         </ul>
         <p>AND any 3 of the following password complexity rules required by Active Directory:</p>
         <ul>
            <li>
               <p>Numbers and upper case and lowercase (?=.*\d)(?=.*[A-Z])(?=.*[a-z])</p>
            </li>
            <li>
               <p>Numbers and special characters and lower case
          (?=.*\d)(?=.*[^A-Za-z0-9\s])(?=.*[a-z])</p>
            </li>
            <li>
               <p>Special characters and upper case and lower case
          (?=.*[^A-Za-z0-9\s])(?=.*[A-Z])(?=.*[a-z])</p>
            </li>
            <li>
               <p>Numbers and upper case and special characters
          (?=.*\d)(?=.*[A-Z])(?=.*[^A-Za-z0-9\s])</p>
            </li>
         </ul>
         <p>For additional information about how Active Directory passwords are enforced, see <a href="https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/password-must-meet-complexity-requirements">Password must meet complexity requirements</a> on the Microsoft website.</p>
    size = "value"  # <p>The size of the directory.</p>
}

```

---


### Certificate

Certificate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate` | String | <p>Information about the certificate, including registered date time, certificate state, the
      reason for the state, expiration date time, and certificate common name.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificate outputs
certificate_id = certificate.id
certificate_certificate = certificate.certificate
```

---


### Client_authentication_settings

ClientAuthenticationSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_authentication_settings_info` | Vec<String> | <p>Information about the type of client authentication for the specified directory. The
      following information is retrieved: The date and time when the status of the client
      authentication type was last updated, whether the client authentication type is enabled or
      disabled, and the type of client authentication.</p> |
| `next_token` | String | <p>The next token used to retrieve the client authentication settings if the number of
      setting types exceeds page limit and there is another page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_authentication_settings outputs
client_authentication_settings_id = client_authentication_settings.id
client_authentication_settings_client_authentication_settings_info = client_authentication_settings.client_authentication_settings_info
client_authentication_settings_next_token = client_authentication_settings.next_token
```

---


### Event_topics

EventTopics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_topics` | Vec<String> | <p>A list of Amazon SNS topic names that receive status messages from the specified Directory
      ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_topics outputs
event_topics_id = event_topics.id
event_topics_event_topics = event_topics.event_topics
```

---


### Regions

Regions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regions_description` | Vec<String> | <p>List of Region information related to the directory for each replicated Region.</p> |
| `next_token` | String | <p>If not null, more results are available. Pass this value for the <code>NextToken</code>
      parameter in a subsequent call to <a>DescribeRegions</a> to retrieve the next set
      of items.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access regions outputs
regions_id = regions.id
regions_regions_description = regions.regions_description
regions_next_token = regions.next_token
```

---


### Directory_data_access

DirectoryDataAccess resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_access_status` | String | <p>The current status of data access through the Directory Service Data API.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access directory_data_access outputs
directory_data_access_id = directory_data_access.id
directory_data_access_data_access_status = directory_data_access.data_access_status
```

---


### Number_of_domain_controllers

NumberOfDomainControllers resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `desired_number` | i64 | ✅ | <p>The number of domain controllers desired in the directory.</p> |
| `directory_id` | String | ✅ | <p>Identifier of the directory to which the domain controllers will be added or
      removed.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Microsoft_ad

MicrosoftAD resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags to be assigned to the Managed Microsoft AD directory.</p> |
| `edition` | String |  | <p>Managed Microsoft AD is available in two editions: <code>Standard</code> and
        <code>Enterprise</code>. <code>Enterprise</code> is the default.</p> |
| `description` | String |  | <p>A description for the directory. This label will appear on the Amazon Web Services console
        <code>Directory Details</code> page after the directory is created.</p> |
| `name` | String | ✅ | <p>The fully qualified domain name for the Managed Microsoft AD directory, such as
        <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need
      to be publicly resolvable.</p> |
| `short_name` | String |  | <p>The NetBIOS name for your domain, such as <code>CORP</code>. If you don't specify a
      NetBIOS name, it will default to the first part of your directory DNS. For example,
        <code>CORP</code> for the directory DNS <code>corp.example.com</code>. </p> |
| `vpc_settings` | String | ✅ | <p>Contains VPC information for the <a>CreateDirectory</a> or <a>CreateMicrosoftAD</a> operation.</p> |
| `network_type` | String |  | <p>
      The network type for your domain. The default value is <code>IPv4</code> or <code>IPv6</code>
      based on the provided subnet capabilities.</p> |
| `password` | String | ✅ | <p>The password for the default administrative user named <code>Admin</code>.</p>
         <p>If you need to change the password for the administrator account, you can use the <a>ResetUserPassword</a> API call.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create microsoft_ad
microsoft_ad = provider.directory_service.Microsoft_ad {
    name = "value"  # <p>The fully qualified domain name for the Managed Microsoft AD directory, such as
        <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need
      to be publicly resolvable.</p>
    vpc_settings = "value"  # <p>Contains VPC information for the <a>CreateDirectory</a> or <a>CreateMicrosoftAD</a> operation.</p>
    password = "value"  # <p>The password for the default administrative user named <code>Admin</code>.</p>
         <p>If you need to change the password for the administrator account, you can use the <a>ResetUserPassword</a> API call.</p>
}

```

---


### Shared_directories

SharedDirectories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `shared_directories` | Vec<String> | <p>A list of all shared directories in your account.</p> |
| `next_token` | String | <p>If not null, token that indicates that more results are available. Pass this value for the
        <code>NextToken</code> parameter in a subsequent call to <a>DescribeSharedDirectories</a> to retrieve the next set of items.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access shared_directories outputs
shared_directories_id = shared_directories.id
shared_directories_shared_directories = shared_directories.shared_directories
shared_directories_next_token = shared_directories.next_token
```

---


### Domain_controllers

DomainControllers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_controllers` | Vec<String> | <p>List of the <a>DomainController</a> objects that were retrieved.</p> |
| `next_token` | String | <p>If not null, more results are available. Pass this value for the <code>NextToken</code>
      parameter in a subsequent call to <a>DescribeDomainControllers</a> retrieve the
      next set of items.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_controllers outputs
domain_controllers_id = domain_controllers.id
domain_controllers_domain_controllers = domain_controllers.domain_controllers
domain_controllers_next_token = domain_controllers.next_token
```

---


### Conditional_forwarder

ConditionalForwarder resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `remote_domain_name` | String | ✅ | <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up
      a trust relationship.</p> |
| `directory_id` | String | ✅ | <p>The directory ID of the Amazon Web Services directory for which you are creating the conditional
      forwarder.</p> |
| `dns_ip_addrs` | Vec<String> |  | <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p> |
| `dns_ipv6_addrs` | Vec<String> |  | <p>The IPv6 addresses of the remote DNS server associated with RemoteDomainName.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create conditional_forwarder
conditional_forwarder = provider.directory_service.Conditional_forwarder {
    remote_domain_name = "value"  # <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up
      a trust relationship.</p>
    directory_id = "value"  # <p>The directory ID of the Amazon Web Services directory for which you are creating the conditional
      forwarder.</p>
}

```

---


### Directory_setup

DirectorySetup resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_type` | String | ✅ | <p>The type of update to perform on the directory.</p> |
| `network_update_settings` | String |  | <p>Network configuration to apply during the directory update operation.</p> |
| `create_snapshot_before_update` | bool |  | <p>Specifies whether to create a directory snapshot before performing the update.</p> |
| `os_update_settings` | String |  | <p>Operating system configuration to apply during the directory update operation.</p> |
| `directory_id` | String | ✅ | <p>The identifier of the directory to update.</p> |
| `directory_size_update_settings` | String |  | <p>Directory size configuration to apply during the update operation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Update_directory

UpdateDirectory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p> If not null, more results are available. Pass this value for the <code>NextToken</code>
      parameter. </p> |
| `update_activities` | Vec<String> | <p> The list of update activities on a directory for the requested update type. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access update_directory outputs
update_directory_id = update_directory.id
update_directory_next_token = update_directory.next_token
update_directory_update_activities = update_directory.update_activities
```

---


### Computer

Computer resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `computer_attributes` | Vec<String> |  | <p>An array of <a>Attribute</a> objects that contain any LDAP attributes to apply to the
            computer account.</p> |
| `organizational_unit_distinguished_name` | String |  | <p>The fully-qualified distinguished name of the organizational unit to place the computer account in.</p> |
| `password` | String | ✅ | <p>A one-time password that is used to join the computer to the directory. You should generate a random, strong password to use for this parameter.</p> |
| `directory_id` | String | ✅ | <p>The identifier of the directory in which to create the computer account.</p> |
| `computer_name` | String | ✅ | <p>The name of the computer account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create computer
computer = provider.directory_service.Computer {
    password = "value"  # <p>A one-time password that is used to join the computer to the directory. You should generate a random, strong password to use for this parameter.</p>
    directory_id = "value"  # <p>The identifier of the directory in which to create the computer account.</p>
    computer_name = "value"  # <p>The name of the computer account.</p>
}

```

---


### Directories

Directories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `directory_descriptions` | Vec<String> | <p>The list of available <a>DirectoryDescription</a> objects that were
      retrieved.</p>
         <p>It is possible that this list contains less than the number of items specified in the
        <code>Limit</code> member of the request. This occurs if there are less than the requested
      number of items left to retrieve, or if the limitations of the operation have been
      exceeded.</p> |
| `next_token` | String | <p>If not null, more results are available. Pass this value for the <code>NextToken</code>
      parameter in a subsequent call to <a>DescribeDirectories</a> to retrieve the next
      set of items.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access directories outputs
directories_id = directories.id
directories_directory_descriptions = directories.directory_descriptions
directories_next_token = directories.next_token
```

---


### Alias

Alias resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alias` | String | ✅ | <p>The requested alias.</p>
         <p>The alias must be unique amongst all aliases in Amazon Web Services. This operation throws an
            <code>EntityAlreadyExistsException</code> error if the alias already exists.</p> |
| `directory_id` | String | ✅ | <p>The identifier of the directory for which to create the alias.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create alias
alias = provider.directory_service.Alias {
    alias = "value"  # <p>The requested alias.</p>
         <p>The alias must be unique amongst all aliases in Amazon Web Services. This operation throws an
            <code>EntityAlreadyExistsException</code> error if the alias already exists.</p>
    directory_id = "value"  # <p>The identifier of the directory for which to create the alias.</p>
}

```

---


### Ldaps_settings

LDAPSSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ldaps_settings_info` | Vec<String> | <p>Information about LDAP security for the specified directory, including status of
      enablement, state last updated date time, and the reason for the state.</p> |
| `next_token` | String | <p>The next token used to retrieve the LDAPS settings if the number of setting types exceeds
      page limit and there is another page.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ldaps_settings outputs
ldaps_settings_id = ldaps_settings.id
ldaps_settings_ldaps_settings_info = ldaps_settings.ldaps_settings_info
ldaps_settings_next_token = ldaps_settings.next_token
```

---


### Hybrid_ad

HybridAD resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secret_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the
            credentials for the service account used to join hybrid domain controllers to your
            self-managed AD domain. This secret is used once and not stored.</p>
         <p>The secret must contain key-value pairs with keys matching
                <code>customerAdAdminDomainUsername</code> and
                <code>customerAdAdminDomainPassword</code>. For example:
                <code>{"customerAdAdminDomainUsername":"carlos_salazar","customerAdAdminDomainPassword":"ExamplePassword123!"}</code>.</p> |
| `assessment_id` | String | ✅ | <p>The unique identifier of the successful directory assessment that validates your
            self-managed AD environment. You must have a successful directory assessment before you
            create a hybrid directory.</p> |
| `tags` | Vec<String> |  | <p>The tags to be assigned to the directory. Each tag consists of a key and value pair.
            You can specify multiple tags as a list.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hybrid_ad
hybrid_ad = provider.directory_service.Hybrid_ad {
    secret_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the
            credentials for the service account used to join hybrid domain controllers to your
            self-managed AD domain. This secret is used once and not stored.</p>
         <p>The secret must contain key-value pairs with keys matching
                <code>customerAdAdminDomainUsername</code> and
                <code>customerAdAdminDomainPassword</code>. For example:
                <code>{"customerAdAdminDomainUsername":"carlos_salazar","customerAdAdminDomainPassword":"ExamplePassword123!"}</code>.</p>
    assessment_id = "value"  # <p>The unique identifier of the successful directory assessment that validates your
            self-managed AD environment. You must have a successful directory assessment before you
            create a hybrid directory.</p>
}

```

---


### Radius

Radius resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `radius_settings` | String | ✅ | <p>A <a>RadiusSettings</a> object that contains information about the RADIUS
         server.</p> |
| `directory_id` | String | ✅ | <p>The identifier of the directory for which to update the RADIUS server
         information.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Snapshot

Snapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The descriptive name to apply to the snapshot.</p> |
| `directory_id` | String | ✅ | <p>The identifier of the directory of which to take a snapshot.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot
snapshot = provider.directory_service.Snapshot {
    directory_id = "value"  # <p>The identifier of the directory of which to take a snapshot.</p>
}

```

---


### Snapshot_limits

SnapshotLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshot_limits` | String | <p>A <a>SnapshotLimits</a> object that contains the manual snapshot limits for the specified
         directory.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshot_limits outputs
snapshot_limits_id = snapshot_limits.id
snapshot_limits_snapshot_limits = snapshot_limits.snapshot_limits
```

---


### Snapshots

Snapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshots` | Vec<String> | <p>The list of <a>Snapshot</a> objects that were retrieved.</p>
         <p>It is possible that this list contains less than the number of items specified in the
            <i>Limit</i> member of the request. This occurs if there are less than the requested
         number of items left to retrieve, or if the limitations of the operation have been
         exceeded.</p> |
| `next_token` | String | <p>If not null, more results are available. Pass this value in the <i>NextToken</i> member of
         a subsequent call to <a>DescribeSnapshots</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshots outputs
snapshots_id = snapshots.id
snapshots_snapshots = snapshots.snapshots
snapshots_next_token = snapshots.next_token
```

---


### Trust

Trust resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `remote_domain_name` | String | ✅ | <p>The Fully Qualified Domain Name (FQDN) of the external domain for which to create the
      trust relationship.</p> |
| `trust_type` | String |  | <p>The trust relationship type. <code>Forest</code> is the default.</p> |
| `trust_direction` | String | ✅ | <p>The direction of the trust relationship.</p> |
| `conditional_forwarder_ip_addrs` | Vec<String> |  | <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p> |
| `conditional_forwarder_ipv6_addrs` | Vec<String> |  | <p>The IPv6 addresses of the remote DNS server associated with RemoteDomainName.</p> |
| `selective_auth` | String |  | <p>Optional parameter to enable selective authentication for the trust.</p> |
| `directory_id` | String | ✅ | <p>The Directory ID of the Managed Microsoft AD directory for which to establish the trust
      relationship.</p> |
| `trust_password` | String | ✅ | <p>The trust password. The trust password must be the same password that was used when creating the trust
      relationship on the external domain.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trust
trust = provider.directory_service.Trust {
    remote_domain_name = "value"  # <p>The Fully Qualified Domain Name (FQDN) of the external domain for which to create the
      trust relationship.</p>
    trust_direction = "value"  # <p>The direction of the trust relationship.</p>
    directory_id = "value"  # <p>The Directory ID of the Managed Microsoft AD directory for which to establish the trust
      relationship.</p>
    trust_password = "value"  # <p>The trust password. The trust password must be the same password that was used when creating the trust
      relationship on the external domain.</p>
}

```

---


### Ad_assessment

ADAssessment resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assessment` | String | <p>Detailed information about the self-managed instance settings (IDs and DNS
            IPs).</p> |
| `assessment_reports` | Vec<String> | <p>A list of assessment reports containing validation results for each domain controller
            and test category. Each report includes specific validation details and outcomes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ad_assessment outputs
ad_assessment_id = ad_assessment.id
ad_assessment_assessment = ad_assessment.assessment
ad_assessment_assessment_reports = ad_assessment.assessment_reports
```

---


### Settings

Settings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `settings` | Vec<String> | ✅ | <p>The list of <a>Setting</a> objects.</p> |
| `directory_id` | String | ✅ | <p>The identifier of the directory for which to update settings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `directory_id` | String | <p>The identifier of the directory.</p> |
| `setting_entries` | Vec<String> | <p>The list of <a>SettingEntry</a> objects that were retrieved.</p>
         <p>It is possible that this list contains less than the number of items specified in the
        <code>Limit</code> member of the request. This occurs if there are less than the requested
      number of items left to retrieve, or if the limitations of the operation have been
      exceeded.</p> |
| `next_token` | String | <p>If not null, token that indicates that more results are available. Pass this value for the
        <code>NextToken</code> parameter in a subsequent call to <code>DescribeSettings</code> to
      retrieve the next set of items. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access settings outputs
settings_id = settings.id
settings_directory_id = settings.directory_id
settings_setting_entries = settings.setting_entries
settings_next_token = settings.next_token
```

---


### Log_subscription

LogSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `directory_id` | String | ✅ | <p>Identifier of the directory to which you want to subscribe and receive real-time logs to
      your specified CloudWatch log group.</p> |
| `log_group_name` | String | ✅ | <p>The name of the CloudWatch log group where the real-time domain controller logs are
      forwarded.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create log_subscription
log_subscription = provider.directory_service.Log_subscription {
    directory_id = "value"  # <p>Identifier of the directory to which you want to subscribe and receive real-time logs to
      your specified CloudWatch log group.</p>
    log_group_name = "value"  # <p>The name of the CloudWatch log group where the real-time domain controller logs are
      forwarded.</p>
}

```

---


### Trusts

Trusts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If not null, more results are available. Pass this value for the
        <i>NextToken</i> parameter in a subsequent call to <a>DescribeTrusts</a> to retrieve the next set of items.</p> |
| `trusts` | Vec<String> | <p>The list of Trust objects that were retrieved.</p>
         <p>It is possible that this list contains less than the number of items specified in the
        <i>Limit</i> member of the request. This occurs if there are less than the
      requested number of items left to retrieve, or if the limitations of the operation have been
      exceeded.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trusts outputs
trusts_id = trusts.id
trusts_next_token = trusts.next_token
trusts_trusts = trusts.trusts
```

---


### Directory_limits

DirectoryLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `directory_limits` | String | <p>A <a>DirectoryLimits</a> object that contains the directory limits for the
      current Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access directory_limits outputs
directory_limits_id = directory_limits.id
directory_limits_directory_limits = directory_limits.directory_limits
```

---


### Hybrid_ad_update

HybridADUpdate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If not null, more results are available. Pass this value for the
                <code>NextToken</code> parameter in a subsequent request to retrieve the next set of
            items.</p> |
| `update_activities` | String | <p>Information about update activities for the hybrid directory, organized by update
            type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hybrid_ad_update outputs
hybrid_ad_update_id = hybrid_ad_update.id
hybrid_ad_update_next_token = hybrid_ad_update.next_token
hybrid_ad_update_update_activities = hybrid_ad_update.update_activities
```

---


### Conditional_forwarders

ConditionalForwarders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conditional_forwarders` | Vec<String> | <p>The list of conditional forwarders that have been created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conditional_forwarders outputs
conditional_forwarders_id = conditional_forwarders.id
conditional_forwarders_conditional_forwarders = conditional_forwarders.conditional_forwarders
```

---


### Ca_enrollment_policy

CAEnrollmentPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_date_time` | String | <p>The date and time when the CA enrollment policy was last modified or updated.</p> |
| `directory_id` | String | <p>The identifier of the directory associated with this CA enrollment policy.</p> |
| `pca_connector_arn` | String | <p>The Amazon Resource Name (ARN) of the Amazon Web Services Private Certificate Authority (PCA) connector
      that is configured for automatic certificate enrollment in this directory.</p> |
| `ca_enrollment_policy_status` | String | <p>The current status of the CA enrollment policy. This indicates if automatic certificate
      enrollment is currently active, inactive, or in a transitional state.</p>
         <p>Valid values:</p>
         <ul>
            <li>
               <p>
                  <code>IN_PROGRESS</code> - The policy is being activated T</p>
            </li>
            <li>
               <p>
                  <code>SUCCESS</code> - The policy is active and automatic certificate enrollment is
          operational</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> - The policy activation or deactivation failed</p>
            </li>
            <li>
               <p>
                  <code>DISABLING</code> - The policy is being deactivated</p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> - The policy is inactive and automatic certificate enrollment is
          not available</p>
            </li>
            <li>
               <p>
                  <code>IMPAIRED</code> - Network connectivity is impaired.</p>
            </li>
         </ul> |
| `ca_enrollment_policy_status_reason` | String | <p>Additional information explaining the current status of the CA enrollment policy,
      particularly useful when the policy is in an error or transitional state.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ca_enrollment_policy outputs
ca_enrollment_policy_id = ca_enrollment_policy.id
ca_enrollment_policy_last_updated_date_time = ca_enrollment_policy.last_updated_date_time
ca_enrollment_policy_directory_id = ca_enrollment_policy.directory_id
ca_enrollment_policy_pca_connector_arn = ca_enrollment_policy.pca_connector_arn
ca_enrollment_policy_ca_enrollment_policy_status = ca_enrollment_policy.ca_enrollment_policy_status
ca_enrollment_policy_ca_enrollment_policy_status_reason = ca_enrollment_policy.ca_enrollment_policy_status_reason
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple directory resources
directory_0 = provider.directory_service.Directory {
    name = "value-0"
    password = "value-0"
    size = "value-0"
}
directory_1 = provider.directory_service.Directory {
    name = "value-1"
    password = "value-1"
    size = "value-1"
}
directory_2 = provider.directory_service.Directory {
    name = "value-2"
    password = "value-2"
    size = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    directory = provider.directory_service.Directory {
        name = "production-value"
        password = "production-value"
        size = "production-value"
    }
```

---

## Related Documentation

- [AWS Directory_service Documentation](https://docs.aws.amazon.com/directory_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
