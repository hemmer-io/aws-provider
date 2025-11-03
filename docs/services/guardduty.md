# Guardduty Service



**Resources**: 25

---

## Overview

The guardduty service provides access to 25 resource types:

- [Usage_statistics](#usage_statistics) [R]
- [Findings_feedback](#findings_feedback) [U]
- [Invitations](#invitations) [D]
- [Publishing_destination](#publishing_destination) [CRUD]
- [Filter](#filter) [CRUD]
- [Detector](#detector) [CRUD]
- [Trusted_entity_set](#trusted_entity_set) [CRUD]
- [Malware_scans](#malware_scans) [R]
- [Invitations_count](#invitations_count) [R]
- [Organization_configuration](#organization_configuration) [RU]
- [Ip_set](#ip_set) [CRUD]
- [Master_account](#master_account) [R]
- [Findings_statistics](#findings_statistics) [R]
- [Members](#members) [CRD]
- [Sample_findings](#sample_findings) [C]
- [Organization_statistics](#organization_statistics) [R]
- [Threat_intel_set](#threat_intel_set) [CRUD]
- [Coverage_statistics](#coverage_statistics) [R]
- [Malware_scan_settings](#malware_scan_settings) [RU]
- [Threat_entity_set](#threat_entity_set) [CRUD]
- [Administrator_account](#administrator_account) [R]
- [Remaining_free_trial_days](#remaining_free_trial_days) [R]
- [Member_detectors](#member_detectors) [RU]
- [Findings](#findings) [R]
- [Malware_protection_plan](#malware_protection_plan) [CRUD]

---

## Resources


### Usage_statistics

UsageStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `usage_statistics` | String | <p>The usage statistics object. If a UsageStatisticType was provided, the objects
      representing other types will be null.</p> |
| `next_token` | String | <p>The pagination parameter to be used on the next list operation to retrieve more
      items.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_statistics outputs
usage_statistics_id = usage_statistics.id
usage_statistics_usage_statistics = usage_statistics.usage_statistics
usage_statistics_next_token = usage_statistics.next_token
```

---


### Findings_feedback

FindingsFeedback resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comments` | String |  | <p>Additional feedback about the GuardDuty findings.</p> |
| `finding_ids` | Vec<String> | ✅ | <p>The IDs of the findings that you want to mark as useful or not useful.</p> |
| `feedback` | String | ✅ | <p>The feedback for the finding.</p> |
| `detector_id` | String | ✅ | <p>The ID of the detector that is associated with the findings for which you want to update 
      the feedback.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |



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


### Invitations

Invitations resource

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


### Publishing_destination

PublishingDestination resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `detector_id` | String | ✅ | <p>The ID of the GuardDuty detector associated with the publishing destination.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `destination_type` | String | ✅ | <p>The type of resource for the publishing destination. Currently only Amazon S3 buckets are
      supported.</p> |
| `destination_properties` | String | ✅ | <p>The properties of the publishing destination, including the ARNs for the destination and
      the KMS key used for encryption.</p> |
| `client_token` | String |  | <p>The idempotency token for the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publishing_failure_start_timestamp` | i64 | <p>The time, in epoch millisecond format, at which GuardDuty was first unable to publish
      findings to the destination.</p> |
| `destination_type` | String | <p>The type of publishing destination. Currently, only Amazon S3 buckets are
      supported.</p> |
| `destination_id` | String | <p>The ID of the publishing destination.</p> |
| `status` | String | <p>The status of the publishing destination.</p> |
| `destination_properties` | String | <p>A <code>DestinationProperties</code> object that includes the <code>DestinationArn</code>
      and <code>KmsKeyArn</code> of the publishing destination.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create publishing_destination
publishing_destination = provider.guardduty.Publishing_destination {
    detector_id = "value"  # <p>The ID of the GuardDuty detector associated with the publishing destination.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    destination_type = "value"  # <p>The type of resource for the publishing destination. Currently only Amazon S3 buckets are
      supported.</p>
    destination_properties = "value"  # <p>The properties of the publishing destination, including the ARNs for the destination and
      the KMS key used for encryption.</p>
}

# Access publishing_destination outputs
publishing_destination_id = publishing_destination.id
publishing_destination_publishing_failure_start_timestamp = publishing_destination.publishing_failure_start_timestamp
publishing_destination_destination_type = publishing_destination.destination_type
publishing_destination_destination_id = publishing_destination.destination_id
publishing_destination_status = publishing_destination.status
publishing_destination_destination_properties = publishing_destination.destination_properties
```

---


### Filter

Filter resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rank` | i64 |  | <p>Specifies the position of the filter in the list of current filters. Also specifies the
      order in which this filter is applied to the findings.</p> |
| `finding_criteria` | String | ✅ | <p>Represents the criteria to be used in the filter for querying findings.</p>
         <p>You can only use the following attributes to query findings:</p>
         <ul>
            <li>
               <p>accountId</p>
            </li>
            <li>
               <p>id</p>
            </li>
            <li>
               <p>region</p>
            </li>
            <li>
               <p>severity</p>
               <p>To filter on the basis of severity, the API and CLI use the following input list for
          the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_FindingCriteria.html">FindingCriteria</a>
          condition:</p>
               <ul>
                  <li>
                     <p>
                        <b>Low</b>: <code>["1", "2", "3"]</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <b>Medium</b>: <code>["4", "5", "6"]</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <b>High</b>: <code>["7", "8"]</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <b>Critical</b>: <code>["9", "10"]</code>
                     </p>
                  </li>
               </ul>
               <p>For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_findings-severity.html">Findings severity levels</a>
          in the <i>Amazon GuardDuty User Guide</i>.</p>
            </li>
            <li>
               <p>type</p>
            </li>
            <li>
               <p>updatedAt</p>
               <p>Type: ISO 8601 string format: YYYY-MM-DDTHH:MM:SS.SSSZ or YYYY-MM-DDTHH:MM:SSZ
          depending on whether the value contains milliseconds.</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.accessKeyId</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.principalId</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.userName</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.userType</p>
            </li>
            <li>
               <p>resource.instanceDetails.iamInstanceProfile.id</p>
            </li>
            <li>
               <p>resource.instanceDetails.imageId</p>
            </li>
            <li>
               <p>resource.instanceDetails.instanceId</p>
            </li>
            <li>
               <p>resource.instanceDetails.tags.key</p>
            </li>
            <li>
               <p>resource.instanceDetails.tags.value</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.ipv6Addresses</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.privateIpAddresses.privateIpAddress</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.publicDnsName</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.publicIp</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.securityGroups.groupId</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.securityGroups.groupName</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.subnetId</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.vpcId</p>
            </li>
            <li>
               <p>resource.instanceDetails.outpostArn</p>
            </li>
            <li>
               <p>resource.resourceType</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.publicAccess.effectivePermissions</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.name</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.tags.key</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.tags.value</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.type</p>
            </li>
            <li>
               <p>service.action.actionType</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.api</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.callerType</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.errorCode</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.city.cityName</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.country.countryName</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.organization.asn</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.serviceName</p>
            </li>
            <li>
               <p>service.action.dnsRequestAction.domain</p>
            </li>
            <li>
               <p>service.action.dnsRequestAction.domainWithSuffix</p>
            </li>
            <li>
               <p>service.action.dnsRequestAction.vpcOwnerAccountId</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.blocked</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.connectionDirection</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.localPortDetails.port</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.protocol</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.city.cityName</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.country.countryName</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.organization.asn</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remotePortDetails.port</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteAccountDetails.affiliated</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.remoteIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.remoteIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.namespace</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.remoteIpDetails.organization.asn</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.requestUri</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.statusCode</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.localIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.localIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.protocol</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.serviceName</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteAccountDetails.accountId</p>
            </li>
            <li>
               <p>service.additionalInfo.threatListName</p>
            </li>
            <li>
               <p>service.resourceRole</p>
            </li>
            <li>
               <p>resource.eksClusterDetails.name</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.name</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.namespace</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesUserDetails.username</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.containers.image</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.containers.imagePrefix</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanId</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames.name</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames.severity</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames.filePaths.hash</p>
            </li>
            <li>
               <p>resource.ecsClusterDetails.name</p>
            </li>
            <li>
               <p>resource.ecsClusterDetails.taskDetails.containers.image</p>
            </li>
            <li>
               <p>resource.ecsClusterDetails.taskDetails.definitionArn</p>
            </li>
            <li>
               <p>resource.containerDetails.image</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.dbInstanceIdentifier</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.dbClusterIdentifier</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.engine</p>
            </li>
            <li>
               <p>resource.rdsDbUserDetails.user</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.tags.key</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.tags.value</p>
            </li>
            <li>
               <p>service.runtimeDetails.process.executableSha256</p>
            </li>
            <li>
               <p>service.runtimeDetails.process.name</p>
            </li>
            <li>
               <p>service.runtimeDetails.process.executablePath</p>
            </li>
            <li>
               <p>resource.lambdaDetails.functionName</p>
            </li>
            <li>
               <p>resource.lambdaDetails.functionArn</p>
            </li>
            <li>
               <p>resource.lambdaDetails.tags.key</p>
            </li>
            <li>
               <p>resource.lambdaDetails.tags.value</p>
            </li>
         </ul> |
| `name` | String | ✅ | <p>The name of the filter. Valid characters include period (.), underscore (_), dash (-), and
      alphanumeric characters. A whitespace is considered to be an invalid character.</p> |
| `action` | String |  | <p>Specifies the action that is to be applied to the findings that match the filter.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be added to a new filter resource.</p> |
| `client_token` | String |  | <p>The idempotency token for the create request.</p> |
| `detector_id` | String | ✅ | <p>The detector ID associated with the GuardDuty account for which you want to create a filter.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `description` | String |  | <p>The description of the filter. Valid characters include alphanumeric characters, and
      special characters such as hyphen, period, colon, underscore, parentheses (<code>{ }</code>,
        <code>[ ]</code>, and <code>( )</code>), forward slash, horizontal tab, vertical tab,
      newline, form feed, return, and whitespace.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rank` | i64 | <p>Specifies the position of the filter in the list of current filters. Also specifies the
      order in which this filter is applied to the findings.</p> |
| `tags` | HashMap<String, String> | <p>The tags of the filter resource.</p> |
| `name` | String | <p>The name of the filter.</p> |
| `action` | String | <p>Specifies the action that is to be applied to the findings that match the filter.</p> |
| `finding_criteria` | String | <p>Represents the criteria to be used in the filter for querying findings.</p> |
| `description` | String | <p>The description of the filter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create filter
filter = provider.guardduty.Filter {
    finding_criteria = "value"  # <p>Represents the criteria to be used in the filter for querying findings.</p>
         <p>You can only use the following attributes to query findings:</p>
         <ul>
            <li>
               <p>accountId</p>
            </li>
            <li>
               <p>id</p>
            </li>
            <li>
               <p>region</p>
            </li>
            <li>
               <p>severity</p>
               <p>To filter on the basis of severity, the API and CLI use the following input list for
          the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_FindingCriteria.html">FindingCriteria</a>
          condition:</p>
               <ul>
                  <li>
                     <p>
                        <b>Low</b>: <code>["1", "2", "3"]</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <b>Medium</b>: <code>["4", "5", "6"]</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <b>High</b>: <code>["7", "8"]</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <b>Critical</b>: <code>["9", "10"]</code>
                     </p>
                  </li>
               </ul>
               <p>For more information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_findings-severity.html">Findings severity levels</a>
          in the <i>Amazon GuardDuty User Guide</i>.</p>
            </li>
            <li>
               <p>type</p>
            </li>
            <li>
               <p>updatedAt</p>
               <p>Type: ISO 8601 string format: YYYY-MM-DDTHH:MM:SS.SSSZ or YYYY-MM-DDTHH:MM:SSZ
          depending on whether the value contains milliseconds.</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.accessKeyId</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.principalId</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.userName</p>
            </li>
            <li>
               <p>resource.accessKeyDetails.userType</p>
            </li>
            <li>
               <p>resource.instanceDetails.iamInstanceProfile.id</p>
            </li>
            <li>
               <p>resource.instanceDetails.imageId</p>
            </li>
            <li>
               <p>resource.instanceDetails.instanceId</p>
            </li>
            <li>
               <p>resource.instanceDetails.tags.key</p>
            </li>
            <li>
               <p>resource.instanceDetails.tags.value</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.ipv6Addresses</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.privateIpAddresses.privateIpAddress</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.publicDnsName</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.publicIp</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.securityGroups.groupId</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.securityGroups.groupName</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.subnetId</p>
            </li>
            <li>
               <p>resource.instanceDetails.networkInterfaces.vpcId</p>
            </li>
            <li>
               <p>resource.instanceDetails.outpostArn</p>
            </li>
            <li>
               <p>resource.resourceType</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.publicAccess.effectivePermissions</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.name</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.tags.key</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.tags.value</p>
            </li>
            <li>
               <p>resource.s3BucketDetails.type</p>
            </li>
            <li>
               <p>service.action.actionType</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.api</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.callerType</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.errorCode</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.city.cityName</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.country.countryName</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.organization.asn</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.serviceName</p>
            </li>
            <li>
               <p>service.action.dnsRequestAction.domain</p>
            </li>
            <li>
               <p>service.action.dnsRequestAction.domainWithSuffix</p>
            </li>
            <li>
               <p>service.action.dnsRequestAction.vpcOwnerAccountId</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.blocked</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.connectionDirection</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.localPortDetails.port</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.protocol</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.city.cityName</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.country.countryName</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.organization.asn</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.remotePortDetails.port</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteAccountDetails.affiliated</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.remoteIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.remoteIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.namespace</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.remoteIpDetails.organization.asn</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.requestUri</p>
            </li>
            <li>
               <p>service.action.kubernetesApiCallAction.statusCode</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.localIpDetails.ipAddressV4</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.localIpDetails.ipAddressV6</p>
            </li>
            <li>
               <p>service.action.networkConnectionAction.protocol</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.serviceName</p>
            </li>
            <li>
               <p>service.action.awsApiCallAction.remoteAccountDetails.accountId</p>
            </li>
            <li>
               <p>service.additionalInfo.threatListName</p>
            </li>
            <li>
               <p>service.resourceRole</p>
            </li>
            <li>
               <p>resource.eksClusterDetails.name</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.name</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.namespace</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesUserDetails.username</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.containers.image</p>
            </li>
            <li>
               <p>resource.kubernetesDetails.kubernetesWorkloadDetails.containers.imagePrefix</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanId</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames.name</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames.severity</p>
            </li>
            <li>
               <p>service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames.filePaths.hash</p>
            </li>
            <li>
               <p>resource.ecsClusterDetails.name</p>
            </li>
            <li>
               <p>resource.ecsClusterDetails.taskDetails.containers.image</p>
            </li>
            <li>
               <p>resource.ecsClusterDetails.taskDetails.definitionArn</p>
            </li>
            <li>
               <p>resource.containerDetails.image</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.dbInstanceIdentifier</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.dbClusterIdentifier</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.engine</p>
            </li>
            <li>
               <p>resource.rdsDbUserDetails.user</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.tags.key</p>
            </li>
            <li>
               <p>resource.rdsDbInstanceDetails.tags.value</p>
            </li>
            <li>
               <p>service.runtimeDetails.process.executableSha256</p>
            </li>
            <li>
               <p>service.runtimeDetails.process.name</p>
            </li>
            <li>
               <p>service.runtimeDetails.process.executablePath</p>
            </li>
            <li>
               <p>resource.lambdaDetails.functionName</p>
            </li>
            <li>
               <p>resource.lambdaDetails.functionArn</p>
            </li>
            <li>
               <p>resource.lambdaDetails.tags.key</p>
            </li>
            <li>
               <p>resource.lambdaDetails.tags.value</p>
            </li>
         </ul>
    name = "value"  # <p>The name of the filter. Valid characters include period (.), underscore (_), dash (-), and
      alphanumeric characters. A whitespace is considered to be an invalid character.</p>
    detector_id = "value"  # <p>The detector ID associated with the GuardDuty account for which you want to create a filter.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
}

# Access filter outputs
filter_id = filter.id
filter_rank = filter.rank
filter_tags = filter.tags
filter_name = filter.name
filter_action = filter.action
filter_finding_criteria = filter.finding_criteria
filter_description = filter.description
```

---


### Detector

Detector resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags to be added to a new detector resource.</p> |
| `features` | Vec<String> |  | <p>A list of features that will be configured for the detector.</p> |
| `client_token` | String |  | <p>The idempotency token for the create request.</p> |
| `data_sources` | String |  | <p>Describes which data sources will be enabled for the detector.</p>
         <p>There might be regional differences because some data sources might not be 
      available in all the Amazon Web Services Regions where GuardDuty is presently supported. For more 
      information, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_regions.html">Regions and endpoints</a>.</p> |
| `finding_publishing_frequency` | String |  | <p>A value that specifies how frequently updated findings are exported.</p> |
| `enable` | bool | ✅ | <p>A Boolean value that specifies whether the detector is to be enabled.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated_at` | String | <p>The last-updated timestamp for the detector.</p> |
| `service_role` | String | <p>The GuardDuty service role.</p> |
| `data_sources` | String | <p>Describes which data sources are enabled for the detector.</p> |
| `created_at` | String | <p>The timestamp of when the detector was created.</p> |
| `finding_publishing_frequency` | String | <p>The publishing frequency of the finding.</p> |
| `tags` | HashMap<String, String> | <p>The tags of the detector resource.</p> |
| `status` | String | <p>The detector status.</p> |
| `features` | Vec<String> | <p>Describes the features that have been enabled for the detector.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create detector
detector = provider.guardduty.Detector {
    enable = "value"  # <p>A Boolean value that specifies whether the detector is to be enabled.</p>
}

# Access detector outputs
detector_id = detector.id
detector_updated_at = detector.updated_at
detector_service_role = detector.service_role
detector_data_sources = detector.data_sources
detector_created_at = detector.created_at
detector_finding_publishing_frequency = detector.finding_publishing_frequency
detector_tags = detector.tags
detector_status = detector.status
detector_features = detector.features
```

---


### Trusted_entity_set

TrustedEntitySet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String | ✅ | <p>The URI of the file that contains the threat entity set. The format of the <code>Location</code> URL must be a valid
       Amazon S3 URL format. Invalid URL formats will result in an error, regardless of whether you activate the entity set or not. For more information about
       format of the location URLs, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-lists-create-activate.html">Format of location URL under Step 2: Adding trusted or threat intelligence data</a>
       in the <i>Amazon GuardDuty User Guide</i>.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be added to a new trusted entity set resource.</p> |
| `detector_id` | String | ✅ | <p>The unique ID of the detector of the GuardDuty account for which you want to create a trusted
      entity set.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `activate` | bool | ✅ | <p>A boolean value that indicates whether GuardDuty is to start using the uploaded trusted entity set.</p> |
| `name` | String | ✅ | <p>A user-friendly name to identify the trusted entity set.</p>
         <p>The name of your list can include 
                                lowercase letters, uppercase letters, numbers, dash (-), and underscore (_).</p> |
| `format` | String | ✅ | <p>The format of the file that contains the trusted entity set.</p> |
| `expected_bucket_owner` | String |  | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b>
      parameter.</p> |
| `client_token` | String |  | <p>The idempotency token for the create request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | <p>The URI of the file that contains the trusted entity set.</p> |
| `expected_bucket_owner` | String | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b>
       parameter.</p> |
| `updated_at` | String | <p>The timestamp when the associated trusted entity set was updated.</p> |
| `tags` | HashMap<String, String> | <p>The tags associated with trusted entity set resource.</p> |
| `name` | String | <p>The name of the threat entity set associated with the specified <code>trustedEntitySetId</code>.</p> |
| `status` | String | <p>The status of the associated trusted entity set.</p> |
| `created_at` | String | <p>The timestamp when the associated trusted entity set was created.</p> |
| `error_details` | String | <p>The error details when the status is shown as <code>ERROR</code>.</p> |
| `format` | String | <p>The format of the file that contains the trusted entity set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trusted_entity_set
trusted_entity_set = provider.guardduty.Trusted_entity_set {
    location = "value"  # <p>The URI of the file that contains the threat entity set. The format of the <code>Location</code> URL must be a valid
       Amazon S3 URL format. Invalid URL formats will result in an error, regardless of whether you activate the entity set or not. For more information about
       format of the location URLs, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-lists-create-activate.html">Format of location URL under Step 2: Adding trusted or threat intelligence data</a>
       in the <i>Amazon GuardDuty User Guide</i>.</p>
    detector_id = "value"  # <p>The unique ID of the detector of the GuardDuty account for which you want to create a trusted
      entity set.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    activate = "value"  # <p>A boolean value that indicates whether GuardDuty is to start using the uploaded trusted entity set.</p>
    name = "value"  # <p>A user-friendly name to identify the trusted entity set.</p>
         <p>The name of your list can include 
                                lowercase letters, uppercase letters, numbers, dash (-), and underscore (_).</p>
    format = "value"  # <p>The format of the file that contains the trusted entity set.</p>
}

# Access trusted_entity_set outputs
trusted_entity_set_id = trusted_entity_set.id
trusted_entity_set_location = trusted_entity_set.location
trusted_entity_set_expected_bucket_owner = trusted_entity_set.expected_bucket_owner
trusted_entity_set_updated_at = trusted_entity_set.updated_at
trusted_entity_set_tags = trusted_entity_set.tags
trusted_entity_set_name = trusted_entity_set.name
trusted_entity_set_status = trusted_entity_set.status
trusted_entity_set_created_at = trusted_entity_set.created_at
trusted_entity_set_error_details = trusted_entity_set.error_details
trusted_entity_set_format = trusted_entity_set.format
```

---


### Malware_scans

MalwareScans resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scans` | Vec<String> | <p>Contains information about malware scans associated with GuardDuty Malware Protection for EC2.</p> |
| `next_token` | String | <p>The pagination parameter to be used on the next list operation to retrieve more
      items.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access malware_scans outputs
malware_scans_id = malware_scans.id
malware_scans_scans = malware_scans.scans
malware_scans_next_token = malware_scans.next_token
```

---


### Invitations_count

InvitationsCount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invitations_count` | i64 | <p>The number of received invitations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access invitations_count outputs
invitations_count_id = invitations_count.id
invitations_count_invitations_count = invitations_count.invitations_count
```

---


### Organization_configuration

OrganizationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_enable` | bool |  | <p>Represents whether to automatically enable member accounts in the organization. This
    applies to only new member accounts, not the existing member accounts. When a new account joins the organization,
    the chosen features will be enabled for them by default.</p>
         <p>Even though this is still supported, we recommend using
   <code>AutoEnableOrganizationMembers</code> to achieve the similar results. You must provide a 
    value for either <code>autoEnableOrganizationMembers</code> or <code>autoEnable</code>.</p> |
| `data_sources` | String |  | <p>Describes which data sources will be updated.</p> |
| `features` | Vec<String> |  | <p>A list of features that will be configured for the organization.</p> |
| `auto_enable_organization_members` | String |  | <p>Indicates the auto-enablement configuration of GuardDuty for the member accounts in the
      organization. You must provide a value for either <code>autoEnableOrganizationMembers</code> or <code>autoEnable</code>. </p>
         <p>Use one of the 
    following configuration values for <code>autoEnableOrganizationMembers</code>:</p>
         <ul>
            <li>
               <p>
                  <code>NEW</code>: Indicates that when a new account joins the organization, they will
          have GuardDuty enabled automatically. </p>
            </li>
            <li>
               <p>
                  <code>ALL</code>: Indicates that all accounts in the organization have GuardDuty
          enabled automatically. This includes <code>NEW</code> accounts that join the organization
          and accounts that may have been suspended or removed from the organization in
          GuardDuty.</p>
               <p>It may take up to 24 hours to update the configuration for all the member accounts.</p>
            </li>
            <li>
               <p>
                  <code>NONE</code>: Indicates that GuardDuty will not be automatically enabled for any
          account in the organization. The administrator must manage GuardDuty for each account in the organization individually.</p>
               <p>When you update the auto-enable setting from <code>ALL</code> or <code>NEW</code> to 
          <code>NONE</code>, this action doesn't disable the corresponding option for your existing accounts. This
          configuration will apply to the new accounts that join the organization. After you update the auto-enable settings,
          no new account will have the corresponding option as enabled.</p>
            </li>
         </ul> |
| `detector_id` | String | ✅ | <p>The ID of the detector that configures the delegated administrator.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_enable_organization_members` | String | <p>Indicates the auto-enablement configuration of GuardDuty or any of the corresponding protection plans for the member accounts in the
      organization.</p>
         <ul>
            <li>
               <p>
                  <code>NEW</code>: Indicates that when a new account joins the organization, they will
          have GuardDuty or any of the corresponding protection plans enabled automatically. </p>
            </li>
            <li>
               <p>
                  <code>ALL</code>: Indicates that all accounts in the organization have GuardDuty and any of the corresponding 
          protection plans enabled automatically. This includes <code>NEW</code> accounts that join the organization
          and accounts that may have been suspended or removed from the organization in
          GuardDuty.</p>
            </li>
            <li>
               <p>
                  <code>NONE</code>: Indicates that GuardDuty or any of the corresponding protection plans 
          will not be automatically enabled for any
          account in the organization. The administrator must manage GuardDuty for each account in 
          the organization individually.</p>
               <p>When you update the auto-enable setting from <code>ALL</code> or <code>NEW</code> to 
        <code>NONE</code>, this action doesn't disable the corresponding option for your existing accounts. This
        configuration will apply to the new accounts that join the organization. After you update the auto-enable settings,
        no new account will have the corresponding option as enabled.</p>
            </li>
         </ul> |
| `next_token` | String | <p>The pagination parameter to be used on the next list operation to retrieve more
      items.</p> |
| `member_account_limit_reached` | bool | <p>Indicates whether the maximum number of allowed member accounts are already associated
      with the delegated administrator account for your organization.</p> |
| `auto_enable` | bool | <p>Indicates whether GuardDuty is automatically enabled for accounts added to the
      organization.</p>
         <p>Even though this is still supported, we recommend using
        <code>AutoEnableOrganizationMembers</code> to achieve the similar results.</p> |
| `data_sources` | String | <p>Describes which data sources are enabled automatically for member accounts.</p> |
| `features` | Vec<String> | <p>A list of features that are configured for this organization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_configuration outputs
organization_configuration_id = organization_configuration.id
organization_configuration_auto_enable_organization_members = organization_configuration.auto_enable_organization_members
organization_configuration_next_token = organization_configuration.next_token
organization_configuration_member_account_limit_reached = organization_configuration.member_account_limit_reached
organization_configuration_auto_enable = organization_configuration.auto_enable
organization_configuration_data_sources = organization_configuration.data_sources
organization_configuration_features = organization_configuration.features
```

---


### Ip_set

IPSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The user-friendly name to identify the IPSet.</p>
         <p> Allowed characters are alphanumeric, whitespace, dash (-), and underscores (_).</p> |
| `format` | String | ✅ | <p>The format of the file that contains the IPSet.</p> |
| `activate` | bool | ✅ | <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded
      IPSet.</p> |
| `detector_id` | String | ✅ | <p>The unique ID of the detector of the GuardDuty account for which you want to create an IPSet.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `location` | String | ✅ | <p>The URI of the file that contains the IPSet. </p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be added to a new IP set resource.</p> |
| `expected_bucket_owner` | String |  | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter.</p> |
| `client_token` | String |  | <p>The idempotency token for the create request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | <p>The URI of the file that contains the IPSet.</p> |
| `expected_bucket_owner` | String | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter. This field appears 
      in the response only if it was provided during IPSet creation or update.</p> |
| `format` | String | <p>The format of the file that contains the IPSet.</p> |
| `name` | String | <p>The user-friendly name for the IPSet.</p> |
| `tags` | HashMap<String, String> | <p>The tags of the IPSet resource.</p> |
| `status` | String | <p>The status of IPSet file that was uploaded.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ip_set
ip_set = provider.guardduty.Ip_set {
    name = "value"  # <p>The user-friendly name to identify the IPSet.</p>
         <p> Allowed characters are alphanumeric, whitespace, dash (-), and underscores (_).</p>
    format = "value"  # <p>The format of the file that contains the IPSet.</p>
    activate = "value"  # <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded
      IPSet.</p>
    detector_id = "value"  # <p>The unique ID of the detector of the GuardDuty account for which you want to create an IPSet.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    location = "value"  # <p>The URI of the file that contains the IPSet. </p>
}

# Access ip_set outputs
ip_set_id = ip_set.id
ip_set_location = ip_set.location
ip_set_expected_bucket_owner = ip_set.expected_bucket_owner
ip_set_format = ip_set.format
ip_set_name = ip_set.name
ip_set_tags = ip_set.tags
ip_set_status = ip_set.status
```

---


### Master_account

MasterAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `master` | String | <p>The administrator account details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access master_account outputs
master_account_id = master_account.id
master_account_master = master_account.master
```

---


### Findings_statistics

FindingsStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `finding_statistics` | String | <p>The finding statistics object.</p> |
| `next_token` | String | <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
         <p>This parameter is currently not supported.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings_statistics outputs
findings_statistics_id = findings_statistics.id
findings_statistics_finding_statistics = findings_statistics.finding_statistics
findings_statistics_next_token = findings_statistics.next_token
```

---


### Members

Members resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `detector_id` | String | ✅ | <p>The unique ID of the detector of the GuardDuty account for which you want to associate member
      accounts.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `account_details` | Vec<String> | ✅ | <p>A list of account ID and email address pairs of the accounts that you want to associate
      with the GuardDuty administrator account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unprocessed_accounts` | Vec<String> | <p>A list of objects that contain the unprocessed account and a result string that explains
      why it was unprocessed.</p> |
| `members` | Vec<String> | <p>A list of members.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create members
members = provider.guardduty.Members {
    detector_id = "value"  # <p>The unique ID of the detector of the GuardDuty account for which you want to associate member
      accounts.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    account_details = "value"  # <p>A list of account ID and email address pairs of the accounts that you want to associate
      with the GuardDuty administrator account.</p>
}

# Access members outputs
members_id = members.id
members_unprocessed_accounts = members.unprocessed_accounts
members_members = members.members
```

---


### Sample_findings

SampleFindings resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `detector_id` | String | ✅ | <p>The ID of the detector for which you need to create sample findings.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `finding_types` | Vec<String> |  | <p>The types of sample findings to generate.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create sample_findings
sample_findings = provider.guardduty.Sample_findings {
    detector_id = "value"  # <p>The ID of the detector for which you need to create sample findings.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
}

```

---


### Organization_statistics

OrganizationStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_details` | String | <p>Information about the statistics report for your organization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organization_statistics outputs
organization_statistics_id = organization_statistics.id
organization_statistics_organization_details = organization_statistics.organization_details
```

---


### Threat_intel_set

ThreatIntelSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `activate` | bool | ✅ | <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded
      ThreatIntelSet.</p> |
| `detector_id` | String | ✅ | <p>The unique ID of the detector of the GuardDuty account for which you want to create a
      <code>threatIntelSet</code>.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be added to a new threat list resource.</p> |
| `client_token` | String |  | <p>The idempotency token for the create request.</p> |
| `format` | String | ✅ | <p>The format of the file that contains the ThreatIntelSet.</p> |
| `location` | String | ✅ | <p>The URI of the file that contains the ThreatIntelSet. </p> |
| `expected_bucket_owner` | String |  | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter.</p> |
| `name` | String | ✅ | <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by
      activity that involves IP addresses included in this ThreatIntelSet.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expected_bucket_owner` | String | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter. 
      This field appears in the response only if it was provided during ThreatIntelSet creation or update.</p> |
| `format` | String | <p>The format of the threatIntelSet.</p> |
| `location` | String | <p>The URI of the file that contains the ThreatIntelSet. </p> |
| `tags` | HashMap<String, String> | <p>The tags of the threat list resource.</p> |
| `name` | String | <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by
      activity that involves IP addresses included in this ThreatIntelSet.</p> |
| `status` | String | <p>The status of threatIntelSet file uploaded.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create threat_intel_set
threat_intel_set = provider.guardduty.Threat_intel_set {
    activate = "value"  # <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded
      ThreatIntelSet.</p>
    detector_id = "value"  # <p>The unique ID of the detector of the GuardDuty account for which you want to create a
      <code>threatIntelSet</code>.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    format = "value"  # <p>The format of the file that contains the ThreatIntelSet.</p>
    location = "value"  # <p>The URI of the file that contains the ThreatIntelSet. </p>
    name = "value"  # <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by
      activity that involves IP addresses included in this ThreatIntelSet.</p>
}

# Access threat_intel_set outputs
threat_intel_set_id = threat_intel_set.id
threat_intel_set_expected_bucket_owner = threat_intel_set.expected_bucket_owner
threat_intel_set_format = threat_intel_set.format
threat_intel_set_location = threat_intel_set.location
threat_intel_set_tags = threat_intel_set.tags
threat_intel_set_name = threat_intel_set.name
threat_intel_set_status = threat_intel_set.status
```

---


### Coverage_statistics

CoverageStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `coverage_statistics` | String | <p>Represents the count aggregated by the <code>statusCode</code> and
        <code>resourceType</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access coverage_statistics outputs
coverage_statistics_id = coverage_statistics.id
coverage_statistics_coverage_statistics = coverage_statistics.coverage_statistics
```

---


### Malware_scan_settings

MalwareScanSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ebs_snapshot_preservation` | String |  | <p>An enum value representing possible snapshot preservation settings.</p> |
| `detector_id` | String | ✅ | <p>The unique ID of the detector that specifies the GuardDuty service where you want to
      update scan settings.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `scan_resource_criteria` | String |  | <p>Represents the criteria to be used in the filter for selecting resources to scan.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scan_resource_criteria` | String | <p>Represents the criteria to be used in the filter for scanning resources.</p> |
| `ebs_snapshot_preservation` | String | <p>An enum value representing possible snapshot preservation settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access malware_scan_settings outputs
malware_scan_settings_id = malware_scan_settings.id
malware_scan_settings_scan_resource_criteria = malware_scan_settings.scan_resource_criteria
malware_scan_settings_ebs_snapshot_preservation = malware_scan_settings.ebs_snapshot_preservation
```

---


### Threat_entity_set

ThreatEntitySet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `format` | String | ✅ | <p>The format of the file that contains the threat entity set.</p> |
| `detector_id` | String | ✅ | <p>The unique ID of the detector of the GuardDuty account for which you want to create a threat entity set.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `location` | String | ✅ | <p>The URI of the file that contains the threat entity set. The format of the <code>Location</code> URL must be a valid
       Amazon S3 URL format. Invalid URL formats will result in an error, regardless of whether you activate the entity set or not. For more information about
       format of the location URLs, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-lists-create-activate.html">Format of location URL under Step 2: Adding trusted or threat intelligence data</a>
     in the <i>Amazon GuardDuty User Guide</i>.</p> |
| `expected_bucket_owner` | String |  | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be added to a new threat entity set resource.</p> |
| `client_token` | String |  | <p>The idempotency token for the create request.</p> |
| `name` | String | ✅ | <p>A user-friendly name to identify the threat entity set.</p>
         <p>The name of your list can include 
                                lowercase letters, uppercase letters, numbers, dash (-), and underscore (_).</p> |
| `activate` | bool | ✅ | <p>A boolean value that indicates whether GuardDuty should start using the uploaded threat entity set to
        generate findings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags associated with the threat entity set resource.</p> |
| `error_details` | String | <p>The error details when the status is shown as <code>ERROR</code>.</p> |
| `status` | String | <p>The status of the associated threat entity set.</p> |
| `name` | String | <p>The name of the threat entity set associated with the specified <code>threatEntitySetId</code>.</p> |
| `expected_bucket_owner` | String | <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b>
       parameter.</p> |
| `updated_at` | String | <p>The timestamp when the associated threat entity set was updated.</p> |
| `format` | String | <p>The format of the file that contains the threat entity set.</p> |
| `location` | String | <p>The URI of the file that contains the threat entity set.</p> |
| `created_at` | String | <p>The timestamp when the associated threat entity set was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create threat_entity_set
threat_entity_set = provider.guardduty.Threat_entity_set {
    format = "value"  # <p>The format of the file that contains the threat entity set.</p>
    detector_id = "value"  # <p>The unique ID of the detector of the GuardDuty account for which you want to create a threat entity set.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    location = "value"  # <p>The URI of the file that contains the threat entity set. The format of the <code>Location</code> URL must be a valid
       Amazon S3 URL format. Invalid URL formats will result in an error, regardless of whether you activate the entity set or not. For more information about
       format of the location URLs, see <a href="https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-lists-create-activate.html">Format of location URL under Step 2: Adding trusted or threat intelligence data</a>
     in the <i>Amazon GuardDuty User Guide</i>.</p>
    name = "value"  # <p>A user-friendly name to identify the threat entity set.</p>
         <p>The name of your list can include 
                                lowercase letters, uppercase letters, numbers, dash (-), and underscore (_).</p>
    activate = "value"  # <p>A boolean value that indicates whether GuardDuty should start using the uploaded threat entity set to
        generate findings.</p>
}

# Access threat_entity_set outputs
threat_entity_set_id = threat_entity_set.id
threat_entity_set_tags = threat_entity_set.tags
threat_entity_set_error_details = threat_entity_set.error_details
threat_entity_set_status = threat_entity_set.status
threat_entity_set_name = threat_entity_set.name
threat_entity_set_expected_bucket_owner = threat_entity_set.expected_bucket_owner
threat_entity_set_updated_at = threat_entity_set.updated_at
threat_entity_set_format = threat_entity_set.format
threat_entity_set_location = threat_entity_set.location
threat_entity_set_created_at = threat_entity_set.created_at
```

---


### Administrator_account

AdministratorAccount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `administrator` | String | <p>The administrator account details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access administrator_account outputs
administrator_account_id = administrator_account.id
administrator_account_administrator = administrator_account.administrator
```

---


### Remaining_free_trial_days

RemainingFreeTrialDays resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unprocessed_accounts` | Vec<String> | <p>The member account that was included in a request but for which the request could not be
      processed.</p> |
| `accounts` | Vec<String> | <p>The member accounts which were included in a request and were processed
      successfully.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access remaining_free_trial_days outputs
remaining_free_trial_days_id = remaining_free_trial_days.id
remaining_free_trial_days_unprocessed_accounts = remaining_free_trial_days.unprocessed_accounts
remaining_free_trial_days_accounts = remaining_free_trial_days.accounts
```

---


### Member_detectors

MemberDetectors resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `features` | Vec<String> |  | <p>A list of features that will be updated for the specified member accounts.</p> |
| `account_ids` | Vec<String> | ✅ | <p>A list of member account IDs to be updated.</p> |
| `detector_id` | String | ✅ | <p>The detector ID of the administrator account.</p>
         <p>To find the <code>detectorId</code> in the current Region, see the
Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p> |
| `data_sources` | String |  | <p>Describes which data sources will be updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unprocessed_accounts` | Vec<String> | <p>A list of member account IDs that were unable to be processed along with an explanation
      for why they were not processed.</p> |
| `member_data_source_configurations` | Vec<String> | <p>An object that describes which data sources are enabled for a member account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access member_detectors outputs
member_detectors_id = member_detectors.id
member_detectors_unprocessed_accounts = member_detectors.unprocessed_accounts
member_detectors_member_data_source_configurations = member_detectors.member_data_source_configurations
```

---


### Findings

Findings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `findings` | Vec<String> | <p>A list of findings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access findings outputs
findings_id = findings.id
findings_findings = findings.findings
```

---


### Malware_protection_plan

MalwareProtectionPlan resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | String | ✅ | <p>Amazon Resource Name (ARN) of the IAM role that has the permissions to scan and add tags to the associated
      protected resource.</p> |
| `client_token` | String |  | <p>The idempotency token for the create request.</p> |
| `protected_resource` | String | ✅ | <p>Information about the protected resource that is associated with the created 
      Malware Protection plan. Presently, <code>S3Bucket</code> is the only supported 
      protected resource.</p> |
| `actions` | String |  | <p>Information about whether the tags will be added to the S3 object after scanning.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags added to the Malware Protection plan resource. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_reasons` | Vec<String> | <p>Information about the issue code and message associated to the status of
    your Malware Protection plan.</p> |
| `role` | String | <p>Amazon Resource Name (ARN) of the IAM role that includes the permissions to scan and 
      add tags to the associated protected resource.</p> |
| `protected_resource` | String | <p>Information about the protected resource that is associated with the created 
      Malware Protection plan. Presently, <code>S3Bucket</code> is the only supported 
      protected resource.</p> |
| `actions` | String | <p>Information about whether the tags will be added to the S3 object after scanning.</p> |
| `created_at` | String | <p>The timestamp when the Malware Protection plan resource was created.</p> |
| `status` | String | <p>Malware Protection plan status.</p> |
| `arn` | String | <p>Amazon Resource Name (ARN) of the protected resource.</p> |
| `tags` | HashMap<String, String> | <p>Tags added to the Malware Protection plan resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create malware_protection_plan
malware_protection_plan = provider.guardduty.Malware_protection_plan {
    role = "value"  # <p>Amazon Resource Name (ARN) of the IAM role that has the permissions to scan and add tags to the associated
      protected resource.</p>
    protected_resource = "value"  # <p>Information about the protected resource that is associated with the created 
      Malware Protection plan. Presently, <code>S3Bucket</code> is the only supported 
      protected resource.</p>
}

# Access malware_protection_plan outputs
malware_protection_plan_id = malware_protection_plan.id
malware_protection_plan_status_reasons = malware_protection_plan.status_reasons
malware_protection_plan_role = malware_protection_plan.role
malware_protection_plan_protected_resource = malware_protection_plan.protected_resource
malware_protection_plan_actions = malware_protection_plan.actions
malware_protection_plan_created_at = malware_protection_plan.created_at
malware_protection_plan_status = malware_protection_plan.status
malware_protection_plan_arn = malware_protection_plan.arn
malware_protection_plan_tags = malware_protection_plan.tags
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple usage_statistics resources
usage_statistics_0 = provider.guardduty.Usage_statistics {
}
usage_statistics_1 = provider.guardduty.Usage_statistics {
}
usage_statistics_2 = provider.guardduty.Usage_statistics {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    usage_statistics = provider.guardduty.Usage_statistics {
    }
```

---

## Related Documentation

- [AWS Guardduty Documentation](https://docs.aws.amazon.com/guardduty/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
