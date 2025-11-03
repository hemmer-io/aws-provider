# Lex_models Service



**Resources**: 23

---

## Overview

The lex_models service provides access to 23 resource types:

- [Bot_resource_generation](#bot_resource_generation) [R]
- [Slot_type](#slot_type) [CRUD]
- [Custom_vocabulary](#custom_vocabulary) [D]
- [Slot](#slot) [CRUD]
- [Utterances](#utterances) [D]
- [Custom_vocabulary_metadata](#custom_vocabulary_metadata) [R]
- [Resource_policy](#resource_policy) [CRUD]
- [Bot_version](#bot_version) [CRD]
- [Test_set_generation](#test_set_generation) [R]
- [Bot_locale](#bot_locale) [CRUD]
- [Export](#export) [CRUD]
- [Test_set](#test_set) [RUD]
- [Bot_alias](#bot_alias) [CRUD]
- [Intent](#intent) [CRUD]
- [Bot](#bot) [CRUD]
- [Bot_replica](#bot_replica) [CRD]
- [Import](#import) [RD]
- [Upload_url](#upload_url) [C]
- [Resource_policy_statement](#resource_policy_statement) [CD]
- [Test_set_discrepancy_report](#test_set_discrepancy_report) [CR]
- [Test_execution](#test_execution) [R]
- [Test_execution_artifacts_url](#test_execution_artifacts_url) [R]
- [Bot_recommendation](#bot_recommendation) [RU]

---

## Resources


### Bot_resource_generation

BotResourceGeneration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_id` | String | <p>The unique identifier of the bot for which the generation request was 
      made.</p> |
| `generation_input_prompt` | String | <p>The prompt used in the generation request.</p> |
| `generation_id` | String | <p>The generation ID for which to return the generation details.</p> |
| `generated_bot_locale_url` | String | <p>The Amazon S3 location of the generated bot locale configuration.</p> |
| `last_updated_date_time` | String | <p>The date and time at which the generated item was updated.</p> |
| `creation_date_time` | String | <p>The date and time at which the item was generated.</p> |
| `locale_id` | String | <p>The locale of the bot for which the generation request was made.</p> |
| `generation_status` | String | <p>The status of the generation request.</p> |
| `failure_reasons` | Vec<String> | <p>A list of reasons why the generation of bot resources through natural language description failed.</p> |
| `bot_version` | String | <p>The version of the bot for which the generation request was made.</p> |
| `model_arn` | String | <p>The ARN of the model used to generate the bot resources.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bot_resource_generation outputs
bot_resource_generation_id = bot_resource_generation.id
bot_resource_generation_bot_id = bot_resource_generation.bot_id
bot_resource_generation_generation_input_prompt = bot_resource_generation.generation_input_prompt
bot_resource_generation_generation_id = bot_resource_generation.generation_id
bot_resource_generation_generated_bot_locale_url = bot_resource_generation.generated_bot_locale_url
bot_resource_generation_last_updated_date_time = bot_resource_generation.last_updated_date_time
bot_resource_generation_creation_date_time = bot_resource_generation.creation_date_time
bot_resource_generation_locale_id = bot_resource_generation.locale_id
bot_resource_generation_generation_status = bot_resource_generation.generation_status
bot_resource_generation_failure_reasons = bot_resource_generation.failure_reasons
bot_resource_generation_bot_version = bot_resource_generation.bot_version
bot_resource_generation_model_arn = bot_resource_generation.model_arn
```

---


### Slot_type

SlotType resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_source_setting` | String |  | <p>Sets the type of external information used to create the slot
         type.</p> |
| `composite_slot_type_setting` | String |  | <p>Specifications for a composite slot type.</p> |
| `bot_id` | String | ✅ | <p>The identifier of the bot associated with this slot type.</p> |
| `bot_version` | String | ✅ | <p>The identifier of the bot version associated with this slot
         type.</p> |
| `parent_slot_type_signature` | String |  | <p>The built-in slot type used as a parent of this slot type. When you
         define a parent slot type, the new slot type has the configuration of
         the parent slot type.</p>
         <p>Only <code>AMAZON.AlphaNumeric</code> is supported.</p> |
| `locale_id` | String | ✅ | <p>The identifier of the language and locale that the slot type will be
         used in. The string must match one of the supported locales. All of the
         bots, intents, and slots used by the slot type must have the same
         locale. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p> |
| `description` | String |  | <p>A description of the slot type. Use the description to help identify
         the slot type in lists.</p> |
| `value_selection_setting` | String |  | <p>Determines the strategy that Amazon Lex uses to select a value from the
         list of possible values. The field can be set to one of the following
         values:</p>
         <ul>
            <li>
               <p>
                  <code>ORIGINAL_VALUE</code> - Returns the value entered by the
               user, if the user value is similar to the slot value.</p>
            </li>
            <li>
               <p>
                  <code>TOP_RESOLUTION</code> - If there is a resolution list for
               the slot, return the first value in the resolution list. If there
               is no resolution list, return null.</p>
            </li>
         </ul>
         <p>If you don't specify the <code>valueSelectionSetting</code>
         parameter, the default is <code>ORIGINAL_VALUE</code>.</p> |
| `slot_type_name` | String | ✅ | <p>The name for the slot. A slot type name must be unique within the
         intent.</p> |
| `slot_type_values` | Vec<String> |  | <p>A list of <code>SlotTypeValue</code> objects that defines the values
         that the slot type can take. Each value can have a list of synonyms,
         additional values that help train the machine learning model about the
         values that it resolves for a slot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_date_time` | String | <p>A timestamp of the date and time that the slot type was last
         updated.</p> |
| `composite_slot_type_setting` | String | <p>Specifications for a composite slot type.</p> |
| `description` | String | <p>The description specified for the slot type.</p> |
| `locale_id` | String | <p>The language and locale specified for the slot type.</p> |
| `slot_type_values` | Vec<String> | <p>The values that the slot type can take. Includes any synonyms for
         the slot type values.</p> |
| `parent_slot_type_signature` | String | <p>The built in slot type used as a parent to this slot type.</p> |
| `external_source_setting` | String |  |
| `bot_id` | String | <p>The identifier of the bot associated with the slot type.</p> |
| `bot_version` | String | <p>The version of the bot associated with the slot type.</p> |
| `value_selection_setting` | String | <p>The strategy that Amazon Lex uses to choose a value from a list of
         possible values.</p> |
| `slot_type_id` | String | <p>The unique identifier for the slot type.</p> |
| `slot_type_name` | String | <p>The name specified for the slot type.</p> |
| `creation_date_time` | String | <p>A timestamp of the date and time that the slot type was
         created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create slot_type
slot_type = provider.lex_models.Slot_type {
    bot_id = "value"  # <p>The identifier of the bot associated with this slot type.</p>
    bot_version = "value"  # <p>The identifier of the bot version associated with this slot
         type.</p>
    locale_id = "value"  # <p>The identifier of the language and locale that the slot type will be
         used in. The string must match one of the supported locales. All of the
         bots, intents, and slots used by the slot type must have the same
         locale. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    slot_type_name = "value"  # <p>The name for the slot. A slot type name must be unique within the
         intent.</p>
}

# Access slot_type outputs
slot_type_id = slot_type.id
slot_type_last_updated_date_time = slot_type.last_updated_date_time
slot_type_composite_slot_type_setting = slot_type.composite_slot_type_setting
slot_type_description = slot_type.description
slot_type_locale_id = slot_type.locale_id
slot_type_slot_type_values = slot_type.slot_type_values
slot_type_parent_slot_type_signature = slot_type.parent_slot_type_signature
slot_type_external_source_setting = slot_type.external_source_setting
slot_type_bot_id = slot_type.bot_id
slot_type_bot_version = slot_type.bot_version
slot_type_value_selection_setting = slot_type.value_selection_setting
slot_type_slot_type_id = slot_type.slot_type_id
slot_type_slot_type_name = slot_type.slot_type_name
slot_type_creation_date_time = slot_type.creation_date_time
```

---


### Custom_vocabulary

CustomVocabulary resource

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


### Slot

Slot resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bot_id` | String | ✅ | <p>The identifier of the bot associated with the slot.</p> |
| `slot_name` | String | ✅ | <p>The name of the slot. Slot names must be unique within the bot that
         contains the slot.</p> |
| `description` | String |  | <p>A description of the slot. Use this to help identify the slot in
         lists.</p> |
| `bot_version` | String | ✅ | <p>The version of the bot associated with the slot.</p> |
| `slot_type_id` | String |  | <p>The unique identifier for the slot type associated with this slot.
         The slot type determines the values that can be entered into the
         slot.</p> |
| `obfuscation_setting` | String |  | <p>Determines how slot values are used in Amazon CloudWatch logs. If the value of
         the <code>obfuscationSetting</code> parameter is
            <code>DefaultObfuscation</code>, slot values are obfuscated in the
         log output. If the value is <code>None</code>, the actual value is
         present in the log output.</p>
         <p>The default is to obfuscate values in the CloudWatch logs.</p> |
| `value_elicitation_setting` | String | ✅ | <p>Specifies prompts that Amazon Lex sends to the user to elicit a response
         that provides the value for the slot. </p> |
| `locale_id` | String | ✅ | <p>The identifier of the language and locale that the slot will be used
         in. The string must match one of the supported locales. All of the
         bots, intents, slot types used by the slot must have the same locale.
         For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p> |
| `intent_id` | String | ✅ | <p>The identifier of the intent that contains the slot.</p> |
| `multiple_values_setting` | String |  | <p>Indicates whether the slot returns multiple values in one response.
         Multi-value slots are only available in the <code>en-US</code> locale. 
         If you set this value to <code>true</code> in any other locale, Amazon Lex throws a
            <code>ValidationException</code>. </p>
         <p>If the <code>multipleValuesSetting</code> is not set, the default
         value is <code>false</code>.</p> |
| `sub_slot_setting` | String |  | <p>Specifications for the constituent sub slots and the  
        expression for the composite slot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sub_slot_setting` | String | <p>Specifications for the constituent sub slots and the  
        expression for the composite slot.</p> |
| `slot_name` | String | <p>The name specified for the slot.</p> |
| `bot_id` | String | <p>The identifier of the bot associated with the slot.</p> |
| `slot_type_id` | String | <p>The identifier of the slot type that determines the values entered
         into the slot.</p> |
| `description` | String | <p>The description specified for the slot.</p> |
| `obfuscation_setting` | String | <p>Whether slot values are shown in Amazon CloudWatch logs. If the value is
            <code>None</code>, the actual value of the slot is shown in
         logs.</p> |
| `creation_date_time` | String | <p>A timestamp of the date and time that the slot was created.</p> |
| `slot_id` | String | <p>The unique identifier generated for the slot.</p> |
| `bot_version` | String | <p>The version of the bot associated with the slot.</p> |
| `locale_id` | String | <p>The language and locale specified for the slot.</p> |
| `value_elicitation_setting` | String | <p>Prompts that Amazon Lex uses to elicit a value for the slot.</p> |
| `last_updated_date_time` | String | <p>A timestamp of the date and time that the slot was last
         updated.</p> |
| `multiple_values_setting` | String | <p>Indicates whether the slot accepts multiple values in a single
         utterance.</p>
         <p>If the <code>multipleValuesSetting</code> is not set, the default
         value is <code>false</code>.</p> |
| `intent_id` | String | <p>The identifier of the intent associated with the slot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create slot
slot = provider.lex_models.Slot {
    bot_id = "value"  # <p>The identifier of the bot associated with the slot.</p>
    slot_name = "value"  # <p>The name of the slot. Slot names must be unique within the bot that
         contains the slot.</p>
    bot_version = "value"  # <p>The version of the bot associated with the slot.</p>
    value_elicitation_setting = "value"  # <p>Specifies prompts that Amazon Lex sends to the user to elicit a response
         that provides the value for the slot. </p>
    locale_id = "value"  # <p>The identifier of the language and locale that the slot will be used
         in. The string must match one of the supported locales. All of the
         bots, intents, slot types used by the slot must have the same locale.
         For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    intent_id = "value"  # <p>The identifier of the intent that contains the slot.</p>
}

# Access slot outputs
slot_id = slot.id
slot_sub_slot_setting = slot.sub_slot_setting
slot_slot_name = slot.slot_name
slot_bot_id = slot.bot_id
slot_slot_type_id = slot.slot_type_id
slot_description = slot.description
slot_obfuscation_setting = slot.obfuscation_setting
slot_creation_date_time = slot.creation_date_time
slot_slot_id = slot.slot_id
slot_bot_version = slot.bot_version
slot_locale_id = slot.locale_id
slot_value_elicitation_setting = slot.value_elicitation_setting
slot_last_updated_date_time = slot.last_updated_date_time
slot_multiple_values_setting = slot.multiple_values_setting
slot_intent_id = slot.intent_id
```

---


### Utterances

Utterances resource

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


### Custom_vocabulary_metadata

CustomVocabularyMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_date_time` | String | <p>The date and time that the custom vocabulary was last updated.</p> |
| `bot_id` | String | <p>The identifier of the bot that contains the custom vocabulary.</p> |
| `custom_vocabulary_status` | String | <p>The status of the custom vocabulary. If the status is 
      <code>Ready</code> the custom vocabulary is ready to use.</p> |
| `bot_version` | String | <p>The version of the bot that contains the custom vocabulary to describe.</p> |
| `creation_date_time` | String | <p>The date and time that the custom vocabulary was created.</p> |
| `locale_id` | String | <p>The locale that contains the custom vocabulary to describe.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_vocabulary_metadata outputs
custom_vocabulary_metadata_id = custom_vocabulary_metadata.id
custom_vocabulary_metadata_last_updated_date_time = custom_vocabulary_metadata.last_updated_date_time
custom_vocabulary_metadata_bot_id = custom_vocabulary_metadata.bot_id
custom_vocabulary_metadata_custom_vocabulary_status = custom_vocabulary_metadata.custom_vocabulary_status
custom_vocabulary_metadata_bot_version = custom_vocabulary_metadata.bot_version
custom_vocabulary_metadata_creation_date_time = custom_vocabulary_metadata.creation_date_time
custom_vocabulary_metadata_locale_id = custom_vocabulary_metadata.locale_id
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>A resource policy to add to the resource. The policy is a JSON
         structure that contains one or more statements that define the policy.
         The policy must follow the IAM syntax. For more information about the
         contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy
            reference </a>. </p>
         <p>If the policy isn't valid, Amazon Lex returns a validation
         exception.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the bot or bot alias that the
         resource policy is attached to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The JSON structure that contains the resource policy. For more
         information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy
            reference </a>.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the bot or bot alias that the
         resource policy is attached to.</p> |
| `revision_id` | String | <p>The current revision of the resource policy. Use the revision ID to
         make sure that you are updating the most current version of a resource
         policy when you add a policy statement to a resource, delete a
         resource, or update a resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.lex_models.Resource_policy {
    policy = "value"  # <p>A resource policy to add to the resource. The policy is a JSON
         structure that contains one or more statements that define the policy.
         The policy must follow the IAM syntax. For more information about the
         contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html"> IAM JSON policy
            reference </a>. </p>
         <p>If the policy isn't valid, Amazon Lex returns a validation
         exception.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the bot or bot alias that the
         resource policy is attached to.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
resource_policy_resource_arn = resource_policy.resource_arn
resource_policy_revision_id = resource_policy.revision_id
```

---


### Bot_version

BotVersion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the version. Use the description to help identify
         the version in lists.</p> |
| `bot_version_locale_specification` | HashMap<String, String> | ✅ | <p>Specifies the locales that Amazon Lex adds to this version. You can
         choose the <code>Draft</code> version or any other previously published
         version for each locale. When you specify a source version, the locale
         data is copied from the source version to the new version.</p> |
| `bot_id` | String | ✅ | <p>The identifier of the bot to create the version for.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_version` | String | <p>The version of the bot that was described.</p> |
| `description` | String | <p>The description specified for the bot.</p> |
| `bot_name` | String | <p>The name of the bot that contains the version.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role that has permission to
         access the bot version.</p> |
| `idle_session_ttl_in_seconds` | i64 | <p>The number of seconds that a session with the bot remains active
         before it is discarded by Amazon Lex.</p> |
| `creation_date_time` | String | <p>A timestamp of the date and time that the bot version was
         created.</p> |
| `failure_reasons` | Vec<String> | <p>If the <code>botStatus</code> is <code>Failed</code>, this contains
         a list of reasons that the version couldn't be built.</p> |
| `data_privacy` | String | <p>Data privacy settings for the bot version.</p> |
| `bot_status` | String | <p>The current status of the bot. When the status is
            <code>Available</code>, the bot version is ready for use.</p> |
| `parent_bot_networks` | Vec<String> | <p>A list of the networks to which the bot version you described belongs.</p> |
| `bot_type` | String | <p>The type of the bot in the version that was described.</p> |
| `bot_members` | Vec<String> | <p>The members of bot network in the version that was described.</p> |
| `bot_id` | String | <p>The identifier of the bot that contains the version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot_version
bot_version = provider.lex_models.Bot_version {
    bot_version_locale_specification = "value"  # <p>Specifies the locales that Amazon Lex adds to this version. You can
         choose the <code>Draft</code> version or any other previously published
         version for each locale. When you specify a source version, the locale
         data is copied from the source version to the new version.</p>
    bot_id = "value"  # <p>The identifier of the bot to create the version for.</p>
}

# Access bot_version outputs
bot_version_id = bot_version.id
bot_version_bot_version = bot_version.bot_version
bot_version_description = bot_version.description
bot_version_bot_name = bot_version.bot_name
bot_version_role_arn = bot_version.role_arn
bot_version_idle_session_ttl_in_seconds = bot_version.idle_session_ttl_in_seconds
bot_version_creation_date_time = bot_version.creation_date_time
bot_version_failure_reasons = bot_version.failure_reasons
bot_version_data_privacy = bot_version.data_privacy
bot_version_bot_status = bot_version.bot_status
bot_version_parent_bot_networks = bot_version.parent_bot_networks
bot_version_bot_type = bot_version.bot_type
bot_version_bot_members = bot_version.bot_members
bot_version_bot_id = bot_version.bot_id
```

---


### Test_set_generation

TestSetGeneration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generation_data_source` | String | <p>The data source of the test set used for the test set generation.</p> |
| `description` | String | <p>The test set description for the test set generation.</p> |
| `storage_location` | String | <p>The Amazon S3 storage location for the test set generation.</p> |
| `role_arn` | String | <p> The roleARN of the test set used for the test set generation.</p> |
| `test_set_generation_id` | String | <p>The unique identifier of the test set generation.</p> |
| `failure_reasons` | Vec<String> | <p>The reasons the test set generation failed.</p> |
| `test_set_name` | String | <p>The test set name for the generated test set.</p> |
| `test_set_id` | String | <p>The unique identifier for the test set created for the generated test set.</p> |
| `last_updated_date_time` | String | <p>The date and time of the last update for the test set generation.</p> |
| `creation_date_time` | String | <p>The creation date and time for the test set generation.</p> |
| `test_set_generation_status` | String | <p>The status for the test set generation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access test_set_generation outputs
test_set_generation_id = test_set_generation.id
test_set_generation_generation_data_source = test_set_generation.generation_data_source
test_set_generation_description = test_set_generation.description
test_set_generation_storage_location = test_set_generation.storage_location
test_set_generation_role_arn = test_set_generation.role_arn
test_set_generation_test_set_generation_id = test_set_generation.test_set_generation_id
test_set_generation_failure_reasons = test_set_generation.failure_reasons
test_set_generation_test_set_name = test_set_generation.test_set_name
test_set_generation_test_set_id = test_set_generation.test_set_id
test_set_generation_last_updated_date_time = test_set_generation.last_updated_date_time
test_set_generation_creation_date_time = test_set_generation.creation_date_time
test_set_generation_test_set_generation_status = test_set_generation.test_set_generation_status
```

---


### Bot_locale

BotLocale resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generative_ai_settings` | String |  |  |
| `bot_id` | String | ✅ | <p>The identifier of the bot to create the locale for.</p> |
| `locale_id` | String | ✅ | <p>The identifier of the language and locale that the bot will be used
         in. The string must match one of the supported locales. All of the
         intents, slot types, and slots used in the bot must have the same
         locale. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p> |
| `nlu_intent_confidence_threshold` | f64 | ✅ | <p>Determines the threshold where Amazon Lex will insert the
            <code>AMAZON.FallbackIntent</code>,
            <code>AMAZON.KendraSearchIntent</code>, or both when returning
         alternative intents. <code>AMAZON.FallbackIntent</code> and
            <code>AMAZON.KendraSearchIntent</code> are only inserted if they are
         configured for the bot.</p>
         <p>For example, suppose a bot is configured with the confidence
         threshold of 0.80 and the <code>AMAZON.FallbackIntent</code>. Amazon Lex
         returns three alternative intents with the following confidence scores:
         IntentA (0.70), IntentB (0.60), IntentC (0.50). The response from the
            <code>RecognizeText</code> operation would be:</p>
         <ul>
            <li>
               <p>AMAZON.FallbackIntent</p>
            </li>
            <li>
               <p>IntentA</p>
            </li>
            <li>
               <p>IntentB</p>
            </li>
            <li>
               <p>IntentC</p>
            </li>
         </ul> |
| `description` | String |  | <p>A description of the bot locale. Use this to help identify the bot
         locale in lists.</p> |
| `bot_version` | String | ✅ | <p>The version of the bot to create the locale for. This can only be
         the draft version of the bot.</p> |
| `voice_settings` | String |  | <p>The Amazon Polly voice ID that Amazon Lex uses for voice interaction with the
         user.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the locale.</p> |
| `locale_id` | String | <p>The unique identifier of the described locale.</p> |
| `nlu_intent_confidence_threshold` | f64 | <p>The confidence threshold where Amazon Lex inserts the
            <code>AMAZON.FallbackIntent</code> and
            <code>AMAZON.KendraSearchIntent</code> intents in the list of
         possible intents for an utterance.</p> |
| `intents_count` | i64 | <p>The number of intents defined for the locale.</p> |
| `failure_reasons` | Vec<String> | <p>if <code>botLocaleStatus</code> is <code>Failed</code>, Amazon Lex
         explains why it failed to build the bot.</p> |
| `creation_date_time` | String | <p>The date and time that the locale was created.</p> |
| `generative_ai_settings` | String | <p>Contains settings for Amazon Bedrock's generative AI features for your bot locale.</p> |
| `slot_types_count` | i64 | <p>The number of slot types defined for the locale.</p> |
| `last_build_submitted_date_time` | String | <p>The date and time that the locale was last submitted for
         building.</p> |
| `locale_name` | String | <p>The name of the locale.</p> |
| `bot_locale_status` | String | <p>The status of the bot. If the status is <code>Failed</code>, the
         reasons for the failure are listed in the <code>failureReasons</code>
         field.</p> |
| `recommended_actions` | Vec<String> | <p>Recommended actions to take to resolve an error in the
            <code>failureReasons</code> field.</p> |
| `bot_locale_history_events` | Vec<String> | <p>History of changes, such as when a locale is used in an alias, that
         have taken place for the locale.</p> |
| `bot_id` | String | <p>The identifier of the bot associated with the locale.</p> |
| `bot_version` | String | <p>The version of the bot associated with the
         locale.</p> |
| `last_updated_date_time` | String | <p>The date and time that the locale was last updated.</p> |
| `voice_settings` | String | <p>The Amazon Polly voice Amazon Lex uses for voice interaction with the
         user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot_locale
bot_locale = provider.lex_models.Bot_locale {
    bot_id = "value"  # <p>The identifier of the bot to create the locale for.</p>
    locale_id = "value"  # <p>The identifier of the language and locale that the bot will be used
         in. The string must match one of the supported locales. All of the
         intents, slot types, and slots used in the bot must have the same
         locale. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    nlu_intent_confidence_threshold = "value"  # <p>Determines the threshold where Amazon Lex will insert the
            <code>AMAZON.FallbackIntent</code>,
            <code>AMAZON.KendraSearchIntent</code>, or both when returning
         alternative intents. <code>AMAZON.FallbackIntent</code> and
            <code>AMAZON.KendraSearchIntent</code> are only inserted if they are
         configured for the bot.</p>
         <p>For example, suppose a bot is configured with the confidence
         threshold of 0.80 and the <code>AMAZON.FallbackIntent</code>. Amazon Lex
         returns three alternative intents with the following confidence scores:
         IntentA (0.70), IntentB (0.60), IntentC (0.50). The response from the
            <code>RecognizeText</code> operation would be:</p>
         <ul>
            <li>
               <p>AMAZON.FallbackIntent</p>
            </li>
            <li>
               <p>IntentA</p>
            </li>
            <li>
               <p>IntentB</p>
            </li>
            <li>
               <p>IntentC</p>
            </li>
         </ul>
    bot_version = "value"  # <p>The version of the bot to create the locale for. This can only be
         the draft version of the bot.</p>
}

# Access bot_locale outputs
bot_locale_id = bot_locale.id
bot_locale_description = bot_locale.description
bot_locale_locale_id = bot_locale.locale_id
bot_locale_nlu_intent_confidence_threshold = bot_locale.nlu_intent_confidence_threshold
bot_locale_intents_count = bot_locale.intents_count
bot_locale_failure_reasons = bot_locale.failure_reasons
bot_locale_creation_date_time = bot_locale.creation_date_time
bot_locale_generative_ai_settings = bot_locale.generative_ai_settings
bot_locale_slot_types_count = bot_locale.slot_types_count
bot_locale_last_build_submitted_date_time = bot_locale.last_build_submitted_date_time
bot_locale_locale_name = bot_locale.locale_name
bot_locale_bot_locale_status = bot_locale.bot_locale_status
bot_locale_recommended_actions = bot_locale.recommended_actions
bot_locale_bot_locale_history_events = bot_locale.bot_locale_history_events
bot_locale_bot_id = bot_locale.bot_id
bot_locale_bot_version = bot_locale.bot_version
bot_locale_last_updated_date_time = bot_locale.last_updated_date_time
bot_locale_voice_settings = bot_locale.voice_settings
```

---


### Export

Export resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_format` | String | ✅ | <p>The file format of the bot or bot locale definition files.</p> |
| `resource_specification` | String | ✅ | <p>Specifies the type of resource to export, either a bot or a bot
         locale. You can only specify one type of resource to export.</p> |
| `file_password` | String |  | <p>An password to use to encrypt the exported archive. Using a password
         is optional, but you should encrypt the archive to protect the data in
         transit between Amazon Lex and your local computer.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_format` | String | <p>The file format used in the files that describe the resource.
      </p> |
| `export_id` | String | <p>The unique identifier of the described export.</p> |
| `export_status` | String | <p>The status of the export. When the status is <code>Complete</code>
         the export archive file is available for download.</p> |
| `failure_reasons` | Vec<String> | <p>If the <code>exportStatus</code> is failed, contains one or more
         reasons why the export could not be completed.</p> |
| `download_url` | String | <p>A pre-signed S3 URL that points to the bot or bot locale archive.
         The URL is only available for 5 minutes after calling the
            <code>DescribeExport</code> operation.</p> |
| `resource_specification` | String | <p>The bot, bot ID, and optional locale ID of the exported bot or bot
         locale.</p> |
| `last_updated_date_time` | String | <p>The last date and time that the export was updated.</p> |
| `creation_date_time` | String | <p>The date and time that the export was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create export
export = provider.lex_models.Export {
    file_format = "value"  # <p>The file format of the bot or bot locale definition files.</p>
    resource_specification = "value"  # <p>Specifies the type of resource to export, either a bot or a bot
         locale. You can only specify one type of resource to export.</p>
}

# Access export outputs
export_id = export.id
export_file_format = export.file_format
export_export_id = export.export_id
export_export_status = export.export_status
export_failure_reasons = export.failure_reasons
export_download_url = export.download_url
export_resource_specification = export.resource_specification
export_last_updated_date_time = export.last_updated_date_time
export_creation_date_time = export.creation_date_time
```

---


### Test_set

TestSet resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_set_name` | String | ✅ | <p>The new test set name.</p> |
| `test_set_id` | String | ✅ | <p>The test set Id for which update test operation to be performed.</p> |
| `description` | String |  | <p>The new test set description.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_set_id` | String | <p>The test set Id for the test set response.</p> |
| `role_arn` | String | <p>The roleARN used for any operation in the test set to access 
      resources in the Amazon Web Services account.</p> |
| `test_set_name` | String | <p>The test set name of the test set.</p> |
| `status` | String | <p>The status of the test set.</p> |
| `description` | String | <p>The description of the test set.</p> |
| `modality` | String | <p>Indicates whether the test set is audio or text data.</p> |
| `num_turns` | i64 | <p>The total number of agent and user turn in the test set.</p> |
| `storage_location` | String | <p>The Amazon S3 storage location for the test set data.</p> |
| `last_updated_date_time` | String | <p>The date and time for the last update of the test set data.</p> |
| `creation_date_time` | String | <p>The creation date and time for the test set data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access test_set outputs
test_set_id = test_set.id
test_set_test_set_id = test_set.test_set_id
test_set_role_arn = test_set.role_arn
test_set_test_set_name = test_set.test_set_name
test_set_status = test_set.status
test_set_description = test_set.description
test_set_modality = test_set.modality
test_set_num_turns = test_set.num_turns
test_set_storage_location = test_set.storage_location
test_set_last_updated_date_time = test_set.last_updated_date_time
test_set_creation_date_time = test_set.creation_date_time
```

---


### Bot_alias

BotAlias resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conversation_log_settings` | String |  | <p>Specifies whether Amazon Lex logs text and audio for a conversation with
         the bot. When you enable conversation logs, text logs store text input,
         transcripts of audio input, and associated metadata in Amazon CloudWatch Logs. Audio
         logs store audio input in Amazon S3.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags to add to the bot alias. You can only add tags when
         you create an alias, you can't use the <code>UpdateBotAlias</code>
         operation to update the tags on a bot alias. To update tags, use the
            <code>TagResource</code> operation.</p> |
| `description` | String |  | <p>A description of the alias. Use this description to help identify
         the alias.</p> |
| `bot_version` | String |  | <p>The version of the bot that this alias points to. You can use the
            <a href="https://docs.aws.amazon.com/lexv2/latest/APIReference/API_UpdateBotAlias.html">UpdateBotAlias</a> operation to change the
         bot version associated with the alias.</p> |
| `sentiment_analysis_settings` | String |  |  |
| `bot_id` | String | ✅ | <p>The unique identifier of the bot that the alias applies to.</p> |
| `bot_alias_locale_settings` | HashMap<String, String> |  | <p>Maps configuration information to a specific locale. You can use
         this parameter to specify a specific Lambda function to run different
         functions in different locales.</p> |
| `bot_alias_name` | String | ✅ | <p>The alias to create. The name must be unique for the bot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_alias_name` | String | <p>The name of the bot alias.</p> |
| `description` | String | <p>The description of the bot alias.</p> |
| `bot_alias_locale_settings` | HashMap<String, String> | <p>The locale settings that are unique to the alias.</p> |
| `bot_alias_id` | String | <p>The identifier of the bot alias.</p> |
| `sentiment_analysis_settings` | String |  |
| `bot_alias_history_events` | Vec<String> | <p>A list of events that affect a bot alias. For example, an event is
         recorded when the version that the alias points to changes.</p> |
| `bot_alias_status` | String | <p>The current status of the alias. When the alias is
            <code>Available</code>, the alias is ready for use with your
         bot.</p> |
| `bot_id` | String | <p>The identifier of the bot associated with the bot alias.</p> |
| `creation_date_time` | String | <p>A timestamp of the date and time that the alias was created.</p> |
| `last_updated_date_time` | String | <p>A timestamp of the date and time that the alias was last
         updated.</p> |
| `parent_bot_networks` | Vec<String> | <p>A list of the networks to which the bot alias you described belongs.</p> |
| `bot_version` | String | <p>The version of the bot associated with the bot alias.</p> |
| `conversation_log_settings` | String | <p>Specifics of how Amazon Lex logs text and audio conversations with the
         bot associated with the alias.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot_alias
bot_alias = provider.lex_models.Bot_alias {
    bot_id = "value"  # <p>The unique identifier of the bot that the alias applies to.</p>
    bot_alias_name = "value"  # <p>The alias to create. The name must be unique for the bot.</p>
}

# Access bot_alias outputs
bot_alias_id = bot_alias.id
bot_alias_bot_alias_name = bot_alias.bot_alias_name
bot_alias_description = bot_alias.description
bot_alias_bot_alias_locale_settings = bot_alias.bot_alias_locale_settings
bot_alias_bot_alias_id = bot_alias.bot_alias_id
bot_alias_sentiment_analysis_settings = bot_alias.sentiment_analysis_settings
bot_alias_bot_alias_history_events = bot_alias.bot_alias_history_events
bot_alias_bot_alias_status = bot_alias.bot_alias_status
bot_alias_bot_id = bot_alias.bot_id
bot_alias_creation_date_time = bot_alias.creation_date_time
bot_alias_last_updated_date_time = bot_alias.last_updated_date_time
bot_alias_parent_bot_networks = bot_alias.parent_bot_networks
bot_alias_bot_version = bot_alias.bot_version
bot_alias_conversation_log_settings = bot_alias.conversation_log_settings
```

---


### Intent

Intent resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `q_in_connect_intent_configuration` | String |  | <p>Qinconnect intent configuration details for the create intent request.</p> |
| `input_contexts` | Vec<String> |  | <p>A list of contexts that must be active for this intent to be
         considered by Amazon Lex.</p>
         <p>When an intent has an input context list, Amazon Lex only considers using
         the intent in an interaction with the user when the specified contexts
         are included in the active context list for the session. If the
         contexts are not active, then Amazon Lex will not use the intent.</p>
         <p>A context can be automatically activated using the
            <code>outputContexts</code> property or it can be set at
         runtime.</p>
         <p> For example, if there are two intents with different input contexts
         that respond to the same utterances, only the intent with the active
         context will respond.</p>
         <p>An intent may have up to 5 input contexts. If an intent has multiple
         input contexts, all of the contexts must be active to consider the
         intent.</p> |
| `kendra_configuration` | String |  | <p>Configuration information required to use the
            <code>AMAZON.KendraSearchIntent</code> intent to connect to an Amazon Kendra
         index. The <code>AMAZON.KendraSearchIntent</code> intent is called when
         Amazon Lex can't determine another intent to invoke.</p> |
| `bot_version` | String | ✅ | <p>The version of the bot associated with this
         intent.</p> |
| `parent_intent_signature` | String |  | <p>A unique identifier for the built-in intent to base this intent
         on.</p> |
| `dialog_code_hook` | String |  | <p>Specifies that Amazon Lex invokes the alias Lambda function for each user
         input. You can invoke this Lambda function to personalize user
         interaction.</p>
         <p>For example, suppose that your bot determines that the user's name
         is John. You Lambda function might retrieve John's information from a
         backend database and prepopulate some of the values. For example, if
         you find that John is gluten intolerant, you might set the
         corresponding intent slot, <code>glutenIntolerant</code> to
            <code>true</code>. You might find John's phone number and set the
         corresponding session attribute.</p> |
| `intent_name` | String | ✅ | <p>The name of the intent. Intent names must be unique in the locale
         that contains the intent and cannot match the name of any built-in
         intent.</p> |
| `intent_confirmation_setting` | String |  | <p>Provides prompts that Amazon Lex sends to the user to confirm the
         completion of an intent. If the user answers "no," the settings contain
         a statement that is sent to the user to end the intent.</p> |
| `initial_response_setting` | String |  | <p>Configuration settings for the response that is sent to the user at
         the beginning of a conversation, before eliciting slot values.</p> |
| `description` | String |  | <p>A description of the intent. Use the description to help identify
         the intent in lists.</p> |
| `output_contexts` | Vec<String> |  | <p>A lists of contexts that the intent activates when it is
         fulfilled.</p>
         <p>You can use an output context to indicate the intents that Amazon Lex
         should consider for the next turn of the conversation with a customer. </p>
         <p>When you use the <code>outputContextsList</code> property, all of
         the contexts specified in the list are activated when the intent is
         fulfilled. You can set up to 10 output contexts. You can also set the
         number of conversation turns that the context should be active, or the
         length of time that the context should be active.</p> |
| `sample_utterances` | Vec<String> |  | <p>An array of strings that a user might say to signal the intent. For
         example, "I want a pizza", or "I want a {PizzaSize} pizza". </p>
         <p>In an utterance, slot names are enclosed in curly braces ("{", "}")
         to indicate where they should be displayed in the utterance shown to
         the user.. </p> |
| `fulfillment_code_hook` | String |  | <p>Specifies that Amazon Lex invokes the alias Lambda function when the
         intent is ready for fulfillment. You can invoke this function to
         complete the bot's transaction with the user.</p>
         <p>For example, in a pizza ordering bot, the Lambda function can look up
         the closest pizza restaurant to the customer's location and then place
         an order on the customer's behalf.</p> |
| `intent_closing_setting` | String |  | <p>Sets the response that Amazon Lex sends to the user when the intent is
         closed.</p> |
| `qn_a_intent_configuration` | String |  | <p>Specifies the configuration of the built-in <code>Amazon.QnAIntent</code>. The <code>AMAZON.QnAIntent</code> intent is called when
         Amazon Lex can't determine another intent to invoke. If you specify this field, you can't specify the <code>kendraConfiguration</code> field.</p> |
| `bot_id` | String | ✅ | <p>The identifier of the bot associated with this intent.</p> |
| `locale_id` | String | ✅ | <p>The identifier of the language and locale where this intent is used.
         All of the bots, slot types, and slots used by the intent must have the
         same locale. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date_time` | String | <p>A timestamp of the date and time that the intent was created.</p> |
| `intent_name` | String | <p>The name specified for the intent.</p> |
| `initial_response_setting` | String | <p>Configuration setting for a response sent to the user before Amazon Lex starts eliciting slots.</p> |
| `description` | String | <p>The description of the intent.</p> |
| `intent_confirmation_setting` | String | <p>Prompts that Amazon Lex sends to the user to confirm completion of an
         intent.</p> |
| `dialog_code_hook` | String | <p>The Lambda function called during each turn of a conversation with
         the intent.</p> |
| `last_updated_date_time` | String | <p>A timestamp of the date and time that the intent was last
         updated.</p> |
| `qn_a_intent_configuration` | String | <p>Details about the configuration of the built-in <code>Amazon.QnAIntent</code>.</p> |
| `q_in_connect_intent_configuration` | String | <p>Qinconnect intent configuration details for the describe intent response.</p> |
| `parent_intent_signature` | String | <p>The identifier of the built-in intent that this intent is derived
         from, if any.</p> |
| `intent_id` | String | <p>The unique identifier assigned to the intent when it was
         created.</p> |
| `sample_utterances` | Vec<String> | <p>User utterances that trigger this intent.</p> |
| `bot_id` | String | <p>The identifier of the bot associated with the intent.</p> |
| `fulfillment_code_hook` | String | <p>The Lambda function called when the intent is complete and ready for
         fulfillment.</p> |
| `intent_closing_setting` | String | <p>The response that Amazon Lex sends to when the intent is closed.</p> |
| `kendra_configuration` | String | <p>Configuration information required to use the
            <code>AMAZON.KendraSearchIntent</code> intent.</p> |
| `slot_priorities` | Vec<String> | <p>The list that determines the priority that slots should be elicited
         from the user.</p> |
| `input_contexts` | Vec<String> | <p>A list of contexts that must be active for the intent to be
         considered for sending to the user.</p> |
| `bot_version` | String | <p>The version of the bot associated with the intent.</p> |
| `locale_id` | String | <p>The language and locale specified for the intent.</p> |
| `output_contexts` | Vec<String> | <p>A list of contexts that are activated when the intent is
         fulfilled.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create intent
intent = provider.lex_models.Intent {
    bot_version = "value"  # <p>The version of the bot associated with this
         intent.</p>
    intent_name = "value"  # <p>The name of the intent. Intent names must be unique in the locale
         that contains the intent and cannot match the name of any built-in
         intent.</p>
    bot_id = "value"  # <p>The identifier of the bot associated with this intent.</p>
    locale_id = "value"  # <p>The identifier of the language and locale where this intent is used.
         All of the bots, slot types, and slots used by the intent must have the
         same locale. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
}

# Access intent outputs
intent_id = intent.id
intent_creation_date_time = intent.creation_date_time
intent_intent_name = intent.intent_name
intent_initial_response_setting = intent.initial_response_setting
intent_description = intent.description
intent_intent_confirmation_setting = intent.intent_confirmation_setting
intent_dialog_code_hook = intent.dialog_code_hook
intent_last_updated_date_time = intent.last_updated_date_time
intent_qn_a_intent_configuration = intent.qn_a_intent_configuration
intent_q_in_connect_intent_configuration = intent.q_in_connect_intent_configuration
intent_parent_intent_signature = intent.parent_intent_signature
intent_intent_id = intent.intent_id
intent_sample_utterances = intent.sample_utterances
intent_bot_id = intent.bot_id
intent_fulfillment_code_hook = intent.fulfillment_code_hook
intent_intent_closing_setting = intent.intent_closing_setting
intent_kendra_configuration = intent.kendra_configuration
intent_slot_priorities = intent.slot_priorities
intent_input_contexts = intent.input_contexts
intent_bot_version = intent.bot_version
intent_locale_id = intent.locale_id
intent_output_contexts = intent.output_contexts
```

---


### Bot

Bot resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_privacy` | String | ✅ | <p>Provides information on additional privacy protections Amazon Lex should
         use with the bot's data.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that has permission to
         access the bot.</p> |
| `idle_session_ttl_in_seconds` | i64 | ✅ | <p>The time, in seconds, that Amazon Lex should keep information about a
         user's conversation with the bot. </p>
         <p>A user interaction remains active for the amount of time specified.
         If no conversation occurs during this time, the session expires and
         Amazon Lex deletes any data provided before the timeout.</p>
         <p>You can specify between 60 (1 minute) and 86,400 (24 hours)
         seconds.</p> |
| `test_bot_alias_tags` | HashMap<String, String> |  | <p>A list of tags to add to the test alias for a bot. You can only add
         tags when you create a bot. You can't use the <code>UpdateAlias</code>
         operation to update tags. To update tags on the test alias, use the
            <code>TagResource</code> operation.</p> |
| `bot_type` | String |  | <p>The type of a bot to create.</p> |
| `bot_tags` | HashMap<String, String> |  | <p>A list of tags to add to the bot. You can only add tags when you
         create a bot. You can't use the <code>UpdateBot</code> operation to
         update tags. To update tags, use the <code>TagResource</code>
         operation.</p> |
| `bot_members` | Vec<String> |  | <p>The list of bot members in a network to be created.</p> |
| `description` | String |  | <p>A description of the bot. It appears in lists to help you identify a
         particular bot.</p> |
| `error_log_settings` | String |  | <p>Specifies the configuration for error logging during bot creation.</p> |
| `bot_name` | String | ✅ | <p>The name of the bot. The bot name must be unique in the account that
         creates the bot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_date_time` | String | <p>A timestamp of the date and time that the bot was last
         updated.</p> |
| `creation_date_time` | String | <p>A timestamp of the date and time that the bot was created.</p> |
| `bot_members` | Vec<String> | <p>The list of bots in the network that was described.</p> |
| `bot_id` | String | <p>The unique identifier of the bot.</p> |
| `bot_name` | String | <p>The name of the bot.</p> |
| `data_privacy` | String | <p>Settings for managing data privacy of the bot and its conversations
         with users.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role that has permission to
         access the bot.</p> |
| `bot_status` | String | <p>The current status of the bot. When the status is
            <code>Available</code> the bot is ready to be used in conversations
         with users.</p> |
| `failure_reasons` | Vec<String> | <p>If the <code>botStatus</code> is <code>Failed</code>, this contains
         a list of reasons that the bot couldn't be built.</p> |
| `description` | String | <p>The description of the bot. </p> |
| `bot_type` | String | <p>The type of the bot that was described.</p> |
| `error_log_settings` | String | <p>Contains the configuration for error logging that specifies where and how bot errors are recorded, including destinations like CloudWatch Logs.</p> |
| `idle_session_ttl_in_seconds` | i64 | <p>The maximum time in seconds that Amazon Lex retains the data gathered in
         a conversation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot
bot = provider.lex_models.Bot {
    data_privacy = "value"  # <p>Provides information on additional privacy protections Amazon Lex should
         use with the bot's data.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that has permission to
         access the bot.</p>
    idle_session_ttl_in_seconds = "value"  # <p>The time, in seconds, that Amazon Lex should keep information about a
         user's conversation with the bot. </p>
         <p>A user interaction remains active for the amount of time specified.
         If no conversation occurs during this time, the session expires and
         Amazon Lex deletes any data provided before the timeout.</p>
         <p>You can specify between 60 (1 minute) and 86,400 (24 hours)
         seconds.</p>
    bot_name = "value"  # <p>The name of the bot. The bot name must be unique in the account that
         creates the bot.</p>
}

# Access bot outputs
bot_id = bot.id
bot_last_updated_date_time = bot.last_updated_date_time
bot_creation_date_time = bot.creation_date_time
bot_bot_members = bot.bot_members
bot_bot_id = bot.bot_id
bot_bot_name = bot.bot_name
bot_data_privacy = bot.data_privacy
bot_role_arn = bot.role_arn
bot_bot_status = bot.bot_status
bot_failure_reasons = bot.failure_reasons
bot_description = bot.description
bot_bot_type = bot.bot_type
bot_error_log_settings = bot.error_log_settings
bot_idle_session_ttl_in_seconds = bot.idle_session_ttl_in_seconds
```

---


### Bot_replica

BotReplica resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replica_region` | String | ✅ | <p>The request for the secondary region that will be used in the replication of the source bot.</p> |
| `bot_id` | String | ✅ | <p>The request for the unique bot ID of the source bot to be replicated in the secondary region.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_replica_status` | String | <p>The operational status of the replicated bot being monitored.</p> |
| `creation_date_time` | String | <p>The creation date and time of the replicated bot being monitored.</p> |
| `source_region` | String | <p>The source region of the replicated bot being monitored.</p> |
| `bot_id` | String | <p>The unique bot ID of the replicated bot being monitored.</p> |
| `failure_reasons` | Vec<String> | <p>The failure reasons the bot being monitored failed to replicate.</p> |
| `replica_region` | String | <p>The region of the replicated bot being monitored.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bot_replica
bot_replica = provider.lex_models.Bot_replica {
    replica_region = "value"  # <p>The request for the secondary region that will be used in the replication of the source bot.</p>
    bot_id = "value"  # <p>The request for the unique bot ID of the source bot to be replicated in the secondary region.</p>
}

# Access bot_replica outputs
bot_replica_id = bot_replica.id
bot_replica_bot_replica_status = bot_replica.bot_replica_status
bot_replica_creation_date_time = bot_replica.creation_date_time
bot_replica_source_region = bot_replica.source_region
bot_replica_bot_id = bot_replica.bot_id
bot_replica_failure_reasons = bot_replica.failure_reasons
bot_replica_replica_region = bot_replica.replica_region
```

---


### Import

Import resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_id` | String | <p>The unique identifier of the described import.</p> |
| `imported_resource_name` | String | <p>The name of the imported resource.</p> |
| `resource_specification` | String | <p>The specifications of the imported bot, bot locale, or custom
         vocabulary.</p> |
| `merge_strategy` | String | <p>The strategy used when there was a name conflict between the
         imported resource and an existing resource. When the merge strategy is
            <code>FailOnConflict</code> existing resources are not overwritten
         and the import fails.</p> |
| `creation_date_time` | String | <p>The date and time that the import was created.</p> |
| `last_updated_date_time` | String | <p>The date and time that the import was last updated.</p> |
| `imported_resource_id` | String | <p>The unique identifier that Amazon Lex assigned to the resource created by
         the import.</p> |
| `import_status` | String | <p>The status of the import process. When the status is
            <code>Completed</code> the resource is imported and ready for
         use.</p> |
| `failure_reasons` | Vec<String> | <p>If the <code>importStatus</code> field is <code>Failed</code>, this
         provides one or more reasons for the failure.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import outputs
import_id = import.id
import_import_id = import.import_id
import_imported_resource_name = import.imported_resource_name
import_resource_specification = import.resource_specification
import_merge_strategy = import.merge_strategy
import_creation_date_time = import.creation_date_time
import_last_updated_date_time = import.last_updated_date_time
import_imported_resource_id = import.imported_resource_id
import_import_status = import.import_status
import_failure_reasons = import.failure_reasons
```

---


### Upload_url

UploadUrl resource

**Operations**: ✅ Create

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

# Create upload_url
upload_url = provider.lex_models.Upload_url {
}

```

---


### Resource_policy_statement

ResourcePolicyStatement resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | Vec<String> | ✅ | <p>The Amazon Lex action that this policy either allows or denies. The
         action must apply to the resource type of the specified ARN. For more
         information, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonlexv2.html">
            Actions, resources, and condition keys for Amazon Lex V2</a>.</p> |
| `condition` | HashMap<String, HashMap<String, String>> |  | <p>Specifies a condition when the policy is in effect. If the principal
         of the policy is a service principal, you must provide two condition
         blocks, one with a SourceAccount global condition key and one with a
         SourceArn global condition key.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html">IAM JSON policy elements: Condition </a>.</p> |
| `expected_revision_id` | String |  | <p>The identifier of the revision of the policy to edit. If this
         revision ID doesn't match the current revision ID, Amazon Lex throws an
         exception.</p>
         <p>If you don't specify a revision, Amazon Lex overwrites the contents of
         the policy with the new values.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the bot or bot alias that the
         resource policy is attached to.</p> |
| `effect` | String | ✅ | <p>Determines whether the statement allows or denies access to the
         resource.</p> |
| `principal` | Vec<String> | ✅ | <p>An IAM principal, such as an IAM user, IAM role, 
         or Amazon Web Services services
         that is allowed or denied access to a resource. For more information,
         see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html">Amazon Web Services JSON policy elements: Principal</a>.</p> |
| `statement_id` | String | ✅ | <p>The name of the statement. The ID is the same as the
            <code>Sid</code> IAM property. The statement name must be unique
         within the policy. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_sid.html">IAM
            JSON policy elements: Sid</a>. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy_statement
resource_policy_statement = provider.lex_models.Resource_policy_statement {
    action = "value"  # <p>The Amazon Lex action that this policy either allows or denies. The
         action must apply to the resource type of the specified ARN. For more
         information, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonlexv2.html">
            Actions, resources, and condition keys for Amazon Lex V2</a>.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the bot or bot alias that the
         resource policy is attached to.</p>
    effect = "value"  # <p>Determines whether the statement allows or denies access to the
         resource.</p>
    principal = "value"  # <p>An IAM principal, such as an IAM user, IAM role, 
         or Amazon Web Services services
         that is allowed or denied access to a resource. For more information,
         see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html">Amazon Web Services JSON policy elements: Principal</a>.</p>
    statement_id = "value"  # <p>The name of the statement. The ID is the same as the
            <code>Sid</code> IAM property. The statement name must be unique
         within the policy. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_sid.html">IAM
            JSON policy elements: Sid</a>. </p>
}

```

---


### Test_set_discrepancy_report

TestSetDiscrepancyReport resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target` | String | ✅ | <p>The target bot for the test set discrepancy report.</p> |
| `test_set_id` | String | ✅ | <p>The test set Id for the test set discrepancy report.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date_time` | String | <p>The time and date of creation for the test set discrepancy report.</p> |
| `test_set_discrepancy_report_status` | String | <p>The status for the test set discrepancy report.</p> |
| `failure_reasons` | Vec<String> | <p>The failure report for the test set discrepancy report generation action.</p> |
| `test_set_id` | String | <p>The test set Id for the test set discrepancy report.</p> |
| `target` | String | <p>The target bot location for the test set discrepancy report.</p> |
| `last_updated_data_time` | String | <p>The date and time of the last update for the test set discrepancy report.</p> |
| `test_set_discrepancy_top_errors` | String | <p>The top 200 error results from the test set discrepancy report.</p> |
| `test_set_discrepancy_raw_output_url` | String | <p>Pre-signed Amazon S3 URL to download the test set discrepancy report.</p> |
| `test_set_discrepancy_report_id` | String | <p>The unique identifier of the test set discrepancy report to describe.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create test_set_discrepancy_report
test_set_discrepancy_report = provider.lex_models.Test_set_discrepancy_report {
    target = "value"  # <p>The target bot for the test set discrepancy report.</p>
    test_set_id = "value"  # <p>The test set Id for the test set discrepancy report.</p>
}

# Access test_set_discrepancy_report outputs
test_set_discrepancy_report_id = test_set_discrepancy_report.id
test_set_discrepancy_report_creation_date_time = test_set_discrepancy_report.creation_date_time
test_set_discrepancy_report_test_set_discrepancy_report_status = test_set_discrepancy_report.test_set_discrepancy_report_status
test_set_discrepancy_report_failure_reasons = test_set_discrepancy_report.failure_reasons
test_set_discrepancy_report_test_set_id = test_set_discrepancy_report.test_set_id
test_set_discrepancy_report_target = test_set_discrepancy_report.target
test_set_discrepancy_report_last_updated_data_time = test_set_discrepancy_report.last_updated_data_time
test_set_discrepancy_report_test_set_discrepancy_top_errors = test_set_discrepancy_report.test_set_discrepancy_top_errors
test_set_discrepancy_report_test_set_discrepancy_raw_output_url = test_set_discrepancy_report.test_set_discrepancy_raw_output_url
test_set_discrepancy_report_test_set_discrepancy_report_id = test_set_discrepancy_report.test_set_discrepancy_report_id
```

---


### Test_execution

TestExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_mode` | String | <p>Indicates whether we use streaming or non-streaming APIs are used for 
      the test set execution. For streaming, <code>StartConversation</code> 
      Amazon Lex Runtime API is used. Whereas for non-streaming, <code>RecognizeUtterance</code> 
      and <code>RecognizeText</code> Amazon Lex Runtime API is used.</p> |
| `target` | String | <p>The target bot for the test set execution details.</p> |
| `test_execution_modality` | String | <p>Indicates whether test set is audio or text.</p> |
| `test_set_id` | String | <p>The test set Id for the test set execution.</p> |
| `test_execution_id` | String | <p>The execution Id for the test set execution.</p> |
| `failure_reasons` | Vec<String> | <p>Reasons for the failure of the test set execution.</p> |
| `creation_date_time` | String | <p>The execution creation date and time for the test set execution.</p> |
| `test_execution_status` | String | <p>The test execution status for the test execution.</p> |
| `last_updated_date_time` | String | <p>The date and time of the last update for the execution.</p> |
| `test_set_name` | String | <p>The test set name of the test set execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access test_execution outputs
test_execution_id = test_execution.id
test_execution_api_mode = test_execution.api_mode
test_execution_target = test_execution.target
test_execution_test_execution_modality = test_execution.test_execution_modality
test_execution_test_set_id = test_execution.test_set_id
test_execution_test_execution_id = test_execution.test_execution_id
test_execution_failure_reasons = test_execution.failure_reasons
test_execution_creation_date_time = test_execution.creation_date_time
test_execution_test_execution_status = test_execution.test_execution_status
test_execution_last_updated_date_time = test_execution.last_updated_date_time
test_execution_test_set_name = test_execution.test_set_name
```

---


### Test_execution_artifacts_url

TestExecutionArtifactsUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_execution_id` | String | <p>The unique identifier of the completed test execution.</p> |
| `download_artifacts_url` | String | <p>The pre-signed Amazon S3 URL to download completed test execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access test_execution_artifacts_url outputs
test_execution_artifacts_url_id = test_execution_artifacts_url.id
test_execution_artifacts_url_test_execution_id = test_execution_artifacts_url.test_execution_id
test_execution_artifacts_url_download_artifacts_url = test_execution_artifacts_url.download_artifacts_url
```

---


### Bot_recommendation

BotRecommendation resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `locale_id` | String | ✅ | <p>The identifier of the language and locale of the bot recommendation
         to update. The string must match one of the supported locales. For more
         information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>
         </p> |
| `bot_version` | String | ✅ | <p>The version of the bot containing the bot recommendation to be
         updated.</p> |
| `bot_id` | String | ✅ | <p>The unique identifier of the bot containing the bot recommendation
         to be updated.</p> |
| `bot_recommendation_id` | String | ✅ | <p>The unique identifier of the bot recommendation to be
         updated.</p> |
| `encryption_setting` | String | ✅ | <p>The object representing the passwords that will be used to encrypt
         the data related to the bot recommendation results, as well as the KMS
         key ARN used to encrypt the associated metadata.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_version` | String | <p>The version of the bot associated with the bot
         recommendation.</p> |
| `bot_id` | String | <p>The identifier of the bot associated with the bot
         recommendation.</p> |
| `last_updated_date_time` | String | <p>The date and time that the bot recommendation was last
         updated.</p> |
| `bot_recommendation_status` | String | <p>The status of the bot recommendation. If the status is Failed, then
         the reasons for the failure are listed in the failureReasons field.
      </p> |
| `bot_recommendation_id` | String | <p>The identifier of the bot recommendation being described.</p> |
| `failure_reasons` | Vec<String> | <p>If botRecommendationStatus is Failed, Amazon Lex explains why.</p> |
| `encryption_setting` | String | <p>The object representing the passwords that were used to encrypt the
         data related to the bot recommendation results, as well as the KMS key
         ARN used to encrypt the associated metadata.</p> |
| `locale_id` | String | <p>The identifier of the language and locale of the bot recommendation
         to describe.</p> |
| `bot_recommendation_results` | String | <p>The object representing the URL of the bot definition, the URL of
         the associated transcript and a statistical summary of the bot
         recommendation results.</p> |
| `creation_date_time` | String | <p>The date and time that the bot recommendation was created.</p> |
| `transcript_source_setting` | String | <p>The object representing the Amazon S3 bucket containing the transcript,
         as well as the associated metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bot_recommendation outputs
bot_recommendation_id = bot_recommendation.id
bot_recommendation_bot_version = bot_recommendation.bot_version
bot_recommendation_bot_id = bot_recommendation.bot_id
bot_recommendation_last_updated_date_time = bot_recommendation.last_updated_date_time
bot_recommendation_bot_recommendation_status = bot_recommendation.bot_recommendation_status
bot_recommendation_bot_recommendation_id = bot_recommendation.bot_recommendation_id
bot_recommendation_failure_reasons = bot_recommendation.failure_reasons
bot_recommendation_encryption_setting = bot_recommendation.encryption_setting
bot_recommendation_locale_id = bot_recommendation.locale_id
bot_recommendation_bot_recommendation_results = bot_recommendation.bot_recommendation_results
bot_recommendation_creation_date_time = bot_recommendation.creation_date_time
bot_recommendation_transcript_source_setting = bot_recommendation.transcript_source_setting
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple bot_resource_generation resources
bot_resource_generation_0 = provider.lex_models.Bot_resource_generation {
}
bot_resource_generation_1 = provider.lex_models.Bot_resource_generation {
}
bot_resource_generation_2 = provider.lex_models.Bot_resource_generation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    bot_resource_generation = provider.lex_models.Bot_resource_generation {
    }
```

---

## Related Documentation

- [AWS Lex_models Documentation](https://docs.aws.amazon.com/lex_models/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
