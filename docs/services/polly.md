# Polly Service



**Resources**: 3

---

## Overview

The polly service provides access to 3 resource types:

- [Lexicon](#lexicon) [CRD]
- [Voices](#voices) [R]
- [Speech_synthesis_task](#speech_synthesis_task) [R]

---

## Resources


### Lexicon

Lexicon resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | String | ✅ | <p>Content of the PLS lexicon as string data.</p> |
| `name` | String | ✅ | <p>Name of the lexicon. The name must follow the regular express
      format [0-9A-Za-z]{1,20}. That is, the name is a case-sensitive
      alphanumeric string up to 20 characters long. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lexicon` | String | <p>Lexicon object that provides name and the string content of the
      lexicon. </p> |
| `lexicon_attributes` | String | <p>Metadata of the lexicon, including phonetic alphabetic used,
      language code, lexicon ARN, number of lexemes defined in the lexicon, and
      size of lexicon in bytes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lexicon
lexicon = provider.polly.Lexicon {
    content = "value"  # <p>Content of the PLS lexicon as string data.</p>
    name = "value"  # <p>Name of the lexicon. The name must follow the regular express
      format [0-9A-Za-z]{1,20}. That is, the name is a case-sensitive
      alphanumeric string up to 20 characters long. </p>
}

# Access lexicon outputs
lexicon_id = lexicon.id
lexicon_lexicon = lexicon.lexicon
lexicon_lexicon_attributes = lexicon.lexicon_attributes
```

---


### Voices

Voices resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voices` | Vec<String> | <p>A list of voices with their properties.</p> |
| `next_token` | String | <p>The pagination token to use in the next request to continue the
      listing of voices. <code>NextToken</code> is returned only if the response
      is truncated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access voices outputs
voices_id = voices.id
voices_voices = voices.voices
voices_next_token = voices.next_token
```

---


### Speech_synthesis_task

SpeechSynthesisTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `synthesis_task` | String | <p>SynthesisTask object that provides information from the requested
      task, including output format, creation time, task status, and so
      on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access speech_synthesis_task outputs
speech_synthesis_task_id = speech_synthesis_task.id
speech_synthesis_task_synthesis_task = speech_synthesis_task.synthesis_task
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple lexicon resources
lexicon_0 = provider.polly.Lexicon {
    content = "value-0"
    name = "value-0"
}
lexicon_1 = provider.polly.Lexicon {
    content = "value-1"
    name = "value-1"
}
lexicon_2 = provider.polly.Lexicon {
    content = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    lexicon = provider.polly.Lexicon {
        content = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Polly Documentation](https://docs.aws.amazon.com/polly/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
