# Resiliencehub Service



**Resources**: 14

---

## Overview

The resiliencehub service provides access to 14 resource types:

- [App_version_template](#app_version_template) [R]
- [Draft_app_version_template](#draft_app_version_template) [C]
- [App_version_resources_resolution_status](#app_version_resources_resolution_status) [R]
- [Resource_grouping_recommendation_task](#resource_grouping_recommendation_task) [R]
- [App_assessment](#app_assessment) [RD]
- [App_version_resource](#app_version_resource) [CRUD]
- [Resiliency_policy](#resiliency_policy) [CRUD]
- [App_version_app_component](#app_version_app_component) [CRUD]
- [App](#app) [CRUD]
- [App_input_source](#app_input_source) [D]
- [Metrics_export](#metrics_export) [R]
- [App_version](#app_version) [RU]
- [Recommendation_template](#recommendation_template) [CD]
- [Draft_app_version_resources_import_status](#draft_app_version_resources_import_status) [R]

---

## Resources


### App_version_template

AppVersionTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_arn` | String | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `app_version` | String | <p>The version of the application.</p> |
| `app_template_body` | String | <p>A JSON string that provides information about your application structure. To learn more
      about the <code>appTemplateBody</code> template, see the sample template provided in the
        <i>Examples</i> section.</p>
         <p>The <code>appTemplateBody</code> JSON string has the following structure:</p>
         <ul>
            <li>
               <p>
                  <b>
                     <code>resources</code>
                  </b>
               </p>
               <p>The list of logical resources that must be included in the Resilience Hub
          application.</p>
               <p>Type: Array</p>
               <note>
                  <p>Don't add the resources that you want to exclude.</p>
               </note>
               <p>Each <code>resources</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <i>
                           <code>logicalResourceId</code>
                        </i>
                     </p>
                     <p>Logical identifier of the resource.</p>
                     <p>Type: Object</p>
                     <p>Each <code>logicalResourceId</code> object includes the following fields:</p>
                     <ul>
                        <li>
                           <p>
                              <code>identifier</code>
                           </p>
                           <p>Identifier of the resource.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>logicalStackName</code>
                           </p>
                           <p>The name of the CloudFormation stack this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>resourceGroupName</code>
                           </p>
                           <p>The name of the resource group this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>terraformSourceName</code>
                           </p>
                           <p>The name of the Terraform S3 state file this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>eksSourceName</code>
                           </p>
                           <p>Name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.</p>
                           <note>
                              <p>This parameter accepts values in "eks-cluster/namespace" format.</p>
                           </note>
                           <p>Type: String</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <i>
                           <code>type</code>
                        </i>
                     </p>
                     <p>The type of resource.</p>
                     <p>Type: string</p>
                  </li>
                  <li>
                     <p>
                        <i>
                           <code>name</code>
                        </i>
                     </p>
                     <p>The name of the resource.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>additionalInfo</code>
                     </p>
                     <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
                     <note>
                        <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                        <p>Key: <code>"failover-regions"</code>
                        </p>
                        <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                        </p>
                     </note>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>appComponents</code>
                  </b>
               </p>
               <p>List of Application Components that this resource belongs to. If an Application Component is not part of the Resilience Hub application, it will be added.</p>
               <p>Type: Array</p>
               <p>Each <code>appComponents</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <code>name</code>
                     </p>
                     <p>Name of the Application Component.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>type</code>
                     </p>
                     <p>Type of Application Component. For more information about the types of Application Component, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/AppComponent.grouping.html">Grouping resources in an AppComponent</a>.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>resourceNames</code>
                     </p>
                     <p>The list of included resources that are assigned to the Application Component.</p>
                     <p>Type: Array of strings</p>
                  </li>
                  <li>
                     <p>
                        <code>additionalInfo</code>
                     </p>
                     <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
                     <note>
                        <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                        <p>Key: <code>"failover-regions"</code>
                        </p>
                        <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                        </p>
                     </note>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>excludedResources</code>
                  </b>
               </p>
               <p>The list of logical resource identifiers to be excluded from the application.</p>
               <p>Type: Array</p>
               <note>
                  <p>Don't add the resources that you want to include.</p>
               </note>
               <p>Each <code>excludedResources</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <i>
                           <code>logicalResourceIds</code>
                        </i>
                     </p>
                     <p>Logical identifier of the resource.</p>
                     <p>Type: Object</p>
                     <note>
                        <p>You can configure only one of the following fields:</p>
                        <ul>
                           <li>
                              <p>
                                 <code>logicalStackName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>resourceGroupName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>terraformSourceName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>eksSourceName</code>
                              </p>
                           </li>
                        </ul>
                     </note>
                     <p>Each <code>logicalResourceIds</code> object includes the following fields:</p>
                     <ul>
                        <li>
                           <p>
                              <code>identifier</code>
                           </p>
                           <p>Identifier of the resource.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>logicalStackName</code>
                           </p>
                           <p>The name of the CloudFormation stack this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>resourceGroupName</code>
                           </p>
                           <p>The name of the resource group this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>terraformSourceName</code>
                           </p>
                           <p>The name of the Terraform S3 state file this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>eksSourceName</code>
                           </p>
                           <p>Name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.</p>
                           <note>
                              <p>This parameter accepts values in "eks-cluster/namespace" format.</p>
                           </note>
                           <p>Type: String</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>version</code>
                  </b>
               </p>
               <p>Resilience Hub application version.</p>
            </li>
            <li>
               <p>
                  <code>additionalInfo</code>
               </p>
               <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
               <note>
                  <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                  <p>Key: <code>"failover-regions"</code>
                  </p>
                  <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                  </p>
               </note>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_version_template outputs
app_version_template_id = app_version_template.id
app_version_template_app_arn = app_version_template.app_arn
app_version_template_app_version = app_version_template.app_version
app_version_template_app_template_body = app_version_template.app_template_body
```

---


### Draft_app_version_template

DraftAppVersionTemplate resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_template_body` | String | ✅ | <p>A JSON string that provides information about your application structure. To learn more
      about the <code>appTemplateBody</code> template, see the sample template provided in the
        <i>Examples</i> section.</p>
         <p>The <code>appTemplateBody</code> JSON string has the following structure:</p>
         <ul>
            <li>
               <p>
                  <b>
                     <code>resources</code>
                  </b>
               </p>
               <p>The list of logical resources that must be included in the Resilience Hub
          application.</p>
               <p>Type: Array</p>
               <note>
                  <p>Don't add the resources that you want to exclude.</p>
               </note>
               <p>Each <code>resources</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <i>
                           <code>logicalResourceId</code>
                        </i>
                     </p>
                     <p>Logical identifier of the resource.</p>
                     <p>Type: Object</p>
                     <p>Each <code>logicalResourceId</code> object includes the following fields:</p>
                     <ul>
                        <li>
                           <p>
                              <code>identifier</code>
                           </p>
                           <p>Identifier of the resource.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>logicalStackName</code>
                           </p>
                           <p>The name of the CloudFormation stack this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>resourceGroupName</code>
                           </p>
                           <p>The name of the resource group this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>terraformSourceName</code>
                           </p>
                           <p>The name of the Terraform S3 state file this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>eksSourceName</code>
                           </p>
                           <p>Name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.</p>
                           <note>
                              <p>This parameter accepts values in "eks-cluster/namespace" format.</p>
                           </note>
                           <p>Type: String</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <i>
                           <code>type</code>
                        </i>
                     </p>
                     <p>The type of resource.</p>
                     <p>Type: string</p>
                  </li>
                  <li>
                     <p>
                        <i>
                           <code>name</code>
                        </i>
                     </p>
                     <p>The name of the resource.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>additionalInfo</code>
                     </p>
                     <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
                     <note>
                        <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                        <p>Key: <code>"failover-regions"</code>
                        </p>
                        <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                        </p>
                     </note>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>appComponents</code>
                  </b>
               </p>
               <p>List of Application Components that this resource belongs to. If an Application Component is not part of the Resilience Hub application, it will be added.</p>
               <p>Type: Array</p>
               <p>Each <code>appComponents</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <code>name</code>
                     </p>
                     <p>Name of the Application Component.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>type</code>
                     </p>
                     <p>Type of Application Component. For more information about the types of Application Component, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/AppComponent.grouping.html">Grouping resources in an AppComponent</a>.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>resourceNames</code>
                     </p>
                     <p>The list of included resources that are assigned to the Application Component.</p>
                     <p>Type: Array of strings</p>
                  </li>
                  <li>
                     <p>
                        <code>additionalInfo</code>
                     </p>
                     <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
                     <note>
                        <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                        <p>Key: <code>"failover-regions"</code>
                        </p>
                        <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                        </p>
                     </note>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>excludedResources</code>
                  </b>
               </p>
               <p>The list of logical resource identifiers to be excluded from the application.</p>
               <p>Type: Array</p>
               <note>
                  <p>Don't add the resources that you want to include.</p>
               </note>
               <p>Each <code>excludedResources</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <i>
                           <code>logicalResourceIds</code>
                        </i>
                     </p>
                     <p>Logical identifier of the resource.</p>
                     <p>Type: Object</p>
                     <note>
                        <p>You can configure only one of the following fields:</p>
                        <ul>
                           <li>
                              <p>
                                 <code>logicalStackName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>resourceGroupName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>terraformSourceName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>eksSourceName</code>
                              </p>
                           </li>
                        </ul>
                     </note>
                     <p>Each <code>logicalResourceIds</code> object includes the following fields:</p>
                     <ul>
                        <li>
                           <p>
                              <code>identifier</code>
                           </p>
                           <p>Identifier of the resource.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>logicalStackName</code>
                           </p>
                           <p>The name of the CloudFormation stack this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>resourceGroupName</code>
                           </p>
                           <p>The name of the resource group this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>terraformSourceName</code>
                           </p>
                           <p>The name of the Terraform S3 state file this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>eksSourceName</code>
                           </p>
                           <p>Name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.</p>
                           <note>
                              <p>This parameter accepts values in "eks-cluster/namespace" format.</p>
                           </note>
                           <p>Type: String</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>version</code>
                  </b>
               </p>
               <p>Resilience Hub application version.</p>
            </li>
            <li>
               <p>
                  <code>additionalInfo</code>
               </p>
               <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
               <note>
                  <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                  <p>Key: <code>"failover-regions"</code>
                  </p>
                  <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                  </p>
               </note>
            </li>
         </ul> |
| `app_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create draft_app_version_template
draft_app_version_template = provider.resiliencehub.Draft_app_version_template {
    app_template_body = "value"  # <p>A JSON string that provides information about your application structure. To learn more
      about the <code>appTemplateBody</code> template, see the sample template provided in the
        <i>Examples</i> section.</p>
         <p>The <code>appTemplateBody</code> JSON string has the following structure:</p>
         <ul>
            <li>
               <p>
                  <b>
                     <code>resources</code>
                  </b>
               </p>
               <p>The list of logical resources that must be included in the Resilience Hub
          application.</p>
               <p>Type: Array</p>
               <note>
                  <p>Don't add the resources that you want to exclude.</p>
               </note>
               <p>Each <code>resources</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <i>
                           <code>logicalResourceId</code>
                        </i>
                     </p>
                     <p>Logical identifier of the resource.</p>
                     <p>Type: Object</p>
                     <p>Each <code>logicalResourceId</code> object includes the following fields:</p>
                     <ul>
                        <li>
                           <p>
                              <code>identifier</code>
                           </p>
                           <p>Identifier of the resource.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>logicalStackName</code>
                           </p>
                           <p>The name of the CloudFormation stack this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>resourceGroupName</code>
                           </p>
                           <p>The name of the resource group this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>terraformSourceName</code>
                           </p>
                           <p>The name of the Terraform S3 state file this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>eksSourceName</code>
                           </p>
                           <p>Name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.</p>
                           <note>
                              <p>This parameter accepts values in "eks-cluster/namespace" format.</p>
                           </note>
                           <p>Type: String</p>
                        </li>
                     </ul>
                  </li>
                  <li>
                     <p>
                        <i>
                           <code>type</code>
                        </i>
                     </p>
                     <p>The type of resource.</p>
                     <p>Type: string</p>
                  </li>
                  <li>
                     <p>
                        <i>
                           <code>name</code>
                        </i>
                     </p>
                     <p>The name of the resource.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>additionalInfo</code>
                     </p>
                     <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
                     <note>
                        <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                        <p>Key: <code>"failover-regions"</code>
                        </p>
                        <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                        </p>
                     </note>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>appComponents</code>
                  </b>
               </p>
               <p>List of Application Components that this resource belongs to. If an Application Component is not part of the Resilience Hub application, it will be added.</p>
               <p>Type: Array</p>
               <p>Each <code>appComponents</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <code>name</code>
                     </p>
                     <p>Name of the Application Component.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>type</code>
                     </p>
                     <p>Type of Application Component. For more information about the types of Application Component, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/AppComponent.grouping.html">Grouping resources in an AppComponent</a>.</p>
                     <p>Type: String</p>
                  </li>
                  <li>
                     <p>
                        <code>resourceNames</code>
                     </p>
                     <p>The list of included resources that are assigned to the Application Component.</p>
                     <p>Type: Array of strings</p>
                  </li>
                  <li>
                     <p>
                        <code>additionalInfo</code>
                     </p>
                     <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
                     <note>
                        <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                        <p>Key: <code>"failover-regions"</code>
                        </p>
                        <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                        </p>
                     </note>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>excludedResources</code>
                  </b>
               </p>
               <p>The list of logical resource identifiers to be excluded from the application.</p>
               <p>Type: Array</p>
               <note>
                  <p>Don't add the resources that you want to include.</p>
               </note>
               <p>Each <code>excludedResources</code> array item includes the following fields:</p>
               <ul>
                  <li>
                     <p>
                        <i>
                           <code>logicalResourceIds</code>
                        </i>
                     </p>
                     <p>Logical identifier of the resource.</p>
                     <p>Type: Object</p>
                     <note>
                        <p>You can configure only one of the following fields:</p>
                        <ul>
                           <li>
                              <p>
                                 <code>logicalStackName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>resourceGroupName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>terraformSourceName</code>
                              </p>
                           </li>
                           <li>
                              <p>
                                 <code>eksSourceName</code>
                              </p>
                           </li>
                        </ul>
                     </note>
                     <p>Each <code>logicalResourceIds</code> object includes the following fields:</p>
                     <ul>
                        <li>
                           <p>
                              <code>identifier</code>
                           </p>
                           <p>Identifier of the resource.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>logicalStackName</code>
                           </p>
                           <p>The name of the CloudFormation stack this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>resourceGroupName</code>
                           </p>
                           <p>The name of the resource group this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>terraformSourceName</code>
                           </p>
                           <p>The name of the Terraform S3 state file this resource belongs to.</p>
                           <p>Type: String</p>
                        </li>
                        <li>
                           <p>
                              <code>eksSourceName</code>
                           </p>
                           <p>Name of the Amazon Elastic Kubernetes Service cluster and namespace this resource belongs to.</p>
                           <note>
                              <p>This parameter accepts values in "eks-cluster/namespace" format.</p>
                           </note>
                           <p>Type: String</p>
                        </li>
                     </ul>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>
                     <code>version</code>
                  </b>
               </p>
               <p>Resilience Hub application version.</p>
            </li>
            <li>
               <p>
                  <code>additionalInfo</code>
               </p>
               <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
               <note>
                  <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
                  <p>Key: <code>"failover-regions"</code>
                  </p>
                  <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
                  </p>
               </note>
            </li>
         </ul>
    app_arn = "value"  # <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p>
}

```

---


### App_version_resources_resolution_status

AppVersionResourcesResolutionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Status of the action.</p> |
| `app_version` | String | <p>The version of the application.</p> |
| `app_arn` | String | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `error_message` | String | <p>The returned error message for the request.</p> |
| `resolution_id` | String | <p>The identifier for a specific resolution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_version_resources_resolution_status outputs
app_version_resources_resolution_status_id = app_version_resources_resolution_status.id
app_version_resources_resolution_status_status = app_version_resources_resolution_status.status
app_version_resources_resolution_status_app_version = app_version_resources_resolution_status.app_version
app_version_resources_resolution_status_app_arn = app_version_resources_resolution_status.app_arn
app_version_resources_resolution_status_error_message = app_version_resources_resolution_status.error_message
app_version_resources_resolution_status_resolution_id = app_version_resources_resolution_status.resolution_id
```

---


### Resource_grouping_recommendation_task

ResourceGroupingRecommendationTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Status of the action.</p> |
| `error_message` | String | <p>Error that occurred while generating a grouping recommendation.</p> |
| `grouping_id` | String | <p>Identifier of the grouping recommendation task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_grouping_recommendation_task outputs
resource_grouping_recommendation_task_id = resource_grouping_recommendation_task.id
resource_grouping_recommendation_task_status = resource_grouping_recommendation_task.status
resource_grouping_recommendation_task_error_message = resource_grouping_recommendation_task.error_message
resource_grouping_recommendation_task_grouping_id = resource_grouping_recommendation_task.grouping_id
```

---


### App_assessment

AppAssessment resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assessment` | String | <p>The assessment for an Resilience Hub application, returned as an object. This
      object includes Amazon Resource Names (ARNs), compliance information, compliance status, cost,
      messages, resiliency scores, and more.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_assessment outputs
app_assessment_id = app_assessment.id
app_assessment_assessment = app_assessment.assessment
```

---


### App_version_resource

AppVersionResource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aws_account_id` | String |  | <p>Amazon Web Services account that owns the physical resource.</p> |
| `resource_type` | String | ✅ | <p>Type of resource.</p> |
| `physical_resource_id` | String | ✅ | <p>Physical identifier of the resource.</p> |
| `aws_region` | String |  | <p>Amazon Web Services region that owns the physical resource.</p> |
| `resource_name` | String |  | <p>Name of the resource.</p> |
| `additional_info` | HashMap<String, Vec<String>> |  | <p>Currently, there is no supported additional information for resources.</p> |
| `client_token` | String |  | <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. 
You should not reuse the same client token for other API requests.</p> |
| `app_components` | Vec<String> | ✅ | <p>List of Application Components that this resource belongs to. If an Application Component is not part of the Resilience Hub application, it will be added.</p> |
| `logical_resource_id` | String | ✅ | <p>Logical identifier of the resource.</p> |
| `app_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_arn` | String | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `physical_resource` | String | <p>Defines a physical resource. A physical resource is a resource that exists in your account. It can be identified using an Amazon Resource Name (ARN) or a Resilience Hub-native identifier.</p> |
| `app_version` | String | <p>Resilience Hub application version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_version_resource
app_version_resource = provider.resiliencehub.App_version_resource {
    resource_type = "value"  # <p>Type of resource.</p>
    physical_resource_id = "value"  # <p>Physical identifier of the resource.</p>
    app_components = "value"  # <p>List of Application Components that this resource belongs to. If an Application Component is not part of the Resilience Hub application, it will be added.</p>
    logical_resource_id = "value"  # <p>Logical identifier of the resource.</p>
    app_arn = "value"  # <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p>
}

# Access app_version_resource outputs
app_version_resource_id = app_version_resource.id
app_version_resource_app_arn = app_version_resource.app_arn
app_version_resource_physical_resource = app_version_resource.physical_resource
app_version_resource_app_version = app_version_resource.app_version
```

---


### Resiliency_policy

ResiliencyPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Tags assigned to the resource. A tag is a label that you assign to an Amazon Web Services resource. 
Each tag consists of a key/value pair.</p> |
| `tier` | String | ✅ | <p>The tier for this resiliency policy, ranging from the highest severity
        (<code>MissionCritical</code>) to lowest (<code>NonCritical</code>).</p> |
| `policy_name` | String | ✅ | <p>Name of the resiliency policy.</p> |
| `policy` | HashMap<String, String> | ✅ | <p>The type of resiliency policy to be created, including the recovery time objective (RTO)
      and recovery point objective (RPO) in seconds.</p> |
| `policy_description` | String |  | <p>Description of the resiliency policy.</p> |
| `data_location_constraint` | String |  | <p>Specifies a high-level geographical location constraint for where your resilience policy
      data can be stored.</p> |
| `client_token` | String |  | <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. 
You should not reuse the same client token for other API requests.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>Information about the specific resiliency policy, returned as an object. This object
      includes creation time, data location constraints, its name, description, tags, the recovery
      time objective (RTO) and recovery point objective (RPO) in seconds, and more.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resiliency_policy
resiliency_policy = provider.resiliencehub.Resiliency_policy {
    tier = "value"  # <p>The tier for this resiliency policy, ranging from the highest severity
        (<code>MissionCritical</code>) to lowest (<code>NonCritical</code>).</p>
    policy_name = "value"  # <p>Name of the resiliency policy.</p>
    policy = "value"  # <p>The type of resiliency policy to be created, including the recovery time objective (RTO)
      and recovery point objective (RPO) in seconds.</p>
}

# Access resiliency_policy outputs
resiliency_policy_id = resiliency_policy.id
resiliency_policy_policy = resiliency_policy.policy
```

---


### App_version_app_component

AppVersionAppComponent resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>Type of Application Component. For more information about the types of Application Component, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/AppComponent.grouping.html">Grouping resources in an AppComponent</a>.</p> |
| `name` | String | ✅ | <p>Name of the Application Component.</p> |
| `additional_info` | HashMap<String, Vec<String>> |  | <p>Currently, there is no supported additional information for Application Components.</p> |
| `client_token` | String |  | <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. 
You should not reuse the same client token for other API requests.</p> |
| `app_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `id` | String |  | <p>Identifier of the Application Component.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_version` | String | <p>Resilience Hub application version.</p> |
| `app_component` | String | <p>List of Application Components that belong to this resource.</p> |
| `app_arn` | String | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_version_app_component
app_version_app_component = provider.resiliencehub.App_version_app_component {
    type = "value"  # <p>Type of Application Component. For more information about the types of Application Component, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/AppComponent.grouping.html">Grouping resources in an AppComponent</a>.</p>
    name = "value"  # <p>Name of the Application Component.</p>
    app_arn = "value"  # <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p>
}

# Access app_version_app_component outputs
app_version_app_component_id = app_version_app_component.id
app_version_app_component_app_version = app_version_app_component.app_version
app_version_app_component_app_component = app_version_app_component.app_component
app_version_app_component_app_arn = app_version_app_component.app_arn
```

---


### App

App resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_arn` | String |  | <p>Amazon Resource Name (ARN) of the resiliency policy. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:resiliency-policy/<code>policy-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags assigned to the resource. A tag is a label that you assign to an Amazon Web Services resource. 
Each tag consists of a key/value pair.</p> |
| `assessment_schedule` | String |  | <p> Assessment execution schedule with 'Daily' or 'Disabled' values. </p> |
| `aws_application_arn` | String |  | <p>Amazon Resource Name (ARN) of  Resource Groups group that is integrated with an AppRegistry application. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `description` | String |  | <p>The optional description for an app.</p> |
| `name` | String | ✅ | <p>Name of the application.</p> |
| `event_subscriptions` | Vec<String> |  | <p>The list of events you would like to subscribe and get notification for. Currently,
        Resilience Hub supports only <b>Drift detected</b> and
        <b>Scheduled assessment failure</b> events notification.</p> |
| `permission_model` | String |  | <p>Defines the roles and credentials that Resilience Hub would use while creating the
      application, importing its resources, and running an assessment.</p> |
| `client_token` | String |  | <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. 
You should not reuse the same client token for other API requests.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app` | String | <p>The specified application, returned as an object with details including compliance status,
      creation time, description, resiliency score, and more.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app
app = provider.resiliencehub.App {
    name = "value"  # <p>Name of the application.</p>
}

# Access app outputs
app_id = app.id
app_app = app.app
```

---


### App_input_source

AppInputSource resource

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


### Metrics_export

MetricsExport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics_export_id` | String | <p>Identifier for the metrics export task.</p> |
| `export_location` | String | <p>Specifies the name of the Amazon S3 bucket where the exported metrics is stored.</p> |
| `error_message` | String | <p>Explains the error that occurred while exporting the metrics.</p> |
| `status` | String | <p>Indicates the status of the metrics export task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metrics_export outputs
metrics_export_id = metrics_export.id
metrics_export_metrics_export_id = metrics_export.metrics_export_id
metrics_export_export_location = metrics_export.export_location
metrics_export_error_message = metrics_export.error_message
metrics_export_status = metrics_export.status
```

---


### App_version

AppVersion resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `additional_info` | HashMap<String, Vec<String>> |  | <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
         <note>
            <p>Currently, this parameter accepts a key-value mapping (in a string format) of only one failover region and one associated account.</p>
            <p>Key: <code>"failover-regions"</code>
            </p>
            <p>Value: <code>"[{"region":"&lt;REGION&gt;", "accounts":[{"id":"&lt;ACCOUNT_ID&gt;"}]}]"</code>
            </p>
         </note> |
| `app_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `additional_info` | HashMap<String, Vec<String>> | <p>Additional configuration parameters for an Resilience Hub application. If you want to implement <code>additionalInfo</code> through the Resilience Hub console rather than using an API call, see <a href="https://docs.aws.amazon.com/resilience-hub/latest/userguide/app-config-param.html">Configure the application configuration parameters</a>.</p>
         <note>
            <p>Currently, this parameter supports only failover region and account.</p>
         </note> |
| `app_arn` | String | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `app_version` | String | <p>Resilience Hub application version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access app_version outputs
app_version_id = app_version.id
app_version_additional_info = app_version.additional_info
app_version_app_arn = app_version.app_arn
app_version_app_version = app_version.app_version
```

---


### Recommendation_template

RecommendationTemplate resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name for the recommendation template.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags assigned to the resource. A tag is a label that you assign to an Amazon Web Services resource. 
Each tag consists of a key/value pair.</p> |
| `assessment_arn` | String | ✅ | <p>Amazon Resource Name (ARN) of the assessment. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `client_token` | String |  | <p>Used for an idempotency token. A client token is a unique, case-sensitive string of up to 64 ASCII characters. 
You should not reuse the same client token for other API requests.</p> |
| `recommendation_ids` | Vec<String> |  | <p>Identifiers for the recommendations used to create a recommendation template.</p> |
| `bucket_name` | String |  | <p>The name of the Amazon S3 bucket that will contain the recommendation template.</p> |
| `format` | String |  | <p>The format for the recommendation template.</p>
         <dl>
            <dt>CfnJson</dt>
            <dd>
               <p>The template is CloudFormation JSON.</p>
            </dd>
            <dt>CfnYaml</dt>
            <dd>
               <p>The template is CloudFormation YAML.</p>
            </dd>
         </dl> |
| `recommendation_types` | Vec<String> |  | <p>An array of strings that specify the recommendation template type or types.</p>
         <dl>
            <dt>Alarm</dt>
            <dd>
               <p>The template is an <a>AlarmRecommendation</a> template.</p>
            </dd>
            <dt>Sop</dt>
            <dd>
               <p>The template is a <a>SopRecommendation</a> template.</p>
            </dd>
            <dt>Test</dt>
            <dd>
               <p>The template is a <a>TestRecommendation</a> template.</p>
            </dd>
         </dl> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recommendation_template
recommendation_template = provider.resiliencehub.Recommendation_template {
    name = "value"  # <p>The name for the recommendation template.</p>
    assessment_arn = "value"  # <p>Amazon Resource Name (ARN) of the assessment. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p>
}

```

---


### Draft_app_version_resources_import_status

DraftAppVersionResourcesImportStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Status of the action.</p> |
| `status_change_time` | String | <p>The time when the status last changed.</p> |
| `error_details` | Vec<String> | <p>List of errors that were encountered while importing resources.</p> |
| `app_arn` | String | <p>Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: 
arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, 
see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">
                    Amazon Resource Names (ARNs)</a> in the 
                    <i>Amazon Web Services General Reference</i> guide.</p> |
| `app_version` | String | <p>The version of the application.</p> |
| `error_message` | String | <p>The error message returned for the resource request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access draft_app_version_resources_import_status outputs
draft_app_version_resources_import_status_id = draft_app_version_resources_import_status.id
draft_app_version_resources_import_status_status = draft_app_version_resources_import_status.status
draft_app_version_resources_import_status_status_change_time = draft_app_version_resources_import_status.status_change_time
draft_app_version_resources_import_status_error_details = draft_app_version_resources_import_status.error_details
draft_app_version_resources_import_status_app_arn = draft_app_version_resources_import_status.app_arn
draft_app_version_resources_import_status_app_version = draft_app_version_resources_import_status.app_version
draft_app_version_resources_import_status_error_message = draft_app_version_resources_import_status.error_message
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple app_version_template resources
app_version_template_0 = provider.resiliencehub.App_version_template {
}
app_version_template_1 = provider.resiliencehub.App_version_template {
}
app_version_template_2 = provider.resiliencehub.App_version_template {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    app_version_template = provider.resiliencehub.App_version_template {
    }
```

---

## Related Documentation

- [AWS Resiliencehub Documentation](https://docs.aws.amazon.com/resiliencehub/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
