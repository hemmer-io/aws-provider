# Codeguruprofiler Service



**Resources**: 1

---

## Overview

The codeguruprofiler service provides access to 1 resource type:

- [Findings_report_account_summary](#findings_report_account_summary) [R]

---

## Resources


### Findings_report_account_summary

FindingsReportAccountSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_summaries` | Vec<String> | <p>The return list of 
            <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_FindingsReportSummary.html">
               <code>FindingsReportSummary</code>
            </a> 
            objects taht contain summaries of analysis results for all profiling groups in your AWS account.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future <code>GetFindingsReportAccountSummary</code> request.
         When the results of a <code>GetFindingsReportAccountSummary</code> request exceed <code>maxResults</code>, this
         value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more
         results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings_report_account_summary outputs
findings_report_account_summary_id = findings_report_account_summary.id
findings_report_account_summary_report_summaries = findings_report_account_summary.report_summaries
findings_report_account_summary_next_token = findings_report_account_summary.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple findings_report_account_summary resources
findings_report_account_summary_0 = provider.codeguruprofiler.Findings_report_account_summary {
}
findings_report_account_summary_1 = provider.codeguruprofiler.Findings_report_account_summary {
}
findings_report_account_summary_2 = provider.codeguruprofiler.Findings_report_account_summary {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    findings_report_account_summary = provider.codeguruprofiler.Findings_report_account_summary {
    }
```

---

## Related Documentation

- [AWS Codeguruprofiler Documentation](https://docs.aws.amazon.com/codeguruprofiler/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
