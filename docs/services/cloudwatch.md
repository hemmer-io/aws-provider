# Cloudwatch Service



**Resources**: 18

---

## Overview

The cloudwatch service provides access to 18 resource types:

- [Metric_widget_image](#metric_widget_image) [R]
- [Anomaly_detector](#anomaly_detector) [CD]
- [Insight_rule](#insight_rule) [C]
- [Alarms](#alarms) [RD]
- [Metric_alarm](#metric_alarm) [C]
- [Alarms_for_metric](#alarms_for_metric) [R]
- [Metric_stream](#metric_stream) [CRD]
- [Dashboards](#dashboards) [D]
- [Metric_statistics](#metric_statistics) [R]
- [Managed_insight_rules](#managed_insight_rules) [C]
- [Alarm_contributors](#alarm_contributors) [R]
- [Insight_rule_report](#insight_rule_report) [R]
- [Insight_rules](#insight_rules) [RD]
- [Alarm_history](#alarm_history) [R]
- [Dashboard](#dashboard) [CR]
- [Composite_alarm](#composite_alarm) [C]
- [Metric_data](#metric_data) [CR]
- [Anomaly_detectors](#anomaly_detectors) [R]

---

## Resources


### Metric_widget_image

MetricWidgetImage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_widget_image` | String | <p>The image of the graph, in the output format specified. The output is
            base64-encoded.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metric_widget_image outputs
metric_widget_image_id = metric_widget_image.id
metric_widget_image_metric_widget_image = metric_widget_image.metric_widget_image
```

---


### Anomaly_detector

AnomalyDetector resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dimensions` | Vec<String> |  | <p>The metric dimensions to create the anomaly detection model for.</p> |
| `single_metric_anomaly_detector` | String |  | <p>A single metric anomaly detector to be created.</p>
         <p>When using <code>SingleMetricAnomalyDetector</code>, you cannot include the following
            parameters in the same operation:</p>
         <ul>
            <li>
               <p>
                  <code>Dimensions</code>
               </p>
            </li>
            <li>
               <p>
                  <code>MetricName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Namespace</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Stat</code>
               </p>
            </li>
            <li>
               <p>the <code>MetricMathAnomalyDetector</code> parameters of
                    <code>PutAnomalyDetectorInput</code>
               </p>
            </li>
         </ul>
         <p>Instead, specify the single metric anomaly detector attributes as part of the property
            <code>SingleMetricAnomalyDetector</code>.</p> |
| `stat` | String |  | <p>The statistic to use for the metric and the anomaly detection model.</p> |
| `metric_name` | String |  | <p>The name of the metric to create the anomaly detection model for.</p> |
| `configuration` | String |  | <p>The configuration specifies details about how the anomaly detection model is to be
            trained, including time ranges to exclude when training and updating the model. You can
            specify as many as 10 time ranges.</p>
         <p>The configuration can also include the time zone to use for the metric.</p> |
| `metric_characteristics` | String |  | <p>Use this object to include parameters to provide information about your metric to
            CloudWatch to help it build more accurate anomaly detection models.
            Currently, it includes the <code>PeriodicSpikes</code> parameter.</p> |
| `metric_math_anomaly_detector` | String |  | <p>The metric math anomaly detector to be created.</p>
         <p>When using <code>MetricMathAnomalyDetector</code>, you cannot include the following
            parameters in the same operation:</p>
         <ul>
            <li>
               <p>
                  <code>Dimensions</code>
               </p>
            </li>
            <li>
               <p>
                  <code>MetricName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Namespace</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Stat</code>
               </p>
            </li>
            <li>
               <p>the <code>SingleMetricAnomalyDetector</code> parameters of
                    <code>PutAnomalyDetectorInput</code>
               </p>
            </li>
         </ul>
         <p>Instead, specify the metric math anomaly detector attributes as part of the property
            <code>MetricMathAnomalyDetector</code>.</p> |
| `namespace` | String |  | <p>The namespace of the metric to create the anomaly detection model for.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create anomaly_detector
anomaly_detector = provider.cloudwatch.Anomaly_detector {
}

```

---


### Insight_rule

InsightRule resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apply_on_transformed_logs` | bool |  | <p>Specify <code>true</code> to have this rule evaluate log events after they have been transformed by  
            <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch-Logs-Transformation.html">Log transformation</a>. If you specify <code>true</code>, then the log events in log groups that have transformers will 
       be evaluated by Contributor Insights after being transformed. Log groups that don't have
        transformers will still have their original log events evaluated by Contributor Insights.</p>
         <p>The default is <code>false</code>
         </p>
         <note>
            <p>If a log group has a transformer, and transformation fails for some log events, those log events won't be evaluated by
        Contributor Insights. For information about investigating log transformation failures, see
            <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Transformation-Errors-Metrics.html">Transformation metrics and errors</a>.</p>
         </note> |
| `rule_state` | String |  | <p>The state of the rule. Valid values are ENABLED and DISABLED.</p> |
| `rule_name` | String | ✅ | <p>A unique name for the rule.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs to associate with the Contributor Insights rule. You can
            associate as many as 50 tags with a rule.</p>
         <p>Tags can help you organize and categorize your resources. You can also use them to
            scope user permissions, by granting a user permission to access or change only the
            resources that have certain tag values.</p>
         <p>To be able to associate tags with a rule, you must have the
            <code>cloudwatch:TagResource</code> permission in addition to the
            <code>cloudwatch:PutInsightRule</code> permission.</p>
         <p>If you are using this operation to update an existing Contributor Insights rule, any
            tags you specify in this parameter are ignored. To change the tags of an existing rule,
            use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html">TagResource</a>.</p> |
| `rule_definition` | String | ✅ | <p>The definition of the rule, as a JSON object. For details on the valid syntax, see
            <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-RuleSyntax.html">Contributor Insights Rule Syntax</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create insight_rule
insight_rule = provider.cloudwatch.Insight_rule {
    rule_name = "value"  # <p>A unique name for the rule.</p>
    rule_definition = "value"  # <p>The definition of the rule, as a JSON object. For details on the valid syntax, see
            <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-RuleSyntax.html">Contributor Insights Rule Syntax</a>.</p>
}

```

---


### Alarms

Alarms resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `composite_alarms` | Vec<String> | <p>The information about any composite alarms returned by the operation.</p> |
| `next_token` | String | <p>The token that marks the start of the next batch of returned results.</p> |
| `metric_alarms` | Vec<String> | <p>The information about any metric alarms returned by the operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access alarms outputs
alarms_id = alarms.id
alarms_composite_alarms = alarms.composite_alarms
alarms_next_token = alarms.next_token
alarms_metric_alarms = alarms.metric_alarms
```

---


### Metric_alarm

MetricAlarm resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alarm_name` | String | ✅ | <p>The name for the alarm. This name must be unique within the Region.</p>
         <p>The name must contain only UTF-8 characters, and can't contain ASCII control
            characters</p> |
| `ok_actions` | Vec<String> |  | <p>The actions to execute when this alarm transitions to an <code>OK</code> state from
            any other state. Each action is specified as an Amazon Resource Name (ARN). Valid
            values:</p>
         <p>
            <b>EC2 actions:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:stop</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:terminate</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:reboot</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:recover</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Stop/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Terminate/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Reboot/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Recover/1.0</code>
               </p>
            </li>
         </ul>
         <p>
            <b>Autoscaling action:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i>:autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>Lambda actions:</b>
         </p>
         <ul>
            <li>
               <p>Invoke the latest version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a specific version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>version-number</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a function by using an alias Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>alias-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>SNS notification action:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>SSM integration actions:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:ssm:<i>region</i>:<i>account-id</i>:opsitem:<i>severity</i>#CATEGORY=<i>category-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:ssm-incidents::<i>account-id</i>:responseplan/<i>response-plan-name</i>
                  </code>
               </p>
            </li>
         </ul> |
| `actions_enabled` | bool |  | <p>Indicates whether actions should be executed during any changes to the alarm state.
            The default is <code>TRUE</code>.</p> |
| `insufficient_data_actions` | Vec<String> |  | <p>The actions to execute when this alarm transitions to the
            <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified
            as an Amazon Resource Name (ARN). Valid values:</p>
         <p>
            <b>EC2 actions:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:stop</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:terminate</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:reboot</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:recover</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Stop/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Terminate/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Reboot/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Recover/1.0</code>
               </p>
            </li>
         </ul>
         <p>
            <b>Autoscaling action:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i>:autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>Lambda actions:</b>
         </p>
         <ul>
            <li>
               <p>Invoke the latest version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a specific version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>version-number</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a function by using an alias Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>alias-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>SNS notification action:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>SSM integration actions:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:ssm:<i>region</i>:<i>account-id</i>:opsitem:<i>severity</i>#CATEGORY=<i>category-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:ssm-incidents::<i>account-id</i>:responseplan/<i>response-plan-name</i>
                  </code>
               </p>
            </li>
         </ul> |
| `alarm_description` | String |  | <p>The description for the alarm.</p> |
| `namespace` | String |  | <p>The namespace for the metric associated specified in
            <code>MetricName</code>.</p> |
| `dimensions` | Vec<String> |  | <p>The dimensions for the metric specified in <code>MetricName</code>.</p> |
| `threshold` | f64 |  | <p>The value against which the specified statistic is compared.</p>
         <p>This parameter is required for alarms based on static thresholds, but should not be
            used for alarms based on anomaly detection models.</p> |
| `statistic` | String |  | <p>The statistic for the metric specified in <code>MetricName</code>, other than
            percentile. For percentile statistics, use <code>ExtendedStatistic</code>. When you call
            <code>PutMetricAlarm</code> and specify a <code>MetricName</code>, you must specify
            either <code>Statistic</code> or <code>ExtendedStatistic,</code> but not both.</p> |
| `evaluate_low_sample_count_percentile` | String |  | <p> Used only for alarms based on percentiles. If you specify <code>ignore</code>, the
            alarm state does not change during periods with too few data points to be statistically
            significant. If you specify <code>evaluate</code> or omit this parameter, the alarm is
            always evaluated and possibly changes state no matter how many data points are
            available. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#percentiles-with-low-samples">Percentile-Based CloudWatch Alarms and Low Data Samples</a>.</p>
         <p>Valid Values: <code>evaluate | ignore</code>
         </p> |
| `treat_missing_data` | String |  | <p> Sets how this alarm is to handle missing data points. If
            <code>TreatMissingData</code> is omitted, the default behavior of
            <code>missing</code> is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarms-and-missing-data">Configuring How CloudWatch Alarms Treats Missing Data</a>.</p>
         <p>Valid Values: <code>breaching | notBreaching | ignore | missing</code>
         </p>
         <note>
            <p>Alarms that evaluate metrics in the <code>AWS/DynamoDB</code> namespace always
                <code>ignore</code> missing data even if you choose a different option for
                <code>TreatMissingData</code>. When an <code>AWS/DynamoDB</code> metric has
                missing data, alarms that evaluate that metric remain in their current state.</p>
         </note> |
| `metric_name` | String |  | <p>The name for the metric associated with the alarm. For each
            <code>PutMetricAlarm</code> operation, you must specify either
            <code>MetricName</code> or a <code>Metrics</code> array.</p>
         <p>If you are creating an alarm based on a math expression, you cannot specify this
            parameter, or any of the <code>Namespace</code>, <code>Dimensions</code>,
            <code>Period</code>, <code>Unit</code>, <code>Statistic</code>, or
            <code>ExtendedStatistic</code> parameters. Instead, you specify all this information
            in the <code>Metrics</code> array.</p> |
| `alarm_actions` | Vec<String> |  | <p>The actions to execute when this alarm transitions to the <code>ALARM</code> state
            from any other state. Each action is specified as an Amazon Resource Name (ARN). Valid
            values:</p>
         <p>
            <b>EC2 actions:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:stop</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:terminate</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:reboot</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:automate:<i>region</i>:ec2:recover</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Stop/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Terminate/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Reboot/1.0</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Recover/1.0</code>
               </p>
            </li>
         </ul>
         <p>
            <b>Autoscaling action:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i>:autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>Lambda actions:</b>
         </p>
         <ul>
            <li>
               <p>Invoke the latest version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a specific version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>version-number</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a function by using an alias Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>alias-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>SNS notification action:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>SSM integration actions:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:ssm:<i>region</i>:<i>account-id</i>:opsitem:<i>severity</i>#CATEGORY=<i>category-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:ssm-incidents::<i>account-id</i>:responseplan/<i>response-plan-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>Start a Amazon Q Developer operational investigation</b>
         </p>
         <p>
            <code>arn:aws:aiops:<i>region</i>:<i>account-id</i>:investigation-group:<i>investigation-group-id</i>
            </code>
         </p> |
| `evaluation_periods` | i64 | ✅ | <p>The number of periods over which data is compared to the specified threshold. If
            you are setting an alarm that requires that a number of consecutive data points be
            breaching to trigger the alarm, this value specifies that number. If you are setting an
            "M out of N" alarm, this value is the N.</p> |
| `comparison_operator` | String | ✅ | <p> The arithmetic operation to use when comparing the specified statistic and
            threshold. The specified statistic value is used as the first operand.</p>
         <p>The values <code>LessThanLowerOrGreaterThanUpperThreshold</code>,
            <code>LessThanLowerThreshold</code>, and <code>GreaterThanUpperThreshold</code> are
            used only for alarms based on anomaly detection models.</p> |
| `metrics` | Vec<String> |  | <p>An array of <code>MetricDataQuery</code> structures that enable you to create an alarm
            based on the result of a metric math expression. For each <code>PutMetricAlarm</code>
            operation, you must specify either <code>MetricName</code> or a <code>Metrics</code>
            array.</p>
         <p>Each item in the <code>Metrics</code> array either retrieves a metric or performs a
            math expression.</p>
         <p>One item in the <code>Metrics</code> array is the expression that the alarm watches.
            You designate this expression by setting <code>ReturnData</code> to true for this object
            in the array. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_MetricDataQuery.html">MetricDataQuery</a>.</p>
         <p>If you use the <code>Metrics</code> parameter, you cannot include the
            <code>Namespace</code>, <code>MetricName</code>, <code>Dimensions</code>,
            <code>Period</code>, <code>Unit</code>, <code>Statistic</code>, or
            <code>ExtendedStatistic</code> parameters of <code>PutMetricAlarm</code> in the same
            operation. Instead, you retrieve the metrics you are using in your math expression as
            part of the <code>Metrics</code> array.</p> |
| `datapoints_to_alarm` | i64 |  | <p>The number of data points that must be breaching to trigger the alarm. This is used
            only if you are setting an "M out of N" alarm. In that case, this value is the M. For
            more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarm-evaluation">Evaluating an Alarm</a> in the <i>Amazon CloudWatch User
            Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs to associate with the alarm. You can associate as many as
            50 tags with an alarm. To be able to associate tags with the alarm when you create the
            alarm, you must have the <code>cloudwatch:TagResource</code> permission.</p>
         <p>Tags can help you organize and categorize your resources. You can also use them to
            scope user permissions by granting a user permission to access or change only resources
            with certain tag values.</p>
         <p>If you are using this operation to update an existing alarm, any tags you specify in
            this parameter are ignored. To change the tags of an existing alarm, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html">TagResource</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_UntagResource.html">UntagResource</a>.</p>
         <p>To use this field to set tags for an alarm when you create it, you must be signed on
            with both the <code>cloudwatch:PutMetricAlarm</code> and
            <code>cloudwatch:TagResource</code> permissions.</p> |
| `threshold_metric_id` | String |  | <p>If this is an alarm based on an anomaly detection model, make this value match the ID
            of the <code>ANOMALY_DETECTION_BAND</code> function.</p>
         <p>For an example of how to use this parameter, see the <b>Anomaly
            Detection Model Alarm</b> example on this page.</p>
         <p>If your alarm uses this parameter, it cannot have Auto Scaling actions.</p> |
| `extended_statistic` | String |  | <p>The extended statistic for the metric specified in <code>MetricName</code>. When
            you call <code>PutMetricAlarm</code> and specify a <code>MetricName</code>, you must
            specify either <code>Statistic</code> or <code>ExtendedStatistic</code> but not
            both.</p>
         <p>If you specify <code>ExtendedStatistic</code>, the following are valid values:</p>
         <ul>
            <li>
               <p>
                  <code>p90</code>
               </p>
            </li>
            <li>
               <p>
                  <code>tm90</code>
               </p>
            </li>
            <li>
               <p>
                  <code>tc90</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ts90</code>
               </p>
            </li>
            <li>
               <p>
                  <code>wm90</code>
               </p>
            </li>
            <li>
               <p>
                  <code>IQM</code>
               </p>
            </li>
            <li>
               <p>
                  <code>PR(<i>n</i>:<i>m</i>)</code> where n and m
                    are values of the metric</p>
            </li>
            <li>
               <p>
                  <code>TC(<i>X</i>%:<i>X</i>%)</code> where X is
                    between 10 and 90 inclusive.</p>
            </li>
            <li>
               <p>
                  <code>TM(<i>X</i>%:<i>X</i>%)</code> where X is
                    between 10 and 90 inclusive.</p>
            </li>
            <li>
               <p>
                  <code>TS(<i>X</i>%:<i>X</i>%)</code> where X is
                    between 10 and 90 inclusive.</p>
            </li>
            <li>
               <p>
                  <code>WM(<i>X</i>%:<i>X</i>%)</code> where X is
                    between 10 and 90 inclusive.</p>
            </li>
         </ul>
         <p>For more information about these extended statistics, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html">CloudWatch statistics definitions</a>.</p> |
| `period` | i64 |  | <p>The length, in seconds, used each time the metric specified in
            <code>MetricName</code> is evaluated. Valid values are 10, 20, 30, and any multiple of
            60.</p>
         <p>
            <code>Period</code> is required for alarms based on static thresholds. If you are
            creating an alarm based on a metric math expression, you specify the period for each
            metric within the objects in the <code>Metrics</code> array.</p>
         <p>Be sure to specify 10, 20, or 30 only for metrics that are stored by a
            <code>PutMetricData</code> call with a <code>StorageResolution</code> of 1. If you
            specify a period of 10, 20, or 30 for a metric that does not have sub-minute resolution, the
            alarm still attempts to gather data at the period rate that you specify. In this case,
            it does not receive data for the attempts that do not correspond to a one-minute data
            resolution, and the alarm might often lapse into INSUFFICENT_DATA status. Specifying 10, 20,
            or 30 also sets this alarm as a high-resolution alarm, which has a higher charge than
            other alarms. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch
                Pricing</a>.</p>
         <p>An alarm's total current evaluation period can be no longer than seven days, so
            <code>Period</code> multiplied by <code>EvaluationPeriods</code> can't be more than
            604,800 seconds. For alarms with a period of less than one hour (3,600 seconds), the total evaluation period can't be longer than one day (86,400 seconds).</p> |
| `unit` | String |  | <p>The unit of measure for the statistic. For example, the units for the Amazon EC2
            NetworkIn metric are Bytes because NetworkIn tracks the number of bytes that an instance
            receives on all network interfaces. You can also specify a unit when you create a custom
            metric. Units help provide conceptual meaning to your data. Metric data points that
            specify a unit of measure, such as Percent, are aggregated separately. If you are
            creating an alarm based on a metric math expression, you can specify the unit for each
            metric (if needed) within the objects in the <code>Metrics</code> array.</p>
         <p>If you don't specify <code>Unit</code>, CloudWatch retrieves all unit types that
            have been published for the metric and attempts to evaluate the alarm. Usually, metrics
            are published with only one unit, so the alarm works as intended.</p>
         <p>However, if the metric is published with multiple types of units and you don't
            specify a unit, the alarm's behavior is not defined and it behaves
            unpredictably.</p>
         <p>We recommend omitting <code>Unit</code> so that you don't inadvertently specify an
            incorrect unit that is not published for this metric. Doing so causes the alarm to be
            stuck in the <code>INSUFFICIENT DATA</code> state.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metric_alarm
metric_alarm = provider.cloudwatch.Metric_alarm {
    alarm_name = "value"  # <p>The name for the alarm. This name must be unique within the Region.</p>
         <p>The name must contain only UTF-8 characters, and can't contain ASCII control
            characters</p>
    evaluation_periods = "value"  # <p>The number of periods over which data is compared to the specified threshold. If
            you are setting an alarm that requires that a number of consecutive data points be
            breaching to trigger the alarm, this value specifies that number. If you are setting an
            "M out of N" alarm, this value is the N.</p>
    comparison_operator = "value"  # <p> The arithmetic operation to use when comparing the specified statistic and
            threshold. The specified statistic value is used as the first operand.</p>
         <p>The values <code>LessThanLowerOrGreaterThanUpperThreshold</code>,
            <code>LessThanLowerThreshold</code>, and <code>GreaterThanUpperThreshold</code> are
            used only for alarms based on anomaly detection models.</p>
}

```

---


### Alarms_for_metric

AlarmsForMetric resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_alarms` | Vec<String> | <p>The information for each alarm with the specified metric.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access alarms_for_metric outputs
alarms_for_metric_id = alarms_for_metric.id
alarms_for_metric_metric_alarms = alarms_for_metric.metric_alarms
```

---


### Metric_stream

MetricStream resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `include_filters` | Vec<String> |  | <p>If you specify this parameter, the stream sends only the metrics from the metric
            namespaces that you specify here.</p>
         <p>You cannot include <code>IncludeFilters</code> and <code>ExcludeFilters</code> in the
            same operation.</p> |
| `include_linked_accounts_metrics` | bool |  | <p>If you are creating a metric stream in a monitoring account, specify <code>true</code>
            to include metrics from source accounts in the metric stream.</p> |
| `name` | String | ✅ | <p>If you are creating a new metric stream, this is the name for the new stream. The name
            must be different than the names of other metric streams in this account and
            Region.</p>
         <p>If you are updating a metric stream, specify the name of that stream here.</p>
         <p>Valid characters are A-Z, a-z, 0-9, "-" and "_".</p> |
| `output_format` | String | ✅ | <p>The output format for the stream. Valid values are <code>json</code>,
            <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>. For more
            information about metric stream output formats, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html">
                Metric streams output formats</a>.</p> |
| `firehose_arn` | String | ✅ | <p>The ARN of the Amazon Kinesis Data Firehose delivery stream to use for this metric
            stream. This Amazon Kinesis Data Firehose delivery stream must already exist and must be
            in the same account as the metric stream.</p> |
| `role_arn` | String | ✅ | <p>The ARN of an IAM role that this metric stream will use to access Amazon Kinesis Data
            Firehose resources. This IAM role must already exist and must be in the same account as
            the metric stream. This IAM role must include the following permissions:</p>
         <ul>
            <li>
               <p>firehose:PutRecord</p>
            </li>
            <li>
               <p>firehose:PutRecordBatch</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs to associate with the metric stream. You can associate as
            many as 50 tags with a metric stream.</p>
         <p>Tags can help you organize and categorize your resources. You can also use them to
            scope user permissions by granting a user permission to access or change only resources
            with certain tag values.</p>
         <p>You can use this parameter only when you are creating a new metric stream. If you are
            using this operation to update an existing metric stream, any tags you specify in this
            parameter are ignored. To change the tags of an existing metric stream, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html">TagResource</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_UntagResource.html">UntagResource</a>.</p> |
| `statistics_configurations` | Vec<String> |  | <p>By default, a metric stream always sends the <code>MAX</code>, <code>MIN</code>,
            <code>SUM</code>, and <code>SAMPLECOUNT</code> statistics for each metric that is
            streamed. You can use this parameter to have the metric stream also send additional
            statistics in the stream. This array can have up to 100 members.</p>
         <p>For each entry in this array, you specify one or more metrics and the list of
            additional statistics to stream for those metrics. The additional statistics that you
            can stream depend on the stream's <code>OutputFormat</code>. If the
            <code>OutputFormat</code> is <code>json</code>, you can stream any additional
            statistic that is supported by CloudWatch, listed in <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html">
                CloudWatch statistics definitions</a>. If the <code>OutputFormat</code>
            is <code>opentelemetry1.0</code> or <code>opentelemetry0.7</code>, you can stream
            percentile statistics such as p95, p99.9, and so on.</p> |
| `exclude_filters` | Vec<String> |  | <p>If you specify this parameter, the stream sends metrics from all metric namespaces
            except for the namespaces that you specify here.</p>
         <p>You cannot include <code>ExcludeFilters</code> and <code>IncludeFilters</code> in the
            same operation.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the metric stream.</p> |
| `include_filters` | Vec<String> | <p>If this array of metric namespaces is present, then these namespaces are the only
            metric namespaces that are streamed by this metric stream.</p> |
| `state` | String | <p>The state of the metric stream. The possible values are <code>running</code> and
            <code>stopped</code>.</p> |
| `creation_date` | String | <p>The date that the metric stream was created.</p> |
| `exclude_filters` | Vec<String> | <p>If this array of metric namespaces is present, then these namespaces are the only
            metric namespaces that are not streamed by this metric stream. In this case, all other
            metric namespaces in the account are streamed by this metric stream.</p> |
| `arn` | String | <p>The ARN of the metric stream.</p> |
| `include_linked_accounts_metrics` | bool | <p>If this is <code>true</code> and this metric stream is in a monitoring account, then
            the stream includes metrics from source accounts that the monitoring account is linked
            to.</p> |
| `statistics_configurations` | Vec<String> | <p>Each entry in this array displays information about one or more metrics that include
            additional statistics in the metric stream. For more information about the additional
            statistics, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html">
                CloudWatch statistics definitions</a>. </p> |
| `last_update_date` | String | <p>The date of the most recent update to the metric stream's configuration.</p> |
| `role_arn` | String | <p>The ARN of the IAM role that is used by this metric stream.</p> |
| `output_format` | String | <p>The output format for the stream. Valid values are <code>json</code>,
            <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>. For more
            information about metric stream output formats, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html">Metric streams output formats</a>.</p> |
| `firehose_arn` | String | <p>The ARN of the Amazon Kinesis Data Firehose delivery stream that is used by this
            metric stream.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metric_stream
metric_stream = provider.cloudwatch.Metric_stream {
    name = "value"  # <p>If you are creating a new metric stream, this is the name for the new stream. The name
            must be different than the names of other metric streams in this account and
            Region.</p>
         <p>If you are updating a metric stream, specify the name of that stream here.</p>
         <p>Valid characters are A-Z, a-z, 0-9, "-" and "_".</p>
    output_format = "value"  # <p>The output format for the stream. Valid values are <code>json</code>,
            <code>opentelemetry1.0</code>, and <code>opentelemetry0.7</code>. For more
            information about metric stream output formats, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html">
                Metric streams output formats</a>.</p>
    firehose_arn = "value"  # <p>The ARN of the Amazon Kinesis Data Firehose delivery stream to use for this metric
            stream. This Amazon Kinesis Data Firehose delivery stream must already exist and must be
            in the same account as the metric stream.</p>
    role_arn = "value"  # <p>The ARN of an IAM role that this metric stream will use to access Amazon Kinesis Data
            Firehose resources. This IAM role must already exist and must be in the same account as
            the metric stream. This IAM role must include the following permissions:</p>
         <ul>
            <li>
               <p>firehose:PutRecord</p>
            </li>
            <li>
               <p>firehose:PutRecordBatch</p>
            </li>
         </ul>
}

# Access metric_stream outputs
metric_stream_id = metric_stream.id
metric_stream_name = metric_stream.name
metric_stream_include_filters = metric_stream.include_filters
metric_stream_state = metric_stream.state
metric_stream_creation_date = metric_stream.creation_date
metric_stream_exclude_filters = metric_stream.exclude_filters
metric_stream_arn = metric_stream.arn
metric_stream_include_linked_accounts_metrics = metric_stream.include_linked_accounts_metrics
metric_stream_statistics_configurations = metric_stream.statistics_configurations
metric_stream_last_update_date = metric_stream.last_update_date
metric_stream_role_arn = metric_stream.role_arn
metric_stream_output_format = metric_stream.output_format
metric_stream_firehose_arn = metric_stream.firehose_arn
```

---


### Dashboards

Dashboards resource

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


### Metric_statistics

MetricStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `label` | String | <p>A label for the specified metric.</p> |
| `datapoints` | Vec<String> | <p>The data points for the specified metric.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metric_statistics outputs
metric_statistics_id = metric_statistics.id
metric_statistics_label = metric_statistics.label
metric_statistics_datapoints = metric_statistics.datapoints
```

---


### Managed_insight_rules

ManagedInsightRules resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `managed_rules` | Vec<String> | ✅ | <p> A list of <code>ManagedRules</code> to enable. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create managed_insight_rules
managed_insight_rules = provider.cloudwatch.Managed_insight_rules {
    managed_rules = "value"  # <p> A list of <code>ManagedRules</code> to enable. </p>
}

```

---


### Alarm_contributors

AlarmContributors resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token that marks the start of the next batch of returned results.</p> |
| `alarm_contributors` | Vec<String> | <p>A list of alarm contributors that provide details about the individual time series contributing to the alarm's state.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access alarm_contributors outputs
alarm_contributors_id = alarm_contributors.id
alarm_contributors_next_token = alarm_contributors.next_token
alarm_contributors_alarm_contributors = alarm_contributors.alarm_contributors
```

---


### Insight_rule_report

InsightRuleReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_datapoints` | Vec<String> | <p>A time series of metric data points that matches the time period in the rule
            request.</p> |
| `aggregate_value` | f64 | <p>The sum of the values from all individual contributors that match the rule.</p> |
| `approximate_unique_count` | i64 | <p>An approximate count of the unique contributors found by this rule in this time
            period.</p> |
| `contributors` | Vec<String> | <p>An array of the unique contributors found by this rule in this time period. If the
            rule contains multiple keys, each combination of values for the keys counts as a unique
            contributor.</p> |
| `key_labels` | Vec<String> | <p>An array of the strings used as the keys for this rule. The keys are the dimensions
            used to classify contributors. If the rule contains more than one key, then each unique
            combination of values for the keys is counted as a unique contributor.</p> |
| `aggregation_statistic` | String | <p>Specifies whether this rule aggregates contributor data by COUNT or SUM.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight_rule_report outputs
insight_rule_report_id = insight_rule_report.id
insight_rule_report_metric_datapoints = insight_rule_report.metric_datapoints
insight_rule_report_aggregate_value = insight_rule_report.aggregate_value
insight_rule_report_approximate_unique_count = insight_rule_report.approximate_unique_count
insight_rule_report_contributors = insight_rule_report.contributors
insight_rule_report_key_labels = insight_rule_report.key_labels
insight_rule_report_aggregation_statistic = insight_rule_report.aggregation_statistic
```

---


### Insight_rules

InsightRules resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insight_rules` | Vec<String> | <p>The rules returned by the operation.</p> |
| `next_token` | String | <p>If this parameter is present, it is a token that marks the start of the next batch of
            returned results. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight_rules outputs
insight_rules_id = insight_rules.id
insight_rules_insight_rules = insight_rules.insight_rules
insight_rules_next_token = insight_rules.next_token
```

---


### Alarm_history

AlarmHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alarm_history_items` | Vec<String> | <p>The alarm histories, in JSON format.</p> |
| `next_token` | String | <p>The token that marks the start of the next batch of returned results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access alarm_history outputs
alarm_history_id = alarm_history.id
alarm_history_alarm_history_items = alarm_history.alarm_history_items
alarm_history_next_token = alarm_history.next_token
```

---


### Dashboard

Dashboard resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dashboard_body` | String | ✅ | <p>The detailed information about the dashboard in JSON format, including the widgets
            to include and their location on the dashboard. This parameter is required.</p>
         <p>For more information about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>.</p> |
| `dashboard_name` | String | ✅ | <p>The name of the dashboard. If a dashboard with this name already exists, this call
            modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is
            created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and
            "_". This parameter is required.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dashboard_name` | String | <p>The name of the dashboard.</p> |
| `dashboard_arn` | String | <p>The Amazon Resource Name (ARN) of the dashboard.</p> |
| `dashboard_body` | String | <p>The detailed information about the dashboard, including what widgets are included
            and their location on the dashboard. For more information about the
            <code>DashboardBody</code> syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dashboard
dashboard = provider.cloudwatch.Dashboard {
    dashboard_body = "value"  # <p>The detailed information about the dashboard in JSON format, including the widgets
            to include and their location on the dashboard. This parameter is required.</p>
         <p>For more information about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>.</p>
    dashboard_name = "value"  # <p>The name of the dashboard. If a dashboard with this name already exists, this call
            modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is
            created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and
            "_". This parameter is required.</p>
}

# Access dashboard outputs
dashboard_id = dashboard.id
dashboard_dashboard_name = dashboard.dashboard_name
dashboard_dashboard_arn = dashboard.dashboard_arn
dashboard_dashboard_body = dashboard.dashboard_body
```

---


### Composite_alarm

CompositeAlarm resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alarm_actions` | Vec<String> |  | <p>The actions to execute when this alarm transitions to the <code>ALARM</code> state
            from any other state. Each action is specified as an Amazon Resource Name
            (ARN).</p>
         <p>Valid Values: ]</p>
         <p>
            <b>Amazon SNS actions:</b>
         </p>
         <p>
            <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i>
            </code>
         </p>
         <p>
            <b>Lambda actions:</b>
         </p>
         <ul>
            <li>
               <p>Invoke the latest version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a specific version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>version-number</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a function by using an alias Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>alias-name</i>
                  </code>
               </p>
            </li>
         </ul>
         <p>
            <b>Systems Manager actions:</b>
         </p>
         <p>
            <code>arn:aws:ssm:<i>region</i>:<i>account-id</i>:opsitem:<i>severity</i>
            </code>
         </p>
         <p>
            <b>Start a Amazon Q Developer operational investigation</b>
         </p>
         <p>
            <code>arn:aws:aiops:<i>region</i>:<i>account-id</i>:investigation-group:<i>investigation-group-id</i>
            </code>
         </p> |
| `actions_enabled` | bool |  | <p>Indicates whether actions should be executed during any changes to the alarm state of
            the composite alarm. The default is <code>TRUE</code>.</p> |
| `actions_suppressor_extension_period` | i64 |  | <p> The maximum time in seconds that the composite alarm waits after suppressor alarm
            goes out of the <code>ALARM</code> state. After this time, the composite alarm performs
            its actions. </p>
         <important>
            <p>
               <code>ExtensionPeriod</code> is required only when <code>ActionsSuppressor</code> is
                specified. </p>
         </important> |
| `ok_actions` | Vec<String> |  | <p>The actions to execute when this alarm transitions to an <code>OK</code> state from
            any other state. Each action is specified as an Amazon Resource Name (ARN).</p>
         <p>Valid Values: ]</p>
         <p>
            <b>Amazon SNS actions:</b>
         </p>
         <p>
            <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i>
            </code>
         </p>
         <p>
            <b>Lambda actions:</b>
         </p>
         <ul>
            <li>
               <p>Invoke the latest version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a specific version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>version-number</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a function by using an alias Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>alias-name</i>
                  </code>
               </p>
            </li>
         </ul> |
| `alarm_name` | String | ✅ | <p>The name for the composite alarm. This name must be unique within the
            Region.</p> |
| `alarm_description` | String |  | <p>The description for the composite alarm.</p> |
| `alarm_rule` | String | ✅ | <p>An expression that specifies which other alarms are to be evaluated to determine this
            composite alarm's state. For each alarm that you reference, you designate a function
            that specifies whether that alarm needs to be in ALARM state, OK state, or
            INSUFFICIENT_DATA state. You can use operators (AND, OR and NOT) to combine multiple
            functions in a single expression. You can use parenthesis to logically group the
            functions in your expression.</p>
         <p>You can use either alarm names or ARNs to reference the other alarms that are to be
            evaluated.</p>
         <p>Functions can include the following:</p>
         <ul>
            <li>
               <p>
                  <code>ALARM("<i>alarm-name</i> or
                    <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in
                    ALARM state.</p>
            </li>
            <li>
               <p>
                  <code>OK("<i>alarm-name</i> or
                    <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in OK
                    state.</p>
            </li>
            <li>
               <p>
                  <code>INSUFFICIENT_DATA("<i>alarm-name</i> or
                    <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in
                    INSUFFICIENT_DATA state.</p>
            </li>
            <li>
               <p>
                  <code>TRUE</code> always evaluates to TRUE.</p>
            </li>
            <li>
               <p>
                  <code>FALSE</code> always evaluates to FALSE.</p>
            </li>
         </ul>
         <p>TRUE and FALSE are useful for testing a complex <code>AlarmRule</code> structure, and
            for testing your alarm actions.</p>
         <p>Alarm names specified in <code>AlarmRule</code> can be surrounded with double-quotes
            ("), but do not have to be.</p>
         <p>The following are some examples of <code>AlarmRule</code>:</p>
         <ul>
            <li>
               <p>
                  <code>ALARM(CPUUtilizationTooHigh) AND ALARM(DiskReadOpsTooHigh)</code>
                    specifies that the composite alarm goes into ALARM state only if both
                    CPUUtilizationTooHigh and DiskReadOpsTooHigh alarms are in ALARM state.</p>
            </li>
            <li>
               <p>
                  <code>ALARM(CPUUtilizationTooHigh) AND NOT ALARM(DeploymentInProgress)</code>
                    specifies that the alarm goes to ALARM state if CPUUtilizationTooHigh is in
                    ALARM state and DeploymentInProgress is not in ALARM state. This example reduces
                    alarm noise during a known deployment window.</p>
            </li>
            <li>
               <p>
                  <code>(ALARM(CPUUtilizationTooHigh) OR ALARM(DiskReadOpsTooHigh)) AND
                    OK(NetworkOutTooHigh)</code> goes into ALARM state if CPUUtilizationTooHigh
                    OR DiskReadOpsTooHigh is in ALARM state, and if NetworkOutTooHigh is in OK
                    state. This provides another example of using a composite alarm to prevent
                    noise. This rule ensures that you are not notified with an alarm action on high
                    CPU or disk usage if a known network problem is also occurring.</p>
            </li>
         </ul>
         <p>The <code>AlarmRule</code> can specify as many as 100 "children" alarms. The
            <code>AlarmRule</code> expression can have as many as 500 elements. Elements are
            child alarms, TRUE or FALSE statements, and parentheses.</p> |
| `insufficient_data_actions` | Vec<String> |  | <p>The actions to execute when this alarm transitions to the
            <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified
            as an Amazon Resource Name (ARN).</p>
         <p>Valid Values: ]</p>
         <p>
            <b>Amazon SNS actions:</b>
         </p>
         <p>
            <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i>
            </code>
         </p>
         <p>
            <b>Lambda actions:</b>
         </p>
         <ul>
            <li>
               <p>Invoke the latest version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a specific version of a Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>version-number</i>
                  </code>
               </p>
            </li>
            <li>
               <p>Invoke a function by using an alias Lambda function:
                    <code>arn:aws:lambda:<i>region</i>:<i>account-id</i>:function:<i>function-name</i>:<i>alias-name</i>
                  </code>
               </p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs to associate with the alarm. You can associate as many as
            50 tags with an alarm. To be able to associate tags with the alarm when you create the
            alarm, you must have the <code>cloudwatch:TagResource</code> permission.</p>
         <p>Tags can help you organize and categorize your resources. You can also use them to
            scope user permissions by granting a user permission to access or change only resources
            with certain tag values.</p>
         <p>If you are using this operation to update an existing alarm, any tags you specify in
            this parameter are ignored. To change the tags of an existing alarm, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html">TagResource</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_UntagResource.html">UntagResource</a>.</p> |
| `actions_suppressor` | String |  | <p> Actions will be suppressed if the suppressor alarm is in the <code>ALARM</code>
            state. <code>ActionsSuppressor</code> can be an AlarmName or an Amazon Resource Name
            (ARN) from an existing alarm. </p> |
| `actions_suppressor_wait_period` | i64 |  | <p> The maximum time in seconds that the composite alarm waits for the suppressor alarm
            to go into the <code>ALARM</code> state. After this time, the composite alarm performs
            its actions. </p>
         <important>
            <p>
               <code>WaitPeriod</code> is required only when <code>ActionsSuppressor</code> is
                specified. </p>
         </important> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create composite_alarm
composite_alarm = provider.cloudwatch.Composite_alarm {
    alarm_name = "value"  # <p>The name for the composite alarm. This name must be unique within the
            Region.</p>
    alarm_rule = "value"  # <p>An expression that specifies which other alarms are to be evaluated to determine this
            composite alarm's state. For each alarm that you reference, you designate a function
            that specifies whether that alarm needs to be in ALARM state, OK state, or
            INSUFFICIENT_DATA state. You can use operators (AND, OR and NOT) to combine multiple
            functions in a single expression. You can use parenthesis to logically group the
            functions in your expression.</p>
         <p>You can use either alarm names or ARNs to reference the other alarms that are to be
            evaluated.</p>
         <p>Functions can include the following:</p>
         <ul>
            <li>
               <p>
                  <code>ALARM("<i>alarm-name</i> or
                    <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in
                    ALARM state.</p>
            </li>
            <li>
               <p>
                  <code>OK("<i>alarm-name</i> or
                    <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in OK
                    state.</p>
            </li>
            <li>
               <p>
                  <code>INSUFFICIENT_DATA("<i>alarm-name</i> or
                    <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in
                    INSUFFICIENT_DATA state.</p>
            </li>
            <li>
               <p>
                  <code>TRUE</code> always evaluates to TRUE.</p>
            </li>
            <li>
               <p>
                  <code>FALSE</code> always evaluates to FALSE.</p>
            </li>
         </ul>
         <p>TRUE and FALSE are useful for testing a complex <code>AlarmRule</code> structure, and
            for testing your alarm actions.</p>
         <p>Alarm names specified in <code>AlarmRule</code> can be surrounded with double-quotes
            ("), but do not have to be.</p>
         <p>The following are some examples of <code>AlarmRule</code>:</p>
         <ul>
            <li>
               <p>
                  <code>ALARM(CPUUtilizationTooHigh) AND ALARM(DiskReadOpsTooHigh)</code>
                    specifies that the composite alarm goes into ALARM state only if both
                    CPUUtilizationTooHigh and DiskReadOpsTooHigh alarms are in ALARM state.</p>
            </li>
            <li>
               <p>
                  <code>ALARM(CPUUtilizationTooHigh) AND NOT ALARM(DeploymentInProgress)</code>
                    specifies that the alarm goes to ALARM state if CPUUtilizationTooHigh is in
                    ALARM state and DeploymentInProgress is not in ALARM state. This example reduces
                    alarm noise during a known deployment window.</p>
            </li>
            <li>
               <p>
                  <code>(ALARM(CPUUtilizationTooHigh) OR ALARM(DiskReadOpsTooHigh)) AND
                    OK(NetworkOutTooHigh)</code> goes into ALARM state if CPUUtilizationTooHigh
                    OR DiskReadOpsTooHigh is in ALARM state, and if NetworkOutTooHigh is in OK
                    state. This provides another example of using a composite alarm to prevent
                    noise. This rule ensures that you are not notified with an alarm action on high
                    CPU or disk usage if a known network problem is also occurring.</p>
            </li>
         </ul>
         <p>The <code>AlarmRule</code> can specify as many as 100 "children" alarms. The
            <code>AlarmRule</code> expression can have as many as 500 elements. Elements are
            child alarms, TRUE or FALSE statements, and parentheses.</p>
}

```

---


### Metric_data

MetricData resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `namespace` | String | ✅ | <p>The namespace for the metric data. You can use ASCII characters for the namespace,
            except for control characters which are not supported.</p>
         <p>To avoid conflicts with Amazon Web Services service namespaces, you should not
            specify a namespace that begins with <code>AWS/</code>
         </p> |
| `strict_entity_validation` | bool |  | <p>Whether to accept valid metric data when an invalid entity is sent.</p>
         <ul>
            <li>
               <p>When set to <code>true</code>: Any validation error (for entity or metric 
                    data) will fail the entire request, and no data will be ingested. The failed 
                    operation will return a 400 result with the error.</p>
            </li>
            <li>
               <p>When set to <code>false</code>: Validation errors in the entity will not 
                    associate the metric with the entity, but the metric data will still be 
                    accepted and ingested. Validation errors in the metric data will fail the 
                    entire request, and no data will be ingested.</p>
               <p>In the case of an invalid entity, the operation will return a 
                    <code>200</code> status, but an additional response header will contain 
                    information about the validation errors. The new header, 
                    <code>X-Amzn-Failure-Message</code> is an enumeration of the following 
                    values:</p>
               <ul>
                  <li>
                     <p>
                        <code>InvalidEntity</code> - The provided entity is invalid.</p>
                  </li>
                  <li>
                     <p>
                        <code>InvalidKeyAttributes</code> - The provided
                            <code>KeyAttributes</code> of an entity is invalid.</p>
                  </li>
                  <li>
                     <p>
                        <code>InvalidAttributes</code> - The provided <code>Attributes</code>
                            of an entity is invalid.</p>
                  </li>
                  <li>
                     <p>
                        <code>InvalidTypeValue</code> - The provided <code>Type</code> in the
                            <code>KeyAttributes</code> of an entity is invalid.</p>
                  </li>
                  <li>
                     <p>
                        <code>EntitySizeTooLarge</code> - The number of 
                            <code>EntityMetricData</code> objects allowed is 2.</p>
                  </li>
                  <li>
                     <p>
                        <code>MissingRequiredFields</code> - There are missing required 
                            fields in the <code>KeyAttributes</code> for the provided
                            <code>Type</code>.</p>
                  </li>
               </ul>
               <p>For details of the requirements for specifying an entity, see 
                    <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/adding-your-own-related-telemetry.html">How 
                    to add related information to telemetry</a> in the 
                    <i>CloudWatch User Guide</i>.</p>
            </li>
         </ul>
         <p>This parameter is <i>required</i> when <code>EntityMetricData</code> is
            included.</p> |
| `entity_metric_data` | Vec<String> |  | <p>Data for metrics that contain associated entity information. You can include up to 
            two <code>EntityMetricData</code> objects, each of which can contain a single 
            <code>Entity</code> and associated metrics.</p>
         <p>The limit of metrics allowed, 1000, is the sum of both <code>EntityMetricData</code> 
            and <code>MetricData</code> metrics.</p> |
| `metric_data` | Vec<String> |  | <p>The data for the metrics. Use this parameter if your metrics do not contain
            associated entities. The array can include no more than 1000 metrics per call.</p>
         <p>The limit of metrics allowed, 1000, is the sum of both <code>EntityMetricData</code> 
            and <code>MetricData</code> metrics.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token that marks the next batch of returned results.</p> |
| `metric_data_results` | Vec<String> | <p>The metrics that are returned, including the metric name, namespace, and
            dimensions.</p> |
| `messages` | Vec<String> | <p>Contains a message about this <code>GetMetricData</code> operation, if the operation
            results in such a message. An example of a message that might be returned is
                <code>Maximum number of allowed metrics exceeded</code>. If there is a message, as
            much of the operation as possible is still executed.</p>
         <p>A message appears here only if it is related to the global <code>GetMetricData</code>
            operation. Any message about a specific metric returned by the operation appears in the
                <code>MetricDataResult</code> object returned for that metric.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metric_data
metric_data = provider.cloudwatch.Metric_data {
    namespace = "value"  # <p>The namespace for the metric data. You can use ASCII characters for the namespace,
            except for control characters which are not supported.</p>
         <p>To avoid conflicts with Amazon Web Services service namespaces, you should not
            specify a namespace that begins with <code>AWS/</code>
         </p>
}

# Access metric_data outputs
metric_data_id = metric_data.id
metric_data_next_token = metric_data.next_token
metric_data_metric_data_results = metric_data.metric_data_results
metric_data_messages = metric_data.messages
```

---


### Anomaly_detectors

AnomalyDetectors resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anomaly_detectors` | Vec<String> | <p>The list of anomaly detection models returned by the operation.</p> |
| `next_token` | String | <p>A token that you can use in a subsequent operation to retrieve the next set of
            results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access anomaly_detectors outputs
anomaly_detectors_id = anomaly_detectors.id
anomaly_detectors_anomaly_detectors = anomaly_detectors.anomaly_detectors
anomaly_detectors_next_token = anomaly_detectors.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple metric_widget_image resources
metric_widget_image_0 = provider.cloudwatch.Metric_widget_image {
}
metric_widget_image_1 = provider.cloudwatch.Metric_widget_image {
}
metric_widget_image_2 = provider.cloudwatch.Metric_widget_image {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    metric_widget_image = provider.cloudwatch.Metric_widget_image {
    }
```

---

## Related Documentation

- [AWS Cloudwatch Documentation](https://docs.aws.amazon.com/cloudwatch/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
