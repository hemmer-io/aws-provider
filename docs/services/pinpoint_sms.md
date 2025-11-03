# Pinpoint_sms Service



**Resources**: 47

---

## Overview

The pinpoint_sms service provides access to 47 resource types:

- [Configuration_set_event_destination](#configuration_set_event_destination) [CUD]
- [Configuration_set_event_destinations](#configuration_set_event_destinations) [R]
- [Configuration_set](#configuration_set) [CD]
- [Registration_version](#registration_version) [C]
- [Registrations](#registrations) [R]
- [Registration_attachment](#registration_attachment) [CD]
- [Verified_destination_number](#verified_destination_number) [CD]
- [Registration_association](#registration_association) [C]
- [Opted_out_number](#opted_out_number) [CD]
- [Phone_numbers](#phone_numbers) [R]
- [Account_default_protect_configuration](#account_default_protect_configuration) [D]
- [Keywords](#keywords) [R]
- [Pool](#pool) [CUD]
- [Protect_configuration](#protect_configuration) [CUD]
- [Account_attributes](#account_attributes) [R]
- [Registration_field_definitions](#registration_field_definitions) [R]
- [Registration_type_definitions](#registration_type_definitions) [R]
- [Media_message_spend_limit_override](#media_message_spend_limit_override) [D]
- [Sender_id](#sender_id) [U]
- [Registration_versions](#registration_versions) [R]
- [Message_feedback](#message_feedback) [C]
- [Registration_section_definitions](#registration_section_definitions) [R]
- [Text_message_spend_limit_override](#text_message_spend_limit_override) [D]
- [Account_limits](#account_limits) [R]
- [Protect_configurations](#protect_configurations) [R]
- [Registration_field_value](#registration_field_value) [CD]
- [Default_message_type](#default_message_type) [D]
- [Registration_field_values](#registration_field_values) [R]
- [Opt_out_list](#opt_out_list) [CD]
- [Sender_ids](#sender_ids) [R]
- [Protect_configuration_country_rule_set](#protect_configuration_country_rule_set) [RU]
- [Resource_policy](#resource_policy) [CRD]
- [Opted_out_numbers](#opted_out_numbers) [R]
- [Registration_attachments](#registration_attachments) [R]
- [Registration](#registration) [CD]
- [Event_destination](#event_destination) [CUD]
- [Protect_configuration_rule_set_number_override](#protect_configuration_rule_set_number_override) [CD]
- [Opt_out_lists](#opt_out_lists) [R]
- [Default_sender_id](#default_sender_id) [D]
- [Verified_destination_numbers](#verified_destination_numbers) [R]
- [Pools](#pools) [R]
- [Configuration_set](#configuration_set) [CD]
- [Voice_message_spend_limit_override](#voice_message_spend_limit_override) [D]
- [Configuration_sets](#configuration_sets) [R]
- [Phone_number](#phone_number) [U]
- [Keyword](#keyword) [CD]
- [Spend_limits](#spend_limits) [R]

---

## Resources


### Configuration_set_event_destination

ConfigurationSetEventDestination resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | ConfigurationSetName |
| `event_destination_name` | String |  | A name that identifies the event destination. |
| `event_destination` | String |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_event_destination
configuration_set_event_destination = provider.pinpoint_sms.Configuration_set_event_destination {
    configuration_set_name = "value"  # ConfigurationSetName
}

```

---


### Configuration_set_event_destinations

ConfigurationSetEventDestinations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_destinations` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_set_event_destinations outputs
configuration_set_event_destinations_id = configuration_set_event_destinations.id
configuration_set_event_destinations_event_destinations = configuration_set_event_destinations.event_destinations
```

---


### Configuration_set

ConfigurationSet resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String |  | The name that you want to give the configuration set. |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set
configuration_set = provider.pinpoint_sms.Configuration_set {
}

```

---


### Registration_version

RegistrationVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registration_id` | String | ✅ | <p>The unique identifier for the registration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registration_version
registration_version = provider.pinpoint_sms.Registration_version {
    registration_id = "value"  # <p>The unique identifier for the registration.</p>
}

```

---


### Registrations

Registrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |
| `registrations` | Vec<String> | <p>An array of RegistrationInformation objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registrations outputs
registrations_id = registrations.id
registrations_next_token = registrations.next_token
registrations_registrations = registrations.registrations
```

---


### Registration_attachment

RegistrationAttachment resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attachment_url` | String |  | <p>Registration files have to be stored in an Amazon S3 bucket. The URI to use when sending is in the format <code>s3://BucketName/FileName</code>.</p> |
| `tags` | Vec<String> |  | <p>An array of tags (key and value pairs) to associate with the registration attachment.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |
| `attachment_body` | String |  | <p>The registration file to upload. The maximum file size is 500KB and valid file extensions are PDF, JPEG and PNG.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registration_attachment
registration_attachment = provider.pinpoint_sms.Registration_attachment {
}

```

---


### Verified_destination_number

VerifiedDestinationNumber resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |
| `destination_phone_number` | String | ✅ | <p>The verified destination phone number, in E.164 format.</p> |
| `tags` | Vec<String> |  | <p>An array of tags (key and value pairs) to associate with the destination number.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create verified_destination_number
verified_destination_number = provider.pinpoint_sms.Verified_destination_number {
    destination_phone_number = "value"  # <p>The verified destination phone number, in E.164 format.</p>
}

```

---


### Registration_association

RegistrationAssociation resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_id` | String | ✅ | <p>The unique identifier for the origination identity. For example this could be a <b>PhoneNumberId</b> or <b>SenderId</b>.</p> |
| `registration_id` | String | ✅ | <p>The unique identifier for the registration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registration_association
registration_association = provider.pinpoint_sms.Registration_association {
    resource_id = "value"  # <p>The unique identifier for the origination identity. For example this could be a <b>PhoneNumberId</b> or <b>SenderId</b>.</p>
    registration_id = "value"  # <p>The unique identifier for the registration.</p>
}

```

---


### Opted_out_number

OptedOutNumber resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `opted_out_number` | String | ✅ | <p>The phone number to add to the OptOutList in E.164 format.</p> |
| `opt_out_list_name` | String | ✅ | <p>The OptOutListName or OptOutListArn to add the phone number to.</p> <important> <p>If you are using a shared AWS End User Messaging SMS and Voice resource then you must use the full Amazon Resource Name(ARN).</p> </important> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create opted_out_number
opted_out_number = provider.pinpoint_sms.Opted_out_number {
    opted_out_number = "value"  # <p>The phone number to add to the OptOutList in E.164 format.</p>
    opt_out_list_name = "value"  # <p>The OptOutListName or OptOutListArn to add the phone number to.</p> <important> <p>If you are using a shared AWS End User Messaging SMS and Voice resource then you must use the full Amazon Resource Name(ARN).</p> </important>
}

```

---


### Phone_numbers

PhoneNumbers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `phone_numbers` | Vec<String> | <p>An array of PhoneNumberInformation objects that contain the details for the requested phone numbers.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access phone_numbers outputs
phone_numbers_id = phone_numbers.id
phone_numbers_next_token = phone_numbers.next_token
phone_numbers_phone_numbers = phone_numbers.phone_numbers
```

---


### Account_default_protect_configuration

AccountDefaultProtectConfiguration resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Keywords

Keywords resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `origination_identity_arn` | String | <p>The PhoneNumberArn or PoolArn that is associated with the OriginationIdentity. </p> |
| `keywords` | Vec<String> | <p>An array of KeywordInformation objects that contain the results.</p> |
| `origination_identity` | String | <p>The PhoneNumberId or PoolId that is associated with the OriginationIdentity.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access keywords outputs
keywords_id = keywords.id
keywords_next_token = keywords.next_token
keywords_origination_identity_arn = keywords.origination_identity_arn
keywords_keywords = keywords.keywords
keywords_origination_identity = keywords.origination_identity
```

---


### Pool

Pool resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message_type` | String | ✅ | <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive. After the pool is created the MessageType can't be changed.</p> |
| `deletion_protection_enabled` | bool |  | <p>By default this is set to false. When set to true the pool can't be deleted. You can change this value using the <a>UpdatePool</a> action.</p> |
| `iso_country_code` | String | ✅ | <p>The new two-character code, in ISO 3166-1 alpha-2 format, for the country or region of the new pool.</p> |
| `tags` | Vec<String> |  | <p>An array of tags (key and value pairs) associated with the pool.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |
| `origination_identity` | String | ✅ | <p>The origination identity to use such as a PhoneNumberId, PhoneNumberArn, SenderId or SenderIdArn. You can use <a>DescribePhoneNumbers</a> to find the values for PhoneNumberId and PhoneNumberArn while <a>DescribeSenderIds</a> can be used to get the values for SenderId and SenderIdArn.</p> <p>After the pool is created you can add more origination identities to the pool by using <a href="https://docs.aws.amazon.com/pinpoint/latest/apireference_smsvoicev2/API_AssociateOriginationIdentity.html">AssociateOriginationIdentity</a>.</p> <important> <p>If you are using a shared AWS End User Messaging SMS and Voice resource then you must use the full Amazon Resource Name(ARN).</p> </important> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pool
pool = provider.pinpoint_sms.Pool {
    message_type = "value"  # <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive. After the pool is created the MessageType can't be changed.</p>
    iso_country_code = "value"  # <p>The new two-character code, in ISO 3166-1 alpha-2 format, for the country or region of the new pool.</p>
    origination_identity = "value"  # <p>The origination identity to use such as a PhoneNumberId, PhoneNumberArn, SenderId or SenderIdArn. You can use <a>DescribePhoneNumbers</a> to find the values for PhoneNumberId and PhoneNumberArn while <a>DescribeSenderIds</a> can be used to get the values for SenderId and SenderIdArn.</p> <p>After the pool is created you can add more origination identities to the pool by using <a href="https://docs.aws.amazon.com/pinpoint/latest/apireference_smsvoicev2/API_AssociateOriginationIdentity.html">AssociateOriginationIdentity</a>.</p> <important> <p>If you are using a shared AWS End User Messaging SMS and Voice resource then you must use the full Amazon Resource Name(ARN).</p> </important>
}

```

---


### Protect_configuration

ProtectConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of key and value pair tags that are associated with the resource.</p> |
| `deletion_protection_enabled` | bool |  | <p>When set to true deletion protection is enabled. By default this is set to false. </p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create protect_configuration
protect_configuration = provider.pinpoint_sms.Protect_configuration {
}

```

---


### Account_attributes

AccountAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `account_attributes` | Vec<String> | <p>An array of AccountAttributes objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_attributes outputs
account_attributes_id = account_attributes.id
account_attributes_next_token = account_attributes.next_token
account_attributes_account_attributes = account_attributes.account_attributes
```

---


### Registration_field_definitions

RegistrationFieldDefinitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |
| `registration_type` | String | <p>The type of registration form. The list of <b>RegistrationTypes</b> can be found using the <a>DescribeRegistrationTypeDefinitions</a> action.</p> |
| `registration_field_definitions` | Vec<String> | <p>An array of RegistrationFieldDefinitions objects that contain the details for the requested fields. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registration_field_definitions outputs
registration_field_definitions_id = registration_field_definitions.id
registration_field_definitions_next_token = registration_field_definitions.next_token
registration_field_definitions_registration_type = registration_field_definitions.registration_type
registration_field_definitions_registration_field_definitions = registration_field_definitions.registration_field_definitions
```

---


### Registration_type_definitions

RegistrationTypeDefinitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registration_type_definitions` | Vec<String> | <p>The type of registration form. The list of <b>RegistrationTypes</b> can be found using the <a>DescribeRegistrationTypeDefinitions</a> action.</p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registration_type_definitions outputs
registration_type_definitions_id = registration_type_definitions.id
registration_type_definitions_registration_type_definitions = registration_type_definitions.registration_type_definitions
registration_type_definitions_next_token = registration_type_definitions.next_token
```

---


### Media_message_spend_limit_override

MediaMessageSpendLimitOverride resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Sender_id

SenderId resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `iso_country_code` | String | ✅ | <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p> |
| `deletion_protection_enabled` | bool |  | <p>By default this is set to false. When set to true the sender ID can't be deleted.</p> |
| `sender_id` | String | ✅ | <p>The sender ID to update.</p> |



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


### Registration_versions

RegistrationVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registration_versions` | Vec<String> | <p>An array of RegistrationVersions objects.</p> |
| `registration_arn` | String | <p>The Amazon Resource Name (ARN) for the registration.</p> |
| `registration_id` | String | <p>The unique identifier for the registration.</p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registration_versions outputs
registration_versions_id = registration_versions.id
registration_versions_registration_versions = registration_versions.registration_versions
registration_versions_registration_arn = registration_versions.registration_arn
registration_versions_registration_id = registration_versions.registration_id
registration_versions_next_token = registration_versions.next_token
```

---


### Message_feedback

MessageFeedback resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message_id` | String | ✅ | <p>The unique identifier for the message.</p> |
| `message_feedback_status` | String | ✅ | <p>Set the message feedback to be either <code>RECEIVED</code> or <code>FAILED</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create message_feedback
message_feedback = provider.pinpoint_sms.Message_feedback {
    message_id = "value"  # <p>The unique identifier for the message.</p>
    message_feedback_status = "value"  # <p>Set the message feedback to be either <code>RECEIVED</code> or <code>FAILED</code>.</p>
}

```

---


### Registration_section_definitions

RegistrationSectionDefinitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registration_type` | String | <p>The type of registration form. The list of <b>RegistrationTypes</b> can be found using the <a>DescribeRegistrationTypeDefinitions</a> action.</p> |
| `registration_section_definitions` | Vec<String> | <p>An array of RegistrationSectionDefinition objects.</p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registration_section_definitions outputs
registration_section_definitions_id = registration_section_definitions.id
registration_section_definitions_registration_type = registration_section_definitions.registration_type
registration_section_definitions_registration_section_definitions = registration_section_definitions.registration_section_definitions
registration_section_definitions_next_token = registration_section_definitions.next_token
```

---


### Text_message_spend_limit_override

TextMessageSpendLimitOverride resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Account_limits

AccountLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `account_limits` | Vec<String> | <p>An array of AccountLimit objects that show the current spend limits.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_limits outputs
account_limits_id = account_limits.id
account_limits_next_token = account_limits.next_token
account_limits_account_limits = account_limits.account_limits
```

---


### Protect_configurations

ProtectConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protect_configurations` | Vec<String> | <p>An array of ProtectConfigurationInformation objects that contain the details for the request. </p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access protect_configurations outputs
protect_configurations_id = protect_configurations.id
protect_configurations_protect_configurations = protect_configurations.protect_configurations
protect_configurations_next_token = protect_configurations.next_token
```

---


### Registration_field_value

RegistrationFieldValue resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registration_attachment_id` | String |  | <p>The unique identifier for the registration attachment.</p> |
| `registration_id` | String | ✅ | <p>The unique identifier for the registration.</p> |
| `field_path` | String | ✅ | <p>The path to the registration form field. You can use <a>DescribeRegistrationFieldDefinitions</a> for a list of <b>FieldPaths</b>.</p> |
| `text_value` | String |  | <p>The text data for a free form field.</p> |
| `select_choices` | Vec<String> |  | <p>An array of values for the form field.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registration_field_value
registration_field_value = provider.pinpoint_sms.Registration_field_value {
    registration_id = "value"  # <p>The unique identifier for the registration.</p>
    field_path = "value"  # <p>The path to the registration form field. You can use <a>DescribeRegistrationFieldDefinitions</a> for a list of <b>FieldPaths</b>.</p>
}

```

---


### Default_message_type

DefaultMessageType resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Registration_field_values

RegistrationFieldValues resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version_number` | i64 | <p>The current version of the registration.</p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |
| `registration_arn` | String | <p>The Amazon Resource Name (ARN) for the registration.</p> |
| `registration_field_values` | Vec<String> | <p>An array of RegistrationFieldValues objects that contain the values for the requested registration. </p> |
| `registration_id` | String | <p>The unique identifier for the registration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registration_field_values outputs
registration_field_values_id = registration_field_values.id
registration_field_values_version_number = registration_field_values.version_number
registration_field_values_next_token = registration_field_values.next_token
registration_field_values_registration_arn = registration_field_values.registration_arn
registration_field_values_registration_field_values = registration_field_values.registration_field_values
registration_field_values_registration_id = registration_field_values.registration_id
```

---


### Opt_out_list

OptOutList resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `opt_out_list_name` | String | ✅ | <p>The name of the new OptOutList.</p> |
| `tags` | Vec<String> |  | <p>An array of tags (key and value pairs) to associate with the new OptOutList.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create opt_out_list
opt_out_list = provider.pinpoint_sms.Opt_out_list {
    opt_out_list_name = "value"  # <p>The name of the new OptOutList.</p>
}

```

---


### Sender_ids

SenderIds resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `sender_ids` | Vec<String> | <p>An array of SernderIdInformation objects that contain the details for the requested SenderIds.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sender_ids outputs
sender_ids_id = sender_ids.id
sender_ids_next_token = sender_ids.next_token
sender_ids_sender_ids = sender_ids.sender_ids
```

---


### Protect_configuration_country_rule_set

ProtectConfigurationCountryRuleSet resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `country_rule_set_updates` | HashMap<String, String> | ✅ | <p>A map of ProtectConfigurationCountryRuleSetInformation objects that contain the details for the requested NumberCapability. The Key is the two-letter ISO country code. For a list of supported ISO country codes, see <a href="https://docs.aws.amazon.com/sms-voice/latest/userguide/phone-numbers-sms-by-country.html">Supported countries and regions (SMS channel)</a> in the AWS End User Messaging SMS User Guide.</p> <p>For example, to set the United States as allowed and Canada as blocked, the <code>CountryRuleSetUpdates</code> would be formatted as: <code>"CountryRuleSetUpdates": { "US" : { "ProtectStatus": "ALLOW" } "CA" : { "ProtectStatus": "BLOCK" } }</code> </p> |
| `protect_configuration_id` | String | ✅ | <p>The unique identifier for the protect configuration.</p> |
| `number_capability` | String | ✅ | <p>The number capability to apply the CountryRuleSetUpdates updates to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `number_capability` | String | <p>The capability type associated with the returned ProtectConfigurationCountryRuleSetInformation objects.</p> |
| `protect_configuration_arn` | String | <p>The Amazon Resource Name (ARN) of the protect configuration.</p> |
| `protect_configuration_id` | String | <p>The unique identifier for the protect configuration.</p> |
| `country_rule_set` | HashMap<String, String> | <p>A map of ProtectConfigurationCountryRuleSetInformation objects that contain the details for the requested NumberCapability. The Key is the two-letter ISO country code. For a list of supported ISO country codes, see <a href="https://docs.aws.amazon.com/sms-voice/latest/userguide/phone-numbers-sms-by-country.html">Supported countries and regions (SMS channel)</a> in the AWS End User Messaging SMS User Guide.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access protect_configuration_country_rule_set outputs
protect_configuration_country_rule_set_id = protect_configuration_country_rule_set.id
protect_configuration_country_rule_set_number_capability = protect_configuration_country_rule_set.number_capability
protect_configuration_country_rule_set_protect_configuration_arn = protect_configuration_country_rule_set.protect_configuration_arn
protect_configuration_country_rule_set_protect_configuration_id = protect_configuration_country_rule_set.protect_configuration_id
protect_configuration_country_rule_set_country_rule_set = protect_configuration_country_rule_set.country_rule_set
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The JSON formatted resource-based policy to attach.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the AWS End User Messaging SMS and Voice resource to attach the resource-based policy to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the AWS End User Messaging SMS and Voice resource attached to the resource-based policy.</p> |
| `policy` | String | <p>The JSON formatted string that contains the resource-based policy attached to the AWS End User Messaging SMS and Voice resource. </p> |
| `created_timestamp` | String | <p>The time when the resource-based policy was created, in <a href="https://www.epochconverter.com/">UNIX epoch time</a> format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.pinpoint_sms.Resource_policy {
    policy = "value"  # <p>The JSON formatted resource-based policy to attach.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the AWS End User Messaging SMS and Voice resource to attach the resource-based policy to.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_resource_arn = resource_policy.resource_arn
resource_policy_policy = resource_policy.policy
resource_policy_created_timestamp = resource_policy.created_timestamp
```

---


### Opted_out_numbers

OptedOutNumbers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `opted_out_numbers` | Vec<String> | <p>An array of OptedOutNumbersInformation objects that provide information about the requested OptedOutNumbers.</p> |
| `opt_out_list_arn` | String | <p>The Amazon Resource Name (ARN) of the OptOutList.</p> |
| `opt_out_list_name` | String | <p>The name of the OptOutList.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access opted_out_numbers outputs
opted_out_numbers_id = opted_out_numbers.id
opted_out_numbers_next_token = opted_out_numbers.next_token
opted_out_numbers_opted_out_numbers = opted_out_numbers.opted_out_numbers
opted_out_numbers_opt_out_list_arn = opted_out_numbers.opt_out_list_arn
opted_out_numbers_opt_out_list_name = opted_out_numbers.opt_out_list_name
```

---


### Registration_attachments

RegistrationAttachments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registration_attachments` | Vec<String> | <p>An array of <b>RegistrationAttachments</b> objects that contain the details for the requested registration attachments. </p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registration_attachments outputs
registration_attachments_id = registration_attachments.id
registration_attachments_registration_attachments = registration_attachments.registration_attachments
registration_attachments_next_token = registration_attachments.next_token
```

---


### Registration

Registration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |
| `registration_type` | String | ✅ | <p>The type of registration form to create. The list of <b>RegistrationTypes</b> can be found using the <a>DescribeRegistrationTypeDefinitions</a> action.</p> |
| `tags` | Vec<String> |  | <p>An array of tags (key and value pairs) to associate with the registration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registration
registration = provider.pinpoint_sms.Registration {
    registration_type = "value"  # <p>The type of registration form to create. The list of <b>RegistrationTypes</b> can be found using the <a>DescribeRegistrationTypeDefinitions</a> action.</p>
}

```

---


### Event_destination

EventDestination resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_destination_name` | String | ✅ | <p>The name that identifies the event destination.</p> |
| `configuration_set_name` | String | ✅ | <p>Either the name of the configuration set or the configuration set ARN to apply event logging to. The ConfigurateSetName and ConfigurationSetArn can be found using the <a>DescribeConfigurationSets</a> action.</p> |
| `matching_event_types` | Vec<String> | ✅ | <p>An array of event types that determine which events to log. If "ALL" is used, then AWS End User Messaging SMS and Voice logs every event type.</p> <note> <p>The <code>TEXT_SENT</code> event type is not supported.</p> </note> |
| `kinesis_firehose_destination` | String |  | <p>An object that contains information about an event destination for logging to Amazon Data Firehose.</p> |
| `cloud_watch_logs_destination` | String |  | <p>An object that contains information about an event destination for logging to Amazon CloudWatch Logs.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |
| `sns_destination` | String |  | <p>An object that contains information about an event destination for logging to Amazon SNS.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_destination
event_destination = provider.pinpoint_sms.Event_destination {
    event_destination_name = "value"  # <p>The name that identifies the event destination.</p>
    configuration_set_name = "value"  # <p>Either the name of the configuration set or the configuration set ARN to apply event logging to. The ConfigurateSetName and ConfigurationSetArn can be found using the <a>DescribeConfigurationSets</a> action.</p>
    matching_event_types = "value"  # <p>An array of event types that determine which events to log. If "ALL" is used, then AWS End User Messaging SMS and Voice logs every event type.</p> <note> <p>The <code>TEXT_SENT</code> event type is not supported.</p> </note>
}

```

---


### Protect_configuration_rule_set_number_override

ProtectConfigurationRuleSetNumberOverride resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_phone_number` | String | ✅ | <p>The destination phone number in E.164 format.</p> |
| `expiration_timestamp` | String |  | <p>The time the rule will expire at. If <code>ExpirationTimestamp</code> is not set then the rule does not expire.</p> |
| `protect_configuration_id` | String | ✅ | <p>The unique identifier for the protect configuration.</p> |
| `action` | String | ✅ | <p>The action for the rule to either block or allow messages to the destination phone number.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create protect_configuration_rule_set_number_override
protect_configuration_rule_set_number_override = provider.pinpoint_sms.Protect_configuration_rule_set_number_override {
    destination_phone_number = "value"  # <p>The destination phone number in E.164 format.</p>
    protect_configuration_id = "value"  # <p>The unique identifier for the protect configuration.</p>
    action = "value"  # <p>The action for the rule to either block or allow messages to the destination phone number.</p>
}

```

---


### Opt_out_lists

OptOutLists resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `opt_out_lists` | Vec<String> | <p>An array of OptOutListInformation objects that contain the details for the requested OptOutLists.</p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access opt_out_lists outputs
opt_out_lists_id = opt_out_lists.id
opt_out_lists_opt_out_lists = opt_out_lists.opt_out_lists
opt_out_lists_next_token = opt_out_lists.next_token
```

---


### Default_sender_id

DefaultSenderId resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Verified_destination_numbers

VerifiedDestinationNumbers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `verified_destination_numbers` | Vec<String> | <p>An array of VerifiedDestinationNumberInformation objects</p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_destination_numbers outputs
verified_destination_numbers_id = verified_destination_numbers.id
verified_destination_numbers_verified_destination_numbers = verified_destination_numbers.verified_destination_numbers
verified_destination_numbers_next_token = verified_destination_numbers.next_token
```

---


### Pools

Pools resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `pools` | Vec<String> | <p>An array of PoolInformation objects that contain the details for the requested pools. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pools outputs
pools_id = pools.id
pools_next_token = pools.next_token
pools_pools = pools.pools
```

---


### Configuration_set

ConfigurationSet resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of key and value pair tags that's associated with the new configuration set. </p> |
| `configuration_set_name` | String | ✅ | <p>The name to use for the new configuration set.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set
configuration_set = provider.pinpoint_sms.Configuration_set {
    configuration_set_name = "value"  # <p>The name to use for the new configuration set.</p>
}

```

---


### Voice_message_spend_limit_override

VoiceMessageSpendLimitOverride resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Configuration_sets

ConfigurationSets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |
| `configuration_sets` | Vec<String> | <p>An array of ConfigurationSets objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_sets outputs
configuration_sets_id = configuration_sets.id
configuration_sets_next_token = configuration_sets.next_token
configuration_sets_configuration_sets = configuration_sets.configuration_sets
```

---


### Phone_number

PhoneNumber resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `two_way_channel_role` | String |  | <p>An optional IAM Role Arn for a service to assume, to be able to post inbound SMS messages.</p> |
| `phone_number_id` | String | ✅ | <p>The unique identifier of the phone number. Valid values for this field can be either the PhoneNumberId or PhoneNumberArn.</p> <important> <p>If you are using a shared AWS End User Messaging SMS and Voice resource then you must use the full Amazon Resource Name(ARN).</p> </important> |
| `two_way_channel_arn` | String |  | <p>The Amazon Resource Name (ARN) of the two way channel.</p> |
| `deletion_protection_enabled` | bool |  | <p>By default this is set to false. When set to true the phone number can't be deleted. </p> |
| `opt_out_list_name` | String |  | <p>The OptOutList to add the phone number to. Valid values for this field can be either the OutOutListName or OutOutListArn.</p> |
| `two_way_enabled` | bool |  | <p>By default this is set to false. When set to true you can receive incoming text messages from your end recipients.</p> |
| `international_sending_enabled` | bool |  | <p>By default this is set to false. When set to true the international sending of phone number is Enabled. </p> |
| `self_managed_opt_outs_enabled` | bool |  | <p>By default this is set to false. When an end recipient sends a message that begins with HELP or STOP to one of your dedicated numbers, AWS End User Messaging SMS and Voice automatically replies with a customizable message and adds the end recipient to the OptOutList. When set to true you're responsible for responding to HELP and STOP requests. You're also responsible for tracking and honoring opt-out requests.</p> |



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


### Keyword

Keyword resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `keyword_action` | String |  | <p>The action to perform for the new keyword when it is received.</p> <ul> <li> <p>AUTOMATIC_RESPONSE: A message is sent to the recipient.</p> </li> <li> <p>OPT_OUT: Keeps the recipient from receiving future messages.</p> </li> <li> <p>OPT_IN: The recipient wants to receive future messages.</p> </li> </ul> |
| `keyword` | String | ✅ | <p>The new keyword to add.</p> |
| `origination_identity` | String | ✅ | <p>The origination identity to use such as a PhoneNumberId, PhoneNumberArn, SenderId or SenderIdArn. You can use <a>DescribePhoneNumbers</a> get the values for PhoneNumberId and PhoneNumberArn while <a>DescribeSenderIds</a> can be used to get the values for SenderId and SenderIdArn.</p> <important> <p>If you are using a shared AWS End User Messaging SMS and Voice resource then you must use the full Amazon Resource Name(ARN).</p> </important> |
| `keyword_message` | String | ✅ | <p>The message associated with the keyword.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create keyword
keyword = provider.pinpoint_sms.Keyword {
    keyword = "value"  # <p>The new keyword to add.</p>
    origination_identity = "value"  # <p>The origination identity to use such as a PhoneNumberId, PhoneNumberArn, SenderId or SenderIdArn. You can use <a>DescribePhoneNumbers</a> get the values for PhoneNumberId and PhoneNumberArn while <a>DescribeSenderIds</a> can be used to get the values for SenderId and SenderIdArn.</p> <important> <p>If you are using a shared AWS End User Messaging SMS and Voice resource then you must use the full Amazon Resource Name(ARN).</p> </important>
    keyword_message = "value"  # <p>The message associated with the keyword.</p>
}

```

---


### Spend_limits

SpendLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spend_limits` | Vec<String> | <p>An array of SpendLimit objects that contain the details for the requested spend limits.</p> |
| `next_token` | String | <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access spend_limits outputs
spend_limits_id = spend_limits.id
spend_limits_spend_limits = spend_limits.spend_limits
spend_limits_next_token = spend_limits.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple configuration_set_event_destination resources
configuration_set_event_destination_0 = provider.pinpoint_sms.Configuration_set_event_destination {
    configuration_set_name = "value-0"
}
configuration_set_event_destination_1 = provider.pinpoint_sms.Configuration_set_event_destination {
    configuration_set_name = "value-1"
}
configuration_set_event_destination_2 = provider.pinpoint_sms.Configuration_set_event_destination {
    configuration_set_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    configuration_set_event_destination = provider.pinpoint_sms.Configuration_set_event_destination {
        configuration_set_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Pinpoint_sms Documentation](https://docs.aws.amazon.com/pinpoint_sms/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
