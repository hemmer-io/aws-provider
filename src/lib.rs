//! Aws Provider for Hemmer
//!
//! Auto-generated unified provider from aws SDK version v1
//!
//! This provider includes multiple services:
//! - oam_2022_06_10
//! - forecast_2018_06_26
//! - bedrock_runtime_2023_09_30
//! - codebuild_2016_10_06
//! - iotdeviceadvisor_2020_09_18
//! - auto_scaling_2011_01_01
//! - database_migration_service_2016_01_01
//! - codeguruprofiler_2019_07_18
//! - kinesis_2013_12_02
//! - pinpoint_2016_12_01
//! - chime_2018_05_01
//! - iottwinmaker_2021_11_29
//! - organizations_2016_11_28
//! - networkflowmonitor_2023_04_19
//! - shield_2016_06_02
//! - ssm_2014_11_06
//! - arc_zonal_shift_2022_10_30
//! - workspaces_thin_client_2023_08_22
//! - partnercentral_selling_2022_07_26
//! - geo_places_2020_11_19
//! - signer_2017_08_25
//! - marketplace_reporting_2018_05_10
//! - lakeformation_2017_03_31
//! - pcs_2023_02_10
//! - kinesis
//! - mediaconnect_2018_11_14
//! - secrets_manager_2017_10_17
//! - mwaa_2020_07_01
//! - kms_2014_11_01
//! - quicksight_2018_04_01
//! - workmail_2017_10_01
//! - eventbridge_2015_10_07
//! - frauddetector_2019_11_15
//! - cloudwatch_events_2015_10_07
//! - cloudhsm
//! - cloudsearch_domain_2013_01_01
//! - lookoutequipment_2020_12_15
//! - iot_events_data_2018_10_23
//! - securitylake_2018_05_10
//! - cloudwatch_2010_08_01
//! - glue_2017_03_31
//! - application_auto_scaling_2016_02_06
//! - personalize_events_2018_03_22
//! - voice_id_2021_09_27
//! - s3_2006_03_01
//! - appintegrations_2020_07_29
//! - lex_runtime_service_2016_11_28
//! - ssm_contacts_2021_05_03
//! - chime_sdk_meetings_2021_07_15
//! - sesv2_2019_09_27
//! - bedrock_agentcore_control_2023_06_05
//! - emr_2009_03_31
//! - controltower_2018_05_10
//! - resource_explorer_2_2022_07_28
//! - s3_control_2018_08_20
//! - rtbfabric_2023_05_15
//! - personalize_2018_05_22
//! - marketplace_entitlement_service_2017_01_11
//! - directory_service_data_2023_05_31
//! - ssm_sap_2018_05_10
//! - outposts_2019_12_03
//! - workdocs_2016_05_01
//! - networkmanager_2019_07_05
//! - kinesis
//! - socialmessaging_2024_01_01
//! - bedrock_agentcore_2024_02_28
//! - omics_2022_11_28
//! - bcm_recommended_actions_2024_11_14
//! - mediapackage_2017_10_12
//! - sagemaker_featurestore_runtime_2020_07_01
//! - config_service_2014_11_12
//! - cloudwatch_logs_2014_03_28
//! - medialive_2017_10_14
//! - backup_gateway_2021_01_01
//! - connect_contact_lens_2020_08_21
//! - mediaconvert_2017_08_29
//! - sns_2010_03_31
//! - evs_2023_07_27
//! - datasync_2018_11_09
//! - greengrassv2_2020_11_30
//! - cleanroomsml_2023_09_06
//! - neptunedata_2023_08_01
//! - acm_pca_2017_08_22
//! - service_catalog_2015_12_10
//! - b2bi_2022_06_23
//! - iotanalytics_2017_11_27
//! - lex_model_building_service_2017_04_19
//! - inspector2_2020_06_08
//! - groundstation_2019_05_23
//! - ecr_public_2020_10_30
//! - fis_2020_12_01
//! - proton_2020_07_20
//! - api_gateway_2015_07_09
//! - cloudhsm_2014_05_30
//! - mpa_2022_07_26
//! - osis_2022_01_01
//! - memorydb_2021_01_01
//! - inspector_2016_02_16
//! - translate_2017_07_01
//! - mailmanager_2023_10_17
//! - neptune_graph_2023_11_29
//! - chatbot_2017_10_11
//! - fms_2018_01_01
//! - qapps_2023_11_27
//! - customer_profiles_2020_08_15
//! - geo_maps_2020_11_19
//! - route_53_domains_2014_05_15
//! - service_catalog_appregistry_2020_06_24
//! - qbusiness_2023_11_27
//! - synthetics_2017_10_11
//! - codecatalyst_2022_09_28
//! - keyspacesstreams_2024_09_09
//! - storage_gateway_2013_06_30
//! - elastic_transcoder_2012_09_25
//! - bcm_data_exports_2023_11_26
//! - iotsecuretunneling_2018_10_05
//! - cloudfront_2020_05_31
//! - bedrock_data_automation_2023_07_26
//! - location_2020_11_19
//! - wafv2_2019_07_29
//! - opensearch_2021_01_01
//! - iotthingsgraph_2018_09_06
//! - security_ir_2018_05_10
//! - repostspace_2022_05_13
//! - health_2016_08_04
//! - workmailmessageflow_2019_05_01
//! - mediastore_data_2017_09_01
//! - ec2_instance_connect_2018_04_02
//! - comprehendmedical_2018_10_30
//! - iotfleetwise_2021_06_17
//! - route53profiles_2018_05_10
//! - application_signals_2024_04_15
//! - resource_groups_tagging_api_2017_01_26
//! - accessanalyzer_2019_11_01
//! - glacier_2012_06_01
//! - lightsail_2016_11_28
//! - rum_2018_05_10
//! - direct_connect_2012_10_25
//! - elastic_beanstalk_2010_12_01
//! - imagebuilder_2019_12_02
//! - simspaceweaver_2022_10_28
//! - iot_jobs_data_plane_2017_09_29
//! - freetier_2023_09_07
//! - bedrock_agent_runtime_2023_07_26
//! - neptune_2014_10_31
//! - transfer_2018_11_05
//! - deadline_2023_10_12
//! - braket_2019_09_01
//! - verifiedpermissions_2021_12_01
//! - scheduler_2021_06_30
//! - waf_regional_2016_11_28
//! - marketplace_commerce_analytics_2015_07_01
//! - ecr_2015_09_21
//! - cost_and_usage_report_service_2017_01_06
//! - kinesis
//! - dynamodb_2012_08_10
//! - resiliencehub_2020_04_30
//! - macie2_2020_01_01
//! - entityresolution_2018_05_10
//! - s3outposts_2017_07_25
//! - grafana_2020_08_18
//! - kinesis_analytics_2015_08_14
//! - eks_auth_2023_11_26
//! - rbin_2021_06_15
//! - service_quotas_2019_06_24
//! - ecs_2014_11_13
//! - marketplace_metering_2016_01_14
//! - timestream_write_2018_11_01
//! - qconnect_2020_10_19
//! - cloudsearch_2013_01_01
//! - bcm_pricing_calculator_2024_06_19
//! - appfabric_2023_05_19
//! - route53resolver_2018_04_01
//! - marketplace_agreement_2020_03_01
//! - workspaces_2015_04_08
//! - gameliftstreams_2018_05_10
//! - taxsettings_2018_05_10
//! - pinpoint_sms
//! - fsx_2018_03_01
//! - codepipeline_2015_07_09
//! - schemas_2019_12_02
//! - emr_serverless_2021_07_13
//! - sqs_2012_11_05
//! - license_manager_user_subscriptions_2018_05_10
//! - route53_recovery_cluster_2019_12_02
//! - migrationhuborchestrator_2021_08_28
//! - iot_2015_05_28
//! - sso_oidc_2019_06_10
//! - codestar_notifications_2019_10_15
//! - ebs_2019_11_02
//! - aiops_2018_05_10
//! - amplify_2017_07_25
//! - cloudcontrol_2021_09_30
//! - wellarchitected_2020_03_31
//! - route_53_2013_04_01
//! - ssm_incidents_2018_05_10
//! - bedrock_2023_04_20
//! - ivs_realtime_2020_07_14
//! - migrationhub_config_2019_06_30
//! - redshift_2012_12_01
//! - inspector_scan_2023_08_08
//! - connectcases_2022_10_03
//! - kinesis
//! - pca_connector_scep_2018_05_10
//! - appflow_2020_08_23
//! - gamelift_2015_10_01
//! - cloudtrail_2013_11_01
//! - resource_groups_2017_11_27
//! - supplychain_2024_01_01
//! - timestream_influxdb_2023_01_27
//! - pipes_2015_10_07
//! - evidently_2021_02_01
//! - codeguru_security_2018_05_10
//! - cost_optimization_hub_2022_07_26
//! - amplifyuibuilder_2021_08_11
//! - route53_recovery_control_config_2020_11_02
//! - vpc_lattice_2022_11_30
//! - managedblockchain_query_2023_05_04
//! - redshift_data_2019_12_20
//! - mediatailor_2018_04_23
//! - mediapackagev2_2022_12_25
//! - pi_2018_02_27
//! - appconfig_2019_10_09
//! - networkmonitor_2023_08_01
//! - network_firewall_2020_11_12
//! - connectparticipant_2018_09_07
//! - mgn_2020_02_26
//! - sagemaker_edge_2020_09_23
//! - applicationcostprofiler_2020_09_10
//! - keyspaces_2022_02_10
//! - iam_2010_05_08
//! - data_pipeline_2012_10_29
//! - odb_2024_08_20
//! - chime_sdk_messaging_2021_05_15
//! - mediastore_2017_09_01
//! - iot_wireless_2020_11_22
//! - cloud9_2017_09_23
//! - wisdom_2020_10_19
//! - sagemaker_runtime_2017_05_13
//! - sso_2019_06_10
//! - auditmanager_2017_07_25
//! - snowball_2016_06_30
//! - migration_hub_2017_05_31
//! - identitystore_2020_06_15
//! - elastic_load_balancing
//! - connectcampaigns_2021_01_30
//! - textract_2018_06_27
//! - compute_optimizer_2019_11_01
//! - s3tables_2018_05_10
//! - eks_2017_11_01
//! - support_2013_04_15
//! - mturk_2017_01_17
//! - apigatewayv2_2018_11_29
//! - cognito_identity_provider_2016_04_18
//! - bedrock_data_automation_runtime_2024_06_13
//! - pinpoint_sms
//! - amp_2020_08_01
//! - drs_2020_02_26
//! - payment_cryptography_2021_09_14
//! - kafkaconnect_2021_09_14
//! - kafka_2018_11_14
//! - databrew_2017_07_25
//! - support_app_2021_08_20
//! - codedeploy_2014_10_06
//! - batch_2016_08_10
//! - savingsplans_2019_06_28
//! - bedrock_agent_2023_06_05
//! - directory_service_2015_04_16
//! - workspaces_instances_2022_07_26
//! - chime_sdk_media_pipelines_2021_07_15
//! - migrationhubstrategy_2020_02_19
//! - timestream_query_2018_11_01
//! - codeguru_reviewer_2019_09_19
//! - appsync_2017_07_25
//! - dlm_2018_01_12
//! - iot_data_plane_2015_05_28
//! - global_accelerator_2018_08_08
//! - amplifybackend_2020_08_11
//! - datazone_2018_05_10
//! - connectcampaignsv2_2024_04_23
//! - billingconductor_2021_07_30
//! - budgets_2016_10_20
//! - cloudtrail_data_2021_08_11
//! - geo_routes_2020_11_19
//! - m2_2021_04_28
//! - pinpoint_email_2018_07_26
//! - lex_models
//! - finspace_2021_03_12
//! - detective_2018_10_26
//! - lambda_2015_03_31
//! - kinesis_analytics
//! - panorama_2019_07_24
//! - iot_events_2018_07_27
//! - app_mesh_2019_01_25
//! - managedblockchain_2018_09_24
//! - waf_2015_08_24
//! - ivs_2020_07_14
//! - devops_guru_2020_12_01
//! - cost_explorer_2017_10_25
//! - mq_2017_11_27
//! - route53_recovery_readiness_2019_12_02
//! - internetmonitor_2021_06_03
//! - license_manager_2018_08_01
//! - codestar_connections_2019_12_01
//! - artifact_2018_05_10
//! - ssm_guiconnect_2021_05_01
//! - iotsitewise_2019_12_02
//! - serverlessapplicationrepository_2017_09_08
//! - ssm_quicksetup_2018_05_10
//! - docdb_elastic_2022_11_28
//! - sagemaker_metrics_2022_09_30
//! - license_manager_linux_subscriptions_2018_05_10
//! - sagemaker_geospatial_2020_05_27
//! - personalize_runtime_2018_05_22
//! - clouddirectory_2017_01_11
//! - chime_sdk
//! - notificationscontacts_2018_05_10
//! - backupsearch_2018_05_10
//! - dynamodb_streams_2012_08_10
//! - iot_managed_integrations_2025_03_03
//! - cognito_sync_2014_06_30
//! - codeartifact_2018_09_22
//! - arc_region_switch_2022_07_26
//! - elastic_load_balancing_2012_06_01
//! - guardduty_2017_11_28
//! - cleanrooms_2022_02_17
//! - trustedadvisor_2022_09_15
//! - sagemaker_a2i_runtime_2019_11_07
//! - dax_2017_04_19
//! - docdb_2014_10_31
//! - firehose_2015_08_04
//! - ivschat_2020_07_14
//! - ses_2010_12_01
//! - bcm_dashboards_2025_08_18
//! - application_insights_2018_11_25
//! - device_farm_2015_06_23
//! - account_2021_02_01
//! - launch_wizard_2018_05_10
//! - finspace_data_2020_07_13
//! - appconfigdata_2021_11_11
//! - controlcatalog_2018_05_10
//! - greengrass_2017_06_07
//! - kendra_ranking_2022_10_19
//! - snow_device_management_2021_08_04
//! - securityhub_2018_10_26
//! - s3vectors_2025_07_15
//! - workspaces_web_2020_07_08
//! - backup_2018_11_15
//! - opensearchserverless_2021_11_01
//! - cloudformation_2010_05_15
//! - kendra_2019_02_03
//! - connect_2017_08_08
//! - machine_learning_2014_12_12
//! - elasticache_2015_02_02
//! - sfn_2016_11_23
//! - sso_admin_2020_07_20
//! - auto_scaling_plans_2018_01_06
//! - comprehend_2017_11_27
//! - rds_data_2018_08_01
//! - chime_sdk_identity_2021_04_20
//! - rekognition_2016_06_27
//! - appstream_2016_12_01
//! - polly_2016_06_10
//! - invoicing_2024_12_01
//! - rds_2014_10_31
//! - pricing_2017_10_15
//! - swf_2012_01_25
//! - cloudfront_keyvaluestore_2022_07_26
//! - marketplace_deployment_2023_01_25
//! - medical_imaging_2023_07_19
//! - transcribe_2017_10_26
//! - observabilityadmin_2018_05_10
//! - notifications_2018_05_10
//! - codecommit_2015_04_13
//! - lex_runtime
//! - pca_connector_ad_2018_05_10
//! - forecastquery_2018_06_26
//! - healthlake_2017_07_01
//! - rolesanywhere_2018_05_10
//! - marketplace_catalog_2018_09_17
//! - billing_2023_09_07
//! - emr_containers_2020_10_01
//! - apigatewaymanagementapi_2018_11_29
//! - xray_2016_04_12
//! - transcribe_streaming_2017_10_26
//! - ram_2018_01_04
//! - codeconnections_2023_12_01
//! - efs_2015_02_01
//! - migration_hub_refactor_spaces_2021_10_26
//! - elasticsearch_service_2015_01_01
//! - cognito_identity_2014_06_30
//! - payment_cryptography_data_2022_02_03
//! - dataexchange_2017_07_25
//! - sts_2011_06_15
//! - sagemaker_2017_07_24
//! - kinesis
//! - acm_2015_12_08
//! - athena_2017_05_18
//! - dsql_2018_05_10
//! - mediapackage
//! - tnb_2008_10_21
//! - ec2_2016_11_15
//! - apprunner_2020_05_15
//! - redshift_serverless_2021_04_21


pub mod oam_2022_06_10;
pub mod forecast_2018_06_26;
pub mod bedrock_runtime_2023_09_30;
pub mod codebuild_2016_10_06;
pub mod iotdeviceadvisor_2020_09_18;
pub mod auto_scaling_2011_01_01;
pub mod database_migration_service_2016_01_01;
pub mod codeguruprofiler_2019_07_18;
pub mod kinesis_2013_12_02;
pub mod pinpoint_2016_12_01;
pub mod chime_2018_05_01;
pub mod iottwinmaker_2021_11_29;
pub mod organizations_2016_11_28;
pub mod networkflowmonitor_2023_04_19;
pub mod shield_2016_06_02;
pub mod ssm_2014_11_06;
pub mod arc_zonal_shift_2022_10_30;
pub mod workspaces_thin_client_2023_08_22;
pub mod partnercentral_selling_2022_07_26;
pub mod geo_places_2020_11_19;
pub mod signer_2017_08_25;
pub mod marketplace_reporting_2018_05_10;
pub mod lakeformation_2017_03_31;
pub mod pcs_2023_02_10;
pub mod kinesis;
pub mod mediaconnect_2018_11_14;
pub mod secrets_manager_2017_10_17;
pub mod mwaa_2020_07_01;
pub mod kms_2014_11_01;
pub mod quicksight_2018_04_01;
pub mod workmail_2017_10_01;
pub mod eventbridge_2015_10_07;
pub mod frauddetector_2019_11_15;
pub mod cloudwatch_events_2015_10_07;
pub mod cloudhsm;
pub mod cloudsearch_domain_2013_01_01;
pub mod lookoutequipment_2020_12_15;
pub mod iot_events_data_2018_10_23;
pub mod securitylake_2018_05_10;
pub mod cloudwatch_2010_08_01;
pub mod glue_2017_03_31;
pub mod application_auto_scaling_2016_02_06;
pub mod personalize_events_2018_03_22;
pub mod voice_id_2021_09_27;
pub mod s3_2006_03_01;
pub mod appintegrations_2020_07_29;
pub mod lex_runtime_service_2016_11_28;
pub mod ssm_contacts_2021_05_03;
pub mod chime_sdk_meetings_2021_07_15;
pub mod sesv2_2019_09_27;
pub mod bedrock_agentcore_control_2023_06_05;
pub mod emr_2009_03_31;
pub mod controltower_2018_05_10;
pub mod resource_explorer_2_2022_07_28;
pub mod s3_control_2018_08_20;
pub mod rtbfabric_2023_05_15;
pub mod personalize_2018_05_22;
pub mod marketplace_entitlement_service_2017_01_11;
pub mod directory_service_data_2023_05_31;
pub mod ssm_sap_2018_05_10;
pub mod outposts_2019_12_03;
pub mod workdocs_2016_05_01;
pub mod networkmanager_2019_07_05;
pub mod kinesis;
pub mod socialmessaging_2024_01_01;
pub mod bedrock_agentcore_2024_02_28;
pub mod omics_2022_11_28;
pub mod bcm_recommended_actions_2024_11_14;
pub mod mediapackage_2017_10_12;
pub mod sagemaker_featurestore_runtime_2020_07_01;
pub mod config_service_2014_11_12;
pub mod cloudwatch_logs_2014_03_28;
pub mod medialive_2017_10_14;
pub mod backup_gateway_2021_01_01;
pub mod connect_contact_lens_2020_08_21;
pub mod mediaconvert_2017_08_29;
pub mod sns_2010_03_31;
pub mod evs_2023_07_27;
pub mod datasync_2018_11_09;
pub mod greengrassv2_2020_11_30;
pub mod cleanroomsml_2023_09_06;
pub mod neptunedata_2023_08_01;
pub mod acm_pca_2017_08_22;
pub mod service_catalog_2015_12_10;
pub mod b2bi_2022_06_23;
pub mod iotanalytics_2017_11_27;
pub mod lex_model_building_service_2017_04_19;
pub mod inspector2_2020_06_08;
pub mod groundstation_2019_05_23;
pub mod ecr_public_2020_10_30;
pub mod fis_2020_12_01;
pub mod proton_2020_07_20;
pub mod api_gateway_2015_07_09;
pub mod cloudhsm_2014_05_30;
pub mod mpa_2022_07_26;
pub mod osis_2022_01_01;
pub mod memorydb_2021_01_01;
pub mod inspector_2016_02_16;
pub mod translate_2017_07_01;
pub mod mailmanager_2023_10_17;
pub mod neptune_graph_2023_11_29;
pub mod chatbot_2017_10_11;
pub mod fms_2018_01_01;
pub mod qapps_2023_11_27;
pub mod customer_profiles_2020_08_15;
pub mod geo_maps_2020_11_19;
pub mod route_53_domains_2014_05_15;
pub mod service_catalog_appregistry_2020_06_24;
pub mod qbusiness_2023_11_27;
pub mod synthetics_2017_10_11;
pub mod codecatalyst_2022_09_28;
pub mod keyspacesstreams_2024_09_09;
pub mod storage_gateway_2013_06_30;
pub mod elastic_transcoder_2012_09_25;
pub mod bcm_data_exports_2023_11_26;
pub mod iotsecuretunneling_2018_10_05;
pub mod cloudfront_2020_05_31;
pub mod bedrock_data_automation_2023_07_26;
pub mod location_2020_11_19;
pub mod wafv2_2019_07_29;
pub mod opensearch_2021_01_01;
pub mod iotthingsgraph_2018_09_06;
pub mod security_ir_2018_05_10;
pub mod repostspace_2022_05_13;
pub mod health_2016_08_04;
pub mod workmailmessageflow_2019_05_01;
pub mod mediastore_data_2017_09_01;
pub mod ec2_instance_connect_2018_04_02;
pub mod comprehendmedical_2018_10_30;
pub mod iotfleetwise_2021_06_17;
pub mod route53profiles_2018_05_10;
pub mod application_signals_2024_04_15;
pub mod resource_groups_tagging_api_2017_01_26;
pub mod accessanalyzer_2019_11_01;
pub mod glacier_2012_06_01;
pub mod lightsail_2016_11_28;
pub mod rum_2018_05_10;
pub mod direct_connect_2012_10_25;
pub mod elastic_beanstalk_2010_12_01;
pub mod imagebuilder_2019_12_02;
pub mod simspaceweaver_2022_10_28;
pub mod iot_jobs_data_plane_2017_09_29;
pub mod freetier_2023_09_07;
pub mod bedrock_agent_runtime_2023_07_26;
pub mod neptune_2014_10_31;
pub mod transfer_2018_11_05;
pub mod deadline_2023_10_12;
pub mod braket_2019_09_01;
pub mod verifiedpermissions_2021_12_01;
pub mod scheduler_2021_06_30;
pub mod waf_regional_2016_11_28;
pub mod marketplace_commerce_analytics_2015_07_01;
pub mod ecr_2015_09_21;
pub mod cost_and_usage_report_service_2017_01_06;
pub mod kinesis;
pub mod dynamodb_2012_08_10;
pub mod resiliencehub_2020_04_30;
pub mod macie2_2020_01_01;
pub mod entityresolution_2018_05_10;
pub mod s3outposts_2017_07_25;
pub mod grafana_2020_08_18;
pub mod kinesis_analytics_2015_08_14;
pub mod eks_auth_2023_11_26;
pub mod rbin_2021_06_15;
pub mod service_quotas_2019_06_24;
pub mod ecs_2014_11_13;
pub mod marketplace_metering_2016_01_14;
pub mod timestream_write_2018_11_01;
pub mod qconnect_2020_10_19;
pub mod cloudsearch_2013_01_01;
pub mod bcm_pricing_calculator_2024_06_19;
pub mod appfabric_2023_05_19;
pub mod route53resolver_2018_04_01;
pub mod marketplace_agreement_2020_03_01;
pub mod workspaces_2015_04_08;
pub mod gameliftstreams_2018_05_10;
pub mod taxsettings_2018_05_10;
pub mod pinpoint_sms;
pub mod fsx_2018_03_01;
pub mod codepipeline_2015_07_09;
pub mod schemas_2019_12_02;
pub mod emr_serverless_2021_07_13;
pub mod sqs_2012_11_05;
pub mod license_manager_user_subscriptions_2018_05_10;
pub mod route53_recovery_cluster_2019_12_02;
pub mod migrationhuborchestrator_2021_08_28;
pub mod iot_2015_05_28;
pub mod sso_oidc_2019_06_10;
pub mod codestar_notifications_2019_10_15;
pub mod ebs_2019_11_02;
pub mod aiops_2018_05_10;
pub mod amplify_2017_07_25;
pub mod cloudcontrol_2021_09_30;
pub mod wellarchitected_2020_03_31;
pub mod route_53_2013_04_01;
pub mod ssm_incidents_2018_05_10;
pub mod bedrock_2023_04_20;
pub mod ivs_realtime_2020_07_14;
pub mod migrationhub_config_2019_06_30;
pub mod redshift_2012_12_01;
pub mod inspector_scan_2023_08_08;
pub mod connectcases_2022_10_03;
pub mod kinesis;
pub mod pca_connector_scep_2018_05_10;
pub mod appflow_2020_08_23;
pub mod gamelift_2015_10_01;
pub mod cloudtrail_2013_11_01;
pub mod resource_groups_2017_11_27;
pub mod supplychain_2024_01_01;
pub mod timestream_influxdb_2023_01_27;
pub mod pipes_2015_10_07;
pub mod evidently_2021_02_01;
pub mod codeguru_security_2018_05_10;
pub mod cost_optimization_hub_2022_07_26;
pub mod amplifyuibuilder_2021_08_11;
pub mod route53_recovery_control_config_2020_11_02;
pub mod vpc_lattice_2022_11_30;
pub mod managedblockchain_query_2023_05_04;
pub mod redshift_data_2019_12_20;
pub mod mediatailor_2018_04_23;
pub mod mediapackagev2_2022_12_25;
pub mod pi_2018_02_27;
pub mod appconfig_2019_10_09;
pub mod networkmonitor_2023_08_01;
pub mod network_firewall_2020_11_12;
pub mod connectparticipant_2018_09_07;
pub mod mgn_2020_02_26;
pub mod sagemaker_edge_2020_09_23;
pub mod applicationcostprofiler_2020_09_10;
pub mod keyspaces_2022_02_10;
pub mod iam_2010_05_08;
pub mod data_pipeline_2012_10_29;
pub mod odb_2024_08_20;
pub mod chime_sdk_messaging_2021_05_15;
pub mod mediastore_2017_09_01;
pub mod iot_wireless_2020_11_22;
pub mod cloud9_2017_09_23;
pub mod wisdom_2020_10_19;
pub mod sagemaker_runtime_2017_05_13;
pub mod sso_2019_06_10;
pub mod auditmanager_2017_07_25;
pub mod snowball_2016_06_30;
pub mod migration_hub_2017_05_31;
pub mod identitystore_2020_06_15;
pub mod elastic_load_balancing;
pub mod connectcampaigns_2021_01_30;
pub mod textract_2018_06_27;
pub mod compute_optimizer_2019_11_01;
pub mod s3tables_2018_05_10;
pub mod eks_2017_11_01;
pub mod support_2013_04_15;
pub mod mturk_2017_01_17;
pub mod apigatewayv2_2018_11_29;
pub mod cognito_identity_provider_2016_04_18;
pub mod bedrock_data_automation_runtime_2024_06_13;
pub mod pinpoint_sms;
pub mod amp_2020_08_01;
pub mod drs_2020_02_26;
pub mod payment_cryptography_2021_09_14;
pub mod kafkaconnect_2021_09_14;
pub mod kafka_2018_11_14;
pub mod databrew_2017_07_25;
pub mod support_app_2021_08_20;
pub mod codedeploy_2014_10_06;
pub mod batch_2016_08_10;
pub mod savingsplans_2019_06_28;
pub mod bedrock_agent_2023_06_05;
pub mod directory_service_2015_04_16;
pub mod workspaces_instances_2022_07_26;
pub mod chime_sdk_media_pipelines_2021_07_15;
pub mod migrationhubstrategy_2020_02_19;
pub mod timestream_query_2018_11_01;
pub mod codeguru_reviewer_2019_09_19;
pub mod appsync_2017_07_25;
pub mod dlm_2018_01_12;
pub mod iot_data_plane_2015_05_28;
pub mod global_accelerator_2018_08_08;
pub mod amplifybackend_2020_08_11;
pub mod datazone_2018_05_10;
pub mod connectcampaignsv2_2024_04_23;
pub mod billingconductor_2021_07_30;
pub mod budgets_2016_10_20;
pub mod cloudtrail_data_2021_08_11;
pub mod geo_routes_2020_11_19;
pub mod m2_2021_04_28;
pub mod pinpoint_email_2018_07_26;
pub mod lex_models;
pub mod finspace_2021_03_12;
pub mod detective_2018_10_26;
pub mod lambda_2015_03_31;
pub mod kinesis_analytics;
pub mod panorama_2019_07_24;
pub mod iot_events_2018_07_27;
pub mod app_mesh_2019_01_25;
pub mod managedblockchain_2018_09_24;
pub mod waf_2015_08_24;
pub mod ivs_2020_07_14;
pub mod devops_guru_2020_12_01;
pub mod cost_explorer_2017_10_25;
pub mod mq_2017_11_27;
pub mod route53_recovery_readiness_2019_12_02;
pub mod internetmonitor_2021_06_03;
pub mod license_manager_2018_08_01;
pub mod codestar_connections_2019_12_01;
pub mod artifact_2018_05_10;
pub mod ssm_guiconnect_2021_05_01;
pub mod iotsitewise_2019_12_02;
pub mod serverlessapplicationrepository_2017_09_08;
pub mod ssm_quicksetup_2018_05_10;
pub mod docdb_elastic_2022_11_28;
pub mod sagemaker_metrics_2022_09_30;
pub mod license_manager_linux_subscriptions_2018_05_10;
pub mod sagemaker_geospatial_2020_05_27;
pub mod personalize_runtime_2018_05_22;
pub mod clouddirectory_2017_01_11;
pub mod chime_sdk;
pub mod notificationscontacts_2018_05_10;
pub mod backupsearch_2018_05_10;
pub mod dynamodb_streams_2012_08_10;
pub mod iot_managed_integrations_2025_03_03;
pub mod cognito_sync_2014_06_30;
pub mod codeartifact_2018_09_22;
pub mod arc_region_switch_2022_07_26;
pub mod elastic_load_balancing_2012_06_01;
pub mod guardduty_2017_11_28;
pub mod cleanrooms_2022_02_17;
pub mod trustedadvisor_2022_09_15;
pub mod sagemaker_a2i_runtime_2019_11_07;
pub mod dax_2017_04_19;
pub mod docdb_2014_10_31;
pub mod firehose_2015_08_04;
pub mod ivschat_2020_07_14;
pub mod ses_2010_12_01;
pub mod bcm_dashboards_2025_08_18;
pub mod application_insights_2018_11_25;
pub mod device_farm_2015_06_23;
pub mod account_2021_02_01;
pub mod launch_wizard_2018_05_10;
pub mod finspace_data_2020_07_13;
pub mod appconfigdata_2021_11_11;
pub mod controlcatalog_2018_05_10;
pub mod greengrass_2017_06_07;
pub mod kendra_ranking_2022_10_19;
pub mod snow_device_management_2021_08_04;
pub mod securityhub_2018_10_26;
pub mod s3vectors_2025_07_15;
pub mod workspaces_web_2020_07_08;
pub mod backup_2018_11_15;
pub mod opensearchserverless_2021_11_01;
pub mod cloudformation_2010_05_15;
pub mod kendra_2019_02_03;
pub mod connect_2017_08_08;
pub mod machine_learning_2014_12_12;
pub mod elasticache_2015_02_02;
pub mod sfn_2016_11_23;
pub mod sso_admin_2020_07_20;
pub mod auto_scaling_plans_2018_01_06;
pub mod comprehend_2017_11_27;
pub mod rds_data_2018_08_01;
pub mod chime_sdk_identity_2021_04_20;
pub mod rekognition_2016_06_27;
pub mod appstream_2016_12_01;
pub mod polly_2016_06_10;
pub mod invoicing_2024_12_01;
pub mod rds_2014_10_31;
pub mod pricing_2017_10_15;
pub mod swf_2012_01_25;
pub mod cloudfront_keyvaluestore_2022_07_26;
pub mod marketplace_deployment_2023_01_25;
pub mod medical_imaging_2023_07_19;
pub mod transcribe_2017_10_26;
pub mod observabilityadmin_2018_05_10;
pub mod notifications_2018_05_10;
pub mod codecommit_2015_04_13;
pub mod lex_runtime;
pub mod pca_connector_ad_2018_05_10;
pub mod forecastquery_2018_06_26;
pub mod healthlake_2017_07_01;
pub mod rolesanywhere_2018_05_10;
pub mod marketplace_catalog_2018_09_17;
pub mod billing_2023_09_07;
pub mod emr_containers_2020_10_01;
pub mod apigatewaymanagementapi_2018_11_29;
pub mod xray_2016_04_12;
pub mod transcribe_streaming_2017_10_26;
pub mod ram_2018_01_04;
pub mod codeconnections_2023_12_01;
pub mod efs_2015_02_01;
pub mod migration_hub_refactor_spaces_2021_10_26;
pub mod elasticsearch_service_2015_01_01;
pub mod cognito_identity_2014_06_30;
pub mod payment_cryptography_data_2022_02_03;
pub mod dataexchange_2017_07_25;
pub mod sts_2011_06_15;
pub mod sagemaker_2017_07_24;
pub mod kinesis;
pub mod acm_2015_12_08;
pub mod athena_2017_05_18;
pub mod dsql_2018_05_10;
pub mod mediapackage;
pub mod tnb_2008_10_21;
pub mod ec2_2016_11_15;
pub mod apprunner_2020_05_15;
pub mod redshift_serverless_2021_04_21;


use thiserror::Error;

/// Provider error types
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("SDK error: {0}")]
    SdkError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for provider operations
pub type Result<T> = std::result::Result<T, ProviderError>;

/// Unified provider client for Aws
pub struct AwsProvider {
    oam_2022_06_10_client: aws_sdk_oam_2022_06_10::Client,
    forecast_2018_06_26_client: aws_sdk_forecast_2018_06_26::Client,
    bedrock_runtime_2023_09_30_client: aws_sdk_bedrock_runtime_2023_09_30::Client,
    codebuild_2016_10_06_client: aws_sdk_codebuild_2016_10_06::Client,
    iotdeviceadvisor_2020_09_18_client: aws_sdk_iotdeviceadvisor_2020_09_18::Client,
    auto_scaling_2011_01_01_client: aws_sdk_auto_scaling_2011_01_01::Client,
    database_migration_service_2016_01_01_client: aws_sdk_database_migration_service_2016_01_01::Client,
    codeguruprofiler_2019_07_18_client: aws_sdk_codeguruprofiler_2019_07_18::Client,
    kinesis_2013_12_02_client: aws_sdk_kinesis_2013_12_02::Client,
    pinpoint_2016_12_01_client: aws_sdk_pinpoint_2016_12_01::Client,
    chime_2018_05_01_client: aws_sdk_chime_2018_05_01::Client,
    iottwinmaker_2021_11_29_client: aws_sdk_iottwinmaker_2021_11_29::Client,
    organizations_2016_11_28_client: aws_sdk_organizations_2016_11_28::Client,
    networkflowmonitor_2023_04_19_client: aws_sdk_networkflowmonitor_2023_04_19::Client,
    shield_2016_06_02_client: aws_sdk_shield_2016_06_02::Client,
    ssm_2014_11_06_client: aws_sdk_ssm_2014_11_06::Client,
    arc_zonal_shift_2022_10_30_client: aws_sdk_arc_zonal_shift_2022_10_30::Client,
    workspaces_thin_client_2023_08_22_client: aws_sdk_workspaces_thin_client_2023_08_22::Client,
    partnercentral_selling_2022_07_26_client: aws_sdk_partnercentral_selling_2022_07_26::Client,
    geo_places_2020_11_19_client: aws_sdk_geo_places_2020_11_19::Client,
    signer_2017_08_25_client: aws_sdk_signer_2017_08_25::Client,
    marketplace_reporting_2018_05_10_client: aws_sdk_marketplace_reporting_2018_05_10::Client,
    lakeformation_2017_03_31_client: aws_sdk_lakeformation_2017_03_31::Client,
    pcs_2023_02_10_client: aws_sdk_pcs_2023_02_10::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    mediaconnect_2018_11_14_client: aws_sdk_mediaconnect_2018_11_14::Client,
    secrets_manager_2017_10_17_client: aws_sdk_secrets_manager_2017_10_17::Client,
    mwaa_2020_07_01_client: aws_sdk_mwaa_2020_07_01::Client,
    kms_2014_11_01_client: aws_sdk_kms_2014_11_01::Client,
    quicksight_2018_04_01_client: aws_sdk_quicksight_2018_04_01::Client,
    workmail_2017_10_01_client: aws_sdk_workmail_2017_10_01::Client,
    eventbridge_2015_10_07_client: aws_sdk_eventbridge_2015_10_07::Client,
    frauddetector_2019_11_15_client: aws_sdk_frauddetector_2019_11_15::Client,
    cloudwatch_events_2015_10_07_client: aws_sdk_cloudwatch_events_2015_10_07::Client,
    cloudhsm_client: aws_sdk_cloudhsm::Client,
    cloudsearch_domain_2013_01_01_client: aws_sdk_cloudsearch_domain_2013_01_01::Client,
    lookoutequipment_2020_12_15_client: aws_sdk_lookoutequipment_2020_12_15::Client,
    iot_events_data_2018_10_23_client: aws_sdk_iot_events_data_2018_10_23::Client,
    securitylake_2018_05_10_client: aws_sdk_securitylake_2018_05_10::Client,
    cloudwatch_2010_08_01_client: aws_sdk_cloudwatch_2010_08_01::Client,
    glue_2017_03_31_client: aws_sdk_glue_2017_03_31::Client,
    application_auto_scaling_2016_02_06_client: aws_sdk_application_auto_scaling_2016_02_06::Client,
    personalize_events_2018_03_22_client: aws_sdk_personalize_events_2018_03_22::Client,
    voice_id_2021_09_27_client: aws_sdk_voice_id_2021_09_27::Client,
    s3_2006_03_01_client: aws_sdk_s3_2006_03_01::Client,
    appintegrations_2020_07_29_client: aws_sdk_appintegrations_2020_07_29::Client,
    lex_runtime_service_2016_11_28_client: aws_sdk_lex_runtime_service_2016_11_28::Client,
    ssm_contacts_2021_05_03_client: aws_sdk_ssm_contacts_2021_05_03::Client,
    chime_sdk_meetings_2021_07_15_client: aws_sdk_chime_sdk_meetings_2021_07_15::Client,
    sesv2_2019_09_27_client: aws_sdk_sesv2_2019_09_27::Client,
    bedrock_agentcore_control_2023_06_05_client: aws_sdk_bedrock_agentcore_control_2023_06_05::Client,
    emr_2009_03_31_client: aws_sdk_emr_2009_03_31::Client,
    controltower_2018_05_10_client: aws_sdk_controltower_2018_05_10::Client,
    resource_explorer_2_2022_07_28_client: aws_sdk_resource_explorer_2_2022_07_28::Client,
    s3_control_2018_08_20_client: aws_sdk_s3_control_2018_08_20::Client,
    rtbfabric_2023_05_15_client: aws_sdk_rtbfabric_2023_05_15::Client,
    personalize_2018_05_22_client: aws_sdk_personalize_2018_05_22::Client,
    marketplace_entitlement_service_2017_01_11_client: aws_sdk_marketplace_entitlement_service_2017_01_11::Client,
    directory_service_data_2023_05_31_client: aws_sdk_directory_service_data_2023_05_31::Client,
    ssm_sap_2018_05_10_client: aws_sdk_ssm_sap_2018_05_10::Client,
    outposts_2019_12_03_client: aws_sdk_outposts_2019_12_03::Client,
    workdocs_2016_05_01_client: aws_sdk_workdocs_2016_05_01::Client,
    networkmanager_2019_07_05_client: aws_sdk_networkmanager_2019_07_05::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    socialmessaging_2024_01_01_client: aws_sdk_socialmessaging_2024_01_01::Client,
    bedrock_agentcore_2024_02_28_client: aws_sdk_bedrock_agentcore_2024_02_28::Client,
    omics_2022_11_28_client: aws_sdk_omics_2022_11_28::Client,
    bcm_recommended_actions_2024_11_14_client: aws_sdk_bcm_recommended_actions_2024_11_14::Client,
    mediapackage_2017_10_12_client: aws_sdk_mediapackage_2017_10_12::Client,
    sagemaker_featurestore_runtime_2020_07_01_client: aws_sdk_sagemaker_featurestore_runtime_2020_07_01::Client,
    config_service_2014_11_12_client: aws_sdk_config_service_2014_11_12::Client,
    cloudwatch_logs_2014_03_28_client: aws_sdk_cloudwatch_logs_2014_03_28::Client,
    medialive_2017_10_14_client: aws_sdk_medialive_2017_10_14::Client,
    backup_gateway_2021_01_01_client: aws_sdk_backup_gateway_2021_01_01::Client,
    connect_contact_lens_2020_08_21_client: aws_sdk_connect_contact_lens_2020_08_21::Client,
    mediaconvert_2017_08_29_client: aws_sdk_mediaconvert_2017_08_29::Client,
    sns_2010_03_31_client: aws_sdk_sns_2010_03_31::Client,
    evs_2023_07_27_client: aws_sdk_evs_2023_07_27::Client,
    datasync_2018_11_09_client: aws_sdk_datasync_2018_11_09::Client,
    greengrassv2_2020_11_30_client: aws_sdk_greengrassv2_2020_11_30::Client,
    cleanroomsml_2023_09_06_client: aws_sdk_cleanroomsml_2023_09_06::Client,
    neptunedata_2023_08_01_client: aws_sdk_neptunedata_2023_08_01::Client,
    acm_pca_2017_08_22_client: aws_sdk_acm_pca_2017_08_22::Client,
    service_catalog_2015_12_10_client: aws_sdk_service_catalog_2015_12_10::Client,
    b2bi_2022_06_23_client: aws_sdk_b2bi_2022_06_23::Client,
    iotanalytics_2017_11_27_client: aws_sdk_iotanalytics_2017_11_27::Client,
    lex_model_building_service_2017_04_19_client: aws_sdk_lex_model_building_service_2017_04_19::Client,
    inspector2_2020_06_08_client: aws_sdk_inspector2_2020_06_08::Client,
    groundstation_2019_05_23_client: aws_sdk_groundstation_2019_05_23::Client,
    ecr_public_2020_10_30_client: aws_sdk_ecr_public_2020_10_30::Client,
    fis_2020_12_01_client: aws_sdk_fis_2020_12_01::Client,
    proton_2020_07_20_client: aws_sdk_proton_2020_07_20::Client,
    api_gateway_2015_07_09_client: aws_sdk_api_gateway_2015_07_09::Client,
    cloudhsm_2014_05_30_client: aws_sdk_cloudhsm_2014_05_30::Client,
    mpa_2022_07_26_client: aws_sdk_mpa_2022_07_26::Client,
    osis_2022_01_01_client: aws_sdk_osis_2022_01_01::Client,
    memorydb_2021_01_01_client: aws_sdk_memorydb_2021_01_01::Client,
    inspector_2016_02_16_client: aws_sdk_inspector_2016_02_16::Client,
    translate_2017_07_01_client: aws_sdk_translate_2017_07_01::Client,
    mailmanager_2023_10_17_client: aws_sdk_mailmanager_2023_10_17::Client,
    neptune_graph_2023_11_29_client: aws_sdk_neptune_graph_2023_11_29::Client,
    chatbot_2017_10_11_client: aws_sdk_chatbot_2017_10_11::Client,
    fms_2018_01_01_client: aws_sdk_fms_2018_01_01::Client,
    qapps_2023_11_27_client: aws_sdk_qapps_2023_11_27::Client,
    customer_profiles_2020_08_15_client: aws_sdk_customer_profiles_2020_08_15::Client,
    geo_maps_2020_11_19_client: aws_sdk_geo_maps_2020_11_19::Client,
    route_53_domains_2014_05_15_client: aws_sdk_route_53_domains_2014_05_15::Client,
    service_catalog_appregistry_2020_06_24_client: aws_sdk_service_catalog_appregistry_2020_06_24::Client,
    qbusiness_2023_11_27_client: aws_sdk_qbusiness_2023_11_27::Client,
    synthetics_2017_10_11_client: aws_sdk_synthetics_2017_10_11::Client,
    codecatalyst_2022_09_28_client: aws_sdk_codecatalyst_2022_09_28::Client,
    keyspacesstreams_2024_09_09_client: aws_sdk_keyspacesstreams_2024_09_09::Client,
    storage_gateway_2013_06_30_client: aws_sdk_storage_gateway_2013_06_30::Client,
    elastic_transcoder_2012_09_25_client: aws_sdk_elastic_transcoder_2012_09_25::Client,
    bcm_data_exports_2023_11_26_client: aws_sdk_bcm_data_exports_2023_11_26::Client,
    iotsecuretunneling_2018_10_05_client: aws_sdk_iotsecuretunneling_2018_10_05::Client,
    cloudfront_2020_05_31_client: aws_sdk_cloudfront_2020_05_31::Client,
    bedrock_data_automation_2023_07_26_client: aws_sdk_bedrock_data_automation_2023_07_26::Client,
    location_2020_11_19_client: aws_sdk_location_2020_11_19::Client,
    wafv2_2019_07_29_client: aws_sdk_wafv2_2019_07_29::Client,
    opensearch_2021_01_01_client: aws_sdk_opensearch_2021_01_01::Client,
    iotthingsgraph_2018_09_06_client: aws_sdk_iotthingsgraph_2018_09_06::Client,
    security_ir_2018_05_10_client: aws_sdk_security_ir_2018_05_10::Client,
    repostspace_2022_05_13_client: aws_sdk_repostspace_2022_05_13::Client,
    health_2016_08_04_client: aws_sdk_health_2016_08_04::Client,
    workmailmessageflow_2019_05_01_client: aws_sdk_workmailmessageflow_2019_05_01::Client,
    mediastore_data_2017_09_01_client: aws_sdk_mediastore_data_2017_09_01::Client,
    ec2_instance_connect_2018_04_02_client: aws_sdk_ec2_instance_connect_2018_04_02::Client,
    comprehendmedical_2018_10_30_client: aws_sdk_comprehendmedical_2018_10_30::Client,
    iotfleetwise_2021_06_17_client: aws_sdk_iotfleetwise_2021_06_17::Client,
    route53profiles_2018_05_10_client: aws_sdk_route53profiles_2018_05_10::Client,
    application_signals_2024_04_15_client: aws_sdk_application_signals_2024_04_15::Client,
    resource_groups_tagging_api_2017_01_26_client: aws_sdk_resource_groups_tagging_api_2017_01_26::Client,
    accessanalyzer_2019_11_01_client: aws_sdk_accessanalyzer_2019_11_01::Client,
    glacier_2012_06_01_client: aws_sdk_glacier_2012_06_01::Client,
    lightsail_2016_11_28_client: aws_sdk_lightsail_2016_11_28::Client,
    rum_2018_05_10_client: aws_sdk_rum_2018_05_10::Client,
    direct_connect_2012_10_25_client: aws_sdk_direct_connect_2012_10_25::Client,
    elastic_beanstalk_2010_12_01_client: aws_sdk_elastic_beanstalk_2010_12_01::Client,
    imagebuilder_2019_12_02_client: aws_sdk_imagebuilder_2019_12_02::Client,
    simspaceweaver_2022_10_28_client: aws_sdk_simspaceweaver_2022_10_28::Client,
    iot_jobs_data_plane_2017_09_29_client: aws_sdk_iot_jobs_data_plane_2017_09_29::Client,
    freetier_2023_09_07_client: aws_sdk_freetier_2023_09_07::Client,
    bedrock_agent_runtime_2023_07_26_client: aws_sdk_bedrock_agent_runtime_2023_07_26::Client,
    neptune_2014_10_31_client: aws_sdk_neptune_2014_10_31::Client,
    transfer_2018_11_05_client: aws_sdk_transfer_2018_11_05::Client,
    deadline_2023_10_12_client: aws_sdk_deadline_2023_10_12::Client,
    braket_2019_09_01_client: aws_sdk_braket_2019_09_01::Client,
    verifiedpermissions_2021_12_01_client: aws_sdk_verifiedpermissions_2021_12_01::Client,
    scheduler_2021_06_30_client: aws_sdk_scheduler_2021_06_30::Client,
    waf_regional_2016_11_28_client: aws_sdk_waf_regional_2016_11_28::Client,
    marketplace_commerce_analytics_2015_07_01_client: aws_sdk_marketplace_commerce_analytics_2015_07_01::Client,
    ecr_2015_09_21_client: aws_sdk_ecr_2015_09_21::Client,
    cost_and_usage_report_service_2017_01_06_client: aws_sdk_cost_and_usage_report_service_2017_01_06::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    dynamodb_2012_08_10_client: aws_sdk_dynamodb_2012_08_10::Client,
    resiliencehub_2020_04_30_client: aws_sdk_resiliencehub_2020_04_30::Client,
    macie2_2020_01_01_client: aws_sdk_macie2_2020_01_01::Client,
    entityresolution_2018_05_10_client: aws_sdk_entityresolution_2018_05_10::Client,
    s3outposts_2017_07_25_client: aws_sdk_s3outposts_2017_07_25::Client,
    grafana_2020_08_18_client: aws_sdk_grafana_2020_08_18::Client,
    kinesis_analytics_2015_08_14_client: aws_sdk_kinesis_analytics_2015_08_14::Client,
    eks_auth_2023_11_26_client: aws_sdk_eks_auth_2023_11_26::Client,
    rbin_2021_06_15_client: aws_sdk_rbin_2021_06_15::Client,
    service_quotas_2019_06_24_client: aws_sdk_service_quotas_2019_06_24::Client,
    ecs_2014_11_13_client: aws_sdk_ecs_2014_11_13::Client,
    marketplace_metering_2016_01_14_client: aws_sdk_marketplace_metering_2016_01_14::Client,
    timestream_write_2018_11_01_client: aws_sdk_timestream_write_2018_11_01::Client,
    qconnect_2020_10_19_client: aws_sdk_qconnect_2020_10_19::Client,
    cloudsearch_2013_01_01_client: aws_sdk_cloudsearch_2013_01_01::Client,
    bcm_pricing_calculator_2024_06_19_client: aws_sdk_bcm_pricing_calculator_2024_06_19::Client,
    appfabric_2023_05_19_client: aws_sdk_appfabric_2023_05_19::Client,
    route53resolver_2018_04_01_client: aws_sdk_route53resolver_2018_04_01::Client,
    marketplace_agreement_2020_03_01_client: aws_sdk_marketplace_agreement_2020_03_01::Client,
    workspaces_2015_04_08_client: aws_sdk_workspaces_2015_04_08::Client,
    gameliftstreams_2018_05_10_client: aws_sdk_gameliftstreams_2018_05_10::Client,
    taxsettings_2018_05_10_client: aws_sdk_taxsettings_2018_05_10::Client,
    pinpoint_sms_client: aws_sdk_pinpoint_sms::Client,
    fsx_2018_03_01_client: aws_sdk_fsx_2018_03_01::Client,
    codepipeline_2015_07_09_client: aws_sdk_codepipeline_2015_07_09::Client,
    schemas_2019_12_02_client: aws_sdk_schemas_2019_12_02::Client,
    emr_serverless_2021_07_13_client: aws_sdk_emr_serverless_2021_07_13::Client,
    sqs_2012_11_05_client: aws_sdk_sqs_2012_11_05::Client,
    license_manager_user_subscriptions_2018_05_10_client: aws_sdk_license_manager_user_subscriptions_2018_05_10::Client,
    route53_recovery_cluster_2019_12_02_client: aws_sdk_route53_recovery_cluster_2019_12_02::Client,
    migrationhuborchestrator_2021_08_28_client: aws_sdk_migrationhuborchestrator_2021_08_28::Client,
    iot_2015_05_28_client: aws_sdk_iot_2015_05_28::Client,
    sso_oidc_2019_06_10_client: aws_sdk_sso_oidc_2019_06_10::Client,
    codestar_notifications_2019_10_15_client: aws_sdk_codestar_notifications_2019_10_15::Client,
    ebs_2019_11_02_client: aws_sdk_ebs_2019_11_02::Client,
    aiops_2018_05_10_client: aws_sdk_aiops_2018_05_10::Client,
    amplify_2017_07_25_client: aws_sdk_amplify_2017_07_25::Client,
    cloudcontrol_2021_09_30_client: aws_sdk_cloudcontrol_2021_09_30::Client,
    wellarchitected_2020_03_31_client: aws_sdk_wellarchitected_2020_03_31::Client,
    route_53_2013_04_01_client: aws_sdk_route_53_2013_04_01::Client,
    ssm_incidents_2018_05_10_client: aws_sdk_ssm_incidents_2018_05_10::Client,
    bedrock_2023_04_20_client: aws_sdk_bedrock_2023_04_20::Client,
    ivs_realtime_2020_07_14_client: aws_sdk_ivs_realtime_2020_07_14::Client,
    migrationhub_config_2019_06_30_client: aws_sdk_migrationhub_config_2019_06_30::Client,
    redshift_2012_12_01_client: aws_sdk_redshift_2012_12_01::Client,
    inspector_scan_2023_08_08_client: aws_sdk_inspector_scan_2023_08_08::Client,
    connectcases_2022_10_03_client: aws_sdk_connectcases_2022_10_03::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    pca_connector_scep_2018_05_10_client: aws_sdk_pca_connector_scep_2018_05_10::Client,
    appflow_2020_08_23_client: aws_sdk_appflow_2020_08_23::Client,
    gamelift_2015_10_01_client: aws_sdk_gamelift_2015_10_01::Client,
    cloudtrail_2013_11_01_client: aws_sdk_cloudtrail_2013_11_01::Client,
    resource_groups_2017_11_27_client: aws_sdk_resource_groups_2017_11_27::Client,
    supplychain_2024_01_01_client: aws_sdk_supplychain_2024_01_01::Client,
    timestream_influxdb_2023_01_27_client: aws_sdk_timestream_influxdb_2023_01_27::Client,
    pipes_2015_10_07_client: aws_sdk_pipes_2015_10_07::Client,
    evidently_2021_02_01_client: aws_sdk_evidently_2021_02_01::Client,
    codeguru_security_2018_05_10_client: aws_sdk_codeguru_security_2018_05_10::Client,
    cost_optimization_hub_2022_07_26_client: aws_sdk_cost_optimization_hub_2022_07_26::Client,
    amplifyuibuilder_2021_08_11_client: aws_sdk_amplifyuibuilder_2021_08_11::Client,
    route53_recovery_control_config_2020_11_02_client: aws_sdk_route53_recovery_control_config_2020_11_02::Client,
    vpc_lattice_2022_11_30_client: aws_sdk_vpc_lattice_2022_11_30::Client,
    managedblockchain_query_2023_05_04_client: aws_sdk_managedblockchain_query_2023_05_04::Client,
    redshift_data_2019_12_20_client: aws_sdk_redshift_data_2019_12_20::Client,
    mediatailor_2018_04_23_client: aws_sdk_mediatailor_2018_04_23::Client,
    mediapackagev2_2022_12_25_client: aws_sdk_mediapackagev2_2022_12_25::Client,
    pi_2018_02_27_client: aws_sdk_pi_2018_02_27::Client,
    appconfig_2019_10_09_client: aws_sdk_appconfig_2019_10_09::Client,
    networkmonitor_2023_08_01_client: aws_sdk_networkmonitor_2023_08_01::Client,
    network_firewall_2020_11_12_client: aws_sdk_network_firewall_2020_11_12::Client,
    connectparticipant_2018_09_07_client: aws_sdk_connectparticipant_2018_09_07::Client,
    mgn_2020_02_26_client: aws_sdk_mgn_2020_02_26::Client,
    sagemaker_edge_2020_09_23_client: aws_sdk_sagemaker_edge_2020_09_23::Client,
    applicationcostprofiler_2020_09_10_client: aws_sdk_applicationcostprofiler_2020_09_10::Client,
    keyspaces_2022_02_10_client: aws_sdk_keyspaces_2022_02_10::Client,
    iam_2010_05_08_client: aws_sdk_iam_2010_05_08::Client,
    data_pipeline_2012_10_29_client: aws_sdk_data_pipeline_2012_10_29::Client,
    odb_2024_08_20_client: aws_sdk_odb_2024_08_20::Client,
    chime_sdk_messaging_2021_05_15_client: aws_sdk_chime_sdk_messaging_2021_05_15::Client,
    mediastore_2017_09_01_client: aws_sdk_mediastore_2017_09_01::Client,
    iot_wireless_2020_11_22_client: aws_sdk_iot_wireless_2020_11_22::Client,
    cloud9_2017_09_23_client: aws_sdk_cloud9_2017_09_23::Client,
    wisdom_2020_10_19_client: aws_sdk_wisdom_2020_10_19::Client,
    sagemaker_runtime_2017_05_13_client: aws_sdk_sagemaker_runtime_2017_05_13::Client,
    sso_2019_06_10_client: aws_sdk_sso_2019_06_10::Client,
    auditmanager_2017_07_25_client: aws_sdk_auditmanager_2017_07_25::Client,
    snowball_2016_06_30_client: aws_sdk_snowball_2016_06_30::Client,
    migration_hub_2017_05_31_client: aws_sdk_migration_hub_2017_05_31::Client,
    identitystore_2020_06_15_client: aws_sdk_identitystore_2020_06_15::Client,
    elastic_load_balancing_client: aws_sdk_elastic_load_balancing::Client,
    connectcampaigns_2021_01_30_client: aws_sdk_connectcampaigns_2021_01_30::Client,
    textract_2018_06_27_client: aws_sdk_textract_2018_06_27::Client,
    compute_optimizer_2019_11_01_client: aws_sdk_compute_optimizer_2019_11_01::Client,
    s3tables_2018_05_10_client: aws_sdk_s3tables_2018_05_10::Client,
    eks_2017_11_01_client: aws_sdk_eks_2017_11_01::Client,
    support_2013_04_15_client: aws_sdk_support_2013_04_15::Client,
    mturk_2017_01_17_client: aws_sdk_mturk_2017_01_17::Client,
    apigatewayv2_2018_11_29_client: aws_sdk_apigatewayv2_2018_11_29::Client,
    cognito_identity_provider_2016_04_18_client: aws_sdk_cognito_identity_provider_2016_04_18::Client,
    bedrock_data_automation_runtime_2024_06_13_client: aws_sdk_bedrock_data_automation_runtime_2024_06_13::Client,
    pinpoint_sms_client: aws_sdk_pinpoint_sms::Client,
    amp_2020_08_01_client: aws_sdk_amp_2020_08_01::Client,
    drs_2020_02_26_client: aws_sdk_drs_2020_02_26::Client,
    payment_cryptography_2021_09_14_client: aws_sdk_payment_cryptography_2021_09_14::Client,
    kafkaconnect_2021_09_14_client: aws_sdk_kafkaconnect_2021_09_14::Client,
    kafka_2018_11_14_client: aws_sdk_kafka_2018_11_14::Client,
    databrew_2017_07_25_client: aws_sdk_databrew_2017_07_25::Client,
    support_app_2021_08_20_client: aws_sdk_support_app_2021_08_20::Client,
    codedeploy_2014_10_06_client: aws_sdk_codedeploy_2014_10_06::Client,
    batch_2016_08_10_client: aws_sdk_batch_2016_08_10::Client,
    savingsplans_2019_06_28_client: aws_sdk_savingsplans_2019_06_28::Client,
    bedrock_agent_2023_06_05_client: aws_sdk_bedrock_agent_2023_06_05::Client,
    directory_service_2015_04_16_client: aws_sdk_directory_service_2015_04_16::Client,
    workspaces_instances_2022_07_26_client: aws_sdk_workspaces_instances_2022_07_26::Client,
    chime_sdk_media_pipelines_2021_07_15_client: aws_sdk_chime_sdk_media_pipelines_2021_07_15::Client,
    migrationhubstrategy_2020_02_19_client: aws_sdk_migrationhubstrategy_2020_02_19::Client,
    timestream_query_2018_11_01_client: aws_sdk_timestream_query_2018_11_01::Client,
    codeguru_reviewer_2019_09_19_client: aws_sdk_codeguru_reviewer_2019_09_19::Client,
    appsync_2017_07_25_client: aws_sdk_appsync_2017_07_25::Client,
    dlm_2018_01_12_client: aws_sdk_dlm_2018_01_12::Client,
    iot_data_plane_2015_05_28_client: aws_sdk_iot_data_plane_2015_05_28::Client,
    global_accelerator_2018_08_08_client: aws_sdk_global_accelerator_2018_08_08::Client,
    amplifybackend_2020_08_11_client: aws_sdk_amplifybackend_2020_08_11::Client,
    datazone_2018_05_10_client: aws_sdk_datazone_2018_05_10::Client,
    connectcampaignsv2_2024_04_23_client: aws_sdk_connectcampaignsv2_2024_04_23::Client,
    billingconductor_2021_07_30_client: aws_sdk_billingconductor_2021_07_30::Client,
    budgets_2016_10_20_client: aws_sdk_budgets_2016_10_20::Client,
    cloudtrail_data_2021_08_11_client: aws_sdk_cloudtrail_data_2021_08_11::Client,
    geo_routes_2020_11_19_client: aws_sdk_geo_routes_2020_11_19::Client,
    m2_2021_04_28_client: aws_sdk_m2_2021_04_28::Client,
    pinpoint_email_2018_07_26_client: aws_sdk_pinpoint_email_2018_07_26::Client,
    lex_models_client: aws_sdk_lex_models::Client,
    finspace_2021_03_12_client: aws_sdk_finspace_2021_03_12::Client,
    detective_2018_10_26_client: aws_sdk_detective_2018_10_26::Client,
    lambda_2015_03_31_client: aws_sdk_lambda_2015_03_31::Client,
    kinesis_analytics_client: aws_sdk_kinesis_analytics::Client,
    panorama_2019_07_24_client: aws_sdk_panorama_2019_07_24::Client,
    iot_events_2018_07_27_client: aws_sdk_iot_events_2018_07_27::Client,
    app_mesh_2019_01_25_client: aws_sdk_app_mesh_2019_01_25::Client,
    managedblockchain_2018_09_24_client: aws_sdk_managedblockchain_2018_09_24::Client,
    waf_2015_08_24_client: aws_sdk_waf_2015_08_24::Client,
    ivs_2020_07_14_client: aws_sdk_ivs_2020_07_14::Client,
    devops_guru_2020_12_01_client: aws_sdk_devops_guru_2020_12_01::Client,
    cost_explorer_2017_10_25_client: aws_sdk_cost_explorer_2017_10_25::Client,
    mq_2017_11_27_client: aws_sdk_mq_2017_11_27::Client,
    route53_recovery_readiness_2019_12_02_client: aws_sdk_route53_recovery_readiness_2019_12_02::Client,
    internetmonitor_2021_06_03_client: aws_sdk_internetmonitor_2021_06_03::Client,
    license_manager_2018_08_01_client: aws_sdk_license_manager_2018_08_01::Client,
    codestar_connections_2019_12_01_client: aws_sdk_codestar_connections_2019_12_01::Client,
    artifact_2018_05_10_client: aws_sdk_artifact_2018_05_10::Client,
    ssm_guiconnect_2021_05_01_client: aws_sdk_ssm_guiconnect_2021_05_01::Client,
    iotsitewise_2019_12_02_client: aws_sdk_iotsitewise_2019_12_02::Client,
    serverlessapplicationrepository_2017_09_08_client: aws_sdk_serverlessapplicationrepository_2017_09_08::Client,
    ssm_quicksetup_2018_05_10_client: aws_sdk_ssm_quicksetup_2018_05_10::Client,
    docdb_elastic_2022_11_28_client: aws_sdk_docdb_elastic_2022_11_28::Client,
    sagemaker_metrics_2022_09_30_client: aws_sdk_sagemaker_metrics_2022_09_30::Client,
    license_manager_linux_subscriptions_2018_05_10_client: aws_sdk_license_manager_linux_subscriptions_2018_05_10::Client,
    sagemaker_geospatial_2020_05_27_client: aws_sdk_sagemaker_geospatial_2020_05_27::Client,
    personalize_runtime_2018_05_22_client: aws_sdk_personalize_runtime_2018_05_22::Client,
    clouddirectory_2017_01_11_client: aws_sdk_clouddirectory_2017_01_11::Client,
    chime_sdk_client: aws_sdk_chime_sdk::Client,
    notificationscontacts_2018_05_10_client: aws_sdk_notificationscontacts_2018_05_10::Client,
    backupsearch_2018_05_10_client: aws_sdk_backupsearch_2018_05_10::Client,
    dynamodb_streams_2012_08_10_client: aws_sdk_dynamodb_streams_2012_08_10::Client,
    iot_managed_integrations_2025_03_03_client: aws_sdk_iot_managed_integrations_2025_03_03::Client,
    cognito_sync_2014_06_30_client: aws_sdk_cognito_sync_2014_06_30::Client,
    codeartifact_2018_09_22_client: aws_sdk_codeartifact_2018_09_22::Client,
    arc_region_switch_2022_07_26_client: aws_sdk_arc_region_switch_2022_07_26::Client,
    elastic_load_balancing_2012_06_01_client: aws_sdk_elastic_load_balancing_2012_06_01::Client,
    guardduty_2017_11_28_client: aws_sdk_guardduty_2017_11_28::Client,
    cleanrooms_2022_02_17_client: aws_sdk_cleanrooms_2022_02_17::Client,
    trustedadvisor_2022_09_15_client: aws_sdk_trustedadvisor_2022_09_15::Client,
    sagemaker_a2i_runtime_2019_11_07_client: aws_sdk_sagemaker_a2i_runtime_2019_11_07::Client,
    dax_2017_04_19_client: aws_sdk_dax_2017_04_19::Client,
    docdb_2014_10_31_client: aws_sdk_docdb_2014_10_31::Client,
    firehose_2015_08_04_client: aws_sdk_firehose_2015_08_04::Client,
    ivschat_2020_07_14_client: aws_sdk_ivschat_2020_07_14::Client,
    ses_2010_12_01_client: aws_sdk_ses_2010_12_01::Client,
    bcm_dashboards_2025_08_18_client: aws_sdk_bcm_dashboards_2025_08_18::Client,
    application_insights_2018_11_25_client: aws_sdk_application_insights_2018_11_25::Client,
    device_farm_2015_06_23_client: aws_sdk_device_farm_2015_06_23::Client,
    account_2021_02_01_client: aws_sdk_account_2021_02_01::Client,
    launch_wizard_2018_05_10_client: aws_sdk_launch_wizard_2018_05_10::Client,
    finspace_data_2020_07_13_client: aws_sdk_finspace_data_2020_07_13::Client,
    appconfigdata_2021_11_11_client: aws_sdk_appconfigdata_2021_11_11::Client,
    controlcatalog_2018_05_10_client: aws_sdk_controlcatalog_2018_05_10::Client,
    greengrass_2017_06_07_client: aws_sdk_greengrass_2017_06_07::Client,
    kendra_ranking_2022_10_19_client: aws_sdk_kendra_ranking_2022_10_19::Client,
    snow_device_management_2021_08_04_client: aws_sdk_snow_device_management_2021_08_04::Client,
    securityhub_2018_10_26_client: aws_sdk_securityhub_2018_10_26::Client,
    s3vectors_2025_07_15_client: aws_sdk_s3vectors_2025_07_15::Client,
    workspaces_web_2020_07_08_client: aws_sdk_workspaces_web_2020_07_08::Client,
    backup_2018_11_15_client: aws_sdk_backup_2018_11_15::Client,
    opensearchserverless_2021_11_01_client: aws_sdk_opensearchserverless_2021_11_01::Client,
    cloudformation_2010_05_15_client: aws_sdk_cloudformation_2010_05_15::Client,
    kendra_2019_02_03_client: aws_sdk_kendra_2019_02_03::Client,
    connect_2017_08_08_client: aws_sdk_connect_2017_08_08::Client,
    machine_learning_2014_12_12_client: aws_sdk_machine_learning_2014_12_12::Client,
    elasticache_2015_02_02_client: aws_sdk_elasticache_2015_02_02::Client,
    sfn_2016_11_23_client: aws_sdk_sfn_2016_11_23::Client,
    sso_admin_2020_07_20_client: aws_sdk_sso_admin_2020_07_20::Client,
    auto_scaling_plans_2018_01_06_client: aws_sdk_auto_scaling_plans_2018_01_06::Client,
    comprehend_2017_11_27_client: aws_sdk_comprehend_2017_11_27::Client,
    rds_data_2018_08_01_client: aws_sdk_rds_data_2018_08_01::Client,
    chime_sdk_identity_2021_04_20_client: aws_sdk_chime_sdk_identity_2021_04_20::Client,
    rekognition_2016_06_27_client: aws_sdk_rekognition_2016_06_27::Client,
    appstream_2016_12_01_client: aws_sdk_appstream_2016_12_01::Client,
    polly_2016_06_10_client: aws_sdk_polly_2016_06_10::Client,
    invoicing_2024_12_01_client: aws_sdk_invoicing_2024_12_01::Client,
    rds_2014_10_31_client: aws_sdk_rds_2014_10_31::Client,
    pricing_2017_10_15_client: aws_sdk_pricing_2017_10_15::Client,
    swf_2012_01_25_client: aws_sdk_swf_2012_01_25::Client,
    cloudfront_keyvaluestore_2022_07_26_client: aws_sdk_cloudfront_keyvaluestore_2022_07_26::Client,
    marketplace_deployment_2023_01_25_client: aws_sdk_marketplace_deployment_2023_01_25::Client,
    medical_imaging_2023_07_19_client: aws_sdk_medical_imaging_2023_07_19::Client,
    transcribe_2017_10_26_client: aws_sdk_transcribe_2017_10_26::Client,
    observabilityadmin_2018_05_10_client: aws_sdk_observabilityadmin_2018_05_10::Client,
    notifications_2018_05_10_client: aws_sdk_notifications_2018_05_10::Client,
    codecommit_2015_04_13_client: aws_sdk_codecommit_2015_04_13::Client,
    lex_runtime_client: aws_sdk_lex_runtime::Client,
    pca_connector_ad_2018_05_10_client: aws_sdk_pca_connector_ad_2018_05_10::Client,
    forecastquery_2018_06_26_client: aws_sdk_forecastquery_2018_06_26::Client,
    healthlake_2017_07_01_client: aws_sdk_healthlake_2017_07_01::Client,
    rolesanywhere_2018_05_10_client: aws_sdk_rolesanywhere_2018_05_10::Client,
    marketplace_catalog_2018_09_17_client: aws_sdk_marketplace_catalog_2018_09_17::Client,
    billing_2023_09_07_client: aws_sdk_billing_2023_09_07::Client,
    emr_containers_2020_10_01_client: aws_sdk_emr_containers_2020_10_01::Client,
    apigatewaymanagementapi_2018_11_29_client: aws_sdk_apigatewaymanagementapi_2018_11_29::Client,
    xray_2016_04_12_client: aws_sdk_xray_2016_04_12::Client,
    transcribe_streaming_2017_10_26_client: aws_sdk_transcribe_streaming_2017_10_26::Client,
    ram_2018_01_04_client: aws_sdk_ram_2018_01_04::Client,
    codeconnections_2023_12_01_client: aws_sdk_codeconnections_2023_12_01::Client,
    efs_2015_02_01_client: aws_sdk_efs_2015_02_01::Client,
    migration_hub_refactor_spaces_2021_10_26_client: aws_sdk_migration_hub_refactor_spaces_2021_10_26::Client,
    elasticsearch_service_2015_01_01_client: aws_sdk_elasticsearch_service_2015_01_01::Client,
    cognito_identity_2014_06_30_client: aws_sdk_cognito_identity_2014_06_30::Client,
    payment_cryptography_data_2022_02_03_client: aws_sdk_payment_cryptography_data_2022_02_03::Client,
    dataexchange_2017_07_25_client: aws_sdk_dataexchange_2017_07_25::Client,
    sts_2011_06_15_client: aws_sdk_sts_2011_06_15::Client,
    sagemaker_2017_07_24_client: aws_sdk_sagemaker_2017_07_24::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    acm_2015_12_08_client: aws_sdk_acm_2015_12_08::Client,
    athena_2017_05_18_client: aws_sdk_athena_2017_05_18::Client,
    dsql_2018_05_10_client: aws_sdk_dsql_2018_05_10::Client,
    mediapackage_client: aws_sdk_mediapackage::Client,
    tnb_2008_10_21_client: aws_sdk_tnb_2008_10_21::Client,
    ec2_2016_11_15_client: aws_sdk_ec2_2016_11_15::Client,
    apprunner_2020_05_15_client: aws_sdk_apprunner_2020_05_15::Client,
    redshift_serverless_2021_04_21_client: aws_sdk_redshift_serverless_2021_04_21::Client,

}

impl AwsProvider {
    /// Create a new unified provider instance
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_from_env().await;
        let oam_2022_06_10_client = aws_sdk_oam_2022_06_10::Client::new(&config);
        let forecast_2018_06_26_client = aws_sdk_forecast_2018_06_26::Client::new(&config);
        let bedrock_runtime_2023_09_30_client = aws_sdk_bedrock_runtime_2023_09_30::Client::new(&config);
        let codebuild_2016_10_06_client = aws_sdk_codebuild_2016_10_06::Client::new(&config);
        let iotdeviceadvisor_2020_09_18_client = aws_sdk_iotdeviceadvisor_2020_09_18::Client::new(&config);
        let auto_scaling_2011_01_01_client = aws_sdk_auto_scaling_2011_01_01::Client::new(&config);
        let database_migration_service_2016_01_01_client = aws_sdk_database_migration_service_2016_01_01::Client::new(&config);
        let codeguruprofiler_2019_07_18_client = aws_sdk_codeguruprofiler_2019_07_18::Client::new(&config);
        let kinesis_2013_12_02_client = aws_sdk_kinesis_2013_12_02::Client::new(&config);
        let pinpoint_2016_12_01_client = aws_sdk_pinpoint_2016_12_01::Client::new(&config);
        let chime_2018_05_01_client = aws_sdk_chime_2018_05_01::Client::new(&config);
        let iottwinmaker_2021_11_29_client = aws_sdk_iottwinmaker_2021_11_29::Client::new(&config);
        let organizations_2016_11_28_client = aws_sdk_organizations_2016_11_28::Client::new(&config);
        let networkflowmonitor_2023_04_19_client = aws_sdk_networkflowmonitor_2023_04_19::Client::new(&config);
        let shield_2016_06_02_client = aws_sdk_shield_2016_06_02::Client::new(&config);
        let ssm_2014_11_06_client = aws_sdk_ssm_2014_11_06::Client::new(&config);
        let arc_zonal_shift_2022_10_30_client = aws_sdk_arc_zonal_shift_2022_10_30::Client::new(&config);
        let workspaces_thin_client_2023_08_22_client = aws_sdk_workspaces_thin_client_2023_08_22::Client::new(&config);
        let partnercentral_selling_2022_07_26_client = aws_sdk_partnercentral_selling_2022_07_26::Client::new(&config);
        let geo_places_2020_11_19_client = aws_sdk_geo_places_2020_11_19::Client::new(&config);
        let signer_2017_08_25_client = aws_sdk_signer_2017_08_25::Client::new(&config);
        let marketplace_reporting_2018_05_10_client = aws_sdk_marketplace_reporting_2018_05_10::Client::new(&config);
        let lakeformation_2017_03_31_client = aws_sdk_lakeformation_2017_03_31::Client::new(&config);
        let pcs_2023_02_10_client = aws_sdk_pcs_2023_02_10::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let mediaconnect_2018_11_14_client = aws_sdk_mediaconnect_2018_11_14::Client::new(&config);
        let secrets_manager_2017_10_17_client = aws_sdk_secrets_manager_2017_10_17::Client::new(&config);
        let mwaa_2020_07_01_client = aws_sdk_mwaa_2020_07_01::Client::new(&config);
        let kms_2014_11_01_client = aws_sdk_kms_2014_11_01::Client::new(&config);
        let quicksight_2018_04_01_client = aws_sdk_quicksight_2018_04_01::Client::new(&config);
        let workmail_2017_10_01_client = aws_sdk_workmail_2017_10_01::Client::new(&config);
        let eventbridge_2015_10_07_client = aws_sdk_eventbridge_2015_10_07::Client::new(&config);
        let frauddetector_2019_11_15_client = aws_sdk_frauddetector_2019_11_15::Client::new(&config);
        let cloudwatch_events_2015_10_07_client = aws_sdk_cloudwatch_events_2015_10_07::Client::new(&config);
        let cloudhsm_client = aws_sdk_cloudhsm::Client::new(&config);
        let cloudsearch_domain_2013_01_01_client = aws_sdk_cloudsearch_domain_2013_01_01::Client::new(&config);
        let lookoutequipment_2020_12_15_client = aws_sdk_lookoutequipment_2020_12_15::Client::new(&config);
        let iot_events_data_2018_10_23_client = aws_sdk_iot_events_data_2018_10_23::Client::new(&config);
        let securitylake_2018_05_10_client = aws_sdk_securitylake_2018_05_10::Client::new(&config);
        let cloudwatch_2010_08_01_client = aws_sdk_cloudwatch_2010_08_01::Client::new(&config);
        let glue_2017_03_31_client = aws_sdk_glue_2017_03_31::Client::new(&config);
        let application_auto_scaling_2016_02_06_client = aws_sdk_application_auto_scaling_2016_02_06::Client::new(&config);
        let personalize_events_2018_03_22_client = aws_sdk_personalize_events_2018_03_22::Client::new(&config);
        let voice_id_2021_09_27_client = aws_sdk_voice_id_2021_09_27::Client::new(&config);
        let s3_2006_03_01_client = aws_sdk_s3_2006_03_01::Client::new(&config);
        let appintegrations_2020_07_29_client = aws_sdk_appintegrations_2020_07_29::Client::new(&config);
        let lex_runtime_service_2016_11_28_client = aws_sdk_lex_runtime_service_2016_11_28::Client::new(&config);
        let ssm_contacts_2021_05_03_client = aws_sdk_ssm_contacts_2021_05_03::Client::new(&config);
        let chime_sdk_meetings_2021_07_15_client = aws_sdk_chime_sdk_meetings_2021_07_15::Client::new(&config);
        let sesv2_2019_09_27_client = aws_sdk_sesv2_2019_09_27::Client::new(&config);
        let bedrock_agentcore_control_2023_06_05_client = aws_sdk_bedrock_agentcore_control_2023_06_05::Client::new(&config);
        let emr_2009_03_31_client = aws_sdk_emr_2009_03_31::Client::new(&config);
        let controltower_2018_05_10_client = aws_sdk_controltower_2018_05_10::Client::new(&config);
        let resource_explorer_2_2022_07_28_client = aws_sdk_resource_explorer_2_2022_07_28::Client::new(&config);
        let s3_control_2018_08_20_client = aws_sdk_s3_control_2018_08_20::Client::new(&config);
        let rtbfabric_2023_05_15_client = aws_sdk_rtbfabric_2023_05_15::Client::new(&config);
        let personalize_2018_05_22_client = aws_sdk_personalize_2018_05_22::Client::new(&config);
        let marketplace_entitlement_service_2017_01_11_client = aws_sdk_marketplace_entitlement_service_2017_01_11::Client::new(&config);
        let directory_service_data_2023_05_31_client = aws_sdk_directory_service_data_2023_05_31::Client::new(&config);
        let ssm_sap_2018_05_10_client = aws_sdk_ssm_sap_2018_05_10::Client::new(&config);
        let outposts_2019_12_03_client = aws_sdk_outposts_2019_12_03::Client::new(&config);
        let workdocs_2016_05_01_client = aws_sdk_workdocs_2016_05_01::Client::new(&config);
        let networkmanager_2019_07_05_client = aws_sdk_networkmanager_2019_07_05::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let socialmessaging_2024_01_01_client = aws_sdk_socialmessaging_2024_01_01::Client::new(&config);
        let bedrock_agentcore_2024_02_28_client = aws_sdk_bedrock_agentcore_2024_02_28::Client::new(&config);
        let omics_2022_11_28_client = aws_sdk_omics_2022_11_28::Client::new(&config);
        let bcm_recommended_actions_2024_11_14_client = aws_sdk_bcm_recommended_actions_2024_11_14::Client::new(&config);
        let mediapackage_2017_10_12_client = aws_sdk_mediapackage_2017_10_12::Client::new(&config);
        let sagemaker_featurestore_runtime_2020_07_01_client = aws_sdk_sagemaker_featurestore_runtime_2020_07_01::Client::new(&config);
        let config_service_2014_11_12_client = aws_sdk_config_service_2014_11_12::Client::new(&config);
        let cloudwatch_logs_2014_03_28_client = aws_sdk_cloudwatch_logs_2014_03_28::Client::new(&config);
        let medialive_2017_10_14_client = aws_sdk_medialive_2017_10_14::Client::new(&config);
        let backup_gateway_2021_01_01_client = aws_sdk_backup_gateway_2021_01_01::Client::new(&config);
        let connect_contact_lens_2020_08_21_client = aws_sdk_connect_contact_lens_2020_08_21::Client::new(&config);
        let mediaconvert_2017_08_29_client = aws_sdk_mediaconvert_2017_08_29::Client::new(&config);
        let sns_2010_03_31_client = aws_sdk_sns_2010_03_31::Client::new(&config);
        let evs_2023_07_27_client = aws_sdk_evs_2023_07_27::Client::new(&config);
        let datasync_2018_11_09_client = aws_sdk_datasync_2018_11_09::Client::new(&config);
        let greengrassv2_2020_11_30_client = aws_sdk_greengrassv2_2020_11_30::Client::new(&config);
        let cleanroomsml_2023_09_06_client = aws_sdk_cleanroomsml_2023_09_06::Client::new(&config);
        let neptunedata_2023_08_01_client = aws_sdk_neptunedata_2023_08_01::Client::new(&config);
        let acm_pca_2017_08_22_client = aws_sdk_acm_pca_2017_08_22::Client::new(&config);
        let service_catalog_2015_12_10_client = aws_sdk_service_catalog_2015_12_10::Client::new(&config);
        let b2bi_2022_06_23_client = aws_sdk_b2bi_2022_06_23::Client::new(&config);
        let iotanalytics_2017_11_27_client = aws_sdk_iotanalytics_2017_11_27::Client::new(&config);
        let lex_model_building_service_2017_04_19_client = aws_sdk_lex_model_building_service_2017_04_19::Client::new(&config);
        let inspector2_2020_06_08_client = aws_sdk_inspector2_2020_06_08::Client::new(&config);
        let groundstation_2019_05_23_client = aws_sdk_groundstation_2019_05_23::Client::new(&config);
        let ecr_public_2020_10_30_client = aws_sdk_ecr_public_2020_10_30::Client::new(&config);
        let fis_2020_12_01_client = aws_sdk_fis_2020_12_01::Client::new(&config);
        let proton_2020_07_20_client = aws_sdk_proton_2020_07_20::Client::new(&config);
        let api_gateway_2015_07_09_client = aws_sdk_api_gateway_2015_07_09::Client::new(&config);
        let cloudhsm_2014_05_30_client = aws_sdk_cloudhsm_2014_05_30::Client::new(&config);
        let mpa_2022_07_26_client = aws_sdk_mpa_2022_07_26::Client::new(&config);
        let osis_2022_01_01_client = aws_sdk_osis_2022_01_01::Client::new(&config);
        let memorydb_2021_01_01_client = aws_sdk_memorydb_2021_01_01::Client::new(&config);
        let inspector_2016_02_16_client = aws_sdk_inspector_2016_02_16::Client::new(&config);
        let translate_2017_07_01_client = aws_sdk_translate_2017_07_01::Client::new(&config);
        let mailmanager_2023_10_17_client = aws_sdk_mailmanager_2023_10_17::Client::new(&config);
        let neptune_graph_2023_11_29_client = aws_sdk_neptune_graph_2023_11_29::Client::new(&config);
        let chatbot_2017_10_11_client = aws_sdk_chatbot_2017_10_11::Client::new(&config);
        let fms_2018_01_01_client = aws_sdk_fms_2018_01_01::Client::new(&config);
        let qapps_2023_11_27_client = aws_sdk_qapps_2023_11_27::Client::new(&config);
        let customer_profiles_2020_08_15_client = aws_sdk_customer_profiles_2020_08_15::Client::new(&config);
        let geo_maps_2020_11_19_client = aws_sdk_geo_maps_2020_11_19::Client::new(&config);
        let route_53_domains_2014_05_15_client = aws_sdk_route_53_domains_2014_05_15::Client::new(&config);
        let service_catalog_appregistry_2020_06_24_client = aws_sdk_service_catalog_appregistry_2020_06_24::Client::new(&config);
        let qbusiness_2023_11_27_client = aws_sdk_qbusiness_2023_11_27::Client::new(&config);
        let synthetics_2017_10_11_client = aws_sdk_synthetics_2017_10_11::Client::new(&config);
        let codecatalyst_2022_09_28_client = aws_sdk_codecatalyst_2022_09_28::Client::new(&config);
        let keyspacesstreams_2024_09_09_client = aws_sdk_keyspacesstreams_2024_09_09::Client::new(&config);
        let storage_gateway_2013_06_30_client = aws_sdk_storage_gateway_2013_06_30::Client::new(&config);
        let elastic_transcoder_2012_09_25_client = aws_sdk_elastic_transcoder_2012_09_25::Client::new(&config);
        let bcm_data_exports_2023_11_26_client = aws_sdk_bcm_data_exports_2023_11_26::Client::new(&config);
        let iotsecuretunneling_2018_10_05_client = aws_sdk_iotsecuretunneling_2018_10_05::Client::new(&config);
        let cloudfront_2020_05_31_client = aws_sdk_cloudfront_2020_05_31::Client::new(&config);
        let bedrock_data_automation_2023_07_26_client = aws_sdk_bedrock_data_automation_2023_07_26::Client::new(&config);
        let location_2020_11_19_client = aws_sdk_location_2020_11_19::Client::new(&config);
        let wafv2_2019_07_29_client = aws_sdk_wafv2_2019_07_29::Client::new(&config);
        let opensearch_2021_01_01_client = aws_sdk_opensearch_2021_01_01::Client::new(&config);
        let iotthingsgraph_2018_09_06_client = aws_sdk_iotthingsgraph_2018_09_06::Client::new(&config);
        let security_ir_2018_05_10_client = aws_sdk_security_ir_2018_05_10::Client::new(&config);
        let repostspace_2022_05_13_client = aws_sdk_repostspace_2022_05_13::Client::new(&config);
        let health_2016_08_04_client = aws_sdk_health_2016_08_04::Client::new(&config);
        let workmailmessageflow_2019_05_01_client = aws_sdk_workmailmessageflow_2019_05_01::Client::new(&config);
        let mediastore_data_2017_09_01_client = aws_sdk_mediastore_data_2017_09_01::Client::new(&config);
        let ec2_instance_connect_2018_04_02_client = aws_sdk_ec2_instance_connect_2018_04_02::Client::new(&config);
        let comprehendmedical_2018_10_30_client = aws_sdk_comprehendmedical_2018_10_30::Client::new(&config);
        let iotfleetwise_2021_06_17_client = aws_sdk_iotfleetwise_2021_06_17::Client::new(&config);
        let route53profiles_2018_05_10_client = aws_sdk_route53profiles_2018_05_10::Client::new(&config);
        let application_signals_2024_04_15_client = aws_sdk_application_signals_2024_04_15::Client::new(&config);
        let resource_groups_tagging_api_2017_01_26_client = aws_sdk_resource_groups_tagging_api_2017_01_26::Client::new(&config);
        let accessanalyzer_2019_11_01_client = aws_sdk_accessanalyzer_2019_11_01::Client::new(&config);
        let glacier_2012_06_01_client = aws_sdk_glacier_2012_06_01::Client::new(&config);
        let lightsail_2016_11_28_client = aws_sdk_lightsail_2016_11_28::Client::new(&config);
        let rum_2018_05_10_client = aws_sdk_rum_2018_05_10::Client::new(&config);
        let direct_connect_2012_10_25_client = aws_sdk_direct_connect_2012_10_25::Client::new(&config);
        let elastic_beanstalk_2010_12_01_client = aws_sdk_elastic_beanstalk_2010_12_01::Client::new(&config);
        let imagebuilder_2019_12_02_client = aws_sdk_imagebuilder_2019_12_02::Client::new(&config);
        let simspaceweaver_2022_10_28_client = aws_sdk_simspaceweaver_2022_10_28::Client::new(&config);
        let iot_jobs_data_plane_2017_09_29_client = aws_sdk_iot_jobs_data_plane_2017_09_29::Client::new(&config);
        let freetier_2023_09_07_client = aws_sdk_freetier_2023_09_07::Client::new(&config);
        let bedrock_agent_runtime_2023_07_26_client = aws_sdk_bedrock_agent_runtime_2023_07_26::Client::new(&config);
        let neptune_2014_10_31_client = aws_sdk_neptune_2014_10_31::Client::new(&config);
        let transfer_2018_11_05_client = aws_sdk_transfer_2018_11_05::Client::new(&config);
        let deadline_2023_10_12_client = aws_sdk_deadline_2023_10_12::Client::new(&config);
        let braket_2019_09_01_client = aws_sdk_braket_2019_09_01::Client::new(&config);
        let verifiedpermissions_2021_12_01_client = aws_sdk_verifiedpermissions_2021_12_01::Client::new(&config);
        let scheduler_2021_06_30_client = aws_sdk_scheduler_2021_06_30::Client::new(&config);
        let waf_regional_2016_11_28_client = aws_sdk_waf_regional_2016_11_28::Client::new(&config);
        let marketplace_commerce_analytics_2015_07_01_client = aws_sdk_marketplace_commerce_analytics_2015_07_01::Client::new(&config);
        let ecr_2015_09_21_client = aws_sdk_ecr_2015_09_21::Client::new(&config);
        let cost_and_usage_report_service_2017_01_06_client = aws_sdk_cost_and_usage_report_service_2017_01_06::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let dynamodb_2012_08_10_client = aws_sdk_dynamodb_2012_08_10::Client::new(&config);
        let resiliencehub_2020_04_30_client = aws_sdk_resiliencehub_2020_04_30::Client::new(&config);
        let macie2_2020_01_01_client = aws_sdk_macie2_2020_01_01::Client::new(&config);
        let entityresolution_2018_05_10_client = aws_sdk_entityresolution_2018_05_10::Client::new(&config);
        let s3outposts_2017_07_25_client = aws_sdk_s3outposts_2017_07_25::Client::new(&config);
        let grafana_2020_08_18_client = aws_sdk_grafana_2020_08_18::Client::new(&config);
        let kinesis_analytics_2015_08_14_client = aws_sdk_kinesis_analytics_2015_08_14::Client::new(&config);
        let eks_auth_2023_11_26_client = aws_sdk_eks_auth_2023_11_26::Client::new(&config);
        let rbin_2021_06_15_client = aws_sdk_rbin_2021_06_15::Client::new(&config);
        let service_quotas_2019_06_24_client = aws_sdk_service_quotas_2019_06_24::Client::new(&config);
        let ecs_2014_11_13_client = aws_sdk_ecs_2014_11_13::Client::new(&config);
        let marketplace_metering_2016_01_14_client = aws_sdk_marketplace_metering_2016_01_14::Client::new(&config);
        let timestream_write_2018_11_01_client = aws_sdk_timestream_write_2018_11_01::Client::new(&config);
        let qconnect_2020_10_19_client = aws_sdk_qconnect_2020_10_19::Client::new(&config);
        let cloudsearch_2013_01_01_client = aws_sdk_cloudsearch_2013_01_01::Client::new(&config);
        let bcm_pricing_calculator_2024_06_19_client = aws_sdk_bcm_pricing_calculator_2024_06_19::Client::new(&config);
        let appfabric_2023_05_19_client = aws_sdk_appfabric_2023_05_19::Client::new(&config);
        let route53resolver_2018_04_01_client = aws_sdk_route53resolver_2018_04_01::Client::new(&config);
        let marketplace_agreement_2020_03_01_client = aws_sdk_marketplace_agreement_2020_03_01::Client::new(&config);
        let workspaces_2015_04_08_client = aws_sdk_workspaces_2015_04_08::Client::new(&config);
        let gameliftstreams_2018_05_10_client = aws_sdk_gameliftstreams_2018_05_10::Client::new(&config);
        let taxsettings_2018_05_10_client = aws_sdk_taxsettings_2018_05_10::Client::new(&config);
        let pinpoint_sms_client = aws_sdk_pinpoint_sms::Client::new(&config);
        let fsx_2018_03_01_client = aws_sdk_fsx_2018_03_01::Client::new(&config);
        let codepipeline_2015_07_09_client = aws_sdk_codepipeline_2015_07_09::Client::new(&config);
        let schemas_2019_12_02_client = aws_sdk_schemas_2019_12_02::Client::new(&config);
        let emr_serverless_2021_07_13_client = aws_sdk_emr_serverless_2021_07_13::Client::new(&config);
        let sqs_2012_11_05_client = aws_sdk_sqs_2012_11_05::Client::new(&config);
        let license_manager_user_subscriptions_2018_05_10_client = aws_sdk_license_manager_user_subscriptions_2018_05_10::Client::new(&config);
        let route53_recovery_cluster_2019_12_02_client = aws_sdk_route53_recovery_cluster_2019_12_02::Client::new(&config);
        let migrationhuborchestrator_2021_08_28_client = aws_sdk_migrationhuborchestrator_2021_08_28::Client::new(&config);
        let iot_2015_05_28_client = aws_sdk_iot_2015_05_28::Client::new(&config);
        let sso_oidc_2019_06_10_client = aws_sdk_sso_oidc_2019_06_10::Client::new(&config);
        let codestar_notifications_2019_10_15_client = aws_sdk_codestar_notifications_2019_10_15::Client::new(&config);
        let ebs_2019_11_02_client = aws_sdk_ebs_2019_11_02::Client::new(&config);
        let aiops_2018_05_10_client = aws_sdk_aiops_2018_05_10::Client::new(&config);
        let amplify_2017_07_25_client = aws_sdk_amplify_2017_07_25::Client::new(&config);
        let cloudcontrol_2021_09_30_client = aws_sdk_cloudcontrol_2021_09_30::Client::new(&config);
        let wellarchitected_2020_03_31_client = aws_sdk_wellarchitected_2020_03_31::Client::new(&config);
        let route_53_2013_04_01_client = aws_sdk_route_53_2013_04_01::Client::new(&config);
        let ssm_incidents_2018_05_10_client = aws_sdk_ssm_incidents_2018_05_10::Client::new(&config);
        let bedrock_2023_04_20_client = aws_sdk_bedrock_2023_04_20::Client::new(&config);
        let ivs_realtime_2020_07_14_client = aws_sdk_ivs_realtime_2020_07_14::Client::new(&config);
        let migrationhub_config_2019_06_30_client = aws_sdk_migrationhub_config_2019_06_30::Client::new(&config);
        let redshift_2012_12_01_client = aws_sdk_redshift_2012_12_01::Client::new(&config);
        let inspector_scan_2023_08_08_client = aws_sdk_inspector_scan_2023_08_08::Client::new(&config);
        let connectcases_2022_10_03_client = aws_sdk_connectcases_2022_10_03::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let pca_connector_scep_2018_05_10_client = aws_sdk_pca_connector_scep_2018_05_10::Client::new(&config);
        let appflow_2020_08_23_client = aws_sdk_appflow_2020_08_23::Client::new(&config);
        let gamelift_2015_10_01_client = aws_sdk_gamelift_2015_10_01::Client::new(&config);
        let cloudtrail_2013_11_01_client = aws_sdk_cloudtrail_2013_11_01::Client::new(&config);
        let resource_groups_2017_11_27_client = aws_sdk_resource_groups_2017_11_27::Client::new(&config);
        let supplychain_2024_01_01_client = aws_sdk_supplychain_2024_01_01::Client::new(&config);
        let timestream_influxdb_2023_01_27_client = aws_sdk_timestream_influxdb_2023_01_27::Client::new(&config);
        let pipes_2015_10_07_client = aws_sdk_pipes_2015_10_07::Client::new(&config);
        let evidently_2021_02_01_client = aws_sdk_evidently_2021_02_01::Client::new(&config);
        let codeguru_security_2018_05_10_client = aws_sdk_codeguru_security_2018_05_10::Client::new(&config);
        let cost_optimization_hub_2022_07_26_client = aws_sdk_cost_optimization_hub_2022_07_26::Client::new(&config);
        let amplifyuibuilder_2021_08_11_client = aws_sdk_amplifyuibuilder_2021_08_11::Client::new(&config);
        let route53_recovery_control_config_2020_11_02_client = aws_sdk_route53_recovery_control_config_2020_11_02::Client::new(&config);
        let vpc_lattice_2022_11_30_client = aws_sdk_vpc_lattice_2022_11_30::Client::new(&config);
        let managedblockchain_query_2023_05_04_client = aws_sdk_managedblockchain_query_2023_05_04::Client::new(&config);
        let redshift_data_2019_12_20_client = aws_sdk_redshift_data_2019_12_20::Client::new(&config);
        let mediatailor_2018_04_23_client = aws_sdk_mediatailor_2018_04_23::Client::new(&config);
        let mediapackagev2_2022_12_25_client = aws_sdk_mediapackagev2_2022_12_25::Client::new(&config);
        let pi_2018_02_27_client = aws_sdk_pi_2018_02_27::Client::new(&config);
        let appconfig_2019_10_09_client = aws_sdk_appconfig_2019_10_09::Client::new(&config);
        let networkmonitor_2023_08_01_client = aws_sdk_networkmonitor_2023_08_01::Client::new(&config);
        let network_firewall_2020_11_12_client = aws_sdk_network_firewall_2020_11_12::Client::new(&config);
        let connectparticipant_2018_09_07_client = aws_sdk_connectparticipant_2018_09_07::Client::new(&config);
        let mgn_2020_02_26_client = aws_sdk_mgn_2020_02_26::Client::new(&config);
        let sagemaker_edge_2020_09_23_client = aws_sdk_sagemaker_edge_2020_09_23::Client::new(&config);
        let applicationcostprofiler_2020_09_10_client = aws_sdk_applicationcostprofiler_2020_09_10::Client::new(&config);
        let keyspaces_2022_02_10_client = aws_sdk_keyspaces_2022_02_10::Client::new(&config);
        let iam_2010_05_08_client = aws_sdk_iam_2010_05_08::Client::new(&config);
        let data_pipeline_2012_10_29_client = aws_sdk_data_pipeline_2012_10_29::Client::new(&config);
        let odb_2024_08_20_client = aws_sdk_odb_2024_08_20::Client::new(&config);
        let chime_sdk_messaging_2021_05_15_client = aws_sdk_chime_sdk_messaging_2021_05_15::Client::new(&config);
        let mediastore_2017_09_01_client = aws_sdk_mediastore_2017_09_01::Client::new(&config);
        let iot_wireless_2020_11_22_client = aws_sdk_iot_wireless_2020_11_22::Client::new(&config);
        let cloud9_2017_09_23_client = aws_sdk_cloud9_2017_09_23::Client::new(&config);
        let wisdom_2020_10_19_client = aws_sdk_wisdom_2020_10_19::Client::new(&config);
        let sagemaker_runtime_2017_05_13_client = aws_sdk_sagemaker_runtime_2017_05_13::Client::new(&config);
        let sso_2019_06_10_client = aws_sdk_sso_2019_06_10::Client::new(&config);
        let auditmanager_2017_07_25_client = aws_sdk_auditmanager_2017_07_25::Client::new(&config);
        let snowball_2016_06_30_client = aws_sdk_snowball_2016_06_30::Client::new(&config);
        let migration_hub_2017_05_31_client = aws_sdk_migration_hub_2017_05_31::Client::new(&config);
        let identitystore_2020_06_15_client = aws_sdk_identitystore_2020_06_15::Client::new(&config);
        let elastic_load_balancing_client = aws_sdk_elastic_load_balancing::Client::new(&config);
        let connectcampaigns_2021_01_30_client = aws_sdk_connectcampaigns_2021_01_30::Client::new(&config);
        let textract_2018_06_27_client = aws_sdk_textract_2018_06_27::Client::new(&config);
        let compute_optimizer_2019_11_01_client = aws_sdk_compute_optimizer_2019_11_01::Client::new(&config);
        let s3tables_2018_05_10_client = aws_sdk_s3tables_2018_05_10::Client::new(&config);
        let eks_2017_11_01_client = aws_sdk_eks_2017_11_01::Client::new(&config);
        let support_2013_04_15_client = aws_sdk_support_2013_04_15::Client::new(&config);
        let mturk_2017_01_17_client = aws_sdk_mturk_2017_01_17::Client::new(&config);
        let apigatewayv2_2018_11_29_client = aws_sdk_apigatewayv2_2018_11_29::Client::new(&config);
        let cognito_identity_provider_2016_04_18_client = aws_sdk_cognito_identity_provider_2016_04_18::Client::new(&config);
        let bedrock_data_automation_runtime_2024_06_13_client = aws_sdk_bedrock_data_automation_runtime_2024_06_13::Client::new(&config);
        let pinpoint_sms_client = aws_sdk_pinpoint_sms::Client::new(&config);
        let amp_2020_08_01_client = aws_sdk_amp_2020_08_01::Client::new(&config);
        let drs_2020_02_26_client = aws_sdk_drs_2020_02_26::Client::new(&config);
        let payment_cryptography_2021_09_14_client = aws_sdk_payment_cryptography_2021_09_14::Client::new(&config);
        let kafkaconnect_2021_09_14_client = aws_sdk_kafkaconnect_2021_09_14::Client::new(&config);
        let kafka_2018_11_14_client = aws_sdk_kafka_2018_11_14::Client::new(&config);
        let databrew_2017_07_25_client = aws_sdk_databrew_2017_07_25::Client::new(&config);
        let support_app_2021_08_20_client = aws_sdk_support_app_2021_08_20::Client::new(&config);
        let codedeploy_2014_10_06_client = aws_sdk_codedeploy_2014_10_06::Client::new(&config);
        let batch_2016_08_10_client = aws_sdk_batch_2016_08_10::Client::new(&config);
        let savingsplans_2019_06_28_client = aws_sdk_savingsplans_2019_06_28::Client::new(&config);
        let bedrock_agent_2023_06_05_client = aws_sdk_bedrock_agent_2023_06_05::Client::new(&config);
        let directory_service_2015_04_16_client = aws_sdk_directory_service_2015_04_16::Client::new(&config);
        let workspaces_instances_2022_07_26_client = aws_sdk_workspaces_instances_2022_07_26::Client::new(&config);
        let chime_sdk_media_pipelines_2021_07_15_client = aws_sdk_chime_sdk_media_pipelines_2021_07_15::Client::new(&config);
        let migrationhubstrategy_2020_02_19_client = aws_sdk_migrationhubstrategy_2020_02_19::Client::new(&config);
        let timestream_query_2018_11_01_client = aws_sdk_timestream_query_2018_11_01::Client::new(&config);
        let codeguru_reviewer_2019_09_19_client = aws_sdk_codeguru_reviewer_2019_09_19::Client::new(&config);
        let appsync_2017_07_25_client = aws_sdk_appsync_2017_07_25::Client::new(&config);
        let dlm_2018_01_12_client = aws_sdk_dlm_2018_01_12::Client::new(&config);
        let iot_data_plane_2015_05_28_client = aws_sdk_iot_data_plane_2015_05_28::Client::new(&config);
        let global_accelerator_2018_08_08_client = aws_sdk_global_accelerator_2018_08_08::Client::new(&config);
        let amplifybackend_2020_08_11_client = aws_sdk_amplifybackend_2020_08_11::Client::new(&config);
        let datazone_2018_05_10_client = aws_sdk_datazone_2018_05_10::Client::new(&config);
        let connectcampaignsv2_2024_04_23_client = aws_sdk_connectcampaignsv2_2024_04_23::Client::new(&config);
        let billingconductor_2021_07_30_client = aws_sdk_billingconductor_2021_07_30::Client::new(&config);
        let budgets_2016_10_20_client = aws_sdk_budgets_2016_10_20::Client::new(&config);
        let cloudtrail_data_2021_08_11_client = aws_sdk_cloudtrail_data_2021_08_11::Client::new(&config);
        let geo_routes_2020_11_19_client = aws_sdk_geo_routes_2020_11_19::Client::new(&config);
        let m2_2021_04_28_client = aws_sdk_m2_2021_04_28::Client::new(&config);
        let pinpoint_email_2018_07_26_client = aws_sdk_pinpoint_email_2018_07_26::Client::new(&config);
        let lex_models_client = aws_sdk_lex_models::Client::new(&config);
        let finspace_2021_03_12_client = aws_sdk_finspace_2021_03_12::Client::new(&config);
        let detective_2018_10_26_client = aws_sdk_detective_2018_10_26::Client::new(&config);
        let lambda_2015_03_31_client = aws_sdk_lambda_2015_03_31::Client::new(&config);
        let kinesis_analytics_client = aws_sdk_kinesis_analytics::Client::new(&config);
        let panorama_2019_07_24_client = aws_sdk_panorama_2019_07_24::Client::new(&config);
        let iot_events_2018_07_27_client = aws_sdk_iot_events_2018_07_27::Client::new(&config);
        let app_mesh_2019_01_25_client = aws_sdk_app_mesh_2019_01_25::Client::new(&config);
        let managedblockchain_2018_09_24_client = aws_sdk_managedblockchain_2018_09_24::Client::new(&config);
        let waf_2015_08_24_client = aws_sdk_waf_2015_08_24::Client::new(&config);
        let ivs_2020_07_14_client = aws_sdk_ivs_2020_07_14::Client::new(&config);
        let devops_guru_2020_12_01_client = aws_sdk_devops_guru_2020_12_01::Client::new(&config);
        let cost_explorer_2017_10_25_client = aws_sdk_cost_explorer_2017_10_25::Client::new(&config);
        let mq_2017_11_27_client = aws_sdk_mq_2017_11_27::Client::new(&config);
        let route53_recovery_readiness_2019_12_02_client = aws_sdk_route53_recovery_readiness_2019_12_02::Client::new(&config);
        let internetmonitor_2021_06_03_client = aws_sdk_internetmonitor_2021_06_03::Client::new(&config);
        let license_manager_2018_08_01_client = aws_sdk_license_manager_2018_08_01::Client::new(&config);
        let codestar_connections_2019_12_01_client = aws_sdk_codestar_connections_2019_12_01::Client::new(&config);
        let artifact_2018_05_10_client = aws_sdk_artifact_2018_05_10::Client::new(&config);
        let ssm_guiconnect_2021_05_01_client = aws_sdk_ssm_guiconnect_2021_05_01::Client::new(&config);
        let iotsitewise_2019_12_02_client = aws_sdk_iotsitewise_2019_12_02::Client::new(&config);
        let serverlessapplicationrepository_2017_09_08_client = aws_sdk_serverlessapplicationrepository_2017_09_08::Client::new(&config);
        let ssm_quicksetup_2018_05_10_client = aws_sdk_ssm_quicksetup_2018_05_10::Client::new(&config);
        let docdb_elastic_2022_11_28_client = aws_sdk_docdb_elastic_2022_11_28::Client::new(&config);
        let sagemaker_metrics_2022_09_30_client = aws_sdk_sagemaker_metrics_2022_09_30::Client::new(&config);
        let license_manager_linux_subscriptions_2018_05_10_client = aws_sdk_license_manager_linux_subscriptions_2018_05_10::Client::new(&config);
        let sagemaker_geospatial_2020_05_27_client = aws_sdk_sagemaker_geospatial_2020_05_27::Client::new(&config);
        let personalize_runtime_2018_05_22_client = aws_sdk_personalize_runtime_2018_05_22::Client::new(&config);
        let clouddirectory_2017_01_11_client = aws_sdk_clouddirectory_2017_01_11::Client::new(&config);
        let chime_sdk_client = aws_sdk_chime_sdk::Client::new(&config);
        let notificationscontacts_2018_05_10_client = aws_sdk_notificationscontacts_2018_05_10::Client::new(&config);
        let backupsearch_2018_05_10_client = aws_sdk_backupsearch_2018_05_10::Client::new(&config);
        let dynamodb_streams_2012_08_10_client = aws_sdk_dynamodb_streams_2012_08_10::Client::new(&config);
        let iot_managed_integrations_2025_03_03_client = aws_sdk_iot_managed_integrations_2025_03_03::Client::new(&config);
        let cognito_sync_2014_06_30_client = aws_sdk_cognito_sync_2014_06_30::Client::new(&config);
        let codeartifact_2018_09_22_client = aws_sdk_codeartifact_2018_09_22::Client::new(&config);
        let arc_region_switch_2022_07_26_client = aws_sdk_arc_region_switch_2022_07_26::Client::new(&config);
        let elastic_load_balancing_2012_06_01_client = aws_sdk_elastic_load_balancing_2012_06_01::Client::new(&config);
        let guardduty_2017_11_28_client = aws_sdk_guardduty_2017_11_28::Client::new(&config);
        let cleanrooms_2022_02_17_client = aws_sdk_cleanrooms_2022_02_17::Client::new(&config);
        let trustedadvisor_2022_09_15_client = aws_sdk_trustedadvisor_2022_09_15::Client::new(&config);
        let sagemaker_a2i_runtime_2019_11_07_client = aws_sdk_sagemaker_a2i_runtime_2019_11_07::Client::new(&config);
        let dax_2017_04_19_client = aws_sdk_dax_2017_04_19::Client::new(&config);
        let docdb_2014_10_31_client = aws_sdk_docdb_2014_10_31::Client::new(&config);
        let firehose_2015_08_04_client = aws_sdk_firehose_2015_08_04::Client::new(&config);
        let ivschat_2020_07_14_client = aws_sdk_ivschat_2020_07_14::Client::new(&config);
        let ses_2010_12_01_client = aws_sdk_ses_2010_12_01::Client::new(&config);
        let bcm_dashboards_2025_08_18_client = aws_sdk_bcm_dashboards_2025_08_18::Client::new(&config);
        let application_insights_2018_11_25_client = aws_sdk_application_insights_2018_11_25::Client::new(&config);
        let device_farm_2015_06_23_client = aws_sdk_device_farm_2015_06_23::Client::new(&config);
        let account_2021_02_01_client = aws_sdk_account_2021_02_01::Client::new(&config);
        let launch_wizard_2018_05_10_client = aws_sdk_launch_wizard_2018_05_10::Client::new(&config);
        let finspace_data_2020_07_13_client = aws_sdk_finspace_data_2020_07_13::Client::new(&config);
        let appconfigdata_2021_11_11_client = aws_sdk_appconfigdata_2021_11_11::Client::new(&config);
        let controlcatalog_2018_05_10_client = aws_sdk_controlcatalog_2018_05_10::Client::new(&config);
        let greengrass_2017_06_07_client = aws_sdk_greengrass_2017_06_07::Client::new(&config);
        let kendra_ranking_2022_10_19_client = aws_sdk_kendra_ranking_2022_10_19::Client::new(&config);
        let snow_device_management_2021_08_04_client = aws_sdk_snow_device_management_2021_08_04::Client::new(&config);
        let securityhub_2018_10_26_client = aws_sdk_securityhub_2018_10_26::Client::new(&config);
        let s3vectors_2025_07_15_client = aws_sdk_s3vectors_2025_07_15::Client::new(&config);
        let workspaces_web_2020_07_08_client = aws_sdk_workspaces_web_2020_07_08::Client::new(&config);
        let backup_2018_11_15_client = aws_sdk_backup_2018_11_15::Client::new(&config);
        let opensearchserverless_2021_11_01_client = aws_sdk_opensearchserverless_2021_11_01::Client::new(&config);
        let cloudformation_2010_05_15_client = aws_sdk_cloudformation_2010_05_15::Client::new(&config);
        let kendra_2019_02_03_client = aws_sdk_kendra_2019_02_03::Client::new(&config);
        let connect_2017_08_08_client = aws_sdk_connect_2017_08_08::Client::new(&config);
        let machine_learning_2014_12_12_client = aws_sdk_machine_learning_2014_12_12::Client::new(&config);
        let elasticache_2015_02_02_client = aws_sdk_elasticache_2015_02_02::Client::new(&config);
        let sfn_2016_11_23_client = aws_sdk_sfn_2016_11_23::Client::new(&config);
        let sso_admin_2020_07_20_client = aws_sdk_sso_admin_2020_07_20::Client::new(&config);
        let auto_scaling_plans_2018_01_06_client = aws_sdk_auto_scaling_plans_2018_01_06::Client::new(&config);
        let comprehend_2017_11_27_client = aws_sdk_comprehend_2017_11_27::Client::new(&config);
        let rds_data_2018_08_01_client = aws_sdk_rds_data_2018_08_01::Client::new(&config);
        let chime_sdk_identity_2021_04_20_client = aws_sdk_chime_sdk_identity_2021_04_20::Client::new(&config);
        let rekognition_2016_06_27_client = aws_sdk_rekognition_2016_06_27::Client::new(&config);
        let appstream_2016_12_01_client = aws_sdk_appstream_2016_12_01::Client::new(&config);
        let polly_2016_06_10_client = aws_sdk_polly_2016_06_10::Client::new(&config);
        let invoicing_2024_12_01_client = aws_sdk_invoicing_2024_12_01::Client::new(&config);
        let rds_2014_10_31_client = aws_sdk_rds_2014_10_31::Client::new(&config);
        let pricing_2017_10_15_client = aws_sdk_pricing_2017_10_15::Client::new(&config);
        let swf_2012_01_25_client = aws_sdk_swf_2012_01_25::Client::new(&config);
        let cloudfront_keyvaluestore_2022_07_26_client = aws_sdk_cloudfront_keyvaluestore_2022_07_26::Client::new(&config);
        let marketplace_deployment_2023_01_25_client = aws_sdk_marketplace_deployment_2023_01_25::Client::new(&config);
        let medical_imaging_2023_07_19_client = aws_sdk_medical_imaging_2023_07_19::Client::new(&config);
        let transcribe_2017_10_26_client = aws_sdk_transcribe_2017_10_26::Client::new(&config);
        let observabilityadmin_2018_05_10_client = aws_sdk_observabilityadmin_2018_05_10::Client::new(&config);
        let notifications_2018_05_10_client = aws_sdk_notifications_2018_05_10::Client::new(&config);
        let codecommit_2015_04_13_client = aws_sdk_codecommit_2015_04_13::Client::new(&config);
        let lex_runtime_client = aws_sdk_lex_runtime::Client::new(&config);
        let pca_connector_ad_2018_05_10_client = aws_sdk_pca_connector_ad_2018_05_10::Client::new(&config);
        let forecastquery_2018_06_26_client = aws_sdk_forecastquery_2018_06_26::Client::new(&config);
        let healthlake_2017_07_01_client = aws_sdk_healthlake_2017_07_01::Client::new(&config);
        let rolesanywhere_2018_05_10_client = aws_sdk_rolesanywhere_2018_05_10::Client::new(&config);
        let marketplace_catalog_2018_09_17_client = aws_sdk_marketplace_catalog_2018_09_17::Client::new(&config);
        let billing_2023_09_07_client = aws_sdk_billing_2023_09_07::Client::new(&config);
        let emr_containers_2020_10_01_client = aws_sdk_emr_containers_2020_10_01::Client::new(&config);
        let apigatewaymanagementapi_2018_11_29_client = aws_sdk_apigatewaymanagementapi_2018_11_29::Client::new(&config);
        let xray_2016_04_12_client = aws_sdk_xray_2016_04_12::Client::new(&config);
        let transcribe_streaming_2017_10_26_client = aws_sdk_transcribe_streaming_2017_10_26::Client::new(&config);
        let ram_2018_01_04_client = aws_sdk_ram_2018_01_04::Client::new(&config);
        let codeconnections_2023_12_01_client = aws_sdk_codeconnections_2023_12_01::Client::new(&config);
        let efs_2015_02_01_client = aws_sdk_efs_2015_02_01::Client::new(&config);
        let migration_hub_refactor_spaces_2021_10_26_client = aws_sdk_migration_hub_refactor_spaces_2021_10_26::Client::new(&config);
        let elasticsearch_service_2015_01_01_client = aws_sdk_elasticsearch_service_2015_01_01::Client::new(&config);
        let cognito_identity_2014_06_30_client = aws_sdk_cognito_identity_2014_06_30::Client::new(&config);
        let payment_cryptography_data_2022_02_03_client = aws_sdk_payment_cryptography_data_2022_02_03::Client::new(&config);
        let dataexchange_2017_07_25_client = aws_sdk_dataexchange_2017_07_25::Client::new(&config);
        let sts_2011_06_15_client = aws_sdk_sts_2011_06_15::Client::new(&config);
        let sagemaker_2017_07_24_client = aws_sdk_sagemaker_2017_07_24::Client::new(&config);
        let kinesis_client = aws_sdk_kinesis::Client::new(&config);
        let acm_2015_12_08_client = aws_sdk_acm_2015_12_08::Client::new(&config);
        let athena_2017_05_18_client = aws_sdk_athena_2017_05_18::Client::new(&config);
        let dsql_2018_05_10_client = aws_sdk_dsql_2018_05_10::Client::new(&config);
        let mediapackage_client = aws_sdk_mediapackage::Client::new(&config);
        let tnb_2008_10_21_client = aws_sdk_tnb_2008_10_21::Client::new(&config);
        let ec2_2016_11_15_client = aws_sdk_ec2_2016_11_15::Client::new(&config);
        let apprunner_2020_05_15_client = aws_sdk_apprunner_2020_05_15::Client::new(&config);
        let redshift_serverless_2021_04_21_client = aws_sdk_redshift_serverless_2021_04_21::Client::new(&config);

        Ok(Self {
            oam_2022_06_10_client,
            forecast_2018_06_26_client,
            bedrock_runtime_2023_09_30_client,
            codebuild_2016_10_06_client,
            iotdeviceadvisor_2020_09_18_client,
            auto_scaling_2011_01_01_client,
            database_migration_service_2016_01_01_client,
            codeguruprofiler_2019_07_18_client,
            kinesis_2013_12_02_client,
            pinpoint_2016_12_01_client,
            chime_2018_05_01_client,
            iottwinmaker_2021_11_29_client,
            organizations_2016_11_28_client,
            networkflowmonitor_2023_04_19_client,
            shield_2016_06_02_client,
            ssm_2014_11_06_client,
            arc_zonal_shift_2022_10_30_client,
            workspaces_thin_client_2023_08_22_client,
            partnercentral_selling_2022_07_26_client,
            geo_places_2020_11_19_client,
            signer_2017_08_25_client,
            marketplace_reporting_2018_05_10_client,
            lakeformation_2017_03_31_client,
            pcs_2023_02_10_client,
            kinesis_client,
            mediaconnect_2018_11_14_client,
            secrets_manager_2017_10_17_client,
            mwaa_2020_07_01_client,
            kms_2014_11_01_client,
            quicksight_2018_04_01_client,
            workmail_2017_10_01_client,
            eventbridge_2015_10_07_client,
            frauddetector_2019_11_15_client,
            cloudwatch_events_2015_10_07_client,
            cloudhsm_client,
            cloudsearch_domain_2013_01_01_client,
            lookoutequipment_2020_12_15_client,
            iot_events_data_2018_10_23_client,
            securitylake_2018_05_10_client,
            cloudwatch_2010_08_01_client,
            glue_2017_03_31_client,
            application_auto_scaling_2016_02_06_client,
            personalize_events_2018_03_22_client,
            voice_id_2021_09_27_client,
            s3_2006_03_01_client,
            appintegrations_2020_07_29_client,
            lex_runtime_service_2016_11_28_client,
            ssm_contacts_2021_05_03_client,
            chime_sdk_meetings_2021_07_15_client,
            sesv2_2019_09_27_client,
            bedrock_agentcore_control_2023_06_05_client,
            emr_2009_03_31_client,
            controltower_2018_05_10_client,
            resource_explorer_2_2022_07_28_client,
            s3_control_2018_08_20_client,
            rtbfabric_2023_05_15_client,
            personalize_2018_05_22_client,
            marketplace_entitlement_service_2017_01_11_client,
            directory_service_data_2023_05_31_client,
            ssm_sap_2018_05_10_client,
            outposts_2019_12_03_client,
            workdocs_2016_05_01_client,
            networkmanager_2019_07_05_client,
            kinesis_client,
            socialmessaging_2024_01_01_client,
            bedrock_agentcore_2024_02_28_client,
            omics_2022_11_28_client,
            bcm_recommended_actions_2024_11_14_client,
            mediapackage_2017_10_12_client,
            sagemaker_featurestore_runtime_2020_07_01_client,
            config_service_2014_11_12_client,
            cloudwatch_logs_2014_03_28_client,
            medialive_2017_10_14_client,
            backup_gateway_2021_01_01_client,
            connect_contact_lens_2020_08_21_client,
            mediaconvert_2017_08_29_client,
            sns_2010_03_31_client,
            evs_2023_07_27_client,
            datasync_2018_11_09_client,
            greengrassv2_2020_11_30_client,
            cleanroomsml_2023_09_06_client,
            neptunedata_2023_08_01_client,
            acm_pca_2017_08_22_client,
            service_catalog_2015_12_10_client,
            b2bi_2022_06_23_client,
            iotanalytics_2017_11_27_client,
            lex_model_building_service_2017_04_19_client,
            inspector2_2020_06_08_client,
            groundstation_2019_05_23_client,
            ecr_public_2020_10_30_client,
            fis_2020_12_01_client,
            proton_2020_07_20_client,
            api_gateway_2015_07_09_client,
            cloudhsm_2014_05_30_client,
            mpa_2022_07_26_client,
            osis_2022_01_01_client,
            memorydb_2021_01_01_client,
            inspector_2016_02_16_client,
            translate_2017_07_01_client,
            mailmanager_2023_10_17_client,
            neptune_graph_2023_11_29_client,
            chatbot_2017_10_11_client,
            fms_2018_01_01_client,
            qapps_2023_11_27_client,
            customer_profiles_2020_08_15_client,
            geo_maps_2020_11_19_client,
            route_53_domains_2014_05_15_client,
            service_catalog_appregistry_2020_06_24_client,
            qbusiness_2023_11_27_client,
            synthetics_2017_10_11_client,
            codecatalyst_2022_09_28_client,
            keyspacesstreams_2024_09_09_client,
            storage_gateway_2013_06_30_client,
            elastic_transcoder_2012_09_25_client,
            bcm_data_exports_2023_11_26_client,
            iotsecuretunneling_2018_10_05_client,
            cloudfront_2020_05_31_client,
            bedrock_data_automation_2023_07_26_client,
            location_2020_11_19_client,
            wafv2_2019_07_29_client,
            opensearch_2021_01_01_client,
            iotthingsgraph_2018_09_06_client,
            security_ir_2018_05_10_client,
            repostspace_2022_05_13_client,
            health_2016_08_04_client,
            workmailmessageflow_2019_05_01_client,
            mediastore_data_2017_09_01_client,
            ec2_instance_connect_2018_04_02_client,
            comprehendmedical_2018_10_30_client,
            iotfleetwise_2021_06_17_client,
            route53profiles_2018_05_10_client,
            application_signals_2024_04_15_client,
            resource_groups_tagging_api_2017_01_26_client,
            accessanalyzer_2019_11_01_client,
            glacier_2012_06_01_client,
            lightsail_2016_11_28_client,
            rum_2018_05_10_client,
            direct_connect_2012_10_25_client,
            elastic_beanstalk_2010_12_01_client,
            imagebuilder_2019_12_02_client,
            simspaceweaver_2022_10_28_client,
            iot_jobs_data_plane_2017_09_29_client,
            freetier_2023_09_07_client,
            bedrock_agent_runtime_2023_07_26_client,
            neptune_2014_10_31_client,
            transfer_2018_11_05_client,
            deadline_2023_10_12_client,
            braket_2019_09_01_client,
            verifiedpermissions_2021_12_01_client,
            scheduler_2021_06_30_client,
            waf_regional_2016_11_28_client,
            marketplace_commerce_analytics_2015_07_01_client,
            ecr_2015_09_21_client,
            cost_and_usage_report_service_2017_01_06_client,
            kinesis_client,
            dynamodb_2012_08_10_client,
            resiliencehub_2020_04_30_client,
            macie2_2020_01_01_client,
            entityresolution_2018_05_10_client,
            s3outposts_2017_07_25_client,
            grafana_2020_08_18_client,
            kinesis_analytics_2015_08_14_client,
            eks_auth_2023_11_26_client,
            rbin_2021_06_15_client,
            service_quotas_2019_06_24_client,
            ecs_2014_11_13_client,
            marketplace_metering_2016_01_14_client,
            timestream_write_2018_11_01_client,
            qconnect_2020_10_19_client,
            cloudsearch_2013_01_01_client,
            bcm_pricing_calculator_2024_06_19_client,
            appfabric_2023_05_19_client,
            route53resolver_2018_04_01_client,
            marketplace_agreement_2020_03_01_client,
            workspaces_2015_04_08_client,
            gameliftstreams_2018_05_10_client,
            taxsettings_2018_05_10_client,
            pinpoint_sms_client,
            fsx_2018_03_01_client,
            codepipeline_2015_07_09_client,
            schemas_2019_12_02_client,
            emr_serverless_2021_07_13_client,
            sqs_2012_11_05_client,
            license_manager_user_subscriptions_2018_05_10_client,
            route53_recovery_cluster_2019_12_02_client,
            migrationhuborchestrator_2021_08_28_client,
            iot_2015_05_28_client,
            sso_oidc_2019_06_10_client,
            codestar_notifications_2019_10_15_client,
            ebs_2019_11_02_client,
            aiops_2018_05_10_client,
            amplify_2017_07_25_client,
            cloudcontrol_2021_09_30_client,
            wellarchitected_2020_03_31_client,
            route_53_2013_04_01_client,
            ssm_incidents_2018_05_10_client,
            bedrock_2023_04_20_client,
            ivs_realtime_2020_07_14_client,
            migrationhub_config_2019_06_30_client,
            redshift_2012_12_01_client,
            inspector_scan_2023_08_08_client,
            connectcases_2022_10_03_client,
            kinesis_client,
            pca_connector_scep_2018_05_10_client,
            appflow_2020_08_23_client,
            gamelift_2015_10_01_client,
            cloudtrail_2013_11_01_client,
            resource_groups_2017_11_27_client,
            supplychain_2024_01_01_client,
            timestream_influxdb_2023_01_27_client,
            pipes_2015_10_07_client,
            evidently_2021_02_01_client,
            codeguru_security_2018_05_10_client,
            cost_optimization_hub_2022_07_26_client,
            amplifyuibuilder_2021_08_11_client,
            route53_recovery_control_config_2020_11_02_client,
            vpc_lattice_2022_11_30_client,
            managedblockchain_query_2023_05_04_client,
            redshift_data_2019_12_20_client,
            mediatailor_2018_04_23_client,
            mediapackagev2_2022_12_25_client,
            pi_2018_02_27_client,
            appconfig_2019_10_09_client,
            networkmonitor_2023_08_01_client,
            network_firewall_2020_11_12_client,
            connectparticipant_2018_09_07_client,
            mgn_2020_02_26_client,
            sagemaker_edge_2020_09_23_client,
            applicationcostprofiler_2020_09_10_client,
            keyspaces_2022_02_10_client,
            iam_2010_05_08_client,
            data_pipeline_2012_10_29_client,
            odb_2024_08_20_client,
            chime_sdk_messaging_2021_05_15_client,
            mediastore_2017_09_01_client,
            iot_wireless_2020_11_22_client,
            cloud9_2017_09_23_client,
            wisdom_2020_10_19_client,
            sagemaker_runtime_2017_05_13_client,
            sso_2019_06_10_client,
            auditmanager_2017_07_25_client,
            snowball_2016_06_30_client,
            migration_hub_2017_05_31_client,
            identitystore_2020_06_15_client,
            elastic_load_balancing_client,
            connectcampaigns_2021_01_30_client,
            textract_2018_06_27_client,
            compute_optimizer_2019_11_01_client,
            s3tables_2018_05_10_client,
            eks_2017_11_01_client,
            support_2013_04_15_client,
            mturk_2017_01_17_client,
            apigatewayv2_2018_11_29_client,
            cognito_identity_provider_2016_04_18_client,
            bedrock_data_automation_runtime_2024_06_13_client,
            pinpoint_sms_client,
            amp_2020_08_01_client,
            drs_2020_02_26_client,
            payment_cryptography_2021_09_14_client,
            kafkaconnect_2021_09_14_client,
            kafka_2018_11_14_client,
            databrew_2017_07_25_client,
            support_app_2021_08_20_client,
            codedeploy_2014_10_06_client,
            batch_2016_08_10_client,
            savingsplans_2019_06_28_client,
            bedrock_agent_2023_06_05_client,
            directory_service_2015_04_16_client,
            workspaces_instances_2022_07_26_client,
            chime_sdk_media_pipelines_2021_07_15_client,
            migrationhubstrategy_2020_02_19_client,
            timestream_query_2018_11_01_client,
            codeguru_reviewer_2019_09_19_client,
            appsync_2017_07_25_client,
            dlm_2018_01_12_client,
            iot_data_plane_2015_05_28_client,
            global_accelerator_2018_08_08_client,
            amplifybackend_2020_08_11_client,
            datazone_2018_05_10_client,
            connectcampaignsv2_2024_04_23_client,
            billingconductor_2021_07_30_client,
            budgets_2016_10_20_client,
            cloudtrail_data_2021_08_11_client,
            geo_routes_2020_11_19_client,
            m2_2021_04_28_client,
            pinpoint_email_2018_07_26_client,
            lex_models_client,
            finspace_2021_03_12_client,
            detective_2018_10_26_client,
            lambda_2015_03_31_client,
            kinesis_analytics_client,
            panorama_2019_07_24_client,
            iot_events_2018_07_27_client,
            app_mesh_2019_01_25_client,
            managedblockchain_2018_09_24_client,
            waf_2015_08_24_client,
            ivs_2020_07_14_client,
            devops_guru_2020_12_01_client,
            cost_explorer_2017_10_25_client,
            mq_2017_11_27_client,
            route53_recovery_readiness_2019_12_02_client,
            internetmonitor_2021_06_03_client,
            license_manager_2018_08_01_client,
            codestar_connections_2019_12_01_client,
            artifact_2018_05_10_client,
            ssm_guiconnect_2021_05_01_client,
            iotsitewise_2019_12_02_client,
            serverlessapplicationrepository_2017_09_08_client,
            ssm_quicksetup_2018_05_10_client,
            docdb_elastic_2022_11_28_client,
            sagemaker_metrics_2022_09_30_client,
            license_manager_linux_subscriptions_2018_05_10_client,
            sagemaker_geospatial_2020_05_27_client,
            personalize_runtime_2018_05_22_client,
            clouddirectory_2017_01_11_client,
            chime_sdk_client,
            notificationscontacts_2018_05_10_client,
            backupsearch_2018_05_10_client,
            dynamodb_streams_2012_08_10_client,
            iot_managed_integrations_2025_03_03_client,
            cognito_sync_2014_06_30_client,
            codeartifact_2018_09_22_client,
            arc_region_switch_2022_07_26_client,
            elastic_load_balancing_2012_06_01_client,
            guardduty_2017_11_28_client,
            cleanrooms_2022_02_17_client,
            trustedadvisor_2022_09_15_client,
            sagemaker_a2i_runtime_2019_11_07_client,
            dax_2017_04_19_client,
            docdb_2014_10_31_client,
            firehose_2015_08_04_client,
            ivschat_2020_07_14_client,
            ses_2010_12_01_client,
            bcm_dashboards_2025_08_18_client,
            application_insights_2018_11_25_client,
            device_farm_2015_06_23_client,
            account_2021_02_01_client,
            launch_wizard_2018_05_10_client,
            finspace_data_2020_07_13_client,
            appconfigdata_2021_11_11_client,
            controlcatalog_2018_05_10_client,
            greengrass_2017_06_07_client,
            kendra_ranking_2022_10_19_client,
            snow_device_management_2021_08_04_client,
            securityhub_2018_10_26_client,
            s3vectors_2025_07_15_client,
            workspaces_web_2020_07_08_client,
            backup_2018_11_15_client,
            opensearchserverless_2021_11_01_client,
            cloudformation_2010_05_15_client,
            kendra_2019_02_03_client,
            connect_2017_08_08_client,
            machine_learning_2014_12_12_client,
            elasticache_2015_02_02_client,
            sfn_2016_11_23_client,
            sso_admin_2020_07_20_client,
            auto_scaling_plans_2018_01_06_client,
            comprehend_2017_11_27_client,
            rds_data_2018_08_01_client,
            chime_sdk_identity_2021_04_20_client,
            rekognition_2016_06_27_client,
            appstream_2016_12_01_client,
            polly_2016_06_10_client,
            invoicing_2024_12_01_client,
            rds_2014_10_31_client,
            pricing_2017_10_15_client,
            swf_2012_01_25_client,
            cloudfront_keyvaluestore_2022_07_26_client,
            marketplace_deployment_2023_01_25_client,
            medical_imaging_2023_07_19_client,
            transcribe_2017_10_26_client,
            observabilityadmin_2018_05_10_client,
            notifications_2018_05_10_client,
            codecommit_2015_04_13_client,
            lex_runtime_client,
            pca_connector_ad_2018_05_10_client,
            forecastquery_2018_06_26_client,
            healthlake_2017_07_01_client,
            rolesanywhere_2018_05_10_client,
            marketplace_catalog_2018_09_17_client,
            billing_2023_09_07_client,
            emr_containers_2020_10_01_client,
            apigatewaymanagementapi_2018_11_29_client,
            xray_2016_04_12_client,
            transcribe_streaming_2017_10_26_client,
            ram_2018_01_04_client,
            codeconnections_2023_12_01_client,
            efs_2015_02_01_client,
            migration_hub_refactor_spaces_2021_10_26_client,
            elasticsearch_service_2015_01_01_client,
            cognito_identity_2014_06_30_client,
            payment_cryptography_data_2022_02_03_client,
            dataexchange_2017_07_25_client,
            sts_2011_06_15_client,
            sagemaker_2017_07_24_client,
            kinesis_client,
            acm_2015_12_08_client,
            athena_2017_05_18_client,
            dsql_2018_05_10_client,
            mediapackage_client,
            tnb_2008_10_21_client,
            ec2_2016_11_15_client,
            apprunner_2020_05_15_client,
            redshift_serverless_2021_04_21_client,

        })
    }

    /// Get oam_2022_06_10 service handler
    pub fn oam_2022_06_10(&self) -> oam_2022_06_10::Oam_2022_06_10Service<'_> {
        oam_2022_06_10::Oam_2022_06_10Service::new(self)
    }
    /// Get forecast_2018_06_26 service handler
    pub fn forecast_2018_06_26(&self) -> forecast_2018_06_26::Forecast_2018_06_26Service<'_> {
        forecast_2018_06_26::Forecast_2018_06_26Service::new(self)
    }
    /// Get bedrock_runtime_2023_09_30 service handler
    pub fn bedrock_runtime_2023_09_30(&self) -> bedrock_runtime_2023_09_30::Bedrock_runtime_2023_09_30Service<'_> {
        bedrock_runtime_2023_09_30::Bedrock_runtime_2023_09_30Service::new(self)
    }
    /// Get codebuild_2016_10_06 service handler
    pub fn codebuild_2016_10_06(&self) -> codebuild_2016_10_06::Codebuild_2016_10_06Service<'_> {
        codebuild_2016_10_06::Codebuild_2016_10_06Service::new(self)
    }
    /// Get iotdeviceadvisor_2020_09_18 service handler
    pub fn iotdeviceadvisor_2020_09_18(&self) -> iotdeviceadvisor_2020_09_18::Iotdeviceadvisor_2020_09_18Service<'_> {
        iotdeviceadvisor_2020_09_18::Iotdeviceadvisor_2020_09_18Service::new(self)
    }
    /// Get auto_scaling_2011_01_01 service handler
    pub fn auto_scaling_2011_01_01(&self) -> auto_scaling_2011_01_01::Auto_scaling_2011_01_01Service<'_> {
        auto_scaling_2011_01_01::Auto_scaling_2011_01_01Service::new(self)
    }
    /// Get database_migration_service_2016_01_01 service handler
    pub fn database_migration_service_2016_01_01(&self) -> database_migration_service_2016_01_01::Database_migration_service_2016_01_01Service<'_> {
        database_migration_service_2016_01_01::Database_migration_service_2016_01_01Service::new(self)
    }
    /// Get codeguruprofiler_2019_07_18 service handler
    pub fn codeguruprofiler_2019_07_18(&self) -> codeguruprofiler_2019_07_18::Codeguruprofiler_2019_07_18Service<'_> {
        codeguruprofiler_2019_07_18::Codeguruprofiler_2019_07_18Service::new(self)
    }
    /// Get kinesis_2013_12_02 service handler
    pub fn kinesis_2013_12_02(&self) -> kinesis_2013_12_02::Kinesis_2013_12_02Service<'_> {
        kinesis_2013_12_02::Kinesis_2013_12_02Service::new(self)
    }
    /// Get pinpoint_2016_12_01 service handler
    pub fn pinpoint_2016_12_01(&self) -> pinpoint_2016_12_01::Pinpoint_2016_12_01Service<'_> {
        pinpoint_2016_12_01::Pinpoint_2016_12_01Service::new(self)
    }
    /// Get chime_2018_05_01 service handler
    pub fn chime_2018_05_01(&self) -> chime_2018_05_01::Chime_2018_05_01Service<'_> {
        chime_2018_05_01::Chime_2018_05_01Service::new(self)
    }
    /// Get iottwinmaker_2021_11_29 service handler
    pub fn iottwinmaker_2021_11_29(&self) -> iottwinmaker_2021_11_29::Iottwinmaker_2021_11_29Service<'_> {
        iottwinmaker_2021_11_29::Iottwinmaker_2021_11_29Service::new(self)
    }
    /// Get organizations_2016_11_28 service handler
    pub fn organizations_2016_11_28(&self) -> organizations_2016_11_28::Organizations_2016_11_28Service<'_> {
        organizations_2016_11_28::Organizations_2016_11_28Service::new(self)
    }
    /// Get networkflowmonitor_2023_04_19 service handler
    pub fn networkflowmonitor_2023_04_19(&self) -> networkflowmonitor_2023_04_19::Networkflowmonitor_2023_04_19Service<'_> {
        networkflowmonitor_2023_04_19::Networkflowmonitor_2023_04_19Service::new(self)
    }
    /// Get shield_2016_06_02 service handler
    pub fn shield_2016_06_02(&self) -> shield_2016_06_02::Shield_2016_06_02Service<'_> {
        shield_2016_06_02::Shield_2016_06_02Service::new(self)
    }
    /// Get ssm_2014_11_06 service handler
    pub fn ssm_2014_11_06(&self) -> ssm_2014_11_06::Ssm_2014_11_06Service<'_> {
        ssm_2014_11_06::Ssm_2014_11_06Service::new(self)
    }
    /// Get arc_zonal_shift_2022_10_30 service handler
    pub fn arc_zonal_shift_2022_10_30(&self) -> arc_zonal_shift_2022_10_30::Arc_zonal_shift_2022_10_30Service<'_> {
        arc_zonal_shift_2022_10_30::Arc_zonal_shift_2022_10_30Service::new(self)
    }
    /// Get workspaces_thin_client_2023_08_22 service handler
    pub fn workspaces_thin_client_2023_08_22(&self) -> workspaces_thin_client_2023_08_22::Workspaces_thin_client_2023_08_22Service<'_> {
        workspaces_thin_client_2023_08_22::Workspaces_thin_client_2023_08_22Service::new(self)
    }
    /// Get partnercentral_selling_2022_07_26 service handler
    pub fn partnercentral_selling_2022_07_26(&self) -> partnercentral_selling_2022_07_26::Partnercentral_selling_2022_07_26Service<'_> {
        partnercentral_selling_2022_07_26::Partnercentral_selling_2022_07_26Service::new(self)
    }
    /// Get geo_places_2020_11_19 service handler
    pub fn geo_places_2020_11_19(&self) -> geo_places_2020_11_19::Geo_places_2020_11_19Service<'_> {
        geo_places_2020_11_19::Geo_places_2020_11_19Service::new(self)
    }
    /// Get signer_2017_08_25 service handler
    pub fn signer_2017_08_25(&self) -> signer_2017_08_25::Signer_2017_08_25Service<'_> {
        signer_2017_08_25::Signer_2017_08_25Service::new(self)
    }
    /// Get marketplace_reporting_2018_05_10 service handler
    pub fn marketplace_reporting_2018_05_10(&self) -> marketplace_reporting_2018_05_10::Marketplace_reporting_2018_05_10Service<'_> {
        marketplace_reporting_2018_05_10::Marketplace_reporting_2018_05_10Service::new(self)
    }
    /// Get lakeformation_2017_03_31 service handler
    pub fn lakeformation_2017_03_31(&self) -> lakeformation_2017_03_31::Lakeformation_2017_03_31Service<'_> {
        lakeformation_2017_03_31::Lakeformation_2017_03_31Service::new(self)
    }
    /// Get pcs_2023_02_10 service handler
    pub fn pcs_2023_02_10(&self) -> pcs_2023_02_10::Pcs_2023_02_10Service<'_> {
        pcs_2023_02_10::Pcs_2023_02_10Service::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get mediaconnect_2018_11_14 service handler
    pub fn mediaconnect_2018_11_14(&self) -> mediaconnect_2018_11_14::Mediaconnect_2018_11_14Service<'_> {
        mediaconnect_2018_11_14::Mediaconnect_2018_11_14Service::new(self)
    }
    /// Get secrets_manager_2017_10_17 service handler
    pub fn secrets_manager_2017_10_17(&self) -> secrets_manager_2017_10_17::Secrets_manager_2017_10_17Service<'_> {
        secrets_manager_2017_10_17::Secrets_manager_2017_10_17Service::new(self)
    }
    /// Get mwaa_2020_07_01 service handler
    pub fn mwaa_2020_07_01(&self) -> mwaa_2020_07_01::Mwaa_2020_07_01Service<'_> {
        mwaa_2020_07_01::Mwaa_2020_07_01Service::new(self)
    }
    /// Get kms_2014_11_01 service handler
    pub fn kms_2014_11_01(&self) -> kms_2014_11_01::Kms_2014_11_01Service<'_> {
        kms_2014_11_01::Kms_2014_11_01Service::new(self)
    }
    /// Get quicksight_2018_04_01 service handler
    pub fn quicksight_2018_04_01(&self) -> quicksight_2018_04_01::Quicksight_2018_04_01Service<'_> {
        quicksight_2018_04_01::Quicksight_2018_04_01Service::new(self)
    }
    /// Get workmail_2017_10_01 service handler
    pub fn workmail_2017_10_01(&self) -> workmail_2017_10_01::Workmail_2017_10_01Service<'_> {
        workmail_2017_10_01::Workmail_2017_10_01Service::new(self)
    }
    /// Get eventbridge_2015_10_07 service handler
    pub fn eventbridge_2015_10_07(&self) -> eventbridge_2015_10_07::Eventbridge_2015_10_07Service<'_> {
        eventbridge_2015_10_07::Eventbridge_2015_10_07Service::new(self)
    }
    /// Get frauddetector_2019_11_15 service handler
    pub fn frauddetector_2019_11_15(&self) -> frauddetector_2019_11_15::Frauddetector_2019_11_15Service<'_> {
        frauddetector_2019_11_15::Frauddetector_2019_11_15Service::new(self)
    }
    /// Get cloudwatch_events_2015_10_07 service handler
    pub fn cloudwatch_events_2015_10_07(&self) -> cloudwatch_events_2015_10_07::Cloudwatch_events_2015_10_07Service<'_> {
        cloudwatch_events_2015_10_07::Cloudwatch_events_2015_10_07Service::new(self)
    }
    /// Get cloudhsm service handler
    pub fn cloudhsm(&self) -> cloudhsm::CloudhsmService<'_> {
        cloudhsm::CloudhsmService::new(self)
    }
    /// Get cloudsearch_domain_2013_01_01 service handler
    pub fn cloudsearch_domain_2013_01_01(&self) -> cloudsearch_domain_2013_01_01::Cloudsearch_domain_2013_01_01Service<'_> {
        cloudsearch_domain_2013_01_01::Cloudsearch_domain_2013_01_01Service::new(self)
    }
    /// Get lookoutequipment_2020_12_15 service handler
    pub fn lookoutequipment_2020_12_15(&self) -> lookoutequipment_2020_12_15::Lookoutequipment_2020_12_15Service<'_> {
        lookoutequipment_2020_12_15::Lookoutequipment_2020_12_15Service::new(self)
    }
    /// Get iot_events_data_2018_10_23 service handler
    pub fn iot_events_data_2018_10_23(&self) -> iot_events_data_2018_10_23::Iot_events_data_2018_10_23Service<'_> {
        iot_events_data_2018_10_23::Iot_events_data_2018_10_23Service::new(self)
    }
    /// Get securitylake_2018_05_10 service handler
    pub fn securitylake_2018_05_10(&self) -> securitylake_2018_05_10::Securitylake_2018_05_10Service<'_> {
        securitylake_2018_05_10::Securitylake_2018_05_10Service::new(self)
    }
    /// Get cloudwatch_2010_08_01 service handler
    pub fn cloudwatch_2010_08_01(&self) -> cloudwatch_2010_08_01::Cloudwatch_2010_08_01Service<'_> {
        cloudwatch_2010_08_01::Cloudwatch_2010_08_01Service::new(self)
    }
    /// Get glue_2017_03_31 service handler
    pub fn glue_2017_03_31(&self) -> glue_2017_03_31::Glue_2017_03_31Service<'_> {
        glue_2017_03_31::Glue_2017_03_31Service::new(self)
    }
    /// Get application_auto_scaling_2016_02_06 service handler
    pub fn application_auto_scaling_2016_02_06(&self) -> application_auto_scaling_2016_02_06::Application_auto_scaling_2016_02_06Service<'_> {
        application_auto_scaling_2016_02_06::Application_auto_scaling_2016_02_06Service::new(self)
    }
    /// Get personalize_events_2018_03_22 service handler
    pub fn personalize_events_2018_03_22(&self) -> personalize_events_2018_03_22::Personalize_events_2018_03_22Service<'_> {
        personalize_events_2018_03_22::Personalize_events_2018_03_22Service::new(self)
    }
    /// Get voice_id_2021_09_27 service handler
    pub fn voice_id_2021_09_27(&self) -> voice_id_2021_09_27::Voice_id_2021_09_27Service<'_> {
        voice_id_2021_09_27::Voice_id_2021_09_27Service::new(self)
    }
    /// Get s3_2006_03_01 service handler
    pub fn s3_2006_03_01(&self) -> s3_2006_03_01::S3_2006_03_01Service<'_> {
        s3_2006_03_01::S3_2006_03_01Service::new(self)
    }
    /// Get appintegrations_2020_07_29 service handler
    pub fn appintegrations_2020_07_29(&self) -> appintegrations_2020_07_29::Appintegrations_2020_07_29Service<'_> {
        appintegrations_2020_07_29::Appintegrations_2020_07_29Service::new(self)
    }
    /// Get lex_runtime_service_2016_11_28 service handler
    pub fn lex_runtime_service_2016_11_28(&self) -> lex_runtime_service_2016_11_28::Lex_runtime_service_2016_11_28Service<'_> {
        lex_runtime_service_2016_11_28::Lex_runtime_service_2016_11_28Service::new(self)
    }
    /// Get ssm_contacts_2021_05_03 service handler
    pub fn ssm_contacts_2021_05_03(&self) -> ssm_contacts_2021_05_03::Ssm_contacts_2021_05_03Service<'_> {
        ssm_contacts_2021_05_03::Ssm_contacts_2021_05_03Service::new(self)
    }
    /// Get chime_sdk_meetings_2021_07_15 service handler
    pub fn chime_sdk_meetings_2021_07_15(&self) -> chime_sdk_meetings_2021_07_15::Chime_sdk_meetings_2021_07_15Service<'_> {
        chime_sdk_meetings_2021_07_15::Chime_sdk_meetings_2021_07_15Service::new(self)
    }
    /// Get sesv2_2019_09_27 service handler
    pub fn sesv2_2019_09_27(&self) -> sesv2_2019_09_27::Sesv2_2019_09_27Service<'_> {
        sesv2_2019_09_27::Sesv2_2019_09_27Service::new(self)
    }
    /// Get bedrock_agentcore_control_2023_06_05 service handler
    pub fn bedrock_agentcore_control_2023_06_05(&self) -> bedrock_agentcore_control_2023_06_05::Bedrock_agentcore_control_2023_06_05Service<'_> {
        bedrock_agentcore_control_2023_06_05::Bedrock_agentcore_control_2023_06_05Service::new(self)
    }
    /// Get emr_2009_03_31 service handler
    pub fn emr_2009_03_31(&self) -> emr_2009_03_31::Emr_2009_03_31Service<'_> {
        emr_2009_03_31::Emr_2009_03_31Service::new(self)
    }
    /// Get controltower_2018_05_10 service handler
    pub fn controltower_2018_05_10(&self) -> controltower_2018_05_10::Controltower_2018_05_10Service<'_> {
        controltower_2018_05_10::Controltower_2018_05_10Service::new(self)
    }
    /// Get resource_explorer_2_2022_07_28 service handler
    pub fn resource_explorer_2_2022_07_28(&self) -> resource_explorer_2_2022_07_28::Resource_explorer_2_2022_07_28Service<'_> {
        resource_explorer_2_2022_07_28::Resource_explorer_2_2022_07_28Service::new(self)
    }
    /// Get s3_control_2018_08_20 service handler
    pub fn s3_control_2018_08_20(&self) -> s3_control_2018_08_20::S3_control_2018_08_20Service<'_> {
        s3_control_2018_08_20::S3_control_2018_08_20Service::new(self)
    }
    /// Get rtbfabric_2023_05_15 service handler
    pub fn rtbfabric_2023_05_15(&self) -> rtbfabric_2023_05_15::Rtbfabric_2023_05_15Service<'_> {
        rtbfabric_2023_05_15::Rtbfabric_2023_05_15Service::new(self)
    }
    /// Get personalize_2018_05_22 service handler
    pub fn personalize_2018_05_22(&self) -> personalize_2018_05_22::Personalize_2018_05_22Service<'_> {
        personalize_2018_05_22::Personalize_2018_05_22Service::new(self)
    }
    /// Get marketplace_entitlement_service_2017_01_11 service handler
    pub fn marketplace_entitlement_service_2017_01_11(&self) -> marketplace_entitlement_service_2017_01_11::Marketplace_entitlement_service_2017_01_11Service<'_> {
        marketplace_entitlement_service_2017_01_11::Marketplace_entitlement_service_2017_01_11Service::new(self)
    }
    /// Get directory_service_data_2023_05_31 service handler
    pub fn directory_service_data_2023_05_31(&self) -> directory_service_data_2023_05_31::Directory_service_data_2023_05_31Service<'_> {
        directory_service_data_2023_05_31::Directory_service_data_2023_05_31Service::new(self)
    }
    /// Get ssm_sap_2018_05_10 service handler
    pub fn ssm_sap_2018_05_10(&self) -> ssm_sap_2018_05_10::Ssm_sap_2018_05_10Service<'_> {
        ssm_sap_2018_05_10::Ssm_sap_2018_05_10Service::new(self)
    }
    /// Get outposts_2019_12_03 service handler
    pub fn outposts_2019_12_03(&self) -> outposts_2019_12_03::Outposts_2019_12_03Service<'_> {
        outposts_2019_12_03::Outposts_2019_12_03Service::new(self)
    }
    /// Get workdocs_2016_05_01 service handler
    pub fn workdocs_2016_05_01(&self) -> workdocs_2016_05_01::Workdocs_2016_05_01Service<'_> {
        workdocs_2016_05_01::Workdocs_2016_05_01Service::new(self)
    }
    /// Get networkmanager_2019_07_05 service handler
    pub fn networkmanager_2019_07_05(&self) -> networkmanager_2019_07_05::Networkmanager_2019_07_05Service<'_> {
        networkmanager_2019_07_05::Networkmanager_2019_07_05Service::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get socialmessaging_2024_01_01 service handler
    pub fn socialmessaging_2024_01_01(&self) -> socialmessaging_2024_01_01::Socialmessaging_2024_01_01Service<'_> {
        socialmessaging_2024_01_01::Socialmessaging_2024_01_01Service::new(self)
    }
    /// Get bedrock_agentcore_2024_02_28 service handler
    pub fn bedrock_agentcore_2024_02_28(&self) -> bedrock_agentcore_2024_02_28::Bedrock_agentcore_2024_02_28Service<'_> {
        bedrock_agentcore_2024_02_28::Bedrock_agentcore_2024_02_28Service::new(self)
    }
    /// Get omics_2022_11_28 service handler
    pub fn omics_2022_11_28(&self) -> omics_2022_11_28::Omics_2022_11_28Service<'_> {
        omics_2022_11_28::Omics_2022_11_28Service::new(self)
    }
    /// Get bcm_recommended_actions_2024_11_14 service handler
    pub fn bcm_recommended_actions_2024_11_14(&self) -> bcm_recommended_actions_2024_11_14::Bcm_recommended_actions_2024_11_14Service<'_> {
        bcm_recommended_actions_2024_11_14::Bcm_recommended_actions_2024_11_14Service::new(self)
    }
    /// Get mediapackage_2017_10_12 service handler
    pub fn mediapackage_2017_10_12(&self) -> mediapackage_2017_10_12::Mediapackage_2017_10_12Service<'_> {
        mediapackage_2017_10_12::Mediapackage_2017_10_12Service::new(self)
    }
    /// Get sagemaker_featurestore_runtime_2020_07_01 service handler
    pub fn sagemaker_featurestore_runtime_2020_07_01(&self) -> sagemaker_featurestore_runtime_2020_07_01::Sagemaker_featurestore_runtime_2020_07_01Service<'_> {
        sagemaker_featurestore_runtime_2020_07_01::Sagemaker_featurestore_runtime_2020_07_01Service::new(self)
    }
    /// Get config_service_2014_11_12 service handler
    pub fn config_service_2014_11_12(&self) -> config_service_2014_11_12::Config_service_2014_11_12Service<'_> {
        config_service_2014_11_12::Config_service_2014_11_12Service::new(self)
    }
    /// Get cloudwatch_logs_2014_03_28 service handler
    pub fn cloudwatch_logs_2014_03_28(&self) -> cloudwatch_logs_2014_03_28::Cloudwatch_logs_2014_03_28Service<'_> {
        cloudwatch_logs_2014_03_28::Cloudwatch_logs_2014_03_28Service::new(self)
    }
    /// Get medialive_2017_10_14 service handler
    pub fn medialive_2017_10_14(&self) -> medialive_2017_10_14::Medialive_2017_10_14Service<'_> {
        medialive_2017_10_14::Medialive_2017_10_14Service::new(self)
    }
    /// Get backup_gateway_2021_01_01 service handler
    pub fn backup_gateway_2021_01_01(&self) -> backup_gateway_2021_01_01::Backup_gateway_2021_01_01Service<'_> {
        backup_gateway_2021_01_01::Backup_gateway_2021_01_01Service::new(self)
    }
    /// Get connect_contact_lens_2020_08_21 service handler
    pub fn connect_contact_lens_2020_08_21(&self) -> connect_contact_lens_2020_08_21::Connect_contact_lens_2020_08_21Service<'_> {
        connect_contact_lens_2020_08_21::Connect_contact_lens_2020_08_21Service::new(self)
    }
    /// Get mediaconvert_2017_08_29 service handler
    pub fn mediaconvert_2017_08_29(&self) -> mediaconvert_2017_08_29::Mediaconvert_2017_08_29Service<'_> {
        mediaconvert_2017_08_29::Mediaconvert_2017_08_29Service::new(self)
    }
    /// Get sns_2010_03_31 service handler
    pub fn sns_2010_03_31(&self) -> sns_2010_03_31::Sns_2010_03_31Service<'_> {
        sns_2010_03_31::Sns_2010_03_31Service::new(self)
    }
    /// Get evs_2023_07_27 service handler
    pub fn evs_2023_07_27(&self) -> evs_2023_07_27::Evs_2023_07_27Service<'_> {
        evs_2023_07_27::Evs_2023_07_27Service::new(self)
    }
    /// Get datasync_2018_11_09 service handler
    pub fn datasync_2018_11_09(&self) -> datasync_2018_11_09::Datasync_2018_11_09Service<'_> {
        datasync_2018_11_09::Datasync_2018_11_09Service::new(self)
    }
    /// Get greengrassv2_2020_11_30 service handler
    pub fn greengrassv2_2020_11_30(&self) -> greengrassv2_2020_11_30::Greengrassv2_2020_11_30Service<'_> {
        greengrassv2_2020_11_30::Greengrassv2_2020_11_30Service::new(self)
    }
    /// Get cleanroomsml_2023_09_06 service handler
    pub fn cleanroomsml_2023_09_06(&self) -> cleanroomsml_2023_09_06::Cleanroomsml_2023_09_06Service<'_> {
        cleanroomsml_2023_09_06::Cleanroomsml_2023_09_06Service::new(self)
    }
    /// Get neptunedata_2023_08_01 service handler
    pub fn neptunedata_2023_08_01(&self) -> neptunedata_2023_08_01::Neptunedata_2023_08_01Service<'_> {
        neptunedata_2023_08_01::Neptunedata_2023_08_01Service::new(self)
    }
    /// Get acm_pca_2017_08_22 service handler
    pub fn acm_pca_2017_08_22(&self) -> acm_pca_2017_08_22::Acm_pca_2017_08_22Service<'_> {
        acm_pca_2017_08_22::Acm_pca_2017_08_22Service::new(self)
    }
    /// Get service_catalog_2015_12_10 service handler
    pub fn service_catalog_2015_12_10(&self) -> service_catalog_2015_12_10::Service_catalog_2015_12_10Service<'_> {
        service_catalog_2015_12_10::Service_catalog_2015_12_10Service::new(self)
    }
    /// Get b2bi_2022_06_23 service handler
    pub fn b2bi_2022_06_23(&self) -> b2bi_2022_06_23::B2bi_2022_06_23Service<'_> {
        b2bi_2022_06_23::B2bi_2022_06_23Service::new(self)
    }
    /// Get iotanalytics_2017_11_27 service handler
    pub fn iotanalytics_2017_11_27(&self) -> iotanalytics_2017_11_27::Iotanalytics_2017_11_27Service<'_> {
        iotanalytics_2017_11_27::Iotanalytics_2017_11_27Service::new(self)
    }
    /// Get lex_model_building_service_2017_04_19 service handler
    pub fn lex_model_building_service_2017_04_19(&self) -> lex_model_building_service_2017_04_19::Lex_model_building_service_2017_04_19Service<'_> {
        lex_model_building_service_2017_04_19::Lex_model_building_service_2017_04_19Service::new(self)
    }
    /// Get inspector2_2020_06_08 service handler
    pub fn inspector2_2020_06_08(&self) -> inspector2_2020_06_08::Inspector2_2020_06_08Service<'_> {
        inspector2_2020_06_08::Inspector2_2020_06_08Service::new(self)
    }
    /// Get groundstation_2019_05_23 service handler
    pub fn groundstation_2019_05_23(&self) -> groundstation_2019_05_23::Groundstation_2019_05_23Service<'_> {
        groundstation_2019_05_23::Groundstation_2019_05_23Service::new(self)
    }
    /// Get ecr_public_2020_10_30 service handler
    pub fn ecr_public_2020_10_30(&self) -> ecr_public_2020_10_30::Ecr_public_2020_10_30Service<'_> {
        ecr_public_2020_10_30::Ecr_public_2020_10_30Service::new(self)
    }
    /// Get fis_2020_12_01 service handler
    pub fn fis_2020_12_01(&self) -> fis_2020_12_01::Fis_2020_12_01Service<'_> {
        fis_2020_12_01::Fis_2020_12_01Service::new(self)
    }
    /// Get proton_2020_07_20 service handler
    pub fn proton_2020_07_20(&self) -> proton_2020_07_20::Proton_2020_07_20Service<'_> {
        proton_2020_07_20::Proton_2020_07_20Service::new(self)
    }
    /// Get api_gateway_2015_07_09 service handler
    pub fn api_gateway_2015_07_09(&self) -> api_gateway_2015_07_09::Api_gateway_2015_07_09Service<'_> {
        api_gateway_2015_07_09::Api_gateway_2015_07_09Service::new(self)
    }
    /// Get cloudhsm_2014_05_30 service handler
    pub fn cloudhsm_2014_05_30(&self) -> cloudhsm_2014_05_30::Cloudhsm_2014_05_30Service<'_> {
        cloudhsm_2014_05_30::Cloudhsm_2014_05_30Service::new(self)
    }
    /// Get mpa_2022_07_26 service handler
    pub fn mpa_2022_07_26(&self) -> mpa_2022_07_26::Mpa_2022_07_26Service<'_> {
        mpa_2022_07_26::Mpa_2022_07_26Service::new(self)
    }
    /// Get osis_2022_01_01 service handler
    pub fn osis_2022_01_01(&self) -> osis_2022_01_01::Osis_2022_01_01Service<'_> {
        osis_2022_01_01::Osis_2022_01_01Service::new(self)
    }
    /// Get memorydb_2021_01_01 service handler
    pub fn memorydb_2021_01_01(&self) -> memorydb_2021_01_01::Memorydb_2021_01_01Service<'_> {
        memorydb_2021_01_01::Memorydb_2021_01_01Service::new(self)
    }
    /// Get inspector_2016_02_16 service handler
    pub fn inspector_2016_02_16(&self) -> inspector_2016_02_16::Inspector_2016_02_16Service<'_> {
        inspector_2016_02_16::Inspector_2016_02_16Service::new(self)
    }
    /// Get translate_2017_07_01 service handler
    pub fn translate_2017_07_01(&self) -> translate_2017_07_01::Translate_2017_07_01Service<'_> {
        translate_2017_07_01::Translate_2017_07_01Service::new(self)
    }
    /// Get mailmanager_2023_10_17 service handler
    pub fn mailmanager_2023_10_17(&self) -> mailmanager_2023_10_17::Mailmanager_2023_10_17Service<'_> {
        mailmanager_2023_10_17::Mailmanager_2023_10_17Service::new(self)
    }
    /// Get neptune_graph_2023_11_29 service handler
    pub fn neptune_graph_2023_11_29(&self) -> neptune_graph_2023_11_29::Neptune_graph_2023_11_29Service<'_> {
        neptune_graph_2023_11_29::Neptune_graph_2023_11_29Service::new(self)
    }
    /// Get chatbot_2017_10_11 service handler
    pub fn chatbot_2017_10_11(&self) -> chatbot_2017_10_11::Chatbot_2017_10_11Service<'_> {
        chatbot_2017_10_11::Chatbot_2017_10_11Service::new(self)
    }
    /// Get fms_2018_01_01 service handler
    pub fn fms_2018_01_01(&self) -> fms_2018_01_01::Fms_2018_01_01Service<'_> {
        fms_2018_01_01::Fms_2018_01_01Service::new(self)
    }
    /// Get qapps_2023_11_27 service handler
    pub fn qapps_2023_11_27(&self) -> qapps_2023_11_27::Qapps_2023_11_27Service<'_> {
        qapps_2023_11_27::Qapps_2023_11_27Service::new(self)
    }
    /// Get customer_profiles_2020_08_15 service handler
    pub fn customer_profiles_2020_08_15(&self) -> customer_profiles_2020_08_15::Customer_profiles_2020_08_15Service<'_> {
        customer_profiles_2020_08_15::Customer_profiles_2020_08_15Service::new(self)
    }
    /// Get geo_maps_2020_11_19 service handler
    pub fn geo_maps_2020_11_19(&self) -> geo_maps_2020_11_19::Geo_maps_2020_11_19Service<'_> {
        geo_maps_2020_11_19::Geo_maps_2020_11_19Service::new(self)
    }
    /// Get route_53_domains_2014_05_15 service handler
    pub fn route_53_domains_2014_05_15(&self) -> route_53_domains_2014_05_15::Route_53_domains_2014_05_15Service<'_> {
        route_53_domains_2014_05_15::Route_53_domains_2014_05_15Service::new(self)
    }
    /// Get service_catalog_appregistry_2020_06_24 service handler
    pub fn service_catalog_appregistry_2020_06_24(&self) -> service_catalog_appregistry_2020_06_24::Service_catalog_appregistry_2020_06_24Service<'_> {
        service_catalog_appregistry_2020_06_24::Service_catalog_appregistry_2020_06_24Service::new(self)
    }
    /// Get qbusiness_2023_11_27 service handler
    pub fn qbusiness_2023_11_27(&self) -> qbusiness_2023_11_27::Qbusiness_2023_11_27Service<'_> {
        qbusiness_2023_11_27::Qbusiness_2023_11_27Service::new(self)
    }
    /// Get synthetics_2017_10_11 service handler
    pub fn synthetics_2017_10_11(&self) -> synthetics_2017_10_11::Synthetics_2017_10_11Service<'_> {
        synthetics_2017_10_11::Synthetics_2017_10_11Service::new(self)
    }
    /// Get codecatalyst_2022_09_28 service handler
    pub fn codecatalyst_2022_09_28(&self) -> codecatalyst_2022_09_28::Codecatalyst_2022_09_28Service<'_> {
        codecatalyst_2022_09_28::Codecatalyst_2022_09_28Service::new(self)
    }
    /// Get keyspacesstreams_2024_09_09 service handler
    pub fn keyspacesstreams_2024_09_09(&self) -> keyspacesstreams_2024_09_09::Keyspacesstreams_2024_09_09Service<'_> {
        keyspacesstreams_2024_09_09::Keyspacesstreams_2024_09_09Service::new(self)
    }
    /// Get storage_gateway_2013_06_30 service handler
    pub fn storage_gateway_2013_06_30(&self) -> storage_gateway_2013_06_30::Storage_gateway_2013_06_30Service<'_> {
        storage_gateway_2013_06_30::Storage_gateway_2013_06_30Service::new(self)
    }
    /// Get elastic_transcoder_2012_09_25 service handler
    pub fn elastic_transcoder_2012_09_25(&self) -> elastic_transcoder_2012_09_25::Elastic_transcoder_2012_09_25Service<'_> {
        elastic_transcoder_2012_09_25::Elastic_transcoder_2012_09_25Service::new(self)
    }
    /// Get bcm_data_exports_2023_11_26 service handler
    pub fn bcm_data_exports_2023_11_26(&self) -> bcm_data_exports_2023_11_26::Bcm_data_exports_2023_11_26Service<'_> {
        bcm_data_exports_2023_11_26::Bcm_data_exports_2023_11_26Service::new(self)
    }
    /// Get iotsecuretunneling_2018_10_05 service handler
    pub fn iotsecuretunneling_2018_10_05(&self) -> iotsecuretunneling_2018_10_05::Iotsecuretunneling_2018_10_05Service<'_> {
        iotsecuretunneling_2018_10_05::Iotsecuretunneling_2018_10_05Service::new(self)
    }
    /// Get cloudfront_2020_05_31 service handler
    pub fn cloudfront_2020_05_31(&self) -> cloudfront_2020_05_31::Cloudfront_2020_05_31Service<'_> {
        cloudfront_2020_05_31::Cloudfront_2020_05_31Service::new(self)
    }
    /// Get bedrock_data_automation_2023_07_26 service handler
    pub fn bedrock_data_automation_2023_07_26(&self) -> bedrock_data_automation_2023_07_26::Bedrock_data_automation_2023_07_26Service<'_> {
        bedrock_data_automation_2023_07_26::Bedrock_data_automation_2023_07_26Service::new(self)
    }
    /// Get location_2020_11_19 service handler
    pub fn location_2020_11_19(&self) -> location_2020_11_19::Location_2020_11_19Service<'_> {
        location_2020_11_19::Location_2020_11_19Service::new(self)
    }
    /// Get wafv2_2019_07_29 service handler
    pub fn wafv2_2019_07_29(&self) -> wafv2_2019_07_29::Wafv2_2019_07_29Service<'_> {
        wafv2_2019_07_29::Wafv2_2019_07_29Service::new(self)
    }
    /// Get opensearch_2021_01_01 service handler
    pub fn opensearch_2021_01_01(&self) -> opensearch_2021_01_01::Opensearch_2021_01_01Service<'_> {
        opensearch_2021_01_01::Opensearch_2021_01_01Service::new(self)
    }
    /// Get iotthingsgraph_2018_09_06 service handler
    pub fn iotthingsgraph_2018_09_06(&self) -> iotthingsgraph_2018_09_06::Iotthingsgraph_2018_09_06Service<'_> {
        iotthingsgraph_2018_09_06::Iotthingsgraph_2018_09_06Service::new(self)
    }
    /// Get security_ir_2018_05_10 service handler
    pub fn security_ir_2018_05_10(&self) -> security_ir_2018_05_10::Security_ir_2018_05_10Service<'_> {
        security_ir_2018_05_10::Security_ir_2018_05_10Service::new(self)
    }
    /// Get repostspace_2022_05_13 service handler
    pub fn repostspace_2022_05_13(&self) -> repostspace_2022_05_13::Repostspace_2022_05_13Service<'_> {
        repostspace_2022_05_13::Repostspace_2022_05_13Service::new(self)
    }
    /// Get health_2016_08_04 service handler
    pub fn health_2016_08_04(&self) -> health_2016_08_04::Health_2016_08_04Service<'_> {
        health_2016_08_04::Health_2016_08_04Service::new(self)
    }
    /// Get workmailmessageflow_2019_05_01 service handler
    pub fn workmailmessageflow_2019_05_01(&self) -> workmailmessageflow_2019_05_01::Workmailmessageflow_2019_05_01Service<'_> {
        workmailmessageflow_2019_05_01::Workmailmessageflow_2019_05_01Service::new(self)
    }
    /// Get mediastore_data_2017_09_01 service handler
    pub fn mediastore_data_2017_09_01(&self) -> mediastore_data_2017_09_01::Mediastore_data_2017_09_01Service<'_> {
        mediastore_data_2017_09_01::Mediastore_data_2017_09_01Service::new(self)
    }
    /// Get ec2_instance_connect_2018_04_02 service handler
    pub fn ec2_instance_connect_2018_04_02(&self) -> ec2_instance_connect_2018_04_02::Ec2_instance_connect_2018_04_02Service<'_> {
        ec2_instance_connect_2018_04_02::Ec2_instance_connect_2018_04_02Service::new(self)
    }
    /// Get comprehendmedical_2018_10_30 service handler
    pub fn comprehendmedical_2018_10_30(&self) -> comprehendmedical_2018_10_30::Comprehendmedical_2018_10_30Service<'_> {
        comprehendmedical_2018_10_30::Comprehendmedical_2018_10_30Service::new(self)
    }
    /// Get iotfleetwise_2021_06_17 service handler
    pub fn iotfleetwise_2021_06_17(&self) -> iotfleetwise_2021_06_17::Iotfleetwise_2021_06_17Service<'_> {
        iotfleetwise_2021_06_17::Iotfleetwise_2021_06_17Service::new(self)
    }
    /// Get route53profiles_2018_05_10 service handler
    pub fn route53profiles_2018_05_10(&self) -> route53profiles_2018_05_10::Route53profiles_2018_05_10Service<'_> {
        route53profiles_2018_05_10::Route53profiles_2018_05_10Service::new(self)
    }
    /// Get application_signals_2024_04_15 service handler
    pub fn application_signals_2024_04_15(&self) -> application_signals_2024_04_15::Application_signals_2024_04_15Service<'_> {
        application_signals_2024_04_15::Application_signals_2024_04_15Service::new(self)
    }
    /// Get resource_groups_tagging_api_2017_01_26 service handler
    pub fn resource_groups_tagging_api_2017_01_26(&self) -> resource_groups_tagging_api_2017_01_26::Resource_groups_tagging_api_2017_01_26Service<'_> {
        resource_groups_tagging_api_2017_01_26::Resource_groups_tagging_api_2017_01_26Service::new(self)
    }
    /// Get accessanalyzer_2019_11_01 service handler
    pub fn accessanalyzer_2019_11_01(&self) -> accessanalyzer_2019_11_01::Accessanalyzer_2019_11_01Service<'_> {
        accessanalyzer_2019_11_01::Accessanalyzer_2019_11_01Service::new(self)
    }
    /// Get glacier_2012_06_01 service handler
    pub fn glacier_2012_06_01(&self) -> glacier_2012_06_01::Glacier_2012_06_01Service<'_> {
        glacier_2012_06_01::Glacier_2012_06_01Service::new(self)
    }
    /// Get lightsail_2016_11_28 service handler
    pub fn lightsail_2016_11_28(&self) -> lightsail_2016_11_28::Lightsail_2016_11_28Service<'_> {
        lightsail_2016_11_28::Lightsail_2016_11_28Service::new(self)
    }
    /// Get rum_2018_05_10 service handler
    pub fn rum_2018_05_10(&self) -> rum_2018_05_10::Rum_2018_05_10Service<'_> {
        rum_2018_05_10::Rum_2018_05_10Service::new(self)
    }
    /// Get direct_connect_2012_10_25 service handler
    pub fn direct_connect_2012_10_25(&self) -> direct_connect_2012_10_25::Direct_connect_2012_10_25Service<'_> {
        direct_connect_2012_10_25::Direct_connect_2012_10_25Service::new(self)
    }
    /// Get elastic_beanstalk_2010_12_01 service handler
    pub fn elastic_beanstalk_2010_12_01(&self) -> elastic_beanstalk_2010_12_01::Elastic_beanstalk_2010_12_01Service<'_> {
        elastic_beanstalk_2010_12_01::Elastic_beanstalk_2010_12_01Service::new(self)
    }
    /// Get imagebuilder_2019_12_02 service handler
    pub fn imagebuilder_2019_12_02(&self) -> imagebuilder_2019_12_02::Imagebuilder_2019_12_02Service<'_> {
        imagebuilder_2019_12_02::Imagebuilder_2019_12_02Service::new(self)
    }
    /// Get simspaceweaver_2022_10_28 service handler
    pub fn simspaceweaver_2022_10_28(&self) -> simspaceweaver_2022_10_28::Simspaceweaver_2022_10_28Service<'_> {
        simspaceweaver_2022_10_28::Simspaceweaver_2022_10_28Service::new(self)
    }
    /// Get iot_jobs_data_plane_2017_09_29 service handler
    pub fn iot_jobs_data_plane_2017_09_29(&self) -> iot_jobs_data_plane_2017_09_29::Iot_jobs_data_plane_2017_09_29Service<'_> {
        iot_jobs_data_plane_2017_09_29::Iot_jobs_data_plane_2017_09_29Service::new(self)
    }
    /// Get freetier_2023_09_07 service handler
    pub fn freetier_2023_09_07(&self) -> freetier_2023_09_07::Freetier_2023_09_07Service<'_> {
        freetier_2023_09_07::Freetier_2023_09_07Service::new(self)
    }
    /// Get bedrock_agent_runtime_2023_07_26 service handler
    pub fn bedrock_agent_runtime_2023_07_26(&self) -> bedrock_agent_runtime_2023_07_26::Bedrock_agent_runtime_2023_07_26Service<'_> {
        bedrock_agent_runtime_2023_07_26::Bedrock_agent_runtime_2023_07_26Service::new(self)
    }
    /// Get neptune_2014_10_31 service handler
    pub fn neptune_2014_10_31(&self) -> neptune_2014_10_31::Neptune_2014_10_31Service<'_> {
        neptune_2014_10_31::Neptune_2014_10_31Service::new(self)
    }
    /// Get transfer_2018_11_05 service handler
    pub fn transfer_2018_11_05(&self) -> transfer_2018_11_05::Transfer_2018_11_05Service<'_> {
        transfer_2018_11_05::Transfer_2018_11_05Service::new(self)
    }
    /// Get deadline_2023_10_12 service handler
    pub fn deadline_2023_10_12(&self) -> deadline_2023_10_12::Deadline_2023_10_12Service<'_> {
        deadline_2023_10_12::Deadline_2023_10_12Service::new(self)
    }
    /// Get braket_2019_09_01 service handler
    pub fn braket_2019_09_01(&self) -> braket_2019_09_01::Braket_2019_09_01Service<'_> {
        braket_2019_09_01::Braket_2019_09_01Service::new(self)
    }
    /// Get verifiedpermissions_2021_12_01 service handler
    pub fn verifiedpermissions_2021_12_01(&self) -> verifiedpermissions_2021_12_01::Verifiedpermissions_2021_12_01Service<'_> {
        verifiedpermissions_2021_12_01::Verifiedpermissions_2021_12_01Service::new(self)
    }
    /// Get scheduler_2021_06_30 service handler
    pub fn scheduler_2021_06_30(&self) -> scheduler_2021_06_30::Scheduler_2021_06_30Service<'_> {
        scheduler_2021_06_30::Scheduler_2021_06_30Service::new(self)
    }
    /// Get waf_regional_2016_11_28 service handler
    pub fn waf_regional_2016_11_28(&self) -> waf_regional_2016_11_28::Waf_regional_2016_11_28Service<'_> {
        waf_regional_2016_11_28::Waf_regional_2016_11_28Service::new(self)
    }
    /// Get marketplace_commerce_analytics_2015_07_01 service handler
    pub fn marketplace_commerce_analytics_2015_07_01(&self) -> marketplace_commerce_analytics_2015_07_01::Marketplace_commerce_analytics_2015_07_01Service<'_> {
        marketplace_commerce_analytics_2015_07_01::Marketplace_commerce_analytics_2015_07_01Service::new(self)
    }
    /// Get ecr_2015_09_21 service handler
    pub fn ecr_2015_09_21(&self) -> ecr_2015_09_21::Ecr_2015_09_21Service<'_> {
        ecr_2015_09_21::Ecr_2015_09_21Service::new(self)
    }
    /// Get cost_and_usage_report_service_2017_01_06 service handler
    pub fn cost_and_usage_report_service_2017_01_06(&self) -> cost_and_usage_report_service_2017_01_06::Cost_and_usage_report_service_2017_01_06Service<'_> {
        cost_and_usage_report_service_2017_01_06::Cost_and_usage_report_service_2017_01_06Service::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get dynamodb_2012_08_10 service handler
    pub fn dynamodb_2012_08_10(&self) -> dynamodb_2012_08_10::Dynamodb_2012_08_10Service<'_> {
        dynamodb_2012_08_10::Dynamodb_2012_08_10Service::new(self)
    }
    /// Get resiliencehub_2020_04_30 service handler
    pub fn resiliencehub_2020_04_30(&self) -> resiliencehub_2020_04_30::Resiliencehub_2020_04_30Service<'_> {
        resiliencehub_2020_04_30::Resiliencehub_2020_04_30Service::new(self)
    }
    /// Get macie2_2020_01_01 service handler
    pub fn macie2_2020_01_01(&self) -> macie2_2020_01_01::Macie2_2020_01_01Service<'_> {
        macie2_2020_01_01::Macie2_2020_01_01Service::new(self)
    }
    /// Get entityresolution_2018_05_10 service handler
    pub fn entityresolution_2018_05_10(&self) -> entityresolution_2018_05_10::Entityresolution_2018_05_10Service<'_> {
        entityresolution_2018_05_10::Entityresolution_2018_05_10Service::new(self)
    }
    /// Get s3outposts_2017_07_25 service handler
    pub fn s3outposts_2017_07_25(&self) -> s3outposts_2017_07_25::S3outposts_2017_07_25Service<'_> {
        s3outposts_2017_07_25::S3outposts_2017_07_25Service::new(self)
    }
    /// Get grafana_2020_08_18 service handler
    pub fn grafana_2020_08_18(&self) -> grafana_2020_08_18::Grafana_2020_08_18Service<'_> {
        grafana_2020_08_18::Grafana_2020_08_18Service::new(self)
    }
    /// Get kinesis_analytics_2015_08_14 service handler
    pub fn kinesis_analytics_2015_08_14(&self) -> kinesis_analytics_2015_08_14::Kinesis_analytics_2015_08_14Service<'_> {
        kinesis_analytics_2015_08_14::Kinesis_analytics_2015_08_14Service::new(self)
    }
    /// Get eks_auth_2023_11_26 service handler
    pub fn eks_auth_2023_11_26(&self) -> eks_auth_2023_11_26::Eks_auth_2023_11_26Service<'_> {
        eks_auth_2023_11_26::Eks_auth_2023_11_26Service::new(self)
    }
    /// Get rbin_2021_06_15 service handler
    pub fn rbin_2021_06_15(&self) -> rbin_2021_06_15::Rbin_2021_06_15Service<'_> {
        rbin_2021_06_15::Rbin_2021_06_15Service::new(self)
    }
    /// Get service_quotas_2019_06_24 service handler
    pub fn service_quotas_2019_06_24(&self) -> service_quotas_2019_06_24::Service_quotas_2019_06_24Service<'_> {
        service_quotas_2019_06_24::Service_quotas_2019_06_24Service::new(self)
    }
    /// Get ecs_2014_11_13 service handler
    pub fn ecs_2014_11_13(&self) -> ecs_2014_11_13::Ecs_2014_11_13Service<'_> {
        ecs_2014_11_13::Ecs_2014_11_13Service::new(self)
    }
    /// Get marketplace_metering_2016_01_14 service handler
    pub fn marketplace_metering_2016_01_14(&self) -> marketplace_metering_2016_01_14::Marketplace_metering_2016_01_14Service<'_> {
        marketplace_metering_2016_01_14::Marketplace_metering_2016_01_14Service::new(self)
    }
    /// Get timestream_write_2018_11_01 service handler
    pub fn timestream_write_2018_11_01(&self) -> timestream_write_2018_11_01::Timestream_write_2018_11_01Service<'_> {
        timestream_write_2018_11_01::Timestream_write_2018_11_01Service::new(self)
    }
    /// Get qconnect_2020_10_19 service handler
    pub fn qconnect_2020_10_19(&self) -> qconnect_2020_10_19::Qconnect_2020_10_19Service<'_> {
        qconnect_2020_10_19::Qconnect_2020_10_19Service::new(self)
    }
    /// Get cloudsearch_2013_01_01 service handler
    pub fn cloudsearch_2013_01_01(&self) -> cloudsearch_2013_01_01::Cloudsearch_2013_01_01Service<'_> {
        cloudsearch_2013_01_01::Cloudsearch_2013_01_01Service::new(self)
    }
    /// Get bcm_pricing_calculator_2024_06_19 service handler
    pub fn bcm_pricing_calculator_2024_06_19(&self) -> bcm_pricing_calculator_2024_06_19::Bcm_pricing_calculator_2024_06_19Service<'_> {
        bcm_pricing_calculator_2024_06_19::Bcm_pricing_calculator_2024_06_19Service::new(self)
    }
    /// Get appfabric_2023_05_19 service handler
    pub fn appfabric_2023_05_19(&self) -> appfabric_2023_05_19::Appfabric_2023_05_19Service<'_> {
        appfabric_2023_05_19::Appfabric_2023_05_19Service::new(self)
    }
    /// Get route53resolver_2018_04_01 service handler
    pub fn route53resolver_2018_04_01(&self) -> route53resolver_2018_04_01::Route53resolver_2018_04_01Service<'_> {
        route53resolver_2018_04_01::Route53resolver_2018_04_01Service::new(self)
    }
    /// Get marketplace_agreement_2020_03_01 service handler
    pub fn marketplace_agreement_2020_03_01(&self) -> marketplace_agreement_2020_03_01::Marketplace_agreement_2020_03_01Service<'_> {
        marketplace_agreement_2020_03_01::Marketplace_agreement_2020_03_01Service::new(self)
    }
    /// Get workspaces_2015_04_08 service handler
    pub fn workspaces_2015_04_08(&self) -> workspaces_2015_04_08::Workspaces_2015_04_08Service<'_> {
        workspaces_2015_04_08::Workspaces_2015_04_08Service::new(self)
    }
    /// Get gameliftstreams_2018_05_10 service handler
    pub fn gameliftstreams_2018_05_10(&self) -> gameliftstreams_2018_05_10::Gameliftstreams_2018_05_10Service<'_> {
        gameliftstreams_2018_05_10::Gameliftstreams_2018_05_10Service::new(self)
    }
    /// Get taxsettings_2018_05_10 service handler
    pub fn taxsettings_2018_05_10(&self) -> taxsettings_2018_05_10::Taxsettings_2018_05_10Service<'_> {
        taxsettings_2018_05_10::Taxsettings_2018_05_10Service::new(self)
    }
    /// Get pinpoint_sms service handler
    pub fn pinpoint_sms(&self) -> pinpoint_sms::Pinpoint_smsService<'_> {
        pinpoint_sms::Pinpoint_smsService::new(self)
    }
    /// Get fsx_2018_03_01 service handler
    pub fn fsx_2018_03_01(&self) -> fsx_2018_03_01::Fsx_2018_03_01Service<'_> {
        fsx_2018_03_01::Fsx_2018_03_01Service::new(self)
    }
    /// Get codepipeline_2015_07_09 service handler
    pub fn codepipeline_2015_07_09(&self) -> codepipeline_2015_07_09::Codepipeline_2015_07_09Service<'_> {
        codepipeline_2015_07_09::Codepipeline_2015_07_09Service::new(self)
    }
    /// Get schemas_2019_12_02 service handler
    pub fn schemas_2019_12_02(&self) -> schemas_2019_12_02::Schemas_2019_12_02Service<'_> {
        schemas_2019_12_02::Schemas_2019_12_02Service::new(self)
    }
    /// Get emr_serverless_2021_07_13 service handler
    pub fn emr_serverless_2021_07_13(&self) -> emr_serverless_2021_07_13::Emr_serverless_2021_07_13Service<'_> {
        emr_serverless_2021_07_13::Emr_serverless_2021_07_13Service::new(self)
    }
    /// Get sqs_2012_11_05 service handler
    pub fn sqs_2012_11_05(&self) -> sqs_2012_11_05::Sqs_2012_11_05Service<'_> {
        sqs_2012_11_05::Sqs_2012_11_05Service::new(self)
    }
    /// Get license_manager_user_subscriptions_2018_05_10 service handler
    pub fn license_manager_user_subscriptions_2018_05_10(&self) -> license_manager_user_subscriptions_2018_05_10::License_manager_user_subscriptions_2018_05_10Service<'_> {
        license_manager_user_subscriptions_2018_05_10::License_manager_user_subscriptions_2018_05_10Service::new(self)
    }
    /// Get route53_recovery_cluster_2019_12_02 service handler
    pub fn route53_recovery_cluster_2019_12_02(&self) -> route53_recovery_cluster_2019_12_02::Route53_recovery_cluster_2019_12_02Service<'_> {
        route53_recovery_cluster_2019_12_02::Route53_recovery_cluster_2019_12_02Service::new(self)
    }
    /// Get migrationhuborchestrator_2021_08_28 service handler
    pub fn migrationhuborchestrator_2021_08_28(&self) -> migrationhuborchestrator_2021_08_28::Migrationhuborchestrator_2021_08_28Service<'_> {
        migrationhuborchestrator_2021_08_28::Migrationhuborchestrator_2021_08_28Service::new(self)
    }
    /// Get iot_2015_05_28 service handler
    pub fn iot_2015_05_28(&self) -> iot_2015_05_28::Iot_2015_05_28Service<'_> {
        iot_2015_05_28::Iot_2015_05_28Service::new(self)
    }
    /// Get sso_oidc_2019_06_10 service handler
    pub fn sso_oidc_2019_06_10(&self) -> sso_oidc_2019_06_10::Sso_oidc_2019_06_10Service<'_> {
        sso_oidc_2019_06_10::Sso_oidc_2019_06_10Service::new(self)
    }
    /// Get codestar_notifications_2019_10_15 service handler
    pub fn codestar_notifications_2019_10_15(&self) -> codestar_notifications_2019_10_15::Codestar_notifications_2019_10_15Service<'_> {
        codestar_notifications_2019_10_15::Codestar_notifications_2019_10_15Service::new(self)
    }
    /// Get ebs_2019_11_02 service handler
    pub fn ebs_2019_11_02(&self) -> ebs_2019_11_02::Ebs_2019_11_02Service<'_> {
        ebs_2019_11_02::Ebs_2019_11_02Service::new(self)
    }
    /// Get aiops_2018_05_10 service handler
    pub fn aiops_2018_05_10(&self) -> aiops_2018_05_10::Aiops_2018_05_10Service<'_> {
        aiops_2018_05_10::Aiops_2018_05_10Service::new(self)
    }
    /// Get amplify_2017_07_25 service handler
    pub fn amplify_2017_07_25(&self) -> amplify_2017_07_25::Amplify_2017_07_25Service<'_> {
        amplify_2017_07_25::Amplify_2017_07_25Service::new(self)
    }
    /// Get cloudcontrol_2021_09_30 service handler
    pub fn cloudcontrol_2021_09_30(&self) -> cloudcontrol_2021_09_30::Cloudcontrol_2021_09_30Service<'_> {
        cloudcontrol_2021_09_30::Cloudcontrol_2021_09_30Service::new(self)
    }
    /// Get wellarchitected_2020_03_31 service handler
    pub fn wellarchitected_2020_03_31(&self) -> wellarchitected_2020_03_31::Wellarchitected_2020_03_31Service<'_> {
        wellarchitected_2020_03_31::Wellarchitected_2020_03_31Service::new(self)
    }
    /// Get route_53_2013_04_01 service handler
    pub fn route_53_2013_04_01(&self) -> route_53_2013_04_01::Route_53_2013_04_01Service<'_> {
        route_53_2013_04_01::Route_53_2013_04_01Service::new(self)
    }
    /// Get ssm_incidents_2018_05_10 service handler
    pub fn ssm_incidents_2018_05_10(&self) -> ssm_incidents_2018_05_10::Ssm_incidents_2018_05_10Service<'_> {
        ssm_incidents_2018_05_10::Ssm_incidents_2018_05_10Service::new(self)
    }
    /// Get bedrock_2023_04_20 service handler
    pub fn bedrock_2023_04_20(&self) -> bedrock_2023_04_20::Bedrock_2023_04_20Service<'_> {
        bedrock_2023_04_20::Bedrock_2023_04_20Service::new(self)
    }
    /// Get ivs_realtime_2020_07_14 service handler
    pub fn ivs_realtime_2020_07_14(&self) -> ivs_realtime_2020_07_14::Ivs_realtime_2020_07_14Service<'_> {
        ivs_realtime_2020_07_14::Ivs_realtime_2020_07_14Service::new(self)
    }
    /// Get migrationhub_config_2019_06_30 service handler
    pub fn migrationhub_config_2019_06_30(&self) -> migrationhub_config_2019_06_30::Migrationhub_config_2019_06_30Service<'_> {
        migrationhub_config_2019_06_30::Migrationhub_config_2019_06_30Service::new(self)
    }
    /// Get redshift_2012_12_01 service handler
    pub fn redshift_2012_12_01(&self) -> redshift_2012_12_01::Redshift_2012_12_01Service<'_> {
        redshift_2012_12_01::Redshift_2012_12_01Service::new(self)
    }
    /// Get inspector_scan_2023_08_08 service handler
    pub fn inspector_scan_2023_08_08(&self) -> inspector_scan_2023_08_08::Inspector_scan_2023_08_08Service<'_> {
        inspector_scan_2023_08_08::Inspector_scan_2023_08_08Service::new(self)
    }
    /// Get connectcases_2022_10_03 service handler
    pub fn connectcases_2022_10_03(&self) -> connectcases_2022_10_03::Connectcases_2022_10_03Service<'_> {
        connectcases_2022_10_03::Connectcases_2022_10_03Service::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get pca_connector_scep_2018_05_10 service handler
    pub fn pca_connector_scep_2018_05_10(&self) -> pca_connector_scep_2018_05_10::Pca_connector_scep_2018_05_10Service<'_> {
        pca_connector_scep_2018_05_10::Pca_connector_scep_2018_05_10Service::new(self)
    }
    /// Get appflow_2020_08_23 service handler
    pub fn appflow_2020_08_23(&self) -> appflow_2020_08_23::Appflow_2020_08_23Service<'_> {
        appflow_2020_08_23::Appflow_2020_08_23Service::new(self)
    }
    /// Get gamelift_2015_10_01 service handler
    pub fn gamelift_2015_10_01(&self) -> gamelift_2015_10_01::Gamelift_2015_10_01Service<'_> {
        gamelift_2015_10_01::Gamelift_2015_10_01Service::new(self)
    }
    /// Get cloudtrail_2013_11_01 service handler
    pub fn cloudtrail_2013_11_01(&self) -> cloudtrail_2013_11_01::Cloudtrail_2013_11_01Service<'_> {
        cloudtrail_2013_11_01::Cloudtrail_2013_11_01Service::new(self)
    }
    /// Get resource_groups_2017_11_27 service handler
    pub fn resource_groups_2017_11_27(&self) -> resource_groups_2017_11_27::Resource_groups_2017_11_27Service<'_> {
        resource_groups_2017_11_27::Resource_groups_2017_11_27Service::new(self)
    }
    /// Get supplychain_2024_01_01 service handler
    pub fn supplychain_2024_01_01(&self) -> supplychain_2024_01_01::Supplychain_2024_01_01Service<'_> {
        supplychain_2024_01_01::Supplychain_2024_01_01Service::new(self)
    }
    /// Get timestream_influxdb_2023_01_27 service handler
    pub fn timestream_influxdb_2023_01_27(&self) -> timestream_influxdb_2023_01_27::Timestream_influxdb_2023_01_27Service<'_> {
        timestream_influxdb_2023_01_27::Timestream_influxdb_2023_01_27Service::new(self)
    }
    /// Get pipes_2015_10_07 service handler
    pub fn pipes_2015_10_07(&self) -> pipes_2015_10_07::Pipes_2015_10_07Service<'_> {
        pipes_2015_10_07::Pipes_2015_10_07Service::new(self)
    }
    /// Get evidently_2021_02_01 service handler
    pub fn evidently_2021_02_01(&self) -> evidently_2021_02_01::Evidently_2021_02_01Service<'_> {
        evidently_2021_02_01::Evidently_2021_02_01Service::new(self)
    }
    /// Get codeguru_security_2018_05_10 service handler
    pub fn codeguru_security_2018_05_10(&self) -> codeguru_security_2018_05_10::Codeguru_security_2018_05_10Service<'_> {
        codeguru_security_2018_05_10::Codeguru_security_2018_05_10Service::new(self)
    }
    /// Get cost_optimization_hub_2022_07_26 service handler
    pub fn cost_optimization_hub_2022_07_26(&self) -> cost_optimization_hub_2022_07_26::Cost_optimization_hub_2022_07_26Service<'_> {
        cost_optimization_hub_2022_07_26::Cost_optimization_hub_2022_07_26Service::new(self)
    }
    /// Get amplifyuibuilder_2021_08_11 service handler
    pub fn amplifyuibuilder_2021_08_11(&self) -> amplifyuibuilder_2021_08_11::Amplifyuibuilder_2021_08_11Service<'_> {
        amplifyuibuilder_2021_08_11::Amplifyuibuilder_2021_08_11Service::new(self)
    }
    /// Get route53_recovery_control_config_2020_11_02 service handler
    pub fn route53_recovery_control_config_2020_11_02(&self) -> route53_recovery_control_config_2020_11_02::Route53_recovery_control_config_2020_11_02Service<'_> {
        route53_recovery_control_config_2020_11_02::Route53_recovery_control_config_2020_11_02Service::new(self)
    }
    /// Get vpc_lattice_2022_11_30 service handler
    pub fn vpc_lattice_2022_11_30(&self) -> vpc_lattice_2022_11_30::Vpc_lattice_2022_11_30Service<'_> {
        vpc_lattice_2022_11_30::Vpc_lattice_2022_11_30Service::new(self)
    }
    /// Get managedblockchain_query_2023_05_04 service handler
    pub fn managedblockchain_query_2023_05_04(&self) -> managedblockchain_query_2023_05_04::Managedblockchain_query_2023_05_04Service<'_> {
        managedblockchain_query_2023_05_04::Managedblockchain_query_2023_05_04Service::new(self)
    }
    /// Get redshift_data_2019_12_20 service handler
    pub fn redshift_data_2019_12_20(&self) -> redshift_data_2019_12_20::Redshift_data_2019_12_20Service<'_> {
        redshift_data_2019_12_20::Redshift_data_2019_12_20Service::new(self)
    }
    /// Get mediatailor_2018_04_23 service handler
    pub fn mediatailor_2018_04_23(&self) -> mediatailor_2018_04_23::Mediatailor_2018_04_23Service<'_> {
        mediatailor_2018_04_23::Mediatailor_2018_04_23Service::new(self)
    }
    /// Get mediapackagev2_2022_12_25 service handler
    pub fn mediapackagev2_2022_12_25(&self) -> mediapackagev2_2022_12_25::Mediapackagev2_2022_12_25Service<'_> {
        mediapackagev2_2022_12_25::Mediapackagev2_2022_12_25Service::new(self)
    }
    /// Get pi_2018_02_27 service handler
    pub fn pi_2018_02_27(&self) -> pi_2018_02_27::Pi_2018_02_27Service<'_> {
        pi_2018_02_27::Pi_2018_02_27Service::new(self)
    }
    /// Get appconfig_2019_10_09 service handler
    pub fn appconfig_2019_10_09(&self) -> appconfig_2019_10_09::Appconfig_2019_10_09Service<'_> {
        appconfig_2019_10_09::Appconfig_2019_10_09Service::new(self)
    }
    /// Get networkmonitor_2023_08_01 service handler
    pub fn networkmonitor_2023_08_01(&self) -> networkmonitor_2023_08_01::Networkmonitor_2023_08_01Service<'_> {
        networkmonitor_2023_08_01::Networkmonitor_2023_08_01Service::new(self)
    }
    /// Get network_firewall_2020_11_12 service handler
    pub fn network_firewall_2020_11_12(&self) -> network_firewall_2020_11_12::Network_firewall_2020_11_12Service<'_> {
        network_firewall_2020_11_12::Network_firewall_2020_11_12Service::new(self)
    }
    /// Get connectparticipant_2018_09_07 service handler
    pub fn connectparticipant_2018_09_07(&self) -> connectparticipant_2018_09_07::Connectparticipant_2018_09_07Service<'_> {
        connectparticipant_2018_09_07::Connectparticipant_2018_09_07Service::new(self)
    }
    /// Get mgn_2020_02_26 service handler
    pub fn mgn_2020_02_26(&self) -> mgn_2020_02_26::Mgn_2020_02_26Service<'_> {
        mgn_2020_02_26::Mgn_2020_02_26Service::new(self)
    }
    /// Get sagemaker_edge_2020_09_23 service handler
    pub fn sagemaker_edge_2020_09_23(&self) -> sagemaker_edge_2020_09_23::Sagemaker_edge_2020_09_23Service<'_> {
        sagemaker_edge_2020_09_23::Sagemaker_edge_2020_09_23Service::new(self)
    }
    /// Get applicationcostprofiler_2020_09_10 service handler
    pub fn applicationcostprofiler_2020_09_10(&self) -> applicationcostprofiler_2020_09_10::Applicationcostprofiler_2020_09_10Service<'_> {
        applicationcostprofiler_2020_09_10::Applicationcostprofiler_2020_09_10Service::new(self)
    }
    /// Get keyspaces_2022_02_10 service handler
    pub fn keyspaces_2022_02_10(&self) -> keyspaces_2022_02_10::Keyspaces_2022_02_10Service<'_> {
        keyspaces_2022_02_10::Keyspaces_2022_02_10Service::new(self)
    }
    /// Get iam_2010_05_08 service handler
    pub fn iam_2010_05_08(&self) -> iam_2010_05_08::Iam_2010_05_08Service<'_> {
        iam_2010_05_08::Iam_2010_05_08Service::new(self)
    }
    /// Get data_pipeline_2012_10_29 service handler
    pub fn data_pipeline_2012_10_29(&self) -> data_pipeline_2012_10_29::Data_pipeline_2012_10_29Service<'_> {
        data_pipeline_2012_10_29::Data_pipeline_2012_10_29Service::new(self)
    }
    /// Get odb_2024_08_20 service handler
    pub fn odb_2024_08_20(&self) -> odb_2024_08_20::Odb_2024_08_20Service<'_> {
        odb_2024_08_20::Odb_2024_08_20Service::new(self)
    }
    /// Get chime_sdk_messaging_2021_05_15 service handler
    pub fn chime_sdk_messaging_2021_05_15(&self) -> chime_sdk_messaging_2021_05_15::Chime_sdk_messaging_2021_05_15Service<'_> {
        chime_sdk_messaging_2021_05_15::Chime_sdk_messaging_2021_05_15Service::new(self)
    }
    /// Get mediastore_2017_09_01 service handler
    pub fn mediastore_2017_09_01(&self) -> mediastore_2017_09_01::Mediastore_2017_09_01Service<'_> {
        mediastore_2017_09_01::Mediastore_2017_09_01Service::new(self)
    }
    /// Get iot_wireless_2020_11_22 service handler
    pub fn iot_wireless_2020_11_22(&self) -> iot_wireless_2020_11_22::Iot_wireless_2020_11_22Service<'_> {
        iot_wireless_2020_11_22::Iot_wireless_2020_11_22Service::new(self)
    }
    /// Get cloud9_2017_09_23 service handler
    pub fn cloud9_2017_09_23(&self) -> cloud9_2017_09_23::Cloud9_2017_09_23Service<'_> {
        cloud9_2017_09_23::Cloud9_2017_09_23Service::new(self)
    }
    /// Get wisdom_2020_10_19 service handler
    pub fn wisdom_2020_10_19(&self) -> wisdom_2020_10_19::Wisdom_2020_10_19Service<'_> {
        wisdom_2020_10_19::Wisdom_2020_10_19Service::new(self)
    }
    /// Get sagemaker_runtime_2017_05_13 service handler
    pub fn sagemaker_runtime_2017_05_13(&self) -> sagemaker_runtime_2017_05_13::Sagemaker_runtime_2017_05_13Service<'_> {
        sagemaker_runtime_2017_05_13::Sagemaker_runtime_2017_05_13Service::new(self)
    }
    /// Get sso_2019_06_10 service handler
    pub fn sso_2019_06_10(&self) -> sso_2019_06_10::Sso_2019_06_10Service<'_> {
        sso_2019_06_10::Sso_2019_06_10Service::new(self)
    }
    /// Get auditmanager_2017_07_25 service handler
    pub fn auditmanager_2017_07_25(&self) -> auditmanager_2017_07_25::Auditmanager_2017_07_25Service<'_> {
        auditmanager_2017_07_25::Auditmanager_2017_07_25Service::new(self)
    }
    /// Get snowball_2016_06_30 service handler
    pub fn snowball_2016_06_30(&self) -> snowball_2016_06_30::Snowball_2016_06_30Service<'_> {
        snowball_2016_06_30::Snowball_2016_06_30Service::new(self)
    }
    /// Get migration_hub_2017_05_31 service handler
    pub fn migration_hub_2017_05_31(&self) -> migration_hub_2017_05_31::Migration_hub_2017_05_31Service<'_> {
        migration_hub_2017_05_31::Migration_hub_2017_05_31Service::new(self)
    }
    /// Get identitystore_2020_06_15 service handler
    pub fn identitystore_2020_06_15(&self) -> identitystore_2020_06_15::Identitystore_2020_06_15Service<'_> {
        identitystore_2020_06_15::Identitystore_2020_06_15Service::new(self)
    }
    /// Get elastic_load_balancing service handler
    pub fn elastic_load_balancing(&self) -> elastic_load_balancing::Elastic_load_balancingService<'_> {
        elastic_load_balancing::Elastic_load_balancingService::new(self)
    }
    /// Get connectcampaigns_2021_01_30 service handler
    pub fn connectcampaigns_2021_01_30(&self) -> connectcampaigns_2021_01_30::Connectcampaigns_2021_01_30Service<'_> {
        connectcampaigns_2021_01_30::Connectcampaigns_2021_01_30Service::new(self)
    }
    /// Get textract_2018_06_27 service handler
    pub fn textract_2018_06_27(&self) -> textract_2018_06_27::Textract_2018_06_27Service<'_> {
        textract_2018_06_27::Textract_2018_06_27Service::new(self)
    }
    /// Get compute_optimizer_2019_11_01 service handler
    pub fn compute_optimizer_2019_11_01(&self) -> compute_optimizer_2019_11_01::Compute_optimizer_2019_11_01Service<'_> {
        compute_optimizer_2019_11_01::Compute_optimizer_2019_11_01Service::new(self)
    }
    /// Get s3tables_2018_05_10 service handler
    pub fn s3tables_2018_05_10(&self) -> s3tables_2018_05_10::S3tables_2018_05_10Service<'_> {
        s3tables_2018_05_10::S3tables_2018_05_10Service::new(self)
    }
    /// Get eks_2017_11_01 service handler
    pub fn eks_2017_11_01(&self) -> eks_2017_11_01::Eks_2017_11_01Service<'_> {
        eks_2017_11_01::Eks_2017_11_01Service::new(self)
    }
    /// Get support_2013_04_15 service handler
    pub fn support_2013_04_15(&self) -> support_2013_04_15::Support_2013_04_15Service<'_> {
        support_2013_04_15::Support_2013_04_15Service::new(self)
    }
    /// Get mturk_2017_01_17 service handler
    pub fn mturk_2017_01_17(&self) -> mturk_2017_01_17::Mturk_2017_01_17Service<'_> {
        mturk_2017_01_17::Mturk_2017_01_17Service::new(self)
    }
    /// Get apigatewayv2_2018_11_29 service handler
    pub fn apigatewayv2_2018_11_29(&self) -> apigatewayv2_2018_11_29::Apigatewayv2_2018_11_29Service<'_> {
        apigatewayv2_2018_11_29::Apigatewayv2_2018_11_29Service::new(self)
    }
    /// Get cognito_identity_provider_2016_04_18 service handler
    pub fn cognito_identity_provider_2016_04_18(&self) -> cognito_identity_provider_2016_04_18::Cognito_identity_provider_2016_04_18Service<'_> {
        cognito_identity_provider_2016_04_18::Cognito_identity_provider_2016_04_18Service::new(self)
    }
    /// Get bedrock_data_automation_runtime_2024_06_13 service handler
    pub fn bedrock_data_automation_runtime_2024_06_13(&self) -> bedrock_data_automation_runtime_2024_06_13::Bedrock_data_automation_runtime_2024_06_13Service<'_> {
        bedrock_data_automation_runtime_2024_06_13::Bedrock_data_automation_runtime_2024_06_13Service::new(self)
    }
    /// Get pinpoint_sms service handler
    pub fn pinpoint_sms(&self) -> pinpoint_sms::Pinpoint_smsService<'_> {
        pinpoint_sms::Pinpoint_smsService::new(self)
    }
    /// Get amp_2020_08_01 service handler
    pub fn amp_2020_08_01(&self) -> amp_2020_08_01::Amp_2020_08_01Service<'_> {
        amp_2020_08_01::Amp_2020_08_01Service::new(self)
    }
    /// Get drs_2020_02_26 service handler
    pub fn drs_2020_02_26(&self) -> drs_2020_02_26::Drs_2020_02_26Service<'_> {
        drs_2020_02_26::Drs_2020_02_26Service::new(self)
    }
    /// Get payment_cryptography_2021_09_14 service handler
    pub fn payment_cryptography_2021_09_14(&self) -> payment_cryptography_2021_09_14::Payment_cryptography_2021_09_14Service<'_> {
        payment_cryptography_2021_09_14::Payment_cryptography_2021_09_14Service::new(self)
    }
    /// Get kafkaconnect_2021_09_14 service handler
    pub fn kafkaconnect_2021_09_14(&self) -> kafkaconnect_2021_09_14::Kafkaconnect_2021_09_14Service<'_> {
        kafkaconnect_2021_09_14::Kafkaconnect_2021_09_14Service::new(self)
    }
    /// Get kafka_2018_11_14 service handler
    pub fn kafka_2018_11_14(&self) -> kafka_2018_11_14::Kafka_2018_11_14Service<'_> {
        kafka_2018_11_14::Kafka_2018_11_14Service::new(self)
    }
    /// Get databrew_2017_07_25 service handler
    pub fn databrew_2017_07_25(&self) -> databrew_2017_07_25::Databrew_2017_07_25Service<'_> {
        databrew_2017_07_25::Databrew_2017_07_25Service::new(self)
    }
    /// Get support_app_2021_08_20 service handler
    pub fn support_app_2021_08_20(&self) -> support_app_2021_08_20::Support_app_2021_08_20Service<'_> {
        support_app_2021_08_20::Support_app_2021_08_20Service::new(self)
    }
    /// Get codedeploy_2014_10_06 service handler
    pub fn codedeploy_2014_10_06(&self) -> codedeploy_2014_10_06::Codedeploy_2014_10_06Service<'_> {
        codedeploy_2014_10_06::Codedeploy_2014_10_06Service::new(self)
    }
    /// Get batch_2016_08_10 service handler
    pub fn batch_2016_08_10(&self) -> batch_2016_08_10::Batch_2016_08_10Service<'_> {
        batch_2016_08_10::Batch_2016_08_10Service::new(self)
    }
    /// Get savingsplans_2019_06_28 service handler
    pub fn savingsplans_2019_06_28(&self) -> savingsplans_2019_06_28::Savingsplans_2019_06_28Service<'_> {
        savingsplans_2019_06_28::Savingsplans_2019_06_28Service::new(self)
    }
    /// Get bedrock_agent_2023_06_05 service handler
    pub fn bedrock_agent_2023_06_05(&self) -> bedrock_agent_2023_06_05::Bedrock_agent_2023_06_05Service<'_> {
        bedrock_agent_2023_06_05::Bedrock_agent_2023_06_05Service::new(self)
    }
    /// Get directory_service_2015_04_16 service handler
    pub fn directory_service_2015_04_16(&self) -> directory_service_2015_04_16::Directory_service_2015_04_16Service<'_> {
        directory_service_2015_04_16::Directory_service_2015_04_16Service::new(self)
    }
    /// Get workspaces_instances_2022_07_26 service handler
    pub fn workspaces_instances_2022_07_26(&self) -> workspaces_instances_2022_07_26::Workspaces_instances_2022_07_26Service<'_> {
        workspaces_instances_2022_07_26::Workspaces_instances_2022_07_26Service::new(self)
    }
    /// Get chime_sdk_media_pipelines_2021_07_15 service handler
    pub fn chime_sdk_media_pipelines_2021_07_15(&self) -> chime_sdk_media_pipelines_2021_07_15::Chime_sdk_media_pipelines_2021_07_15Service<'_> {
        chime_sdk_media_pipelines_2021_07_15::Chime_sdk_media_pipelines_2021_07_15Service::new(self)
    }
    /// Get migrationhubstrategy_2020_02_19 service handler
    pub fn migrationhubstrategy_2020_02_19(&self) -> migrationhubstrategy_2020_02_19::Migrationhubstrategy_2020_02_19Service<'_> {
        migrationhubstrategy_2020_02_19::Migrationhubstrategy_2020_02_19Service::new(self)
    }
    /// Get timestream_query_2018_11_01 service handler
    pub fn timestream_query_2018_11_01(&self) -> timestream_query_2018_11_01::Timestream_query_2018_11_01Service<'_> {
        timestream_query_2018_11_01::Timestream_query_2018_11_01Service::new(self)
    }
    /// Get codeguru_reviewer_2019_09_19 service handler
    pub fn codeguru_reviewer_2019_09_19(&self) -> codeguru_reviewer_2019_09_19::Codeguru_reviewer_2019_09_19Service<'_> {
        codeguru_reviewer_2019_09_19::Codeguru_reviewer_2019_09_19Service::new(self)
    }
    /// Get appsync_2017_07_25 service handler
    pub fn appsync_2017_07_25(&self) -> appsync_2017_07_25::Appsync_2017_07_25Service<'_> {
        appsync_2017_07_25::Appsync_2017_07_25Service::new(self)
    }
    /// Get dlm_2018_01_12 service handler
    pub fn dlm_2018_01_12(&self) -> dlm_2018_01_12::Dlm_2018_01_12Service<'_> {
        dlm_2018_01_12::Dlm_2018_01_12Service::new(self)
    }
    /// Get iot_data_plane_2015_05_28 service handler
    pub fn iot_data_plane_2015_05_28(&self) -> iot_data_plane_2015_05_28::Iot_data_plane_2015_05_28Service<'_> {
        iot_data_plane_2015_05_28::Iot_data_plane_2015_05_28Service::new(self)
    }
    /// Get global_accelerator_2018_08_08 service handler
    pub fn global_accelerator_2018_08_08(&self) -> global_accelerator_2018_08_08::Global_accelerator_2018_08_08Service<'_> {
        global_accelerator_2018_08_08::Global_accelerator_2018_08_08Service::new(self)
    }
    /// Get amplifybackend_2020_08_11 service handler
    pub fn amplifybackend_2020_08_11(&self) -> amplifybackend_2020_08_11::Amplifybackend_2020_08_11Service<'_> {
        amplifybackend_2020_08_11::Amplifybackend_2020_08_11Service::new(self)
    }
    /// Get datazone_2018_05_10 service handler
    pub fn datazone_2018_05_10(&self) -> datazone_2018_05_10::Datazone_2018_05_10Service<'_> {
        datazone_2018_05_10::Datazone_2018_05_10Service::new(self)
    }
    /// Get connectcampaignsv2_2024_04_23 service handler
    pub fn connectcampaignsv2_2024_04_23(&self) -> connectcampaignsv2_2024_04_23::Connectcampaignsv2_2024_04_23Service<'_> {
        connectcampaignsv2_2024_04_23::Connectcampaignsv2_2024_04_23Service::new(self)
    }
    /// Get billingconductor_2021_07_30 service handler
    pub fn billingconductor_2021_07_30(&self) -> billingconductor_2021_07_30::Billingconductor_2021_07_30Service<'_> {
        billingconductor_2021_07_30::Billingconductor_2021_07_30Service::new(self)
    }
    /// Get budgets_2016_10_20 service handler
    pub fn budgets_2016_10_20(&self) -> budgets_2016_10_20::Budgets_2016_10_20Service<'_> {
        budgets_2016_10_20::Budgets_2016_10_20Service::new(self)
    }
    /// Get cloudtrail_data_2021_08_11 service handler
    pub fn cloudtrail_data_2021_08_11(&self) -> cloudtrail_data_2021_08_11::Cloudtrail_data_2021_08_11Service<'_> {
        cloudtrail_data_2021_08_11::Cloudtrail_data_2021_08_11Service::new(self)
    }
    /// Get geo_routes_2020_11_19 service handler
    pub fn geo_routes_2020_11_19(&self) -> geo_routes_2020_11_19::Geo_routes_2020_11_19Service<'_> {
        geo_routes_2020_11_19::Geo_routes_2020_11_19Service::new(self)
    }
    /// Get m2_2021_04_28 service handler
    pub fn m2_2021_04_28(&self) -> m2_2021_04_28::M2_2021_04_28Service<'_> {
        m2_2021_04_28::M2_2021_04_28Service::new(self)
    }
    /// Get pinpoint_email_2018_07_26 service handler
    pub fn pinpoint_email_2018_07_26(&self) -> pinpoint_email_2018_07_26::Pinpoint_email_2018_07_26Service<'_> {
        pinpoint_email_2018_07_26::Pinpoint_email_2018_07_26Service::new(self)
    }
    /// Get lex_models service handler
    pub fn lex_models(&self) -> lex_models::Lex_modelsService<'_> {
        lex_models::Lex_modelsService::new(self)
    }
    /// Get finspace_2021_03_12 service handler
    pub fn finspace_2021_03_12(&self) -> finspace_2021_03_12::Finspace_2021_03_12Service<'_> {
        finspace_2021_03_12::Finspace_2021_03_12Service::new(self)
    }
    /// Get detective_2018_10_26 service handler
    pub fn detective_2018_10_26(&self) -> detective_2018_10_26::Detective_2018_10_26Service<'_> {
        detective_2018_10_26::Detective_2018_10_26Service::new(self)
    }
    /// Get lambda_2015_03_31 service handler
    pub fn lambda_2015_03_31(&self) -> lambda_2015_03_31::Lambda_2015_03_31Service<'_> {
        lambda_2015_03_31::Lambda_2015_03_31Service::new(self)
    }
    /// Get kinesis_analytics service handler
    pub fn kinesis_analytics(&self) -> kinesis_analytics::Kinesis_analyticsService<'_> {
        kinesis_analytics::Kinesis_analyticsService::new(self)
    }
    /// Get panorama_2019_07_24 service handler
    pub fn panorama_2019_07_24(&self) -> panorama_2019_07_24::Panorama_2019_07_24Service<'_> {
        panorama_2019_07_24::Panorama_2019_07_24Service::new(self)
    }
    /// Get iot_events_2018_07_27 service handler
    pub fn iot_events_2018_07_27(&self) -> iot_events_2018_07_27::Iot_events_2018_07_27Service<'_> {
        iot_events_2018_07_27::Iot_events_2018_07_27Service::new(self)
    }
    /// Get app_mesh_2019_01_25 service handler
    pub fn app_mesh_2019_01_25(&self) -> app_mesh_2019_01_25::App_mesh_2019_01_25Service<'_> {
        app_mesh_2019_01_25::App_mesh_2019_01_25Service::new(self)
    }
    /// Get managedblockchain_2018_09_24 service handler
    pub fn managedblockchain_2018_09_24(&self) -> managedblockchain_2018_09_24::Managedblockchain_2018_09_24Service<'_> {
        managedblockchain_2018_09_24::Managedblockchain_2018_09_24Service::new(self)
    }
    /// Get waf_2015_08_24 service handler
    pub fn waf_2015_08_24(&self) -> waf_2015_08_24::Waf_2015_08_24Service<'_> {
        waf_2015_08_24::Waf_2015_08_24Service::new(self)
    }
    /// Get ivs_2020_07_14 service handler
    pub fn ivs_2020_07_14(&self) -> ivs_2020_07_14::Ivs_2020_07_14Service<'_> {
        ivs_2020_07_14::Ivs_2020_07_14Service::new(self)
    }
    /// Get devops_guru_2020_12_01 service handler
    pub fn devops_guru_2020_12_01(&self) -> devops_guru_2020_12_01::Devops_guru_2020_12_01Service<'_> {
        devops_guru_2020_12_01::Devops_guru_2020_12_01Service::new(self)
    }
    /// Get cost_explorer_2017_10_25 service handler
    pub fn cost_explorer_2017_10_25(&self) -> cost_explorer_2017_10_25::Cost_explorer_2017_10_25Service<'_> {
        cost_explorer_2017_10_25::Cost_explorer_2017_10_25Service::new(self)
    }
    /// Get mq_2017_11_27 service handler
    pub fn mq_2017_11_27(&self) -> mq_2017_11_27::Mq_2017_11_27Service<'_> {
        mq_2017_11_27::Mq_2017_11_27Service::new(self)
    }
    /// Get route53_recovery_readiness_2019_12_02 service handler
    pub fn route53_recovery_readiness_2019_12_02(&self) -> route53_recovery_readiness_2019_12_02::Route53_recovery_readiness_2019_12_02Service<'_> {
        route53_recovery_readiness_2019_12_02::Route53_recovery_readiness_2019_12_02Service::new(self)
    }
    /// Get internetmonitor_2021_06_03 service handler
    pub fn internetmonitor_2021_06_03(&self) -> internetmonitor_2021_06_03::Internetmonitor_2021_06_03Service<'_> {
        internetmonitor_2021_06_03::Internetmonitor_2021_06_03Service::new(self)
    }
    /// Get license_manager_2018_08_01 service handler
    pub fn license_manager_2018_08_01(&self) -> license_manager_2018_08_01::License_manager_2018_08_01Service<'_> {
        license_manager_2018_08_01::License_manager_2018_08_01Service::new(self)
    }
    /// Get codestar_connections_2019_12_01 service handler
    pub fn codestar_connections_2019_12_01(&self) -> codestar_connections_2019_12_01::Codestar_connections_2019_12_01Service<'_> {
        codestar_connections_2019_12_01::Codestar_connections_2019_12_01Service::new(self)
    }
    /// Get artifact_2018_05_10 service handler
    pub fn artifact_2018_05_10(&self) -> artifact_2018_05_10::Artifact_2018_05_10Service<'_> {
        artifact_2018_05_10::Artifact_2018_05_10Service::new(self)
    }
    /// Get ssm_guiconnect_2021_05_01 service handler
    pub fn ssm_guiconnect_2021_05_01(&self) -> ssm_guiconnect_2021_05_01::Ssm_guiconnect_2021_05_01Service<'_> {
        ssm_guiconnect_2021_05_01::Ssm_guiconnect_2021_05_01Service::new(self)
    }
    /// Get iotsitewise_2019_12_02 service handler
    pub fn iotsitewise_2019_12_02(&self) -> iotsitewise_2019_12_02::Iotsitewise_2019_12_02Service<'_> {
        iotsitewise_2019_12_02::Iotsitewise_2019_12_02Service::new(self)
    }
    /// Get serverlessapplicationrepository_2017_09_08 service handler
    pub fn serverlessapplicationrepository_2017_09_08(&self) -> serverlessapplicationrepository_2017_09_08::Serverlessapplicationrepository_2017_09_08Service<'_> {
        serverlessapplicationrepository_2017_09_08::Serverlessapplicationrepository_2017_09_08Service::new(self)
    }
    /// Get ssm_quicksetup_2018_05_10 service handler
    pub fn ssm_quicksetup_2018_05_10(&self) -> ssm_quicksetup_2018_05_10::Ssm_quicksetup_2018_05_10Service<'_> {
        ssm_quicksetup_2018_05_10::Ssm_quicksetup_2018_05_10Service::new(self)
    }
    /// Get docdb_elastic_2022_11_28 service handler
    pub fn docdb_elastic_2022_11_28(&self) -> docdb_elastic_2022_11_28::Docdb_elastic_2022_11_28Service<'_> {
        docdb_elastic_2022_11_28::Docdb_elastic_2022_11_28Service::new(self)
    }
    /// Get sagemaker_metrics_2022_09_30 service handler
    pub fn sagemaker_metrics_2022_09_30(&self) -> sagemaker_metrics_2022_09_30::Sagemaker_metrics_2022_09_30Service<'_> {
        sagemaker_metrics_2022_09_30::Sagemaker_metrics_2022_09_30Service::new(self)
    }
    /// Get license_manager_linux_subscriptions_2018_05_10 service handler
    pub fn license_manager_linux_subscriptions_2018_05_10(&self) -> license_manager_linux_subscriptions_2018_05_10::License_manager_linux_subscriptions_2018_05_10Service<'_> {
        license_manager_linux_subscriptions_2018_05_10::License_manager_linux_subscriptions_2018_05_10Service::new(self)
    }
    /// Get sagemaker_geospatial_2020_05_27 service handler
    pub fn sagemaker_geospatial_2020_05_27(&self) -> sagemaker_geospatial_2020_05_27::Sagemaker_geospatial_2020_05_27Service<'_> {
        sagemaker_geospatial_2020_05_27::Sagemaker_geospatial_2020_05_27Service::new(self)
    }
    /// Get personalize_runtime_2018_05_22 service handler
    pub fn personalize_runtime_2018_05_22(&self) -> personalize_runtime_2018_05_22::Personalize_runtime_2018_05_22Service<'_> {
        personalize_runtime_2018_05_22::Personalize_runtime_2018_05_22Service::new(self)
    }
    /// Get clouddirectory_2017_01_11 service handler
    pub fn clouddirectory_2017_01_11(&self) -> clouddirectory_2017_01_11::Clouddirectory_2017_01_11Service<'_> {
        clouddirectory_2017_01_11::Clouddirectory_2017_01_11Service::new(self)
    }
    /// Get chime_sdk service handler
    pub fn chime_sdk(&self) -> chime_sdk::Chime_sdkService<'_> {
        chime_sdk::Chime_sdkService::new(self)
    }
    /// Get notificationscontacts_2018_05_10 service handler
    pub fn notificationscontacts_2018_05_10(&self) -> notificationscontacts_2018_05_10::Notificationscontacts_2018_05_10Service<'_> {
        notificationscontacts_2018_05_10::Notificationscontacts_2018_05_10Service::new(self)
    }
    /// Get backupsearch_2018_05_10 service handler
    pub fn backupsearch_2018_05_10(&self) -> backupsearch_2018_05_10::Backupsearch_2018_05_10Service<'_> {
        backupsearch_2018_05_10::Backupsearch_2018_05_10Service::new(self)
    }
    /// Get dynamodb_streams_2012_08_10 service handler
    pub fn dynamodb_streams_2012_08_10(&self) -> dynamodb_streams_2012_08_10::Dynamodb_streams_2012_08_10Service<'_> {
        dynamodb_streams_2012_08_10::Dynamodb_streams_2012_08_10Service::new(self)
    }
    /// Get iot_managed_integrations_2025_03_03 service handler
    pub fn iot_managed_integrations_2025_03_03(&self) -> iot_managed_integrations_2025_03_03::Iot_managed_integrations_2025_03_03Service<'_> {
        iot_managed_integrations_2025_03_03::Iot_managed_integrations_2025_03_03Service::new(self)
    }
    /// Get cognito_sync_2014_06_30 service handler
    pub fn cognito_sync_2014_06_30(&self) -> cognito_sync_2014_06_30::Cognito_sync_2014_06_30Service<'_> {
        cognito_sync_2014_06_30::Cognito_sync_2014_06_30Service::new(self)
    }
    /// Get codeartifact_2018_09_22 service handler
    pub fn codeartifact_2018_09_22(&self) -> codeartifact_2018_09_22::Codeartifact_2018_09_22Service<'_> {
        codeartifact_2018_09_22::Codeartifact_2018_09_22Service::new(self)
    }
    /// Get arc_region_switch_2022_07_26 service handler
    pub fn arc_region_switch_2022_07_26(&self) -> arc_region_switch_2022_07_26::Arc_region_switch_2022_07_26Service<'_> {
        arc_region_switch_2022_07_26::Arc_region_switch_2022_07_26Service::new(self)
    }
    /// Get elastic_load_balancing_2012_06_01 service handler
    pub fn elastic_load_balancing_2012_06_01(&self) -> elastic_load_balancing_2012_06_01::Elastic_load_balancing_2012_06_01Service<'_> {
        elastic_load_balancing_2012_06_01::Elastic_load_balancing_2012_06_01Service::new(self)
    }
    /// Get guardduty_2017_11_28 service handler
    pub fn guardduty_2017_11_28(&self) -> guardduty_2017_11_28::Guardduty_2017_11_28Service<'_> {
        guardduty_2017_11_28::Guardduty_2017_11_28Service::new(self)
    }
    /// Get cleanrooms_2022_02_17 service handler
    pub fn cleanrooms_2022_02_17(&self) -> cleanrooms_2022_02_17::Cleanrooms_2022_02_17Service<'_> {
        cleanrooms_2022_02_17::Cleanrooms_2022_02_17Service::new(self)
    }
    /// Get trustedadvisor_2022_09_15 service handler
    pub fn trustedadvisor_2022_09_15(&self) -> trustedadvisor_2022_09_15::Trustedadvisor_2022_09_15Service<'_> {
        trustedadvisor_2022_09_15::Trustedadvisor_2022_09_15Service::new(self)
    }
    /// Get sagemaker_a2i_runtime_2019_11_07 service handler
    pub fn sagemaker_a2i_runtime_2019_11_07(&self) -> sagemaker_a2i_runtime_2019_11_07::Sagemaker_a2i_runtime_2019_11_07Service<'_> {
        sagemaker_a2i_runtime_2019_11_07::Sagemaker_a2i_runtime_2019_11_07Service::new(self)
    }
    /// Get dax_2017_04_19 service handler
    pub fn dax_2017_04_19(&self) -> dax_2017_04_19::Dax_2017_04_19Service<'_> {
        dax_2017_04_19::Dax_2017_04_19Service::new(self)
    }
    /// Get docdb_2014_10_31 service handler
    pub fn docdb_2014_10_31(&self) -> docdb_2014_10_31::Docdb_2014_10_31Service<'_> {
        docdb_2014_10_31::Docdb_2014_10_31Service::new(self)
    }
    /// Get firehose_2015_08_04 service handler
    pub fn firehose_2015_08_04(&self) -> firehose_2015_08_04::Firehose_2015_08_04Service<'_> {
        firehose_2015_08_04::Firehose_2015_08_04Service::new(self)
    }
    /// Get ivschat_2020_07_14 service handler
    pub fn ivschat_2020_07_14(&self) -> ivschat_2020_07_14::Ivschat_2020_07_14Service<'_> {
        ivschat_2020_07_14::Ivschat_2020_07_14Service::new(self)
    }
    /// Get ses_2010_12_01 service handler
    pub fn ses_2010_12_01(&self) -> ses_2010_12_01::Ses_2010_12_01Service<'_> {
        ses_2010_12_01::Ses_2010_12_01Service::new(self)
    }
    /// Get bcm_dashboards_2025_08_18 service handler
    pub fn bcm_dashboards_2025_08_18(&self) -> bcm_dashboards_2025_08_18::Bcm_dashboards_2025_08_18Service<'_> {
        bcm_dashboards_2025_08_18::Bcm_dashboards_2025_08_18Service::new(self)
    }
    /// Get application_insights_2018_11_25 service handler
    pub fn application_insights_2018_11_25(&self) -> application_insights_2018_11_25::Application_insights_2018_11_25Service<'_> {
        application_insights_2018_11_25::Application_insights_2018_11_25Service::new(self)
    }
    /// Get device_farm_2015_06_23 service handler
    pub fn device_farm_2015_06_23(&self) -> device_farm_2015_06_23::Device_farm_2015_06_23Service<'_> {
        device_farm_2015_06_23::Device_farm_2015_06_23Service::new(self)
    }
    /// Get account_2021_02_01 service handler
    pub fn account_2021_02_01(&self) -> account_2021_02_01::Account_2021_02_01Service<'_> {
        account_2021_02_01::Account_2021_02_01Service::new(self)
    }
    /// Get launch_wizard_2018_05_10 service handler
    pub fn launch_wizard_2018_05_10(&self) -> launch_wizard_2018_05_10::Launch_wizard_2018_05_10Service<'_> {
        launch_wizard_2018_05_10::Launch_wizard_2018_05_10Service::new(self)
    }
    /// Get finspace_data_2020_07_13 service handler
    pub fn finspace_data_2020_07_13(&self) -> finspace_data_2020_07_13::Finspace_data_2020_07_13Service<'_> {
        finspace_data_2020_07_13::Finspace_data_2020_07_13Service::new(self)
    }
    /// Get appconfigdata_2021_11_11 service handler
    pub fn appconfigdata_2021_11_11(&self) -> appconfigdata_2021_11_11::Appconfigdata_2021_11_11Service<'_> {
        appconfigdata_2021_11_11::Appconfigdata_2021_11_11Service::new(self)
    }
    /// Get controlcatalog_2018_05_10 service handler
    pub fn controlcatalog_2018_05_10(&self) -> controlcatalog_2018_05_10::Controlcatalog_2018_05_10Service<'_> {
        controlcatalog_2018_05_10::Controlcatalog_2018_05_10Service::new(self)
    }
    /// Get greengrass_2017_06_07 service handler
    pub fn greengrass_2017_06_07(&self) -> greengrass_2017_06_07::Greengrass_2017_06_07Service<'_> {
        greengrass_2017_06_07::Greengrass_2017_06_07Service::new(self)
    }
    /// Get kendra_ranking_2022_10_19 service handler
    pub fn kendra_ranking_2022_10_19(&self) -> kendra_ranking_2022_10_19::Kendra_ranking_2022_10_19Service<'_> {
        kendra_ranking_2022_10_19::Kendra_ranking_2022_10_19Service::new(self)
    }
    /// Get snow_device_management_2021_08_04 service handler
    pub fn snow_device_management_2021_08_04(&self) -> snow_device_management_2021_08_04::Snow_device_management_2021_08_04Service<'_> {
        snow_device_management_2021_08_04::Snow_device_management_2021_08_04Service::new(self)
    }
    /// Get securityhub_2018_10_26 service handler
    pub fn securityhub_2018_10_26(&self) -> securityhub_2018_10_26::Securityhub_2018_10_26Service<'_> {
        securityhub_2018_10_26::Securityhub_2018_10_26Service::new(self)
    }
    /// Get s3vectors_2025_07_15 service handler
    pub fn s3vectors_2025_07_15(&self) -> s3vectors_2025_07_15::S3vectors_2025_07_15Service<'_> {
        s3vectors_2025_07_15::S3vectors_2025_07_15Service::new(self)
    }
    /// Get workspaces_web_2020_07_08 service handler
    pub fn workspaces_web_2020_07_08(&self) -> workspaces_web_2020_07_08::Workspaces_web_2020_07_08Service<'_> {
        workspaces_web_2020_07_08::Workspaces_web_2020_07_08Service::new(self)
    }
    /// Get backup_2018_11_15 service handler
    pub fn backup_2018_11_15(&self) -> backup_2018_11_15::Backup_2018_11_15Service<'_> {
        backup_2018_11_15::Backup_2018_11_15Service::new(self)
    }
    /// Get opensearchserverless_2021_11_01 service handler
    pub fn opensearchserverless_2021_11_01(&self) -> opensearchserverless_2021_11_01::Opensearchserverless_2021_11_01Service<'_> {
        opensearchserverless_2021_11_01::Opensearchserverless_2021_11_01Service::new(self)
    }
    /// Get cloudformation_2010_05_15 service handler
    pub fn cloudformation_2010_05_15(&self) -> cloudformation_2010_05_15::Cloudformation_2010_05_15Service<'_> {
        cloudformation_2010_05_15::Cloudformation_2010_05_15Service::new(self)
    }
    /// Get kendra_2019_02_03 service handler
    pub fn kendra_2019_02_03(&self) -> kendra_2019_02_03::Kendra_2019_02_03Service<'_> {
        kendra_2019_02_03::Kendra_2019_02_03Service::new(self)
    }
    /// Get connect_2017_08_08 service handler
    pub fn connect_2017_08_08(&self) -> connect_2017_08_08::Connect_2017_08_08Service<'_> {
        connect_2017_08_08::Connect_2017_08_08Service::new(self)
    }
    /// Get machine_learning_2014_12_12 service handler
    pub fn machine_learning_2014_12_12(&self) -> machine_learning_2014_12_12::Machine_learning_2014_12_12Service<'_> {
        machine_learning_2014_12_12::Machine_learning_2014_12_12Service::new(self)
    }
    /// Get elasticache_2015_02_02 service handler
    pub fn elasticache_2015_02_02(&self) -> elasticache_2015_02_02::Elasticache_2015_02_02Service<'_> {
        elasticache_2015_02_02::Elasticache_2015_02_02Service::new(self)
    }
    /// Get sfn_2016_11_23 service handler
    pub fn sfn_2016_11_23(&self) -> sfn_2016_11_23::Sfn_2016_11_23Service<'_> {
        sfn_2016_11_23::Sfn_2016_11_23Service::new(self)
    }
    /// Get sso_admin_2020_07_20 service handler
    pub fn sso_admin_2020_07_20(&self) -> sso_admin_2020_07_20::Sso_admin_2020_07_20Service<'_> {
        sso_admin_2020_07_20::Sso_admin_2020_07_20Service::new(self)
    }
    /// Get auto_scaling_plans_2018_01_06 service handler
    pub fn auto_scaling_plans_2018_01_06(&self) -> auto_scaling_plans_2018_01_06::Auto_scaling_plans_2018_01_06Service<'_> {
        auto_scaling_plans_2018_01_06::Auto_scaling_plans_2018_01_06Service::new(self)
    }
    /// Get comprehend_2017_11_27 service handler
    pub fn comprehend_2017_11_27(&self) -> comprehend_2017_11_27::Comprehend_2017_11_27Service<'_> {
        comprehend_2017_11_27::Comprehend_2017_11_27Service::new(self)
    }
    /// Get rds_data_2018_08_01 service handler
    pub fn rds_data_2018_08_01(&self) -> rds_data_2018_08_01::Rds_data_2018_08_01Service<'_> {
        rds_data_2018_08_01::Rds_data_2018_08_01Service::new(self)
    }
    /// Get chime_sdk_identity_2021_04_20 service handler
    pub fn chime_sdk_identity_2021_04_20(&self) -> chime_sdk_identity_2021_04_20::Chime_sdk_identity_2021_04_20Service<'_> {
        chime_sdk_identity_2021_04_20::Chime_sdk_identity_2021_04_20Service::new(self)
    }
    /// Get rekognition_2016_06_27 service handler
    pub fn rekognition_2016_06_27(&self) -> rekognition_2016_06_27::Rekognition_2016_06_27Service<'_> {
        rekognition_2016_06_27::Rekognition_2016_06_27Service::new(self)
    }
    /// Get appstream_2016_12_01 service handler
    pub fn appstream_2016_12_01(&self) -> appstream_2016_12_01::Appstream_2016_12_01Service<'_> {
        appstream_2016_12_01::Appstream_2016_12_01Service::new(self)
    }
    /// Get polly_2016_06_10 service handler
    pub fn polly_2016_06_10(&self) -> polly_2016_06_10::Polly_2016_06_10Service<'_> {
        polly_2016_06_10::Polly_2016_06_10Service::new(self)
    }
    /// Get invoicing_2024_12_01 service handler
    pub fn invoicing_2024_12_01(&self) -> invoicing_2024_12_01::Invoicing_2024_12_01Service<'_> {
        invoicing_2024_12_01::Invoicing_2024_12_01Service::new(self)
    }
    /// Get rds_2014_10_31 service handler
    pub fn rds_2014_10_31(&self) -> rds_2014_10_31::Rds_2014_10_31Service<'_> {
        rds_2014_10_31::Rds_2014_10_31Service::new(self)
    }
    /// Get pricing_2017_10_15 service handler
    pub fn pricing_2017_10_15(&self) -> pricing_2017_10_15::Pricing_2017_10_15Service<'_> {
        pricing_2017_10_15::Pricing_2017_10_15Service::new(self)
    }
    /// Get swf_2012_01_25 service handler
    pub fn swf_2012_01_25(&self) -> swf_2012_01_25::Swf_2012_01_25Service<'_> {
        swf_2012_01_25::Swf_2012_01_25Service::new(self)
    }
    /// Get cloudfront_keyvaluestore_2022_07_26 service handler
    pub fn cloudfront_keyvaluestore_2022_07_26(&self) -> cloudfront_keyvaluestore_2022_07_26::Cloudfront_keyvaluestore_2022_07_26Service<'_> {
        cloudfront_keyvaluestore_2022_07_26::Cloudfront_keyvaluestore_2022_07_26Service::new(self)
    }
    /// Get marketplace_deployment_2023_01_25 service handler
    pub fn marketplace_deployment_2023_01_25(&self) -> marketplace_deployment_2023_01_25::Marketplace_deployment_2023_01_25Service<'_> {
        marketplace_deployment_2023_01_25::Marketplace_deployment_2023_01_25Service::new(self)
    }
    /// Get medical_imaging_2023_07_19 service handler
    pub fn medical_imaging_2023_07_19(&self) -> medical_imaging_2023_07_19::Medical_imaging_2023_07_19Service<'_> {
        medical_imaging_2023_07_19::Medical_imaging_2023_07_19Service::new(self)
    }
    /// Get transcribe_2017_10_26 service handler
    pub fn transcribe_2017_10_26(&self) -> transcribe_2017_10_26::Transcribe_2017_10_26Service<'_> {
        transcribe_2017_10_26::Transcribe_2017_10_26Service::new(self)
    }
    /// Get observabilityadmin_2018_05_10 service handler
    pub fn observabilityadmin_2018_05_10(&self) -> observabilityadmin_2018_05_10::Observabilityadmin_2018_05_10Service<'_> {
        observabilityadmin_2018_05_10::Observabilityadmin_2018_05_10Service::new(self)
    }
    /// Get notifications_2018_05_10 service handler
    pub fn notifications_2018_05_10(&self) -> notifications_2018_05_10::Notifications_2018_05_10Service<'_> {
        notifications_2018_05_10::Notifications_2018_05_10Service::new(self)
    }
    /// Get codecommit_2015_04_13 service handler
    pub fn codecommit_2015_04_13(&self) -> codecommit_2015_04_13::Codecommit_2015_04_13Service<'_> {
        codecommit_2015_04_13::Codecommit_2015_04_13Service::new(self)
    }
    /// Get lex_runtime service handler
    pub fn lex_runtime(&self) -> lex_runtime::Lex_runtimeService<'_> {
        lex_runtime::Lex_runtimeService::new(self)
    }
    /// Get pca_connector_ad_2018_05_10 service handler
    pub fn pca_connector_ad_2018_05_10(&self) -> pca_connector_ad_2018_05_10::Pca_connector_ad_2018_05_10Service<'_> {
        pca_connector_ad_2018_05_10::Pca_connector_ad_2018_05_10Service::new(self)
    }
    /// Get forecastquery_2018_06_26 service handler
    pub fn forecastquery_2018_06_26(&self) -> forecastquery_2018_06_26::Forecastquery_2018_06_26Service<'_> {
        forecastquery_2018_06_26::Forecastquery_2018_06_26Service::new(self)
    }
    /// Get healthlake_2017_07_01 service handler
    pub fn healthlake_2017_07_01(&self) -> healthlake_2017_07_01::Healthlake_2017_07_01Service<'_> {
        healthlake_2017_07_01::Healthlake_2017_07_01Service::new(self)
    }
    /// Get rolesanywhere_2018_05_10 service handler
    pub fn rolesanywhere_2018_05_10(&self) -> rolesanywhere_2018_05_10::Rolesanywhere_2018_05_10Service<'_> {
        rolesanywhere_2018_05_10::Rolesanywhere_2018_05_10Service::new(self)
    }
    /// Get marketplace_catalog_2018_09_17 service handler
    pub fn marketplace_catalog_2018_09_17(&self) -> marketplace_catalog_2018_09_17::Marketplace_catalog_2018_09_17Service<'_> {
        marketplace_catalog_2018_09_17::Marketplace_catalog_2018_09_17Service::new(self)
    }
    /// Get billing_2023_09_07 service handler
    pub fn billing_2023_09_07(&self) -> billing_2023_09_07::Billing_2023_09_07Service<'_> {
        billing_2023_09_07::Billing_2023_09_07Service::new(self)
    }
    /// Get emr_containers_2020_10_01 service handler
    pub fn emr_containers_2020_10_01(&self) -> emr_containers_2020_10_01::Emr_containers_2020_10_01Service<'_> {
        emr_containers_2020_10_01::Emr_containers_2020_10_01Service::new(self)
    }
    /// Get apigatewaymanagementapi_2018_11_29 service handler
    pub fn apigatewaymanagementapi_2018_11_29(&self) -> apigatewaymanagementapi_2018_11_29::Apigatewaymanagementapi_2018_11_29Service<'_> {
        apigatewaymanagementapi_2018_11_29::Apigatewaymanagementapi_2018_11_29Service::new(self)
    }
    /// Get xray_2016_04_12 service handler
    pub fn xray_2016_04_12(&self) -> xray_2016_04_12::Xray_2016_04_12Service<'_> {
        xray_2016_04_12::Xray_2016_04_12Service::new(self)
    }
    /// Get transcribe_streaming_2017_10_26 service handler
    pub fn transcribe_streaming_2017_10_26(&self) -> transcribe_streaming_2017_10_26::Transcribe_streaming_2017_10_26Service<'_> {
        transcribe_streaming_2017_10_26::Transcribe_streaming_2017_10_26Service::new(self)
    }
    /// Get ram_2018_01_04 service handler
    pub fn ram_2018_01_04(&self) -> ram_2018_01_04::Ram_2018_01_04Service<'_> {
        ram_2018_01_04::Ram_2018_01_04Service::new(self)
    }
    /// Get codeconnections_2023_12_01 service handler
    pub fn codeconnections_2023_12_01(&self) -> codeconnections_2023_12_01::Codeconnections_2023_12_01Service<'_> {
        codeconnections_2023_12_01::Codeconnections_2023_12_01Service::new(self)
    }
    /// Get efs_2015_02_01 service handler
    pub fn efs_2015_02_01(&self) -> efs_2015_02_01::Efs_2015_02_01Service<'_> {
        efs_2015_02_01::Efs_2015_02_01Service::new(self)
    }
    /// Get migration_hub_refactor_spaces_2021_10_26 service handler
    pub fn migration_hub_refactor_spaces_2021_10_26(&self) -> migration_hub_refactor_spaces_2021_10_26::Migration_hub_refactor_spaces_2021_10_26Service<'_> {
        migration_hub_refactor_spaces_2021_10_26::Migration_hub_refactor_spaces_2021_10_26Service::new(self)
    }
    /// Get elasticsearch_service_2015_01_01 service handler
    pub fn elasticsearch_service_2015_01_01(&self) -> elasticsearch_service_2015_01_01::Elasticsearch_service_2015_01_01Service<'_> {
        elasticsearch_service_2015_01_01::Elasticsearch_service_2015_01_01Service::new(self)
    }
    /// Get cognito_identity_2014_06_30 service handler
    pub fn cognito_identity_2014_06_30(&self) -> cognito_identity_2014_06_30::Cognito_identity_2014_06_30Service<'_> {
        cognito_identity_2014_06_30::Cognito_identity_2014_06_30Service::new(self)
    }
    /// Get payment_cryptography_data_2022_02_03 service handler
    pub fn payment_cryptography_data_2022_02_03(&self) -> payment_cryptography_data_2022_02_03::Payment_cryptography_data_2022_02_03Service<'_> {
        payment_cryptography_data_2022_02_03::Payment_cryptography_data_2022_02_03Service::new(self)
    }
    /// Get dataexchange_2017_07_25 service handler
    pub fn dataexchange_2017_07_25(&self) -> dataexchange_2017_07_25::Dataexchange_2017_07_25Service<'_> {
        dataexchange_2017_07_25::Dataexchange_2017_07_25Service::new(self)
    }
    /// Get sts_2011_06_15 service handler
    pub fn sts_2011_06_15(&self) -> sts_2011_06_15::Sts_2011_06_15Service<'_> {
        sts_2011_06_15::Sts_2011_06_15Service::new(self)
    }
    /// Get sagemaker_2017_07_24 service handler
    pub fn sagemaker_2017_07_24(&self) -> sagemaker_2017_07_24::Sagemaker_2017_07_24Service<'_> {
        sagemaker_2017_07_24::Sagemaker_2017_07_24Service::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get acm_2015_12_08 service handler
    pub fn acm_2015_12_08(&self) -> acm_2015_12_08::Acm_2015_12_08Service<'_> {
        acm_2015_12_08::Acm_2015_12_08Service::new(self)
    }
    /// Get athena_2017_05_18 service handler
    pub fn athena_2017_05_18(&self) -> athena_2017_05_18::Athena_2017_05_18Service<'_> {
        athena_2017_05_18::Athena_2017_05_18Service::new(self)
    }
    /// Get dsql_2018_05_10 service handler
    pub fn dsql_2018_05_10(&self) -> dsql_2018_05_10::Dsql_2018_05_10Service<'_> {
        dsql_2018_05_10::Dsql_2018_05_10Service::new(self)
    }
    /// Get mediapackage service handler
    pub fn mediapackage(&self) -> mediapackage::MediapackageService<'_> {
        mediapackage::MediapackageService::new(self)
    }
    /// Get tnb_2008_10_21 service handler
    pub fn tnb_2008_10_21(&self) -> tnb_2008_10_21::Tnb_2008_10_21Service<'_> {
        tnb_2008_10_21::Tnb_2008_10_21Service::new(self)
    }
    /// Get ec2_2016_11_15 service handler
    pub fn ec2_2016_11_15(&self) -> ec2_2016_11_15::Ec2_2016_11_15Service<'_> {
        ec2_2016_11_15::Ec2_2016_11_15Service::new(self)
    }
    /// Get apprunner_2020_05_15 service handler
    pub fn apprunner_2020_05_15(&self) -> apprunner_2020_05_15::Apprunner_2020_05_15Service<'_> {
        apprunner_2020_05_15::Apprunner_2020_05_15Service::new(self)
    }
    /// Get redshift_serverless_2021_04_21 service handler
    pub fn redshift_serverless_2021_04_21(&self) -> redshift_serverless_2021_04_21::Redshift_serverless_2021_04_21Service<'_> {
        redshift_serverless_2021_04_21::Redshift_serverless_2021_04_21Service::new(self)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_creation() {
        // Provider creation test
        // Note: This will fail without proper credentials
    }
}
