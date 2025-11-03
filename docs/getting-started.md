# Getting Started

This guide will help you get started using the Aws provider for Hemmer.

---

## Prerequisites

- Hemmer CLI installed
- Aws provider installed ([Installation Guide](installation.md))
- AWS credentials configured (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)

---

## Basic Usage

### 1. Initialize Provider

Create a new Rust project and add the provider dependency:

```bash
cargo new my-aws-app
cd my-aws-app
```

Add to `Cargo.toml`:

```toml
[dependencies]
hemmer-aws-provider = "*"
hemmer-core = "*"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

### 2. Basic Example

```kcl
# main.k
import aws

# Create provider instance
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create a property_value_history
property_value_history = provider.iottwinmaker.Property_value_history {
}

```

---

## Common Patterns

### Conditional Resource Creation

```kcl
# Only create resource if condition is met
if environment == "production":
    property_value_history = provider.iottwinmaker.Property_value_history {
        # configuration
    }
```

### Referencing Resource Outputs

```kcl
# Create a resource
property_value_history = provider.iottwinmaker.Property_value_history {
    # configuration
}

# Reference its outputs
output_value = property_value_history.id
```

---

## Available Services

This provider includes 394 services:

### 1. Iottwinmaker

**Resources**: 9

- Property_value_history [R]
- Component_type [CRUD]
- Sync_job [CRD]
- Metadata_transfer_job [CR]
- Pricing_plan [RU]
- Workspace [CRUD]
- Entity [CRUD]
- Property_value [R]
- Scene [CRUD]

📖 [Full iottwinmaker documentation](services/iottwinmaker.md)

### 2. Payment_cryptography

**Resources**: 5

- Certificate_signing_request [R]
- Parameters_for_export [R]
- Parameters_for_import [R]
- Public_key_certificate [R]
- Default_key_replication_regions [R]

📖 [Full payment_cryptography documentation](services/payment_cryptography.md)

### 3. Timestream_write

**Resources**: 4

- Endpoints [R]
- Batch_load_task [CR]
- Database [CRUD]
- Table [CRUD]

📖 [Full timestream_write documentation](services/timestream_write.md)

### 4. Workspaces_instances

**Resources**: 2

- Workspace_instance [CRD]
- Volume [CD]

📖 [Full workspaces_instances documentation](services/workspaces_instances.md)

### 5. Guardduty

**Resources**: 25

- Organization_configuration [RU]
- Remaining_free_trial_days [R]
- Malware_scans [R]
- Publishing_destination [CRUD]
- Filter [CRUD]
- Findings_statistics [R]
- Member_detectors [RU]
- Usage_statistics [R]
- Coverage_statistics [R]
- Trusted_entity_set [CRUD]
- Invitations_count [R]
- Invitations [D]
- Administrator_account [R]
- Detector [CRUD]
- Master_account [R]
- Sample_findings [C]
- Malware_protection_plan [CRUD]
- Threat_entity_set [CRUD]
- Findings [R]
- Ip_set [CRUD]
- Malware_scan_settings [RU]
- Members [CRD]
- Threat_intel_set [CRUD]
- Organization_statistics [R]
- Findings_feedback [U]

📖 [Full guardduty documentation](services/guardduty.md)

### 6. Resiliencehub

**Resources**: 14

- App_version_app_component [CRUD]
- App_version_resource [CRUD]
- App [CRUD]
- Metrics_export [R]
- Draft_app_version_template [C]
- Recommendation_template [CD]
- App_version_resources_resolution_status [R]
- App_version_template [R]
- Resiliency_policy [CRUD]
- Draft_app_version_resources_import_status [R]
- App_input_source [D]
- App_assessment [RD]
- App_version [RU]
- Resource_grouping_recommendation_task [R]

📖 [Full resiliencehub documentation](services/resiliencehub.md)

### 7. Sts

**Resources**: 4

- Federation_token [R]
- Access_key_info [R]
- Session_token [R]
- Caller_identity [R]

📖 [Full sts documentation](services/sts.md)

### 8. Lightsail

**Resources**: 89

- Distribution_bundle [U]
- Bucket [CUD]
- Disk_snapshots [R]
- Container_service_registry_login [C]
- Relational_database_from_snapshot [C]
- Bucket_access_keys [R]
- Contact_methods [R]
- Container_service_metric_data [R]
- Container_services [R]
- Container_images [R]
- Load_balancer [CRD]
- Key_pair [CRD]
- Auto_snapshots [R]
- Load_balancer_metric_data [R]
- Instance_public_ports [C]
- Container_service [CUD]
- Container_image [D]
- Bundles [R]
- Relational_database_log_streams [R]
- Container_log [R]
- Cost_estimate [R]
- Gui_session_access_details [C]
- Cloud_formation_stack_records [R]
- Instance_access_details [R]
- Contact_method [CD]
- Distribution_metric_data [R]
- Instance_port_states [R]
- Key_pairs [R]
- Instance [RD]
- Disk_from_snapshot [C]
- Bucket_metric_data [R]
- Domains [R]
- Disk [CRD]
- Cloud_formation_stack [C]
- Relational_database_snapshots [R]
- Bucket_bundle [U]
- Load_balancer_tls_certificate [CD]
- Container_service_deployment [C]
- Alarm [CD]
- Export_snapshot_records [R]
- Certificates [R]
- Buckets [R]
- Instance_snapshots [R]
- Operations_for_resource [R]
- Regions [R]
- Distribution_bundles [R]
- Load_balancer_attribute [U]
- Container_api_metadata [R]
- Certificate [CD]
- Active_names [R]
- Bucket_bundles [R]
- Domain_entry [CUD]
- Distribution_latest_cache_reset [R]
- Load_balancers [R]
- Load_balancer_tls_certificates [R]
- Relational_database_blueprints [R]
- Relational_database_bundles [R]
- Instance_metadata_options [U]
- Instances_from_snapshot [C]
- Static_ip [R]
- Relational_database_master_user_password [R]
- Blueprints [R]
- Container_service_deployments [R]
- Instance_snapshot [CRD]
- Static_ips [R]
- Relational_database_events [R]
- Container_service_powers [R]
- Disk_snapshot [CRD]
- Bucket_access_key [CD]
- Instances [CR]
- Load_balancer_tls_policies [R]
- Instance_metric_data [R]
- Relational_database_metric_data [R]
- Domain [CRD]
- Relational_database [CRUD]
- Relational_databases [R]
- Disks [R]
- Distributions [R]
- Relational_database_snapshot [CRD]
- Relational_database_parameters [RU]
- Instance_state [R]
- Operation [R]
- Relational_database_log_events [R]
- Distribution [CUD]
- Known_host_keys [D]
- Setup_history [R]
- Auto_snapshot [D]
- Alarms [R]
- Operations [R]

📖 [Full lightsail documentation](services/lightsail.md)

### 9. License_manager_user_subscriptions

**Resources**: 2

- License_server_endpoint [CD]
- Identity_provider_settings [U]

📖 [Full license_manager_user_subscriptions documentation](services/license_manager_user_subscriptions.md)

### 10. Route53_recovery_readiness

**Resources**: 10

- Cell [CRUD]
- Architecture_recommendations [R]
- Cell_readiness_summary [R]
- Readiness_check_status [R]
- Recovery_group_readiness_summary [R]
- Cross_account_authorization [CD]
- Recovery_group [CRUD]
- Readiness_check_resource_status [R]
- Readiness_check [CRUD]
- Resource_set [CRUD]

📖 [Full route53_recovery_readiness documentation](services/route53_recovery_readiness.md)

### 11. Chatbot

**Resources**: 12

- Chime_webhook_configurations [R]
- Microsoft_teams_configured_team [D]
- Slack_workspace_authorization [D]
- Slack_channel_configuration [CUD]
- Slack_workspaces [R]
- Chime_webhook_configuration [CUD]
- Microsoft_teams_user_identity [D]
- Microsoft_teams_channel_configuration [CRUD]
- Slack_user_identity [D]
- Slack_channel_configurations [R]
- Slack_user_identities [R]
- Account_preferences [RU]

📖 [Full chatbot documentation](services/chatbot.md)

### 12. Networkmanager

**Resources**: 37

- Site_to_site_vpn_attachment [CR]
- Site [CUD]
- Global_networks [R]
- Connect_peer_associations [R]
- Customer_gateway_associations [R]
- Network_telemetry [R]
- Transit_gateway_connect_peer_associations [R]
- Connections [R]
- Vpc_attachment [CRU]
- Transit_gateway_registrations [R]
- Link [CUD]
- Connection [CUD]
- Peering [D]
- Network_resource_metadata [U]
- Connect_peer [CRD]
- Device [CUD]
- Core_network_change_set [R]
- Network_resource_relationships [R]
- Transit_gateway_route_table_attachment [CR]
- Core_network_change_events [R]
- Core_network_policy [CR]
- Resource_policy [CRD]
- Network_resource_counts [R]
- Links [R]
- Link_associations [R]
- Connect_attachment [CR]
- Attachment [D]
- Sites [R]
- Transit_gateway_peering [CR]
- Global_network [CUD]
- Network_resources [R]
- Devices [R]
- Network_routes [R]
- Route_analysis [R]
- Core_network_policy_version [D]
- Core_network [CRUD]
- Direct_connect_gateway_attachment [CRU]

📖 [Full networkmanager documentation](services/networkmanager.md)

### 13. Pinpoint

**Resources**: 53

- App [CRD]
- Segment_versions [R]
- Email_channel [RUD]
- Recommender_configuration [CRUD]
- Apns_channel [RUD]
- Export_jobs [R]
- Push_template [CRUD]
- Segment [CRUD]
- Gcm_channel [RUD]
- Application_date_range_kpi [R]
- Endpoint [RUD]
- Export_job [CR]
- User_endpoints [RD]
- Campaigns [R]
- Journey_run_execution_metrics [R]
- Journey [CRUD]
- Journey_state [U]
- Voice_channel [RUD]
- Journey_runs [R]
- Segments [R]
- Application_settings [RU]
- Import_job [CR]
- Campaign [CRUD]
- Voice_template [CRUD]
- Apns_sandbox_channel [RUD]
- Apns_voip_channel [RUD]
- Apps [R]
- Import_jobs [R]
- Apns_voip_sandbox_channel [RUD]
- Sms_template [CRUD]
- Campaign_date_range_kpi [R]
- Campaign_version [R]
- Journey_execution_metrics [R]
- Segment_version [R]
- Events [C]
- Template_active_version [U]
- Campaign_versions [R]
- Journey_date_range_kpi [R]
- Recommender_configurations [R]
- Segment_import_jobs [R]
- In_app_template [CRUD]
- Endpoints_batch [U]
- Adm_channel [RUD]
- Channels [R]
- Segment_export_jobs [R]
- Baidu_channel [RUD]
- Event_stream [CRD]
- Sms_channel [RUD]
- Campaign_activities [R]
- In_app_messages [R]
- Journey_execution_activity_metrics [R]
- Journey_run_execution_activity_metrics [R]
- Email_template [CRUD]

📖 [Full pinpoint documentation](services/pinpoint.md)

### 14. Detective

**Resources**: 6

- Investigation_state [U]
- Datasource_packages [U]
- Organization_configuration [RU]
- Members [CRD]
- Graph [CD]
- Investigation [R]

📖 [Full detective documentation](services/detective.md)

### 15. Managedblockchain_query

**Resources**: 3

- Transaction [R]
- Asset_contract [R]
- Token_balance [R]

📖 [Full managedblockchain_query documentation](services/managedblockchain_query.md)

### 16. Fis

**Resources**: 8

- Experiment [R]
- Safety_lever [R]
- Experiment_target_account_configuration [R]
- Target_resource_type [R]
- Target_account_configuration [CRUD]
- Action [R]
- Experiment_template [CRUD]
- Safety_lever_state [U]

📖 [Full fis documentation](services/fis.md)

### 17. S3vectors

**Resources**: 0


📖 [Full s3vectors documentation](services/s3vectors.md)

### 18. Codeguru_reviewer

**Resources**: 3

- Code_review [CR]
- Recommendation_feedback [CR]
- Repository_association [R]

📖 [Full codeguru_reviewer documentation](services/codeguru_reviewer.md)

### 19. Cleanroomsml

**Resources**: 0


📖 [Full cleanroomsml documentation](services/cleanroomsml.md)

### 20. Databrew

**Resources**: 10

- Schedule [CRUD]
- Profile_job [CU]
- Project [CRUD]
- Job_run [R]
- Job [RD]
- Recipe_version [D]
- Recipe_job [CU]
- Ruleset [CRUD]
- Recipe [CRU]
- Dataset [CRUD]

📖 [Full databrew documentation](services/databrew.md)

### 21. Workdocs

**Resources**: 21

- Document_versions [R]
- Root_folders [R]
- Folder_path [R]
- Notification_subscription [CD]
- Document [RUD]
- Document_path [R]
- Users [R]
- Resources [R]
- Groups [R]
- Folder [CRUD]
- Document_version [RUD]
- Folder_contents [RD]
- Custom_metadata [CD]
- Activities [R]
- Resource_permissions [R]
- Current_user [R]
- Notification_subscriptions [R]
- Comment [CD]
- Comments [R]
- Labels [CD]
- User [CUD]

📖 [Full workdocs documentation](services/workdocs.md)

### 22. Pca_connector_scep

**Resources**: 0


📖 [Full pca_connector_scep documentation](services/pca_connector_scep.md)

### 23. S3tables

**Resources**: 0


📖 [Full s3tables documentation](services/s3tables.md)

### 24. Route53_recovery_cluster

**Resources**: 2

- Routing_control_state [RU]
- Routing_control_states [U]

📖 [Full route53_recovery_cluster documentation](services/route53_recovery_cluster.md)

### 25. Cloudfront

**Resources**: 41

- Field_level_encryption [R]
- Streaming_distribution_config [R]
- Origin_access_control [CRUD]
- Origin_request_policy [CRUD]
- Origin_access_control_config [R]
- Cache_policy [CRUD]
- Distribution_tenant [CRUD]
- Managed_certificate_details [R]
- Key_value_store [CRUD]
- Key_group_config [R]
- Response_headers_policy [CRUD]
- Response_headers_policy_config [R]
- Realtime_log_config [CRUD]
- Anycast_ip_list [CRD]
- Field_level_encryption_profile_config [R]
- Field_level_encryption_profile [CRUD]
- Streaming_distribution [CRUD]
- Distribution_config [R]
- Public_key [CRUD]
- Public_key_config [R]
- Streaming_distribution_with_tags [C]
- Cloud_front_origin_access_identity [CRUD]
- Continuous_deployment_policy_config [R]
- Continuous_deployment_policy [CRUD]
- Invalidation_for_distribution_tenant [CR]
- Distribution [CRUD]
- Key_group [CRUD]
- Field_level_encryption_config [CRUD]
- Origin_request_policy_config [R]
- Distribution_tenant_by_domain [R]
- Connection_group_by_routing_endpoint [R]
- Connection_group [CRUD]
- Distribution_with_tags [C]
- Function [CRUD]
- Cloud_front_origin_access_identity_config [R]
- Domain_association [U]
- Invalidation [CR]
- Monitoring_subscription [CRD]
- Distribution_with_staging_config [U]
- Cache_policy_config [R]
- Vpc_origin [CRUD]

📖 [Full cloudfront documentation](services/cloudfront.md)

### 26. Kendra_ranking

**Resources**: 1

- Rescore_execution_plan [CRUD]

📖 [Full kendra_ranking documentation](services/kendra_ranking.md)

### 27. Forecastquery

**Resources**: 0


📖 [Full forecastquery documentation](services/forecastquery.md)

### 28. Rbin

**Resources**: 1

- Rule [CRUD]

📖 [Full rbin documentation](services/rbin.md)

### 29. Memorydb

**Resources**: 22

- Reserved_nodes [R]
- Reserved_nodes_offerings [R]
- Ac_ls [R]
- Multi_region_clusters [R]
- Multi_region_cluster [CUD]
- Parameter_groups [R]
- Snapshots [R]
- Subnet_groups [R]
- Users [R]
- Cluster [CUD]
- Snapshot [CD]
- Engine_versions [R]
- Multi_region_parameters [R]
- Service_updates [R]
- Subnet_group [CUD]
- Multi_region_parameter_groups [R]
- Events [R]
- Parameter_group [CUD]
- Acl [CUD]
- User [CUD]
- Clusters [R]
- Parameters [R]

📖 [Full memorydb documentation](services/memorydb.md)

### 30. Codedeploy

**Resources**: 11

- Application_revision [R]
- Deployment_config [CRD]
- Application [CRUD]
- Deployment_target [R]
- On_premises_instance [R]
- Resources_by_external_id [D]
- Lifecycle_event_hook_execution_status [C]
- Deployment_group [CRUD]
- Git_hub_account_token [D]
- Deployment_instance [R]
- Deployment [CR]

📖 [Full codedeploy documentation](services/codedeploy.md)

### 31. Ses

**Resources**: 23

- Receipt_filter [CD]
- Verified_email_address [D]
- Custom_verification_email_template [CRUD]
- Receipt_rule [CRUD]
- Identity_notification_attributes [R]
- Identity_dkim_attributes [R]
- Configuration_set_event_destination [CUD]
- Identity [D]
- Identity_policy [CD]
- Identity_policies [R]
- Active_receipt_rule_set [R]
- Identity_verification_attributes [R]
- Template [CRUD]
- Send_statistics [R]
- Configuration_set_delivery_options [C]
- Configuration_set_reputation_metrics_enabled [U]
- Receipt_rule_set [CRD]
- Configuration_set [CRD]
- Account_sending_enabled [RU]
- Configuration_set_sending_enabled [U]
- Identity_mail_from_domain_attributes [R]
- Send_quota [R]
- Configuration_set_tracking_options [CUD]

📖 [Full ses documentation](services/ses.md)

### 32. Kafka

**Resources**: 21

- Cluster_v2 [CR]
- Connectivity [U]
- Configuration_revision [R]
- Cluster [CRD]
- Bootstrap_brokers [R]
- Broker_storage [U]
- Security [U]
- Monitoring [U]
- Cluster_operation_v2 [R]
- Cluster_kafka_version [U]
- Broker_count [U]
- Compatible_kafka_versions [R]
- Cluster_policy [CRD]
- Replicator [CRD]
- Cluster_operation [R]
- Configuration [CRUD]
- Vpc_connection [CRD]
- Broker_type [U]
- Cluster_configuration [U]
- Replication_info [U]
- Storage [U]

📖 [Full kafka documentation](services/kafka.md)

### 33. Socialmessaging

**Resources**: 3

- Whats_app_message_template_media [C]
- Whats_app_message_template_from_library [C]
- Whats_app_message_template [CRUD]

📖 [Full socialmessaging documentation](services/socialmessaging.md)

### 34. Partnercentral_selling

**Resources**: 1

- Selling_system_settings [CR]

📖 [Full partnercentral_selling documentation](services/partnercentral_selling.md)

### 35. Mailmanager

**Resources**: 7

- Archive_search_results [R]
- Member_of_address_list [R]
- Address_list_import_job [CR]
- Archive_message_content [R]
- Archive_message [R]
- Archive_search [R]
- Archive_export [R]

📖 [Full mailmanager documentation](services/mailmanager.md)

### 36. Personalize

**Resources**: 19

- Solution_metrics [R]
- Schema [CRD]
- Solution_version [CR]
- Dataset [CRUD]
- Data_deletion_job [CR]
- Algorithm [R]
- Batch_inference_job [CR]
- Recommender [CRUD]
- Feature_transformation [R]
- Dataset_export_job [CR]
- Dataset_group [CRD]
- Metric_attribution [CRUD]
- Solution [CRUD]
- Dataset_import_job [CR]
- Campaign [CRUD]
- Filter [CRD]
- Recipe [R]
- Batch_segment_job [CR]
- Event_tracker [CRD]

📖 [Full personalize documentation](services/personalize.md)

### 37. Iot_managed_integrations

**Resources**: 1

- Custom_endpoint [R]

📖 [Full iot_managed_integrations documentation](services/iot_managed_integrations.md)

### 38. Securityhub

**Resources**: 36

- Action_target [CUD]
- Master_account [R]
- Hub [R]
- Automation_rule_v2 [CRUD]
- Insight [CUD]
- Invitations_count [R]
- Security_control [U]
- Findings [RU]
- Products [R]
- Connector_v2 [CRUD]
- Organization_configuration [RU]
- Insight_results [R]
- Finding_history [R]
- Security_control_definition [R]
- Standards [R]
- Insights [R]
- Resources_v2 [R]
- Standards_control [U]
- Standards_controls [R]
- Administrator_account [R]
- Resources_statistics_v2 [R]
- Ticket_v2 [C]
- Action_targets [R]
- Enabled_standards [R]
- Finding_statistics_v2 [R]
- Aggregator_v2 [CRUD]
- Findings_v2 [R]
- Invitations [D]
- Products_v2 [R]
- Configuration_policy [CRUD]
- Security_hub_configuration [U]
- Finding_aggregator [CRUD]
- Automation_rule [C]
- Members [CRD]
- Security_hub_v2 [R]
- Configuration_policy_association [R]

📖 [Full securityhub documentation](services/securityhub.md)

### 39. Mwaa

**Resources**: 3

- Web_login_token [C]
- Cli_token [C]
- Environment [CRUD]

📖 [Full mwaa documentation](services/mwaa.md)

### 40. Transcribe_streaming

**Resources**: 1

- Medical_scribe_stream [R]

📖 [Full transcribe_streaming documentation](services/transcribe_streaming.md)

### 41. Observabilityadmin

**Resources**: 6

- Centralization_rule_for_organization [CRUD]
- Telemetry_enrichment_status [R]
- Telemetry_rule_for_organization [CRUD]
- Telemetry_rule [CRUD]
- Telemetry_evaluation_status [R]
- Telemetry_evaluation_status_for_organization [R]

📖 [Full observabilityadmin documentation](services/observabilityadmin.md)

### 42. Chime_sdk_meetings

**Resources**: 4

- Meeting [CRD]
- Meeting_with_attendees [C]
- Attendee_capabilities [U]
- Attendee [CRD]

📖 [Full chime_sdk_meetings documentation](services/chime_sdk_meetings.md)

### 43. Marketplace_reporting

**Resources**: 0


📖 [Full marketplace_reporting documentation](services/marketplace_reporting.md)

### 44. Appflow

**Resources**: 8

- Connector_entity [R]
- Connector [R]
- Connector_profiles [R]
- Flow [CRUD]
- Connectors [R]
- Flow_execution_records [R]
- Connector_registration [U]
- Connector_profile [CUD]

📖 [Full appflow documentation](services/appflow.md)

### 45. Comprehend

**Resources**: 16

- Document_classification_job [R]
- Entity_recognizer [CRD]
- Events_detection_job [R]
- Dominant_language_detection_job [R]
- Sentiment_detection_job [R]
- Endpoint [CRUD]
- Flywheel [CRUD]
- Topics_detection_job [R]
- Dataset [CR]
- Entities_detection_job [R]
- Document_classifier [CRD]
- Pii_entities_detection_job [R]
- Key_phrases_detection_job [R]
- Resource_policy [CRD]
- Targeted_sentiment_detection_job [R]
- Flywheel_iteration [R]

📖 [Full comprehend documentation](services/comprehend.md)

### 46. Launch_wizard

**Resources**: 0


📖 [Full launch_wizard documentation](services/launch_wizard.md)

### 47. S3_control

**Resources**: 33

- Public_access_block [CRD]
- Bucket_policy [CRD]
- Access_point_policy_for_object_lambda [CRD]
- Access_point_scope [CRD]
- Job_tagging [CRD]
- Access_grants_instance [CRD]
- Access_grants_location [CRUD]
- Job [CR]
- Access_point_policy_status_for_object_lambda [R]
- Data_access [R]
- Multi_region_access_point_policy_status [R]
- Job_status [U]
- Access_point [CRD]
- Access_point_for_object_lambda [CRD]
- Access_grants_instance_resource_policy [CRD]
- Storage_lens_configuration [CRD]
- Access_grant [CRD]
- Bucket_lifecycle_configuration [CRD]
- Access_grants_instance_for_prefix [R]
- Access_point_configuration_for_object_lambda [CR]
- Multi_region_access_point [CRD]
- Bucket_replication [CRD]
- Bucket [CRD]
- Storage_lens_group [CRUD]
- Multi_region_access_point_operation [R]
- Multi_region_access_point_policy [CR]
- Bucket_tagging [CRD]
- Job_priority [U]
- Multi_region_access_point_routes [R]
- Storage_lens_configuration_tagging [CRD]
- Access_point_policy_status [R]
- Access_point_policy [CRD]
- Bucket_versioning [CR]

📖 [Full s3_control documentation](services/s3_control.md)

### 48. Controlcatalog

**Resources**: 0


📖 [Full controlcatalog documentation](services/controlcatalog.md)

### 49. Applicationcostprofiler

**Resources**: 1

- Report_definition [CRUD]

📖 [Full applicationcostprofiler documentation](services/applicationcostprofiler.md)

### 50. Cloudtrail

**Resources**: 13

- Event_data_store [CRUD]
- Trail_status [R]
- Channel [CRUD]
- Trails [R]
- Dashboard [CRUD]
- Query_results [R]
- Import [R]
- Insight_selectors [CR]
- Trail [CRUD]
- Resource_policy [CRD]
- Event_configuration [CR]
- Event_selectors [CR]
- Query [R]

📖 [Full cloudtrail documentation](services/cloudtrail.md)

### 51. Keyspacesstreams

**Resources**: 3

- Shard_iterator [R]
- Records [R]
- Stream [R]

📖 [Full keyspacesstreams documentation](services/keyspacesstreams.md)

### 52. Personalize_events

**Resources**: 5

- Action_interactions [C]
- Actions [C]
- Items [C]
- Users [C]
- Events [C]

📖 [Full personalize_events documentation](services/personalize_events.md)

### 53. Wellarchitected

**Resources**: 21

- Profile_template [R]
- Lens_version [C]
- Milestone [CR]
- Answer [RU]
- Review_template_answer [RU]
- Consolidated_report [R]
- Lens_review_report [R]
- Review_template_lens_review [RU]
- Lens_share [CD]
- Lens [RD]
- Workload [CRUD]
- Lens_version_difference [R]
- Share_invitation [U]
- Template_share [CD]
- Integration [U]
- Lens_review [RU]
- Profile [CRUD]
- Review_template [CRUD]
- Workload_share [CUD]
- Global_settings [RU]
- Profile_share [CD]

📖 [Full wellarchitected documentation](services/wellarchitected.md)

### 54. Cloudhsm

**Resources**: 10

- Config [R]
- Hapg [CRD]
- Hsm [CRD]
- Luna_client [CRD]
- Hsm [CD]
- Resource_policy [CRD]
- Backups [R]
- Clusters [R]
- Cluster [CD]
- Backup [D]

📖 [Full cloudhsm documentation](services/cloudhsm.md)

### 55. Sesv2

**Resources**: 50

- Dedicated_ip_pool_scaling_attributes [C]
- Tenant [CRD]
- Custom_verification_email_template [CRUD]
- Email_template [CRUD]
- Dedicated_ip [R]
- Import_job [CR]
- Deliverability_dashboard_options [R]
- Dedicated_ips [R]
- Export_job [CR]
- Configuration_set_event_destination [CUD]
- Multi_region_endpoint [CRD]
- Email_identity [CRD]
- Account [R]
- Email_identity_policies [R]
- Reputation_entity [R]
- Configuration_set_archiving_options [C]
- Configuration_set_tracking_options [C]
- Email_identity_feedback_attributes [C]
- Account_details [C]
- Email_identity_mail_from_attributes [C]
- Deliverability_dashboard_option [C]
- Deliverability_test_report [CR]
- Suppressed_destination [CRD]
- Reputation_entity_customer_managed_status [U]
- Blacklist_reports [R]
- Account_dedicated_ip_warmup_attributes [C]
- Account_suppression_attributes [C]
- Account_vdm_attributes [C]
- Email_identity_configuration_set_attributes [C]
- Email_identity_dkim_attributes [C]
- Domain_statistics_report [R]
- Email_identity_dkim_signing_attributes [C]
- Reputation_entity_policy [U]
- Contact_list [CRUD]
- Configuration_set_suppression_options [C]
- Message_insights [R]
- Contact [CRUD]
- Email_identity_policy [CUD]
- Configuration_set_vdm_options [C]
- Dedicated_ip_warmup_attributes [C]
- Tenant_resource_association [CD]
- Domain_deliverability_campaign [R]
- Dedicated_ip_pool [CRD]
- Configuration_set_event_destinations [R]
- Account_sending_attributes [C]
- Configuration_set [CRD]
- Configuration_set_sending_options [C]
- Configuration_set_reputation_options [C]
- Configuration_set_delivery_options [C]
- Dedicated_ip_in_pool [C]

📖 [Full sesv2 documentation](services/sesv2.md)

### 56. Iot_wireless

**Resources**: 29

- Wireless_device_import_task [RUD]
- Wireless_gateway_task [CRD]
- Fuota_task [CRUD]
- Wireless_gateway_firmware_information [R]
- Destination [CRUD]
- Resource_event_configuration [RU]
- Position_configuration [CR]
- Position_estimate [R]
- Wireless_device [CRUD]
- Position [RU]
- Wireless_gateway_task_definition [CRD]
- Wireless_device_statistics [R]
- Wireless_gateway_certificate [R]
- Service_endpoint [R]
- Partner_account [RU]
- Service_profile [CRD]
- Log_levels_by_resource_types [RU]
- Metric_configuration [RU]
- Multicast_group [CRUD]
- Wireless_gateway_statistics [R]
- Resource_log_level [CR]
- Resource_position [RU]
- Device_profile [CRD]
- Network_analyzer_configuration [CRUD]
- Queued_messages [D]
- Event_configuration_by_resource_types [RU]
- Multicast_group_session [R]
- Metrics [R]
- Wireless_gateway [CRUD]

📖 [Full iot_wireless documentation](services/iot_wireless.md)

### 57. Amp

**Resources**: 1

- Default_scraper_configuration [R]

📖 [Full amp documentation](services/amp.md)

### 58. Finspace_data

**Resources**: 8

- External_data_view_access_details [R]
- Dataset [CRUD]
- Changeset [CRU]
- User [CRU]
- Data_view [CR]
- Working_location [R]
- Programmatic_access_credentials [R]
- Permission_group [CRUD]

📖 [Full finspace_data documentation](services/finspace_data.md)

### 59. S3outposts

**Resources**: 1

- Endpoint [CD]

📖 [Full s3outposts documentation](services/s3outposts.md)

### 60. Pi

**Resources**: 5

- Dimension_keys [R]
- Dimension_key_details [R]
- Resource_metrics [R]
- Performance_analysis_report [CRD]
- Resource_metadata [R]

📖 [Full pi documentation](services/pi.md)

### 61. Neptunedata

**Resources**: 14

- Ml_model_training_job [R]
- Ml_data_processing_job [R]
- Propertygraph_stream [R]
- Propertygraph_statistics [RD]
- Sparql_stream [R]
- Ml_model_transform_job [R]
- Loader_job_status [R]
- Open_cypher_query_status [R]
- Engine_status [R]
- Rdf_graph_summary [R]
- Ml_endpoint [CRD]
- Sparql_statistics [RD]
- Gremlin_query_status [R]
- Propertygraph_summary [R]

📖 [Full neptunedata documentation](services/neptunedata.md)

### 62. Codeconnections

**Resources**: 8

- Host [CRUD]
- Repository_link [CRUD]
- Sync_configuration [CRUD]
- Repository_sync_status [R]
- Connection [CRD]
- Resource_sync_status [R]
- Sync_blocker_summary [R]
- Sync_blocker [U]

📖 [Full codeconnections documentation](services/codeconnections.md)

### 63. Keyspaces

**Resources**: 4

- Table [CRUD]
- Table_auto_scaling_settings [R]
- Type [CRD]
- Keyspace [CRUD]

📖 [Full keyspaces documentation](services/keyspaces.md)

### 64. Outposts

**Resources**: 11

- Outpost_supported_instance_types [R]
- Capacity_task [R]
- Site [CRUD]
- Outpost_billing_information [R]
- Outpost [CRUD]
- Order [CR]
- Catalog_item [R]
- Connection [R]
- Site_rack_physical_properties [U]
- Site_address [RU]
- Outpost_instance_types [R]

📖 [Full outposts documentation](services/outposts.md)

### 65. Mediastore

**Resources**: 5

- Cors_policy [CRD]
- Container [CRD]
- Lifecycle_policy [CRD]
- Metric_policy [CRD]
- Container_policy [CRD]

📖 [Full mediastore documentation](services/mediastore.md)

### 66. Iotdeviceadvisor

**Resources**: 4

- Suite_run_report [R]
- Endpoint [R]
- Suite_definition [CRUD]
- Suite_run [R]

📖 [Full iotdeviceadvisor documentation](services/iotdeviceadvisor.md)

### 67. Deadline

**Resources**: 3

- Queue_limit_association [CRUD]
- Sessions_statistics_aggregation [R]
- Queue_fleet_association [CRUD]

📖 [Full deadline documentation](services/deadline.md)

### 68. Pcs

**Resources**: 0


📖 [Full pcs documentation](services/pcs.md)

### 69. Panorama

**Resources**: 11

- Node_from_template_job [CR]
- Device_metadata [U]
- Application_instance [CR]
- Package_import_job [CR]
- Device_job [R]
- Package [CRD]
- Application_instance_details [R]
- Package_version [R]
- Node [R]
- Job_for_devices [C]
- Device [RD]

📖 [Full panorama documentation](services/panorama.md)

### 70. Codebuild

**Resources**: 12

- Webhook [CUD]
- Build_batch [D]
- Resource_policy [CRD]
- Test_cases [R]
- Fleet [CUD]
- Report [D]
- Report_group [CUD]
- Source_credentials [D]
- Report_group_trend [R]
- Project [CUD]
- Project_visibility [U]
- Code_coverages [R]

📖 [Full codebuild documentation](services/codebuild.md)

### 71. Greengrassv2

**Resources**: 7

- Deployment [CRD]
- Connectivity_info [RU]
- Component [RD]
- Component_version_artifact [R]
- Core_device [RD]
- Service_role_for_account [R]
- Component_version [C]

📖 [Full greengrassv2 documentation](services/greengrassv2.md)

### 72. Health

**Resources**: 12

- Event_types [R]
- Entity_aggregates [R]
- Health_service_status_for_organization [R]
- Events_for_organization [R]
- Event_details [R]
- Entity_aggregates_for_organization [R]
- Affected_accounts_for_organization [R]
- Affected_entities [R]
- Event_details_for_organization [R]
- Affected_entities_for_organization [R]
- Event_aggregates [R]
- Events [R]

📖 [Full health documentation](services/health.md)

### 73. Polly

**Resources**: 3

- Lexicon [CRD]
- Voices [R]
- Speech_synthesis_task [R]

📖 [Full polly documentation](services/polly.md)

### 74. Workmail

**Resources**: 29

- Organization [CRD]
- Email_monitoring_configuration [CRD]
- Availability_configuration [CUD]
- Default_mail_domain [U]
- Inbound_dmarc_settings [CR]
- Personal_access_token [D]
- Identity_center_application [CD]
- Retention_policy [CD]
- Mailbox_export_job [R]
- Default_retention_policy [R]
- Mobile_device_access_effect [R]
- Alias [CD]
- Entity [R]
- Mailbox_details [R]
- Mobile_device_access_rule [CUD]
- Personal_access_token_metadata [R]
- Access_control_rule [CD]
- Mailbox_quota [U]
- User [CRUD]
- Mobile_device_access_override [CRD]
- Access_control_effect [R]
- Group [CRUD]
- Resource [CRUD]
- Mail_domain [R]
- Impersonation_role [CRUD]
- Primary_email_address [U]
- Impersonation_role_effect [R]
- Identity_provider_configuration [CRD]
- Mailbox_permissions [CD]

📖 [Full workmail documentation](services/workmail.md)

### 75. Cloudformation

**Resources**: 25

- Type_registration [R]
- Change_set [CRD]
- Resource_scan [R]
- Organizations_access [R]
- Stacks [R]
- Stack_set_operation [R]
- Stack [CUD]
- Stack_events [R]
- Publisher [R]
- Stack_policy [R]
- Stack_resources [R]
- Change_set_hooks [R]
- Stack_refactor [CR]
- Type [R]
- Stack_resource [R]
- Termination_protection [U]
- Stack_set [CRUD]
- Stack_resource_drifts [R]
- Template_summary [R]
- Stack_instances [CUD]
- Stack_drift_detection_status [R]
- Account_limits [R]
- Stack_instance [R]
- Template [R]
- Generated_template [CRUD]

📖 [Full cloudformation documentation](services/cloudformation.md)

### 76. Evidently

**Resources**: 0


📖 [Full evidently documentation](services/evidently.md)

### 77. Geo_routes

**Resources**: 0


📖 [Full geo_routes documentation](services/geo_routes.md)

### 78. Appconfig

**Resources**: 10

- Configuration [R]
- Deployment_strategy [CRUD]
- Configuration_profile [CRUD]
- Extension_association [CRUD]
- Application [CRUD]
- Environment [CRUD]
- Extension [CRUD]
- Account_settings [RU]
- Deployment [R]
- Hosted_configuration_version [CRD]

📖 [Full appconfig documentation](services/appconfig.md)

### 79. Appintegrations

**Resources**: 4

- Data_integration [CRUD]
- Data_integration_association [CU]
- Application [CRUD]
- Event_integration [CRUD]

📖 [Full appintegrations documentation](services/appintegrations.md)

### 80. Gamelift

**Resources**: 47

- Fleet_events [R]
- Game_server_instances [R]
- Game_session_placement [R]
- Game_session [CU]
- Script [CRUD]
- Fleet_utilization [R]
- Fleet_attributes [RU]
- Matchmaking_configurations [R]
- Runtime_configuration [RU]
- Fleet_location_utilization [R]
- Game_session_details [R]
- Game_session_queue [CUD]
- Fleet_location_attributes [R]
- Instances [R]
- Alias [CRUD]
- Fleet_location_capacity [R]
- Vpc_peering_connection [CD]
- Container_group_definition [CRUD]
- Compute_auth_token [R]
- Compute [R]
- Player_sessions [CR]
- Instance_access [R]
- Fleet_locations [CD]
- Matchmaking [R]
- Game_session_queues [R]
- Game_session_log_url [R]
- Location [CD]
- Matchmaking_rule_set [CD]
- Game_server_group [CRUD]
- Game_server [RU]
- Matchmaking_configuration [CUD]
- Fleet_deployment [R]
- Matchmaking_rule_sets [R]
- Scaling_policies [R]
- Scaling_policy [CD]
- Fleet_capacity [RU]
- Game_sessions [R]
- Vpc_peering_connections [R]
- Vpc_peering_authorizations [R]
- Build [CRUD]
- Container_fleet [CRUD]
- Ec2_instance_limits [R]
- Vpc_peering_authorization [CD]
- Fleet_port_settings [RU]
- Compute_access [R]
- Player_session [C]
- Fleet [CD]

📖 [Full gamelift documentation](services/gamelift.md)

### 81. Billingconductor

**Resources**: 1

- Billing_group_cost_report [R]

📖 [Full billingconductor documentation](services/billingconductor.md)

### 82. Waf_regional

**Resources**: 20

- Size_constraint_set [CRUD]
- Regex_match_set [CRUD]
- Change_token_status [R]
- Sampled_requests [R]
- Change_token [R]
- Byte_match_set [CRUD]
- Regex_pattern_set [CRUD]
- Permission_policy [CRD]
- Rate_based_rule_managed_keys [R]
- Sql_injection_match_set [CRUD]
- Web_acl_for_resource [R]
- Rule_group [CRUD]
- Web_acl [CRUD]
- Ip_set [CRUD]
- Logging_configuration [CRD]
- Rate_based_rule [CRUD]
- Xss_match_set [CRUD]
- Geo_match_set [CRUD]
- Web_acl_migration_stack [C]
- Rule [CRUD]

📖 [Full waf_regional documentation](services/waf_regional.md)

### 83. Proton

**Resources**: 4

- Repository_sync_status [R]
- Service_instance_sync_status [R]
- Template_sync_status [R]
- Resources_summary [R]

📖 [Full proton documentation](services/proton.md)

### 84. Resource_explorer_2

**Resources**: 7

- Managed_view [R]
- Account_level_service_configuration [R]
- Service_view [R]
- Index [R]
- Resource_explorer_setup [CRD]
- Service_index [R]
- Default_view [R]

📖 [Full resource_explorer_2 documentation](services/resource_explorer_2.md)

### 85. Schemas

**Resources**: 8

- Code_binding [CR]
- Registry [CRUD]
- Discovered_schema [R]
- Schema [CRUD]
- Code_binding_source [R]
- Resource_policy [CRD]
- Discoverer [CRUD]
- Schema_version [D]

📖 [Full schemas documentation](services/schemas.md)

### 86. Aiops

**Resources**: 0


📖 [Full aiops documentation](services/aiops.md)

### 87. Elasticsearch_service

**Resources**: 21

- Reserved_elasticsearch_instance_offerings [R]
- Domain_auto_tunes [R]
- Upgrade_status [R]
- Outbound_cross_cluster_search_connection [CD]
- Elasticsearch_domain [CRD]
- Domain_change_progress [R]
- Package_version_history [R]
- Reserved_elasticsearch_instances [R]
- Elasticsearch_domains [R]
- Elasticsearch_domain_config [RU]
- Inbound_cross_cluster_search_connection [D]
- Compatible_elasticsearch_versions [R]
- Package [CUD]
- Vpc_endpoint [CUD]
- Vpc_endpoints [R]
- Elasticsearch_instance_type_limits [R]
- Elasticsearch_service_role [D]
- Outbound_cross_cluster_search_connections [R]
- Packages [R]
- Inbound_cross_cluster_search_connections [R]
- Upgrade_history [R]

📖 [Full elasticsearch_service documentation](services/elasticsearch_service.md)

### 88. Sso

**Resources**: 1

- Role_credentials [R]

📖 [Full sso documentation](services/sso.md)

### 89. Secrets_manager

**Resources**: 5

- Secret_version_stage [U]
- Secret [CRUD]
- Random_password [R]
- Resource_policy [CRD]
- Secret_value [CR]

📖 [Full secrets_manager documentation](services/secrets_manager.md)

### 90. Cognito_identity_provider

**Resources**: 26

- Ui_customization [R]
- User_pool_mfa_config [R]
- User_pool_domain [CRUD]
- Auth_event_feedback [U]
- Signing_certificate [R]
- Device [R]
- User_auth_factors [R]
- Device_status [U]
- User_pool_client [CRUD]
- User_attribute_verification_code [R]
- Web_authn_credential [D]
- Risk_configuration [R]
- Tokens_from_refresh_token [R]
- Identity_provider_by_identifier [R]
- Group [CRUD]
- Identity_provider [CRUD]
- User_pool [CRUD]
- Managed_login_branding [CRUD]
- Managed_login_branding_by_client [R]
- User [RD]
- User_import_job [CR]
- Csv_header [R]
- Log_delivery_configuration [R]
- Resource_server [CRUD]
- User_attributes [UD]
- Terms [CRUD]

📖 [Full cognito_identity_provider documentation](services/cognito_identity_provider.md)

### 91. Cloudsearch_domain

**Resources**: 0


📖 [Full cloudsearch_domain documentation](services/cloudsearch_domain.md)

### 92. Amplifyuibuilder

**Resources**: 2

- Metadata_flag [C]
- Metadata [R]

📖 [Full amplifyuibuilder documentation](services/amplifyuibuilder.md)

### 93. Networkmonitor

**Resources**: 0


📖 [Full networkmonitor documentation](services/networkmonitor.md)

### 94. Iotanalytics

**Resources**: 6

- Dataset [CRUD]
- Channel [CRUD]
- Pipeline [CRUD]
- Dataset_content [CRD]
- Datastore [CRUD]
- Logging_options [CR]

📖 [Full iotanalytics documentation](services/iotanalytics.md)

### 95. Medical_imaging

**Resources**: 4

- Image_set_metadata [RU]
- Image_set [RD]
- Image_frame [R]
- Dicom_import_job [R]

📖 [Full medical_imaging documentation](services/medical_imaging.md)

### 96. Bcm_dashboards

**Resources**: 2

- Dashboard [CRUD]
- Resource_policy [R]

📖 [Full bcm_dashboards documentation](services/bcm_dashboards.md)

### 97. Drs

**Resources**: 2

- Extended_source_server [C]
- Launch_action [CD]

📖 [Full drs documentation](services/drs.md)

### 98. Support_app

**Resources**: 3

- Account_alias [CRD]
- Slack_channel_configuration [CUD]
- Slack_workspace_configuration [D]

📖 [Full support_app documentation](services/support_app.md)

### 99. Tnb

**Resources**: 9

- Sol_function_package_content [CR]
- Sol_network_instance [CRUD]
- Sol_network_package_content [CR]
- Sol_network_package [CRUD]
- Sol_function_package [CRUD]
- Sol_function_package_descriptor [R]
- Sol_network_operation [R]
- Sol_network_package_descriptor [R]
- Sol_function_instance [R]

📖 [Full tnb documentation](services/tnb.md)

### 100. Connectcases

**Resources**: 0


📖 [Full connectcases documentation](services/connectcases.md)

### 101. Route53_recovery_control_config

**Resources**: 5

- Resource_policy [R]
- Safety_rule [CRUD]
- Control_panel [CRUD]
- Cluster [CRUD]
- Routing_control [CRUD]

📖 [Full route53_recovery_control_config documentation](services/route53_recovery_control_config.md)

### 102. Support

**Resources**: 12

- Attachment [R]
- Communications [R]
- Severity_levels [R]
- Trusted_advisor_checks [R]
- Trusted_advisor_check_summaries [R]
- Create_case_options [R]
- Trusted_advisor_check_result [R]
- Cases [R]
- Services [R]
- Supported_languages [R]
- Trusted_advisor_check_refresh_statuses [R]
- Case [C]

📖 [Full support documentation](services/support.md)

### 103. Groundstation

**Resources**: 1

- Minute_usage [R]

📖 [Full groundstation documentation](services/groundstation.md)

### 104. Pinpoint_email

**Resources**: 25

- Configuration_set_delivery_options [C]
- Email_identity_dkim_attributes [C]
- Deliverability_dashboard_option [C]
- Email_identity_feedback_attributes [C]
- Domain_statistics_report [R]
- Email_identity_mail_from_attributes [C]
- Deliverability_dashboard_options [R]
- Dedicated_ips [R]
- Configuration_set_event_destinations [R]
- Account_dedicated_ip_warmup_attributes [C]
- Dedicated_ip_in_pool [C]
- Dedicated_ip_pool [CD]
- Deliverability_test_report [CR]
- Account [R]
- Blacklist_reports [R]
- Configuration_set_tracking_options [C]
- Account_sending_attributes [C]
- Email_identity [CRD]
- Dedicated_ip [R]
- Configuration_set_sending_options [C]
- Configuration_set_reputation_options [C]
- Configuration_set [CRD]
- Domain_deliverability_campaign [R]
- Configuration_set_event_destination [CUD]
- Dedicated_ip_warmup_attributes [C]

📖 [Full pinpoint_email documentation](services/pinpoint_email.md)

### 105. Ebs

**Resources**: 1

- Snapshot_block [CR]

📖 [Full ebs documentation](services/ebs.md)

### 106. Medialive

**Resources**: 27

- Schedule [RD]
- Node [CRUD]
- Node_state [U]
- Input [CRUD]
- Thumbnails [R]
- Signal_map [CRD]
- Offering [R]
- Tags [CD]
- Event_bridge_rule_template_group [CRUD]
- Event_bridge_rule_template [CRUD]
- Channel_placement_group [CRUD]
- Account_configuration [RU]
- Input_device_thumbnail [R]
- Cluster [CRUD]
- Multiplex_program [CRUD]
- Sdi_source [CRUD]
- Cloud_watch_alarm_template [CRUD]
- Partner_input [C]
- Input_device [RU]
- Reservation [RUD]
- Node_registration_script [C]
- Input_security_group [CRUD]
- Network [CRUD]
- Multiplex [CRUD]
- Cloud_watch_alarm_template_group [CRUD]
- Channel [CRUD]
- Channel_class [U]

📖 [Full medialive documentation](services/medialive.md)

### 107. Efs

**Resources**: 15

- Replication_configurations [R]
- Access_point [CD]
- File_system_policy [CRD]
- Mount_targets [R]
- Backup_policy [CR]
- Account_preferences [CR]
- Access_points [R]
- Mount_target_security_groups [R]
- File_system_protection [U]
- File_system [CUD]
- Replication_configuration [CD]
- File_systems [R]
- Tags [CRD]
- Mount_target [CD]
- Lifecycle_configuration [CR]

📖 [Full efs documentation](services/efs.md)

### 108. Mediastore_data

**Resources**: 1

- Object [CRD]

📖 [Full mediastore_data documentation](services/mediastore_data.md)

### 109. Repostspace

**Resources**: 2

- Channel [CRU]
- Space [CRUD]

📖 [Full repostspace documentation](services/repostspace.md)

### 110. Greengrass

**Resources**: 26

- Service_role_for_account [R]
- Connector_definition_version [CR]
- Subscription_definition [CRUD]
- Software_update_job [C]
- Connectivity_info [RU]
- Deployment_status [R]
- Group_certificate_configuration [RU]
- Subscription_definition_version [CR]
- Group_version [CR]
- Function_definition [CRUD]
- Bulk_deployment_status [R]
- Group [CRUD]
- Resource_definition [CRUD]
- Connector_definition [CRUD]
- Logger_definition_version [CR]
- Core_definition [CRUD]
- Deployment [C]
- Device_definition [CRUD]
- Thing_runtime_configuration [RU]
- Core_definition_version [CR]
- Device_definition_version [CR]
- Function_definition_version [CR]
- Group_certificate_authority [CR]
- Associated_role [R]
- Logger_definition [CRUD]
- Resource_definition_version [CR]

📖 [Full greengrass documentation](services/greengrass.md)

### 111. Marketplace_commerce_analytics

**Resources**: 0


📖 [Full marketplace_commerce_analytics documentation](services/marketplace_commerce_analytics.md)

### 112. Migrationhubstrategy

**Resources**: 12

- Application_component_config [U]
- Application_component_strategies [R]
- Server_details [R]
- Portfolio_preferences [CR]
- Server_strategies [R]
- Latest_assessment_id [R]
- Portfolio_summary [R]
- Server_config [U]
- Application_component_details [R]
- Assessment [R]
- Import_file_task [R]
- Recommendation_report_details [R]

📖 [Full migrationhubstrategy documentation](services/migrationhubstrategy.md)

### 113. Oam

**Resources**: 3

- Sink_policy [CR]
- Link [CRUD]
- Sink [CRD]

📖 [Full oam documentation](services/oam.md)

### 114. Dax

**Resources**: 9

- Clusters [R]
- Events [R]
- Subnet_groups [R]
- Parameter_groups [R]
- Parameters [R]
- Cluster [CUD]
- Subnet_group [CUD]
- Default_parameters [R]
- Parameter_group [CUD]

📖 [Full dax documentation](services/dax.md)

### 115. Managedblockchain

**Resources**: 5

- Network [CR]
- Node [CRUD]
- Accessor [CRD]
- Proposal [CR]
- Member [CRUD]

📖 [Full managedblockchain documentation](services/managedblockchain.md)

### 116. Waf

**Resources**: 19

- Rate_based_rule [CRUD]
- Regex_pattern_set [CRUD]
- Xss_match_set [CRUD]
- Sampled_requests [R]
- Rule [CRUD]
- Rule_group [CRUD]
- Byte_match_set [CRUD]
- Regex_match_set [CRUD]
- Size_constraint_set [CRUD]
- Web_acl [CRUD]
- Geo_match_set [CRUD]
- Permission_policy [CRD]
- Ip_set [CRUD]
- Logging_configuration [CRD]
- Sql_injection_match_set [CRUD]
- Rate_based_rule_managed_keys [R]
- Web_acl_migration_stack [C]
- Change_token_status [R]
- Change_token [R]

📖 [Full waf documentation](services/waf.md)

### 117. Backup

**Resources**: 31

- Framework [CRUD]
- Recovery_point_lifecycle [U]
- Restore_testing_plan [CRUD]
- Region_settings [RU]
- Restore_testing_selection [CRUD]
- Recovery_point [RD]
- Backup_vault [CRD]
- Restore_job_metadata [R]
- Backup_plan [CRUD]
- Backup_job [R]
- Protected_resource [R]
- Recovery_point_index_details [R]
- Report_plan [CRUD]
- Restore_access_backup_vault [C]
- Supported_resource_types [R]
- Restore_job [R]
- Restore_validation_result [C]
- Backup_plan_from_json [R]
- Backup_vault_access_policy [CRD]
- Backup_vault_lock_configuration [CD]
- Restore_testing_inferred_metadata [R]
- Backup_selection [CRD]
- Legal_hold [CR]
- Copy_job [R]
- Recovery_point_index_settings [U]
- Report_job [R]
- Backup_vault_notifications [CRD]
- Logically_air_gapped_backup_vault [C]
- Global_settings [RU]
- Recovery_point_restore_metadata [R]
- Backup_plan_from_template [R]

📖 [Full backup documentation](services/backup.md)

### 118. Network_firewall

**Resources**: 20

- Rule_group [CRUD]
- Firewall_policy_change_protection [U]
- Analysis_report_results [R]
- Subnet_change_protection [U]
- Resource_policy [CRD]
- Vpc_endpoint_association [CRD]
- Firewall [CRD]
- Tls_inspection_configuration [CRUD]
- Firewall_encryption_configuration [U]
- Rule_group_summary [R]
- Network_firewall_transit_gateway_attachment [D]
- Availability_zone_change_protection [U]
- Firewall_policy [CRUD]
- Firewall_analysis_settings [U]
- Firewall_metadata [R]
- Firewall_description [U]
- Rule_group_metadata [R]
- Flow_operation [R]
- Firewall_delete_protection [U]
- Logging_configuration [RU]

📖 [Full network_firewall documentation](services/network_firewall.md)

### 119. Rds

**Resources**: 68

- Db_shard_group [CD]
- Db_instance_automated_backups [R]
- Db_instance [CD]
- Db_cluster_snapshot [CD]
- Db_parameter_groups [R]
- Db_proxy_endpoint [CD]
- Db_cluster [CD]
- Source_regions [R]
- Db_log_files [R]
- Db_proxies [R]
- Db_subnet_groups [R]
- Integration [CD]
- Db_proxy_targets [R]
- Global_clusters [R]
- Db_instance_read_replica [C]
- Db_snapshot_attributes [R]
- Db_cluster_endpoint [CD]
- Db_proxy [CD]
- Option_group_options [R]
- Db_snapshot [CD]
- Db_major_engine_versions [R]
- Db_proxy_endpoints [R]
- Option_group [CD]
- Custom_db_engine_version [CD]
- Event_subscription [CD]
- Db_cluster_automated_backup [D]
- Blue_green_deployments [R]
- Pending_maintenance_actions [R]
- Db_cluster_automated_backups [R]
- Engine_default_parameters [R]
- Db_cluster_endpoints [R]
- Db_snapshots [R]
- Events [R]
- Db_security_group [CD]
- Db_shard_groups [R]
- Db_cluster_parameter_group [CD]
- Tenant_databases [R]
- Db_security_groups [R]
- Db_cluster_snapshots [R]
- Integrations [R]
- Db_cluster_backtracks [R]
- Reserved_db_instances_offerings [R]
- Export_tasks [R]
- Orderable_db_instance_options [R]
- Global_cluster [CD]
- Db_recommendations [R]
- Tenant_database [CD]
- Blue_green_deployment [CD]
- Db_instance_automated_backup [D]
- Db_parameter_group [CD]
- Engine_default_cluster_parameters [R]
- Event_categories [R]
- Reserved_db_instances [R]
- Certificates [R]
- Valid_db_instance_modifications [R]
- Db_proxy_target_groups [R]
- Db_cluster_parameter_groups [R]
- Option_groups [R]
- Db_subnet_group [CD]
- Db_parameters [R]
- Db_snapshot_tenant_databases [R]
- Account_attributes [R]
- Db_engine_versions [R]
- Db_cluster_snapshot_attributes [R]
- Db_instances [R]
- Db_cluster_parameters [R]
- Db_clusters [R]
- Event_subscriptions [R]

📖 [Full rds documentation](services/rds.md)

### 120. Iotthingsgraph

**Resources**: 9

- Flow_template [CRUD]
- Namespace_deletion_status [R]
- Flow_template_revisions [R]
- Entities [R]
- System_template_revisions [R]
- System_instance [CRD]
- Namespace [RD]
- Upload_status [R]
- System_template [CRUD]

📖 [Full iotthingsgraph documentation](services/iotthingsgraph.md)

### 121. Firehose

**Resources**: 4

- Destination [U]
- Record [C]
- Delivery_stream [CRD]
- Record_batch [C]

📖 [Full firehose documentation](services/firehose.md)

### 122. Textract

**Resources**: 7

- Lending_analysis [R]
- Document_text_detection [R]
- Expense_analysis [R]
- Document_analysis [R]
- Lending_analysis_summary [R]
- Adapter [CRUD]
- Adapter_version [CRD]

📖 [Full textract documentation](services/textract.md)

### 123. Macie2

**Resources**: 29

- Custom_data_identifier [CRD]
- Member [CRD]
- Buckets [R]
- Administrator_account [R]
- Classification_export_configuration [CR]
- Invitations_count [R]
- Classification_scope [RU]
- Allow_list [CRUD]
- Resource_profile_detections [U]
- Findings_publication_configuration [CR]
- Finding_statistics [R]
- Macie_session [RU]
- Sample_findings [C]
- Sensitive_data_occurrences [R]
- Usage_totals [R]
- Classification_job [CRU]
- Automated_discovery_configuration [RU]
- Master_account [R]
- Reveal_configuration [RU]
- Organization_configuration [RU]
- Findings [R]
- Invitations [CD]
- Sensitive_data_occurrences_availability [R]
- Sensitivity_inspection_template [RU]
- Usage_statistics [R]
- Resource_profile [RU]
- Findings_filter [CRUD]
- Bucket_statistics [R]
- Member_session [U]

📖 [Full macie2 documentation](services/macie2.md)

### 124. Comprehendmedical

**Resources**: 5

- Rx_norm_inference_job [R]
- Snomedct_inference_job [R]
- Entities_detection_v2_job [R]
- Icd10_cm_inference_job [R]
- Phi_detection_job [R]

📖 [Full comprehendmedical documentation](services/comprehendmedical.md)

### 125. Sagemaker_a2i_runtime

**Resources**: 1

- Human_loop [RD]

📖 [Full sagemaker_a2i_runtime documentation](services/sagemaker_a2i_runtime.md)

### 126. Budgets

**Resources**: 12

- Subscriber [CUD]
- Budget_notifications_for_account [R]
- Notifications_for_budget [R]
- Budget_action_histories [R]
- Subscribers_for_notification [R]
- Budgets [R]
- Budget_performance_history [R]
- Notification [CUD]
- Budget_action [CRUD]
- Budget_actions_for_budget [R]
- Budget [CRUD]
- Budget_actions_for_account [R]

📖 [Full budgets documentation](services/budgets.md)

### 127. Auto_scaling_plans

**Resources**: 4

- Scaling_plan [CUD]
- Scaling_plan_resources [R]
- Scaling_plans [R]
- Scaling_plan_resource_forecast_data [R]

📖 [Full auto_scaling_plans documentation](services/auto_scaling_plans.md)

### 128. Qbusiness

**Resources**: 12

- Chat_response_configuration [CRUD]
- Anonymous_web_experience_url [C]
- User [CRUD]
- Conversation [D]
- Attachment [D]
- Document_content [R]
- Policy [R]
- Chat_controls_configuration [RUD]
- Feedback [C]
- Subscription [CU]
- Media [R]
- Group [CRD]

📖 [Full qbusiness documentation](services/qbusiness.md)

### 129. Connectcampaignsv2

**Resources**: 17

- Connect_instance_config [RD]
- Instance_communication_limits [CR]
- Campaign_state_batch [R]
- Connect_instance_integration [CD]
- Instance_onboarding_job_status [R]
- Campaign_communication_limits [UD]
- Campaign_schedule [U]
- Campaign_state [R]
- Instance_onboarding_job [D]
- Profile_outbound_request_batch [C]
- Campaign_flow_association [U]
- Outbound_request_batch [C]
- Campaign_source [U]
- Campaign_channel_subtype_config [UD]
- Campaign [CRD]
- Campaign_communication_time [UD]
- Campaign_name [U]

📖 [Full connectcampaignsv2 documentation](services/connectcampaignsv2.md)

### 130. Inspector2

**Resources**: 18

- Findings_report [C]
- Organization_configuration [RU]
- Code_security_scan [R]
- Cis_scan_result_details [R]
- Sbom_export [CR]
- Member [R]
- Cis_scan_report [R]
- Ec2_deep_inspection_configuration [RU]
- Delegated_admin_account [R]
- Findings_report_status [R]
- Clusters_for_image [R]
- Code_security_integration [CRUD]
- Encryption_key [RU]
- Configuration [RU]
- Code_security_scan_configuration [CRUD]
- Org_ec2_deep_inspection_configuration [U]
- Filter [CUD]
- Cis_scan_configuration [CUD]

📖 [Full inspector2 documentation](services/inspector2.md)

### 131. Mq

**Resources**: 7

- Tags [CD]
- User [CRUD]
- Configuration [CRUD]
- Configuration_revision [R]
- Broker [CRUD]
- Broker_instance_options [R]
- Broker_engine_types [R]

📖 [Full mq documentation](services/mq.md)

### 132. Osis

**Resources**: 5

- Pipeline [CRUD]
- Pipeline_change_progress [R]
- Pipeline_blueprint [R]
- Pipeline_endpoint [CD]
- Resource_policy [CRD]

📖 [Full osis documentation](services/osis.md)

### 133. Bcm_recommended_actions

**Resources**: 0


📖 [Full bcm_recommended_actions documentation](services/bcm_recommended_actions.md)

### 134. Direct_connect

**Resources**: 30

- Interconnect_loa [R]
- Virtual_interfaces [R]
- Public_virtual_interface [C]
- Direct_connect_gateway_association_proposal [CD]
- Connections_on_interconnect [R]
- Direct_connect_gateways [R]
- Virtual_gateways [R]
- Customer_metadata [R]
- Direct_connect_gateway [CUD]
- Bgp_peer [CD]
- Lag [CUD]
- Direct_connect_gateway_associations [R]
- Connections [R]
- Connection [CUD]
- Transit_virtual_interface [C]
- Virtual_interface_attributes [U]
- Private_virtual_interface [C]
- Locations [R]
- Tags [R]
- Hosted_connections [R]
- Interconnect [CD]
- Loa [R]
- Direct_connect_gateway_attachments [R]
- Connection_loa [R]
- Lags [R]
- Virtual_interface [D]
- Direct_connect_gateway_association_proposals [R]
- Interconnects [R]
- Direct_connect_gateway_association [CUD]
- Router_configuration [R]

📖 [Full direct_connect documentation](services/direct_connect.md)

### 135. Opensearchserverless

**Resources**: 5

- Security_policy [C]
- Policies_stats [R]
- Vpc_endpoint [U]
- Lifecycle_policy [C]
- Account_settings [RU]

📖 [Full opensearchserverless documentation](services/opensearchserverless.md)

### 136. Personalize_runtime

**Resources**: 3

- Personalized_ranking [R]
- Recommendations [R]
- Action_recommendations [R]

📖 [Full personalize_runtime documentation](services/personalize_runtime.md)

### 137. Emr

**Resources**: 16

- Job_flows [R]
- Cluster_session_credentials [R]
- On_cluster_app_ui_presigned_url [R]
- Persistent_app_ui_presigned_url [R]
- Auto_termination_policy [CR]
- Release_label [R]
- Cluster [R]
- Notebook_execution [R]
- Persistent_app_ui [CR]
- Studio_session_mapping [CRUD]
- Block_public_access_configuration [CR]
- Step [R]
- Auto_scaling_policy [C]
- Studio [CRUD]
- Security_configuration [CRD]
- Managed_scaling_policy [CR]

📖 [Full emr documentation](services/emr.md)

### 138. Marketplace_agreement

**Resources**: 2

- Agreement_terms [R]
- Agreement [R]

📖 [Full marketplace_agreement documentation](services/marketplace_agreement.md)

### 139. Chime_sdk_messaging

**Resources**: 14

- Channel_ban [CRD]
- Channel_message [RUD]
- Channel_moderated_by_app_instance_user [R]
- Channel_read_marker [U]
- Channel_expiration_settings [C]
- Messaging_streaming_configurations [CRD]
- Channel_moderator [CRD]
- Channel_membership_preferences [CR]
- Channel_message_status [R]
- Messaging_session_endpoint [R]
- Channel [CRUD]
- Channel_flow [CRUD]
- Channel_membership [CRD]
- Channel_membership_for_app_instance_user [R]

📖 [Full chime_sdk_messaging documentation](services/chime_sdk_messaging.md)

### 140. Ram

**Resources**: 7

- Resource_policies [R]
- Resource_share_invitations [R]
- Resource_share [CUD]
- Permission [CRD]
- Resource_shares [R]
- Permission_version [CD]
- Resource_share_associations [R]

📖 [Full ram documentation](services/ram.md)

### 141. Batch

**Resources**: 13

- Job_queue_snapshot [R]
- Scheduling_policy [CUD]
- Service_environments [R]
- Scheduling_policies [R]
- Service_environment [CUD]
- Job_definitions [R]
- Jobs [R]
- Job_queues [R]
- Job_queue [CUD]
- Consumable_resource [CRUD]
- Service_job [R]
- Compute_environments [R]
- Compute_environment [CUD]

📖 [Full batch documentation](services/batch.md)

### 142. Wafv2

**Resources**: 19

- Managed_rule_set_versions [C]
- Sampled_requests [R]
- Api_key [CD]
- Mobile_sdk_release [R]
- Permission_policy [CRD]
- Rule_group [CRUD]
- Managed_products_by_vendor [R]
- Firewall_manager_rule_groups [D]
- Regex_pattern_set [CRUD]
- Web_acl [CRUD]
- Ip_set [CRUD]
- Managed_rule_group [R]
- Decrypted_api_key [R]
- All_managed_products [R]
- Managed_rule_set_version_expiry_date [U]
- Managed_rule_set [R]
- Rate_based_statement_managed_keys [R]
- Web_acl_for_resource [R]
- Logging_configuration [CRD]

📖 [Full wafv2 documentation](services/wafv2.md)

### 143. Notificationscontacts

**Resources**: 0


📖 [Full notificationscontacts documentation](services/notificationscontacts.md)

### 144. Evs

**Resources**: 0


📖 [Full evs documentation](services/evs.md)

### 145. Elastic_transcoder

**Resources**: 5

- Pipeline_status [U]
- Pipeline_notifications [U]
- Job [C]
- Pipeline [CUD]
- Preset [CD]

📖 [Full elastic_transcoder documentation](services/elastic_transcoder.md)

### 146. Vpc_lattice

**Resources**: 2

- Auth_policy [CRD]
- Resource_policy [CRD]

📖 [Full vpc_lattice documentation](services/vpc_lattice.md)

### 147. Marketplace_deployment

**Resources**: 0


📖 [Full marketplace_deployment documentation](services/marketplace_deployment.md)

### 148. Ivschat

**Resources**: 4

- Logging_configuration [CRUD]
- Message [D]
- Room [CRUD]
- Chat_token [C]

📖 [Full ivschat documentation](services/ivschat.md)

### 149. Xray

**Resources**: 22

- Encryption_config [CR]
- Time_series_service_statistics [R]
- Trace_segment_destination [RU]
- Insight_summaries [R]
- Sampling_statistic_summaries [R]
- Resource_policy [CD]
- Insight [R]
- Sampling_targets [R]
- Sampling_rules [R]
- Group [CRUD]
- Retrieved_traces_graph [R]
- Trace_graph [R]
- Telemetry_records [C]
- Trace_summaries [R]
- Groups [R]
- Trace_segments [C]
- Indexing_rule [U]
- Service_graph [R]
- Sampling_rule [CUD]
- Indexing_rules [R]
- Insight_impact_graph [R]
- Insight_events [R]

📖 [Full xray documentation](services/xray.md)

### 150. Iot_jobs_data_plane

**Resources**: 2

- Pending_job_executions [R]
- Job_execution [RU]

📖 [Full iot_jobs_data_plane documentation](services/iot_jobs_data_plane.md)

### 151. Lex_model_building_service

**Resources**: 25

- Slot_types [R]
- Bot_alias [CRD]
- Builtin_intents [R]
- Import [R]
- Bot_channel_association [RD]
- Intent [CRD]
- Bot [CRD]
- Builtin_slot_types [R]
- Intent_versions [R]
- Bot_versions [R]
- Migration [R]
- Slot_type_version [CD]
- Utterances [D]
- Bot_aliases [R]
- Migrations [R]
- Slot_type_versions [R]
- Utterances_view [R]
- Intent_version [CD]
- Slot_type [CRD]
- Builtin_intent [R]
- Bot_channel_associations [R]
- Bot_version [CD]
- Bots [R]
- Export [R]
- Intents [R]

📖 [Full lex_model_building_service documentation](services/lex_model_building_service.md)

### 152. Customer_profiles

**Resources**: 26

- Segment_snapshot [CR]
- Upload_job [CR]
- Integration [CRD]
- Profile_object_type_template [R]
- Workflow_steps [R]
- Profile_object_type [CRD]
- Profile [CUD]
- Profile_object [CD]
- Event_trigger [CRUD]
- Segment_membership [R]
- Event_stream [CRD]
- Integration_workflow [C]
- Domain_layout [CRUD]
- Upload_job_path [R]
- Segment_definition [CRD]
- Calculated_attribute_for_profile [R]
- Domain [CRUD]
- Profile_history_record [R]
- Similar_profiles [R]
- Auto_merging_preview [R]
- Identity_resolution_job [R]
- Segment_estimate [CR]
- Matches [R]
- Profile_key [D]
- Calculated_attribute_definition [CRUD]
- Workflow [RD]

📖 [Full customer_profiles documentation](services/customer_profiles.md)

### 153. Amplify

**Resources**: 8

- Deployment [C]
- App [CRUD]
- Job [RD]
- Backend_environment [CRD]
- Domain_association [CRUD]
- Webhook [CRUD]
- Branch [CRUD]
- Artifact_url [R]

📖 [Full amplify documentation](services/amplify.md)

### 154. Service_catalog_appregistry

**Resources**: 4

- Attribute_group [CRUD]
- Configuration [CR]
- Application [CRUD]
- Associated_resource [R]

📖 [Full service_catalog_appregistry documentation](services/service_catalog_appregistry.md)

### 155. Iotsecuretunneling

**Resources**: 1

- Tunnel [R]

📖 [Full iotsecuretunneling documentation](services/iotsecuretunneling.md)

### 156. Backup_gateway

**Resources**: 0


📖 [Full backup_gateway documentation](services/backup_gateway.md)

### 157. Bedrock_runtime

**Resources**: 0


📖 [Full bedrock_runtime documentation](services/bedrock_runtime.md)

### 158. Machine_learning

**Resources**: 13

- Data_source_from_redshift [C]
- Data_source [RUD]
- Tags [RD]
- Data_source_from_rds [C]
- Batch_predictions [R]
- Data_source_from_s3 [C]
- Ml_models [R]
- Evaluation [CRUD]
- Batch_prediction [CRUD]
- Realtime_endpoint [CD]
- Evaluations [R]
- Data_sources [R]
- Ml_model [CRUD]

📖 [Full machine_learning documentation](services/machine_learning.md)

### 159. Kendra

**Resources**: 12

- Thesaurus [CRUD]
- Principal_mapping [CRD]
- Index [CRUD]
- Faq [CRD]
- Data_source [CRUD]
- Query_suggestions_block_list [CRUD]
- Snapshots [R]
- Featured_results_set [CRU]
- Query_suggestions_config [RU]
- Query_suggestions [R]
- Access_control_configuration [CRUD]
- Experience [CRUD]

📖 [Full kendra documentation](services/kendra.md)

### 160. Route_53

**Resources**: 24

- Traffic_policy_comment [U]
- Vpc_association_authorization [CD]
- Hosted_zone_comment [U]
- Checker_ip_ranges [R]
- Health_check_last_failure_reason [R]
- Cidr_collection [CD]
- Health_check [CRUD]
- Traffic_policy_version [C]
- Health_check_count [R]
- Reusable_delegation_set [CRD]
- Hosted_zone_count [R]
- Change [R]
- Geo_location [R]
- Query_logging_config [CRD]
- Health_check_status [R]
- Hosted_zone_limit [R]
- Traffic_policy [CRD]
- Reusable_delegation_set_limit [R]
- Traffic_policy_instance [CRUD]
- Key_signing_key [CD]
- Hosted_zone [CRD]
- Account_limit [R]
- Traffic_policy_instance_count [R]
- Dnssec [R]

📖 [Full route_53 documentation](services/route_53.md)

### 161. Cloudtrail_data

**Resources**: 1

- Audit_events [C]

📖 [Full cloudtrail_data documentation](services/cloudtrail_data.md)

### 162. Connectparticipant

**Resources**: 5

- Authentication_url [R]
- Participant_connection [C]
- Transcript [R]
- View [R]
- Attachment [R]

📖 [Full connectparticipant documentation](services/connectparticipant.md)

### 163. Savingsplans

**Resources**: 6

- Savings_plans [R]
- Savings_plans_offering_rates [R]
- Queued_savings_plan [D]
- Savings_plans_offerings [R]
- Savings_plan [C]
- Savings_plan_rates [R]

📖 [Full savingsplans documentation](services/savingsplans.md)

### 164. Codepipeline

**Resources**: 14

- Third_party_job_details [R]
- Pipeline [CRUD]
- Job_success_result [C]
- Pipeline_state [R]
- Approval_result [C]
- Webhook [CD]
- Job_details [R]
- Action_revision [C]
- Pipeline_execution [R]
- Third_party_job_success_result [C]
- Job_failure_result [C]
- Third_party_job_failure_result [C]
- Action_type [RU]
- Custom_action_type [CD]

📖 [Full codepipeline documentation](services/codepipeline.md)

### 165. Bedrock_agentcore_control

**Resources**: 1

- Token_vault [R]

📖 [Full bedrock_agentcore_control documentation](services/bedrock_agentcore_control.md)

### 166. Codecommit

**Resources**: 36

- Comments_for_compared_commit [R]
- Pull_request_description [U]
- Commit [CR]
- Repository [CRD]
- Comment_reaction [C]
- Folder [R]
- Pull_request_events [R]
- Approval_rule_template_description [U]
- Comment_content [D]
- Pull_request_approval_states [R]
- Pull_request_approval_rule [CD]
- Merge_conflicts [R]
- Comment_reactions [R]
- Default_branch [U]
- Pull_request_approval_state [U]
- Repository_description [U]
- Branch [CRD]
- Repository_triggers [CR]
- File [CRD]
- Pull_request_status [U]
- Blob [R]
- Pull_request [CR]
- Comment [RU]
- Comments_for_pull_request [R]
- Approval_rule_template_name [U]
- Pull_request_approval_rule_content [U]
- Repository_name [U]
- Approval_rule_template [CRD]
- Merge_commit [R]
- Repository_encryption_key [U]
- Approval_rule_template_content [U]
- Pull_request_override_state [R]
- Merge_options [R]
- Pull_request_title [U]
- Differences [R]
- Unreferenced_merge_commit [C]

📖 [Full codecommit documentation](services/codecommit.md)

### 167. Glacier

**Resources**: 8

- Data_retrieval_policy [R]
- Job [R]
- Job_output [R]
- Archive [D]
- Vault_lock [R]
- Vault [CRD]
- Vault_notifications [RD]
- Vault_access_policy [RD]

📖 [Full glacier documentation](services/glacier.md)

### 168. Qconnect

**Resources**: 0


📖 [Full qconnect documentation](services/qconnect.md)

### 169. Taxsettings

**Resources**: 6

- Tax_exemption_types [R]
- Tax_registration [CRD]
- Supplemental_tax_registration [CD]
- Tax_registration_document [R]
- Tax_exemption [C]
- Tax_inheritance [CR]

📖 [Full taxsettings documentation](services/taxsettings.md)

### 170. Marketplace_catalog

**Resources**: 3

- Change_set [R]
- Resource_policy [CRD]
- Entity [R]

📖 [Full marketplace_catalog documentation](services/marketplace_catalog.md)

### 171. Database_migration_service

**Resources**: 59

- Applicable_individual_assessments [R]
- Data_providers [R]
- Connection [D]
- Endpoints [R]
- Instance_profile [CD]
- Certificates [R]
- Replication_configs [R]
- Replication_instances [R]
- Pending_maintenance_actions [R]
- Fleet_advisor_databases [RD]
- Endpoint_settings [R]
- Replications [R]
- Replication_task_assessment_run [D]
- Replication_subnet_groups [R]
- Data_migrations [R]
- Migration_project [CD]
- Recommendations [R]
- Replication_instance_task_logs [R]
- Subscriptions_to_event_bridge [U]
- Replication_tasks [R]
- Endpoint_types [R]
- Connections [R]
- Migration_projects [R]
- Metadata_model_imports [R]
- Fleet_advisor_schemas [R]
- Event_categories [R]
- Fleet_advisor_collectors [R]
- Recommendation_limitations [R]
- Events [R]
- Conversion_configuration [R]
- Event_subscriptions [R]
- Orderable_replication_instances [R]
- Fleet_advisor_schema_object_summary [R]
- Event_subscription [CD]
- Metadata_model_exports_to_target [R]
- Replication_task_individual_assessments [R]
- Table_statistics [R]
- Data_migration [CD]
- Replication_config [CD]
- Replication_subnet_group [CD]
- Data_provider [CD]
- Replication_instance [CD]
- Schemas [R]
- Fleet_advisor_collector [CD]
- Replication_task_assessment_results [R]
- Replication_task_assessment_runs [R]
- Metadata_model_assessments [R]
- Account_attributes [R]
- Engine_versions [R]
- Fleet_advisor_lsa_analysis [R]
- Refresh_schemas_status [R]
- Extension_pack_associations [R]
- Certificate [D]
- Replication_task [CD]
- Instance_profiles [R]
- Replication_table_statistics [R]
- Endpoint [CD]
- Metadata_model_conversions [R]
- Metadata_model_exports_as_script [R]

📖 [Full database_migration_service documentation](services/database_migration_service.md)

### 172. Lookoutequipment

**Resources**: 10

- Resource_policy [CRD]
- Active_model_version [U]
- Model [CRUD]
- Inference_scheduler [CRUD]
- Dataset [CRD]
- Retraining_scheduler [CRUD]
- Data_ingestion_job [R]
- Label_group [CRUD]
- Model_version [R]
- Label [CRD]

📖 [Full lookoutequipment documentation](services/lookoutequipment.md)

### 173. Account

**Resources**: 0


📖 [Full account documentation](services/account.md)

### 174. Neptune

**Resources**: 29

- Db_instance [CD]
- Db_subnet_group [CD]
- Engine_default_cluster_parameters [R]
- Orderable_db_instance_options [R]
- Db_parameter_group [CD]
- Event_subscriptions [R]
- Event_categories [R]
- Db_cluster_snapshot_attributes [R]
- Pending_maintenance_actions [R]
- Db_cluster_endpoints [R]
- Valid_db_instance_modifications [R]
- Db_cluster_parameter_group [CD]
- Db_cluster_snapshot [CD]
- Events [R]
- Global_cluster [CD]
- Db_engine_versions [R]
- Global_clusters [R]
- Db_instances [R]
- Db_cluster_endpoint [CD]
- Db_cluster_parameter_groups [R]
- Db_cluster_snapshots [R]
- Db_subnet_groups [R]
- Db_parameter_groups [R]
- Event_subscription [CD]
- Db_cluster [CD]
- Db_parameters [R]
- Engine_default_parameters [R]
- Db_cluster_parameters [R]
- Db_clusters [R]

📖 [Full neptune documentation](services/neptune.md)

### 175. Qapps

**Resources**: 7

- Q_app [CRUD]
- Library_item [CRUD]
- Presigned_url [C]
- Q_app_session [RU]
- Q_app_permissions [RU]
- Q_app_session_metadata [RU]
- Library_item_metadata [U]

📖 [Full qapps documentation](services/qapps.md)

### 176. Scheduler

**Resources**: 0


📖 [Full scheduler documentation](services/scheduler.md)

### 177. Connect_contact_lens

**Resources**: 0


📖 [Full connect_contact_lens documentation](services/connect_contact_lens.md)

### 178. Eventbridge

**Resources**: 13

- Replay [R]
- Events [C]
- Endpoint [CRUD]
- Connection [CRUD]
- Event_bus [CRUD]
- Event_source [R]
- Partner_events [C]
- Partner_event_source [CRD]
- Permission [C]
- Targets [C]
- Rule [CRD]
- Api_destination [CRUD]
- Archive [CRUD]

📖 [Full eventbridge documentation](services/eventbridge.md)

### 179. Devops_guru

**Resources**: 13

- Insight [RD]
- Organization_resource_collection_health [R]
- Feedback [CR]
- Cost_estimation [R]
- Resource_collection_health [R]
- Account_overview [R]
- Event_sources_config [RU]
- Organization_overview [R]
- Anomaly [R]
- Service_integration [RU]
- Account_health [R]
- Resource_collection [RU]
- Organization_health [R]

📖 [Full devops_guru documentation](services/devops_guru.md)

### 180. Config_service

**Resources**: 60

- Conformance_pack_compliance_details [R]
- Organization_conformance_pack_detailed_status [R]
- Pending_aggregation_request [D]
- Configuration_aggregator_sources_status [R]
- Organization_config_rule [CD]
- Aggregate_compliance_by_config_rules [R]
- Compliance_by_resource [R]
- Discovered_resource_counts [R]
- Custom_rule_policy [R]
- Compliance_summary_by_config_rule [R]
- Stored_query [CRD]
- Delivery_channel_status [R]
- Conformance_pack_status [R]
- Conformance_pack_compliance [R]
- Evaluation_results [D]
- Compliance_by_config_rule [R]
- Organization_config_rule_detailed_status [R]
- Configuration_recorder [CD]
- Remediation_execution_status [R]
- Aggregate_conformance_pack_compliance_summary [R]
- Config_rule [CD]
- Organization_custom_rule_policy [R]
- Conformance_pack_compliance_summary [R]
- Aggregation_authorization [CD]
- Remediation_configurations [CR]
- Remediation_configuration [D]
- Aggregate_discovered_resource_counts [R]
- Remediation_exceptions [CRD]
- Organization_conformance_pack_statuses [R]
- External_evaluation [C]
- Compliance_details_by_config_rule [R]
- Aggregate_compliance_by_conformance_packs [R]
- Evaluations [C]
- Conformance_packs [R]
- Aggregate_compliance_details_by_config_rule [R]
- Config_rule_evaluation_status [R]
- Retention_configurations [R]
- Configuration_recorders [R]
- Organization_config_rules [R]
- Retention_configuration [CD]
- Configuration_aggregator [CD]
- Aggregation_authorizations [R]
- Compliance_summary_by_resource_type [R]
- Resource_config [CD]
- Aggregate_resource_config [R]
- Service_linked_configuration_recorder [CD]
- Resource_config_history [R]
- Delivery_channels [R]
- Configuration_aggregators [R]
- Config_rules [R]
- Conformance_pack [CD]
- Organization_config_rule_statuses [R]
- Organization_conformance_packs [R]
- Resource_evaluation_summary [R]
- Configuration_recorder_status [R]
- Aggregate_config_rule_compliance_summary [R]
- Pending_aggregation_requests [R]
- Compliance_details_by_resource [R]
- Organization_conformance_pack [CD]
- Delivery_channel [CD]

📖 [Full config_service documentation](services/config_service.md)

### 181. Bedrock_data_automation_runtime

**Resources**: 0


📖 [Full bedrock_data_automation_runtime documentation](services/bedrock_data_automation_runtime.md)

### 182. Dsql

**Resources**: 0


📖 [Full dsql documentation](services/dsql.md)

### 183. Eks

**Resources**: 18

- Nodegroup_config [U]
- Nodegroup_version [U]
- Fargate_profile [CRD]
- Nodegroup [CRD]
- Addon_versions [R]
- Insights_refresh [R]
- Cluster [CRD]
- Addon [CRUD]
- Identity_provider_config [R]
- Insight [R]
- Addon_configuration [R]
- Pod_identity_association [CRUD]
- Access_entry [CRUD]
- Cluster_versions [R]
- Cluster_version [U]
- Cluster_config [U]
- Update [R]
- Eks_anywhere_subscription [CRUD]

📖 [Full eks documentation](services/eks.md)

### 184. Opensearch

**Resources**: 29

- Domain_health [R]
- Reserved_instance_offerings [R]
- Upgrade_status [R]
- Reserved_instances [R]
- Outbound_connections [R]
- Domain_config [RU]
- Domain [CRD]
- Packages [R]
- Inbound_connections [R]
- Domain_change_progress [R]
- Instance_type_limits [R]
- Outbound_connection [CD]
- Scheduled_action [U]
- Package [CUD]
- Dry_run_progress [R]
- Domain_auto_tunes [R]
- Direct_query_data_source [RUD]
- Inbound_connection [D]
- Data_source [RUD]
- Domains [R]
- Vpc_endpoint [CUD]
- Application [CRUD]
- Compatible_versions [R]
- Vpc_endpoints [R]
- Package_scope [U]
- Domain_nodes [R]
- Package_version_history [R]
- Upgrade_history [R]
- Domain_maintenance_status [R]

📖 [Full opensearch documentation](services/opensearch.md)

### 185. Sqs

**Resources**: 5

- Message [D]
- Message_batch [D]
- Queue_url [R]
- Queue_attributes [R]
- Queue [CD]

📖 [Full sqs documentation](services/sqs.md)

### 186. Elastic_load_balancing

**Resources**: 37

- Listener_certificates [R]
- Resource_policy [R]
- Target_group [CD]
- Target_groups [R]
- Listener_attributes [R]
- Ssl_policies [R]
- Listeners [R]
- Shared_trust_store_association [D]
- Load_balancers [R]
- Account_limits [R]
- Capacity_reservation [R]
- Trust_store [CD]
- Load_balancer_attributes [R]
- Tags [R]
- Target_group_attributes [R]
- Trust_store_associations [R]
- Trust_store_revocations [R]
- Load_balancer [CD]
- Rules [R]
- Target_health [R]
- Trust_stores [R]
- Trust_store_revocation_content [R]
- Listener [CD]
- Trust_store_ca_certificates_bundle [R]
- Rule [CD]
- Load_balancer [CD]
- Load_balancer_listeners [CD]
- Load_balancer_policies [R]
- Tags [R]
- Load_balancer_policy_types [R]
- Account_limits [R]
- Lb_cookie_stickiness_policy [C]
- Load_balancer_attributes [R]
- Load_balancers [R]
- App_cookie_stickiness_policy [C]
- Load_balancer_policy [CD]
- Instance_health [R]

📖 [Full elastic_load_balancing documentation](services/elastic_load_balancing.md)

### 187. Apigatewaymanagementapi

**Resources**: 1

- Connection [RD]

📖 [Full apigatewaymanagementapi documentation](services/apigatewaymanagementapi.md)

### 188. Internetmonitor

**Resources**: 0


📖 [Full internetmonitor documentation](services/internetmonitor.md)

### 189. Auditmanager

**Resources**: 23

- Control [CRUD]
- Assessment_control [U]
- Assessment_framework_share [UD]
- Evidence_file_upload_url [R]
- Delegations [R]
- Evidence_folders_by_assessment [R]
- Evidence_folders_by_assessment_control [R]
- Assessment_status [U]
- Organization_admin_account [R]
- Change_logs [R]
- Services_in_scope [R]
- Assessment_framework [CRUD]
- Account_status [R]
- Assessment_control_set_status [U]
- Evidence_by_evidence_folder [R]
- Assessment_report_url [R]
- Evidence_folder [R]
- Settings [RU]
- Assessment [CRUD]
- Evidence [R]
- Insights [R]
- Insights_by_assessment [R]
- Assessment_report [CD]

📖 [Full auditmanager documentation](services/auditmanager.md)

### 190. Athena

**Resources**: 19

- Capacity_reservation [CRUD]
- Query_execution [R]
- Query_runtime_statistics [R]
- Named_query [CRUD]
- Presigned_notebook_url [C]
- Notebook_metadata [RU]
- Capacity_assignment_configuration [CR]
- Prepared_statement [CRUD]
- Calculation_execution [R]
- Work_group [CRUD]
- Query_results [R]
- Session [R]
- Session_status [R]
- Calculation_execution_code [R]
- Notebook [CUD]
- Database [R]
- Table_metadata [R]
- Data_catalog [CRUD]
- Calculation_execution_status [R]

📖 [Full athena documentation](services/athena.md)

### 191. Signer

**Resources**: 4

- Signing_platform [R]
- Signing_job [R]
- Signing_profile [CR]
- Revocation_status [R]

📖 [Full signer documentation](services/signer.md)

### 192. Invoicing

**Resources**: 1

- Invoice_unit [CRUD]

📖 [Full invoicing documentation](services/invoicing.md)

### 193. Application_auto_scaling

**Resources**: 7

- Scalable_targets [R]
- Predictive_scaling_forecast [R]
- Scaling_policy [CD]
- Scaling_activities [R]
- Scaling_policies [R]
- Scheduled_action [CD]
- Scheduled_actions [R]

📖 [Full application_auto_scaling documentation](services/application_auto_scaling.md)

### 194. Voice_id

**Resources**: 5

- Fraudster_registration_job [R]
- Fraudster [RD]
- Speaker_enrollment_job [R]
- Watchlist [CRUD]
- Speaker [RD]

📖 [Full voice_id documentation](services/voice_id.md)

### 195. Trustedadvisor

**Resources**: 4

- Recommendation [R]
- Recommendation_lifecycle [U]
- Organization_recommendation_lifecycle [U]
- Organization_recommendation [R]

📖 [Full trustedadvisor documentation](services/trustedadvisor.md)

### 196. Iot_events_data

**Resources**: 2

- Alarm [R]
- Detector [R]

📖 [Full iot_events_data documentation](services/iot_events_data.md)

### 197. Docdb

**Resources**: 23

- Db_cluster_parameter_groups [R]
- Db_cluster_snapshots [R]
- Db_subnet_groups [R]
- Engine_default_cluster_parameters [R]
- Db_cluster_parameter_group [CD]
- Event_categories [R]
- Global_clusters [R]
- Event_subscription [CD]
- Pending_maintenance_actions [R]
- Certificates [R]
- Db_clusters [R]
- Orderable_db_instance_options [R]
- Db_engine_versions [R]
- Db_cluster_parameters [R]
- Db_instances [R]
- Events [R]
- Db_instance [CD]
- Event_subscriptions [R]
- Db_cluster_snapshot [CD]
- Global_cluster [CD]
- Db_cluster_snapshot_attributes [R]
- Db_subnet_group [CD]
- Db_cluster [CD]

📖 [Full docdb documentation](services/docdb.md)

### 198. Sso_admin

**Resources**: 19

- Account_assignment_deletion_status [R]
- Application_assignment_configuration [CR]
- Inline_policy_from_permission_set [D]
- Permissions_boundary_from_permission_set [D]
- Permissions_boundary_for_permission_set [R]
- Application [CRUD]
- Application_assignment [CRD]
- Account_assignment [CD]
- Account_assignment_creation_status [R]
- Instance [CRUD]
- Permission_set [CRUD]
- Application_provider [R]
- Inline_policy_for_permission_set [R]
- Instance_access_control_attribute_configuration [CRUD]
- Trusted_token_issuer [CRUD]
- Application_session_configuration [CR]
- Inline_policy_to_permission_set [C]
- Permission_set_provisioning_status [R]
- Permissions_boundary_to_permission_set [C]

📖 [Full sso_admin documentation](services/sso_admin.md)

### 199. Neptune_graph

**Resources**: 2

- Graph_summary [R]
- Query [R]

📖 [Full neptune_graph documentation](services/neptune_graph.md)

### 200. Ssm_sap

**Resources**: 7

- Resource_permission [CRD]
- Operation [R]
- Component [R]
- Database [R]
- Application_settings [U]
- Application [R]
- Configuration_check_operation [R]

📖 [Full ssm_sap documentation](services/ssm_sap.md)

### 201. Ssm_contacts

**Resources**: 7

- Rotation_override [CRD]
- Rotation [CRUD]
- Page [R]
- Contact_policy [CR]
- Engagement [R]
- Contact_channel [CRUD]
- Contact [CRUD]

📖 [Full ssm_contacts documentation](services/ssm_contacts.md)

### 202. Pricing

**Resources**: 4

- Services [R]
- Price_list_file_url [R]
- Attribute_values [R]
- Products [R]

📖 [Full pricing documentation](services/pricing.md)

### 203. Forecast

**Resources**: 16

- Dataset_group [CRUD]
- Forecast_export_job [CRD]
- Resource_tree [D]
- Accuracy_metrics [R]
- Dataset [CRD]
- Monitor [CRD]
- Dataset_import_job [CRD]
- Predictor_backtest_export_job [CRD]
- Explainability_export [CRD]
- Explainability [CRD]
- What_if_analysis [CRD]
- What_if_forecast_export [CRD]
- Forecast [CRD]
- What_if_forecast [CRD]
- Auto_predictor [CR]
- Predictor [CRD]

📖 [Full forecast documentation](services/forecast.md)

### 204. Compute_optimizer

**Resources**: 17

- Effective_recommendation_preferences [R]
- Rds_database_recommendations [R]
- Ec2_instance_recommendations [R]
- Rds_database_recommendation_projected_metrics [R]
- Ecs_service_recommendation_projected_metrics [R]
- Ecs_service_recommendations [R]
- Auto_scaling_group_recommendations [R]
- Ebs_volume_recommendations [R]
- Enrollment_status [RU]
- Ec2_recommendation_projected_metrics [R]
- Recommendation_summaries [R]
- Lambda_function_recommendations [R]
- Enrollment_statuses_for_organization [R]
- Idle_recommendations [R]
- Recommendation_export_jobs [R]
- Recommendation_preferences [CRD]
- License_recommendations [R]

📖 [Full compute_optimizer documentation](services/compute_optimizer.md)

### 205. Iotfleetwise

**Resources**: 4

- Logging_options [CR]
- Register_account_status [R]
- Vehicle_status [R]
- Encryption_configuration [CR]

📖 [Full iotfleetwise documentation](services/iotfleetwise.md)

### 206. Auto_scaling

**Resources**: 31

- Instance_refreshes [R]
- Termination_policy_types [R]
- Scheduled_actions [R]
- Notification_configuration [CD]
- Lifecycle_hooks [R]
- Or_update_tags [C]
- Load_balancer_target_groups [R]
- Scaling_activities [R]
- Launch_configurations [R]
- Scheduled_update_group_action [C]
- Adjustment_types [R]
- Warm_pool [CRD]
- Metric_collection_types [R]
- Launch_configuration [CD]
- Policy [D]
- Load_balancers [R]
- Lifecycle_hook [CD]
- Tags [RD]
- Policies [R]
- Traffic_sources [R]
- Auto_scaling_groups [R]
- Predictive_scaling_forecast [R]
- Scaling_policy [C]
- Notification_configurations [R]
- Lifecycle_hook_types [R]
- Scaling_process_types [R]
- Account_limits [R]
- Auto_scaling_notification_types [R]
- Auto_scaling_group [CUD]
- Scheduled_action [D]
- Auto_scaling_instances [R]

📖 [Full auto_scaling documentation](services/auto_scaling.md)

### 207. Bcm_pricing_calculator

**Resources**: 1

- Preferences [RU]

📖 [Full bcm_pricing_calculator documentation](services/bcm_pricing_calculator.md)

### 208. Bedrock_agent_runtime

**Resources**: 0


📖 [Full bedrock_agent_runtime documentation](services/bedrock_agent_runtime.md)

### 209. S3

**Resources**: 38

- Bucket_intelligent_tiering_configuration [CRD]
- Object_retention [CR]
- Bucket_acl [CR]
- Object [CRD]
- Bucket_notification_configuration [CR]
- Object_attributes [R]
- Bucket_lifecycle_configuration [CR]
- Bucket_encryption [CRD]
- Session [C]
- Bucket_analytics_configuration [CRD]
- Object_acl [CR]
- Bucket_location [R]
- Bucket_metadata_journal_table_configuration [U]
- Bucket_cors [CRD]
- Bucket_replication [CRD]
- Bucket_metadata_inventory_table_configuration [U]
- Bucket_ownership_controls [CRD]
- Object_tagging [CRD]
- Object_torrent [R]
- Bucket_tagging [CRD]
- Public_access_block [CRD]
- Bucket_website [CRD]
- Bucket_accelerate_configuration [CR]
- Bucket_logging [CR]
- Objects [D]
- Multipart_upload [C]
- Bucket_request_payment [CR]
- Bucket_versioning [CR]
- Bucket_policy [CRD]
- Object_legal_hold [CR]
- Bucket_metadata_configuration [CRD]
- Bucket [CRD]
- Bucket_metadata_table_configuration [CRD]
- Bucket_lifecycle [D]
- Bucket_metrics_configuration [CRD]
- Bucket_policy_status [R]
- Object_lock_configuration [CR]
- Bucket_inventory_configuration [CRD]

📖 [Full s3 documentation](services/s3.md)

### 210. Mediatailor

**Resources**: 0


📖 [Full mediatailor documentation](services/mediatailor.md)

### 211. Dynamodb_streams

**Resources**: 3

- Records [R]
- Shard_iterator [R]
- Stream [R]

📖 [Full dynamodb_streams documentation](services/dynamodb_streams.md)

### 212. Iotsitewise

**Resources**: 26

- Time_series [RD]
- Execution [R]
- Asset_property [RU]
- Gateway [CRUD]
- Computation_model_execution_summary [R]
- Asset_property_value [R]
- Dashboard [CRUD]
- Action [R]
- Project [CRUD]
- Asset_model [CRUD]
- Asset_model_composite_model [CRUD]
- Logging_options [CR]
- Bulk_import_job [CR]
- Computation_model [CRUD]
- Default_encryption_configuration [CR]
- Dataset [CRUD]
- Asset [CRUD]
- Asset_model_interface_relationship [CRD]
- Asset_composite_model [R]
- Interpolated_asset_property_values [R]
- Gateway_capability_configuration [RU]
- Storage_configuration [CR]
- Asset_property_value_history [R]
- Portal [CRUD]
- Asset_property_aggregates [R]
- Access_policy [CRUD]

📖 [Full iotsitewise documentation](services/iotsitewise.md)

### 213. Emr_containers

**Resources**: 6

- Job_run [R]
- Managed_endpoint [CRD]
- Virtual_cluster [CRD]
- Security_configuration [CR]
- Managed_endpoint_session_credentials [R]
- Job_template [CRD]

📖 [Full emr_containers documentation](services/emr_containers.md)

### 214. Bcm_data_exports

**Resources**: 3

- Export [CRUD]
- Execution [R]
- Table [R]

📖 [Full bcm_data_exports documentation](services/bcm_data_exports.md)

### 215. Ecr_public

**Resources**: 10

- Image [C]
- Repository_catalog_data [CR]
- Images [R]
- Repository [CD]
- Image_tags [R]
- Repository_policy [RD]
- Registries [R]
- Registry_catalog_data [CR]
- Authorization_token [R]
- Repositories [R]

📖 [Full ecr_public documentation](services/ecr_public.md)

### 216. Pinpoint_sms

**Resources**: 47

- Configuration_set_event_destination [CUD]
- Configuration_set_event_destinations [R]
- Configuration_set [CD]
- Registration_version [C]
- Registrations [R]
- Registration_attachment [CD]
- Verified_destination_number [CD]
- Registration_association [C]
- Opted_out_number [CD]
- Phone_numbers [R]
- Account_default_protect_configuration [D]
- Keywords [R]
- Pool [CUD]
- Protect_configuration [CUD]
- Account_attributes [R]
- Registration_field_definitions [R]
- Registration_type_definitions [R]
- Media_message_spend_limit_override [D]
- Sender_id [U]
- Registration_versions [R]
- Message_feedback [C]
- Registration_section_definitions [R]
- Text_message_spend_limit_override [D]
- Account_limits [R]
- Protect_configurations [R]
- Registration_field_value [CD]
- Default_message_type [D]
- Registration_field_values [R]
- Opt_out_list [CD]
- Sender_ids [R]
- Protect_configuration_country_rule_set [RU]
- Resource_policy [CRD]
- Opted_out_numbers [R]
- Registration_attachments [R]
- Registration [CD]
- Event_destination [CUD]
- Protect_configuration_rule_set_number_override [CD]
- Opt_out_lists [R]
- Default_sender_id [D]
- Verified_destination_numbers [R]
- Pools [R]
- Configuration_set [CD]
- Voice_message_spend_limit_override [D]
- Configuration_sets [R]
- Phone_number [U]
- Keyword [CD]
- Spend_limits [R]

📖 [Full pinpoint_sms documentation](services/pinpoint_sms.md)

### 217. Api_gateway

**Resources**: 48

- Method_response [CRUD]
- Sdk_types [R]
- Integration_response [CRUD]
- Api_keys [R]
- Sdk_type [R]
- Usage_plan_key [CRD]
- Base_path_mappings [R]
- Gateway_response [CRUD]
- Models [R]
- Method [CRUD]
- Client_certificate [RUD]
- Gateway_responses [R]
- Stage [CRUD]
- Domain_names [R]
- Rest_apis [R]
- Documentation_part [CRUD]
- Documentation_versions [R]
- Export [R]
- Stages [R]
- Deployment [CRUD]
- Documentation_version [CRUD]
- Domain_name [CRUD]
- Authorizers [R]
- Domain_name_access_associations [R]
- Documentation_parts [R]
- Model_template [R]
- Api_key [CRUD]
- Sdk [R]
- Authorizer [CRUD]
- Deployments [R]
- Tags [R]
- Usage_plans [R]
- Model [CRUD]
- Client_certificates [R]
- Integration [CRUD]
- Vpc_links [R]
- Domain_name_access_association [CD]
- Account [RU]
- Request_validators [R]
- Resource [CRUD]
- Usage [RU]
- Usage_plan_keys [R]
- Request_validator [CRUD]
- Base_path_mapping [CRUD]
- Resources [R]
- Vpc_link [CRUD]
- Usage_plan [CRUD]
- Rest_api [CRUD]

📖 [Full api_gateway documentation](services/api_gateway.md)

### 218. Codeartifact

**Resources**: 16

- Repository [CRUD]
- Package_group [CRUD]
- Package_group_origin_configuration [U]
- Package_version_asset [R]
- Authorization_token [R]
- Associated_package_group [R]
- Package_versions [D]
- Repository_permissions_policy [CRD]
- Package_version_readme [R]
- Domain_permissions_policy [CRD]
- Domain [CRD]
- Package_version [R]
- Repository_endpoint [R]
- Package_origin_configuration [C]
- Package_versions_status [U]
- Package [RD]

📖 [Full codeartifact documentation](services/codeartifact.md)

### 219. Redshift_data

**Resources**: 4

- Statement [R]
- Statement_result_v2 [R]
- Table [R]
- Statement_result [R]

📖 [Full redshift_data documentation](services/redshift_data.md)

### 220. Ec2

**Resources**: 335

- Capacity_block_status [R]
- Vpc_endpoints [RD]
- Local_gateway_route_table_virtual_interface_group_associations [R]
- Vpn_connection_route [CD]
- Ipam_pool_allocations [R]
- Import_image_tasks [R]
- Vpc_endpoint [C]
- Network_interface_permission [CD]
- Network_insights_access_scope_analyses [R]
- Traffic_mirror_sessions [R]
- Vpc_attribute [R]
- Aws_network_performance_data [R]
- Vpc_peering_connection [CD]
- Route_table [CD]
- Instance_type_offerings [R]
- Mac_hosts [R]
- Addresses_attribute [R]
- Capacity_blocks [R]
- Iam_instance_profile_associations [R]
- Image_usage_reports [R]
- Launch_template [CD]
- Client_vpn_endpoints [R]
- Transit_gateway_prefix_list_references [R]
- Ipam_prefix_list_resolver_target [CD]
- Ipam_discovered_accounts [R]
- Launch_template_version [C]
- Capacity_block_extension_offerings [R]
- Transit_gateway_policy_tables [R]
- Dhcp_options [CRD]
- Network_insights_access_scope_content [R]
- Route [CD]
- Capacity_reservation_fleet [C]
- Capacity_reservation_by_splitting [C]
- Transit_gateway_connect [CD]
- Flow_logs [CRD]
- Vpc_endpoint_connection_notifications [RD]
- Transit_gateway_prefix_list_reference [CD]
- Fpga_images [R]
- Aws_network_performance_metric_subscriptions [R]
- Verified_access_groups [R]
- Image_usage_report_entries [R]
- Vpn_connection_device_sample_configuration [R]
- Route_server_peer [CD]
- Ipam_prefix_list_resolver_version_entries [R]
- Snapshot_block_public_access_state [R]
- Security_groups [R]
- Transit_gateway_attachment_propagations [R]
- Default_vpc [C]
- Transit_gateway_route_table_associations [R]
- Vpc_endpoint_service_configuration [C]
- Active_vpn_tunnel_status [R]
- Vpn_tunnel_replacement_status [R]
- Capacity_manager_data_exports [R]
- Vpn_connections [R]
- Snapshot_attribute [R]
- Network_interface_permissions [R]
- Transit_gateway_vpc_attachments [R]
- Verified_access_endpoints [R]
- Trunk_interface_associations [R]
- Classic_link_instances [R]
- Security_group_rule_descriptions_egress [U]
- Egress_only_internet_gateway [CD]
- Snapshot [CD]
- Ipam_prefix_list_resolvers [R]
- Network_interfaces [R]
- Traffic_mirror_filters [R]
- Client_vpn_route [CD]
- Allowed_images_settings [R]
- Internet_gateways [R]
- Route_server_peers [R]
- Egress_only_internet_gateways [R]
- Subnets [R]
- Snapshots [CR]
- Host_reservations [R]
- Image_block_public_access_state [R]
- Route_server_associations [R]
- Fpga_image [CD]
- Local_gateway_route_table_vpc_association [CD]
- Traffic_mirror_filter_rule [CD]
- Security_group [CD]
- Stale_security_groups [R]
- Host_reservation_purchase_preview [R]
- Locked_snapshots [R]
- Export_tasks [R]
- Transit_gateway_peering_attachment [CD]
- Ipam_prefix_list_resolver_targets [R]
- Volumes_modifications [R]
- Spot_price_history [R]
- Coip_cidr [CD]
- Queued_reserved_instances [D]
- Image_attribute [R]
- Spot_datafeed_subscription [CRD]
- Client_vpn_connections [R]
- Fast_snapshot_restores [R]
- Availability_zones [R]
- Transit_gateway_policy_table [CD]
- Vpc_endpoint_connection_notification [C]
- Host_reservation_offerings [R]
- Ipam_address_history [R]
- Console_screenshot [R]
- Replace_root_volume_task [C]
- Fast_launch_images [R]
- Placement_groups [R]
- Subnet_cidr_reservation [CD]
- Capacity_manager_attributes [R]
- Default_credit_specification [R]
- Principal_id_format [R]
- Vpn_connection [CD]
- Transit_gateway_route [CD]
- Ipam_discovered_public_addresses [R]
- Flow_logs_integration_template [R]
- Instance_status [R]
- Service_link_virtual_interfaces [R]
- Instance_connect_endpoints [R]
- Transit_gateway_multicast_domains [R]
- Transit_gateway_multicast_domain [CD]
- Verified_access_endpoint [CD]
- Network_insights_analysis [D]
- Instances [R]
- Instance_event_notification_attributes [R]
- Capacity_reservation_fleets [R]
- Ipam_external_resource_verification_tokens [R]
- Local_gateway_route_table_vpc_associations [R]
- Network_insights_access_scope_analysis_findings [R]
- Capacity_manager_organizations_access [U]
- Capacity_manager_metric_dimensions [R]
- Carrier_gateway [CD]
- Launch_template_data [R]
- Image_references [R]
- Reserved_instances_offerings [R]
- Conversion_tasks [R]
- Capacity_manager_metric_data [R]
- Default_subnet [C]
- Placement_group [CD]
- Hosts [R]
- Bundle_tasks [R]
- Tags [CRD]
- Vpc_block_public_access_exclusions [R]
- Instance_export_task [C]
- Instance_uefi_data [R]
- Verified_access_group [CD]
- Network_insights_access_scope_analysis [D]
- Password_data [R]
- Ipam_prefix_list_resolver [CD]
- Associated_enclave_certificate_iam_roles [R]
- Network_insights_access_scope [CD]
- Capacity_manager_data_export [CD]
- Network_insights_access_scopes [R]
- Vpc_endpoint_associations [R]
- Route_server_endpoint [CD]
- Nat_gateway [CD]
- Verified_access_trust_providers [R]
- Spot_fleet_request_history [R]
- Launch_templates [R]
- Managed_prefix_lists [R]
- Vpc_block_public_access_exclusion [CD]
- Capacity_reservation_usage [R]
- Coip_pool_usage [R]
- Transit_gateway_peering_attachments [R]
- Vpc_endpoint_connections [R]
- Subnet_cidr_reservations [R]
- Route_server_endpoints [R]
- Transit_gateways [R]
- Transit_gateway_multicast_domain_associations [R]
- Spot_instance_requests [R]
- Outpost_lags [R]
- Local_gateways [R]
- Client_vpn_authorization_rules [R]
- Key_pairs [R]
- Moving_addresses [R]
- Mac_system_integrity_protection_modification_task [C]
- Network_insights_analyses [R]
- Spot_fleet_instances [R]
- Managed_prefix_list [CD]
- Ipam_resource_discoveries [R]
- Local_gateway_virtual_interfaces [R]
- Volumes [R]
- Vpc_classic_link_dns_support [R]
- Vpc_peering_connections [R]
- Reserved_instances [R]
- Key_pair [CD]
- Import_snapshot_tasks [R]
- Ipam_scope [CD]
- Local_gateway_route [CD]
- Instance_connect_endpoint [CD]
- Identity_id_format [R]
- Traffic_mirror_filter_rules [R]
- Store_image_task [C]
- Delegate_mac_volume_ownership_task [C]
- Security_group_rules [R]
- Ipam_prefix_list_resolver_rules [R]
- Export_image_tasks [R]
- Image [C]
- Local_gateway_route_table [CD]
- Ipam_discovered_resource_cidrs [R]
- Transit_gateway_connects [R]
- Image_usage_report [CD]
- Aggregate_id_format [R]
- Vpc_endpoint_services [R]
- Client_vpn_routes [R]
- Ebs_default_kms_key_id [R]
- Verified_access_endpoint_targets [R]
- Volume_attribute [R]
- Capacity_reservation [C]
- Fpga_image_attribute [R]
- Ipam_byoasn [R]
- Volume [CD]
- Ipam_pools [R]
- Vpc_endpoint_service_configurations [RD]
- Regions [R]
- Capacity_reservation_billing_requests [R]
- Verified_access_trust_provider [CD]
- Restore_image_task [C]
- Verified_access_endpoint_policy [R]
- Public_ipv4_pools [R]
- Transit_gateway_route_table_announcements [R]
- Network_acls [R]
- Ipam_resource_discovery_associations [R]
- Network_insights_paths [R]
- Nat_gateways [R]
- Security_group_vpc_associations [R]
- Verified_access_instance_logging_configurations [R]
- Transit_gateway_route_tables [R]
- Traffic_mirror_session [CD]
- Network_insights_path [CD]
- Reserved_instances_exchange_quote [R]
- Launch_template_versions [RD]
- Route_server_propagations [R]
- Customer_gateways [R]
- Traffic_mirror_targets [R]
- Verified_access_instance [CD]
- Local_gateway_virtual_interface [CD]
- Reserved_instances_listings [R]
- Transit_gateway_policy_table_entries [R]
- Network_acl_entry [CD]
- Transit_gateway_vpc_attachment [CD]
- Vpc_classic_link [R]
- Ipam_pool [CD]
- Instance_topology [R]
- Byoip_cidrs [R]
- Capacity_reservations [R]
- Instance_credit_specifications [R]
- Transit_gateway_attachments [R]
- Declarative_policies_report_summary [R]
- Route_servers [R]
- Scheduled_instance_availability [R]
- Transit_gateway_connect_peers [R]
- Fleet [C]
- Local_gateway_route_table_virtual_interface_group_association [CD]
- Transit_gateway_route_table [CD]
- Vpc [CD]
- Ebs_encryption_by_default [R]
- Fleets [RD]
- Account_attributes [R]
- Id_format [R]
- Instance_tpm_ek_pub [R]
- Scheduled_instances [R]
- Ipam_prefix_list_resolver_versions [R]
- Transit_gateway_connect_peer [CD]
- Customer_gateway [CD]
- Instance_image_metadata [R]
- Reserved_instances_modifications [R]
- Instance_event_windows [R]
- Spot_fleet_requests [R]
- Security_group_rule_descriptions_ingress [U]
- Vpcs [R]
- Instance_attribute [R]
- Internet_gateway [CD]
- Instance_types_from_instance_requirements [R]
- Capacity_block_extension_history [R]
- Transit_gateway_route_table_announcement [CD]
- Elastic_gpus [R]
- Ipam_external_resource_verification_token [CD]
- Public_ipv4_pool [CD]
- Ipv6_pools [R]
- Network_interface_attribute [R]
- Coip_pools [R]
- Route_server_routing_database [R]
- Vpc_endpoint_service_permissions [R]
- Carrier_gateways [R]
- Transit_gateway_policy_table_associations [R]
- Network_acl [CD]
- Vpn_gateway [CD]
- Local_gateway_route_tables [R]
- Instance_metadata_defaults [R]
- Capacity_reservation_topology [R]
- Addresses [R]
- Traffic_mirror_filter [CD]
- Spot_placement_scores [R]
- Transit_gateway_route_table_propagations [R]
- Console_output [R]
- Mac_modification_tasks [R]
- Route_tables [R]
- Ipam_pool_cidrs [R]
- Store_image_tasks [R]
- Ipam_resource_cidrs [R]
- Network_interface [CD]
- Security_group_references [R]
- Serial_console_access_status [R]
- Reserved_instances_listing [C]
- Fleet_instances [R]
- Verified_access_instances [R]
- Vpc_block_public_access_options [R]
- Local_gateway_virtual_interface_group [CD]
- Vpn_gateways [R]
- Transit_gateway [CD]
- Associated_ipv6_pool_cidrs [R]
- Groups_for_capacity_reservation [R]
- Route_server [CD]
- Ipam [CD]
- Subnet [CD]
- Volume_status [R]
- Client_vpn_endpoint [CD]
- Traffic_mirror_target [CD]
- Fleet_history [R]
- Ipams [R]
- Ipam_scopes [R]
- Security_groups_for_vpc [R]
- Verified_access_group_policy [R]
- Vpn_connection_device_types [R]
- Prefix_lists [R]
- Instance_types [R]
- Ipam_resource_discovery [CD]
- Replace_root_volume_tasks [R]
- Snapshot_tier_status [R]
- Images [R]
- Address_transfers [R]
- Capacity_block_offerings [R]
- Instance_event_window [CD]
- Declarative_policies_reports [R]
- Local_gateway_virtual_interface_groups [R]
- Managed_prefix_list_entries [R]
- Coip_pool [CD]
- Client_vpn_target_networks [R]
- Managed_prefix_list_associations [R]

📖 [Full ec2 documentation](services/ec2.md)

### 221. Cloudfront_keyvaluestore

**Resources**: 3

- Key [CRD]
- Key_value_store [R]
- Keys [U]

📖 [Full cloudfront_keyvaluestore documentation](services/cloudfront_keyvaluestore.md)

### 222. Elastic_beanstalk

**Resources**: 21

- Configuration_settings [R]
- Platform_version [CRD]
- Configuration_template [CUD]
- Configuration_options [R]
- Environment_resources [R]
- Application [CUD]
- Application_resource_lifecycle [U]
- Instances_health [R]
- Application_versions [R]
- Storage_location [C]
- Environment_managed_action_history [R]
- Tags_for_resource [U]
- Environment_health [R]
- Events [R]
- Applications [R]
- Application_version [CUD]
- Account_attributes [R]
- Environment_managed_actions [R]
- Environment_configuration [D]
- Environment [CU]
- Environments [R]

📖 [Full elastic_beanstalk documentation](services/elastic_beanstalk.md)

### 223. Bedrock_agent

**Resources**: 0


📖 [Full bedrock_agent documentation](services/bedrock_agent.md)

### 224. Cloudsearch

**Resources**: 14

- Domain [CD]
- Scaling_parameters [RU]
- Service_access_policies [RU]
- Analysis_schemes [R]
- Domain_endpoint_options [RU]
- Expressions [R]
- Suggester [D]
- Index_fields [R]
- Domains [R]
- Analysis_scheme [D]
- Index_field [D]
- Expression [D]
- Suggesters [R]
- Availability_options [RU]

📖 [Full cloudsearch documentation](services/cloudsearch.md)

### 225. Appconfigdata

**Resources**: 1

- Latest_configuration [R]

📖 [Full appconfigdata documentation](services/appconfigdata.md)

### 226. Dlm

**Resources**: 2

- Lifecycle_policy [CRUD]
- Lifecycle_policies [R]

📖 [Full dlm documentation](services/dlm.md)

### 227. Appsync

**Resources**: 16

- Domain_name [CRUD]
- Resolver [CRUD]
- Graphql_api_environment_variables [CR]
- Channel_namespace [CRUD]
- Schema_creation_status [R]
- Function [CRUD]
- Type [CRUD]
- Api_cache [CRUD]
- Api [CRUD]
- Data_source_introspection [R]
- Api_association [R]
- Graphql_api [CRUD]
- Api_key [CUD]
- Data_source [CRUD]
- Source_api_association [RU]
- Introspection_schema [R]

📖 [Full appsync documentation](services/appsync.md)

### 228. Ivs

**Resources**: 8

- Playback_restriction_policy [CRUD]
- Playback_key_pair [RD]
- Metadata [C]
- Recording_configuration [CRD]
- Stream_session [R]
- Stream_key [CRD]
- Channel [CRUD]
- Stream [R]

📖 [Full ivs documentation](services/ivs.md)

### 229. Rum

**Resources**: 1

- Rum_events [C]

📖 [Full rum documentation](services/rum.md)

### 230. Dataexchange

**Resources**: 7

- Revision [CRUD]
- Asset [RUD]
- Received_data_grant [R]
- Job [CR]
- Event_action [CRUD]
- Data_set [CRUD]
- Data_grant [CRD]

📖 [Full dataexchange documentation](services/dataexchange.md)

### 231. App_mesh

**Resources**: 0


📖 [Full app_mesh documentation](services/app_mesh.md)

### 232. Rekognition

**Resources**: 23

- Face_liveness_session [C]
- Stream_processor [CRUD]
- Person_tracking [R]
- Segment_detection [R]
- Dataset [CRD]
- User [CD]
- Face_search [R]
- Faces [D]
- Celebrity_info [R]
- Media_analysis_job [R]
- Label_detection [R]
- Face_liveness_session_results [R]
- Face_detection [R]
- Celebrity_recognition [R]
- Content_moderation [R]
- Project [CD]
- Projects [R]
- Collection [CRD]
- Text_detection [R]
- Project_policy [CD]
- Project_version [CD]
- Project_versions [R]
- Dataset_entries [U]

📖 [Full rekognition documentation](services/rekognition.md)

### 233. Ssm_quicksetup

**Resources**: 4

- Service_settings [RU]
- Configuration [R]
- Configuration_definition [U]
- Configuration_manager [CRUD]

📖 [Full ssm_quicksetup documentation](services/ssm_quicksetup.md)

### 234. Mturk

**Resources**: 14

- Additional_assignments_for_hit [C]
- Qualification_score [R]
- Hit_with_hit_type [C]
- Expiration_for_hit [U]
- Account_balance [R]
- Worker_block [CD]
- File_upload_url [R]
- Qualification_type [CRUD]
- Hit_type [C]
- Hit [CRD]
- Assignment [R]
- Hit_review_status [U]
- Hit_type_of_hit [U]
- Notification_settings [U]

📖 [Full mturk documentation](services/mturk.md)

### 235. Frauddetector

**Resources**: 40

- Events_by_event_type [D]
- Model_version_status [U]
- Label [CD]
- Labels [R]
- Event_prediction_metadata [R]
- List [CUD]
- Event_type [CD]
- Batch_import_jobs [R]
- Event_types [R]
- Event_label [U]
- Models [R]
- Model_versions [R]
- Model [CUD]
- Batch_prediction_job [CD]
- Outcome [CD]
- Rule_metadata [U]
- Batch_import_job [CD]
- Kms_encryption_key [CR]
- Detector_version [CRUD]
- Variable [CUD]
- Variables [R]
- Detectors [R]
- Rule [CD]
- Detector [CRD]
- External_model [CD]
- External_models [R]
- List_elements [R]
- Rules [R]
- Outcomes [R]
- Entity_type [CD]
- Event_prediction [R]
- Rule_version [U]
- Entity_types [R]
- Detector_version_status [U]
- Event [RD]
- Detector_version_metadata [U]
- Delete_events_by_event_type_status [R]
- Batch_prediction_jobs [R]
- Lists_metadata [R]
- Model_version [CRUD]

📖 [Full frauddetector documentation](services/frauddetector.md)

### 236. Sfn

**Resources**: 9

- State_machine_alias [CRUD]
- Execution [R]
- Execution_history [R]
- State_machine_for_execution [R]
- Activity_task [R]
- State_machine [CRUD]
- State_machine_version [D]
- Map_run [RU]
- Activity [CRD]

📖 [Full sfn documentation](services/sfn.md)

### 237. Timestream_query

**Resources**: 3

- Account_settings [RU]
- Scheduled_query [CRUD]
- Endpoints [R]

📖 [Full timestream_query documentation](services/timestream_query.md)

### 238. Dynamodb

**Resources**: 15

- Resource_policy [CRD]
- Global_table_settings [RU]
- Export [R]
- Item [CRUD]
- Limits [R]
- Contributor_insights [RU]
- Table_replica_auto_scaling [RU]
- Global_table [CRU]
- Endpoints [R]
- Import [R]
- Kinesis_streaming_destination [RU]
- Continuous_backups [RU]
- Backup [CRD]
- Table [CRUD]
- Time_to_live [RU]

📖 [Full dynamodb documentation](services/dynamodb.md)

### 239. Workspaces

**Resources**: 37

- Workspaces_pools [R]
- Workspace_bundle [CUD]
- Workspace_directories [R]
- Standby_workspaces [C]
- Updated_workspace_image [C]
- Account [R]
- Connection_aliases [R]
- Ip_group [CD]
- Workspace_image [CD]
- Connect_client_add_in [CUD]
- Connection_alias [CD]
- Applications [R]
- Connection_alias_permissions [R]
- Workspace_image_permission [U]
- Workspace_images [R]
- Connect_client_add_ins [R]
- Workspace_bundles [R]
- Workspaces [CR]
- Rules_of_ip_group [U]
- Connection_alias_permission [U]
- Application_associations [R]
- Workspace_associations [R]
- Workspaces_pool_sessions [R]
- Client_branding [RD]
- Image_associations [R]
- Workspaces_connection_status [R]
- Account_link_invitation [CD]
- Workspace_snapshots [R]
- Ip_groups [R]
- Workspaces_pool [CU]
- Account_modifications [R]
- Bundle_associations [R]
- Custom_workspace_image_import [R]
- Workspace_image_permissions [R]
- Tags [CRD]
- Client_properties [R]
- Account_link [R]

📖 [Full workspaces documentation](services/workspaces.md)

### 240. Artifact

**Resources**: 0


📖 [Full artifact documentation](services/artifact.md)

### 241. Ssm

**Resources**: 68

- Ops_items [R]
- Access_token [R]
- Document_metadata [U]
- Automation_execution [R]
- Maintenance_window_target [U]
- Ops_item [CRUD]
- Maintenance_window_execution_task [R]
- Document [CRUD]
- Instance_information [R]
- Maintenance_windows [R]
- Patch_baseline [CRUD]
- Execution_preview [R]
- Parameter [CRD]
- Maintenance_window_execution_task_invocation [R]
- Parameters [RD]
- Managed_instance_role [U]
- Resource_policy [CD]
- Patch_baselines [R]
- Ops_summary [R]
- Maintenance_windows_for_target [R]
- Instance_associations_status [R]
- Service_setting [RU]
- Compliance_items [C]
- Activation [CD]
- Association_batch [C]
- Resource_data_sync [CUD]
- Deployable_patch_snapshot_for_instance [R]
- Maintenance_window_execution [R]
- Maintenance_window_tasks [R]
- Effective_patches_for_patch_baseline [R]
- Effective_instance_associations [R]
- Association_executions [R]
- Maintenance_window_execution_tasks [R]
- Calendar_state [R]
- Automation_executions [R]
- Association [CRUD]
- Patch_groups [R]
- Association_execution_targets [R]
- Connection_status [R]
- Maintenance_window_task [RU]
- Activations [R]
- Automation_step_executions [R]
- Inventory_deletions [R]
- Maintenance_window_executions [R]
- Parameters_by_path [R]
- Maintenance_window_schedule [R]
- Patch_properties [R]
- Available_patches [R]
- Instance_patch_states_for_patch_group [R]
- Command_invocation [R]
- Maintenance_window [CRUD]
- Instance_patch_states [R]
- Sessions [R]
- Default_patch_baseline [R]
- Parameter_history [R]
- Resource_policies [R]
- Document_default_version [U]
- Instance_patches [R]
- Maintenance_window_execution_task_invocations [R]
- Maintenance_window_targets [R]
- Ops_metadata [CRUD]
- Instance_properties [R]
- Patch_group_state [R]
- Patch_baseline_for_patch_group [R]
- Association_status [U]
- Inventory [CRD]
- Inventory_schema [R]
- Document_permission [R]

📖 [Full ssm documentation](services/ssm.md)

### 242. Pipes

**Resources**: 0


📖 [Full pipes documentation](services/pipes.md)

### 243. Migrationhub_config

**Resources**: 3

- Home_region_controls [R]
- Home_region [R]
- Home_region_control [CD]

📖 [Full migrationhub_config documentation](services/migrationhub_config.md)

### 244. Redshift

**Resources**: 64

- Cluster_snapshots [R]
- Cluster_snapshot [CD]
- Cluster_db_revisions [R]
- Default_cluster_parameters [R]
- Scheduled_actions [R]
- Snapshot_schedules [R]
- Usage_limits [R]
- Cluster_credentials_with_iam [R]
- Clusters [R]
- Partner_status [U]
- Authentication_profile [CD]
- Reserved_node_offerings [R]
- Cluster_parameters [R]
- Partners [R]
- Cluster_parameter_group [CD]
- Reserved_node_exchange_offerings [R]
- Snapshot_copy_grants [R]
- Integrations [R]
- Hsm_client_certificate [CD]
- Redshift_idc_application [CD]
- Event_subscriptions [R]
- Cluster_tracks [R]
- Integration [CD]
- Reserved_node_exchange_status [R]
- Custom_domain_associations [R]
- Snapshot_copy_grant [CD]
- Endpoint_authorization [R]
- Cluster_versions [R]
- Orderable_cluster_options [R]
- Cluster_credentials [R]
- Cluster_parameter_groups [R]
- Tags [CRD]
- Cluster_security_group [CD]
- Data_shares_for_producer [R]
- Custom_domain_association [CD]
- Usage_limit [CD]
- Hsm_configurations [R]
- Endpoint_access [CRD]
- Data_shares [R]
- Hsm_client_certificates [R]
- Partner [D]
- Resize [R]
- Account_attributes [R]
- Scheduled_action [CD]
- Resource_policy [CRD]
- Reserved_nodes [R]
- Data_shares_for_consumer [R]
- Cluster_security_groups [R]
- Events [R]
- Hsm_configuration [CD]
- Event_subscription [CD]
- Inbound_integrations [R]
- Logging_status [R]
- Cluster [CD]
- Node_configuration_options [R]
- Event_categories [R]
- Storage [R]
- Snapshot_schedule [CD]
- Table_restore_status [R]
- Cluster_subnet_group [CD]
- Cluster_subnet_groups [R]
- Reserved_node_exchange_configuration_options [R]
- Authentication_profiles [R]
- Redshift_idc_applications [R]

📖 [Full redshift documentation](services/redshift.md)

### 245. Marketplace_metering

**Resources**: 0


📖 [Full marketplace_metering documentation](services/marketplace_metering.md)

### 246. Migration_hub

**Resources**: 4

- Migration_task [R]
- Resource_attributes [C]
- Application_state [R]
- Progress_update_stream [CD]

📖 [Full migration_hub documentation](services/migration_hub.md)

### 247. Codestar_connections

**Resources**: 8

- Repository_sync_status [R]
- Repository_link [CRUD]
- Sync_blocker [U]
- Sync_blocker_summary [R]
- Host [CRUD]
- Connection [CRD]
- Sync_configuration [CRUD]
- Resource_sync_status [R]

📖 [Full codestar_connections documentation](services/codestar_connections.md)

### 248. Data_pipeline

**Resources**: 4

- Objects [R]
- Pipeline_definition [CR]
- Pipelines [R]
- Pipeline [CD]

📖 [Full data_pipeline documentation](services/data_pipeline.md)

### 249. License_manager

**Resources**: 13

- License_conversion_task [R]
- License_version [C]
- License_usage [R]
- License [CRD]
- License_manager_report_generator [CRUD]
- License_conversion_task_for_resource [C]
- Grant [CRD]
- License_configuration [CRUD]
- Access_token [R]
- Service_settings [RU]
- Token [CD]
- License_specifications_for_resource [U]
- Grant_version [C]

📖 [Full license_manager documentation](services/license_manager.md)

### 250. Workmailmessageflow

**Resources**: 1

- Raw_message_content [CR]

📖 [Full workmailmessageflow documentation](services/workmailmessageflow.md)

### 251. Workspaces_web

**Resources**: 1

- Session [R]

📖 [Full workspaces_web documentation](services/workspaces_web.md)

### 252. Directory_service

**Resources**: 31

- Directory [CD]
- Certificate [R]
- Client_authentication_settings [R]
- Event_topics [R]
- Regions [R]
- Directory_data_access [R]
- Number_of_domain_controllers [U]
- Microsoft_ad [C]
- Shared_directories [R]
- Domain_controllers [R]
- Conditional_forwarder [CUD]
- Directory_setup [U]
- Update_directory [R]
- Computer [C]
- Directories [R]
- Alias [C]
- Ldaps_settings [R]
- Hybrid_ad [CU]
- Radius [U]
- Snapshot [CD]
- Snapshot_limits [R]
- Snapshots [R]
- Trust [CUD]
- Ad_assessment [RD]
- Settings [RU]
- Log_subscription [CD]
- Trusts [R]
- Directory_limits [R]
- Hybrid_ad_update [R]
- Conditional_forwarders [R]
- Ca_enrollment_policy [R]

📖 [Full directory_service documentation](services/directory_service.md)

### 253. Sagemaker_metrics

**Resources**: 0


📖 [Full sagemaker_metrics documentation](services/sagemaker_metrics.md)

### 254. Workspaces_thin_client

**Resources**: 3

- Environment [CRUD]
- Device [RUD]
- Software_set [RU]

📖 [Full workspaces_thin_client documentation](services/workspaces_thin_client.md)

### 255. Directory_service_data

**Resources**: 2

- Group [CRUD]
- User [CRUD]

📖 [Full directory_service_data documentation](services/directory_service_data.md)

### 256. Resource_groups

**Resources**: 6

- Tag_sync_task [R]
- Group [CRUD]
- Account_settings [RU]
- Group_query [RU]
- Tags [R]
- Group_configuration [CR]

📖 [Full resource_groups documentation](services/resource_groups.md)

### 257. Verifiedpermissions

**Resources**: 0


📖 [Full verifiedpermissions documentation](services/verifiedpermissions.md)

### 258. Sns

**Resources**: 12

- Sms_sandbox_phone_number [CD]
- Endpoint_attributes [R]
- Topic_attributes [R]
- Platform_endpoint [C]
- Subscription_attributes [R]
- Endpoint [D]
- Platform_application [CD]
- Sms_sandbox_account_status [R]
- Data_protection_policy [CR]
- Platform_application_attributes [R]
- Topic [CD]
- Sms_attributes [R]

📖 [Full sns documentation](services/sns.md)

### 259. Cognito_sync

**Resources**: 7

- Cognito_events [R]
- Identity_pool_usage [R]
- Records [U]
- Identity_pool_configuration [R]
- Bulk_publish_details [R]
- Dataset [RD]
- Identity_usage [R]

📖 [Full cognito_sync documentation](services/cognito_sync.md)

### 260. Apigatewayv2

**Resources**: 31

- Route [CRUD]
- Integration [CRUD]
- Integrations [R]
- Model_template [R]
- Model [CRUD]
- Vpc_link [CRUD]
- Models [R]
- Api_mapping [CRUD]
- Deployments [R]
- Tags [R]
- Domain_names [R]
- Domain_name [CRUD]
- Routes [R]
- Deployment [CRUD]
- Apis [R]
- Stage [CRUD]
- Stages [R]
- Api [CRUD]
- Access_log_settings [D]
- Route_settings [D]
- Authorizers [R]
- Integration_responses [R]
- Route_request_parameter [D]
- Route_responses [R]
- Route_response [CRUD]
- Routing_rule [CRD]
- Cors_configuration [D]
- Vpc_links [R]
- Integration_response [CRUD]
- Api_mappings [R]
- Authorizer [CRUD]

📖 [Full apigatewayv2 documentation](services/apigatewayv2.md)

### 261. Arc_zonal_shift

**Resources**: 0


📖 [Full arc_zonal_shift documentation](services/arc_zonal_shift.md)

### 262. Appstream

**Resources**: 34

- Application [CUD]
- User [CD]
- App_block_builders [R]
- Images [R]
- Updated_image [C]
- Fleets [R]
- User_stack_associations [R]
- App_block_builder_app_block_associations [R]
- Image_permissions [RUD]
- Theme_for_stack [CRUD]
- App_block_builder_streaming_url [C]
- App_license_usage [R]
- Entitlements [R]
- Stacks [R]
- Applications [R]
- Streaming_url [C]
- Application_fleet_associations [R]
- Sessions [R]
- App_block [CD]
- Usage_report_subscription [CD]
- App_blocks [R]
- Image_builders [R]
- Directory_configs [R]
- Fleet [CUD]
- Image_builder_streaming_url [C]
- Entitlement [CUD]
- Stack [CUD]
- Users [R]
- Image [D]
- Software_associations [R]
- Directory_config [CUD]
- Usage_report_subscriptions [R]
- App_block_builder [CUD]
- Image_builder [CD]

📖 [Full appstream documentation](services/appstream.md)

### 263. Migration_hub_refactor_spaces

**Resources**: 5

- Resource_policy [CRD]
- Route [CRUD]
- Environment [CRD]
- Application [CRD]
- Service [CRD]

📖 [Full migration_hub_refactor_spaces documentation](services/migration_hub_refactor_spaces.md)

### 264. Arc_region_switch

**Resources**: 4

- Plan_in_region [R]
- Plan_evaluation_status [R]
- Plan_execution [RU]
- Plan_execution_step [U]

📖 [Full arc_region_switch documentation](services/arc_region_switch.md)

### 265. Accessanalyzer

**Resources**: 8

- Finding_v2 [R]
- Generated_policy [R]
- Finding_recommendation [R]
- Analyzed_resource [R]
- Access_preview [CR]
- Finding [R]
- Findings [U]
- Findings_statistics [R]

📖 [Full accessanalyzer documentation](services/accessanalyzer.md)

### 266. Bedrock

**Resources**: 0


📖 [Full bedrock documentation](services/bedrock.md)

### 267. Kms

**Resources**: 12

- Custom_key_stores [R]
- Key [CR]
- Parameters_for_import [R]
- Primary_region [U]
- Custom_key_store [CUD]
- Imported_key_material [D]
- Grant [C]
- Alias [CUD]
- Key_rotation_status [R]
- Key_policy [CR]
- Public_key [R]
- Key_description [U]

📖 [Full kms documentation](services/kms.md)

### 268. Cost_explorer

**Resources**: 27

- Savings_plans_coverage [R]
- Cost_category_definition [CRUD]
- Savings_plans_utilization_details [R]
- Anomalies [R]
- Reservation_purchase_recommendation [R]
- Reservation_utilization [R]
- Commitment_purchase_analysis [R]
- Cost_categories [R]
- Anomaly_monitor [CUD]
- Cost_and_usage [R]
- Rightsizing_recommendation [R]
- Dimension_values [R]
- Approximate_usage_records [R]
- Reservation_coverage [R]
- Cost_allocation_tags_status [U]
- Anomaly_monitors [R]
- Usage_forecast [R]
- Cost_comparison_drivers [R]
- Cost_and_usage_comparisons [R]
- Savings_plans_utilization [R]
- Savings_plan_purchase_recommendation_details [R]
- Tags [R]
- Anomaly_subscription [CUD]
- Anomaly_subscriptions [R]
- Cost_forecast [R]
- Cost_and_usage_with_resources [R]
- Savings_plans_purchase_recommendation [R]

📖 [Full cost_explorer documentation](services/cost_explorer.md)

### 269. Kinesis_analytics

**Resources**: 16

- Application_maintenance_configuration [U]
- Application_presigned_url [C]
- Application_output [D]
- Application_version [R]
- Application_operation [R]
- Application_cloud_watch_logging_option [D]
- Application_reference_data_source [D]
- Application_input_processing_configuration [D]
- Application_snapshot [CRD]
- Application_vpc_configuration [D]
- Application [CRUD]
- Application_output [D]
- Application_reference_data_source [D]
- Application [CRUD]
- Application_cloud_watch_logging_option [D]
- Application_input_processing_configuration [D]

📖 [Full kinesis_analytics documentation](services/kinesis_analytics.md)

### 270. Acm_pca

**Resources**: 7

- Certificate_authority_csr [R]
- Certificate [R]
- Certificate_authority_audit_report [CR]
- Policy [CRD]
- Certificate_authority_certificate [R]
- Certificate_authority [CRUD]
- Permission [CD]

📖 [Full acm_pca documentation](services/acm_pca.md)

### 271. Sagemaker

**Resources**: 90

- Hub_content_reference [CUD]
- Model_package [CRUD]
- Partner_app [CRUD]
- App_image_config [CRUD]
- Pipeline_definition_for_execution [R]
- Code_repository [CRUD]
- Auto_ml_job_v2 [CR]
- Scaling_configuration_recommendation [R]
- Artifact [CRUD]
- Hub_content [RUD]
- Tags [D]
- Cluster_scheduler_config [CRUD]
- Compute_quota [CRUD]
- Model_card_export_job [CR]
- Data_quality_job_definition [CRD]
- Model_package_group [CRD]
- Trial [CRUD]
- Device_fleet_report [R]
- Monitoring_alert [U]
- Compilation_job [CRD]
- Sagemaker_servicecatalog_portfolio_status [R]
- Cluster_event [R]
- Reserved_capacity [R]
- Cluster_software [U]
- Edge_deployment_plan [CRD]
- Domain [CRUD]
- Notebook_instance_lifecycle_config [CRUD]
- Devices [U]
- Endpoint_weights_and_capacities [U]
- Optimization_job [CRD]
- Endpoint [CRUD]
- Model_bias_job_definition [CRD]
- Presigned_mlflow_tracking_server_url [C]
- Flow_definition [CRD]
- Training_plan [CR]
- Notebook_instance [CRUD]
- Image_version [CRUD]
- Search_suggestions [R]
- Processing_job [CRD]
- Cluster [CRUD]
- Feature_metadata [RU]
- Pipeline [CRUD]
- Project [CRUD]
- Hub_content_presigned_urls [C]
- Transform_job [CR]
- Auto_ml_job [CR]
- Feature_group [CRUD]
- Inference_experiment [CRUD]
- Model_card [CRUD]
- Device [R]
- Hyper_parameter_tuning_job [CRD]
- Lineage_group [R]
- Training_job [CRUD]
- Subscribed_workteam [R]
- Experiment [CRUD]
- Lineage_group_policy [R]
- Endpoint_config [CRD]
- Trial_component [CRUD]
- Partner_app_presigned_url [C]
- Presigned_domain_url [C]
- Cluster_node [R]
- Model_explainability_job_definition [CRD]
- Pipeline_version [U]
- Monitoring_schedule [CRUD]
- User_profile [CRUD]
- Presigned_notebook_instance_url [C]
- App [CRD]
- Studio_lifecycle_config [CRD]
- Edge_deployment_stage [CD]
- Association [D]
- Human_task_ui [CRD]
- Action [CRUD]
- Inference_component_runtime_config [U]
- Mlflow_tracking_server [CRUD]
- Image [CRUD]
- Inference_recommendations_job [CR]
- Labeling_job [CR]
- Context [CRUD]
- Workforce [CRUD]
- Workteam [CRUD]
- Model_package_group_policy [CRD]
- Pipeline_execution [RU]
- Hub [CRUD]
- Inference_component [CRUD]
- Space [CRUD]
- Model [CRD]
- Algorithm [CRD]
- Device_fleet [CRUD]
- Edge_packaging_job [CR]
- Model_quality_job_definition [CRD]

📖 [Full sagemaker documentation](services/sagemaker.md)

### 272. Cloudwatch_logs

**Resources**: 43

- Delivery_sources [R]
- Query_definitions [R]
- Log_object [R]
- Log_group [CD]
- Configuration_templates [R]
- Export_task [C]
- Subscription_filter [CD]
- Account_policies [R]
- Log_anomaly_detector [CRUD]
- Destinations [R]
- Account_policy [CD]
- Transformer [CRD]
- Log_streams [R]
- Data_protection_policy [CRD]
- Destination_policy [C]
- Log_groups [R]
- Delivery_configuration [U]
- Delivery_destination [CRD]
- Subscription_filters [R]
- Delivery_destinations [R]
- Index_policies [R]
- Delivery [CRD]
- Integration [CRD]
- Log_record [R]
- Metric_filter [CD]
- Index_policy [CD]
- Delivery_source [CRD]
- Retention_policy [CD]
- Metric_filters [R]
- Resource_policy [CD]
- Log_group_fields [R]
- Deliveries [R]
- Log_stream [CD]
- Destination [CD]
- Query_definition [CD]
- Queries [R]
- Log_events [CR]
- Query_results [R]
- Anomaly [U]
- Delivery_destination_policy [CRD]
- Export_tasks [R]
- Resource_policies [R]
- Field_indexes [R]

📖 [Full cloudwatch_logs documentation](services/cloudwatch_logs.md)

### 273. Swf

**Resources**: 5

- Workflow_execution [R]
- Workflow_execution_history [R]
- Workflow_type [RD]
- Domain [R]
- Activity_type [RD]

📖 [Full swf documentation](services/swf.md)

### 274. Emr_serverless

**Resources**: 0


📖 [Full emr_serverless documentation](services/emr_serverless.md)

### 275. Bedrock_data_automation

**Resources**: 1

- Blueprint_version [C]

📖 [Full bedrock_data_automation documentation](services/bedrock_data_automation.md)

### 276. Mediaconvert

**Resources**: 8

- Endpoints [R]
- Jobs_query_results [R]
- Resource_share [C]
- Policy [CRD]
- Job_template [CRUD]
- Job [CR]
- Queue [CRUD]
- Preset [CRUD]

📖 [Full mediaconvert documentation](services/mediaconvert.md)

### 277. Cost_optimization_hub

**Resources**: 3

- Recommendation [R]
- Enrollment_status [U]
- Preferences [RU]

📖 [Full cost_optimization_hub documentation](services/cost_optimization_hub.md)

### 278. Connectcampaigns

**Resources**: 10

- Instance_onboarding_job_status [R]
- Dial_request_batch [C]
- Campaign_state [R]
- Instance_onboarding_job [D]
- Campaign [CRD]
- Campaign_state_batch [R]
- Campaign_name [U]
- Campaign_dialer_config [U]
- Campaign_outbound_call_config [U]
- Connect_instance_config [RD]

📖 [Full connectcampaigns documentation](services/connectcampaigns.md)

### 279. Datasync

**Resources**: 15

- Location_hdfs [CRU]
- Location_fsx_lustre [CRU]
- Location [D]
- Location_efs [CRU]
- Task_execution [RU]
- Location_s3 [CRU]
- Location_nfs [CRU]
- Location_azure_blob [CRU]
- Location_fsx_open_zfs [CRU]
- Task [CRUD]
- Agent [CRUD]
- Location_object_storage [CRU]
- Location_fsx_ontap [CRU]
- Location_fsx_windows [CRU]
- Location_smb [CRU]

📖 [Full datasync documentation](services/datasync.md)

### 280. Chime_sdk_media_pipelines

**Resources**: 11

- Media_insights_pipeline_status [U]
- Media_capture_pipeline [CRD]
- Media_insights_pipeline_configuration [CRUD]
- Media_live_connector_pipeline [C]
- Media_insights_pipeline [C]
- Media_stream_pipeline [C]
- Media_pipeline_kinesis_video_stream_pool [CRUD]
- Media_pipeline [RD]
- Speaker_search_task [R]
- Media_concatenation_pipeline [C]
- Voice_tone_analysis_task [R]

📖 [Full chime_sdk_media_pipelines documentation](services/chime_sdk_media_pipelines.md)

### 281. Application_signals

**Resources**: 2

- Grouping_configuration [CD]
- Service [R]

📖 [Full application_signals documentation](services/application_signals.md)

### 282. Sagemaker_geospatial

**Resources**: 0


📖 [Full sagemaker_geospatial documentation](services/sagemaker_geospatial.md)

### 283. Securitylake

**Resources**: 1

- Data_lake_exception_subscription [CRUD]

📖 [Full securitylake documentation](services/securitylake.md)

### 284. Inspector_scan

**Resources**: 0


📖 [Full inspector_scan documentation](services/inspector_scan.md)

### 285. Rtbfabric

**Resources**: 0


📖 [Full rtbfabric documentation](services/rtbfabric.md)

### 286. Cost_and_usage_report_service

**Resources**: 2

- Report_definitions [R]
- Report_definition [CD]

📖 [Full cost_and_usage_report_service documentation](services/cost_and_usage_report_service.md)

### 287. Gameliftstreams

**Resources**: 2

- Stream_session_connection [C]
- Stream_session [R]

📖 [Full gameliftstreams documentation](services/gameliftstreams.md)

### 288. Sso_oidc

**Resources**: 2

- Token [C]
- Token_with_iam [C]

📖 [Full sso_oidc documentation](services/sso_oidc.md)

### 289. Payment_cryptography_data

**Resources**: 0


📖 [Full payment_cryptography_data documentation](services/payment_cryptography_data.md)

### 290. Billing

**Resources**: 2

- Resource_policy [R]
- Billing_view [CRUD]

📖 [Full billing documentation](services/billing.md)

### 291. Iam

**Resources**: 37

- Role_permissions_boundary [CD]
- Service_specific_credential [CUD]
- Server_certificate [RUD]
- Role_description [U]
- Policy [CRD]
- Instance_profile [CRD]
- Account_summary [R]
- Mfa_device [R]
- Open_id_connect_provider [CRD]
- Account_alias [CD]
- Role [CRUD]
- User [CRUD]
- Signing_certificate [UD]
- Organizations_access_report [R]
- User_permissions_boundary [CD]
- Login_profile [CRUD]
- Group_policy [CRD]
- Context_keys_for_custom_policy [R]
- Open_id_connect_provider_thumbprint [U]
- User_policy [CRD]
- Account_password_policy [RUD]
- Credential_report [R]
- Service_last_accessed_details_with_entities [R]
- Service_linked_role_deletion_status [R]
- Policy_version [CRD]
- Virtual_mfa_device [CD]
- Assume_role_policy [U]
- Saml_provider [CRUD]
- Context_keys_for_principal_policy [R]
- Service_last_accessed_details [R]
- Role_policy [CRD]
- Ssh_public_key [RUD]
- Account_authorization_details [R]
- Service_linked_role [CD]
- Group [CRUD]
- Access_key [CUD]
- Access_key_last_used [R]

📖 [Full iam documentation](services/iam.md)

### 292. Chime_sdk

**Resources**: 25

- Voice_connector_origination [CRD]
- Speaker_search_task [R]
- Voice_connector_emergency_calling_configuration [CRD]
- Voice_connector_termination_health [R]
- Voice_connector [CRUD]
- Phone_number_order [CR]
- Voice_connector_logging_configuration [CR]
- Sip_media_application_call [CU]
- Voice_connector_termination_credentials [CD]
- Phone_number_settings [RU]
- Voice_connector_proxy [CRD]
- Voice_connector_group [CRUD]
- Sip_media_application [CRUD]
- Sip_rule [CRUD]
- Voice_connector_streaming_configuration [CRD]
- Voice_profile [CRUD]
- Proxy_session [CRUD]
- Voice_profile_domain [CRUD]
- Voice_connector_external_systems_configuration [CRD]
- Voice_connector_termination [CRD]
- Sip_media_application_logging_configuration [CR]
- Phone_number [RUD]
- Global_settings [RU]
- Voice_tone_analysis_task [R]
- Sip_media_application_alexa_skill_configuration [CR]

📖 [Full chime_sdk documentation](services/chime_sdk.md)

### 293. Sagemaker_edge

**Resources**: 2

- Deployments [R]
- Device_registration [R]

📖 [Full sagemaker_edge documentation](services/sagemaker_edge.md)

### 294. Iot_events

**Resources**: 6

- Detector_model_analysis_results [R]
- Alarm_model [CRUD]
- Detector_model [CRUD]
- Logging_options [CR]
- Input [CRUD]
- Detector_model_analysis [R]

📖 [Full iot_events documentation](services/iot_events.md)

### 295. Connect

**Resources**: 81

- User_hierarchy [U]
- User [CRD]
- User_security_profiles [U]
- Instance_attribute [RU]
- Contact_attributes [RU]
- Current_metric_data [R]
- Metric_data [R]
- Security_profile [CRUD]
- Use_case [CD]
- Contact_metrics [R]
- Attached_file [RD]
- User_identity_info [U]
- User_hierarchy_group_name [U]
- User_phone_config [U]
- View_content [U]
- Contact_routing_data [U]
- Flow_association [R]
- Email_address_metadata [U]
- Email_address [CRD]
- Queue [CRD]
- Evaluation_form [CRUD]
- Queue_hours_of_operation [U]
- Queue_max_contacts [U]
- Queue_status [U]
- Hours_of_operation_override [CRUD]
- Contact_flow_module_metadata [U]
- Routing_profile_name [U]
- View_metadata [U]
- Agent_status [CRU]
- Integration_association [CD]
- Quick_connect [CRD]
- User_hierarchy_group [CRD]
- Authentication_profile [RU]
- Contact_flow [CRD]
- Prompt [CRUD]
- Contact_evaluation [RUD]
- Traffic_distribution [RU]
- User_hierarchy_structure [RU]
- Contact_flow_module_content [U]
- Queue_outbound_email_config [U]
- Quick_connect_name [U]
- Routing_profile_default_outbound_queue [U]
- Federation_token [R]
- Quick_connect_config [U]
- View [CRD]
- Traffic_distribution_group [CRD]
- Instance [CRD]
- Contact_flow_content [U]
- Participant_role_config [U]
- Metric_data_v2 [R]
- Phone_number_metadata [U]
- User_proficiencies [U]
- Contact_flow_version [CD]
- Rule [CRUD]
- View_version [CD]
- Contact_flow_module [CRD]
- Contact_flow_metadata [U]
- Routing_profile_concurrency [U]
- Push_notification_registration [CD]
- Current_user_data [R]
- Routing_profile_queues [U]
- User_routing_profile [U]
- Task_template [CRUD]
- Phone_number [RU]
- Hours_of_operation [CRUD]
- User_status [C]
- Contact [CRU]
- Persistent_contact_association [C]
- Routing_profile_agent_availability_timer [U]
- Predefined_attribute [CRUD]
- Participant_authentication [U]
- Routing_profile [CRD]
- Instance_storage_config [RU]
- Participant [C]
- Contact_flow_name [U]
- Queue_outbound_caller_config [U]
- Effective_hours_of_operations [R]
- Contact_schedule [U]
- Queue_name [U]
- Prompt_file [R]
- Vocabulary [CRD]

📖 [Full connect documentation](services/connect.md)

### 296. Identitystore

**Resources**: 3

- User_id [R]
- Group_id [R]
- Group_membership_id [R]

📖 [Full identitystore documentation](services/identitystore.md)

### 297. Iot_data_plane

**Resources**: 3

- Retained_message [R]
- Thing_shadow [RUD]
- Connection [D]

📖 [Full iot_data_plane documentation](services/iot_data_plane.md)

### 298. Pca_connector_ad

**Resources**: 0


📖 [Full pca_connector_ad documentation](services/pca_connector_ad.md)

### 299. Freetier

**Resources**: 3

- Free_tier_usage [R]
- Account_activity [R]
- Account_plan_state [R]

📖 [Full freetier documentation](services/freetier.md)

### 300. Rds_data

**Resources**: 0


📖 [Full rds_data documentation](services/rds_data.md)

### 301. Lex_runtime_service

**Resources**: 1

- Session [CRD]

📖 [Full lex_runtime_service documentation](services/lex_runtime_service.md)

### 302. Odb

**Resources**: 1

- Oci_onboarding_status [R]

📖 [Full odb documentation](services/odb.md)

### 303. Eks_auth

**Resources**: 0


📖 [Full eks_auth documentation](services/eks_auth.md)

### 304. Organizations

**Resources**: 9

- Resource_policy [CRD]
- Account [CR]
- Organizational_unit [CRUD]
- Gov_cloud_account [C]
- Effective_policy [R]
- Handshake [R]
- Policy [CRUD]
- Organization [CRD]
- Create_account_status [R]

📖 [Full organizations documentation](services/organizations.md)

### 305. Networkflowmonitor

**Resources**: 0


📖 [Full networkflowmonitor documentation](services/networkflowmonitor.md)

### 306. Glue

**Resources**: 91

- Job_run [R]
- Schema_version_metadata [C]
- Trigger [CRUD]
- Tags [R]
- Column_statistics_for_table [RUD]
- Integrations [R]
- Partition_index [CD]
- Integration_table_properties [CRUD]
- Job [CRUD]
- Entity [R]
- Column_statistics_for_partition [RUD]
- Usage_profile [CRUD]
- Plan [R]
- Unfiltered_partitions_metadata [R]
- Connection_type [R]
- Data_quality_model [R]
- Data_quality_rule_recommendation_run [R]
- User_defined_function [CRUD]
- Custom_entity_type [CRD]
- Ml_transform [CRUD]
- Column_statistics_task_runs [R]
- Schema_by_definition [R]
- Schema_versions_diff [R]
- Classifier [CRUD]
- Security_configurations [R]
- Statement [R]
- Blueprint_runs [R]
- Tables [R]
- Schema_version [R]
- Workflow_runs [R]
- Job_from_source_control [U]
- Source_control_from_job [U]
- Database [CRUD]
- Script [C]
- Data_quality_ruleset_evaluation_run [R]
- Dataflow_graph [R]
- Schema_versions [D]
- Workflow_run_properties [CR]
- Schema [CRUD]
- Connections [R]
- Data_quality_result [R]
- Catalog [CRUD]
- Registry [CRUD]
- Unfiltered_partition_metadata [R]
- Blueprint [CRUD]
- Classifiers [R]
- Crawlers [R]
- Connection [CRUD]
- Table_version [RD]
- Databases [R]
- Blueprint_run [R]
- Crawler [CRUD]
- Job_bookmark [R]
- Ml_transforms [R]
- Table_versions [R]
- User_defined_functions [R]
- Integration_resource_property [CRU]
- Data_quality_profile_annotation [C]
- Crawler_schedule [U]
- Unfiltered_table_metadata [R]
- Partitions [R]
- Column_statistics_task_settings [CRUD]
- Mapping [R]
- Resource_policies [R]
- Triggers [R]
- Workflow_run [R]
- Inbound_integrations [R]
- Data_quality_model_result [R]
- Jobs [R]
- Column_statistics_task_run [R]
- Session [CRD]
- Partition [CRUD]
- Dev_endpoints [R]
- Entity_records [R]
- Resource_policy [CRD]
- Glue_identity_center_configuration [CRUD]
- Workflow [CRUD]
- Integration [CD]
- Data_catalog_encryption_settings [CR]
- Ml_task_run [R]
- Job_runs [R]
- Data_quality_ruleset [CRUD]
- Catalogs [R]
- Security_configuration [CRD]
- Table [CRUD]
- Ml_task_runs [R]
- Crawler_metrics [R]
- Catalog_import_status [R]
- Table_optimizer [CRUD]
- Partition_indexes [R]
- Dev_endpoint [CRUD]

📖 [Full glue documentation](services/glue.md)

### 307. Elasticache

**Resources**: 30

- Serverless_cache_snapshot [CD]
- User [CD]
- Cache_clusters [R]
- Cache_security_group [CD]
- Serverless_cache [CD]
- Global_replication_group [CD]
- Cache_security_groups [R]
- Cache_subnet_groups [R]
- Cache_subnet_group [CD]
- Serverless_caches [R]
- Events [R]
- User_groups [R]
- Users [R]
- Reserved_cache_nodes [R]
- Reserved_cache_nodes_offerings [R]
- Global_replication_groups [R]
- Service_updates [R]
- User_group [CD]
- Snapshot [CD]
- Update_actions [R]
- Replication_groups [R]
- Engine_default_parameters [R]
- Cache_parameters [R]
- Serverless_cache_snapshots [R]
- Cache_parameter_group [CD]
- Cache_cluster [CD]
- Replication_group [CD]
- Cache_parameter_groups [R]
- Snapshots [R]
- Cache_engine_versions [R]

📖 [Full elasticache documentation](services/elasticache.md)

### 308. Clouddirectory

**Resources**: 13

- Schema_as_json [R]
- Directory [CRD]
- Typed_link_facet [CUD]
- Object [CD]
- Facet [CRUD]
- Schema [CUD]
- Typed_link_facet_information [R]
- Applied_schema_version [R]
- Object_information [R]
- Index [C]
- Object_attributes [RU]
- Schema_from_json [C]
- Link_attributes [RU]

📖 [Full clouddirectory documentation](services/clouddirectory.md)

### 309. Healthlake

**Resources**: 3

- Fhir_datastore [CRD]
- Fhir_import_job [R]
- Fhir_export_job [R]

📖 [Full healthlake documentation](services/healthlake.md)

### 310. Codestar_notifications

**Resources**: 2

- Target [D]
- Notification_rule [CRUD]

📖 [Full codestar_notifications documentation](services/codestar_notifications.md)

### 311. Quicksight

**Resources**: 68

- Action_connector [CRUD]
- Brand [CRUD]
- Template_alias [CRUD]
- Key_registration [RU]
- Dashboard_snapshot_job_result [R]
- Role_custom_permission [RUD]
- Data_set_refresh_properties [CRD]
- Template_permissions [RU]
- Account_subscription [CRD]
- Asset_bundle_export_job [R]
- Iam_policy_assignment [CRUD]
- Account_custom_permission [RUD]
- Refresh_schedule [CRUD]
- Dashboard [CRUD]
- Account_customization [CRUD]
- Role_membership [CD]
- Theme_permissions [RU]
- Dashboard_permissions [RU]
- Dashboard_links [U]
- Flow_permissions [RU]
- Data_set_permissions [RU]
- Flow_metadata [R]
- Custom_permissions [CRUD]
- Q_personalization_configuration [RU]
- Vpc_connection [CRUD]
- Folder_membership [CD]
- Data_source_permissions [RU]
- Identity_propagation_config [UD]
- Action_connector_permissions [RU]
- Topic_refresh [R]
- Folder_permissions [RU]
- Dashboard_embed_url [R]
- Theme_alias [CRUD]
- Topic_refresh_schedule [CRUD]
- User_custom_permission [UD]
- Brand_published_version [RU]
- Folder [CRUD]
- Data_source [CRUD]
- Ip_restriction [RU]
- Topic [CRUD]
- Dashboard_published_version [U]
- Namespace [CRD]
- Theme [CRUD]
- Analysis_definition [R]
- Folder_resolved_permissions [R]
- Analysis [CRUD]
- Account_settings [RU]
- Template_definition [R]
- Topic_permissions [RU]
- Dashboard_definition [R]
- Template [CRUD]
- User_by_principal_id [D]
- User [RUD]
- Asset_bundle_import_job [R]
- Quick_sight_q_search_configuration [RU]
- Group [CRUD]
- Ingestion [CR]
- Analysis_permissions [RU]
- Data_set [CRUD]
- Dashboard_snapshot_job [R]
- Default_q_business_application [RUD]
- Brand_assignment [RUD]
- Group_membership [CRD]
- Application_with_token_exchange_grant [U]
- Session_embed_url [R]
- Dashboards_qa_configuration [RU]
- Public_sharing_settings [U]
- Spice_capacity_configuration [U]

📖 [Full quicksight documentation](services/quicksight.md)

### 312. Route53resolver

**Resources**: 17

- Resolver_query_log_config [CRD]
- Resolver_rule_association [R]
- Firewall_domain_list [CRD]
- Resolver_rule [CRUD]
- Resolver_dnssec_config [RU]
- Resolver_query_log_config_policy [CR]
- Firewall_rule_group [CRD]
- Firewall_rule_group_association [RU]
- Firewall_config [RU]
- Firewall_domains [U]
- Resolver_query_log_config_association [R]
- Firewall_rule [CUD]
- Outpost_resolver [CRUD]
- Resolver_config [RU]
- Resolver_endpoint [CRUD]
- Firewall_rule_group_policy [CR]
- Resolver_rule_policy [CR]

📖 [Full route53resolver documentation](services/route53resolver.md)

### 313. Synthetics

**Resources**: 6

- Canaries_last_run [R]
- Canaries [R]
- Group [CRD]
- Canary [CRUD]
- Runtime_versions [R]
- Canary_runs [R]

📖 [Full synthetics documentation](services/synthetics.md)

### 314. Transcribe

**Resources**: 9

- Vocabulary_filter [CRUD]
- Medical_scribe_job [RD]
- Call_analytics_job [RD]
- Language_model [CRD]
- Vocabulary [CRUD]
- Transcription_job [RD]
- Medical_vocabulary [CRUD]
- Call_analytics_category [CRUD]
- Medical_transcription_job [RD]

📖 [Full transcribe documentation](services/transcribe.md)

### 315. Chime

**Resources**: 14

- Events_configuration [CRD]
- Room [CRUD]
- User [CRU]
- Meeting_dial_out [C]
- Phone_number_settings [RU]
- Global_settings [RU]
- Account [CRUD]
- Retention_settings [CR]
- Phone_number [RUD]
- Bot [CRU]
- User_settings [RU]
- Account_settings [RU]
- Phone_number_order [CR]
- Room_membership [CUD]

📖 [Full chime documentation](services/chime.md)

### 316. Fms

**Resources**: 11

- Protocols_list [CRD]
- Notification_channel [CRD]
- Resource_set [CRD]
- Admin_scope [R]
- Violation_details [R]
- Apps_list [CRD]
- Third_party_firewall_association_status [R]
- Protection_status [R]
- Admin_account [CR]
- Policy [CRD]
- Compliance_detail [R]

📖 [Full fms documentation](services/fms.md)

### 317. Translate

**Resources**: 3

- Text_translation_job [R]
- Terminology [RD]
- Parallel_data [CRUD]

📖 [Full translate documentation](services/translate.md)

### 318. Amplifybackend

**Resources**: 8

- Backend_api [CRUD]
- Backend_auth [CRUD]
- Backend_config [CU]
- Token [CRD]
- Backend_job [RU]
- Backend_storage [CRUD]
- Backend_api_models [R]
- Backend [CRD]

📖 [Full amplifybackend documentation](services/amplifybackend.md)

### 319. Inspector

**Resources**: 15

- Assessment_runs [R]
- Assessment_targets [R]
- Assessment_target [CUD]
- Exclusions_preview [CR]
- Assessment_templates [R]
- Cross_account_access_role [R]
- Resource_groups [R]
- Assessment_report [R]
- Exclusions [R]
- Assessment_run [D]
- Assessment_template [CD]
- Findings [R]
- Telemetry_metadata [R]
- Rules_packages [R]
- Resource_group [C]

📖 [Full inspector documentation](services/inspector.md)

### 320. Migrationhuborchestrator

**Resources**: 0


📖 [Full migrationhuborchestrator documentation](services/migrationhuborchestrator.md)

### 321. Backupsearch

**Resources**: 0


📖 [Full backupsearch documentation](services/backupsearch.md)

### 322. Ssm_guiconnect

**Resources**: 0


📖 [Full ssm_guiconnect documentation](services/ssm_guiconnect.md)

### 323. Serverlessapplicationrepository

**Resources**: 5

- Application_version [C]
- Cloud_formation_template [CR]
- Application_policy [CR]
- Cloud_formation_change_set [C]
- Application [CRUD]

📖 [Full serverlessapplicationrepository documentation](services/serverlessapplicationrepository.md)

### 324. Kinesis

**Resources**: 28

- Media_for_fragment_list [R]
- Dash_streaming_session_url [R]
- Hls_streaming_session_url [R]
- Images [R]
- Clip [R]
- Media [R]
- Image_generation_configuration [RU]
- Data_retention [U]
- Mapped_resource_configuration [R]
- Media_storage_configuration [RU]
- Data_endpoint [R]
- Notification_configuration [RU]
- Stream [CRUD]
- Signaling_channel_endpoint [R]
- Edge_configuration [RD]
- Signaling_channel [CRUD]
- Shard_iterator [R]
- Record [C]
- Limits [R]
- Stream_consumer [R]
- Stream [CRD]
- Max_record_size [U]
- Shard_count [U]
- Resource_policy [CRD]
- Stream_summary [R]
- Stream_mode [U]
- Records [CR]
- Ice_server_config [R]

📖 [Full kinesis documentation](services/kinesis.md)

### 325. Geo_places

**Resources**: 0


📖 [Full geo_places documentation](services/geo_places.md)

### 326. Cloudwatch

**Resources**: 18

- Metric_widget_image [R]
- Anomaly_detector [CD]
- Insight_rule [C]
- Alarms [RD]
- Metric_alarm [C]
- Alarms_for_metric [R]
- Metric_stream [CRD]
- Dashboards [D]
- Metric_statistics [R]
- Managed_insight_rules [C]
- Alarm_contributors [R]
- Insight_rule_report [R]
- Insight_rules [RD]
- Alarm_history [R]
- Dashboard [CR]
- Composite_alarm [C]
- Metric_data [CR]
- Anomaly_detectors [R]

📖 [Full cloudwatch documentation](services/cloudwatch.md)

### 327. Ivs_realtime

**Resources**: 9

- Participant_token [C]
- Composition [R]
- Storage_configuration [CRD]
- Participant [R]
- Public_key [RD]
- Stage_session [R]
- Ingest_configuration [CRUD]
- Stage [CRUD]
- Encoder_configuration [CRD]

📖 [Full ivs_realtime documentation](services/ivs_realtime.md)

### 328. Controltower

**Resources**: 0


📖 [Full controltower documentation](services/controltower.md)

### 329. Mediapackage

**Resources**: 6

- Channel [CRUD]
- Origin_endpoint [CRUD]
- Harvest_job [CR]
- Packaging_configuration [CRD]
- Packaging_group [CRUD]
- Asset [CRD]

📖 [Full mediapackage documentation](services/mediapackage.md)

### 330. Service_quotas

**Resources**: 9

- Service_quota [R]
- Service_quota_increase_request_into_template [C]
- Aws_default_service_quota [R]
- Requested_service_quota_change [R]
- Service_quota_increase_request_from_template [RD]
- Auto_management_configuration [R]
- Support_case [C]
- Auto_management [U]
- Association_for_service_quota_template [R]

📖 [Full service_quotas documentation](services/service_quotas.md)

### 331. Service_catalog

**Resources**: 20

- Product [CRUD]
- Copy_product_status [R]
- Constraint [CRUD]
- Service_action [CRUD]
- Provisioning_artifact [CRUD]
- Provisioning_parameters [R]
- Portfolio_shares [R]
- Provisioned_product_plan [CRD]
- Portfolio [CRUD]
- Portfolio_share_status [R]
- Provisioned_product_properties [U]
- Record [R]
- Aws_organizations_access_status [R]
- Portfolio_share [CUD]
- Provisioned_product_outputs [R]
- Service_action_execution_parameters [R]
- Product_view [R]
- Tag_option [CRUD]
- Provisioned_product [RU]
- Product_as_admin [R]

📖 [Full service_catalog documentation](services/service_catalog.md)

### 332. Mgn

**Resources**: 0


📖 [Full mgn documentation](services/mgn.md)

### 333. Fsx

**Resources**: 22

- Data_repository_associations [R]
- Data_repository_tasks [R]
- Snapshot [CUD]
- File_cache [CUD]
- Volume_from_backup [C]
- Backup [CD]
- File_system [CUD]
- File_system_aliases [R]
- S3_access_point_attachments [R]
- Storage_virtual_machines [R]
- File_caches [R]
- Shared_vpc_configuration [RU]
- Volumes [R]
- File_systems [R]
- Backups [R]
- Data_repository_association [CUD]
- Snapshots [R]
- And_attach_s3_access_point [C]
- Storage_virtual_machine [CUD]
- Volume [CUD]
- File_system_from_backup [C]
- Data_repository_task [C]

📖 [Full fsx documentation](services/fsx.md)

### 334. Lex_models

**Resources**: 23

- Bot_recommendation [RU]
- Bot_replica [CRD]
- Resource_policy [CRUD]
- Utterances [D]
- Test_set [RUD]
- Test_set_discrepancy_report [CR]
- Import [RD]
- Custom_vocabulary_metadata [R]
- Resource_policy_statement [CD]
- Test_set_generation [R]
- Bot_version [CRD]
- Test_execution_artifacts_url [R]
- Upload_url [C]
- Bot_alias [CRUD]
- Test_execution [R]
- Intent [CRUD]
- Bot_locale [CRUD]
- Export [CRUD]
- Bot_resource_generation [R]
- Slot_type [CRUD]
- Custom_vocabulary [D]
- Slot [CRUD]
- Bot [CRUD]

📖 [Full lex_models documentation](services/lex_models.md)

### 335. Ecr

**Resources**: 22

- Authorization_token [R]
- Lifecycle_policy [CRD]
- Repositories [R]
- Image [C]
- Download_url_for_layer [R]
- Replication_configuration [C]
- Repository_creation_template [CUD]
- Image_scanning_configuration [C]
- Account_setting [CR]
- Pull_through_cache_rule [CUD]
- Images [R]
- Registry_policy [CRD]
- Image_replication_status [R]
- Image_tag_mutability [C]
- Image_scan_findings [R]
- Repository [CD]
- Pull_through_cache_rules [R]
- Repository_creation_templates [R]
- Lifecycle_policy_preview [R]
- Registry_scanning_configuration [CR]
- Repository_policy [RD]
- Registry [R]

📖 [Full ecr documentation](services/ecr.md)

### 336. Cloud9

**Resources**: 6

- Environments [R]
- Environment [UD]
- Environment_memberships [R]
- Environment_ec2 [C]
- Environment_membership [CUD]
- Environment_status [R]

📖 [Full cloud9 documentation](services/cloud9.md)

### 337. Cleanrooms

**Resources**: 0


📖 [Full cleanrooms documentation](services/cleanrooms.md)

### 338. Lex_runtime

**Resources**: 1

- Session [CRD]

📖 [Full lex_runtime documentation](services/lex_runtime.md)

### 339. Grafana

**Resources**: 0


📖 [Full grafana documentation](services/grafana.md)

### 340. Route53profiles

**Resources**: 3

- Profile_association [R]
- Profile_resource_association [RU]
- Profile [CRD]

📖 [Full route53profiles documentation](services/route53profiles.md)

### 341. Resource_groups_tagging_api

**Resources**: 5

- Tag_keys [R]
- Compliance_summary [R]
- Resources [R]
- Tag_values [R]
- Report_creation [R]

📖 [Full resource_groups_tagging_api documentation](services/resource_groups_tagging_api.md)

### 342. Global_accelerator

**Resources**: 9

- Custom_routing_accelerator_attributes [RU]
- Cross_account_attachment [CRUD]
- Custom_routing_accelerator [CRUD]
- Accelerator [CRUD]
- Custom_routing_endpoint_group [CRD]
- Custom_routing_listener [CRUD]
- Accelerator_attributes [RU]
- Listener [CRUD]
- Endpoint_group [CRUD]

📖 [Full global_accelerator documentation](services/global_accelerator.md)

### 343. Finspace

**Resources**: 14

- Kx_dataview [CRUD]
- Kx_cluster_code_configuration [U]
- Kx_database [CRUD]
- Kx_environment_network [U]
- Kx_scaling_group [CRD]
- Kx_cluster_node [D]
- Kx_cluster [CRD]
- Kx_cluster_databases [U]
- Kx_user [CRUD]
- Environment [CRUD]
- Kx_changeset [CR]
- Kx_environment [CRUD]
- Kx_volume [CRUD]
- Kx_connection_string [R]

📖 [Full finspace documentation](services/finspace.md)

### 344. Bedrock_agentcore

**Resources**: 5

- Resource_oauth2_token [R]
- Resource_api_key [R]
- Workload_access_token_for_jwt [R]
- Workload_access_token_for_user_id [R]
- Workload_access_token [R]

📖 [Full bedrock_agentcore documentation](services/bedrock_agentcore.md)

### 345. Supplychain

**Resources**: 2

- Data_integration_event [R]
- Data_integration_flow_execution [R]

📖 [Full supplychain documentation](services/supplychain.md)

### 346. Cognito_identity

**Resources**: 9

- Identities [D]
- Id [R]
- Identity_pool_roles [R]
- Open_id_token [R]
- Identity [R]
- Identity_pool [CRUD]
- Open_id_token_for_developer_identity [R]
- Principal_tag_attribute_map [R]
- Credentials_for_identity [R]

📖 [Full cognito_identity documentation](services/cognito_identity.md)

### 347. Entityresolution

**Resources**: 10

- Matching_workflow [CRUD]
- Policy_statement [D]
- Id_namespace [CRUD]
- Policy [CR]
- Schema_mapping [CRUD]
- Provider_service [R]
- Match_id [R]
- Id_mapping_job [R]
- Matching_job [R]
- Id_mapping_workflow [CRUD]

📖 [Full entityresolution documentation](services/entityresolution.md)

### 348. Shield

**Resources**: 9

- Attack_statistics [R]
- Protection_group [CRUD]
- Attack [R]
- Drt_access [R]
- Subscription_state [R]
- Protection [CRD]
- Application_layer_automatic_response [U]
- Emergency_contact_settings [RU]
- Subscription [CRUD]

📖 [Full shield documentation](services/shield.md)

### 349. B2bi

**Resources**: 2

- Transformer_job [R]
- Starter_mapping_template [C]

📖 [Full b2bi documentation](services/b2bi.md)

### 350. Datazone

**Resources**: 26

- Group_profile [CRU]
- Time_series_data_point [R]
- Job_run [R]
- Subscription_grant [CRD]
- Subscription_request_details [R]
- Time_series_data_points [D]
- Environment [CRUD]
- Environment_profile [CRUD]
- Environment_action [CRUD]
- Subscription_target [CRUD]
- Account_pool [CRUD]
- Project_membership [CD]
- Environment_blueprint [CRUD]
- Project_profile [CRUD]
- Connection [CRUD]
- User_profile [CRU]
- Lineage_node [R]
- Subscription [R]
- Subscription_request [CUD]
- Subscription_grant_status [U]
- Iam_portal_login_url [R]
- Lineage_event [R]
- Environment_credentials [R]
- Project [CRUD]
- Listing_change_set [C]
- Asset_filter [CRUD]

📖 [Full datazone documentation](services/datazone.md)

### 351. Route_53_domains

**Resources**: 9

- Domain_contact [U]
- Domain [D]
- Tags_for_domain [UD]
- Contact_reachability_status [R]
- Domain_detail [R]
- Domain_suggestions [R]
- Operation_detail [R]
- Domain_nameservers [U]
- Domain_contact_privacy [U]

📖 [Full route_53_domains documentation](services/route_53_domains.md)

### 352. Acm

**Resources**: 3

- Account_configuration [CR]
- Certificate [RD]
- Certificate_options [U]

📖 [Full acm documentation](services/acm.md)

### 353. M2

**Resources**: 1

- Signed_bluinsights_url [R]

📖 [Full m2 documentation](services/m2.md)

### 354. Appfabric

**Resources**: 4

- App_authorization [CRUD]
- App_bundle [CRD]
- Ingestion_destination [CRUD]
- Ingestion [CRD]

📖 [Full appfabric documentation](services/appfabric.md)

### 355. Docdb_elastic

**Resources**: 3

- Cluster_snapshot [CRD]
- Cluster [CRUD]
- Pending_maintenance_action [R]

📖 [Full docdb_elastic documentation](services/docdb_elastic.md)

### 356. Geo_maps

**Resources**: 0


📖 [Full geo_maps documentation](services/geo_maps.md)

### 357. Apprunner

**Resources**: 8

- Vpc_connector [CRD]
- Vpc_ingress_connection [CRUD]
- Auto_scaling_configuration [CRD]
- Default_auto_scaling_configuration [U]
- Service [CRUD]
- Connection [CD]
- Custom_domains [R]
- Observability_configuration [CRD]

📖 [Full apprunner documentation](services/apprunner.md)

### 358. Lakeformation

**Resources**: 20

- Data_cells_filter [CRUD]
- Query_state [R]
- Table_objects [RU]
- Lf_tag_expression [CRUD]
- Work_unit_results [R]
- Lf_tag [CRUD]
- Lake_formation_opt_in [CD]
- Data_lake_settings [CR]
- Query_statistics [R]
- Transaction [R]
- Temporary_glue_partition_credentials [R]
- Resource [RU]
- Table_storage_optimizer [U]
- Work_units [R]
- Lake_formation_identity_center_configuration [CRUD]
- Temporary_glue_table_credentials [R]
- Resource_lf_tags [R]
- Effective_permissions_for_path [R]
- Objects_on_cancel [D]
- Data_lake_principal [R]

📖 [Full lakeformation documentation](services/lakeformation.md)

### 359. Snowball

**Resources**: 11

- Return_shipping_label [CR]
- Address [CR]
- Addresses [R]
- Job_manifest [R]
- Job [CRU]
- Cluster [CRU]
- Long_term_pricing [CU]
- Job_unlock_code [R]
- Snowball_usage [R]
- Software_updates [R]
- Job_shipment_state [U]

📖 [Full snowball documentation](services/snowball.md)

### 360. Notifications

**Resources**: 0


📖 [Full notifications documentation](services/notifications.md)

### 361. Mpa

**Resources**: 2

- Policy_version [R]
- Resource_policy [R]

📖 [Full mpa documentation](services/mpa.md)

### 362. Imagebuilder

**Resources**: 17

- Component [CRD]
- Image_recipe_policy [CR]
- Infrastructure_configuration [CRUD]
- Image_recipe [CRD]
- Image_policy [CR]
- Image [CRD]
- Lifecycle_policy [CRUD]
- Container_recipe_policy [CR]
- Distribution_configuration [CRUD]
- Container_recipe [CRD]
- Image_pipeline [CRUD]
- Component_policy [CR]
- Marketplace_resource [R]
- Workflow_execution [R]
- Lifecycle_execution [R]
- Workflow_step_execution [R]
- Workflow [CRD]

📖 [Full imagebuilder documentation](services/imagebuilder.md)

### 363. Mediaconnect

**Resources**: 0


📖 [Full mediaconnect documentation](services/mediaconnect.md)

### 364. Kafkaconnect

**Resources**: 4

- Connector [CRUD]
- Custom_plugin [CRD]
- Worker_configuration [CRD]
- Connector_operation [R]

📖 [Full kafkaconnect documentation](services/kafkaconnect.md)

### 365. Snow_device_management

**Resources**: 0


📖 [Full snow_device_management documentation](services/snow_device_management.md)

### 366. Cloudcontrol

**Resources**: 2

- Resource_request_status [R]
- Resource [CRUD]

📖 [Full cloudcontrol documentation](services/cloudcontrol.md)

### 367. License_manager_linux_subscriptions

**Resources**: 2

- Registered_subscription_provider [R]
- Service_settings [RU]

📖 [Full license_manager_linux_subscriptions documentation](services/license_manager_linux_subscriptions.md)

### 368. Marketplace_entitlement_service

**Resources**: 1

- Entitlements [R]

📖 [Full marketplace_entitlement_service documentation](services/marketplace_entitlement_service.md)

### 369. Chime_sdk_identity

**Resources**: 7

- App_instance_user [CRUD]
- App_instance [CRUD]
- App_instance_user_endpoint [RU]
- App_instance_retention_settings [CR]
- App_instance_user_expiration_settings [C]
- App_instance_admin [CRD]
- App_instance_bot [CRUD]

📖 [Full chime_sdk_identity documentation](services/chime_sdk_identity.md)

### 370. Location

**Resources**: 0


📖 [Full location documentation](services/location.md)

### 371. Device_farm

**Resources**: 19

- Job [R]
- Suite [R]
- Project [CRUD]
- Upload [CRUD]
- Test_grid_url [C]
- Remote_access_session [CRD]
- Instance_profile [CRUD]
- Vpce_configuration [CRUD]
- Account_settings [R]
- Run [RD]
- Device [R]
- Device_pool_compatibility [R]
- Offering_status [R]
- Test [R]
- Device_instance [RU]
- Network_profile [CRUD]
- Test_grid_session [R]
- Test_grid_project [CRUD]
- Device_pool [CRUD]

📖 [Full device_farm documentation](services/device_farm.md)

### 372. Cloudwatch_events

**Resources**: 12

- Permission [C]
- Targets [C]
- Api_destination [CRUD]
- Event_bus [CRD]
- Events [C]
- Partner_event_source [CRD]
- Connection [CRUD]
- Replay [R]
- Event_source [R]
- Rule [CRD]
- Partner_events [C]
- Archive [CRUD]

📖 [Full cloudwatch_events documentation](services/cloudwatch_events.md)

### 373. Braket

**Resources**: 0


📖 [Full braket documentation](services/braket.md)

### 374. Codeguru_security

**Resources**: 5

- Upload_url [C]
- Account_configuration [RU]
- Scan [CR]
- Findings [R]
- Metrics_summary [R]

📖 [Full codeguru_security documentation](services/codeguru_security.md)

### 375. Codeguruprofiler

**Resources**: 1

- Findings_report_account_summary [R]

📖 [Full codeguruprofiler documentation](services/codeguruprofiler.md)

### 376. Iot

**Resources**: 64

- Policy_version [CRD]
- Indexing_configuration [RU]
- Domain_configuration [CRUD]
- Cardinality [R]
- Registration_code [RD]
- Managed_job_template [R]
- Billing_group [CRUD]
- Custom_metric [CRUD]
- Buckets_aggregation [R]
- Behavior_model_training_summaries [R]
- V2_logging_options [R]
- Policy [CRD]
- Account_audit_configuration [RUD]
- Mitigation_action [CRUD]
- Thing [CRUD]
- Audit_finding [R]
- Detect_mitigation_actions_task [R]
- Job [CRUD]
- Job_execution [RD]
- Job_template [CRD]
- Job_document [R]
- Package [CRUD]
- Audit_suppression [CRUD]
- Ca_certificate [RUD]
- Effective_policies [R]
- Security_profile [CRUD]
- Certificate [RUD]
- Percentiles [R]
- Thing_connectivity_data [R]
- Certificate_provider [CRUD]
- Audit_mitigation_actions_task [R]
- Role_alias [CRUD]
- Endpoint [R]
- Dimension [CRUD]
- Stream [CRUD]
- Event_configurations [RU]
- Command [CRUD]
- Provisioning_claim [C]
- Topic_rule_destination [CRUD]
- Audit_task [R]
- Index [R]
- Topic_rule [CRD]
- Logging_options [R]
- Package_configuration [RU]
- Thing_groups_for_thing [U]
- Authorizer [CRUD]
- Ota_update [CRD]
- V2_logging_level [D]
- Provisioning_template [CRUD]
- Fleet_metric [CRUD]
- Keys_and_certificate [C]
- Dynamic_thing_group [CUD]
- Provisioning_template_version [CRD]
- Package_version [CRUD]
- Thing_group [CRUD]
- Thing_type [CRUD]
- Encryption_configuration [RU]
- Command_execution [RD]
- Verification_state_on_violation [C]
- Statistics [R]
- Thing_registration_task [R]
- Certificate_from_csr [C]
- Scheduled_audit [CRUD]
- Default_authorizer [R]

📖 [Full iot documentation](services/iot.md)

### 377. Ssm_incidents

**Resources**: 8

- Incident_record [RUD]
- Resource_policies [R]
- Resource_policy [CD]
- Related_items [U]
- Deletion_protection [U]
- Response_plan [CRUD]
- Replication_set [CRUD]
- Timeline_event [CRUD]

📖 [Full ssm_incidents documentation](services/ssm_incidents.md)

### 378. Rolesanywhere

**Resources**: 1

- Notification_settings [C]

📖 [Full rolesanywhere documentation](services/rolesanywhere.md)

### 379. Transfer

**Resources**: 5

- Security_policy [R]
- Access [CRUD]
- Ssh_public_key [D]
- Host_key [RUD]
- Execution [R]

📖 [Full transfer documentation](services/transfer.md)

### 380. Redshift_serverless

**Resources**: 4

- Track [R]
- Resource_policy [CRD]
- Credentials [R]
- Custom_domain_association [CRUD]

📖 [Full redshift_serverless documentation](services/redshift_serverless.md)

### 381. Wisdom

**Resources**: 0


📖 [Full wisdom documentation](services/wisdom.md)

### 382. Omics

**Resources**: 1

- S3_access_policy [CRD]

📖 [Full omics documentation](services/omics.md)

### 383. Sagemaker_featurestore_runtime

**Resources**: 1

- Record [CRD]

📖 [Full sagemaker_featurestore_runtime documentation](services/sagemaker_featurestore_runtime.md)

### 384. Codecatalyst

**Resources**: 1

- User_details [R]

📖 [Full codecatalyst documentation](services/codecatalyst.md)

### 385. Ecs

**Resources**: 23

- Service_deployments [R]
- Services [R]
- Cluster_settings [U]
- Task_definition [R]
- Container_instances_state [U]
- Capacity_provider [CUD]
- Service [CUD]
- Tasks [R]
- Cluster_capacity_providers [C]
- Container_agent [U]
- Task_set [CUD]
- Task_protection [RU]
- Service_revisions [R]
- Container_instances [R]
- Capacity_providers [R]
- Task_sets [R]
- Clusters [R]
- Account_setting [CD]
- Cluster [CUD]
- Account_setting_default [C]
- Attributes [CD]
- Service_primary_task_set [U]
- Task_definitions [D]

📖 [Full ecs documentation](services/ecs.md)

### 386. Storage_gateway

**Resources**: 41

- Snapshot [C]
- Bandwidth_rate_limit_schedule [RU]
- File_system_associations [R]
- Working_storage [R]
- Chap_credentials [RUD]
- Snapshot_from_volume_recovery_point [C]
- Upload_buffer [R]
- Storedi_scsi_volumes [R]
- Nfs_file_share [CU]
- Bandwidth_rate_limit [RUD]
- Storedi_scsi_volume [C]
- Tape_archive [D]
- Tape_archives [R]
- Gateway_software_now [U]
- Smb_local_groups [U]
- Volume [D]
- Cache_report [RD]
- Tapes [CR]
- Tape_pool [CD]
- Tape_recovery_points [R]
- Smb_file_shares [R]
- Cachedi_scsi_volumes [R]
- Availability_monitor_test [R]
- Smb_security_strategy [U]
- Gateway_information [RU]
- File_share [D]
- Nfs_file_shares [R]
- Automatic_tape_creation_policy [UD]
- Smb_settings [R]
- Tape [D]
- Vtl_devices [R]
- Cache [R]
- Smb_file_share_visibility [U]
- File_system_association [U]
- Gateway [D]
- Maintenance_start_time [RU]
- Cachedi_scsi_volume [C]
- Smb_file_share [CU]
- Tape_with_barcode [C]
- Snapshot_schedule [RUD]
- Vtl_device_type [U]

📖 [Full storage_gateway documentation](services/storage_gateway.md)

### 387. Mediapackagev2

**Resources**: 0


📖 [Full mediapackagev2 documentation](services/mediapackagev2.md)

### 388. Simspaceweaver

**Resources**: 0


📖 [Full simspaceweaver documentation](services/simspaceweaver.md)

### 389. Timestream_influxdb

**Resources**: 0


📖 [Full timestream_influxdb documentation](services/timestream_influxdb.md)

### 390. Sagemaker_runtime

**Resources**: 0


📖 [Full sagemaker_runtime documentation](services/sagemaker_runtime.md)

### 391. Application_insights

**Resources**: 9

- Problem [RU]
- Workload [RU]
- Component [CRUD]
- Log_pattern [CRUD]
- Observation [R]
- Component_configuration [RU]
- Problem_observations [R]
- Application [CRUD]
- Component_configuration_recommendation [R]

📖 [Full application_insights documentation](services/application_insights.md)

### 392. Lambda

**Resources**: 1

- Account_settings [R]

📖 [Full lambda documentation](services/lambda.md)

### 393. Ec2_instance_connect

**Resources**: 0


📖 [Full ec2_instance_connect documentation](services/ec2_instance_connect.md)

### 394. Security_ir

**Resources**: 0


📖 [Full security_ir documentation](services/security_ir.md)


---

## Example: Complete Workflow

Here's a complete example showing a typical workflow:

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create property_value_history
property_value_history = provider.iottwinmaker.Property_value_history {
}

# Use resource outputs
property_value_history_id = property_value_history.id
property_value_history_property_values = property_value_history.property_values
property_value_history_next_token = property_value_history.next_token

```

---

## Configuration

### Environment Variables

Configure AWS credentials using environment variables:

```bash
export AWS_ACCESS_KEY_ID=your_access_key
export AWS_SECRET_ACCESS_KEY=your_secret_key
export AWS_REGION=us-east-1  # Optional, defaults to us-east-1
```

### KCL Configuration

```kcl
# Configure provider in your KCL code
provider = aws.AwsProvider {
    region = "us-west-2"
    # Credentials will be read from environment:
    # AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
}
```

---

## Next Steps

- 📚 [Service Documentation](services/) - Detailed docs for each service
- 📖 [Installation Guide](installation.md) - Installation options
- ⬅️ [Back to README](../README.md)

---

## Need Help?

- 📖 Check service-specific documentation in `docs/services/`
- 🐛 [Report issues](https://github.com/YOUR_ORG/hemmer-provider-aws/issues)
- 💬 [Join discussions](https://github.com/YOUR_ORG/hemmer-provider-aws/discussions)
