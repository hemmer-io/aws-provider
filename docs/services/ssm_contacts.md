# Ssm_contacts Service



**Resources**: 7

---

## Overview

The ssm_contacts service provides access to 7 resource types:

- [Rotation_override](#rotation_override) [CRD]
- [Rotation](#rotation) [CRUD]
- [Page](#page) [R]
- [Contact_policy](#contact_policy) [CR]
- [Engagement](#engagement) [R]
- [Contact_channel](#contact_channel) [CRUD]
- [Contact](#contact) [CRUD]

---

## Resources


### Rotation_override

RotationOverride resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rotation_id` | String | ✅ | <p>The Amazon Resource Name (ARN) of the rotation to create an override for.</p> |
| `end_time` | String | ✅ | <p>The date and time when the override ends.</p> |
| `start_time` | String | ✅ | <p>The date and time when the override goes into effect.</p> |
| `new_contact_ids` | Vec<String> | ✅ | <p>The Amazon Resource Names (ARNs) of the contacts to replace those in the current on-call
         rotation with.</p>
         <p>If you want to include any current team members in the override shift, you must include
         their ARNs in the new contact ID list.</p> |
| `idempotency_token` | String |  | <p>A token that ensures that the operation is called only once with the specified
         details.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | <p>The date and time when the override ends.</p> |
| `create_time` | String | <p>The date and time when the override was created.</p> |
| `new_contact_ids` | Vec<String> | <p>The Amazon Resource Names (ARNs) of the contacts assigned to the override of the on-call
         rotation.</p> |
| `start_time` | String | <p>The date and time when the override goes into effect.</p> |
| `rotation_override_id` | String | <p>The Amazon Resource Name (ARN) of the override to an on-call rotation.</p> |
| `rotation_arn` | String | <p>The Amazon Resource Name (ARN) of the on-call rotation that was overridden.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rotation_override
rotation_override = provider.ssm_contacts.Rotation_override {
    rotation_id = "value"  # <p>The Amazon Resource Name (ARN) of the rotation to create an override for.</p>
    end_time = "value"  # <p>The date and time when the override ends.</p>
    start_time = "value"  # <p>The date and time when the override goes into effect.</p>
    new_contact_ids = "value"  # <p>The Amazon Resource Names (ARNs) of the contacts to replace those in the current on-call
         rotation with.</p>
         <p>If you want to include any current team members in the override shift, you must include
         their ARNs in the new contact ID list.</p>
}

# Access rotation_override outputs
rotation_override_id = rotation_override.id
rotation_override_end_time = rotation_override.end_time
rotation_override_create_time = rotation_override.create_time
rotation_override_new_contact_ids = rotation_override.new_contact_ids
rotation_override_start_time = rotation_override.start_time
rotation_override_rotation_override_id = rotation_override.rotation_override_id
rotation_override_rotation_arn = rotation_override.rotation_arn
```

---


### Rotation

Rotation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_ids` | Vec<String> | ✅ | <p>The Amazon Resource Names (ARNs) of the contacts to add to the rotation.</p>
         <note>
            <p>Only the <code>PERSONAL</code> contact type is supported. The contact types
               <code>ESCALATION</code> and <code>ONCALL_SCHEDULE</code> are not supported for this
            operation. </p>
         </note>
         <p>The order that you list the contacts in is their shift order in the rotation schedule.
         To change the order of the contact's shifts, use the <a>UpdateRotation</a>
         operation.</p> |
| `start_time` | String |  | <p>The date and time that the rotation goes into effect.</p> |
| `recurrence` | String | ✅ | <p>Information about the rule that specifies when a shift's team members rotate.</p> |
| `name` | String | ✅ | <p>The name of the rotation.</p> |
| `idempotency_token` | String |  | <p>A token that ensures that the operation is called only once with the specified
         details.</p> |
| `time_zone_id` | String | ✅ | <p>The time zone to base the rotation’s activity on in Internet Assigned Numbers Authority
         (IANA) format. For example: "America/Los_Angeles", "UTC", or "Asia/Seoul". For more
         information, see the <a href="https://www.iana.org/time-zones">Time Zone
            Database</a> on the IANA website.</p>
         <note>
            <p>Designators for time zones that don’t support Daylight Savings Time rules, such as
            Pacific Standard Time (PST), are not supported.</p>
         </note> |
| `tags` | Vec<String> |  | <p>Optional metadata to assign to the rotation. Tags enable you to categorize a resource in
         different ways, such as by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/incident-manager/latest/userguide/tagging.html">Tagging
            Incident Manager resources</a> in the <i>Incident Manager User
            Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | <p>The specified start time for the on-call rotation.</p> |
| `time_zone_id` | String | <p>The time zone that the rotation’s activity is based on, in Internet Assigned Numbers
         Authority (IANA) format.</p> |
| `contact_ids` | Vec<String> | <p>The Amazon Resource Names (ARNs) of the contacts assigned to the on-call rotation
         team.</p> |
| `rotation_arn` | String | <p>The Amazon Resource Name (ARN) of the on-call rotation.</p> |
| `recurrence` | String | <p>Specifies how long a rotation lasts before restarting at the beginning of the shift
         order.</p> |
| `name` | String | <p>The name of the on-call rotation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rotation
rotation = provider.ssm_contacts.Rotation {
    contact_ids = "value"  # <p>The Amazon Resource Names (ARNs) of the contacts to add to the rotation.</p>
         <note>
            <p>Only the <code>PERSONAL</code> contact type is supported. The contact types
               <code>ESCALATION</code> and <code>ONCALL_SCHEDULE</code> are not supported for this
            operation. </p>
         </note>
         <p>The order that you list the contacts in is their shift order in the rotation schedule.
         To change the order of the contact's shifts, use the <a>UpdateRotation</a>
         operation.</p>
    recurrence = "value"  # <p>Information about the rule that specifies when a shift's team members rotate.</p>
    name = "value"  # <p>The name of the rotation.</p>
    time_zone_id = "value"  # <p>The time zone to base the rotation’s activity on in Internet Assigned Numbers Authority
         (IANA) format. For example: "America/Los_Angeles", "UTC", or "Asia/Seoul". For more
         information, see the <a href="https://www.iana.org/time-zones">Time Zone
            Database</a> on the IANA website.</p>
         <note>
            <p>Designators for time zones that don’t support Daylight Savings Time rules, such as
            Pacific Standard Time (PST), are not supported.</p>
         </note>
}

# Access rotation outputs
rotation_id = rotation.id
rotation_start_time = rotation.start_time
rotation_time_zone_id = rotation.time_zone_id
rotation_contact_ids = rotation.contact_ids
rotation_rotation_arn = rotation.rotation_arn
rotation_recurrence = rotation.recurrence
rotation_name = rotation.name
```

---


### Page

Page resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `read_time` | String | <p>The time that the contact channel acknowledged the engagement.</p> |
| `delivery_time` | String | <p>The time that the contact channel received the engagement.</p> |
| `public_subject` | String | <p>The insecure subject of the message that was sent to the contact. Use this field for
         engagements to <code>SMS</code>.</p> |
| `contact_arn` | String | <p>The ARN of the contact that was engaged.</p> |
| `sender` | String | <p>The user that started the engagement.</p> |
| `content` | String | <p>The secure content of the message that was sent to the contact. Use this field for
         engagements to <code>VOICE</code> and <code>EMAIL</code>.</p> |
| `sent_time` | String | <p>The time the engagement was sent to the contact channel.</p> |
| `subject` | String | <p>The secure subject of the message that was sent to the contact. Use this field for
         engagements to <code>VOICE</code> and <code>EMAIL</code>.</p> |
| `engagement_arn` | String | <p>The ARN of the engagement that engaged the contact channel.</p> |
| `public_content` | String | <p>The insecure content of the message that was sent to the contact. Use this field for
         engagements to <code>SMS</code>.</p> |
| `incident_id` | String | <p>The ARN of the incident that engaged the contact channel.</p> |
| `page_arn` | String | <p>The Amazon Resource Name (ARN) of the engagement to a contact channel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access page outputs
page_id = page.id
page_read_time = page.read_time
page_delivery_time = page.delivery_time
page_public_subject = page.public_subject
page_contact_arn = page.contact_arn
page_sender = page.sender
page_content = page.content
page_sent_time = page.sent_time
page_subject = page.subject
page_engagement_arn = page.engagement_arn
page_public_content = page.public_content
page_incident_id = page.incident_id
page_page_arn = page.page_arn
```

---


### Contact_policy

ContactPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the contact or escalation plan.</p> |
| `policy` | String | ✅ | <p>Details of the resource policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>Details about the resource policy attached to the contact or escalation plan.</p> |
| `contact_arn` | String | <p>The ARN of the contact or escalation plan.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact_policy
contact_policy = provider.ssm_contacts.Contact_policy {
    contact_arn = "value"  # <p>The Amazon Resource Name (ARN) of the contact or escalation plan.</p>
    policy = "value"  # <p>Details of the resource policy.</p>
}

# Access contact_policy outputs
contact_policy_id = contact_policy.id
contact_policy_policy = contact_policy.policy
contact_policy_contact_arn = contact_policy.contact_arn
```

---


### Engagement

Engagement resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | <p>The secure content of the message that was sent to the contact. Use this field for
         engagements to <code>VOICE</code> and <code>EMAIL</code>.</p> |
| `start_time` | String | <p>The time that the engagement started.</p> |
| `public_subject` | String | <p>The insecure subject of the message that was sent to the contact. Use this field for
         engagements to <code>SMS</code>.</p> |
| `engagement_arn` | String | <p>The ARN of the engagement.</p> |
| `stop_time` | String | <p>The time that the engagement ended.</p> |
| `public_content` | String | <p>The insecure content of the message that was sent to the contact. Use this field for
         engagements to <code>SMS</code>.</p> |
| `contact_arn` | String | <p>The ARN of the escalation plan or contacts involved in the engagement.</p> |
| `incident_id` | String | <p>The ARN of the incident in which the engagement occurred.</p> |
| `subject` | String | <p>The secure subject of the message that was sent to the contact. Use this field for
         engagements to <code>VOICE</code> and <code>EMAIL</code>.</p> |
| `sender` | String | <p>The user that started the engagement.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access engagement outputs
engagement_id = engagement.id
engagement_content = engagement.content
engagement_start_time = engagement.start_time
engagement_public_subject = engagement.public_subject
engagement_engagement_arn = engagement.engagement_arn
engagement_stop_time = engagement.stop_time
engagement_public_content = engagement.public_content
engagement_contact_arn = engagement.contact_arn
engagement_incident_id = engagement.incident_id
engagement_subject = engagement.subject
engagement_sender = engagement.sender
```

---


### Contact_channel

ContactChannel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_address` | String | ✅ | <p>The details that Incident Manager uses when trying to engage the contact channel. The format
         is dependent on the type of the contact channel. The following are the expected
         formats:</p>
         <ul>
            <li>
               <p>SMS - '+' followed by the country code and phone number</p>
            </li>
            <li>
               <p>VOICE - '+' followed by the country code and phone number</p>
            </li>
            <li>
               <p>EMAIL - any standard email format</p>
            </li>
         </ul> |
| `defer_activation` | bool |  | <p>If you want to activate the channel at a later time, you can choose to defer activation.
         Incident Manager can't engage your contact channel until it has been activated.</p> |
| `contact_id` | String | ✅ | <p>The Amazon Resource Name (ARN) of the contact you are adding the contact channel
         to.</p> |
| `name` | String | ✅ | <p>The name of the contact channel.</p> |
| `idempotency_token` | String |  | <p>A token ensuring that the operation is called only once with the specified
         details.</p> |
| `type` | String | ✅ | <p>Incident Manager supports three types of contact channels:</p>
         <ul>
            <li>
               <p>
                  <code>SMS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>VOICE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>EMAIL</code>
               </p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The type of contact channel. The type is <code>SMS</code>, <code>VOICE</code>, or
            <code>EMAIL</code>.</p> |
| `contact_channel_arn` | String | <p>The ARN of the contact channel.</p> |
| `delivery_address` | String | <p>The details that Incident Manager uses when trying to engage the contact channel.</p> |
| `name` | String | <p>The name of the contact channel</p> |
| `contact_arn` | String | <p>The ARN of the contact that the channel belongs to.</p> |
| `activation_status` | String | <p>A Boolean value indicating if the contact channel has been activated or not.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact_channel
contact_channel = provider.ssm_contacts.Contact_channel {
    delivery_address = "value"  # <p>The details that Incident Manager uses when trying to engage the contact channel. The format
         is dependent on the type of the contact channel. The following are the expected
         formats:</p>
         <ul>
            <li>
               <p>SMS - '+' followed by the country code and phone number</p>
            </li>
            <li>
               <p>VOICE - '+' followed by the country code and phone number</p>
            </li>
            <li>
               <p>EMAIL - any standard email format</p>
            </li>
         </ul>
    contact_id = "value"  # <p>The Amazon Resource Name (ARN) of the contact you are adding the contact channel
         to.</p>
    name = "value"  # <p>The name of the contact channel.</p>
    type = "value"  # <p>Incident Manager supports three types of contact channels:</p>
         <ul>
            <li>
               <p>
                  <code>SMS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>VOICE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>EMAIL</code>
               </p>
            </li>
         </ul>
}

# Access contact_channel outputs
contact_channel_id = contact_channel.id
contact_channel_type = contact_channel.type
contact_channel_contact_channel_arn = contact_channel.contact_channel_arn
contact_channel_delivery_address = contact_channel.delivery_address
contact_channel_name = contact_channel.name
contact_channel_contact_arn = contact_channel.contact_arn
contact_channel_activation_status = contact_channel.activation_status
```

---


### Contact

Contact resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p>The full name of the contact or escalation plan.</p> |
| `plan` | String | ✅ | <p>A list of stages. A contact has an engagement plan with stages that contact specified
         contact channels. An escalation plan uses stages that contact specified contacts.</p> |
| `tags` | Vec<String> |  | <p>Adds a tag to the target. You can only tag resources created in the first Region of your
         replication set.</p> |
| `idempotency_token` | String |  | <p>A token ensuring that the operation is called only once with the specified
         details.</p> |
| `alias` | String | ✅ | <p>The short name to quickly identify a contact or escalation plan. The contact alias must
         be unique and identifiable.</p> |
| `type` | String | ✅ | <p>The type of contact to create.</p>
         <ul>
            <li>
               <p>
                  <code>PERSONAL</code>: A single, individual contact.</p>
            </li>
            <li>
               <p>
                  <code>ESCALATION</code>: An escalation plan.</p>
            </li>
            <li>
               <p>
                  <code>ONCALL_SCHEDULE</code>: An on-call schedule.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contact_arn` | String | <p>The ARN of the contact or escalation plan.</p> |
| `type` | String | <p>The type of contact.</p> |
| `plan` | String | <p>Details about the specific timing or stages and targets of the escalation plan or
         engagement plan.</p> |
| `display_name` | String | <p>The full name of the contact or escalation plan.</p> |
| `alias` | String | <p>The alias of the contact or escalation plan. The alias is unique and
         identifiable.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact
contact = provider.ssm_contacts.Contact {
    plan = "value"  # <p>A list of stages. A contact has an engagement plan with stages that contact specified
         contact channels. An escalation plan uses stages that contact specified contacts.</p>
    alias = "value"  # <p>The short name to quickly identify a contact or escalation plan. The contact alias must
         be unique and identifiable.</p>
    type = "value"  # <p>The type of contact to create.</p>
         <ul>
            <li>
               <p>
                  <code>PERSONAL</code>: A single, individual contact.</p>
            </li>
            <li>
               <p>
                  <code>ESCALATION</code>: An escalation plan.</p>
            </li>
            <li>
               <p>
                  <code>ONCALL_SCHEDULE</code>: An on-call schedule.</p>
            </li>
         </ul>
}

# Access contact outputs
contact_id = contact.id
contact_contact_arn = contact.contact_arn
contact_type = contact.type
contact_plan = contact.plan
contact_display_name = contact.display_name
contact_alias = contact.alias
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple rotation_override resources
rotation_override_0 = provider.ssm_contacts.Rotation_override {
    rotation_id = "value-0"
    end_time = "value-0"
    start_time = "value-0"
    new_contact_ids = "value-0"
}
rotation_override_1 = provider.ssm_contacts.Rotation_override {
    rotation_id = "value-1"
    end_time = "value-1"
    start_time = "value-1"
    new_contact_ids = "value-1"
}
rotation_override_2 = provider.ssm_contacts.Rotation_override {
    rotation_id = "value-2"
    end_time = "value-2"
    start_time = "value-2"
    new_contact_ids = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rotation_override = provider.ssm_contacts.Rotation_override {
        rotation_id = "production-value"
        end_time = "production-value"
        start_time = "production-value"
        new_contact_ids = "production-value"
    }
```

---

## Related Documentation

- [AWS Ssm_contacts Documentation](https://docs.aws.amazon.com/ssm_contacts/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
