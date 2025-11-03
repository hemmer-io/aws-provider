//! Aws Provider for Hemmer
//!
//! Auto-generated unified provider from aws SDK version v1
//!
//! This provider includes multiple services:
//! - emr_serverless
//! - cloudformation
//! - application_auto_scaling
//! - personalize_events
//! - tnb
//! - rolesanywhere
//! - kms
//! - datasync
//! - bedrock_agent_runtime
//! - pinpoint_email
//! - connect_contact_lens
//! - athena
//! - iotfleetwise
//! - iot_data_plane
//! - bedrock_data_automation_runtime
//! - entityresolution
//! - forecastquery
//! - detective
//! - panorama
//! - backup
//! - mwaa
//! - iot_jobs_data_plane
//! - transcribe
//! - cloudwatch
//! - snow_device_management
//! - workmailmessageflow
//! - appconfig
//! - lightsail
//! - guardduty
//! - apigatewayv2
//! - wafv2
//! - iotsitewise
//! - iotthingsgraph
//! - batch
//! - mailmanager
//! - marketplace_reporting
//! - m2
//! - codedeploy
//! - route53_recovery_control_config
//! - simspaceweaver
//! - resiliencehub
//! - oam
//! - license_manager_linux_subscriptions
//! - voice_id
//! - chime
//! - efs
//! - freetier
//! - storage_gateway
//! - dynamodb_streams
//! - gamelift
//! - inspector2
//! - keyspaces
//! - sqs
//! - ram
//! - ssm_sap
//! - directory_service_data
//! - route_53_domains
//! - bedrock_agentcore
//! - trustedadvisor
//! - migrationhubstrategy
//! - dataexchange
//! - braket
//! - codebuild
//! - acm
//! - route53_recovery_cluster
//! - lookoutequipment
//! - marketplace_catalog
//! - payment_cryptography_data
//! - cloud9
//! - workdocs
//! - license_manager
//! - sts
//! - s3vectors
//! - chime_sdk_media_pipelines
//! - machine_learning
//! - timestream_query
//! - codeguru_reviewer
//! - mgn
//! - evidently
//! - qbusiness
//! - connectcases
//! - fsx
//! - ecr
//! - connectcampaignsv2
//! - rds
//! - qapps
//! - qconnect
//! - omics
//! - bcm_dashboards
//! - geo_routes
//! - quicksight
//! - amp
//! - opensearchserverless
//! - emr
//! - service_quotas
//! - service_catalog_appregistry
//! - migrationhub_config
//! - iam
//! - accessanalyzer
//! - appconfigdata
//! - route53resolver
//! - s3outposts
//! - kendra_ranking
//! - controltower
//! - arc_region_switch
//! - neptune_graph
//! - route53_recovery_readiness
//! - greengrassv2
//! - migration_hub_refactor_spaces
//! - cost_and_usage_report_service
//! - ebs
//! - appflow
//! - migrationhuborchestrator
//! - chime_sdk_identity
//! - cloudfront_keyvaluestore
//! - waf
//! - greengrass
//! - sagemaker_featurestore_runtime
//! - inspector
//! - appfabric
//! - lex_model_building_service
//! - serverlessapplicationrepository
//! - cloudsearch_domain
//! - codeguru_security
//! - socialmessaging
//! - geo_maps
//! - kinesis
//! - dsql
//! - appintegrations
//! - personalize
//! - proton
//! - cloudcontrol
//! - redshift
//! - geo_places
//! - elasticsearch_service
//! - bcm_recommended_actions
//! - invoicing
//! - apprunner
//! - sns
//! - textract
//! - workmail
//! - datazone
//! - rekognition
//! - ssm
//! - medical_imaging
//! - lex_models
//! - support
//! - signer
//! - partnercentral_selling
//! - comprehendmedical
//! - macie2
//! - redshift_data
//! - marketplace_agreement
//! - health
//! - odb
//! - resource_groups_tagging_api
//! - application_insights
//! - timestream_write
//! - pinpoint_sms
//! - mediapackagev2
//! - ec2
//! - cleanrooms
//! - healthlake
//! - sfn
//! - iottwinmaker
//! - cloudtrail
//! - iotdeviceadvisor
//! - ssm_incidents
//! - pcs
//! - support_app
//! - managedblockchain_query
//! - iot_events_data
//! - lex_runtime
//! - observabilityadmin
//! - applicationcostprofiler
//! - billingconductor
//! - artifact
//! - ecr_public
//! - connectparticipant
//! - rds_data
//! - internetmonitor
//! - route_53
//! - bedrock_runtime
//! - amplifybackend
//! - marketplace_deployment
//! - account
//! - snowball
//! - eventbridge
//! - auto_scaling_plans
//! - directory_service
//! - mediapackage
//! - ssm_quicksetup
//! - s3_control
//! - codecatalyst
//! - notificationscontacts
//! - mpa
//! - ec2_instance_connect
//! - sagemaker_geospatial
//! - notifications
//! - securitylake
//! - networkmonitor
//! - codeconnections
//! - app_mesh
//! - workspaces_thin_client
//! - finspace_data
//! - compute_optimizer
//! - secrets_manager
//! - mediastore
//! - ecs
//! - vpc_lattice
//! - auto_scaling
//! - resource_groups
//! - eks
//! - marketplace_entitlement_service
//! - database_migration_service
//! - security_ir
//! - inspector_scan
//! - global_accelerator
//! - kinesis_analytics
//! - neptunedata
//! - swf
//! - cloudwatch_logs
//! - connect
//! - glue
//! - cognito_identity_provider
//! - cloudwatch_events
//! - cost_explorer
//! - network_firewall
//! - firehose
//! - transfer
//! - marketplace_metering
//! - rbin
//! - timestream_influxdb
//! - iotanalytics
//! - ivs
//! - kafka
//! - sesv2
//! - kendra
//! - sagemaker_edge
//! - launch_wizard
//! - securityhub
//! - finspace
//! - keyspacesstreams
//! - cleanroomsml
//! - transcribe_streaming
//! - aiops
//! - service_catalog
//! - databrew
//! - codecommit
//! - resource_explorer_2
//! - acm_pca
//! - payment_cryptography
//! - mq
//! - api_gateway
//! - grafana
//! - glacier
//! - bedrock
//! - s3tables
//! - ivs_realtime
//! - medialive
//! - backupsearch
//! - networkflowmonitor
//! - elasticache
//! - fis
//! - cloudhsm
//! - cost_optimization_hub
//! - synthetics
//! - rum
//! - emr_containers
//! - sagemaker_a2i_runtime
//! - ssm_contacts
//! - bcm_data_exports
//! - opensearch
//! - dax
//! - neptune
//! - pricing
//! - location
//! - route53profiles
//! - lambda
//! - ivschat
//! - billing
//! - wisdom
//! - schemas
//! - bedrock_agentcore_control
//! - controlcatalog
//! - cloudsearch
//! - deadline
//! - managedblockchain
//! - amplify
//! - iotsecuretunneling
//! - connectcampaigns
//! - kafkaconnect
//! - mediaconvert
//! - data_pipeline
//! - codepipeline
//! - clouddirectory
//! - amplifyuibuilder
//! - rtbfabric
//! - memorydb
//! - iot
//! - marketplace_commerce_analytics
//! - frauddetector
//! - bedrock_data_automation
//! - elastic_load_balancing
//! - verifiedpermissions
//! - networkmanager
//! - devops_guru
//! - taxsettings
//! - workspaces_instances
//! - arc_zonal_shift
//! - elastic_transcoder
//! - fms
//! - imagebuilder
//! - chime_sdk
//! - groundstation
//! - forecast
//! - appstream
//! - chime_sdk_meetings
//! - comprehend
//! - redshift_serverless
//! - pinpoint
//! - pi
//! - gameliftstreams
//! - customer_profiles
//! - workspaces
//! - auditmanager
//! - docdb
//! - mturk
//! - cognito_identity
//! - dynamodb
//! - codeartifact
//! - organizations
//! - dlm
//! - sso
//! - osis
//! - migration_hub
//! - chatbot
//! - docdb_elastic
//! - supplychain
//! - ses
//! - repostspace
//! - mediastore_data
//! - bedrock_agent
//! - wellarchitected
//! - budgets
//! - mediatailor
//! - appsync
//! - ssm_guiconnect
//! - evs
//! - eks_auth
//! - chime_sdk_messaging
//! - mediaconnect
//! - identitystore
//! - bcm_pricing_calculator
//! - lakeformation
//! - xray
//! - cloudfront
//! - sagemaker_metrics
//! - sso_oidc
//! - sagemaker
//! - codestar_connections
//! - device_farm
//! - translate
//! - sagemaker_runtime
//! - b2bi
//! - savingsplans
//! - pipes
//! - config_service
//! - codeguruprofiler
//! - s3
//! - polly
//! - cognito_sync
//! - scheduler
//! - pca_connector_ad
//! - waf_regional
//! - apigatewaymanagementapi
//! - workspaces_web
//! - pca_connector_scep
//! - codestar_notifications
//! - direct_connect
//! - shield
//! - application_signals
//! - iot_managed_integrations
//! - iot_wireless
//! - iot_events
//! - backup_gateway
//! - sso_admin
//! - elastic_beanstalk
//! - drs
//! - personalize_runtime
//! - outposts
//! - license_manager_user_subscriptions
//! - cloudtrail_data
//! - lex_runtime_service


pub mod emr_serverless;
pub mod cloudformation;
pub mod application_auto_scaling;
pub mod personalize_events;
pub mod tnb;
pub mod rolesanywhere;
pub mod kms;
pub mod datasync;
pub mod bedrock_agent_runtime;
pub mod pinpoint_email;
pub mod connect_contact_lens;
pub mod athena;
pub mod iotfleetwise;
pub mod iot_data_plane;
pub mod bedrock_data_automation_runtime;
pub mod entityresolution;
pub mod forecastquery;
pub mod detective;
pub mod panorama;
pub mod backup;
pub mod mwaa;
pub mod iot_jobs_data_plane;
pub mod transcribe;
pub mod cloudwatch;
pub mod snow_device_management;
pub mod workmailmessageflow;
pub mod appconfig;
pub mod lightsail;
pub mod guardduty;
pub mod apigatewayv2;
pub mod wafv2;
pub mod iotsitewise;
pub mod iotthingsgraph;
pub mod batch;
pub mod mailmanager;
pub mod marketplace_reporting;
pub mod m2;
pub mod codedeploy;
pub mod route53_recovery_control_config;
pub mod simspaceweaver;
pub mod resiliencehub;
pub mod oam;
pub mod license_manager_linux_subscriptions;
pub mod voice_id;
pub mod chime;
pub mod efs;
pub mod freetier;
pub mod storage_gateway;
pub mod dynamodb_streams;
pub mod gamelift;
pub mod inspector2;
pub mod keyspaces;
pub mod sqs;
pub mod ram;
pub mod ssm_sap;
pub mod directory_service_data;
pub mod route_53_domains;
pub mod bedrock_agentcore;
pub mod trustedadvisor;
pub mod migrationhubstrategy;
pub mod dataexchange;
pub mod braket;
pub mod codebuild;
pub mod acm;
pub mod route53_recovery_cluster;
pub mod lookoutequipment;
pub mod marketplace_catalog;
pub mod payment_cryptography_data;
pub mod cloud9;
pub mod workdocs;
pub mod license_manager;
pub mod sts;
pub mod s3vectors;
pub mod chime_sdk_media_pipelines;
pub mod machine_learning;
pub mod timestream_query;
pub mod codeguru_reviewer;
pub mod mgn;
pub mod evidently;
pub mod qbusiness;
pub mod connectcases;
pub mod fsx;
pub mod ecr;
pub mod connectcampaignsv2;
pub mod rds;
pub mod qapps;
pub mod qconnect;
pub mod omics;
pub mod bcm_dashboards;
pub mod geo_routes;
pub mod quicksight;
pub mod amp;
pub mod opensearchserverless;
pub mod emr;
pub mod service_quotas;
pub mod service_catalog_appregistry;
pub mod migrationhub_config;
pub mod iam;
pub mod accessanalyzer;
pub mod appconfigdata;
pub mod route53resolver;
pub mod s3outposts;
pub mod kendra_ranking;
pub mod controltower;
pub mod arc_region_switch;
pub mod neptune_graph;
pub mod route53_recovery_readiness;
pub mod greengrassv2;
pub mod migration_hub_refactor_spaces;
pub mod cost_and_usage_report_service;
pub mod ebs;
pub mod appflow;
pub mod migrationhuborchestrator;
pub mod chime_sdk_identity;
pub mod cloudfront_keyvaluestore;
pub mod waf;
pub mod greengrass;
pub mod sagemaker_featurestore_runtime;
pub mod inspector;
pub mod appfabric;
pub mod lex_model_building_service;
pub mod serverlessapplicationrepository;
pub mod cloudsearch_domain;
pub mod codeguru_security;
pub mod socialmessaging;
pub mod geo_maps;
pub mod kinesis;
pub mod dsql;
pub mod appintegrations;
pub mod personalize;
pub mod proton;
pub mod cloudcontrol;
pub mod redshift;
pub mod geo_places;
pub mod elasticsearch_service;
pub mod bcm_recommended_actions;
pub mod invoicing;
pub mod apprunner;
pub mod sns;
pub mod textract;
pub mod workmail;
pub mod datazone;
pub mod rekognition;
pub mod ssm;
pub mod medical_imaging;
pub mod lex_models;
pub mod support;
pub mod signer;
pub mod partnercentral_selling;
pub mod comprehendmedical;
pub mod macie2;
pub mod redshift_data;
pub mod marketplace_agreement;
pub mod health;
pub mod odb;
pub mod resource_groups_tagging_api;
pub mod application_insights;
pub mod timestream_write;
pub mod pinpoint_sms;
pub mod mediapackagev2;
pub mod ec2;
pub mod cleanrooms;
pub mod healthlake;
pub mod sfn;
pub mod iottwinmaker;
pub mod cloudtrail;
pub mod iotdeviceadvisor;
pub mod ssm_incidents;
pub mod pcs;
pub mod support_app;
pub mod managedblockchain_query;
pub mod iot_events_data;
pub mod lex_runtime;
pub mod observabilityadmin;
pub mod applicationcostprofiler;
pub mod billingconductor;
pub mod artifact;
pub mod ecr_public;
pub mod connectparticipant;
pub mod rds_data;
pub mod internetmonitor;
pub mod route_53;
pub mod bedrock_runtime;
pub mod amplifybackend;
pub mod marketplace_deployment;
pub mod account;
pub mod snowball;
pub mod eventbridge;
pub mod auto_scaling_plans;
pub mod directory_service;
pub mod mediapackage;
pub mod ssm_quicksetup;
pub mod s3_control;
pub mod codecatalyst;
pub mod notificationscontacts;
pub mod mpa;
pub mod ec2_instance_connect;
pub mod sagemaker_geospatial;
pub mod notifications;
pub mod securitylake;
pub mod networkmonitor;
pub mod codeconnections;
pub mod app_mesh;
pub mod workspaces_thin_client;
pub mod finspace_data;
pub mod compute_optimizer;
pub mod secrets_manager;
pub mod mediastore;
pub mod ecs;
pub mod vpc_lattice;
pub mod auto_scaling;
pub mod resource_groups;
pub mod eks;
pub mod marketplace_entitlement_service;
pub mod database_migration_service;
pub mod security_ir;
pub mod inspector_scan;
pub mod global_accelerator;
pub mod kinesis_analytics;
pub mod neptunedata;
pub mod swf;
pub mod cloudwatch_logs;
pub mod connect;
pub mod glue;
pub mod cognito_identity_provider;
pub mod cloudwatch_events;
pub mod cost_explorer;
pub mod network_firewall;
pub mod firehose;
pub mod transfer;
pub mod marketplace_metering;
pub mod rbin;
pub mod timestream_influxdb;
pub mod iotanalytics;
pub mod ivs;
pub mod kafka;
pub mod sesv2;
pub mod kendra;
pub mod sagemaker_edge;
pub mod launch_wizard;
pub mod securityhub;
pub mod finspace;
pub mod keyspacesstreams;
pub mod cleanroomsml;
pub mod transcribe_streaming;
pub mod aiops;
pub mod service_catalog;
pub mod databrew;
pub mod codecommit;
pub mod resource_explorer_2;
pub mod acm_pca;
pub mod payment_cryptography;
pub mod mq;
pub mod api_gateway;
pub mod grafana;
pub mod glacier;
pub mod bedrock;
pub mod s3tables;
pub mod ivs_realtime;
pub mod medialive;
pub mod backupsearch;
pub mod networkflowmonitor;
pub mod elasticache;
pub mod fis;
pub mod cloudhsm;
pub mod cost_optimization_hub;
pub mod synthetics;
pub mod rum;
pub mod emr_containers;
pub mod sagemaker_a2i_runtime;
pub mod ssm_contacts;
pub mod bcm_data_exports;
pub mod opensearch;
pub mod dax;
pub mod neptune;
pub mod pricing;
pub mod location;
pub mod route53profiles;
pub mod lambda;
pub mod ivschat;
pub mod billing;
pub mod wisdom;
pub mod schemas;
pub mod bedrock_agentcore_control;
pub mod controlcatalog;
pub mod cloudsearch;
pub mod deadline;
pub mod managedblockchain;
pub mod amplify;
pub mod iotsecuretunneling;
pub mod connectcampaigns;
pub mod kafkaconnect;
pub mod mediaconvert;
pub mod data_pipeline;
pub mod codepipeline;
pub mod clouddirectory;
pub mod amplifyuibuilder;
pub mod rtbfabric;
pub mod memorydb;
pub mod iot;
pub mod marketplace_commerce_analytics;
pub mod frauddetector;
pub mod bedrock_data_automation;
pub mod elastic_load_balancing;
pub mod verifiedpermissions;
pub mod networkmanager;
pub mod devops_guru;
pub mod taxsettings;
pub mod workspaces_instances;
pub mod arc_zonal_shift;
pub mod elastic_transcoder;
pub mod fms;
pub mod imagebuilder;
pub mod chime_sdk;
pub mod groundstation;
pub mod forecast;
pub mod appstream;
pub mod chime_sdk_meetings;
pub mod comprehend;
pub mod redshift_serverless;
pub mod pinpoint;
pub mod pi;
pub mod gameliftstreams;
pub mod customer_profiles;
pub mod workspaces;
pub mod auditmanager;
pub mod docdb;
pub mod mturk;
pub mod cognito_identity;
pub mod dynamodb;
pub mod codeartifact;
pub mod organizations;
pub mod dlm;
pub mod sso;
pub mod osis;
pub mod migration_hub;
pub mod chatbot;
pub mod docdb_elastic;
pub mod supplychain;
pub mod ses;
pub mod repostspace;
pub mod mediastore_data;
pub mod bedrock_agent;
pub mod wellarchitected;
pub mod budgets;
pub mod mediatailor;
pub mod appsync;
pub mod ssm_guiconnect;
pub mod evs;
pub mod eks_auth;
pub mod chime_sdk_messaging;
pub mod mediaconnect;
pub mod identitystore;
pub mod bcm_pricing_calculator;
pub mod lakeformation;
pub mod xray;
pub mod cloudfront;
pub mod sagemaker_metrics;
pub mod sso_oidc;
pub mod sagemaker;
pub mod codestar_connections;
pub mod device_farm;
pub mod translate;
pub mod sagemaker_runtime;
pub mod b2bi;
pub mod savingsplans;
pub mod pipes;
pub mod config_service;
pub mod codeguruprofiler;
pub mod s3;
pub mod polly;
pub mod cognito_sync;
pub mod scheduler;
pub mod pca_connector_ad;
pub mod waf_regional;
pub mod apigatewaymanagementapi;
pub mod workspaces_web;
pub mod pca_connector_scep;
pub mod codestar_notifications;
pub mod direct_connect;
pub mod shield;
pub mod application_signals;
pub mod iot_managed_integrations;
pub mod iot_wireless;
pub mod iot_events;
pub mod backup_gateway;
pub mod sso_admin;
pub mod elastic_beanstalk;
pub mod drs;
pub mod personalize_runtime;
pub mod outposts;
pub mod license_manager_user_subscriptions;
pub mod cloudtrail_data;
pub mod lex_runtime_service;


use async_trait::async_trait;
use hemmer_core::Result;
use hemmer_provider::{ProviderConfig, ProviderExecutor, ResourceInput, ResourceOutput, ResourcePlan};
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
    emr_serverless_client: aws_sdk_emr_serverless::Client,
    cloudformation_client: aws_sdk_cloudformation::Client,
    application_auto_scaling_client: aws_sdk_application_auto_scaling::Client,
    personalize_events_client: aws_sdk_personalize_events::Client,
    tnb_client: aws_sdk_tnb::Client,
    rolesanywhere_client: aws_sdk_rolesanywhere::Client,
    kms_client: aws_sdk_kms::Client,
    datasync_client: aws_sdk_datasync::Client,
    bedrock_agent_runtime_client: aws_sdk_bedrock_agent_runtime::Client,
    pinpoint_email_client: aws_sdk_pinpoint_email::Client,
    connect_contact_lens_client: aws_sdk_connect_contact_lens::Client,
    athena_client: aws_sdk_athena::Client,
    iotfleetwise_client: aws_sdk_iotfleetwise::Client,
    iot_data_plane_client: aws_sdk_iot_data_plane::Client,
    bedrock_data_automation_runtime_client: aws_sdk_bedrock_data_automation_runtime::Client,
    entityresolution_client: aws_sdk_entityresolution::Client,
    forecastquery_client: aws_sdk_forecastquery::Client,
    detective_client: aws_sdk_detective::Client,
    panorama_client: aws_sdk_panorama::Client,
    backup_client: aws_sdk_backup::Client,
    mwaa_client: aws_sdk_mwaa::Client,
    iot_jobs_data_plane_client: aws_sdk_iot_jobs_data_plane::Client,
    transcribe_client: aws_sdk_transcribe::Client,
    cloudwatch_client: aws_sdk_cloudwatch::Client,
    snow_device_management_client: aws_sdk_snow_device_management::Client,
    workmailmessageflow_client: aws_sdk_workmailmessageflow::Client,
    appconfig_client: aws_sdk_appconfig::Client,
    lightsail_client: aws_sdk_lightsail::Client,
    guardduty_client: aws_sdk_guardduty::Client,
    apigatewayv2_client: aws_sdk_apigatewayv2::Client,
    wafv2_client: aws_sdk_wafv2::Client,
    iotsitewise_client: aws_sdk_iotsitewise::Client,
    iotthingsgraph_client: aws_sdk_iotthingsgraph::Client,
    batch_client: aws_sdk_batch::Client,
    mailmanager_client: aws_sdk_mailmanager::Client,
    marketplace_reporting_client: aws_sdk_marketplace_reporting::Client,
    m2_client: aws_sdk_m2::Client,
    codedeploy_client: aws_sdk_codedeploy::Client,
    route53_recovery_control_config_client: aws_sdk_route53_recovery_control_config::Client,
    simspaceweaver_client: aws_sdk_simspaceweaver::Client,
    resiliencehub_client: aws_sdk_resiliencehub::Client,
    oam_client: aws_sdk_oam::Client,
    license_manager_linux_subscriptions_client: aws_sdk_license_manager_linux_subscriptions::Client,
    voice_id_client: aws_sdk_voice_id::Client,
    chime_client: aws_sdk_chime::Client,
    efs_client: aws_sdk_efs::Client,
    freetier_client: aws_sdk_freetier::Client,
    storage_gateway_client: aws_sdk_storage_gateway::Client,
    dynamodb_streams_client: aws_sdk_dynamodb_streams::Client,
    gamelift_client: aws_sdk_gamelift::Client,
    inspector2_client: aws_sdk_inspector2::Client,
    keyspaces_client: aws_sdk_keyspaces::Client,
    sqs_client: aws_sdk_sqs::Client,
    ram_client: aws_sdk_ram::Client,
    ssm_sap_client: aws_sdk_ssm_sap::Client,
    directory_service_data_client: aws_sdk_directory_service_data::Client,
    route_53_domains_client: aws_sdk_route_53_domains::Client,
    bedrock_agentcore_client: aws_sdk_bedrock_agentcore::Client,
    trustedadvisor_client: aws_sdk_trustedadvisor::Client,
    migrationhubstrategy_client: aws_sdk_migrationhubstrategy::Client,
    dataexchange_client: aws_sdk_dataexchange::Client,
    braket_client: aws_sdk_braket::Client,
    codebuild_client: aws_sdk_codebuild::Client,
    acm_client: aws_sdk_acm::Client,
    route53_recovery_cluster_client: aws_sdk_route53_recovery_cluster::Client,
    lookoutequipment_client: aws_sdk_lookoutequipment::Client,
    marketplace_catalog_client: aws_sdk_marketplace_catalog::Client,
    payment_cryptography_data_client: aws_sdk_payment_cryptography_data::Client,
    cloud9_client: aws_sdk_cloud9::Client,
    workdocs_client: aws_sdk_workdocs::Client,
    license_manager_client: aws_sdk_license_manager::Client,
    sts_client: aws_sdk_sts::Client,
    s3vectors_client: aws_sdk_s3vectors::Client,
    chime_sdk_media_pipelines_client: aws_sdk_chime_sdk_media_pipelines::Client,
    machine_learning_client: aws_sdk_machine_learning::Client,
    timestream_query_client: aws_sdk_timestream_query::Client,
    codeguru_reviewer_client: aws_sdk_codeguru_reviewer::Client,
    mgn_client: aws_sdk_mgn::Client,
    evidently_client: aws_sdk_evidently::Client,
    qbusiness_client: aws_sdk_qbusiness::Client,
    connectcases_client: aws_sdk_connectcases::Client,
    fsx_client: aws_sdk_fsx::Client,
    ecr_client: aws_sdk_ecr::Client,
    connectcampaignsv2_client: aws_sdk_connectcampaignsv2::Client,
    rds_client: aws_sdk_rds::Client,
    qapps_client: aws_sdk_qapps::Client,
    qconnect_client: aws_sdk_qconnect::Client,
    omics_client: aws_sdk_omics::Client,
    bcm_dashboards_client: aws_sdk_bcm_dashboards::Client,
    geo_routes_client: aws_sdk_geo_routes::Client,
    quicksight_client: aws_sdk_quicksight::Client,
    amp_client: aws_sdk_amp::Client,
    opensearchserverless_client: aws_sdk_opensearchserverless::Client,
    emr_client: aws_sdk_emr::Client,
    service_quotas_client: aws_sdk_service_quotas::Client,
    service_catalog_appregistry_client: aws_sdk_service_catalog_appregistry::Client,
    migrationhub_config_client: aws_sdk_migrationhub_config::Client,
    iam_client: aws_sdk_iam::Client,
    accessanalyzer_client: aws_sdk_accessanalyzer::Client,
    appconfigdata_client: aws_sdk_appconfigdata::Client,
    route53resolver_client: aws_sdk_route53resolver::Client,
    s3outposts_client: aws_sdk_s3outposts::Client,
    kendra_ranking_client: aws_sdk_kendra_ranking::Client,
    controltower_client: aws_sdk_controltower::Client,
    arc_region_switch_client: aws_sdk_arc_region_switch::Client,
    neptune_graph_client: aws_sdk_neptune_graph::Client,
    route53_recovery_readiness_client: aws_sdk_route53_recovery_readiness::Client,
    greengrassv2_client: aws_sdk_greengrassv2::Client,
    migration_hub_refactor_spaces_client: aws_sdk_migration_hub_refactor_spaces::Client,
    cost_and_usage_report_service_client: aws_sdk_cost_and_usage_report_service::Client,
    ebs_client: aws_sdk_ebs::Client,
    appflow_client: aws_sdk_appflow::Client,
    migrationhuborchestrator_client: aws_sdk_migrationhuborchestrator::Client,
    chime_sdk_identity_client: aws_sdk_chime_sdk_identity::Client,
    cloudfront_keyvaluestore_client: aws_sdk_cloudfront_keyvaluestore::Client,
    waf_client: aws_sdk_waf::Client,
    greengrass_client: aws_sdk_greengrass::Client,
    sagemaker_featurestore_runtime_client: aws_sdk_sagemaker_featurestore_runtime::Client,
    inspector_client: aws_sdk_inspector::Client,
    appfabric_client: aws_sdk_appfabric::Client,
    lex_model_building_service_client: aws_sdk_lex_model_building_service::Client,
    serverlessapplicationrepository_client: aws_sdk_serverlessapplicationrepository::Client,
    cloudsearch_domain_client: aws_sdk_cloudsearch_domain::Client,
    codeguru_security_client: aws_sdk_codeguru_security::Client,
    socialmessaging_client: aws_sdk_socialmessaging::Client,
    geo_maps_client: aws_sdk_geo_maps::Client,
    kinesis_client: aws_sdk_kinesis::Client,
    dsql_client: aws_sdk_dsql::Client,
    appintegrations_client: aws_sdk_appintegrations::Client,
    personalize_client: aws_sdk_personalize::Client,
    proton_client: aws_sdk_proton::Client,
    cloudcontrol_client: aws_sdk_cloudcontrol::Client,
    redshift_client: aws_sdk_redshift::Client,
    geo_places_client: aws_sdk_geo_places::Client,
    elasticsearch_service_client: aws_sdk_elasticsearch_service::Client,
    bcm_recommended_actions_client: aws_sdk_bcm_recommended_actions::Client,
    invoicing_client: aws_sdk_invoicing::Client,
    apprunner_client: aws_sdk_apprunner::Client,
    sns_client: aws_sdk_sns::Client,
    textract_client: aws_sdk_textract::Client,
    workmail_client: aws_sdk_workmail::Client,
    datazone_client: aws_sdk_datazone::Client,
    rekognition_client: aws_sdk_rekognition::Client,
    ssm_client: aws_sdk_ssm::Client,
    medical_imaging_client: aws_sdk_medical_imaging::Client,
    lex_models_client: aws_sdk_lex_models::Client,
    support_client: aws_sdk_support::Client,
    signer_client: aws_sdk_signer::Client,
    partnercentral_selling_client: aws_sdk_partnercentral_selling::Client,
    comprehendmedical_client: aws_sdk_comprehendmedical::Client,
    macie2_client: aws_sdk_macie2::Client,
    redshift_data_client: aws_sdk_redshift_data::Client,
    marketplace_agreement_client: aws_sdk_marketplace_agreement::Client,
    health_client: aws_sdk_health::Client,
    odb_client: aws_sdk_odb::Client,
    resource_groups_tagging_api_client: aws_sdk_resource_groups_tagging_api::Client,
    application_insights_client: aws_sdk_application_insights::Client,
    timestream_write_client: aws_sdk_timestream_write::Client,
    pinpoint_sms_client: aws_sdk_pinpoint_sms::Client,
    mediapackagev2_client: aws_sdk_mediapackagev2::Client,
    ec2_client: aws_sdk_ec2::Client,
    cleanrooms_client: aws_sdk_cleanrooms::Client,
    healthlake_client: aws_sdk_healthlake::Client,
    sfn_client: aws_sdk_sfn::Client,
    iottwinmaker_client: aws_sdk_iottwinmaker::Client,
    cloudtrail_client: aws_sdk_cloudtrail::Client,
    iotdeviceadvisor_client: aws_sdk_iotdeviceadvisor::Client,
    ssm_incidents_client: aws_sdk_ssm_incidents::Client,
    pcs_client: aws_sdk_pcs::Client,
    support_app_client: aws_sdk_support_app::Client,
    managedblockchain_query_client: aws_sdk_managedblockchain_query::Client,
    iot_events_data_client: aws_sdk_iot_events_data::Client,
    lex_runtime_client: aws_sdk_lex_runtime::Client,
    observabilityadmin_client: aws_sdk_observabilityadmin::Client,
    applicationcostprofiler_client: aws_sdk_applicationcostprofiler::Client,
    billingconductor_client: aws_sdk_billingconductor::Client,
    artifact_client: aws_sdk_artifact::Client,
    ecr_public_client: aws_sdk_ecr_public::Client,
    connectparticipant_client: aws_sdk_connectparticipant::Client,
    rds_data_client: aws_sdk_rds_data::Client,
    internetmonitor_client: aws_sdk_internetmonitor::Client,
    route_53_client: aws_sdk_route_53::Client,
    bedrock_runtime_client: aws_sdk_bedrock_runtime::Client,
    amplifybackend_client: aws_sdk_amplifybackend::Client,
    marketplace_deployment_client: aws_sdk_marketplace_deployment::Client,
    account_client: aws_sdk_account::Client,
    snowball_client: aws_sdk_snowball::Client,
    eventbridge_client: aws_sdk_eventbridge::Client,
    auto_scaling_plans_client: aws_sdk_auto_scaling_plans::Client,
    directory_service_client: aws_sdk_directory_service::Client,
    mediapackage_client: aws_sdk_mediapackage::Client,
    ssm_quicksetup_client: aws_sdk_ssm_quicksetup::Client,
    s3_control_client: aws_sdk_s3_control::Client,
    codecatalyst_client: aws_sdk_codecatalyst::Client,
    notificationscontacts_client: aws_sdk_notificationscontacts::Client,
    mpa_client: aws_sdk_mpa::Client,
    ec2_instance_connect_client: aws_sdk_ec2_instance_connect::Client,
    sagemaker_geospatial_client: aws_sdk_sagemaker_geospatial::Client,
    notifications_client: aws_sdk_notifications::Client,
    securitylake_client: aws_sdk_securitylake::Client,
    networkmonitor_client: aws_sdk_networkmonitor::Client,
    codeconnections_client: aws_sdk_codeconnections::Client,
    app_mesh_client: aws_sdk_app_mesh::Client,
    workspaces_thin_client_client: aws_sdk_workspaces_thin_client::Client,
    finspace_data_client: aws_sdk_finspace_data::Client,
    compute_optimizer_client: aws_sdk_compute_optimizer::Client,
    secrets_manager_client: aws_sdk_secrets_manager::Client,
    mediastore_client: aws_sdk_mediastore::Client,
    ecs_client: aws_sdk_ecs::Client,
    vpc_lattice_client: aws_sdk_vpc_lattice::Client,
    auto_scaling_client: aws_sdk_auto_scaling::Client,
    resource_groups_client: aws_sdk_resource_groups::Client,
    eks_client: aws_sdk_eks::Client,
    marketplace_entitlement_service_client: aws_sdk_marketplace_entitlement_service::Client,
    database_migration_service_client: aws_sdk_database_migration_service::Client,
    security_ir_client: aws_sdk_security_ir::Client,
    inspector_scan_client: aws_sdk_inspector_scan::Client,
    global_accelerator_client: aws_sdk_global_accelerator::Client,
    kinesis_analytics_client: aws_sdk_kinesis_analytics::Client,
    neptunedata_client: aws_sdk_neptunedata::Client,
    swf_client: aws_sdk_swf::Client,
    cloudwatch_logs_client: aws_sdk_cloudwatch_logs::Client,
    connect_client: aws_sdk_connect::Client,
    glue_client: aws_sdk_glue::Client,
    cognito_identity_provider_client: aws_sdk_cognito_identity_provider::Client,
    cloudwatch_events_client: aws_sdk_cloudwatch_events::Client,
    cost_explorer_client: aws_sdk_cost_explorer::Client,
    network_firewall_client: aws_sdk_network_firewall::Client,
    firehose_client: aws_sdk_firehose::Client,
    transfer_client: aws_sdk_transfer::Client,
    marketplace_metering_client: aws_sdk_marketplace_metering::Client,
    rbin_client: aws_sdk_rbin::Client,
    timestream_influxdb_client: aws_sdk_timestream_influxdb::Client,
    iotanalytics_client: aws_sdk_iotanalytics::Client,
    ivs_client: aws_sdk_ivs::Client,
    kafka_client: aws_sdk_kafka::Client,
    sesv2_client: aws_sdk_sesv2::Client,
    kendra_client: aws_sdk_kendra::Client,
    sagemaker_edge_client: aws_sdk_sagemaker_edge::Client,
    launch_wizard_client: aws_sdk_launch_wizard::Client,
    securityhub_client: aws_sdk_securityhub::Client,
    finspace_client: aws_sdk_finspace::Client,
    keyspacesstreams_client: aws_sdk_keyspacesstreams::Client,
    cleanroomsml_client: aws_sdk_cleanroomsml::Client,
    transcribe_streaming_client: aws_sdk_transcribe_streaming::Client,
    aiops_client: aws_sdk_aiops::Client,
    service_catalog_client: aws_sdk_service_catalog::Client,
    databrew_client: aws_sdk_databrew::Client,
    codecommit_client: aws_sdk_codecommit::Client,
    resource_explorer_2_client: aws_sdk_resource_explorer_2::Client,
    acm_pca_client: aws_sdk_acm_pca::Client,
    payment_cryptography_client: aws_sdk_payment_cryptography::Client,
    mq_client: aws_sdk_mq::Client,
    api_gateway_client: aws_sdk_api_gateway::Client,
    grafana_client: aws_sdk_grafana::Client,
    glacier_client: aws_sdk_glacier::Client,
    bedrock_client: aws_sdk_bedrock::Client,
    s3tables_client: aws_sdk_s3tables::Client,
    ivs_realtime_client: aws_sdk_ivs_realtime::Client,
    medialive_client: aws_sdk_medialive::Client,
    backupsearch_client: aws_sdk_backupsearch::Client,
    networkflowmonitor_client: aws_sdk_networkflowmonitor::Client,
    elasticache_client: aws_sdk_elasticache::Client,
    fis_client: aws_sdk_fis::Client,
    cloudhsm_client: aws_sdk_cloudhsm::Client,
    cost_optimization_hub_client: aws_sdk_cost_optimization_hub::Client,
    synthetics_client: aws_sdk_synthetics::Client,
    rum_client: aws_sdk_rum::Client,
    emr_containers_client: aws_sdk_emr_containers::Client,
    sagemaker_a2i_runtime_client: aws_sdk_sagemaker_a2i_runtime::Client,
    ssm_contacts_client: aws_sdk_ssm_contacts::Client,
    bcm_data_exports_client: aws_sdk_bcm_data_exports::Client,
    opensearch_client: aws_sdk_opensearch::Client,
    dax_client: aws_sdk_dax::Client,
    neptune_client: aws_sdk_neptune::Client,
    pricing_client: aws_sdk_pricing::Client,
    location_client: aws_sdk_location::Client,
    route53profiles_client: aws_sdk_route53profiles::Client,
    lambda_client: aws_sdk_lambda::Client,
    ivschat_client: aws_sdk_ivschat::Client,
    billing_client: aws_sdk_billing::Client,
    wisdom_client: aws_sdk_wisdom::Client,
    schemas_client: aws_sdk_schemas::Client,
    bedrock_agentcore_control_client: aws_sdk_bedrock_agentcore_control::Client,
    controlcatalog_client: aws_sdk_controlcatalog::Client,
    cloudsearch_client: aws_sdk_cloudsearch::Client,
    deadline_client: aws_sdk_deadline::Client,
    managedblockchain_client: aws_sdk_managedblockchain::Client,
    amplify_client: aws_sdk_amplify::Client,
    iotsecuretunneling_client: aws_sdk_iotsecuretunneling::Client,
    connectcampaigns_client: aws_sdk_connectcampaigns::Client,
    kafkaconnect_client: aws_sdk_kafkaconnect::Client,
    mediaconvert_client: aws_sdk_mediaconvert::Client,
    data_pipeline_client: aws_sdk_data_pipeline::Client,
    codepipeline_client: aws_sdk_codepipeline::Client,
    clouddirectory_client: aws_sdk_clouddirectory::Client,
    amplifyuibuilder_client: aws_sdk_amplifyuibuilder::Client,
    rtbfabric_client: aws_sdk_rtbfabric::Client,
    memorydb_client: aws_sdk_memorydb::Client,
    iot_client: aws_sdk_iot::Client,
    marketplace_commerce_analytics_client: aws_sdk_marketplace_commerce_analytics::Client,
    frauddetector_client: aws_sdk_frauddetector::Client,
    bedrock_data_automation_client: aws_sdk_bedrock_data_automation::Client,
    elastic_load_balancing_client: aws_sdk_elastic_load_balancing::Client,
    verifiedpermissions_client: aws_sdk_verifiedpermissions::Client,
    networkmanager_client: aws_sdk_networkmanager::Client,
    devops_guru_client: aws_sdk_devops_guru::Client,
    taxsettings_client: aws_sdk_taxsettings::Client,
    workspaces_instances_client: aws_sdk_workspaces_instances::Client,
    arc_zonal_shift_client: aws_sdk_arc_zonal_shift::Client,
    elastic_transcoder_client: aws_sdk_elastic_transcoder::Client,
    fms_client: aws_sdk_fms::Client,
    imagebuilder_client: aws_sdk_imagebuilder::Client,
    chime_sdk_client: aws_sdk_chime_sdk::Client,
    groundstation_client: aws_sdk_groundstation::Client,
    forecast_client: aws_sdk_forecast::Client,
    appstream_client: aws_sdk_appstream::Client,
    chime_sdk_meetings_client: aws_sdk_chime_sdk_meetings::Client,
    comprehend_client: aws_sdk_comprehend::Client,
    redshift_serverless_client: aws_sdk_redshift_serverless::Client,
    pinpoint_client: aws_sdk_pinpoint::Client,
    pi_client: aws_sdk_pi::Client,
    gameliftstreams_client: aws_sdk_gameliftstreams::Client,
    customer_profiles_client: aws_sdk_customer_profiles::Client,
    workspaces_client: aws_sdk_workspaces::Client,
    auditmanager_client: aws_sdk_auditmanager::Client,
    docdb_client: aws_sdk_docdb::Client,
    mturk_client: aws_sdk_mturk::Client,
    cognito_identity_client: aws_sdk_cognito_identity::Client,
    dynamodb_client: aws_sdk_dynamodb::Client,
    codeartifact_client: aws_sdk_codeartifact::Client,
    organizations_client: aws_sdk_organizations::Client,
    dlm_client: aws_sdk_dlm::Client,
    sso_client: aws_sdk_sso::Client,
    osis_client: aws_sdk_osis::Client,
    migration_hub_client: aws_sdk_migration_hub::Client,
    chatbot_client: aws_sdk_chatbot::Client,
    docdb_elastic_client: aws_sdk_docdb_elastic::Client,
    supplychain_client: aws_sdk_supplychain::Client,
    ses_client: aws_sdk_ses::Client,
    repostspace_client: aws_sdk_repostspace::Client,
    mediastore_data_client: aws_sdk_mediastore_data::Client,
    bedrock_agent_client: aws_sdk_bedrock_agent::Client,
    wellarchitected_client: aws_sdk_wellarchitected::Client,
    budgets_client: aws_sdk_budgets::Client,
    mediatailor_client: aws_sdk_mediatailor::Client,
    appsync_client: aws_sdk_appsync::Client,
    ssm_guiconnect_client: aws_sdk_ssm_guiconnect::Client,
    evs_client: aws_sdk_evs::Client,
    eks_auth_client: aws_sdk_eks_auth::Client,
    chime_sdk_messaging_client: aws_sdk_chime_sdk_messaging::Client,
    mediaconnect_client: aws_sdk_mediaconnect::Client,
    identitystore_client: aws_sdk_identitystore::Client,
    bcm_pricing_calculator_client: aws_sdk_bcm_pricing_calculator::Client,
    lakeformation_client: aws_sdk_lakeformation::Client,
    xray_client: aws_sdk_xray::Client,
    cloudfront_client: aws_sdk_cloudfront::Client,
    sagemaker_metrics_client: aws_sdk_sagemaker_metrics::Client,
    sso_oidc_client: aws_sdk_sso_oidc::Client,
    sagemaker_client: aws_sdk_sagemaker::Client,
    codestar_connections_client: aws_sdk_codestar_connections::Client,
    device_farm_client: aws_sdk_device_farm::Client,
    translate_client: aws_sdk_translate::Client,
    sagemaker_runtime_client: aws_sdk_sagemaker_runtime::Client,
    b2bi_client: aws_sdk_b2bi::Client,
    savingsplans_client: aws_sdk_savingsplans::Client,
    pipes_client: aws_sdk_pipes::Client,
    config_service_client: aws_sdk_config_service::Client,
    codeguruprofiler_client: aws_sdk_codeguruprofiler::Client,
    s3_client: aws_sdk_s3::Client,
    polly_client: aws_sdk_polly::Client,
    cognito_sync_client: aws_sdk_cognito_sync::Client,
    scheduler_client: aws_sdk_scheduler::Client,
    pca_connector_ad_client: aws_sdk_pca_connector_ad::Client,
    waf_regional_client: aws_sdk_waf_regional::Client,
    apigatewaymanagementapi_client: aws_sdk_apigatewaymanagementapi::Client,
    workspaces_web_client: aws_sdk_workspaces_web::Client,
    pca_connector_scep_client: aws_sdk_pca_connector_scep::Client,
    codestar_notifications_client: aws_sdk_codestar_notifications::Client,
    direct_connect_client: aws_sdk_direct_connect::Client,
    shield_client: aws_sdk_shield::Client,
    application_signals_client: aws_sdk_application_signals::Client,
    iot_managed_integrations_client: aws_sdk_iot_managed_integrations::Client,
    iot_wireless_client: aws_sdk_iot_wireless::Client,
    iot_events_client: aws_sdk_iot_events::Client,
    backup_gateway_client: aws_sdk_backup_gateway::Client,
    sso_admin_client: aws_sdk_sso_admin::Client,
    elastic_beanstalk_client: aws_sdk_elastic_beanstalk::Client,
    drs_client: aws_sdk_drs::Client,
    personalize_runtime_client: aws_sdk_personalize_runtime::Client,
    outposts_client: aws_sdk_outposts::Client,
    license_manager_user_subscriptions_client: aws_sdk_license_manager_user_subscriptions::Client,
    cloudtrail_data_client: aws_sdk_cloudtrail_data::Client,
    lex_runtime_service_client: aws_sdk_lex_runtime_service::Client,
    /// Tokio runtime for async operations
    /// Each provider instance owns its own runtime to ensure async operations
    /// work correctly when loaded as a dynamic library (cdylib)
    runtime: tokio::runtime::Runtime,

}

impl AwsProvider {
    /// Create a new unified provider instance
    pub fn new() -> Result<Self> {
        // Create Tokio runtime for async operations
        // This ensures async AWS SDK calls work when the provider is loaded as a dynamic library
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| ProviderError::SdkError(format!("Failed to create Tokio runtime: {}", e)))?;

        // Load AWS config and initialize clients using the runtime
        let (config, emr_serverless_client, cloudformation_client, application_auto_scaling_client, personalize_events_client, tnb_client, rolesanywhere_client, kms_client, datasync_client, bedrock_agent_runtime_client, pinpoint_email_client, connect_contact_lens_client, athena_client, iotfleetwise_client, iot_data_plane_client, bedrock_data_automation_runtime_client, entityresolution_client, forecastquery_client, detective_client, panorama_client, backup_client, mwaa_client, iot_jobs_data_plane_client, transcribe_client, cloudwatch_client, snow_device_management_client, workmailmessageflow_client, appconfig_client, lightsail_client, guardduty_client, apigatewayv2_client, wafv2_client, iotsitewise_client, iotthingsgraph_client, batch_client, mailmanager_client, marketplace_reporting_client, m2_client, codedeploy_client, route53_recovery_control_config_client, simspaceweaver_client, resiliencehub_client, oam_client, license_manager_linux_subscriptions_client, voice_id_client, chime_client, efs_client, freetier_client, storage_gateway_client, dynamodb_streams_client, gamelift_client, inspector2_client, keyspaces_client, sqs_client, ram_client, ssm_sap_client, directory_service_data_client, route_53_domains_client, bedrock_agentcore_client, trustedadvisor_client, migrationhubstrategy_client, dataexchange_client, braket_client, codebuild_client, acm_client, route53_recovery_cluster_client, lookoutequipment_client, marketplace_catalog_client, payment_cryptography_data_client, cloud9_client, workdocs_client, license_manager_client, sts_client, s3vectors_client, chime_sdk_media_pipelines_client, machine_learning_client, timestream_query_client, codeguru_reviewer_client, mgn_client, evidently_client, qbusiness_client, connectcases_client, fsx_client, ecr_client, connectcampaignsv2_client, rds_client, qapps_client, qconnect_client, omics_client, bcm_dashboards_client, geo_routes_client, quicksight_client, amp_client, opensearchserverless_client, emr_client, service_quotas_client, service_catalog_appregistry_client, migrationhub_config_client, iam_client, accessanalyzer_client, appconfigdata_client, route53resolver_client, s3outposts_client, kendra_ranking_client, controltower_client, arc_region_switch_client, neptune_graph_client, route53_recovery_readiness_client, greengrassv2_client, migration_hub_refactor_spaces_client, cost_and_usage_report_service_client, ebs_client, appflow_client, migrationhuborchestrator_client, chime_sdk_identity_client, cloudfront_keyvaluestore_client, waf_client, greengrass_client, sagemaker_featurestore_runtime_client, inspector_client, appfabric_client, lex_model_building_service_client, serverlessapplicationrepository_client, cloudsearch_domain_client, codeguru_security_client, socialmessaging_client, geo_maps_client, kinesis_client, dsql_client, appintegrations_client, personalize_client, proton_client, cloudcontrol_client, redshift_client, geo_places_client, elasticsearch_service_client, bcm_recommended_actions_client, invoicing_client, apprunner_client, sns_client, textract_client, workmail_client, datazone_client, rekognition_client, ssm_client, medical_imaging_client, lex_models_client, support_client, signer_client, partnercentral_selling_client, comprehendmedical_client, macie2_client, redshift_data_client, marketplace_agreement_client, health_client, odb_client, resource_groups_tagging_api_client, application_insights_client, timestream_write_client, pinpoint_sms_client, mediapackagev2_client, ec2_client, cleanrooms_client, healthlake_client, sfn_client, iottwinmaker_client, cloudtrail_client, iotdeviceadvisor_client, ssm_incidents_client, pcs_client, support_app_client, managedblockchain_query_client, iot_events_data_client, lex_runtime_client, observabilityadmin_client, applicationcostprofiler_client, billingconductor_client, artifact_client, ecr_public_client, connectparticipant_client, rds_data_client, internetmonitor_client, route_53_client, bedrock_runtime_client, amplifybackend_client, marketplace_deployment_client, account_client, snowball_client, eventbridge_client, auto_scaling_plans_client, directory_service_client, mediapackage_client, ssm_quicksetup_client, s3_control_client, codecatalyst_client, notificationscontacts_client, mpa_client, ec2_instance_connect_client, sagemaker_geospatial_client, notifications_client, securitylake_client, networkmonitor_client, codeconnections_client, app_mesh_client, workspaces_thin_client_client, finspace_data_client, compute_optimizer_client, secrets_manager_client, mediastore_client, ecs_client, vpc_lattice_client, auto_scaling_client, resource_groups_client, eks_client, marketplace_entitlement_service_client, database_migration_service_client, security_ir_client, inspector_scan_client, global_accelerator_client, kinesis_analytics_client, neptunedata_client, swf_client, cloudwatch_logs_client, connect_client, glue_client, cognito_identity_provider_client, cloudwatch_events_client, cost_explorer_client, network_firewall_client, firehose_client, transfer_client, marketplace_metering_client, rbin_client, timestream_influxdb_client, iotanalytics_client, ivs_client, kafka_client, sesv2_client, kendra_client, sagemaker_edge_client, launch_wizard_client, securityhub_client, finspace_client, keyspacesstreams_client, cleanroomsml_client, transcribe_streaming_client, aiops_client, service_catalog_client, databrew_client, codecommit_client, resource_explorer_2_client, acm_pca_client, payment_cryptography_client, mq_client, api_gateway_client, grafana_client, glacier_client, bedrock_client, s3tables_client, ivs_realtime_client, medialive_client, backupsearch_client, networkflowmonitor_client, elasticache_client, fis_client, cloudhsm_client, cost_optimization_hub_client, synthetics_client, rum_client, emr_containers_client, sagemaker_a2i_runtime_client, ssm_contacts_client, bcm_data_exports_client, opensearch_client, dax_client, neptune_client, pricing_client, location_client, route53profiles_client, lambda_client, ivschat_client, billing_client, wisdom_client, schemas_client, bedrock_agentcore_control_client, controlcatalog_client, cloudsearch_client, deadline_client, managedblockchain_client, amplify_client, iotsecuretunneling_client, connectcampaigns_client, kafkaconnect_client, mediaconvert_client, data_pipeline_client, codepipeline_client, clouddirectory_client, amplifyuibuilder_client, rtbfabric_client, memorydb_client, iot_client, marketplace_commerce_analytics_client, frauddetector_client, bedrock_data_automation_client, elastic_load_balancing_client, verifiedpermissions_client, networkmanager_client, devops_guru_client, taxsettings_client, workspaces_instances_client, arc_zonal_shift_client, elastic_transcoder_client, fms_client, imagebuilder_client, chime_sdk_client, groundstation_client, forecast_client, appstream_client, chime_sdk_meetings_client, comprehend_client, redshift_serverless_client, pinpoint_client, pi_client, gameliftstreams_client, customer_profiles_client, workspaces_client, auditmanager_client, docdb_client, mturk_client, cognito_identity_client, dynamodb_client, codeartifact_client, organizations_client, dlm_client, sso_client, osis_client, migration_hub_client, chatbot_client, docdb_elastic_client, supplychain_client, ses_client, repostspace_client, mediastore_data_client, bedrock_agent_client, wellarchitected_client, budgets_client, mediatailor_client, appsync_client, ssm_guiconnect_client, evs_client, eks_auth_client, chime_sdk_messaging_client, mediaconnect_client, identitystore_client, bcm_pricing_calculator_client, lakeformation_client, xray_client, cloudfront_client, sagemaker_metrics_client, sso_oidc_client, sagemaker_client, codestar_connections_client, device_farm_client, translate_client, sagemaker_runtime_client, b2bi_client, savingsplans_client, pipes_client, config_service_client, codeguruprofiler_client, s3_client, polly_client, cognito_sync_client, scheduler_client, pca_connector_ad_client, waf_regional_client, apigatewaymanagementapi_client, workspaces_web_client, pca_connector_scep_client, codestar_notifications_client, direct_connect_client, shield_client, application_signals_client, iot_managed_integrations_client, iot_wireless_client, iot_events_client, backup_gateway_client, sso_admin_client, elastic_beanstalk_client, drs_client, personalize_runtime_client, outposts_client, license_manager_user_subscriptions_client, cloudtrail_data_client, lex_runtime_service_client) = runtime.block_on(async {
            let config = aws_config::load_from_env().await;
            let emr_serverless_client = aws_sdk_emr_serverless::Client::new(&config);
            let cloudformation_client = aws_sdk_cloudformation::Client::new(&config);
            let application_auto_scaling_client = aws_sdk_application_auto_scaling::Client::new(&config);
            let personalize_events_client = aws_sdk_personalize_events::Client::new(&config);
            let tnb_client = aws_sdk_tnb::Client::new(&config);
            let rolesanywhere_client = aws_sdk_rolesanywhere::Client::new(&config);
            let kms_client = aws_sdk_kms::Client::new(&config);
            let datasync_client = aws_sdk_datasync::Client::new(&config);
            let bedrock_agent_runtime_client = aws_sdk_bedrock_agent_runtime::Client::new(&config);
            let pinpoint_email_client = aws_sdk_pinpoint_email::Client::new(&config);
            let connect_contact_lens_client = aws_sdk_connect_contact_lens::Client::new(&config);
            let athena_client = aws_sdk_athena::Client::new(&config);
            let iotfleetwise_client = aws_sdk_iotfleetwise::Client::new(&config);
            let iot_data_plane_client = aws_sdk_iot_data_plane::Client::new(&config);
            let bedrock_data_automation_runtime_client = aws_sdk_bedrock_data_automation_runtime::Client::new(&config);
            let entityresolution_client = aws_sdk_entityresolution::Client::new(&config);
            let forecastquery_client = aws_sdk_forecastquery::Client::new(&config);
            let detective_client = aws_sdk_detective::Client::new(&config);
            let panorama_client = aws_sdk_panorama::Client::new(&config);
            let backup_client = aws_sdk_backup::Client::new(&config);
            let mwaa_client = aws_sdk_mwaa::Client::new(&config);
            let iot_jobs_data_plane_client = aws_sdk_iot_jobs_data_plane::Client::new(&config);
            let transcribe_client = aws_sdk_transcribe::Client::new(&config);
            let cloudwatch_client = aws_sdk_cloudwatch::Client::new(&config);
            let snow_device_management_client = aws_sdk_snow_device_management::Client::new(&config);
            let workmailmessageflow_client = aws_sdk_workmailmessageflow::Client::new(&config);
            let appconfig_client = aws_sdk_appconfig::Client::new(&config);
            let lightsail_client = aws_sdk_lightsail::Client::new(&config);
            let guardduty_client = aws_sdk_guardduty::Client::new(&config);
            let apigatewayv2_client = aws_sdk_apigatewayv2::Client::new(&config);
            let wafv2_client = aws_sdk_wafv2::Client::new(&config);
            let iotsitewise_client = aws_sdk_iotsitewise::Client::new(&config);
            let iotthingsgraph_client = aws_sdk_iotthingsgraph::Client::new(&config);
            let batch_client = aws_sdk_batch::Client::new(&config);
            let mailmanager_client = aws_sdk_mailmanager::Client::new(&config);
            let marketplace_reporting_client = aws_sdk_marketplace_reporting::Client::new(&config);
            let m2_client = aws_sdk_m2::Client::new(&config);
            let codedeploy_client = aws_sdk_codedeploy::Client::new(&config);
            let route53_recovery_control_config_client = aws_sdk_route53_recovery_control_config::Client::new(&config);
            let simspaceweaver_client = aws_sdk_simspaceweaver::Client::new(&config);
            let resiliencehub_client = aws_sdk_resiliencehub::Client::new(&config);
            let oam_client = aws_sdk_oam::Client::new(&config);
            let license_manager_linux_subscriptions_client = aws_sdk_license_manager_linux_subscriptions::Client::new(&config);
            let voice_id_client = aws_sdk_voice_id::Client::new(&config);
            let chime_client = aws_sdk_chime::Client::new(&config);
            let efs_client = aws_sdk_efs::Client::new(&config);
            let freetier_client = aws_sdk_freetier::Client::new(&config);
            let storage_gateway_client = aws_sdk_storage_gateway::Client::new(&config);
            let dynamodb_streams_client = aws_sdk_dynamodb_streams::Client::new(&config);
            let gamelift_client = aws_sdk_gamelift::Client::new(&config);
            let inspector2_client = aws_sdk_inspector2::Client::new(&config);
            let keyspaces_client = aws_sdk_keyspaces::Client::new(&config);
            let sqs_client = aws_sdk_sqs::Client::new(&config);
            let ram_client = aws_sdk_ram::Client::new(&config);
            let ssm_sap_client = aws_sdk_ssm_sap::Client::new(&config);
            let directory_service_data_client = aws_sdk_directory_service_data::Client::new(&config);
            let route_53_domains_client = aws_sdk_route_53_domains::Client::new(&config);
            let bedrock_agentcore_client = aws_sdk_bedrock_agentcore::Client::new(&config);
            let trustedadvisor_client = aws_sdk_trustedadvisor::Client::new(&config);
            let migrationhubstrategy_client = aws_sdk_migrationhubstrategy::Client::new(&config);
            let dataexchange_client = aws_sdk_dataexchange::Client::new(&config);
            let braket_client = aws_sdk_braket::Client::new(&config);
            let codebuild_client = aws_sdk_codebuild::Client::new(&config);
            let acm_client = aws_sdk_acm::Client::new(&config);
            let route53_recovery_cluster_client = aws_sdk_route53_recovery_cluster::Client::new(&config);
            let lookoutequipment_client = aws_sdk_lookoutequipment::Client::new(&config);
            let marketplace_catalog_client = aws_sdk_marketplace_catalog::Client::new(&config);
            let payment_cryptography_data_client = aws_sdk_payment_cryptography_data::Client::new(&config);
            let cloud9_client = aws_sdk_cloud9::Client::new(&config);
            let workdocs_client = aws_sdk_workdocs::Client::new(&config);
            let license_manager_client = aws_sdk_license_manager::Client::new(&config);
            let sts_client = aws_sdk_sts::Client::new(&config);
            let s3vectors_client = aws_sdk_s3vectors::Client::new(&config);
            let chime_sdk_media_pipelines_client = aws_sdk_chime_sdk_media_pipelines::Client::new(&config);
            let machine_learning_client = aws_sdk_machine_learning::Client::new(&config);
            let timestream_query_client = aws_sdk_timestream_query::Client::new(&config);
            let codeguru_reviewer_client = aws_sdk_codeguru_reviewer::Client::new(&config);
            let mgn_client = aws_sdk_mgn::Client::new(&config);
            let evidently_client = aws_sdk_evidently::Client::new(&config);
            let qbusiness_client = aws_sdk_qbusiness::Client::new(&config);
            let connectcases_client = aws_sdk_connectcases::Client::new(&config);
            let fsx_client = aws_sdk_fsx::Client::new(&config);
            let ecr_client = aws_sdk_ecr::Client::new(&config);
            let connectcampaignsv2_client = aws_sdk_connectcampaignsv2::Client::new(&config);
            let rds_client = aws_sdk_rds::Client::new(&config);
            let qapps_client = aws_sdk_qapps::Client::new(&config);
            let qconnect_client = aws_sdk_qconnect::Client::new(&config);
            let omics_client = aws_sdk_omics::Client::new(&config);
            let bcm_dashboards_client = aws_sdk_bcm_dashboards::Client::new(&config);
            let geo_routes_client = aws_sdk_geo_routes::Client::new(&config);
            let quicksight_client = aws_sdk_quicksight::Client::new(&config);
            let amp_client = aws_sdk_amp::Client::new(&config);
            let opensearchserverless_client = aws_sdk_opensearchserverless::Client::new(&config);
            let emr_client = aws_sdk_emr::Client::new(&config);
            let service_quotas_client = aws_sdk_service_quotas::Client::new(&config);
            let service_catalog_appregistry_client = aws_sdk_service_catalog_appregistry::Client::new(&config);
            let migrationhub_config_client = aws_sdk_migrationhub_config::Client::new(&config);
            let iam_client = aws_sdk_iam::Client::new(&config);
            let accessanalyzer_client = aws_sdk_accessanalyzer::Client::new(&config);
            let appconfigdata_client = aws_sdk_appconfigdata::Client::new(&config);
            let route53resolver_client = aws_sdk_route53resolver::Client::new(&config);
            let s3outposts_client = aws_sdk_s3outposts::Client::new(&config);
            let kendra_ranking_client = aws_sdk_kendra_ranking::Client::new(&config);
            let controltower_client = aws_sdk_controltower::Client::new(&config);
            let arc_region_switch_client = aws_sdk_arc_region_switch::Client::new(&config);
            let neptune_graph_client = aws_sdk_neptune_graph::Client::new(&config);
            let route53_recovery_readiness_client = aws_sdk_route53_recovery_readiness::Client::new(&config);
            let greengrassv2_client = aws_sdk_greengrassv2::Client::new(&config);
            let migration_hub_refactor_spaces_client = aws_sdk_migration_hub_refactor_spaces::Client::new(&config);
            let cost_and_usage_report_service_client = aws_sdk_cost_and_usage_report_service::Client::new(&config);
            let ebs_client = aws_sdk_ebs::Client::new(&config);
            let appflow_client = aws_sdk_appflow::Client::new(&config);
            let migrationhuborchestrator_client = aws_sdk_migrationhuborchestrator::Client::new(&config);
            let chime_sdk_identity_client = aws_sdk_chime_sdk_identity::Client::new(&config);
            let cloudfront_keyvaluestore_client = aws_sdk_cloudfront_keyvaluestore::Client::new(&config);
            let waf_client = aws_sdk_waf::Client::new(&config);
            let greengrass_client = aws_sdk_greengrass::Client::new(&config);
            let sagemaker_featurestore_runtime_client = aws_sdk_sagemaker_featurestore_runtime::Client::new(&config);
            let inspector_client = aws_sdk_inspector::Client::new(&config);
            let appfabric_client = aws_sdk_appfabric::Client::new(&config);
            let lex_model_building_service_client = aws_sdk_lex_model_building_service::Client::new(&config);
            let serverlessapplicationrepository_client = aws_sdk_serverlessapplicationrepository::Client::new(&config);
            let cloudsearch_domain_client = aws_sdk_cloudsearch_domain::Client::new(&config);
            let codeguru_security_client = aws_sdk_codeguru_security::Client::new(&config);
            let socialmessaging_client = aws_sdk_socialmessaging::Client::new(&config);
            let geo_maps_client = aws_sdk_geo_maps::Client::new(&config);
            let kinesis_client = aws_sdk_kinesis::Client::new(&config);
            let dsql_client = aws_sdk_dsql::Client::new(&config);
            let appintegrations_client = aws_sdk_appintegrations::Client::new(&config);
            let personalize_client = aws_sdk_personalize::Client::new(&config);
            let proton_client = aws_sdk_proton::Client::new(&config);
            let cloudcontrol_client = aws_sdk_cloudcontrol::Client::new(&config);
            let redshift_client = aws_sdk_redshift::Client::new(&config);
            let geo_places_client = aws_sdk_geo_places::Client::new(&config);
            let elasticsearch_service_client = aws_sdk_elasticsearch_service::Client::new(&config);
            let bcm_recommended_actions_client = aws_sdk_bcm_recommended_actions::Client::new(&config);
            let invoicing_client = aws_sdk_invoicing::Client::new(&config);
            let apprunner_client = aws_sdk_apprunner::Client::new(&config);
            let sns_client = aws_sdk_sns::Client::new(&config);
            let textract_client = aws_sdk_textract::Client::new(&config);
            let workmail_client = aws_sdk_workmail::Client::new(&config);
            let datazone_client = aws_sdk_datazone::Client::new(&config);
            let rekognition_client = aws_sdk_rekognition::Client::new(&config);
            let ssm_client = aws_sdk_ssm::Client::new(&config);
            let medical_imaging_client = aws_sdk_medical_imaging::Client::new(&config);
            let lex_models_client = aws_sdk_lex_models::Client::new(&config);
            let support_client = aws_sdk_support::Client::new(&config);
            let signer_client = aws_sdk_signer::Client::new(&config);
            let partnercentral_selling_client = aws_sdk_partnercentral_selling::Client::new(&config);
            let comprehendmedical_client = aws_sdk_comprehendmedical::Client::new(&config);
            let macie2_client = aws_sdk_macie2::Client::new(&config);
            let redshift_data_client = aws_sdk_redshift_data::Client::new(&config);
            let marketplace_agreement_client = aws_sdk_marketplace_agreement::Client::new(&config);
            let health_client = aws_sdk_health::Client::new(&config);
            let odb_client = aws_sdk_odb::Client::new(&config);
            let resource_groups_tagging_api_client = aws_sdk_resource_groups_tagging_api::Client::new(&config);
            let application_insights_client = aws_sdk_application_insights::Client::new(&config);
            let timestream_write_client = aws_sdk_timestream_write::Client::new(&config);
            let pinpoint_sms_client = aws_sdk_pinpoint_sms::Client::new(&config);
            let mediapackagev2_client = aws_sdk_mediapackagev2::Client::new(&config);
            let ec2_client = aws_sdk_ec2::Client::new(&config);
            let cleanrooms_client = aws_sdk_cleanrooms::Client::new(&config);
            let healthlake_client = aws_sdk_healthlake::Client::new(&config);
            let sfn_client = aws_sdk_sfn::Client::new(&config);
            let iottwinmaker_client = aws_sdk_iottwinmaker::Client::new(&config);
            let cloudtrail_client = aws_sdk_cloudtrail::Client::new(&config);
            let iotdeviceadvisor_client = aws_sdk_iotdeviceadvisor::Client::new(&config);
            let ssm_incidents_client = aws_sdk_ssm_incidents::Client::new(&config);
            let pcs_client = aws_sdk_pcs::Client::new(&config);
            let support_app_client = aws_sdk_support_app::Client::new(&config);
            let managedblockchain_query_client = aws_sdk_managedblockchain_query::Client::new(&config);
            let iot_events_data_client = aws_sdk_iot_events_data::Client::new(&config);
            let lex_runtime_client = aws_sdk_lex_runtime::Client::new(&config);
            let observabilityadmin_client = aws_sdk_observabilityadmin::Client::new(&config);
            let applicationcostprofiler_client = aws_sdk_applicationcostprofiler::Client::new(&config);
            let billingconductor_client = aws_sdk_billingconductor::Client::new(&config);
            let artifact_client = aws_sdk_artifact::Client::new(&config);
            let ecr_public_client = aws_sdk_ecr_public::Client::new(&config);
            let connectparticipant_client = aws_sdk_connectparticipant::Client::new(&config);
            let rds_data_client = aws_sdk_rds_data::Client::new(&config);
            let internetmonitor_client = aws_sdk_internetmonitor::Client::new(&config);
            let route_53_client = aws_sdk_route_53::Client::new(&config);
            let bedrock_runtime_client = aws_sdk_bedrock_runtime::Client::new(&config);
            let amplifybackend_client = aws_sdk_amplifybackend::Client::new(&config);
            let marketplace_deployment_client = aws_sdk_marketplace_deployment::Client::new(&config);
            let account_client = aws_sdk_account::Client::new(&config);
            let snowball_client = aws_sdk_snowball::Client::new(&config);
            let eventbridge_client = aws_sdk_eventbridge::Client::new(&config);
            let auto_scaling_plans_client = aws_sdk_auto_scaling_plans::Client::new(&config);
            let directory_service_client = aws_sdk_directory_service::Client::new(&config);
            let mediapackage_client = aws_sdk_mediapackage::Client::new(&config);
            let ssm_quicksetup_client = aws_sdk_ssm_quicksetup::Client::new(&config);
            let s3_control_client = aws_sdk_s3_control::Client::new(&config);
            let codecatalyst_client = aws_sdk_codecatalyst::Client::new(&config);
            let notificationscontacts_client = aws_sdk_notificationscontacts::Client::new(&config);
            let mpa_client = aws_sdk_mpa::Client::new(&config);
            let ec2_instance_connect_client = aws_sdk_ec2_instance_connect::Client::new(&config);
            let sagemaker_geospatial_client = aws_sdk_sagemaker_geospatial::Client::new(&config);
            let notifications_client = aws_sdk_notifications::Client::new(&config);
            let securitylake_client = aws_sdk_securitylake::Client::new(&config);
            let networkmonitor_client = aws_sdk_networkmonitor::Client::new(&config);
            let codeconnections_client = aws_sdk_codeconnections::Client::new(&config);
            let app_mesh_client = aws_sdk_app_mesh::Client::new(&config);
            let workspaces_thin_client_client = aws_sdk_workspaces_thin_client::Client::new(&config);
            let finspace_data_client = aws_sdk_finspace_data::Client::new(&config);
            let compute_optimizer_client = aws_sdk_compute_optimizer::Client::new(&config);
            let secrets_manager_client = aws_sdk_secrets_manager::Client::new(&config);
            let mediastore_client = aws_sdk_mediastore::Client::new(&config);
            let ecs_client = aws_sdk_ecs::Client::new(&config);
            let vpc_lattice_client = aws_sdk_vpc_lattice::Client::new(&config);
            let auto_scaling_client = aws_sdk_auto_scaling::Client::new(&config);
            let resource_groups_client = aws_sdk_resource_groups::Client::new(&config);
            let eks_client = aws_sdk_eks::Client::new(&config);
            let marketplace_entitlement_service_client = aws_sdk_marketplace_entitlement_service::Client::new(&config);
            let database_migration_service_client = aws_sdk_database_migration_service::Client::new(&config);
            let security_ir_client = aws_sdk_security_ir::Client::new(&config);
            let inspector_scan_client = aws_sdk_inspector_scan::Client::new(&config);
            let global_accelerator_client = aws_sdk_global_accelerator::Client::new(&config);
            let kinesis_analytics_client = aws_sdk_kinesis_analytics::Client::new(&config);
            let neptunedata_client = aws_sdk_neptunedata::Client::new(&config);
            let swf_client = aws_sdk_swf::Client::new(&config);
            let cloudwatch_logs_client = aws_sdk_cloudwatch_logs::Client::new(&config);
            let connect_client = aws_sdk_connect::Client::new(&config);
            let glue_client = aws_sdk_glue::Client::new(&config);
            let cognito_identity_provider_client = aws_sdk_cognito_identity_provider::Client::new(&config);
            let cloudwatch_events_client = aws_sdk_cloudwatch_events::Client::new(&config);
            let cost_explorer_client = aws_sdk_cost_explorer::Client::new(&config);
            let network_firewall_client = aws_sdk_network_firewall::Client::new(&config);
            let firehose_client = aws_sdk_firehose::Client::new(&config);
            let transfer_client = aws_sdk_transfer::Client::new(&config);
            let marketplace_metering_client = aws_sdk_marketplace_metering::Client::new(&config);
            let rbin_client = aws_sdk_rbin::Client::new(&config);
            let timestream_influxdb_client = aws_sdk_timestream_influxdb::Client::new(&config);
            let iotanalytics_client = aws_sdk_iotanalytics::Client::new(&config);
            let ivs_client = aws_sdk_ivs::Client::new(&config);
            let kafka_client = aws_sdk_kafka::Client::new(&config);
            let sesv2_client = aws_sdk_sesv2::Client::new(&config);
            let kendra_client = aws_sdk_kendra::Client::new(&config);
            let sagemaker_edge_client = aws_sdk_sagemaker_edge::Client::new(&config);
            let launch_wizard_client = aws_sdk_launch_wizard::Client::new(&config);
            let securityhub_client = aws_sdk_securityhub::Client::new(&config);
            let finspace_client = aws_sdk_finspace::Client::new(&config);
            let keyspacesstreams_client = aws_sdk_keyspacesstreams::Client::new(&config);
            let cleanroomsml_client = aws_sdk_cleanroomsml::Client::new(&config);
            let transcribe_streaming_client = aws_sdk_transcribe_streaming::Client::new(&config);
            let aiops_client = aws_sdk_aiops::Client::new(&config);
            let service_catalog_client = aws_sdk_service_catalog::Client::new(&config);
            let databrew_client = aws_sdk_databrew::Client::new(&config);
            let codecommit_client = aws_sdk_codecommit::Client::new(&config);
            let resource_explorer_2_client = aws_sdk_resource_explorer_2::Client::new(&config);
            let acm_pca_client = aws_sdk_acm_pca::Client::new(&config);
            let payment_cryptography_client = aws_sdk_payment_cryptography::Client::new(&config);
            let mq_client = aws_sdk_mq::Client::new(&config);
            let api_gateway_client = aws_sdk_api_gateway::Client::new(&config);
            let grafana_client = aws_sdk_grafana::Client::new(&config);
            let glacier_client = aws_sdk_glacier::Client::new(&config);
            let bedrock_client = aws_sdk_bedrock::Client::new(&config);
            let s3tables_client = aws_sdk_s3tables::Client::new(&config);
            let ivs_realtime_client = aws_sdk_ivs_realtime::Client::new(&config);
            let medialive_client = aws_sdk_medialive::Client::new(&config);
            let backupsearch_client = aws_sdk_backupsearch::Client::new(&config);
            let networkflowmonitor_client = aws_sdk_networkflowmonitor::Client::new(&config);
            let elasticache_client = aws_sdk_elasticache::Client::new(&config);
            let fis_client = aws_sdk_fis::Client::new(&config);
            let cloudhsm_client = aws_sdk_cloudhsm::Client::new(&config);
            let cost_optimization_hub_client = aws_sdk_cost_optimization_hub::Client::new(&config);
            let synthetics_client = aws_sdk_synthetics::Client::new(&config);
            let rum_client = aws_sdk_rum::Client::new(&config);
            let emr_containers_client = aws_sdk_emr_containers::Client::new(&config);
            let sagemaker_a2i_runtime_client = aws_sdk_sagemaker_a2i_runtime::Client::new(&config);
            let ssm_contacts_client = aws_sdk_ssm_contacts::Client::new(&config);
            let bcm_data_exports_client = aws_sdk_bcm_data_exports::Client::new(&config);
            let opensearch_client = aws_sdk_opensearch::Client::new(&config);
            let dax_client = aws_sdk_dax::Client::new(&config);
            let neptune_client = aws_sdk_neptune::Client::new(&config);
            let pricing_client = aws_sdk_pricing::Client::new(&config);
            let location_client = aws_sdk_location::Client::new(&config);
            let route53profiles_client = aws_sdk_route53profiles::Client::new(&config);
            let lambda_client = aws_sdk_lambda::Client::new(&config);
            let ivschat_client = aws_sdk_ivschat::Client::new(&config);
            let billing_client = aws_sdk_billing::Client::new(&config);
            let wisdom_client = aws_sdk_wisdom::Client::new(&config);
            let schemas_client = aws_sdk_schemas::Client::new(&config);
            let bedrock_agentcore_control_client = aws_sdk_bedrock_agentcore_control::Client::new(&config);
            let controlcatalog_client = aws_sdk_controlcatalog::Client::new(&config);
            let cloudsearch_client = aws_sdk_cloudsearch::Client::new(&config);
            let deadline_client = aws_sdk_deadline::Client::new(&config);
            let managedblockchain_client = aws_sdk_managedblockchain::Client::new(&config);
            let amplify_client = aws_sdk_amplify::Client::new(&config);
            let iotsecuretunneling_client = aws_sdk_iotsecuretunneling::Client::new(&config);
            let connectcampaigns_client = aws_sdk_connectcampaigns::Client::new(&config);
            let kafkaconnect_client = aws_sdk_kafkaconnect::Client::new(&config);
            let mediaconvert_client = aws_sdk_mediaconvert::Client::new(&config);
            let data_pipeline_client = aws_sdk_data_pipeline::Client::new(&config);
            let codepipeline_client = aws_sdk_codepipeline::Client::new(&config);
            let clouddirectory_client = aws_sdk_clouddirectory::Client::new(&config);
            let amplifyuibuilder_client = aws_sdk_amplifyuibuilder::Client::new(&config);
            let rtbfabric_client = aws_sdk_rtbfabric::Client::new(&config);
            let memorydb_client = aws_sdk_memorydb::Client::new(&config);
            let iot_client = aws_sdk_iot::Client::new(&config);
            let marketplace_commerce_analytics_client = aws_sdk_marketplace_commerce_analytics::Client::new(&config);
            let frauddetector_client = aws_sdk_frauddetector::Client::new(&config);
            let bedrock_data_automation_client = aws_sdk_bedrock_data_automation::Client::new(&config);
            let elastic_load_balancing_client = aws_sdk_elastic_load_balancing::Client::new(&config);
            let verifiedpermissions_client = aws_sdk_verifiedpermissions::Client::new(&config);
            let networkmanager_client = aws_sdk_networkmanager::Client::new(&config);
            let devops_guru_client = aws_sdk_devops_guru::Client::new(&config);
            let taxsettings_client = aws_sdk_taxsettings::Client::new(&config);
            let workspaces_instances_client = aws_sdk_workspaces_instances::Client::new(&config);
            let arc_zonal_shift_client = aws_sdk_arc_zonal_shift::Client::new(&config);
            let elastic_transcoder_client = aws_sdk_elastic_transcoder::Client::new(&config);
            let fms_client = aws_sdk_fms::Client::new(&config);
            let imagebuilder_client = aws_sdk_imagebuilder::Client::new(&config);
            let chime_sdk_client = aws_sdk_chime_sdk::Client::new(&config);
            let groundstation_client = aws_sdk_groundstation::Client::new(&config);
            let forecast_client = aws_sdk_forecast::Client::new(&config);
            let appstream_client = aws_sdk_appstream::Client::new(&config);
            let chime_sdk_meetings_client = aws_sdk_chime_sdk_meetings::Client::new(&config);
            let comprehend_client = aws_sdk_comprehend::Client::new(&config);
            let redshift_serverless_client = aws_sdk_redshift_serverless::Client::new(&config);
            let pinpoint_client = aws_sdk_pinpoint::Client::new(&config);
            let pi_client = aws_sdk_pi::Client::new(&config);
            let gameliftstreams_client = aws_sdk_gameliftstreams::Client::new(&config);
            let customer_profiles_client = aws_sdk_customer_profiles::Client::new(&config);
            let workspaces_client = aws_sdk_workspaces::Client::new(&config);
            let auditmanager_client = aws_sdk_auditmanager::Client::new(&config);
            let docdb_client = aws_sdk_docdb::Client::new(&config);
            let mturk_client = aws_sdk_mturk::Client::new(&config);
            let cognito_identity_client = aws_sdk_cognito_identity::Client::new(&config);
            let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
            let codeartifact_client = aws_sdk_codeartifact::Client::new(&config);
            let organizations_client = aws_sdk_organizations::Client::new(&config);
            let dlm_client = aws_sdk_dlm::Client::new(&config);
            let sso_client = aws_sdk_sso::Client::new(&config);
            let osis_client = aws_sdk_osis::Client::new(&config);
            let migration_hub_client = aws_sdk_migration_hub::Client::new(&config);
            let chatbot_client = aws_sdk_chatbot::Client::new(&config);
            let docdb_elastic_client = aws_sdk_docdb_elastic::Client::new(&config);
            let supplychain_client = aws_sdk_supplychain::Client::new(&config);
            let ses_client = aws_sdk_ses::Client::new(&config);
            let repostspace_client = aws_sdk_repostspace::Client::new(&config);
            let mediastore_data_client = aws_sdk_mediastore_data::Client::new(&config);
            let bedrock_agent_client = aws_sdk_bedrock_agent::Client::new(&config);
            let wellarchitected_client = aws_sdk_wellarchitected::Client::new(&config);
            let budgets_client = aws_sdk_budgets::Client::new(&config);
            let mediatailor_client = aws_sdk_mediatailor::Client::new(&config);
            let appsync_client = aws_sdk_appsync::Client::new(&config);
            let ssm_guiconnect_client = aws_sdk_ssm_guiconnect::Client::new(&config);
            let evs_client = aws_sdk_evs::Client::new(&config);
            let eks_auth_client = aws_sdk_eks_auth::Client::new(&config);
            let chime_sdk_messaging_client = aws_sdk_chime_sdk_messaging::Client::new(&config);
            let mediaconnect_client = aws_sdk_mediaconnect::Client::new(&config);
            let identitystore_client = aws_sdk_identitystore::Client::new(&config);
            let bcm_pricing_calculator_client = aws_sdk_bcm_pricing_calculator::Client::new(&config);
            let lakeformation_client = aws_sdk_lakeformation::Client::new(&config);
            let xray_client = aws_sdk_xray::Client::new(&config);
            let cloudfront_client = aws_sdk_cloudfront::Client::new(&config);
            let sagemaker_metrics_client = aws_sdk_sagemaker_metrics::Client::new(&config);
            let sso_oidc_client = aws_sdk_sso_oidc::Client::new(&config);
            let sagemaker_client = aws_sdk_sagemaker::Client::new(&config);
            let codestar_connections_client = aws_sdk_codestar_connections::Client::new(&config);
            let device_farm_client = aws_sdk_device_farm::Client::new(&config);
            let translate_client = aws_sdk_translate::Client::new(&config);
            let sagemaker_runtime_client = aws_sdk_sagemaker_runtime::Client::new(&config);
            let b2bi_client = aws_sdk_b2bi::Client::new(&config);
            let savingsplans_client = aws_sdk_savingsplans::Client::new(&config);
            let pipes_client = aws_sdk_pipes::Client::new(&config);
            let config_service_client = aws_sdk_config_service::Client::new(&config);
            let codeguruprofiler_client = aws_sdk_codeguruprofiler::Client::new(&config);
            let s3_client = aws_sdk_s3::Client::new(&config);
            let polly_client = aws_sdk_polly::Client::new(&config);
            let cognito_sync_client = aws_sdk_cognito_sync::Client::new(&config);
            let scheduler_client = aws_sdk_scheduler::Client::new(&config);
            let pca_connector_ad_client = aws_sdk_pca_connector_ad::Client::new(&config);
            let waf_regional_client = aws_sdk_waf_regional::Client::new(&config);
            let apigatewaymanagementapi_client = aws_sdk_apigatewaymanagementapi::Client::new(&config);
            let workspaces_web_client = aws_sdk_workspaces_web::Client::new(&config);
            let pca_connector_scep_client = aws_sdk_pca_connector_scep::Client::new(&config);
            let codestar_notifications_client = aws_sdk_codestar_notifications::Client::new(&config);
            let direct_connect_client = aws_sdk_direct_connect::Client::new(&config);
            let shield_client = aws_sdk_shield::Client::new(&config);
            let application_signals_client = aws_sdk_application_signals::Client::new(&config);
            let iot_managed_integrations_client = aws_sdk_iot_managed_integrations::Client::new(&config);
            let iot_wireless_client = aws_sdk_iot_wireless::Client::new(&config);
            let iot_events_client = aws_sdk_iot_events::Client::new(&config);
            let backup_gateway_client = aws_sdk_backup_gateway::Client::new(&config);
            let sso_admin_client = aws_sdk_sso_admin::Client::new(&config);
            let elastic_beanstalk_client = aws_sdk_elastic_beanstalk::Client::new(&config);
            let drs_client = aws_sdk_drs::Client::new(&config);
            let personalize_runtime_client = aws_sdk_personalize_runtime::Client::new(&config);
            let outposts_client = aws_sdk_outposts::Client::new(&config);
            let license_manager_user_subscriptions_client = aws_sdk_license_manager_user_subscriptions::Client::new(&config);
            let cloudtrail_data_client = aws_sdk_cloudtrail_data::Client::new(&config);
            let lex_runtime_service_client = aws_sdk_lex_runtime_service::Client::new(&config);
            (config, emr_serverless_client, cloudformation_client, application_auto_scaling_client, personalize_events_client, tnb_client, rolesanywhere_client, kms_client, datasync_client, bedrock_agent_runtime_client, pinpoint_email_client, connect_contact_lens_client, athena_client, iotfleetwise_client, iot_data_plane_client, bedrock_data_automation_runtime_client, entityresolution_client, forecastquery_client, detective_client, panorama_client, backup_client, mwaa_client, iot_jobs_data_plane_client, transcribe_client, cloudwatch_client, snow_device_management_client, workmailmessageflow_client, appconfig_client, lightsail_client, guardduty_client, apigatewayv2_client, wafv2_client, iotsitewise_client, iotthingsgraph_client, batch_client, mailmanager_client, marketplace_reporting_client, m2_client, codedeploy_client, route53_recovery_control_config_client, simspaceweaver_client, resiliencehub_client, oam_client, license_manager_linux_subscriptions_client, voice_id_client, chime_client, efs_client, freetier_client, storage_gateway_client, dynamodb_streams_client, gamelift_client, inspector2_client, keyspaces_client, sqs_client, ram_client, ssm_sap_client, directory_service_data_client, route_53_domains_client, bedrock_agentcore_client, trustedadvisor_client, migrationhubstrategy_client, dataexchange_client, braket_client, codebuild_client, acm_client, route53_recovery_cluster_client, lookoutequipment_client, marketplace_catalog_client, payment_cryptography_data_client, cloud9_client, workdocs_client, license_manager_client, sts_client, s3vectors_client, chime_sdk_media_pipelines_client, machine_learning_client, timestream_query_client, codeguru_reviewer_client, mgn_client, evidently_client, qbusiness_client, connectcases_client, fsx_client, ecr_client, connectcampaignsv2_client, rds_client, qapps_client, qconnect_client, omics_client, bcm_dashboards_client, geo_routes_client, quicksight_client, amp_client, opensearchserverless_client, emr_client, service_quotas_client, service_catalog_appregistry_client, migrationhub_config_client, iam_client, accessanalyzer_client, appconfigdata_client, route53resolver_client, s3outposts_client, kendra_ranking_client, controltower_client, arc_region_switch_client, neptune_graph_client, route53_recovery_readiness_client, greengrassv2_client, migration_hub_refactor_spaces_client, cost_and_usage_report_service_client, ebs_client, appflow_client, migrationhuborchestrator_client, chime_sdk_identity_client, cloudfront_keyvaluestore_client, waf_client, greengrass_client, sagemaker_featurestore_runtime_client, inspector_client, appfabric_client, lex_model_building_service_client, serverlessapplicationrepository_client, cloudsearch_domain_client, codeguru_security_client, socialmessaging_client, geo_maps_client, kinesis_client, dsql_client, appintegrations_client, personalize_client, proton_client, cloudcontrol_client, redshift_client, geo_places_client, elasticsearch_service_client, bcm_recommended_actions_client, invoicing_client, apprunner_client, sns_client, textract_client, workmail_client, datazone_client, rekognition_client, ssm_client, medical_imaging_client, lex_models_client, support_client, signer_client, partnercentral_selling_client, comprehendmedical_client, macie2_client, redshift_data_client, marketplace_agreement_client, health_client, odb_client, resource_groups_tagging_api_client, application_insights_client, timestream_write_client, pinpoint_sms_client, mediapackagev2_client, ec2_client, cleanrooms_client, healthlake_client, sfn_client, iottwinmaker_client, cloudtrail_client, iotdeviceadvisor_client, ssm_incidents_client, pcs_client, support_app_client, managedblockchain_query_client, iot_events_data_client, lex_runtime_client, observabilityadmin_client, applicationcostprofiler_client, billingconductor_client, artifact_client, ecr_public_client, connectparticipant_client, rds_data_client, internetmonitor_client, route_53_client, bedrock_runtime_client, amplifybackend_client, marketplace_deployment_client, account_client, snowball_client, eventbridge_client, auto_scaling_plans_client, directory_service_client, mediapackage_client, ssm_quicksetup_client, s3_control_client, codecatalyst_client, notificationscontacts_client, mpa_client, ec2_instance_connect_client, sagemaker_geospatial_client, notifications_client, securitylake_client, networkmonitor_client, codeconnections_client, app_mesh_client, workspaces_thin_client_client, finspace_data_client, compute_optimizer_client, secrets_manager_client, mediastore_client, ecs_client, vpc_lattice_client, auto_scaling_client, resource_groups_client, eks_client, marketplace_entitlement_service_client, database_migration_service_client, security_ir_client, inspector_scan_client, global_accelerator_client, kinesis_analytics_client, neptunedata_client, swf_client, cloudwatch_logs_client, connect_client, glue_client, cognito_identity_provider_client, cloudwatch_events_client, cost_explorer_client, network_firewall_client, firehose_client, transfer_client, marketplace_metering_client, rbin_client, timestream_influxdb_client, iotanalytics_client, ivs_client, kafka_client, sesv2_client, kendra_client, sagemaker_edge_client, launch_wizard_client, securityhub_client, finspace_client, keyspacesstreams_client, cleanroomsml_client, transcribe_streaming_client, aiops_client, service_catalog_client, databrew_client, codecommit_client, resource_explorer_2_client, acm_pca_client, payment_cryptography_client, mq_client, api_gateway_client, grafana_client, glacier_client, bedrock_client, s3tables_client, ivs_realtime_client, medialive_client, backupsearch_client, networkflowmonitor_client, elasticache_client, fis_client, cloudhsm_client, cost_optimization_hub_client, synthetics_client, rum_client, emr_containers_client, sagemaker_a2i_runtime_client, ssm_contacts_client, bcm_data_exports_client, opensearch_client, dax_client, neptune_client, pricing_client, location_client, route53profiles_client, lambda_client, ivschat_client, billing_client, wisdom_client, schemas_client, bedrock_agentcore_control_client, controlcatalog_client, cloudsearch_client, deadline_client, managedblockchain_client, amplify_client, iotsecuretunneling_client, connectcampaigns_client, kafkaconnect_client, mediaconvert_client, data_pipeline_client, codepipeline_client, clouddirectory_client, amplifyuibuilder_client, rtbfabric_client, memorydb_client, iot_client, marketplace_commerce_analytics_client, frauddetector_client, bedrock_data_automation_client, elastic_load_balancing_client, verifiedpermissions_client, networkmanager_client, devops_guru_client, taxsettings_client, workspaces_instances_client, arc_zonal_shift_client, elastic_transcoder_client, fms_client, imagebuilder_client, chime_sdk_client, groundstation_client, forecast_client, appstream_client, chime_sdk_meetings_client, comprehend_client, redshift_serverless_client, pinpoint_client, pi_client, gameliftstreams_client, customer_profiles_client, workspaces_client, auditmanager_client, docdb_client, mturk_client, cognito_identity_client, dynamodb_client, codeartifact_client, organizations_client, dlm_client, sso_client, osis_client, migration_hub_client, chatbot_client, docdb_elastic_client, supplychain_client, ses_client, repostspace_client, mediastore_data_client, bedrock_agent_client, wellarchitected_client, budgets_client, mediatailor_client, appsync_client, ssm_guiconnect_client, evs_client, eks_auth_client, chime_sdk_messaging_client, mediaconnect_client, identitystore_client, bcm_pricing_calculator_client, lakeformation_client, xray_client, cloudfront_client, sagemaker_metrics_client, sso_oidc_client, sagemaker_client, codestar_connections_client, device_farm_client, translate_client, sagemaker_runtime_client, b2bi_client, savingsplans_client, pipes_client, config_service_client, codeguruprofiler_client, s3_client, polly_client, cognito_sync_client, scheduler_client, pca_connector_ad_client, waf_regional_client, apigatewaymanagementapi_client, workspaces_web_client, pca_connector_scep_client, codestar_notifications_client, direct_connect_client, shield_client, application_signals_client, iot_managed_integrations_client, iot_wireless_client, iot_events_client, backup_gateway_client, sso_admin_client, elastic_beanstalk_client, drs_client, personalize_runtime_client, outposts_client, license_manager_user_subscriptions_client, cloudtrail_data_client, lex_runtime_service_client)
        });

        Ok(Self {
            emr_serverless_client,
            cloudformation_client,
            application_auto_scaling_client,
            personalize_events_client,
            tnb_client,
            rolesanywhere_client,
            kms_client,
            datasync_client,
            bedrock_agent_runtime_client,
            pinpoint_email_client,
            connect_contact_lens_client,
            athena_client,
            iotfleetwise_client,
            iot_data_plane_client,
            bedrock_data_automation_runtime_client,
            entityresolution_client,
            forecastquery_client,
            detective_client,
            panorama_client,
            backup_client,
            mwaa_client,
            iot_jobs_data_plane_client,
            transcribe_client,
            cloudwatch_client,
            snow_device_management_client,
            workmailmessageflow_client,
            appconfig_client,
            lightsail_client,
            guardduty_client,
            apigatewayv2_client,
            wafv2_client,
            iotsitewise_client,
            iotthingsgraph_client,
            batch_client,
            mailmanager_client,
            marketplace_reporting_client,
            m2_client,
            codedeploy_client,
            route53_recovery_control_config_client,
            simspaceweaver_client,
            resiliencehub_client,
            oam_client,
            license_manager_linux_subscriptions_client,
            voice_id_client,
            chime_client,
            efs_client,
            freetier_client,
            storage_gateway_client,
            dynamodb_streams_client,
            gamelift_client,
            inspector2_client,
            keyspaces_client,
            sqs_client,
            ram_client,
            ssm_sap_client,
            directory_service_data_client,
            route_53_domains_client,
            bedrock_agentcore_client,
            trustedadvisor_client,
            migrationhubstrategy_client,
            dataexchange_client,
            braket_client,
            codebuild_client,
            acm_client,
            route53_recovery_cluster_client,
            lookoutequipment_client,
            marketplace_catalog_client,
            payment_cryptography_data_client,
            cloud9_client,
            workdocs_client,
            license_manager_client,
            sts_client,
            s3vectors_client,
            chime_sdk_media_pipelines_client,
            machine_learning_client,
            timestream_query_client,
            codeguru_reviewer_client,
            mgn_client,
            evidently_client,
            qbusiness_client,
            connectcases_client,
            fsx_client,
            ecr_client,
            connectcampaignsv2_client,
            rds_client,
            qapps_client,
            qconnect_client,
            omics_client,
            bcm_dashboards_client,
            geo_routes_client,
            quicksight_client,
            amp_client,
            opensearchserverless_client,
            emr_client,
            service_quotas_client,
            service_catalog_appregistry_client,
            migrationhub_config_client,
            iam_client,
            accessanalyzer_client,
            appconfigdata_client,
            route53resolver_client,
            s3outposts_client,
            kendra_ranking_client,
            controltower_client,
            arc_region_switch_client,
            neptune_graph_client,
            route53_recovery_readiness_client,
            greengrassv2_client,
            migration_hub_refactor_spaces_client,
            cost_and_usage_report_service_client,
            ebs_client,
            appflow_client,
            migrationhuborchestrator_client,
            chime_sdk_identity_client,
            cloudfront_keyvaluestore_client,
            waf_client,
            greengrass_client,
            sagemaker_featurestore_runtime_client,
            inspector_client,
            appfabric_client,
            lex_model_building_service_client,
            serverlessapplicationrepository_client,
            cloudsearch_domain_client,
            codeguru_security_client,
            socialmessaging_client,
            geo_maps_client,
            kinesis_client,
            dsql_client,
            appintegrations_client,
            personalize_client,
            proton_client,
            cloudcontrol_client,
            redshift_client,
            geo_places_client,
            elasticsearch_service_client,
            bcm_recommended_actions_client,
            invoicing_client,
            apprunner_client,
            sns_client,
            textract_client,
            workmail_client,
            datazone_client,
            rekognition_client,
            ssm_client,
            medical_imaging_client,
            lex_models_client,
            support_client,
            signer_client,
            partnercentral_selling_client,
            comprehendmedical_client,
            macie2_client,
            redshift_data_client,
            marketplace_agreement_client,
            health_client,
            odb_client,
            resource_groups_tagging_api_client,
            application_insights_client,
            timestream_write_client,
            pinpoint_sms_client,
            mediapackagev2_client,
            ec2_client,
            cleanrooms_client,
            healthlake_client,
            sfn_client,
            iottwinmaker_client,
            cloudtrail_client,
            iotdeviceadvisor_client,
            ssm_incidents_client,
            pcs_client,
            support_app_client,
            managedblockchain_query_client,
            iot_events_data_client,
            lex_runtime_client,
            observabilityadmin_client,
            applicationcostprofiler_client,
            billingconductor_client,
            artifact_client,
            ecr_public_client,
            connectparticipant_client,
            rds_data_client,
            internetmonitor_client,
            route_53_client,
            bedrock_runtime_client,
            amplifybackend_client,
            marketplace_deployment_client,
            account_client,
            snowball_client,
            eventbridge_client,
            auto_scaling_plans_client,
            directory_service_client,
            mediapackage_client,
            ssm_quicksetup_client,
            s3_control_client,
            codecatalyst_client,
            notificationscontacts_client,
            mpa_client,
            ec2_instance_connect_client,
            sagemaker_geospatial_client,
            notifications_client,
            securitylake_client,
            networkmonitor_client,
            codeconnections_client,
            app_mesh_client,
            workspaces_thin_client_client,
            finspace_data_client,
            compute_optimizer_client,
            secrets_manager_client,
            mediastore_client,
            ecs_client,
            vpc_lattice_client,
            auto_scaling_client,
            resource_groups_client,
            eks_client,
            marketplace_entitlement_service_client,
            database_migration_service_client,
            security_ir_client,
            inspector_scan_client,
            global_accelerator_client,
            kinesis_analytics_client,
            neptunedata_client,
            swf_client,
            cloudwatch_logs_client,
            connect_client,
            glue_client,
            cognito_identity_provider_client,
            cloudwatch_events_client,
            cost_explorer_client,
            network_firewall_client,
            firehose_client,
            transfer_client,
            marketplace_metering_client,
            rbin_client,
            timestream_influxdb_client,
            iotanalytics_client,
            ivs_client,
            kafka_client,
            sesv2_client,
            kendra_client,
            sagemaker_edge_client,
            launch_wizard_client,
            securityhub_client,
            finspace_client,
            keyspacesstreams_client,
            cleanroomsml_client,
            transcribe_streaming_client,
            aiops_client,
            service_catalog_client,
            databrew_client,
            codecommit_client,
            resource_explorer_2_client,
            acm_pca_client,
            payment_cryptography_client,
            mq_client,
            api_gateway_client,
            grafana_client,
            glacier_client,
            bedrock_client,
            s3tables_client,
            ivs_realtime_client,
            medialive_client,
            backupsearch_client,
            networkflowmonitor_client,
            elasticache_client,
            fis_client,
            cloudhsm_client,
            cost_optimization_hub_client,
            synthetics_client,
            rum_client,
            emr_containers_client,
            sagemaker_a2i_runtime_client,
            ssm_contacts_client,
            bcm_data_exports_client,
            opensearch_client,
            dax_client,
            neptune_client,
            pricing_client,
            location_client,
            route53profiles_client,
            lambda_client,
            ivschat_client,
            billing_client,
            wisdom_client,
            schemas_client,
            bedrock_agentcore_control_client,
            controlcatalog_client,
            cloudsearch_client,
            deadline_client,
            managedblockchain_client,
            amplify_client,
            iotsecuretunneling_client,
            connectcampaigns_client,
            kafkaconnect_client,
            mediaconvert_client,
            data_pipeline_client,
            codepipeline_client,
            clouddirectory_client,
            amplifyuibuilder_client,
            rtbfabric_client,
            memorydb_client,
            iot_client,
            marketplace_commerce_analytics_client,
            frauddetector_client,
            bedrock_data_automation_client,
            elastic_load_balancing_client,
            verifiedpermissions_client,
            networkmanager_client,
            devops_guru_client,
            taxsettings_client,
            workspaces_instances_client,
            arc_zonal_shift_client,
            elastic_transcoder_client,
            fms_client,
            imagebuilder_client,
            chime_sdk_client,
            groundstation_client,
            forecast_client,
            appstream_client,
            chime_sdk_meetings_client,
            comprehend_client,
            redshift_serverless_client,
            pinpoint_client,
            pi_client,
            gameliftstreams_client,
            customer_profiles_client,
            workspaces_client,
            auditmanager_client,
            docdb_client,
            mturk_client,
            cognito_identity_client,
            dynamodb_client,
            codeartifact_client,
            organizations_client,
            dlm_client,
            sso_client,
            osis_client,
            migration_hub_client,
            chatbot_client,
            docdb_elastic_client,
            supplychain_client,
            ses_client,
            repostspace_client,
            mediastore_data_client,
            bedrock_agent_client,
            wellarchitected_client,
            budgets_client,
            mediatailor_client,
            appsync_client,
            ssm_guiconnect_client,
            evs_client,
            eks_auth_client,
            chime_sdk_messaging_client,
            mediaconnect_client,
            identitystore_client,
            bcm_pricing_calculator_client,
            lakeformation_client,
            xray_client,
            cloudfront_client,
            sagemaker_metrics_client,
            sso_oidc_client,
            sagemaker_client,
            codestar_connections_client,
            device_farm_client,
            translate_client,
            sagemaker_runtime_client,
            b2bi_client,
            savingsplans_client,
            pipes_client,
            config_service_client,
            codeguruprofiler_client,
            s3_client,
            polly_client,
            cognito_sync_client,
            scheduler_client,
            pca_connector_ad_client,
            waf_regional_client,
            apigatewaymanagementapi_client,
            workspaces_web_client,
            pca_connector_scep_client,
            codestar_notifications_client,
            direct_connect_client,
            shield_client,
            application_signals_client,
            iot_managed_integrations_client,
            iot_wireless_client,
            iot_events_client,
            backup_gateway_client,
            sso_admin_client,
            elastic_beanstalk_client,
            drs_client,
            personalize_runtime_client,
            outposts_client,
            license_manager_user_subscriptions_client,
            cloudtrail_data_client,
            lex_runtime_service_client,
            runtime,

        })
    }

    /// Get emr_serverless service handler
    pub fn emr_serverless(&self) -> emr_serverless::Emr_serverlessService<'_> {
        emr_serverless::Emr_serverlessService::new(self)
    }
    /// Get cloudformation service handler
    pub fn cloudformation(&self) -> cloudformation::CloudformationService<'_> {
        cloudformation::CloudformationService::new(self)
    }
    /// Get application_auto_scaling service handler
    pub fn application_auto_scaling(&self) -> application_auto_scaling::Application_auto_scalingService<'_> {
        application_auto_scaling::Application_auto_scalingService::new(self)
    }
    /// Get personalize_events service handler
    pub fn personalize_events(&self) -> personalize_events::Personalize_eventsService<'_> {
        personalize_events::Personalize_eventsService::new(self)
    }
    /// Get tnb service handler
    pub fn tnb(&self) -> tnb::TnbService<'_> {
        tnb::TnbService::new(self)
    }
    /// Get rolesanywhere service handler
    pub fn rolesanywhere(&self) -> rolesanywhere::RolesanywhereService<'_> {
        rolesanywhere::RolesanywhereService::new(self)
    }
    /// Get kms service handler
    pub fn kms(&self) -> kms::KmsService<'_> {
        kms::KmsService::new(self)
    }
    /// Get datasync service handler
    pub fn datasync(&self) -> datasync::DatasyncService<'_> {
        datasync::DatasyncService::new(self)
    }
    /// Get bedrock_agent_runtime service handler
    pub fn bedrock_agent_runtime(&self) -> bedrock_agent_runtime::Bedrock_agent_runtimeService<'_> {
        bedrock_agent_runtime::Bedrock_agent_runtimeService::new(self)
    }
    /// Get pinpoint_email service handler
    pub fn pinpoint_email(&self) -> pinpoint_email::Pinpoint_emailService<'_> {
        pinpoint_email::Pinpoint_emailService::new(self)
    }
    /// Get connect_contact_lens service handler
    pub fn connect_contact_lens(&self) -> connect_contact_lens::Connect_contact_lensService<'_> {
        connect_contact_lens::Connect_contact_lensService::new(self)
    }
    /// Get athena service handler
    pub fn athena(&self) -> athena::AthenaService<'_> {
        athena::AthenaService::new(self)
    }
    /// Get iotfleetwise service handler
    pub fn iotfleetwise(&self) -> iotfleetwise::IotfleetwiseService<'_> {
        iotfleetwise::IotfleetwiseService::new(self)
    }
    /// Get iot_data_plane service handler
    pub fn iot_data_plane(&self) -> iot_data_plane::Iot_data_planeService<'_> {
        iot_data_plane::Iot_data_planeService::new(self)
    }
    /// Get bedrock_data_automation_runtime service handler
    pub fn bedrock_data_automation_runtime(&self) -> bedrock_data_automation_runtime::Bedrock_data_automation_runtimeService<'_> {
        bedrock_data_automation_runtime::Bedrock_data_automation_runtimeService::new(self)
    }
    /// Get entityresolution service handler
    pub fn entityresolution(&self) -> entityresolution::EntityresolutionService<'_> {
        entityresolution::EntityresolutionService::new(self)
    }
    /// Get forecastquery service handler
    pub fn forecastquery(&self) -> forecastquery::ForecastqueryService<'_> {
        forecastquery::ForecastqueryService::new(self)
    }
    /// Get detective service handler
    pub fn detective(&self) -> detective::DetectiveService<'_> {
        detective::DetectiveService::new(self)
    }
    /// Get panorama service handler
    pub fn panorama(&self) -> panorama::PanoramaService<'_> {
        panorama::PanoramaService::new(self)
    }
    /// Get backup service handler
    pub fn backup(&self) -> backup::BackupService<'_> {
        backup::BackupService::new(self)
    }
    /// Get mwaa service handler
    pub fn mwaa(&self) -> mwaa::MwaaService<'_> {
        mwaa::MwaaService::new(self)
    }
    /// Get iot_jobs_data_plane service handler
    pub fn iot_jobs_data_plane(&self) -> iot_jobs_data_plane::Iot_jobs_data_planeService<'_> {
        iot_jobs_data_plane::Iot_jobs_data_planeService::new(self)
    }
    /// Get transcribe service handler
    pub fn transcribe(&self) -> transcribe::TranscribeService<'_> {
        transcribe::TranscribeService::new(self)
    }
    /// Get cloudwatch service handler
    pub fn cloudwatch(&self) -> cloudwatch::CloudwatchService<'_> {
        cloudwatch::CloudwatchService::new(self)
    }
    /// Get snow_device_management service handler
    pub fn snow_device_management(&self) -> snow_device_management::Snow_device_managementService<'_> {
        snow_device_management::Snow_device_managementService::new(self)
    }
    /// Get workmailmessageflow service handler
    pub fn workmailmessageflow(&self) -> workmailmessageflow::WorkmailmessageflowService<'_> {
        workmailmessageflow::WorkmailmessageflowService::new(self)
    }
    /// Get appconfig service handler
    pub fn appconfig(&self) -> appconfig::AppconfigService<'_> {
        appconfig::AppconfigService::new(self)
    }
    /// Get lightsail service handler
    pub fn lightsail(&self) -> lightsail::LightsailService<'_> {
        lightsail::LightsailService::new(self)
    }
    /// Get guardduty service handler
    pub fn guardduty(&self) -> guardduty::GuarddutyService<'_> {
        guardduty::GuarddutyService::new(self)
    }
    /// Get apigatewayv2 service handler
    pub fn apigatewayv2(&self) -> apigatewayv2::Apigatewayv2Service<'_> {
        apigatewayv2::Apigatewayv2Service::new(self)
    }
    /// Get wafv2 service handler
    pub fn wafv2(&self) -> wafv2::Wafv2Service<'_> {
        wafv2::Wafv2Service::new(self)
    }
    /// Get iotsitewise service handler
    pub fn iotsitewise(&self) -> iotsitewise::IotsitewiseService<'_> {
        iotsitewise::IotsitewiseService::new(self)
    }
    /// Get iotthingsgraph service handler
    pub fn iotthingsgraph(&self) -> iotthingsgraph::IotthingsgraphService<'_> {
        iotthingsgraph::IotthingsgraphService::new(self)
    }
    /// Get batch service handler
    pub fn batch(&self) -> batch::BatchService<'_> {
        batch::BatchService::new(self)
    }
    /// Get mailmanager service handler
    pub fn mailmanager(&self) -> mailmanager::MailmanagerService<'_> {
        mailmanager::MailmanagerService::new(self)
    }
    /// Get marketplace_reporting service handler
    pub fn marketplace_reporting(&self) -> marketplace_reporting::Marketplace_reportingService<'_> {
        marketplace_reporting::Marketplace_reportingService::new(self)
    }
    /// Get m2 service handler
    pub fn m2(&self) -> m2::M2Service<'_> {
        m2::M2Service::new(self)
    }
    /// Get codedeploy service handler
    pub fn codedeploy(&self) -> codedeploy::CodedeployService<'_> {
        codedeploy::CodedeployService::new(self)
    }
    /// Get route53_recovery_control_config service handler
    pub fn route53_recovery_control_config(&self) -> route53_recovery_control_config::Route53_recovery_control_configService<'_> {
        route53_recovery_control_config::Route53_recovery_control_configService::new(self)
    }
    /// Get simspaceweaver service handler
    pub fn simspaceweaver(&self) -> simspaceweaver::SimspaceweaverService<'_> {
        simspaceweaver::SimspaceweaverService::new(self)
    }
    /// Get resiliencehub service handler
    pub fn resiliencehub(&self) -> resiliencehub::ResiliencehubService<'_> {
        resiliencehub::ResiliencehubService::new(self)
    }
    /// Get oam service handler
    pub fn oam(&self) -> oam::OamService<'_> {
        oam::OamService::new(self)
    }
    /// Get license_manager_linux_subscriptions service handler
    pub fn license_manager_linux_subscriptions(&self) -> license_manager_linux_subscriptions::License_manager_linux_subscriptionsService<'_> {
        license_manager_linux_subscriptions::License_manager_linux_subscriptionsService::new(self)
    }
    /// Get voice_id service handler
    pub fn voice_id(&self) -> voice_id::Voice_idService<'_> {
        voice_id::Voice_idService::new(self)
    }
    /// Get chime service handler
    pub fn chime(&self) -> chime::ChimeService<'_> {
        chime::ChimeService::new(self)
    }
    /// Get efs service handler
    pub fn efs(&self) -> efs::EfsService<'_> {
        efs::EfsService::new(self)
    }
    /// Get freetier service handler
    pub fn freetier(&self) -> freetier::FreetierService<'_> {
        freetier::FreetierService::new(self)
    }
    /// Get storage_gateway service handler
    pub fn storage_gateway(&self) -> storage_gateway::Storage_gatewayService<'_> {
        storage_gateway::Storage_gatewayService::new(self)
    }
    /// Get dynamodb_streams service handler
    pub fn dynamodb_streams(&self) -> dynamodb_streams::Dynamodb_streamsService<'_> {
        dynamodb_streams::Dynamodb_streamsService::new(self)
    }
    /// Get gamelift service handler
    pub fn gamelift(&self) -> gamelift::GameliftService<'_> {
        gamelift::GameliftService::new(self)
    }
    /// Get inspector2 service handler
    pub fn inspector2(&self) -> inspector2::Inspector2Service<'_> {
        inspector2::Inspector2Service::new(self)
    }
    /// Get keyspaces service handler
    pub fn keyspaces(&self) -> keyspaces::KeyspacesService<'_> {
        keyspaces::KeyspacesService::new(self)
    }
    /// Get sqs service handler
    pub fn sqs(&self) -> sqs::SqsService<'_> {
        sqs::SqsService::new(self)
    }
    /// Get ram service handler
    pub fn ram(&self) -> ram::RamService<'_> {
        ram::RamService::new(self)
    }
    /// Get ssm_sap service handler
    pub fn ssm_sap(&self) -> ssm_sap::Ssm_sapService<'_> {
        ssm_sap::Ssm_sapService::new(self)
    }
    /// Get directory_service_data service handler
    pub fn directory_service_data(&self) -> directory_service_data::Directory_service_dataService<'_> {
        directory_service_data::Directory_service_dataService::new(self)
    }
    /// Get route_53_domains service handler
    pub fn route_53_domains(&self) -> route_53_domains::Route_53_domainsService<'_> {
        route_53_domains::Route_53_domainsService::new(self)
    }
    /// Get bedrock_agentcore service handler
    pub fn bedrock_agentcore(&self) -> bedrock_agentcore::Bedrock_agentcoreService<'_> {
        bedrock_agentcore::Bedrock_agentcoreService::new(self)
    }
    /// Get trustedadvisor service handler
    pub fn trustedadvisor(&self) -> trustedadvisor::TrustedadvisorService<'_> {
        trustedadvisor::TrustedadvisorService::new(self)
    }
    /// Get migrationhubstrategy service handler
    pub fn migrationhubstrategy(&self) -> migrationhubstrategy::MigrationhubstrategyService<'_> {
        migrationhubstrategy::MigrationhubstrategyService::new(self)
    }
    /// Get dataexchange service handler
    pub fn dataexchange(&self) -> dataexchange::DataexchangeService<'_> {
        dataexchange::DataexchangeService::new(self)
    }
    /// Get braket service handler
    pub fn braket(&self) -> braket::BraketService<'_> {
        braket::BraketService::new(self)
    }
    /// Get codebuild service handler
    pub fn codebuild(&self) -> codebuild::CodebuildService<'_> {
        codebuild::CodebuildService::new(self)
    }
    /// Get acm service handler
    pub fn acm(&self) -> acm::AcmService<'_> {
        acm::AcmService::new(self)
    }
    /// Get route53_recovery_cluster service handler
    pub fn route53_recovery_cluster(&self) -> route53_recovery_cluster::Route53_recovery_clusterService<'_> {
        route53_recovery_cluster::Route53_recovery_clusterService::new(self)
    }
    /// Get lookoutequipment service handler
    pub fn lookoutequipment(&self) -> lookoutequipment::LookoutequipmentService<'_> {
        lookoutequipment::LookoutequipmentService::new(self)
    }
    /// Get marketplace_catalog service handler
    pub fn marketplace_catalog(&self) -> marketplace_catalog::Marketplace_catalogService<'_> {
        marketplace_catalog::Marketplace_catalogService::new(self)
    }
    /// Get payment_cryptography_data service handler
    pub fn payment_cryptography_data(&self) -> payment_cryptography_data::Payment_cryptography_dataService<'_> {
        payment_cryptography_data::Payment_cryptography_dataService::new(self)
    }
    /// Get cloud9 service handler
    pub fn cloud9(&self) -> cloud9::Cloud9Service<'_> {
        cloud9::Cloud9Service::new(self)
    }
    /// Get workdocs service handler
    pub fn workdocs(&self) -> workdocs::WorkdocsService<'_> {
        workdocs::WorkdocsService::new(self)
    }
    /// Get license_manager service handler
    pub fn license_manager(&self) -> license_manager::License_managerService<'_> {
        license_manager::License_managerService::new(self)
    }
    /// Get sts service handler
    pub fn sts(&self) -> sts::StsService<'_> {
        sts::StsService::new(self)
    }
    /// Get s3vectors service handler
    pub fn s3vectors(&self) -> s3vectors::S3vectorsService<'_> {
        s3vectors::S3vectorsService::new(self)
    }
    /// Get chime_sdk_media_pipelines service handler
    pub fn chime_sdk_media_pipelines(&self) -> chime_sdk_media_pipelines::Chime_sdk_media_pipelinesService<'_> {
        chime_sdk_media_pipelines::Chime_sdk_media_pipelinesService::new(self)
    }
    /// Get machine_learning service handler
    pub fn machine_learning(&self) -> machine_learning::Machine_learningService<'_> {
        machine_learning::Machine_learningService::new(self)
    }
    /// Get timestream_query service handler
    pub fn timestream_query(&self) -> timestream_query::Timestream_queryService<'_> {
        timestream_query::Timestream_queryService::new(self)
    }
    /// Get codeguru_reviewer service handler
    pub fn codeguru_reviewer(&self) -> codeguru_reviewer::Codeguru_reviewerService<'_> {
        codeguru_reviewer::Codeguru_reviewerService::new(self)
    }
    /// Get mgn service handler
    pub fn mgn(&self) -> mgn::MgnService<'_> {
        mgn::MgnService::new(self)
    }
    /// Get evidently service handler
    pub fn evidently(&self) -> evidently::EvidentlyService<'_> {
        evidently::EvidentlyService::new(self)
    }
    /// Get qbusiness service handler
    pub fn qbusiness(&self) -> qbusiness::QbusinessService<'_> {
        qbusiness::QbusinessService::new(self)
    }
    /// Get connectcases service handler
    pub fn connectcases(&self) -> connectcases::ConnectcasesService<'_> {
        connectcases::ConnectcasesService::new(self)
    }
    /// Get fsx service handler
    pub fn fsx(&self) -> fsx::FsxService<'_> {
        fsx::FsxService::new(self)
    }
    /// Get ecr service handler
    pub fn ecr(&self) -> ecr::EcrService<'_> {
        ecr::EcrService::new(self)
    }
    /// Get connectcampaignsv2 service handler
    pub fn connectcampaignsv2(&self) -> connectcampaignsv2::Connectcampaignsv2Service<'_> {
        connectcampaignsv2::Connectcampaignsv2Service::new(self)
    }
    /// Get rds service handler
    pub fn rds(&self) -> rds::RdsService<'_> {
        rds::RdsService::new(self)
    }
    /// Get qapps service handler
    pub fn qapps(&self) -> qapps::QappsService<'_> {
        qapps::QappsService::new(self)
    }
    /// Get qconnect service handler
    pub fn qconnect(&self) -> qconnect::QconnectService<'_> {
        qconnect::QconnectService::new(self)
    }
    /// Get omics service handler
    pub fn omics(&self) -> omics::OmicsService<'_> {
        omics::OmicsService::new(self)
    }
    /// Get bcm_dashboards service handler
    pub fn bcm_dashboards(&self) -> bcm_dashboards::Bcm_dashboardsService<'_> {
        bcm_dashboards::Bcm_dashboardsService::new(self)
    }
    /// Get geo_routes service handler
    pub fn geo_routes(&self) -> geo_routes::Geo_routesService<'_> {
        geo_routes::Geo_routesService::new(self)
    }
    /// Get quicksight service handler
    pub fn quicksight(&self) -> quicksight::QuicksightService<'_> {
        quicksight::QuicksightService::new(self)
    }
    /// Get amp service handler
    pub fn amp(&self) -> amp::AmpService<'_> {
        amp::AmpService::new(self)
    }
    /// Get opensearchserverless service handler
    pub fn opensearchserverless(&self) -> opensearchserverless::OpensearchserverlessService<'_> {
        opensearchserverless::OpensearchserverlessService::new(self)
    }
    /// Get emr service handler
    pub fn emr(&self) -> emr::EmrService<'_> {
        emr::EmrService::new(self)
    }
    /// Get service_quotas service handler
    pub fn service_quotas(&self) -> service_quotas::Service_quotasService<'_> {
        service_quotas::Service_quotasService::new(self)
    }
    /// Get service_catalog_appregistry service handler
    pub fn service_catalog_appregistry(&self) -> service_catalog_appregistry::Service_catalog_appregistryService<'_> {
        service_catalog_appregistry::Service_catalog_appregistryService::new(self)
    }
    /// Get migrationhub_config service handler
    pub fn migrationhub_config(&self) -> migrationhub_config::Migrationhub_configService<'_> {
        migrationhub_config::Migrationhub_configService::new(self)
    }
    /// Get iam service handler
    pub fn iam(&self) -> iam::IamService<'_> {
        iam::IamService::new(self)
    }
    /// Get accessanalyzer service handler
    pub fn accessanalyzer(&self) -> accessanalyzer::AccessanalyzerService<'_> {
        accessanalyzer::AccessanalyzerService::new(self)
    }
    /// Get appconfigdata service handler
    pub fn appconfigdata(&self) -> appconfigdata::AppconfigdataService<'_> {
        appconfigdata::AppconfigdataService::new(self)
    }
    /// Get route53resolver service handler
    pub fn route53resolver(&self) -> route53resolver::Route53resolverService<'_> {
        route53resolver::Route53resolverService::new(self)
    }
    /// Get s3outposts service handler
    pub fn s3outposts(&self) -> s3outposts::S3outpostsService<'_> {
        s3outposts::S3outpostsService::new(self)
    }
    /// Get kendra_ranking service handler
    pub fn kendra_ranking(&self) -> kendra_ranking::Kendra_rankingService<'_> {
        kendra_ranking::Kendra_rankingService::new(self)
    }
    /// Get controltower service handler
    pub fn controltower(&self) -> controltower::ControltowerService<'_> {
        controltower::ControltowerService::new(self)
    }
    /// Get arc_region_switch service handler
    pub fn arc_region_switch(&self) -> arc_region_switch::Arc_region_switchService<'_> {
        arc_region_switch::Arc_region_switchService::new(self)
    }
    /// Get neptune_graph service handler
    pub fn neptune_graph(&self) -> neptune_graph::Neptune_graphService<'_> {
        neptune_graph::Neptune_graphService::new(self)
    }
    /// Get route53_recovery_readiness service handler
    pub fn route53_recovery_readiness(&self) -> route53_recovery_readiness::Route53_recovery_readinessService<'_> {
        route53_recovery_readiness::Route53_recovery_readinessService::new(self)
    }
    /// Get greengrassv2 service handler
    pub fn greengrassv2(&self) -> greengrassv2::Greengrassv2Service<'_> {
        greengrassv2::Greengrassv2Service::new(self)
    }
    /// Get migration_hub_refactor_spaces service handler
    pub fn migration_hub_refactor_spaces(&self) -> migration_hub_refactor_spaces::Migration_hub_refactor_spacesService<'_> {
        migration_hub_refactor_spaces::Migration_hub_refactor_spacesService::new(self)
    }
    /// Get cost_and_usage_report_service service handler
    pub fn cost_and_usage_report_service(&self) -> cost_and_usage_report_service::Cost_and_usage_report_serviceService<'_> {
        cost_and_usage_report_service::Cost_and_usage_report_serviceService::new(self)
    }
    /// Get ebs service handler
    pub fn ebs(&self) -> ebs::EbsService<'_> {
        ebs::EbsService::new(self)
    }
    /// Get appflow service handler
    pub fn appflow(&self) -> appflow::AppflowService<'_> {
        appflow::AppflowService::new(self)
    }
    /// Get migrationhuborchestrator service handler
    pub fn migrationhuborchestrator(&self) -> migrationhuborchestrator::MigrationhuborchestratorService<'_> {
        migrationhuborchestrator::MigrationhuborchestratorService::new(self)
    }
    /// Get chime_sdk_identity service handler
    pub fn chime_sdk_identity(&self) -> chime_sdk_identity::Chime_sdk_identityService<'_> {
        chime_sdk_identity::Chime_sdk_identityService::new(self)
    }
    /// Get cloudfront_keyvaluestore service handler
    pub fn cloudfront_keyvaluestore(&self) -> cloudfront_keyvaluestore::Cloudfront_keyvaluestoreService<'_> {
        cloudfront_keyvaluestore::Cloudfront_keyvaluestoreService::new(self)
    }
    /// Get waf service handler
    pub fn waf(&self) -> waf::WafService<'_> {
        waf::WafService::new(self)
    }
    /// Get greengrass service handler
    pub fn greengrass(&self) -> greengrass::GreengrassService<'_> {
        greengrass::GreengrassService::new(self)
    }
    /// Get sagemaker_featurestore_runtime service handler
    pub fn sagemaker_featurestore_runtime(&self) -> sagemaker_featurestore_runtime::Sagemaker_featurestore_runtimeService<'_> {
        sagemaker_featurestore_runtime::Sagemaker_featurestore_runtimeService::new(self)
    }
    /// Get inspector service handler
    pub fn inspector(&self) -> inspector::InspectorService<'_> {
        inspector::InspectorService::new(self)
    }
    /// Get appfabric service handler
    pub fn appfabric(&self) -> appfabric::AppfabricService<'_> {
        appfabric::AppfabricService::new(self)
    }
    /// Get lex_model_building_service service handler
    pub fn lex_model_building_service(&self) -> lex_model_building_service::Lex_model_building_serviceService<'_> {
        lex_model_building_service::Lex_model_building_serviceService::new(self)
    }
    /// Get serverlessapplicationrepository service handler
    pub fn serverlessapplicationrepository(&self) -> serverlessapplicationrepository::ServerlessapplicationrepositoryService<'_> {
        serverlessapplicationrepository::ServerlessapplicationrepositoryService::new(self)
    }
    /// Get cloudsearch_domain service handler
    pub fn cloudsearch_domain(&self) -> cloudsearch_domain::Cloudsearch_domainService<'_> {
        cloudsearch_domain::Cloudsearch_domainService::new(self)
    }
    /// Get codeguru_security service handler
    pub fn codeguru_security(&self) -> codeguru_security::Codeguru_securityService<'_> {
        codeguru_security::Codeguru_securityService::new(self)
    }
    /// Get socialmessaging service handler
    pub fn socialmessaging(&self) -> socialmessaging::SocialmessagingService<'_> {
        socialmessaging::SocialmessagingService::new(self)
    }
    /// Get geo_maps service handler
    pub fn geo_maps(&self) -> geo_maps::Geo_mapsService<'_> {
        geo_maps::Geo_mapsService::new(self)
    }
    /// Get kinesis service handler
    pub fn kinesis(&self) -> kinesis::KinesisService<'_> {
        kinesis::KinesisService::new(self)
    }
    /// Get dsql service handler
    pub fn dsql(&self) -> dsql::DsqlService<'_> {
        dsql::DsqlService::new(self)
    }
    /// Get appintegrations service handler
    pub fn appintegrations(&self) -> appintegrations::AppintegrationsService<'_> {
        appintegrations::AppintegrationsService::new(self)
    }
    /// Get personalize service handler
    pub fn personalize(&self) -> personalize::PersonalizeService<'_> {
        personalize::PersonalizeService::new(self)
    }
    /// Get proton service handler
    pub fn proton(&self) -> proton::ProtonService<'_> {
        proton::ProtonService::new(self)
    }
    /// Get cloudcontrol service handler
    pub fn cloudcontrol(&self) -> cloudcontrol::CloudcontrolService<'_> {
        cloudcontrol::CloudcontrolService::new(self)
    }
    /// Get redshift service handler
    pub fn redshift(&self) -> redshift::RedshiftService<'_> {
        redshift::RedshiftService::new(self)
    }
    /// Get geo_places service handler
    pub fn geo_places(&self) -> geo_places::Geo_placesService<'_> {
        geo_places::Geo_placesService::new(self)
    }
    /// Get elasticsearch_service service handler
    pub fn elasticsearch_service(&self) -> elasticsearch_service::Elasticsearch_serviceService<'_> {
        elasticsearch_service::Elasticsearch_serviceService::new(self)
    }
    /// Get bcm_recommended_actions service handler
    pub fn bcm_recommended_actions(&self) -> bcm_recommended_actions::Bcm_recommended_actionsService<'_> {
        bcm_recommended_actions::Bcm_recommended_actionsService::new(self)
    }
    /// Get invoicing service handler
    pub fn invoicing(&self) -> invoicing::InvoicingService<'_> {
        invoicing::InvoicingService::new(self)
    }
    /// Get apprunner service handler
    pub fn apprunner(&self) -> apprunner::ApprunnerService<'_> {
        apprunner::ApprunnerService::new(self)
    }
    /// Get sns service handler
    pub fn sns(&self) -> sns::SnsService<'_> {
        sns::SnsService::new(self)
    }
    /// Get textract service handler
    pub fn textract(&self) -> textract::TextractService<'_> {
        textract::TextractService::new(self)
    }
    /// Get workmail service handler
    pub fn workmail(&self) -> workmail::WorkmailService<'_> {
        workmail::WorkmailService::new(self)
    }
    /// Get datazone service handler
    pub fn datazone(&self) -> datazone::DatazoneService<'_> {
        datazone::DatazoneService::new(self)
    }
    /// Get rekognition service handler
    pub fn rekognition(&self) -> rekognition::RekognitionService<'_> {
        rekognition::RekognitionService::new(self)
    }
    /// Get ssm service handler
    pub fn ssm(&self) -> ssm::SsmService<'_> {
        ssm::SsmService::new(self)
    }
    /// Get medical_imaging service handler
    pub fn medical_imaging(&self) -> medical_imaging::Medical_imagingService<'_> {
        medical_imaging::Medical_imagingService::new(self)
    }
    /// Get lex_models service handler
    pub fn lex_models(&self) -> lex_models::Lex_modelsService<'_> {
        lex_models::Lex_modelsService::new(self)
    }
    /// Get support service handler
    pub fn support(&self) -> support::SupportService<'_> {
        support::SupportService::new(self)
    }
    /// Get signer service handler
    pub fn signer(&self) -> signer::SignerService<'_> {
        signer::SignerService::new(self)
    }
    /// Get partnercentral_selling service handler
    pub fn partnercentral_selling(&self) -> partnercentral_selling::Partnercentral_sellingService<'_> {
        partnercentral_selling::Partnercentral_sellingService::new(self)
    }
    /// Get comprehendmedical service handler
    pub fn comprehendmedical(&self) -> comprehendmedical::ComprehendmedicalService<'_> {
        comprehendmedical::ComprehendmedicalService::new(self)
    }
    /// Get macie2 service handler
    pub fn macie2(&self) -> macie2::Macie2Service<'_> {
        macie2::Macie2Service::new(self)
    }
    /// Get redshift_data service handler
    pub fn redshift_data(&self) -> redshift_data::Redshift_dataService<'_> {
        redshift_data::Redshift_dataService::new(self)
    }
    /// Get marketplace_agreement service handler
    pub fn marketplace_agreement(&self) -> marketplace_agreement::Marketplace_agreementService<'_> {
        marketplace_agreement::Marketplace_agreementService::new(self)
    }
    /// Get health service handler
    pub fn health(&self) -> health::HealthService<'_> {
        health::HealthService::new(self)
    }
    /// Get odb service handler
    pub fn odb(&self) -> odb::OdbService<'_> {
        odb::OdbService::new(self)
    }
    /// Get resource_groups_tagging_api service handler
    pub fn resource_groups_tagging_api(&self) -> resource_groups_tagging_api::Resource_groups_tagging_apiService<'_> {
        resource_groups_tagging_api::Resource_groups_tagging_apiService::new(self)
    }
    /// Get application_insights service handler
    pub fn application_insights(&self) -> application_insights::Application_insightsService<'_> {
        application_insights::Application_insightsService::new(self)
    }
    /// Get timestream_write service handler
    pub fn timestream_write(&self) -> timestream_write::Timestream_writeService<'_> {
        timestream_write::Timestream_writeService::new(self)
    }
    /// Get pinpoint_sms service handler
    pub fn pinpoint_sms(&self) -> pinpoint_sms::Pinpoint_smsService<'_> {
        pinpoint_sms::Pinpoint_smsService::new(self)
    }
    /// Get mediapackagev2 service handler
    pub fn mediapackagev2(&self) -> mediapackagev2::Mediapackagev2Service<'_> {
        mediapackagev2::Mediapackagev2Service::new(self)
    }
    /// Get ec2 service handler
    pub fn ec2(&self) -> ec2::Ec2Service<'_> {
        ec2::Ec2Service::new(self)
    }
    /// Get cleanrooms service handler
    pub fn cleanrooms(&self) -> cleanrooms::CleanroomsService<'_> {
        cleanrooms::CleanroomsService::new(self)
    }
    /// Get healthlake service handler
    pub fn healthlake(&self) -> healthlake::HealthlakeService<'_> {
        healthlake::HealthlakeService::new(self)
    }
    /// Get sfn service handler
    pub fn sfn(&self) -> sfn::SfnService<'_> {
        sfn::SfnService::new(self)
    }
    /// Get iottwinmaker service handler
    pub fn iottwinmaker(&self) -> iottwinmaker::IottwinmakerService<'_> {
        iottwinmaker::IottwinmakerService::new(self)
    }
    /// Get cloudtrail service handler
    pub fn cloudtrail(&self) -> cloudtrail::CloudtrailService<'_> {
        cloudtrail::CloudtrailService::new(self)
    }
    /// Get iotdeviceadvisor service handler
    pub fn iotdeviceadvisor(&self) -> iotdeviceadvisor::IotdeviceadvisorService<'_> {
        iotdeviceadvisor::IotdeviceadvisorService::new(self)
    }
    /// Get ssm_incidents service handler
    pub fn ssm_incidents(&self) -> ssm_incidents::Ssm_incidentsService<'_> {
        ssm_incidents::Ssm_incidentsService::new(self)
    }
    /// Get pcs service handler
    pub fn pcs(&self) -> pcs::PcsService<'_> {
        pcs::PcsService::new(self)
    }
    /// Get support_app service handler
    pub fn support_app(&self) -> support_app::Support_appService<'_> {
        support_app::Support_appService::new(self)
    }
    /// Get managedblockchain_query service handler
    pub fn managedblockchain_query(&self) -> managedblockchain_query::Managedblockchain_queryService<'_> {
        managedblockchain_query::Managedblockchain_queryService::new(self)
    }
    /// Get iot_events_data service handler
    pub fn iot_events_data(&self) -> iot_events_data::Iot_events_dataService<'_> {
        iot_events_data::Iot_events_dataService::new(self)
    }
    /// Get lex_runtime service handler
    pub fn lex_runtime(&self) -> lex_runtime::Lex_runtimeService<'_> {
        lex_runtime::Lex_runtimeService::new(self)
    }
    /// Get observabilityadmin service handler
    pub fn observabilityadmin(&self) -> observabilityadmin::ObservabilityadminService<'_> {
        observabilityadmin::ObservabilityadminService::new(self)
    }
    /// Get applicationcostprofiler service handler
    pub fn applicationcostprofiler(&self) -> applicationcostprofiler::ApplicationcostprofilerService<'_> {
        applicationcostprofiler::ApplicationcostprofilerService::new(self)
    }
    /// Get billingconductor service handler
    pub fn billingconductor(&self) -> billingconductor::BillingconductorService<'_> {
        billingconductor::BillingconductorService::new(self)
    }
    /// Get artifact service handler
    pub fn artifact(&self) -> artifact::ArtifactService<'_> {
        artifact::ArtifactService::new(self)
    }
    /// Get ecr_public service handler
    pub fn ecr_public(&self) -> ecr_public::Ecr_publicService<'_> {
        ecr_public::Ecr_publicService::new(self)
    }
    /// Get connectparticipant service handler
    pub fn connectparticipant(&self) -> connectparticipant::ConnectparticipantService<'_> {
        connectparticipant::ConnectparticipantService::new(self)
    }
    /// Get rds_data service handler
    pub fn rds_data(&self) -> rds_data::Rds_dataService<'_> {
        rds_data::Rds_dataService::new(self)
    }
    /// Get internetmonitor service handler
    pub fn internetmonitor(&self) -> internetmonitor::InternetmonitorService<'_> {
        internetmonitor::InternetmonitorService::new(self)
    }
    /// Get route_53 service handler
    pub fn route_53(&self) -> route_53::Route_53Service<'_> {
        route_53::Route_53Service::new(self)
    }
    /// Get bedrock_runtime service handler
    pub fn bedrock_runtime(&self) -> bedrock_runtime::Bedrock_runtimeService<'_> {
        bedrock_runtime::Bedrock_runtimeService::new(self)
    }
    /// Get amplifybackend service handler
    pub fn amplifybackend(&self) -> amplifybackend::AmplifybackendService<'_> {
        amplifybackend::AmplifybackendService::new(self)
    }
    /// Get marketplace_deployment service handler
    pub fn marketplace_deployment(&self) -> marketplace_deployment::Marketplace_deploymentService<'_> {
        marketplace_deployment::Marketplace_deploymentService::new(self)
    }
    /// Get account service handler
    pub fn account(&self) -> account::AccountService<'_> {
        account::AccountService::new(self)
    }
    /// Get snowball service handler
    pub fn snowball(&self) -> snowball::SnowballService<'_> {
        snowball::SnowballService::new(self)
    }
    /// Get eventbridge service handler
    pub fn eventbridge(&self) -> eventbridge::EventbridgeService<'_> {
        eventbridge::EventbridgeService::new(self)
    }
    /// Get auto_scaling_plans service handler
    pub fn auto_scaling_plans(&self) -> auto_scaling_plans::Auto_scaling_plansService<'_> {
        auto_scaling_plans::Auto_scaling_plansService::new(self)
    }
    /// Get directory_service service handler
    pub fn directory_service(&self) -> directory_service::Directory_serviceService<'_> {
        directory_service::Directory_serviceService::new(self)
    }
    /// Get mediapackage service handler
    pub fn mediapackage(&self) -> mediapackage::MediapackageService<'_> {
        mediapackage::MediapackageService::new(self)
    }
    /// Get ssm_quicksetup service handler
    pub fn ssm_quicksetup(&self) -> ssm_quicksetup::Ssm_quicksetupService<'_> {
        ssm_quicksetup::Ssm_quicksetupService::new(self)
    }
    /// Get s3_control service handler
    pub fn s3_control(&self) -> s3_control::S3_controlService<'_> {
        s3_control::S3_controlService::new(self)
    }
    /// Get codecatalyst service handler
    pub fn codecatalyst(&self) -> codecatalyst::CodecatalystService<'_> {
        codecatalyst::CodecatalystService::new(self)
    }
    /// Get notificationscontacts service handler
    pub fn notificationscontacts(&self) -> notificationscontacts::NotificationscontactsService<'_> {
        notificationscontacts::NotificationscontactsService::new(self)
    }
    /// Get mpa service handler
    pub fn mpa(&self) -> mpa::MpaService<'_> {
        mpa::MpaService::new(self)
    }
    /// Get ec2_instance_connect service handler
    pub fn ec2_instance_connect(&self) -> ec2_instance_connect::Ec2_instance_connectService<'_> {
        ec2_instance_connect::Ec2_instance_connectService::new(self)
    }
    /// Get sagemaker_geospatial service handler
    pub fn sagemaker_geospatial(&self) -> sagemaker_geospatial::Sagemaker_geospatialService<'_> {
        sagemaker_geospatial::Sagemaker_geospatialService::new(self)
    }
    /// Get notifications service handler
    pub fn notifications(&self) -> notifications::NotificationsService<'_> {
        notifications::NotificationsService::new(self)
    }
    /// Get securitylake service handler
    pub fn securitylake(&self) -> securitylake::SecuritylakeService<'_> {
        securitylake::SecuritylakeService::new(self)
    }
    /// Get networkmonitor service handler
    pub fn networkmonitor(&self) -> networkmonitor::NetworkmonitorService<'_> {
        networkmonitor::NetworkmonitorService::new(self)
    }
    /// Get codeconnections service handler
    pub fn codeconnections(&self) -> codeconnections::CodeconnectionsService<'_> {
        codeconnections::CodeconnectionsService::new(self)
    }
    /// Get app_mesh service handler
    pub fn app_mesh(&self) -> app_mesh::App_meshService<'_> {
        app_mesh::App_meshService::new(self)
    }
    /// Get workspaces_thin_client service handler
    pub fn workspaces_thin_client(&self) -> workspaces_thin_client::Workspaces_thin_clientService<'_> {
        workspaces_thin_client::Workspaces_thin_clientService::new(self)
    }
    /// Get finspace_data service handler
    pub fn finspace_data(&self) -> finspace_data::Finspace_dataService<'_> {
        finspace_data::Finspace_dataService::new(self)
    }
    /// Get compute_optimizer service handler
    pub fn compute_optimizer(&self) -> compute_optimizer::Compute_optimizerService<'_> {
        compute_optimizer::Compute_optimizerService::new(self)
    }
    /// Get secrets_manager service handler
    pub fn secrets_manager(&self) -> secrets_manager::Secrets_managerService<'_> {
        secrets_manager::Secrets_managerService::new(self)
    }
    /// Get mediastore service handler
    pub fn mediastore(&self) -> mediastore::MediastoreService<'_> {
        mediastore::MediastoreService::new(self)
    }
    /// Get ecs service handler
    pub fn ecs(&self) -> ecs::EcsService<'_> {
        ecs::EcsService::new(self)
    }
    /// Get vpc_lattice service handler
    pub fn vpc_lattice(&self) -> vpc_lattice::Vpc_latticeService<'_> {
        vpc_lattice::Vpc_latticeService::new(self)
    }
    /// Get auto_scaling service handler
    pub fn auto_scaling(&self) -> auto_scaling::Auto_scalingService<'_> {
        auto_scaling::Auto_scalingService::new(self)
    }
    /// Get resource_groups service handler
    pub fn resource_groups(&self) -> resource_groups::Resource_groupsService<'_> {
        resource_groups::Resource_groupsService::new(self)
    }
    /// Get eks service handler
    pub fn eks(&self) -> eks::EksService<'_> {
        eks::EksService::new(self)
    }
    /// Get marketplace_entitlement_service service handler
    pub fn marketplace_entitlement_service(&self) -> marketplace_entitlement_service::Marketplace_entitlement_serviceService<'_> {
        marketplace_entitlement_service::Marketplace_entitlement_serviceService::new(self)
    }
    /// Get database_migration_service service handler
    pub fn database_migration_service(&self) -> database_migration_service::Database_migration_serviceService<'_> {
        database_migration_service::Database_migration_serviceService::new(self)
    }
    /// Get security_ir service handler
    pub fn security_ir(&self) -> security_ir::Security_irService<'_> {
        security_ir::Security_irService::new(self)
    }
    /// Get inspector_scan service handler
    pub fn inspector_scan(&self) -> inspector_scan::Inspector_scanService<'_> {
        inspector_scan::Inspector_scanService::new(self)
    }
    /// Get global_accelerator service handler
    pub fn global_accelerator(&self) -> global_accelerator::Global_acceleratorService<'_> {
        global_accelerator::Global_acceleratorService::new(self)
    }
    /// Get kinesis_analytics service handler
    pub fn kinesis_analytics(&self) -> kinesis_analytics::Kinesis_analyticsService<'_> {
        kinesis_analytics::Kinesis_analyticsService::new(self)
    }
    /// Get neptunedata service handler
    pub fn neptunedata(&self) -> neptunedata::NeptunedataService<'_> {
        neptunedata::NeptunedataService::new(self)
    }
    /// Get swf service handler
    pub fn swf(&self) -> swf::SwfService<'_> {
        swf::SwfService::new(self)
    }
    /// Get cloudwatch_logs service handler
    pub fn cloudwatch_logs(&self) -> cloudwatch_logs::Cloudwatch_logsService<'_> {
        cloudwatch_logs::Cloudwatch_logsService::new(self)
    }
    /// Get connect service handler
    pub fn connect(&self) -> connect::ConnectService<'_> {
        connect::ConnectService::new(self)
    }
    /// Get glue service handler
    pub fn glue(&self) -> glue::GlueService<'_> {
        glue::GlueService::new(self)
    }
    /// Get cognito_identity_provider service handler
    pub fn cognito_identity_provider(&self) -> cognito_identity_provider::Cognito_identity_providerService<'_> {
        cognito_identity_provider::Cognito_identity_providerService::new(self)
    }
    /// Get cloudwatch_events service handler
    pub fn cloudwatch_events(&self) -> cloudwatch_events::Cloudwatch_eventsService<'_> {
        cloudwatch_events::Cloudwatch_eventsService::new(self)
    }
    /// Get cost_explorer service handler
    pub fn cost_explorer(&self) -> cost_explorer::Cost_explorerService<'_> {
        cost_explorer::Cost_explorerService::new(self)
    }
    /// Get network_firewall service handler
    pub fn network_firewall(&self) -> network_firewall::Network_firewallService<'_> {
        network_firewall::Network_firewallService::new(self)
    }
    /// Get firehose service handler
    pub fn firehose(&self) -> firehose::FirehoseService<'_> {
        firehose::FirehoseService::new(self)
    }
    /// Get transfer service handler
    pub fn transfer(&self) -> transfer::TransferService<'_> {
        transfer::TransferService::new(self)
    }
    /// Get marketplace_metering service handler
    pub fn marketplace_metering(&self) -> marketplace_metering::Marketplace_meteringService<'_> {
        marketplace_metering::Marketplace_meteringService::new(self)
    }
    /// Get rbin service handler
    pub fn rbin(&self) -> rbin::RbinService<'_> {
        rbin::RbinService::new(self)
    }
    /// Get timestream_influxdb service handler
    pub fn timestream_influxdb(&self) -> timestream_influxdb::Timestream_influxdbService<'_> {
        timestream_influxdb::Timestream_influxdbService::new(self)
    }
    /// Get iotanalytics service handler
    pub fn iotanalytics(&self) -> iotanalytics::IotanalyticsService<'_> {
        iotanalytics::IotanalyticsService::new(self)
    }
    /// Get ivs service handler
    pub fn ivs(&self) -> ivs::IvsService<'_> {
        ivs::IvsService::new(self)
    }
    /// Get kafka service handler
    pub fn kafka(&self) -> kafka::KafkaService<'_> {
        kafka::KafkaService::new(self)
    }
    /// Get sesv2 service handler
    pub fn sesv2(&self) -> sesv2::Sesv2Service<'_> {
        sesv2::Sesv2Service::new(self)
    }
    /// Get kendra service handler
    pub fn kendra(&self) -> kendra::KendraService<'_> {
        kendra::KendraService::new(self)
    }
    /// Get sagemaker_edge service handler
    pub fn sagemaker_edge(&self) -> sagemaker_edge::Sagemaker_edgeService<'_> {
        sagemaker_edge::Sagemaker_edgeService::new(self)
    }
    /// Get launch_wizard service handler
    pub fn launch_wizard(&self) -> launch_wizard::Launch_wizardService<'_> {
        launch_wizard::Launch_wizardService::new(self)
    }
    /// Get securityhub service handler
    pub fn securityhub(&self) -> securityhub::SecurityhubService<'_> {
        securityhub::SecurityhubService::new(self)
    }
    /// Get finspace service handler
    pub fn finspace(&self) -> finspace::FinspaceService<'_> {
        finspace::FinspaceService::new(self)
    }
    /// Get keyspacesstreams service handler
    pub fn keyspacesstreams(&self) -> keyspacesstreams::KeyspacesstreamsService<'_> {
        keyspacesstreams::KeyspacesstreamsService::new(self)
    }
    /// Get cleanroomsml service handler
    pub fn cleanroomsml(&self) -> cleanroomsml::CleanroomsmlService<'_> {
        cleanroomsml::CleanroomsmlService::new(self)
    }
    /// Get transcribe_streaming service handler
    pub fn transcribe_streaming(&self) -> transcribe_streaming::Transcribe_streamingService<'_> {
        transcribe_streaming::Transcribe_streamingService::new(self)
    }
    /// Get aiops service handler
    pub fn aiops(&self) -> aiops::AiopsService<'_> {
        aiops::AiopsService::new(self)
    }
    /// Get service_catalog service handler
    pub fn service_catalog(&self) -> service_catalog::Service_catalogService<'_> {
        service_catalog::Service_catalogService::new(self)
    }
    /// Get databrew service handler
    pub fn databrew(&self) -> databrew::DatabrewService<'_> {
        databrew::DatabrewService::new(self)
    }
    /// Get codecommit service handler
    pub fn codecommit(&self) -> codecommit::CodecommitService<'_> {
        codecommit::CodecommitService::new(self)
    }
    /// Get resource_explorer_2 service handler
    pub fn resource_explorer_2(&self) -> resource_explorer_2::Resource_explorer_2Service<'_> {
        resource_explorer_2::Resource_explorer_2Service::new(self)
    }
    /// Get acm_pca service handler
    pub fn acm_pca(&self) -> acm_pca::Acm_pcaService<'_> {
        acm_pca::Acm_pcaService::new(self)
    }
    /// Get payment_cryptography service handler
    pub fn payment_cryptography(&self) -> payment_cryptography::Payment_cryptographyService<'_> {
        payment_cryptography::Payment_cryptographyService::new(self)
    }
    /// Get mq service handler
    pub fn mq(&self) -> mq::MqService<'_> {
        mq::MqService::new(self)
    }
    /// Get api_gateway service handler
    pub fn api_gateway(&self) -> api_gateway::Api_gatewayService<'_> {
        api_gateway::Api_gatewayService::new(self)
    }
    /// Get grafana service handler
    pub fn grafana(&self) -> grafana::GrafanaService<'_> {
        grafana::GrafanaService::new(self)
    }
    /// Get glacier service handler
    pub fn glacier(&self) -> glacier::GlacierService<'_> {
        glacier::GlacierService::new(self)
    }
    /// Get bedrock service handler
    pub fn bedrock(&self) -> bedrock::BedrockService<'_> {
        bedrock::BedrockService::new(self)
    }
    /// Get s3tables service handler
    pub fn s3tables(&self) -> s3tables::S3tablesService<'_> {
        s3tables::S3tablesService::new(self)
    }
    /// Get ivs_realtime service handler
    pub fn ivs_realtime(&self) -> ivs_realtime::Ivs_realtimeService<'_> {
        ivs_realtime::Ivs_realtimeService::new(self)
    }
    /// Get medialive service handler
    pub fn medialive(&self) -> medialive::MedialiveService<'_> {
        medialive::MedialiveService::new(self)
    }
    /// Get backupsearch service handler
    pub fn backupsearch(&self) -> backupsearch::BackupsearchService<'_> {
        backupsearch::BackupsearchService::new(self)
    }
    /// Get networkflowmonitor service handler
    pub fn networkflowmonitor(&self) -> networkflowmonitor::NetworkflowmonitorService<'_> {
        networkflowmonitor::NetworkflowmonitorService::new(self)
    }
    /// Get elasticache service handler
    pub fn elasticache(&self) -> elasticache::ElasticacheService<'_> {
        elasticache::ElasticacheService::new(self)
    }
    /// Get fis service handler
    pub fn fis(&self) -> fis::FisService<'_> {
        fis::FisService::new(self)
    }
    /// Get cloudhsm service handler
    pub fn cloudhsm(&self) -> cloudhsm::CloudhsmService<'_> {
        cloudhsm::CloudhsmService::new(self)
    }
    /// Get cost_optimization_hub service handler
    pub fn cost_optimization_hub(&self) -> cost_optimization_hub::Cost_optimization_hubService<'_> {
        cost_optimization_hub::Cost_optimization_hubService::new(self)
    }
    /// Get synthetics service handler
    pub fn synthetics(&self) -> synthetics::SyntheticsService<'_> {
        synthetics::SyntheticsService::new(self)
    }
    /// Get rum service handler
    pub fn rum(&self) -> rum::RumService<'_> {
        rum::RumService::new(self)
    }
    /// Get emr_containers service handler
    pub fn emr_containers(&self) -> emr_containers::Emr_containersService<'_> {
        emr_containers::Emr_containersService::new(self)
    }
    /// Get sagemaker_a2i_runtime service handler
    pub fn sagemaker_a2i_runtime(&self) -> sagemaker_a2i_runtime::Sagemaker_a2i_runtimeService<'_> {
        sagemaker_a2i_runtime::Sagemaker_a2i_runtimeService::new(self)
    }
    /// Get ssm_contacts service handler
    pub fn ssm_contacts(&self) -> ssm_contacts::Ssm_contactsService<'_> {
        ssm_contacts::Ssm_contactsService::new(self)
    }
    /// Get bcm_data_exports service handler
    pub fn bcm_data_exports(&self) -> bcm_data_exports::Bcm_data_exportsService<'_> {
        bcm_data_exports::Bcm_data_exportsService::new(self)
    }
    /// Get opensearch service handler
    pub fn opensearch(&self) -> opensearch::OpensearchService<'_> {
        opensearch::OpensearchService::new(self)
    }
    /// Get dax service handler
    pub fn dax(&self) -> dax::DaxService<'_> {
        dax::DaxService::new(self)
    }
    /// Get neptune service handler
    pub fn neptune(&self) -> neptune::NeptuneService<'_> {
        neptune::NeptuneService::new(self)
    }
    /// Get pricing service handler
    pub fn pricing(&self) -> pricing::PricingService<'_> {
        pricing::PricingService::new(self)
    }
    /// Get location service handler
    pub fn location(&self) -> location::LocationService<'_> {
        location::LocationService::new(self)
    }
    /// Get route53profiles service handler
    pub fn route53profiles(&self) -> route53profiles::Route53profilesService<'_> {
        route53profiles::Route53profilesService::new(self)
    }
    /// Get lambda service handler
    pub fn lambda(&self) -> lambda::LambdaService<'_> {
        lambda::LambdaService::new(self)
    }
    /// Get ivschat service handler
    pub fn ivschat(&self) -> ivschat::IvschatService<'_> {
        ivschat::IvschatService::new(self)
    }
    /// Get billing service handler
    pub fn billing(&self) -> billing::BillingService<'_> {
        billing::BillingService::new(self)
    }
    /// Get wisdom service handler
    pub fn wisdom(&self) -> wisdom::WisdomService<'_> {
        wisdom::WisdomService::new(self)
    }
    /// Get schemas service handler
    pub fn schemas(&self) -> schemas::SchemasService<'_> {
        schemas::SchemasService::new(self)
    }
    /// Get bedrock_agentcore_control service handler
    pub fn bedrock_agentcore_control(&self) -> bedrock_agentcore_control::Bedrock_agentcore_controlService<'_> {
        bedrock_agentcore_control::Bedrock_agentcore_controlService::new(self)
    }
    /// Get controlcatalog service handler
    pub fn controlcatalog(&self) -> controlcatalog::ControlcatalogService<'_> {
        controlcatalog::ControlcatalogService::new(self)
    }
    /// Get cloudsearch service handler
    pub fn cloudsearch(&self) -> cloudsearch::CloudsearchService<'_> {
        cloudsearch::CloudsearchService::new(self)
    }
    /// Get deadline service handler
    pub fn deadline(&self) -> deadline::DeadlineService<'_> {
        deadline::DeadlineService::new(self)
    }
    /// Get managedblockchain service handler
    pub fn managedblockchain(&self) -> managedblockchain::ManagedblockchainService<'_> {
        managedblockchain::ManagedblockchainService::new(self)
    }
    /// Get amplify service handler
    pub fn amplify(&self) -> amplify::AmplifyService<'_> {
        amplify::AmplifyService::new(self)
    }
    /// Get iotsecuretunneling service handler
    pub fn iotsecuretunneling(&self) -> iotsecuretunneling::IotsecuretunnelingService<'_> {
        iotsecuretunneling::IotsecuretunnelingService::new(self)
    }
    /// Get connectcampaigns service handler
    pub fn connectcampaigns(&self) -> connectcampaigns::ConnectcampaignsService<'_> {
        connectcampaigns::ConnectcampaignsService::new(self)
    }
    /// Get kafkaconnect service handler
    pub fn kafkaconnect(&self) -> kafkaconnect::KafkaconnectService<'_> {
        kafkaconnect::KafkaconnectService::new(self)
    }
    /// Get mediaconvert service handler
    pub fn mediaconvert(&self) -> mediaconvert::MediaconvertService<'_> {
        mediaconvert::MediaconvertService::new(self)
    }
    /// Get data_pipeline service handler
    pub fn data_pipeline(&self) -> data_pipeline::Data_pipelineService<'_> {
        data_pipeline::Data_pipelineService::new(self)
    }
    /// Get codepipeline service handler
    pub fn codepipeline(&self) -> codepipeline::CodepipelineService<'_> {
        codepipeline::CodepipelineService::new(self)
    }
    /// Get clouddirectory service handler
    pub fn clouddirectory(&self) -> clouddirectory::ClouddirectoryService<'_> {
        clouddirectory::ClouddirectoryService::new(self)
    }
    /// Get amplifyuibuilder service handler
    pub fn amplifyuibuilder(&self) -> amplifyuibuilder::AmplifyuibuilderService<'_> {
        amplifyuibuilder::AmplifyuibuilderService::new(self)
    }
    /// Get rtbfabric service handler
    pub fn rtbfabric(&self) -> rtbfabric::RtbfabricService<'_> {
        rtbfabric::RtbfabricService::new(self)
    }
    /// Get memorydb service handler
    pub fn memorydb(&self) -> memorydb::MemorydbService<'_> {
        memorydb::MemorydbService::new(self)
    }
    /// Get iot service handler
    pub fn iot(&self) -> iot::IotService<'_> {
        iot::IotService::new(self)
    }
    /// Get marketplace_commerce_analytics service handler
    pub fn marketplace_commerce_analytics(&self) -> marketplace_commerce_analytics::Marketplace_commerce_analyticsService<'_> {
        marketplace_commerce_analytics::Marketplace_commerce_analyticsService::new(self)
    }
    /// Get frauddetector service handler
    pub fn frauddetector(&self) -> frauddetector::FrauddetectorService<'_> {
        frauddetector::FrauddetectorService::new(self)
    }
    /// Get bedrock_data_automation service handler
    pub fn bedrock_data_automation(&self) -> bedrock_data_automation::Bedrock_data_automationService<'_> {
        bedrock_data_automation::Bedrock_data_automationService::new(self)
    }
    /// Get elastic_load_balancing service handler
    pub fn elastic_load_balancing(&self) -> elastic_load_balancing::Elastic_load_balancingService<'_> {
        elastic_load_balancing::Elastic_load_balancingService::new(self)
    }
    /// Get verifiedpermissions service handler
    pub fn verifiedpermissions(&self) -> verifiedpermissions::VerifiedpermissionsService<'_> {
        verifiedpermissions::VerifiedpermissionsService::new(self)
    }
    /// Get networkmanager service handler
    pub fn networkmanager(&self) -> networkmanager::NetworkmanagerService<'_> {
        networkmanager::NetworkmanagerService::new(self)
    }
    /// Get devops_guru service handler
    pub fn devops_guru(&self) -> devops_guru::Devops_guruService<'_> {
        devops_guru::Devops_guruService::new(self)
    }
    /// Get taxsettings service handler
    pub fn taxsettings(&self) -> taxsettings::TaxsettingsService<'_> {
        taxsettings::TaxsettingsService::new(self)
    }
    /// Get workspaces_instances service handler
    pub fn workspaces_instances(&self) -> workspaces_instances::Workspaces_instancesService<'_> {
        workspaces_instances::Workspaces_instancesService::new(self)
    }
    /// Get arc_zonal_shift service handler
    pub fn arc_zonal_shift(&self) -> arc_zonal_shift::Arc_zonal_shiftService<'_> {
        arc_zonal_shift::Arc_zonal_shiftService::new(self)
    }
    /// Get elastic_transcoder service handler
    pub fn elastic_transcoder(&self) -> elastic_transcoder::Elastic_transcoderService<'_> {
        elastic_transcoder::Elastic_transcoderService::new(self)
    }
    /// Get fms service handler
    pub fn fms(&self) -> fms::FmsService<'_> {
        fms::FmsService::new(self)
    }
    /// Get imagebuilder service handler
    pub fn imagebuilder(&self) -> imagebuilder::ImagebuilderService<'_> {
        imagebuilder::ImagebuilderService::new(self)
    }
    /// Get chime_sdk service handler
    pub fn chime_sdk(&self) -> chime_sdk::Chime_sdkService<'_> {
        chime_sdk::Chime_sdkService::new(self)
    }
    /// Get groundstation service handler
    pub fn groundstation(&self) -> groundstation::GroundstationService<'_> {
        groundstation::GroundstationService::new(self)
    }
    /// Get forecast service handler
    pub fn forecast(&self) -> forecast::ForecastService<'_> {
        forecast::ForecastService::new(self)
    }
    /// Get appstream service handler
    pub fn appstream(&self) -> appstream::AppstreamService<'_> {
        appstream::AppstreamService::new(self)
    }
    /// Get chime_sdk_meetings service handler
    pub fn chime_sdk_meetings(&self) -> chime_sdk_meetings::Chime_sdk_meetingsService<'_> {
        chime_sdk_meetings::Chime_sdk_meetingsService::new(self)
    }
    /// Get comprehend service handler
    pub fn comprehend(&self) -> comprehend::ComprehendService<'_> {
        comprehend::ComprehendService::new(self)
    }
    /// Get redshift_serverless service handler
    pub fn redshift_serverless(&self) -> redshift_serverless::Redshift_serverlessService<'_> {
        redshift_serverless::Redshift_serverlessService::new(self)
    }
    /// Get pinpoint service handler
    pub fn pinpoint(&self) -> pinpoint::PinpointService<'_> {
        pinpoint::PinpointService::new(self)
    }
    /// Get pi service handler
    pub fn pi(&self) -> pi::PiService<'_> {
        pi::PiService::new(self)
    }
    /// Get gameliftstreams service handler
    pub fn gameliftstreams(&self) -> gameliftstreams::GameliftstreamsService<'_> {
        gameliftstreams::GameliftstreamsService::new(self)
    }
    /// Get customer_profiles service handler
    pub fn customer_profiles(&self) -> customer_profiles::Customer_profilesService<'_> {
        customer_profiles::Customer_profilesService::new(self)
    }
    /// Get workspaces service handler
    pub fn workspaces(&self) -> workspaces::WorkspacesService<'_> {
        workspaces::WorkspacesService::new(self)
    }
    /// Get auditmanager service handler
    pub fn auditmanager(&self) -> auditmanager::AuditmanagerService<'_> {
        auditmanager::AuditmanagerService::new(self)
    }
    /// Get docdb service handler
    pub fn docdb(&self) -> docdb::DocdbService<'_> {
        docdb::DocdbService::new(self)
    }
    /// Get mturk service handler
    pub fn mturk(&self) -> mturk::MturkService<'_> {
        mturk::MturkService::new(self)
    }
    /// Get cognito_identity service handler
    pub fn cognito_identity(&self) -> cognito_identity::Cognito_identityService<'_> {
        cognito_identity::Cognito_identityService::new(self)
    }
    /// Get dynamodb service handler
    pub fn dynamodb(&self) -> dynamodb::DynamodbService<'_> {
        dynamodb::DynamodbService::new(self)
    }
    /// Get codeartifact service handler
    pub fn codeartifact(&self) -> codeartifact::CodeartifactService<'_> {
        codeartifact::CodeartifactService::new(self)
    }
    /// Get organizations service handler
    pub fn organizations(&self) -> organizations::OrganizationsService<'_> {
        organizations::OrganizationsService::new(self)
    }
    /// Get dlm service handler
    pub fn dlm(&self) -> dlm::DlmService<'_> {
        dlm::DlmService::new(self)
    }
    /// Get sso service handler
    pub fn sso(&self) -> sso::SsoService<'_> {
        sso::SsoService::new(self)
    }
    /// Get osis service handler
    pub fn osis(&self) -> osis::OsisService<'_> {
        osis::OsisService::new(self)
    }
    /// Get migration_hub service handler
    pub fn migration_hub(&self) -> migration_hub::Migration_hubService<'_> {
        migration_hub::Migration_hubService::new(self)
    }
    /// Get chatbot service handler
    pub fn chatbot(&self) -> chatbot::ChatbotService<'_> {
        chatbot::ChatbotService::new(self)
    }
    /// Get docdb_elastic service handler
    pub fn docdb_elastic(&self) -> docdb_elastic::Docdb_elasticService<'_> {
        docdb_elastic::Docdb_elasticService::new(self)
    }
    /// Get supplychain service handler
    pub fn supplychain(&self) -> supplychain::SupplychainService<'_> {
        supplychain::SupplychainService::new(self)
    }
    /// Get ses service handler
    pub fn ses(&self) -> ses::SesService<'_> {
        ses::SesService::new(self)
    }
    /// Get repostspace service handler
    pub fn repostspace(&self) -> repostspace::RepostspaceService<'_> {
        repostspace::RepostspaceService::new(self)
    }
    /// Get mediastore_data service handler
    pub fn mediastore_data(&self) -> mediastore_data::Mediastore_dataService<'_> {
        mediastore_data::Mediastore_dataService::new(self)
    }
    /// Get bedrock_agent service handler
    pub fn bedrock_agent(&self) -> bedrock_agent::Bedrock_agentService<'_> {
        bedrock_agent::Bedrock_agentService::new(self)
    }
    /// Get wellarchitected service handler
    pub fn wellarchitected(&self) -> wellarchitected::WellarchitectedService<'_> {
        wellarchitected::WellarchitectedService::new(self)
    }
    /// Get budgets service handler
    pub fn budgets(&self) -> budgets::BudgetsService<'_> {
        budgets::BudgetsService::new(self)
    }
    /// Get mediatailor service handler
    pub fn mediatailor(&self) -> mediatailor::MediatailorService<'_> {
        mediatailor::MediatailorService::new(self)
    }
    /// Get appsync service handler
    pub fn appsync(&self) -> appsync::AppsyncService<'_> {
        appsync::AppsyncService::new(self)
    }
    /// Get ssm_guiconnect service handler
    pub fn ssm_guiconnect(&self) -> ssm_guiconnect::Ssm_guiconnectService<'_> {
        ssm_guiconnect::Ssm_guiconnectService::new(self)
    }
    /// Get evs service handler
    pub fn evs(&self) -> evs::EvsService<'_> {
        evs::EvsService::new(self)
    }
    /// Get eks_auth service handler
    pub fn eks_auth(&self) -> eks_auth::Eks_authService<'_> {
        eks_auth::Eks_authService::new(self)
    }
    /// Get chime_sdk_messaging service handler
    pub fn chime_sdk_messaging(&self) -> chime_sdk_messaging::Chime_sdk_messagingService<'_> {
        chime_sdk_messaging::Chime_sdk_messagingService::new(self)
    }
    /// Get mediaconnect service handler
    pub fn mediaconnect(&self) -> mediaconnect::MediaconnectService<'_> {
        mediaconnect::MediaconnectService::new(self)
    }
    /// Get identitystore service handler
    pub fn identitystore(&self) -> identitystore::IdentitystoreService<'_> {
        identitystore::IdentitystoreService::new(self)
    }
    /// Get bcm_pricing_calculator service handler
    pub fn bcm_pricing_calculator(&self) -> bcm_pricing_calculator::Bcm_pricing_calculatorService<'_> {
        bcm_pricing_calculator::Bcm_pricing_calculatorService::new(self)
    }
    /// Get lakeformation service handler
    pub fn lakeformation(&self) -> lakeformation::LakeformationService<'_> {
        lakeformation::LakeformationService::new(self)
    }
    /// Get xray service handler
    pub fn xray(&self) -> xray::XrayService<'_> {
        xray::XrayService::new(self)
    }
    /// Get cloudfront service handler
    pub fn cloudfront(&self) -> cloudfront::CloudfrontService<'_> {
        cloudfront::CloudfrontService::new(self)
    }
    /// Get sagemaker_metrics service handler
    pub fn sagemaker_metrics(&self) -> sagemaker_metrics::Sagemaker_metricsService<'_> {
        sagemaker_metrics::Sagemaker_metricsService::new(self)
    }
    /// Get sso_oidc service handler
    pub fn sso_oidc(&self) -> sso_oidc::Sso_oidcService<'_> {
        sso_oidc::Sso_oidcService::new(self)
    }
    /// Get sagemaker service handler
    pub fn sagemaker(&self) -> sagemaker::SagemakerService<'_> {
        sagemaker::SagemakerService::new(self)
    }
    /// Get codestar_connections service handler
    pub fn codestar_connections(&self) -> codestar_connections::Codestar_connectionsService<'_> {
        codestar_connections::Codestar_connectionsService::new(self)
    }
    /// Get device_farm service handler
    pub fn device_farm(&self) -> device_farm::Device_farmService<'_> {
        device_farm::Device_farmService::new(self)
    }
    /// Get translate service handler
    pub fn translate(&self) -> translate::TranslateService<'_> {
        translate::TranslateService::new(self)
    }
    /// Get sagemaker_runtime service handler
    pub fn sagemaker_runtime(&self) -> sagemaker_runtime::Sagemaker_runtimeService<'_> {
        sagemaker_runtime::Sagemaker_runtimeService::new(self)
    }
    /// Get b2bi service handler
    pub fn b2bi(&self) -> b2bi::B2biService<'_> {
        b2bi::B2biService::new(self)
    }
    /// Get savingsplans service handler
    pub fn savingsplans(&self) -> savingsplans::SavingsplansService<'_> {
        savingsplans::SavingsplansService::new(self)
    }
    /// Get pipes service handler
    pub fn pipes(&self) -> pipes::PipesService<'_> {
        pipes::PipesService::new(self)
    }
    /// Get config_service service handler
    pub fn config_service(&self) -> config_service::Config_serviceService<'_> {
        config_service::Config_serviceService::new(self)
    }
    /// Get codeguruprofiler service handler
    pub fn codeguruprofiler(&self) -> codeguruprofiler::CodeguruprofilerService<'_> {
        codeguruprofiler::CodeguruprofilerService::new(self)
    }
    /// Get s3 service handler
    pub fn s3(&self) -> s3::S3Service<'_> {
        s3::S3Service::new(self)
    }
    /// Get polly service handler
    pub fn polly(&self) -> polly::PollyService<'_> {
        polly::PollyService::new(self)
    }
    /// Get cognito_sync service handler
    pub fn cognito_sync(&self) -> cognito_sync::Cognito_syncService<'_> {
        cognito_sync::Cognito_syncService::new(self)
    }
    /// Get scheduler service handler
    pub fn scheduler(&self) -> scheduler::SchedulerService<'_> {
        scheduler::SchedulerService::new(self)
    }
    /// Get pca_connector_ad service handler
    pub fn pca_connector_ad(&self) -> pca_connector_ad::Pca_connector_adService<'_> {
        pca_connector_ad::Pca_connector_adService::new(self)
    }
    /// Get waf_regional service handler
    pub fn waf_regional(&self) -> waf_regional::Waf_regionalService<'_> {
        waf_regional::Waf_regionalService::new(self)
    }
    /// Get apigatewaymanagementapi service handler
    pub fn apigatewaymanagementapi(&self) -> apigatewaymanagementapi::ApigatewaymanagementapiService<'_> {
        apigatewaymanagementapi::ApigatewaymanagementapiService::new(self)
    }
    /// Get workspaces_web service handler
    pub fn workspaces_web(&self) -> workspaces_web::Workspaces_webService<'_> {
        workspaces_web::Workspaces_webService::new(self)
    }
    /// Get pca_connector_scep service handler
    pub fn pca_connector_scep(&self) -> pca_connector_scep::Pca_connector_scepService<'_> {
        pca_connector_scep::Pca_connector_scepService::new(self)
    }
    /// Get codestar_notifications service handler
    pub fn codestar_notifications(&self) -> codestar_notifications::Codestar_notificationsService<'_> {
        codestar_notifications::Codestar_notificationsService::new(self)
    }
    /// Get direct_connect service handler
    pub fn direct_connect(&self) -> direct_connect::Direct_connectService<'_> {
        direct_connect::Direct_connectService::new(self)
    }
    /// Get shield service handler
    pub fn shield(&self) -> shield::ShieldService<'_> {
        shield::ShieldService::new(self)
    }
    /// Get application_signals service handler
    pub fn application_signals(&self) -> application_signals::Application_signalsService<'_> {
        application_signals::Application_signalsService::new(self)
    }
    /// Get iot_managed_integrations service handler
    pub fn iot_managed_integrations(&self) -> iot_managed_integrations::Iot_managed_integrationsService<'_> {
        iot_managed_integrations::Iot_managed_integrationsService::new(self)
    }
    /// Get iot_wireless service handler
    pub fn iot_wireless(&self) -> iot_wireless::Iot_wirelessService<'_> {
        iot_wireless::Iot_wirelessService::new(self)
    }
    /// Get iot_events service handler
    pub fn iot_events(&self) -> iot_events::Iot_eventsService<'_> {
        iot_events::Iot_eventsService::new(self)
    }
    /// Get backup_gateway service handler
    pub fn backup_gateway(&self) -> backup_gateway::Backup_gatewayService<'_> {
        backup_gateway::Backup_gatewayService::new(self)
    }
    /// Get sso_admin service handler
    pub fn sso_admin(&self) -> sso_admin::Sso_adminService<'_> {
        sso_admin::Sso_adminService::new(self)
    }
    /// Get elastic_beanstalk service handler
    pub fn elastic_beanstalk(&self) -> elastic_beanstalk::Elastic_beanstalkService<'_> {
        elastic_beanstalk::Elastic_beanstalkService::new(self)
    }
    /// Get drs service handler
    pub fn drs(&self) -> drs::DrsService<'_> {
        drs::DrsService::new(self)
    }
    /// Get personalize_runtime service handler
    pub fn personalize_runtime(&self) -> personalize_runtime::Personalize_runtimeService<'_> {
        personalize_runtime::Personalize_runtimeService::new(self)
    }
    /// Get outposts service handler
    pub fn outposts(&self) -> outposts::OutpostsService<'_> {
        outposts::OutpostsService::new(self)
    }
    /// Get license_manager_user_subscriptions service handler
    pub fn license_manager_user_subscriptions(&self) -> license_manager_user_subscriptions::License_manager_user_subscriptionsService<'_> {
        license_manager_user_subscriptions::License_manager_user_subscriptionsService::new(self)
    }
    /// Get cloudtrail_data service handler
    pub fn cloudtrail_data(&self) -> cloudtrail_data::Cloudtrail_dataService<'_> {
        cloudtrail_data::Cloudtrail_dataService::new(self)
    }
    /// Get lex_runtime_service service handler
    pub fn lex_runtime_service(&self) -> lex_runtime_service::Lex_runtime_serviceService<'_> {
        lex_runtime_service::Lex_runtime_serviceService::new(self)
    }


    /// Get reference to the Tokio runtime for executing async operations
    pub(crate) fn runtime(&self) -> &tokio::runtime::Runtime {
        &self.runtime
    }
}

/// Implement ProviderExecutor trait for Hemmer integration
#[async_trait]
impl ProviderExecutor for AwsProvider {
    /// Configure the provider with authentication and settings
    async fn configure(&mut self, config: ProviderConfig) -> Result<()> {
        // Configuration is already handled in new()
        // Additional runtime configuration can be added here
        Ok(())
    }

    /// Plan changes to a resource (diff current vs desired state)
    async fn plan(
        &self,
        resource_type: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // Dispatch to appropriate service based on resource_type
        // Format: "service_name.resource_name" (e.g., "s3.bucket")
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "emr_serverless" => {
                self.emr_serverless().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudformation" => {
                self.cloudformation().plan_resource(resource_name, current_state, desired_input).await
            }
            "application_auto_scaling" => {
                self.application_auto_scaling().plan_resource(resource_name, current_state, desired_input).await
            }
            "personalize_events" => {
                self.personalize_events().plan_resource(resource_name, current_state, desired_input).await
            }
            "tnb" => {
                self.tnb().plan_resource(resource_name, current_state, desired_input).await
            }
            "rolesanywhere" => {
                self.rolesanywhere().plan_resource(resource_name, current_state, desired_input).await
            }
            "kms" => {
                self.kms().plan_resource(resource_name, current_state, desired_input).await
            }
            "datasync" => {
                self.datasync().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock_agent_runtime" => {
                self.bedrock_agent_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "pinpoint_email" => {
                self.pinpoint_email().plan_resource(resource_name, current_state, desired_input).await
            }
            "connect_contact_lens" => {
                self.connect_contact_lens().plan_resource(resource_name, current_state, desired_input).await
            }
            "athena" => {
                self.athena().plan_resource(resource_name, current_state, desired_input).await
            }
            "iotfleetwise" => {
                self.iotfleetwise().plan_resource(resource_name, current_state, desired_input).await
            }
            "iot_data_plane" => {
                self.iot_data_plane().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock_data_automation_runtime" => {
                self.bedrock_data_automation_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "entityresolution" => {
                self.entityresolution().plan_resource(resource_name, current_state, desired_input).await
            }
            "forecastquery" => {
                self.forecastquery().plan_resource(resource_name, current_state, desired_input).await
            }
            "detective" => {
                self.detective().plan_resource(resource_name, current_state, desired_input).await
            }
            "panorama" => {
                self.panorama().plan_resource(resource_name, current_state, desired_input).await
            }
            "backup" => {
                self.backup().plan_resource(resource_name, current_state, desired_input).await
            }
            "mwaa" => {
                self.mwaa().plan_resource(resource_name, current_state, desired_input).await
            }
            "iot_jobs_data_plane" => {
                self.iot_jobs_data_plane().plan_resource(resource_name, current_state, desired_input).await
            }
            "transcribe" => {
                self.transcribe().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudwatch" => {
                self.cloudwatch().plan_resource(resource_name, current_state, desired_input).await
            }
            "snow_device_management" => {
                self.snow_device_management().plan_resource(resource_name, current_state, desired_input).await
            }
            "workmailmessageflow" => {
                self.workmailmessageflow().plan_resource(resource_name, current_state, desired_input).await
            }
            "appconfig" => {
                self.appconfig().plan_resource(resource_name, current_state, desired_input).await
            }
            "lightsail" => {
                self.lightsail().plan_resource(resource_name, current_state, desired_input).await
            }
            "guardduty" => {
                self.guardduty().plan_resource(resource_name, current_state, desired_input).await
            }
            "apigatewayv2" => {
                self.apigatewayv2().plan_resource(resource_name, current_state, desired_input).await
            }
            "wafv2" => {
                self.wafv2().plan_resource(resource_name, current_state, desired_input).await
            }
            "iotsitewise" => {
                self.iotsitewise().plan_resource(resource_name, current_state, desired_input).await
            }
            "iotthingsgraph" => {
                self.iotthingsgraph().plan_resource(resource_name, current_state, desired_input).await
            }
            "batch" => {
                self.batch().plan_resource(resource_name, current_state, desired_input).await
            }
            "mailmanager" => {
                self.mailmanager().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketplace_reporting" => {
                self.marketplace_reporting().plan_resource(resource_name, current_state, desired_input).await
            }
            "m2" => {
                self.m2().plan_resource(resource_name, current_state, desired_input).await
            }
            "codedeploy" => {
                self.codedeploy().plan_resource(resource_name, current_state, desired_input).await
            }
            "route53_recovery_control_config" => {
                self.route53_recovery_control_config().plan_resource(resource_name, current_state, desired_input).await
            }
            "simspaceweaver" => {
                self.simspaceweaver().plan_resource(resource_name, current_state, desired_input).await
            }
            "resiliencehub" => {
                self.resiliencehub().plan_resource(resource_name, current_state, desired_input).await
            }
            "oam" => {
                self.oam().plan_resource(resource_name, current_state, desired_input).await
            }
            "license_manager_linux_subscriptions" => {
                self.license_manager_linux_subscriptions().plan_resource(resource_name, current_state, desired_input).await
            }
            "voice_id" => {
                self.voice_id().plan_resource(resource_name, current_state, desired_input).await
            }
            "chime" => {
                self.chime().plan_resource(resource_name, current_state, desired_input).await
            }
            "efs" => {
                self.efs().plan_resource(resource_name, current_state, desired_input).await
            }
            "freetier" => {
                self.freetier().plan_resource(resource_name, current_state, desired_input).await
            }
            "storage_gateway" => {
                self.storage_gateway().plan_resource(resource_name, current_state, desired_input).await
            }
            "dynamodb_streams" => {
                self.dynamodb_streams().plan_resource(resource_name, current_state, desired_input).await
            }
            "gamelift" => {
                self.gamelift().plan_resource(resource_name, current_state, desired_input).await
            }
            "inspector2" => {
                self.inspector2().plan_resource(resource_name, current_state, desired_input).await
            }
            "keyspaces" => {
                self.keyspaces().plan_resource(resource_name, current_state, desired_input).await
            }
            "sqs" => {
                self.sqs().plan_resource(resource_name, current_state, desired_input).await
            }
            "ram" => {
                self.ram().plan_resource(resource_name, current_state, desired_input).await
            }
            "ssm_sap" => {
                self.ssm_sap().plan_resource(resource_name, current_state, desired_input).await
            }
            "directory_service_data" => {
                self.directory_service_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "route_53_domains" => {
                self.route_53_domains().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock_agentcore" => {
                self.bedrock_agentcore().plan_resource(resource_name, current_state, desired_input).await
            }
            "trustedadvisor" => {
                self.trustedadvisor().plan_resource(resource_name, current_state, desired_input).await
            }
            "migrationhubstrategy" => {
                self.migrationhubstrategy().plan_resource(resource_name, current_state, desired_input).await
            }
            "dataexchange" => {
                self.dataexchange().plan_resource(resource_name, current_state, desired_input).await
            }
            "braket" => {
                self.braket().plan_resource(resource_name, current_state, desired_input).await
            }
            "codebuild" => {
                self.codebuild().plan_resource(resource_name, current_state, desired_input).await
            }
            "acm" => {
                self.acm().plan_resource(resource_name, current_state, desired_input).await
            }
            "route53_recovery_cluster" => {
                self.route53_recovery_cluster().plan_resource(resource_name, current_state, desired_input).await
            }
            "lookoutequipment" => {
                self.lookoutequipment().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketplace_catalog" => {
                self.marketplace_catalog().plan_resource(resource_name, current_state, desired_input).await
            }
            "payment_cryptography_data" => {
                self.payment_cryptography_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloud9" => {
                self.cloud9().plan_resource(resource_name, current_state, desired_input).await
            }
            "workdocs" => {
                self.workdocs().plan_resource(resource_name, current_state, desired_input).await
            }
            "license_manager" => {
                self.license_manager().plan_resource(resource_name, current_state, desired_input).await
            }
            "sts" => {
                self.sts().plan_resource(resource_name, current_state, desired_input).await
            }
            "s3vectors" => {
                self.s3vectors().plan_resource(resource_name, current_state, desired_input).await
            }
            "chime_sdk_media_pipelines" => {
                self.chime_sdk_media_pipelines().plan_resource(resource_name, current_state, desired_input).await
            }
            "machine_learning" => {
                self.machine_learning().plan_resource(resource_name, current_state, desired_input).await
            }
            "timestream_query" => {
                self.timestream_query().plan_resource(resource_name, current_state, desired_input).await
            }
            "codeguru_reviewer" => {
                self.codeguru_reviewer().plan_resource(resource_name, current_state, desired_input).await
            }
            "mgn" => {
                self.mgn().plan_resource(resource_name, current_state, desired_input).await
            }
            "evidently" => {
                self.evidently().plan_resource(resource_name, current_state, desired_input).await
            }
            "qbusiness" => {
                self.qbusiness().plan_resource(resource_name, current_state, desired_input).await
            }
            "connectcases" => {
                self.connectcases().plan_resource(resource_name, current_state, desired_input).await
            }
            "fsx" => {
                self.fsx().plan_resource(resource_name, current_state, desired_input).await
            }
            "ecr" => {
                self.ecr().plan_resource(resource_name, current_state, desired_input).await
            }
            "connectcampaignsv2" => {
                self.connectcampaignsv2().plan_resource(resource_name, current_state, desired_input).await
            }
            "rds" => {
                self.rds().plan_resource(resource_name, current_state, desired_input).await
            }
            "qapps" => {
                self.qapps().plan_resource(resource_name, current_state, desired_input).await
            }
            "qconnect" => {
                self.qconnect().plan_resource(resource_name, current_state, desired_input).await
            }
            "omics" => {
                self.omics().plan_resource(resource_name, current_state, desired_input).await
            }
            "bcm_dashboards" => {
                self.bcm_dashboards().plan_resource(resource_name, current_state, desired_input).await
            }
            "geo_routes" => {
                self.geo_routes().plan_resource(resource_name, current_state, desired_input).await
            }
            "quicksight" => {
                self.quicksight().plan_resource(resource_name, current_state, desired_input).await
            }
            "amp" => {
                self.amp().plan_resource(resource_name, current_state, desired_input).await
            }
            "opensearchserverless" => {
                self.opensearchserverless().plan_resource(resource_name, current_state, desired_input).await
            }
            "emr" => {
                self.emr().plan_resource(resource_name, current_state, desired_input).await
            }
            "service_quotas" => {
                self.service_quotas().plan_resource(resource_name, current_state, desired_input).await
            }
            "service_catalog_appregistry" => {
                self.service_catalog_appregistry().plan_resource(resource_name, current_state, desired_input).await
            }
            "migrationhub_config" => {
                self.migrationhub_config().plan_resource(resource_name, current_state, desired_input).await
            }
            "iam" => {
                self.iam().plan_resource(resource_name, current_state, desired_input).await
            }
            "accessanalyzer" => {
                self.accessanalyzer().plan_resource(resource_name, current_state, desired_input).await
            }
            "appconfigdata" => {
                self.appconfigdata().plan_resource(resource_name, current_state, desired_input).await
            }
            "route53resolver" => {
                self.route53resolver().plan_resource(resource_name, current_state, desired_input).await
            }
            "s3outposts" => {
                self.s3outposts().plan_resource(resource_name, current_state, desired_input).await
            }
            "kendra_ranking" => {
                self.kendra_ranking().plan_resource(resource_name, current_state, desired_input).await
            }
            "controltower" => {
                self.controltower().plan_resource(resource_name, current_state, desired_input).await
            }
            "arc_region_switch" => {
                self.arc_region_switch().plan_resource(resource_name, current_state, desired_input).await
            }
            "neptune_graph" => {
                self.neptune_graph().plan_resource(resource_name, current_state, desired_input).await
            }
            "route53_recovery_readiness" => {
                self.route53_recovery_readiness().plan_resource(resource_name, current_state, desired_input).await
            }
            "greengrassv2" => {
                self.greengrassv2().plan_resource(resource_name, current_state, desired_input).await
            }
            "migration_hub_refactor_spaces" => {
                self.migration_hub_refactor_spaces().plan_resource(resource_name, current_state, desired_input).await
            }
            "cost_and_usage_report_service" => {
                self.cost_and_usage_report_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "ebs" => {
                self.ebs().plan_resource(resource_name, current_state, desired_input).await
            }
            "appflow" => {
                self.appflow().plan_resource(resource_name, current_state, desired_input).await
            }
            "migrationhuborchestrator" => {
                self.migrationhuborchestrator().plan_resource(resource_name, current_state, desired_input).await
            }
            "chime_sdk_identity" => {
                self.chime_sdk_identity().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudfront_keyvaluestore" => {
                self.cloudfront_keyvaluestore().plan_resource(resource_name, current_state, desired_input).await
            }
            "waf" => {
                self.waf().plan_resource(resource_name, current_state, desired_input).await
            }
            "greengrass" => {
                self.greengrass().plan_resource(resource_name, current_state, desired_input).await
            }
            "sagemaker_featurestore_runtime" => {
                self.sagemaker_featurestore_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "inspector" => {
                self.inspector().plan_resource(resource_name, current_state, desired_input).await
            }
            "appfabric" => {
                self.appfabric().plan_resource(resource_name, current_state, desired_input).await
            }
            "lex_model_building_service" => {
                self.lex_model_building_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "serverlessapplicationrepository" => {
                self.serverlessapplicationrepository().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudsearch_domain" => {
                self.cloudsearch_domain().plan_resource(resource_name, current_state, desired_input).await
            }
            "codeguru_security" => {
                self.codeguru_security().plan_resource(resource_name, current_state, desired_input).await
            }
            "socialmessaging" => {
                self.socialmessaging().plan_resource(resource_name, current_state, desired_input).await
            }
            "geo_maps" => {
                self.geo_maps().plan_resource(resource_name, current_state, desired_input).await
            }
            "kinesis" => {
                self.kinesis().plan_resource(resource_name, current_state, desired_input).await
            }
            "dsql" => {
                self.dsql().plan_resource(resource_name, current_state, desired_input).await
            }
            "appintegrations" => {
                self.appintegrations().plan_resource(resource_name, current_state, desired_input).await
            }
            "personalize" => {
                self.personalize().plan_resource(resource_name, current_state, desired_input).await
            }
            "proton" => {
                self.proton().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudcontrol" => {
                self.cloudcontrol().plan_resource(resource_name, current_state, desired_input).await
            }
            "redshift" => {
                self.redshift().plan_resource(resource_name, current_state, desired_input).await
            }
            "geo_places" => {
                self.geo_places().plan_resource(resource_name, current_state, desired_input).await
            }
            "elasticsearch_service" => {
                self.elasticsearch_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "bcm_recommended_actions" => {
                self.bcm_recommended_actions().plan_resource(resource_name, current_state, desired_input).await
            }
            "invoicing" => {
                self.invoicing().plan_resource(resource_name, current_state, desired_input).await
            }
            "apprunner" => {
                self.apprunner().plan_resource(resource_name, current_state, desired_input).await
            }
            "sns" => {
                self.sns().plan_resource(resource_name, current_state, desired_input).await
            }
            "textract" => {
                self.textract().plan_resource(resource_name, current_state, desired_input).await
            }
            "workmail" => {
                self.workmail().plan_resource(resource_name, current_state, desired_input).await
            }
            "datazone" => {
                self.datazone().plan_resource(resource_name, current_state, desired_input).await
            }
            "rekognition" => {
                self.rekognition().plan_resource(resource_name, current_state, desired_input).await
            }
            "ssm" => {
                self.ssm().plan_resource(resource_name, current_state, desired_input).await
            }
            "medical_imaging" => {
                self.medical_imaging().plan_resource(resource_name, current_state, desired_input).await
            }
            "lex_models" => {
                self.lex_models().plan_resource(resource_name, current_state, desired_input).await
            }
            "support" => {
                self.support().plan_resource(resource_name, current_state, desired_input).await
            }
            "signer" => {
                self.signer().plan_resource(resource_name, current_state, desired_input).await
            }
            "partnercentral_selling" => {
                self.partnercentral_selling().plan_resource(resource_name, current_state, desired_input).await
            }
            "comprehendmedical" => {
                self.comprehendmedical().plan_resource(resource_name, current_state, desired_input).await
            }
            "macie2" => {
                self.macie2().plan_resource(resource_name, current_state, desired_input).await
            }
            "redshift_data" => {
                self.redshift_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketplace_agreement" => {
                self.marketplace_agreement().plan_resource(resource_name, current_state, desired_input).await
            }
            "health" => {
                self.health().plan_resource(resource_name, current_state, desired_input).await
            }
            "odb" => {
                self.odb().plan_resource(resource_name, current_state, desired_input).await
            }
            "resource_groups_tagging_api" => {
                self.resource_groups_tagging_api().plan_resource(resource_name, current_state, desired_input).await
            }
            "application_insights" => {
                self.application_insights().plan_resource(resource_name, current_state, desired_input).await
            }
            "timestream_write" => {
                self.timestream_write().plan_resource(resource_name, current_state, desired_input).await
            }
            "pinpoint_sms" => {
                self.pinpoint_sms().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediapackagev2" => {
                self.mediapackagev2().plan_resource(resource_name, current_state, desired_input).await
            }
            "ec2" => {
                self.ec2().plan_resource(resource_name, current_state, desired_input).await
            }
            "cleanrooms" => {
                self.cleanrooms().plan_resource(resource_name, current_state, desired_input).await
            }
            "healthlake" => {
                self.healthlake().plan_resource(resource_name, current_state, desired_input).await
            }
            "sfn" => {
                self.sfn().plan_resource(resource_name, current_state, desired_input).await
            }
            "iottwinmaker" => {
                self.iottwinmaker().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudtrail" => {
                self.cloudtrail().plan_resource(resource_name, current_state, desired_input).await
            }
            "iotdeviceadvisor" => {
                self.iotdeviceadvisor().plan_resource(resource_name, current_state, desired_input).await
            }
            "ssm_incidents" => {
                self.ssm_incidents().plan_resource(resource_name, current_state, desired_input).await
            }
            "pcs" => {
                self.pcs().plan_resource(resource_name, current_state, desired_input).await
            }
            "support_app" => {
                self.support_app().plan_resource(resource_name, current_state, desired_input).await
            }
            "managedblockchain_query" => {
                self.managedblockchain_query().plan_resource(resource_name, current_state, desired_input).await
            }
            "iot_events_data" => {
                self.iot_events_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "lex_runtime" => {
                self.lex_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "observabilityadmin" => {
                self.observabilityadmin().plan_resource(resource_name, current_state, desired_input).await
            }
            "applicationcostprofiler" => {
                self.applicationcostprofiler().plan_resource(resource_name, current_state, desired_input).await
            }
            "billingconductor" => {
                self.billingconductor().plan_resource(resource_name, current_state, desired_input).await
            }
            "artifact" => {
                self.artifact().plan_resource(resource_name, current_state, desired_input).await
            }
            "ecr_public" => {
                self.ecr_public().plan_resource(resource_name, current_state, desired_input).await
            }
            "connectparticipant" => {
                self.connectparticipant().plan_resource(resource_name, current_state, desired_input).await
            }
            "rds_data" => {
                self.rds_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "internetmonitor" => {
                self.internetmonitor().plan_resource(resource_name, current_state, desired_input).await
            }
            "route_53" => {
                self.route_53().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock_runtime" => {
                self.bedrock_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "amplifybackend" => {
                self.amplifybackend().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketplace_deployment" => {
                self.marketplace_deployment().plan_resource(resource_name, current_state, desired_input).await
            }
            "account" => {
                self.account().plan_resource(resource_name, current_state, desired_input).await
            }
            "snowball" => {
                self.snowball().plan_resource(resource_name, current_state, desired_input).await
            }
            "eventbridge" => {
                self.eventbridge().plan_resource(resource_name, current_state, desired_input).await
            }
            "auto_scaling_plans" => {
                self.auto_scaling_plans().plan_resource(resource_name, current_state, desired_input).await
            }
            "directory_service" => {
                self.directory_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediapackage" => {
                self.mediapackage().plan_resource(resource_name, current_state, desired_input).await
            }
            "ssm_quicksetup" => {
                self.ssm_quicksetup().plan_resource(resource_name, current_state, desired_input).await
            }
            "s3_control" => {
                self.s3_control().plan_resource(resource_name, current_state, desired_input).await
            }
            "codecatalyst" => {
                self.codecatalyst().plan_resource(resource_name, current_state, desired_input).await
            }
            "notificationscontacts" => {
                self.notificationscontacts().plan_resource(resource_name, current_state, desired_input).await
            }
            "mpa" => {
                self.mpa().plan_resource(resource_name, current_state, desired_input).await
            }
            "ec2_instance_connect" => {
                self.ec2_instance_connect().plan_resource(resource_name, current_state, desired_input).await
            }
            "sagemaker_geospatial" => {
                self.sagemaker_geospatial().plan_resource(resource_name, current_state, desired_input).await
            }
            "notifications" => {
                self.notifications().plan_resource(resource_name, current_state, desired_input).await
            }
            "securitylake" => {
                self.securitylake().plan_resource(resource_name, current_state, desired_input).await
            }
            "networkmonitor" => {
                self.networkmonitor().plan_resource(resource_name, current_state, desired_input).await
            }
            "codeconnections" => {
                self.codeconnections().plan_resource(resource_name, current_state, desired_input).await
            }
            "app_mesh" => {
                self.app_mesh().plan_resource(resource_name, current_state, desired_input).await
            }
            "workspaces_thin_client" => {
                self.workspaces_thin_client().plan_resource(resource_name, current_state, desired_input).await
            }
            "finspace_data" => {
                self.finspace_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "compute_optimizer" => {
                self.compute_optimizer().plan_resource(resource_name, current_state, desired_input).await
            }
            "secrets_manager" => {
                self.secrets_manager().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediastore" => {
                self.mediastore().plan_resource(resource_name, current_state, desired_input).await
            }
            "ecs" => {
                self.ecs().plan_resource(resource_name, current_state, desired_input).await
            }
            "vpc_lattice" => {
                self.vpc_lattice().plan_resource(resource_name, current_state, desired_input).await
            }
            "auto_scaling" => {
                self.auto_scaling().plan_resource(resource_name, current_state, desired_input).await
            }
            "resource_groups" => {
                self.resource_groups().plan_resource(resource_name, current_state, desired_input).await
            }
            "eks" => {
                self.eks().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketplace_entitlement_service" => {
                self.marketplace_entitlement_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "database_migration_service" => {
                self.database_migration_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "security_ir" => {
                self.security_ir().plan_resource(resource_name, current_state, desired_input).await
            }
            "inspector_scan" => {
                self.inspector_scan().plan_resource(resource_name, current_state, desired_input).await
            }
            "global_accelerator" => {
                self.global_accelerator().plan_resource(resource_name, current_state, desired_input).await
            }
            "kinesis_analytics" => {
                self.kinesis_analytics().plan_resource(resource_name, current_state, desired_input).await
            }
            "neptunedata" => {
                self.neptunedata().plan_resource(resource_name, current_state, desired_input).await
            }
            "swf" => {
                self.swf().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudwatch_logs" => {
                self.cloudwatch_logs().plan_resource(resource_name, current_state, desired_input).await
            }
            "connect" => {
                self.connect().plan_resource(resource_name, current_state, desired_input).await
            }
            "glue" => {
                self.glue().plan_resource(resource_name, current_state, desired_input).await
            }
            "cognito_identity_provider" => {
                self.cognito_identity_provider().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudwatch_events" => {
                self.cloudwatch_events().plan_resource(resource_name, current_state, desired_input).await
            }
            "cost_explorer" => {
                self.cost_explorer().plan_resource(resource_name, current_state, desired_input).await
            }
            "network_firewall" => {
                self.network_firewall().plan_resource(resource_name, current_state, desired_input).await
            }
            "firehose" => {
                self.firehose().plan_resource(resource_name, current_state, desired_input).await
            }
            "transfer" => {
                self.transfer().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketplace_metering" => {
                self.marketplace_metering().plan_resource(resource_name, current_state, desired_input).await
            }
            "rbin" => {
                self.rbin().plan_resource(resource_name, current_state, desired_input).await
            }
            "timestream_influxdb" => {
                self.timestream_influxdb().plan_resource(resource_name, current_state, desired_input).await
            }
            "iotanalytics" => {
                self.iotanalytics().plan_resource(resource_name, current_state, desired_input).await
            }
            "ivs" => {
                self.ivs().plan_resource(resource_name, current_state, desired_input).await
            }
            "kafka" => {
                self.kafka().plan_resource(resource_name, current_state, desired_input).await
            }
            "sesv2" => {
                self.sesv2().plan_resource(resource_name, current_state, desired_input).await
            }
            "kendra" => {
                self.kendra().plan_resource(resource_name, current_state, desired_input).await
            }
            "sagemaker_edge" => {
                self.sagemaker_edge().plan_resource(resource_name, current_state, desired_input).await
            }
            "launch_wizard" => {
                self.launch_wizard().plan_resource(resource_name, current_state, desired_input).await
            }
            "securityhub" => {
                self.securityhub().plan_resource(resource_name, current_state, desired_input).await
            }
            "finspace" => {
                self.finspace().plan_resource(resource_name, current_state, desired_input).await
            }
            "keyspacesstreams" => {
                self.keyspacesstreams().plan_resource(resource_name, current_state, desired_input).await
            }
            "cleanroomsml" => {
                self.cleanroomsml().plan_resource(resource_name, current_state, desired_input).await
            }
            "transcribe_streaming" => {
                self.transcribe_streaming().plan_resource(resource_name, current_state, desired_input).await
            }
            "aiops" => {
                self.aiops().plan_resource(resource_name, current_state, desired_input).await
            }
            "service_catalog" => {
                self.service_catalog().plan_resource(resource_name, current_state, desired_input).await
            }
            "databrew" => {
                self.databrew().plan_resource(resource_name, current_state, desired_input).await
            }
            "codecommit" => {
                self.codecommit().plan_resource(resource_name, current_state, desired_input).await
            }
            "resource_explorer_2" => {
                self.resource_explorer_2().plan_resource(resource_name, current_state, desired_input).await
            }
            "acm_pca" => {
                self.acm_pca().plan_resource(resource_name, current_state, desired_input).await
            }
            "payment_cryptography" => {
                self.payment_cryptography().plan_resource(resource_name, current_state, desired_input).await
            }
            "mq" => {
                self.mq().plan_resource(resource_name, current_state, desired_input).await
            }
            "api_gateway" => {
                self.api_gateway().plan_resource(resource_name, current_state, desired_input).await
            }
            "grafana" => {
                self.grafana().plan_resource(resource_name, current_state, desired_input).await
            }
            "glacier" => {
                self.glacier().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock" => {
                self.bedrock().plan_resource(resource_name, current_state, desired_input).await
            }
            "s3tables" => {
                self.s3tables().plan_resource(resource_name, current_state, desired_input).await
            }
            "ivs_realtime" => {
                self.ivs_realtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "medialive" => {
                self.medialive().plan_resource(resource_name, current_state, desired_input).await
            }
            "backupsearch" => {
                self.backupsearch().plan_resource(resource_name, current_state, desired_input).await
            }
            "networkflowmonitor" => {
                self.networkflowmonitor().plan_resource(resource_name, current_state, desired_input).await
            }
            "elasticache" => {
                self.elasticache().plan_resource(resource_name, current_state, desired_input).await
            }
            "fis" => {
                self.fis().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudhsm" => {
                self.cloudhsm().plan_resource(resource_name, current_state, desired_input).await
            }
            "cost_optimization_hub" => {
                self.cost_optimization_hub().plan_resource(resource_name, current_state, desired_input).await
            }
            "synthetics" => {
                self.synthetics().plan_resource(resource_name, current_state, desired_input).await
            }
            "rum" => {
                self.rum().plan_resource(resource_name, current_state, desired_input).await
            }
            "emr_containers" => {
                self.emr_containers().plan_resource(resource_name, current_state, desired_input).await
            }
            "sagemaker_a2i_runtime" => {
                self.sagemaker_a2i_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "ssm_contacts" => {
                self.ssm_contacts().plan_resource(resource_name, current_state, desired_input).await
            }
            "bcm_data_exports" => {
                self.bcm_data_exports().plan_resource(resource_name, current_state, desired_input).await
            }
            "opensearch" => {
                self.opensearch().plan_resource(resource_name, current_state, desired_input).await
            }
            "dax" => {
                self.dax().plan_resource(resource_name, current_state, desired_input).await
            }
            "neptune" => {
                self.neptune().plan_resource(resource_name, current_state, desired_input).await
            }
            "pricing" => {
                self.pricing().plan_resource(resource_name, current_state, desired_input).await
            }
            "location" => {
                self.location().plan_resource(resource_name, current_state, desired_input).await
            }
            "route53profiles" => {
                self.route53profiles().plan_resource(resource_name, current_state, desired_input).await
            }
            "lambda" => {
                self.lambda().plan_resource(resource_name, current_state, desired_input).await
            }
            "ivschat" => {
                self.ivschat().plan_resource(resource_name, current_state, desired_input).await
            }
            "billing" => {
                self.billing().plan_resource(resource_name, current_state, desired_input).await
            }
            "wisdom" => {
                self.wisdom().plan_resource(resource_name, current_state, desired_input).await
            }
            "schemas" => {
                self.schemas().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock_agentcore_control" => {
                self.bedrock_agentcore_control().plan_resource(resource_name, current_state, desired_input).await
            }
            "controlcatalog" => {
                self.controlcatalog().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudsearch" => {
                self.cloudsearch().plan_resource(resource_name, current_state, desired_input).await
            }
            "deadline" => {
                self.deadline().plan_resource(resource_name, current_state, desired_input).await
            }
            "managedblockchain" => {
                self.managedblockchain().plan_resource(resource_name, current_state, desired_input).await
            }
            "amplify" => {
                self.amplify().plan_resource(resource_name, current_state, desired_input).await
            }
            "iotsecuretunneling" => {
                self.iotsecuretunneling().plan_resource(resource_name, current_state, desired_input).await
            }
            "connectcampaigns" => {
                self.connectcampaigns().plan_resource(resource_name, current_state, desired_input).await
            }
            "kafkaconnect" => {
                self.kafkaconnect().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediaconvert" => {
                self.mediaconvert().plan_resource(resource_name, current_state, desired_input).await
            }
            "data_pipeline" => {
                self.data_pipeline().plan_resource(resource_name, current_state, desired_input).await
            }
            "codepipeline" => {
                self.codepipeline().plan_resource(resource_name, current_state, desired_input).await
            }
            "clouddirectory" => {
                self.clouddirectory().plan_resource(resource_name, current_state, desired_input).await
            }
            "amplifyuibuilder" => {
                self.amplifyuibuilder().plan_resource(resource_name, current_state, desired_input).await
            }
            "rtbfabric" => {
                self.rtbfabric().plan_resource(resource_name, current_state, desired_input).await
            }
            "memorydb" => {
                self.memorydb().plan_resource(resource_name, current_state, desired_input).await
            }
            "iot" => {
                self.iot().plan_resource(resource_name, current_state, desired_input).await
            }
            "marketplace_commerce_analytics" => {
                self.marketplace_commerce_analytics().plan_resource(resource_name, current_state, desired_input).await
            }
            "frauddetector" => {
                self.frauddetector().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock_data_automation" => {
                self.bedrock_data_automation().plan_resource(resource_name, current_state, desired_input).await
            }
            "elastic_load_balancing" => {
                self.elastic_load_balancing().plan_resource(resource_name, current_state, desired_input).await
            }
            "verifiedpermissions" => {
                self.verifiedpermissions().plan_resource(resource_name, current_state, desired_input).await
            }
            "networkmanager" => {
                self.networkmanager().plan_resource(resource_name, current_state, desired_input).await
            }
            "devops_guru" => {
                self.devops_guru().plan_resource(resource_name, current_state, desired_input).await
            }
            "taxsettings" => {
                self.taxsettings().plan_resource(resource_name, current_state, desired_input).await
            }
            "workspaces_instances" => {
                self.workspaces_instances().plan_resource(resource_name, current_state, desired_input).await
            }
            "arc_zonal_shift" => {
                self.arc_zonal_shift().plan_resource(resource_name, current_state, desired_input).await
            }
            "elastic_transcoder" => {
                self.elastic_transcoder().plan_resource(resource_name, current_state, desired_input).await
            }
            "fms" => {
                self.fms().plan_resource(resource_name, current_state, desired_input).await
            }
            "imagebuilder" => {
                self.imagebuilder().plan_resource(resource_name, current_state, desired_input).await
            }
            "chime_sdk" => {
                self.chime_sdk().plan_resource(resource_name, current_state, desired_input).await
            }
            "groundstation" => {
                self.groundstation().plan_resource(resource_name, current_state, desired_input).await
            }
            "forecast" => {
                self.forecast().plan_resource(resource_name, current_state, desired_input).await
            }
            "appstream" => {
                self.appstream().plan_resource(resource_name, current_state, desired_input).await
            }
            "chime_sdk_meetings" => {
                self.chime_sdk_meetings().plan_resource(resource_name, current_state, desired_input).await
            }
            "comprehend" => {
                self.comprehend().plan_resource(resource_name, current_state, desired_input).await
            }
            "redshift_serverless" => {
                self.redshift_serverless().plan_resource(resource_name, current_state, desired_input).await
            }
            "pinpoint" => {
                self.pinpoint().plan_resource(resource_name, current_state, desired_input).await
            }
            "pi" => {
                self.pi().plan_resource(resource_name, current_state, desired_input).await
            }
            "gameliftstreams" => {
                self.gameliftstreams().plan_resource(resource_name, current_state, desired_input).await
            }
            "customer_profiles" => {
                self.customer_profiles().plan_resource(resource_name, current_state, desired_input).await
            }
            "workspaces" => {
                self.workspaces().plan_resource(resource_name, current_state, desired_input).await
            }
            "auditmanager" => {
                self.auditmanager().plan_resource(resource_name, current_state, desired_input).await
            }
            "docdb" => {
                self.docdb().plan_resource(resource_name, current_state, desired_input).await
            }
            "mturk" => {
                self.mturk().plan_resource(resource_name, current_state, desired_input).await
            }
            "cognito_identity" => {
                self.cognito_identity().plan_resource(resource_name, current_state, desired_input).await
            }
            "dynamodb" => {
                self.dynamodb().plan_resource(resource_name, current_state, desired_input).await
            }
            "codeartifact" => {
                self.codeartifact().plan_resource(resource_name, current_state, desired_input).await
            }
            "organizations" => {
                self.organizations().plan_resource(resource_name, current_state, desired_input).await
            }
            "dlm" => {
                self.dlm().plan_resource(resource_name, current_state, desired_input).await
            }
            "sso" => {
                self.sso().plan_resource(resource_name, current_state, desired_input).await
            }
            "osis" => {
                self.osis().plan_resource(resource_name, current_state, desired_input).await
            }
            "migration_hub" => {
                self.migration_hub().plan_resource(resource_name, current_state, desired_input).await
            }
            "chatbot" => {
                self.chatbot().plan_resource(resource_name, current_state, desired_input).await
            }
            "docdb_elastic" => {
                self.docdb_elastic().plan_resource(resource_name, current_state, desired_input).await
            }
            "supplychain" => {
                self.supplychain().plan_resource(resource_name, current_state, desired_input).await
            }
            "ses" => {
                self.ses().plan_resource(resource_name, current_state, desired_input).await
            }
            "repostspace" => {
                self.repostspace().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediastore_data" => {
                self.mediastore_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "bedrock_agent" => {
                self.bedrock_agent().plan_resource(resource_name, current_state, desired_input).await
            }
            "wellarchitected" => {
                self.wellarchitected().plan_resource(resource_name, current_state, desired_input).await
            }
            "budgets" => {
                self.budgets().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediatailor" => {
                self.mediatailor().plan_resource(resource_name, current_state, desired_input).await
            }
            "appsync" => {
                self.appsync().plan_resource(resource_name, current_state, desired_input).await
            }
            "ssm_guiconnect" => {
                self.ssm_guiconnect().plan_resource(resource_name, current_state, desired_input).await
            }
            "evs" => {
                self.evs().plan_resource(resource_name, current_state, desired_input).await
            }
            "eks_auth" => {
                self.eks_auth().plan_resource(resource_name, current_state, desired_input).await
            }
            "chime_sdk_messaging" => {
                self.chime_sdk_messaging().plan_resource(resource_name, current_state, desired_input).await
            }
            "mediaconnect" => {
                self.mediaconnect().plan_resource(resource_name, current_state, desired_input).await
            }
            "identitystore" => {
                self.identitystore().plan_resource(resource_name, current_state, desired_input).await
            }
            "bcm_pricing_calculator" => {
                self.bcm_pricing_calculator().plan_resource(resource_name, current_state, desired_input).await
            }
            "lakeformation" => {
                self.lakeformation().plan_resource(resource_name, current_state, desired_input).await
            }
            "xray" => {
                self.xray().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudfront" => {
                self.cloudfront().plan_resource(resource_name, current_state, desired_input).await
            }
            "sagemaker_metrics" => {
                self.sagemaker_metrics().plan_resource(resource_name, current_state, desired_input).await
            }
            "sso_oidc" => {
                self.sso_oidc().plan_resource(resource_name, current_state, desired_input).await
            }
            "sagemaker" => {
                self.sagemaker().plan_resource(resource_name, current_state, desired_input).await
            }
            "codestar_connections" => {
                self.codestar_connections().plan_resource(resource_name, current_state, desired_input).await
            }
            "device_farm" => {
                self.device_farm().plan_resource(resource_name, current_state, desired_input).await
            }
            "translate" => {
                self.translate().plan_resource(resource_name, current_state, desired_input).await
            }
            "sagemaker_runtime" => {
                self.sagemaker_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "b2bi" => {
                self.b2bi().plan_resource(resource_name, current_state, desired_input).await
            }
            "savingsplans" => {
                self.savingsplans().plan_resource(resource_name, current_state, desired_input).await
            }
            "pipes" => {
                self.pipes().plan_resource(resource_name, current_state, desired_input).await
            }
            "config_service" => {
                self.config_service().plan_resource(resource_name, current_state, desired_input).await
            }
            "codeguruprofiler" => {
                self.codeguruprofiler().plan_resource(resource_name, current_state, desired_input).await
            }
            "s3" => {
                self.s3().plan_resource(resource_name, current_state, desired_input).await
            }
            "polly" => {
                self.polly().plan_resource(resource_name, current_state, desired_input).await
            }
            "cognito_sync" => {
                self.cognito_sync().plan_resource(resource_name, current_state, desired_input).await
            }
            "scheduler" => {
                self.scheduler().plan_resource(resource_name, current_state, desired_input).await
            }
            "pca_connector_ad" => {
                self.pca_connector_ad().plan_resource(resource_name, current_state, desired_input).await
            }
            "waf_regional" => {
                self.waf_regional().plan_resource(resource_name, current_state, desired_input).await
            }
            "apigatewaymanagementapi" => {
                self.apigatewaymanagementapi().plan_resource(resource_name, current_state, desired_input).await
            }
            "workspaces_web" => {
                self.workspaces_web().plan_resource(resource_name, current_state, desired_input).await
            }
            "pca_connector_scep" => {
                self.pca_connector_scep().plan_resource(resource_name, current_state, desired_input).await
            }
            "codestar_notifications" => {
                self.codestar_notifications().plan_resource(resource_name, current_state, desired_input).await
            }
            "direct_connect" => {
                self.direct_connect().plan_resource(resource_name, current_state, desired_input).await
            }
            "shield" => {
                self.shield().plan_resource(resource_name, current_state, desired_input).await
            }
            "application_signals" => {
                self.application_signals().plan_resource(resource_name, current_state, desired_input).await
            }
            "iot_managed_integrations" => {
                self.iot_managed_integrations().plan_resource(resource_name, current_state, desired_input).await
            }
            "iot_wireless" => {
                self.iot_wireless().plan_resource(resource_name, current_state, desired_input).await
            }
            "iot_events" => {
                self.iot_events().plan_resource(resource_name, current_state, desired_input).await
            }
            "backup_gateway" => {
                self.backup_gateway().plan_resource(resource_name, current_state, desired_input).await
            }
            "sso_admin" => {
                self.sso_admin().plan_resource(resource_name, current_state, desired_input).await
            }
            "elastic_beanstalk" => {
                self.elastic_beanstalk().plan_resource(resource_name, current_state, desired_input).await
            }
            "drs" => {
                self.drs().plan_resource(resource_name, current_state, desired_input).await
            }
            "personalize_runtime" => {
                self.personalize_runtime().plan_resource(resource_name, current_state, desired_input).await
            }
            "outposts" => {
                self.outposts().plan_resource(resource_name, current_state, desired_input).await
            }
            "license_manager_user_subscriptions" => {
                self.license_manager_user_subscriptions().plan_resource(resource_name, current_state, desired_input).await
            }
            "cloudtrail_data" => {
                self.cloudtrail_data().plan_resource(resource_name, current_state, desired_input).await
            }
            "lex_runtime_service" => {
                self.lex_runtime_service().plan_resource(resource_name, current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Create a new resource
    async fn create(&self, resource_type: &str, input: ResourceInput) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "emr_serverless" => {
                self.emr_serverless().create_resource(resource_name, input).await
            }
            "cloudformation" => {
                self.cloudformation().create_resource(resource_name, input).await
            }
            "application_auto_scaling" => {
                self.application_auto_scaling().create_resource(resource_name, input).await
            }
            "personalize_events" => {
                self.personalize_events().create_resource(resource_name, input).await
            }
            "tnb" => {
                self.tnb().create_resource(resource_name, input).await
            }
            "rolesanywhere" => {
                self.rolesanywhere().create_resource(resource_name, input).await
            }
            "kms" => {
                self.kms().create_resource(resource_name, input).await
            }
            "datasync" => {
                self.datasync().create_resource(resource_name, input).await
            }
            "bedrock_agent_runtime" => {
                self.bedrock_agent_runtime().create_resource(resource_name, input).await
            }
            "pinpoint_email" => {
                self.pinpoint_email().create_resource(resource_name, input).await
            }
            "connect_contact_lens" => {
                self.connect_contact_lens().create_resource(resource_name, input).await
            }
            "athena" => {
                self.athena().create_resource(resource_name, input).await
            }
            "iotfleetwise" => {
                self.iotfleetwise().create_resource(resource_name, input).await
            }
            "iot_data_plane" => {
                self.iot_data_plane().create_resource(resource_name, input).await
            }
            "bedrock_data_automation_runtime" => {
                self.bedrock_data_automation_runtime().create_resource(resource_name, input).await
            }
            "entityresolution" => {
                self.entityresolution().create_resource(resource_name, input).await
            }
            "forecastquery" => {
                self.forecastquery().create_resource(resource_name, input).await
            }
            "detective" => {
                self.detective().create_resource(resource_name, input).await
            }
            "panorama" => {
                self.panorama().create_resource(resource_name, input).await
            }
            "backup" => {
                self.backup().create_resource(resource_name, input).await
            }
            "mwaa" => {
                self.mwaa().create_resource(resource_name, input).await
            }
            "iot_jobs_data_plane" => {
                self.iot_jobs_data_plane().create_resource(resource_name, input).await
            }
            "transcribe" => {
                self.transcribe().create_resource(resource_name, input).await
            }
            "cloudwatch" => {
                self.cloudwatch().create_resource(resource_name, input).await
            }
            "snow_device_management" => {
                self.snow_device_management().create_resource(resource_name, input).await
            }
            "workmailmessageflow" => {
                self.workmailmessageflow().create_resource(resource_name, input).await
            }
            "appconfig" => {
                self.appconfig().create_resource(resource_name, input).await
            }
            "lightsail" => {
                self.lightsail().create_resource(resource_name, input).await
            }
            "guardduty" => {
                self.guardduty().create_resource(resource_name, input).await
            }
            "apigatewayv2" => {
                self.apigatewayv2().create_resource(resource_name, input).await
            }
            "wafv2" => {
                self.wafv2().create_resource(resource_name, input).await
            }
            "iotsitewise" => {
                self.iotsitewise().create_resource(resource_name, input).await
            }
            "iotthingsgraph" => {
                self.iotthingsgraph().create_resource(resource_name, input).await
            }
            "batch" => {
                self.batch().create_resource(resource_name, input).await
            }
            "mailmanager" => {
                self.mailmanager().create_resource(resource_name, input).await
            }
            "marketplace_reporting" => {
                self.marketplace_reporting().create_resource(resource_name, input).await
            }
            "m2" => {
                self.m2().create_resource(resource_name, input).await
            }
            "codedeploy" => {
                self.codedeploy().create_resource(resource_name, input).await
            }
            "route53_recovery_control_config" => {
                self.route53_recovery_control_config().create_resource(resource_name, input).await
            }
            "simspaceweaver" => {
                self.simspaceweaver().create_resource(resource_name, input).await
            }
            "resiliencehub" => {
                self.resiliencehub().create_resource(resource_name, input).await
            }
            "oam" => {
                self.oam().create_resource(resource_name, input).await
            }
            "license_manager_linux_subscriptions" => {
                self.license_manager_linux_subscriptions().create_resource(resource_name, input).await
            }
            "voice_id" => {
                self.voice_id().create_resource(resource_name, input).await
            }
            "chime" => {
                self.chime().create_resource(resource_name, input).await
            }
            "efs" => {
                self.efs().create_resource(resource_name, input).await
            }
            "freetier" => {
                self.freetier().create_resource(resource_name, input).await
            }
            "storage_gateway" => {
                self.storage_gateway().create_resource(resource_name, input).await
            }
            "dynamodb_streams" => {
                self.dynamodb_streams().create_resource(resource_name, input).await
            }
            "gamelift" => {
                self.gamelift().create_resource(resource_name, input).await
            }
            "inspector2" => {
                self.inspector2().create_resource(resource_name, input).await
            }
            "keyspaces" => {
                self.keyspaces().create_resource(resource_name, input).await
            }
            "sqs" => {
                self.sqs().create_resource(resource_name, input).await
            }
            "ram" => {
                self.ram().create_resource(resource_name, input).await
            }
            "ssm_sap" => {
                self.ssm_sap().create_resource(resource_name, input).await
            }
            "directory_service_data" => {
                self.directory_service_data().create_resource(resource_name, input).await
            }
            "route_53_domains" => {
                self.route_53_domains().create_resource(resource_name, input).await
            }
            "bedrock_agentcore" => {
                self.bedrock_agentcore().create_resource(resource_name, input).await
            }
            "trustedadvisor" => {
                self.trustedadvisor().create_resource(resource_name, input).await
            }
            "migrationhubstrategy" => {
                self.migrationhubstrategy().create_resource(resource_name, input).await
            }
            "dataexchange" => {
                self.dataexchange().create_resource(resource_name, input).await
            }
            "braket" => {
                self.braket().create_resource(resource_name, input).await
            }
            "codebuild" => {
                self.codebuild().create_resource(resource_name, input).await
            }
            "acm" => {
                self.acm().create_resource(resource_name, input).await
            }
            "route53_recovery_cluster" => {
                self.route53_recovery_cluster().create_resource(resource_name, input).await
            }
            "lookoutequipment" => {
                self.lookoutequipment().create_resource(resource_name, input).await
            }
            "marketplace_catalog" => {
                self.marketplace_catalog().create_resource(resource_name, input).await
            }
            "payment_cryptography_data" => {
                self.payment_cryptography_data().create_resource(resource_name, input).await
            }
            "cloud9" => {
                self.cloud9().create_resource(resource_name, input).await
            }
            "workdocs" => {
                self.workdocs().create_resource(resource_name, input).await
            }
            "license_manager" => {
                self.license_manager().create_resource(resource_name, input).await
            }
            "sts" => {
                self.sts().create_resource(resource_name, input).await
            }
            "s3vectors" => {
                self.s3vectors().create_resource(resource_name, input).await
            }
            "chime_sdk_media_pipelines" => {
                self.chime_sdk_media_pipelines().create_resource(resource_name, input).await
            }
            "machine_learning" => {
                self.machine_learning().create_resource(resource_name, input).await
            }
            "timestream_query" => {
                self.timestream_query().create_resource(resource_name, input).await
            }
            "codeguru_reviewer" => {
                self.codeguru_reviewer().create_resource(resource_name, input).await
            }
            "mgn" => {
                self.mgn().create_resource(resource_name, input).await
            }
            "evidently" => {
                self.evidently().create_resource(resource_name, input).await
            }
            "qbusiness" => {
                self.qbusiness().create_resource(resource_name, input).await
            }
            "connectcases" => {
                self.connectcases().create_resource(resource_name, input).await
            }
            "fsx" => {
                self.fsx().create_resource(resource_name, input).await
            }
            "ecr" => {
                self.ecr().create_resource(resource_name, input).await
            }
            "connectcampaignsv2" => {
                self.connectcampaignsv2().create_resource(resource_name, input).await
            }
            "rds" => {
                self.rds().create_resource(resource_name, input).await
            }
            "qapps" => {
                self.qapps().create_resource(resource_name, input).await
            }
            "qconnect" => {
                self.qconnect().create_resource(resource_name, input).await
            }
            "omics" => {
                self.omics().create_resource(resource_name, input).await
            }
            "bcm_dashboards" => {
                self.bcm_dashboards().create_resource(resource_name, input).await
            }
            "geo_routes" => {
                self.geo_routes().create_resource(resource_name, input).await
            }
            "quicksight" => {
                self.quicksight().create_resource(resource_name, input).await
            }
            "amp" => {
                self.amp().create_resource(resource_name, input).await
            }
            "opensearchserverless" => {
                self.opensearchserverless().create_resource(resource_name, input).await
            }
            "emr" => {
                self.emr().create_resource(resource_name, input).await
            }
            "service_quotas" => {
                self.service_quotas().create_resource(resource_name, input).await
            }
            "service_catalog_appregistry" => {
                self.service_catalog_appregistry().create_resource(resource_name, input).await
            }
            "migrationhub_config" => {
                self.migrationhub_config().create_resource(resource_name, input).await
            }
            "iam" => {
                self.iam().create_resource(resource_name, input).await
            }
            "accessanalyzer" => {
                self.accessanalyzer().create_resource(resource_name, input).await
            }
            "appconfigdata" => {
                self.appconfigdata().create_resource(resource_name, input).await
            }
            "route53resolver" => {
                self.route53resolver().create_resource(resource_name, input).await
            }
            "s3outposts" => {
                self.s3outposts().create_resource(resource_name, input).await
            }
            "kendra_ranking" => {
                self.kendra_ranking().create_resource(resource_name, input).await
            }
            "controltower" => {
                self.controltower().create_resource(resource_name, input).await
            }
            "arc_region_switch" => {
                self.arc_region_switch().create_resource(resource_name, input).await
            }
            "neptune_graph" => {
                self.neptune_graph().create_resource(resource_name, input).await
            }
            "route53_recovery_readiness" => {
                self.route53_recovery_readiness().create_resource(resource_name, input).await
            }
            "greengrassv2" => {
                self.greengrassv2().create_resource(resource_name, input).await
            }
            "migration_hub_refactor_spaces" => {
                self.migration_hub_refactor_spaces().create_resource(resource_name, input).await
            }
            "cost_and_usage_report_service" => {
                self.cost_and_usage_report_service().create_resource(resource_name, input).await
            }
            "ebs" => {
                self.ebs().create_resource(resource_name, input).await
            }
            "appflow" => {
                self.appflow().create_resource(resource_name, input).await
            }
            "migrationhuborchestrator" => {
                self.migrationhuborchestrator().create_resource(resource_name, input).await
            }
            "chime_sdk_identity" => {
                self.chime_sdk_identity().create_resource(resource_name, input).await
            }
            "cloudfront_keyvaluestore" => {
                self.cloudfront_keyvaluestore().create_resource(resource_name, input).await
            }
            "waf" => {
                self.waf().create_resource(resource_name, input).await
            }
            "greengrass" => {
                self.greengrass().create_resource(resource_name, input).await
            }
            "sagemaker_featurestore_runtime" => {
                self.sagemaker_featurestore_runtime().create_resource(resource_name, input).await
            }
            "inspector" => {
                self.inspector().create_resource(resource_name, input).await
            }
            "appfabric" => {
                self.appfabric().create_resource(resource_name, input).await
            }
            "lex_model_building_service" => {
                self.lex_model_building_service().create_resource(resource_name, input).await
            }
            "serverlessapplicationrepository" => {
                self.serverlessapplicationrepository().create_resource(resource_name, input).await
            }
            "cloudsearch_domain" => {
                self.cloudsearch_domain().create_resource(resource_name, input).await
            }
            "codeguru_security" => {
                self.codeguru_security().create_resource(resource_name, input).await
            }
            "socialmessaging" => {
                self.socialmessaging().create_resource(resource_name, input).await
            }
            "geo_maps" => {
                self.geo_maps().create_resource(resource_name, input).await
            }
            "kinesis" => {
                self.kinesis().create_resource(resource_name, input).await
            }
            "dsql" => {
                self.dsql().create_resource(resource_name, input).await
            }
            "appintegrations" => {
                self.appintegrations().create_resource(resource_name, input).await
            }
            "personalize" => {
                self.personalize().create_resource(resource_name, input).await
            }
            "proton" => {
                self.proton().create_resource(resource_name, input).await
            }
            "cloudcontrol" => {
                self.cloudcontrol().create_resource(resource_name, input).await
            }
            "redshift" => {
                self.redshift().create_resource(resource_name, input).await
            }
            "geo_places" => {
                self.geo_places().create_resource(resource_name, input).await
            }
            "elasticsearch_service" => {
                self.elasticsearch_service().create_resource(resource_name, input).await
            }
            "bcm_recommended_actions" => {
                self.bcm_recommended_actions().create_resource(resource_name, input).await
            }
            "invoicing" => {
                self.invoicing().create_resource(resource_name, input).await
            }
            "apprunner" => {
                self.apprunner().create_resource(resource_name, input).await
            }
            "sns" => {
                self.sns().create_resource(resource_name, input).await
            }
            "textract" => {
                self.textract().create_resource(resource_name, input).await
            }
            "workmail" => {
                self.workmail().create_resource(resource_name, input).await
            }
            "datazone" => {
                self.datazone().create_resource(resource_name, input).await
            }
            "rekognition" => {
                self.rekognition().create_resource(resource_name, input).await
            }
            "ssm" => {
                self.ssm().create_resource(resource_name, input).await
            }
            "medical_imaging" => {
                self.medical_imaging().create_resource(resource_name, input).await
            }
            "lex_models" => {
                self.lex_models().create_resource(resource_name, input).await
            }
            "support" => {
                self.support().create_resource(resource_name, input).await
            }
            "signer" => {
                self.signer().create_resource(resource_name, input).await
            }
            "partnercentral_selling" => {
                self.partnercentral_selling().create_resource(resource_name, input).await
            }
            "comprehendmedical" => {
                self.comprehendmedical().create_resource(resource_name, input).await
            }
            "macie2" => {
                self.macie2().create_resource(resource_name, input).await
            }
            "redshift_data" => {
                self.redshift_data().create_resource(resource_name, input).await
            }
            "marketplace_agreement" => {
                self.marketplace_agreement().create_resource(resource_name, input).await
            }
            "health" => {
                self.health().create_resource(resource_name, input).await
            }
            "odb" => {
                self.odb().create_resource(resource_name, input).await
            }
            "resource_groups_tagging_api" => {
                self.resource_groups_tagging_api().create_resource(resource_name, input).await
            }
            "application_insights" => {
                self.application_insights().create_resource(resource_name, input).await
            }
            "timestream_write" => {
                self.timestream_write().create_resource(resource_name, input).await
            }
            "pinpoint_sms" => {
                self.pinpoint_sms().create_resource(resource_name, input).await
            }
            "mediapackagev2" => {
                self.mediapackagev2().create_resource(resource_name, input).await
            }
            "ec2" => {
                self.ec2().create_resource(resource_name, input).await
            }
            "cleanrooms" => {
                self.cleanrooms().create_resource(resource_name, input).await
            }
            "healthlake" => {
                self.healthlake().create_resource(resource_name, input).await
            }
            "sfn" => {
                self.sfn().create_resource(resource_name, input).await
            }
            "iottwinmaker" => {
                self.iottwinmaker().create_resource(resource_name, input).await
            }
            "cloudtrail" => {
                self.cloudtrail().create_resource(resource_name, input).await
            }
            "iotdeviceadvisor" => {
                self.iotdeviceadvisor().create_resource(resource_name, input).await
            }
            "ssm_incidents" => {
                self.ssm_incidents().create_resource(resource_name, input).await
            }
            "pcs" => {
                self.pcs().create_resource(resource_name, input).await
            }
            "support_app" => {
                self.support_app().create_resource(resource_name, input).await
            }
            "managedblockchain_query" => {
                self.managedblockchain_query().create_resource(resource_name, input).await
            }
            "iot_events_data" => {
                self.iot_events_data().create_resource(resource_name, input).await
            }
            "lex_runtime" => {
                self.lex_runtime().create_resource(resource_name, input).await
            }
            "observabilityadmin" => {
                self.observabilityadmin().create_resource(resource_name, input).await
            }
            "applicationcostprofiler" => {
                self.applicationcostprofiler().create_resource(resource_name, input).await
            }
            "billingconductor" => {
                self.billingconductor().create_resource(resource_name, input).await
            }
            "artifact" => {
                self.artifact().create_resource(resource_name, input).await
            }
            "ecr_public" => {
                self.ecr_public().create_resource(resource_name, input).await
            }
            "connectparticipant" => {
                self.connectparticipant().create_resource(resource_name, input).await
            }
            "rds_data" => {
                self.rds_data().create_resource(resource_name, input).await
            }
            "internetmonitor" => {
                self.internetmonitor().create_resource(resource_name, input).await
            }
            "route_53" => {
                self.route_53().create_resource(resource_name, input).await
            }
            "bedrock_runtime" => {
                self.bedrock_runtime().create_resource(resource_name, input).await
            }
            "amplifybackend" => {
                self.amplifybackend().create_resource(resource_name, input).await
            }
            "marketplace_deployment" => {
                self.marketplace_deployment().create_resource(resource_name, input).await
            }
            "account" => {
                self.account().create_resource(resource_name, input).await
            }
            "snowball" => {
                self.snowball().create_resource(resource_name, input).await
            }
            "eventbridge" => {
                self.eventbridge().create_resource(resource_name, input).await
            }
            "auto_scaling_plans" => {
                self.auto_scaling_plans().create_resource(resource_name, input).await
            }
            "directory_service" => {
                self.directory_service().create_resource(resource_name, input).await
            }
            "mediapackage" => {
                self.mediapackage().create_resource(resource_name, input).await
            }
            "ssm_quicksetup" => {
                self.ssm_quicksetup().create_resource(resource_name, input).await
            }
            "s3_control" => {
                self.s3_control().create_resource(resource_name, input).await
            }
            "codecatalyst" => {
                self.codecatalyst().create_resource(resource_name, input).await
            }
            "notificationscontacts" => {
                self.notificationscontacts().create_resource(resource_name, input).await
            }
            "mpa" => {
                self.mpa().create_resource(resource_name, input).await
            }
            "ec2_instance_connect" => {
                self.ec2_instance_connect().create_resource(resource_name, input).await
            }
            "sagemaker_geospatial" => {
                self.sagemaker_geospatial().create_resource(resource_name, input).await
            }
            "notifications" => {
                self.notifications().create_resource(resource_name, input).await
            }
            "securitylake" => {
                self.securitylake().create_resource(resource_name, input).await
            }
            "networkmonitor" => {
                self.networkmonitor().create_resource(resource_name, input).await
            }
            "codeconnections" => {
                self.codeconnections().create_resource(resource_name, input).await
            }
            "app_mesh" => {
                self.app_mesh().create_resource(resource_name, input).await
            }
            "workspaces_thin_client" => {
                self.workspaces_thin_client().create_resource(resource_name, input).await
            }
            "finspace_data" => {
                self.finspace_data().create_resource(resource_name, input).await
            }
            "compute_optimizer" => {
                self.compute_optimizer().create_resource(resource_name, input).await
            }
            "secrets_manager" => {
                self.secrets_manager().create_resource(resource_name, input).await
            }
            "mediastore" => {
                self.mediastore().create_resource(resource_name, input).await
            }
            "ecs" => {
                self.ecs().create_resource(resource_name, input).await
            }
            "vpc_lattice" => {
                self.vpc_lattice().create_resource(resource_name, input).await
            }
            "auto_scaling" => {
                self.auto_scaling().create_resource(resource_name, input).await
            }
            "resource_groups" => {
                self.resource_groups().create_resource(resource_name, input).await
            }
            "eks" => {
                self.eks().create_resource(resource_name, input).await
            }
            "marketplace_entitlement_service" => {
                self.marketplace_entitlement_service().create_resource(resource_name, input).await
            }
            "database_migration_service" => {
                self.database_migration_service().create_resource(resource_name, input).await
            }
            "security_ir" => {
                self.security_ir().create_resource(resource_name, input).await
            }
            "inspector_scan" => {
                self.inspector_scan().create_resource(resource_name, input).await
            }
            "global_accelerator" => {
                self.global_accelerator().create_resource(resource_name, input).await
            }
            "kinesis_analytics" => {
                self.kinesis_analytics().create_resource(resource_name, input).await
            }
            "neptunedata" => {
                self.neptunedata().create_resource(resource_name, input).await
            }
            "swf" => {
                self.swf().create_resource(resource_name, input).await
            }
            "cloudwatch_logs" => {
                self.cloudwatch_logs().create_resource(resource_name, input).await
            }
            "connect" => {
                self.connect().create_resource(resource_name, input).await
            }
            "glue" => {
                self.glue().create_resource(resource_name, input).await
            }
            "cognito_identity_provider" => {
                self.cognito_identity_provider().create_resource(resource_name, input).await
            }
            "cloudwatch_events" => {
                self.cloudwatch_events().create_resource(resource_name, input).await
            }
            "cost_explorer" => {
                self.cost_explorer().create_resource(resource_name, input).await
            }
            "network_firewall" => {
                self.network_firewall().create_resource(resource_name, input).await
            }
            "firehose" => {
                self.firehose().create_resource(resource_name, input).await
            }
            "transfer" => {
                self.transfer().create_resource(resource_name, input).await
            }
            "marketplace_metering" => {
                self.marketplace_metering().create_resource(resource_name, input).await
            }
            "rbin" => {
                self.rbin().create_resource(resource_name, input).await
            }
            "timestream_influxdb" => {
                self.timestream_influxdb().create_resource(resource_name, input).await
            }
            "iotanalytics" => {
                self.iotanalytics().create_resource(resource_name, input).await
            }
            "ivs" => {
                self.ivs().create_resource(resource_name, input).await
            }
            "kafka" => {
                self.kafka().create_resource(resource_name, input).await
            }
            "sesv2" => {
                self.sesv2().create_resource(resource_name, input).await
            }
            "kendra" => {
                self.kendra().create_resource(resource_name, input).await
            }
            "sagemaker_edge" => {
                self.sagemaker_edge().create_resource(resource_name, input).await
            }
            "launch_wizard" => {
                self.launch_wizard().create_resource(resource_name, input).await
            }
            "securityhub" => {
                self.securityhub().create_resource(resource_name, input).await
            }
            "finspace" => {
                self.finspace().create_resource(resource_name, input).await
            }
            "keyspacesstreams" => {
                self.keyspacesstreams().create_resource(resource_name, input).await
            }
            "cleanroomsml" => {
                self.cleanroomsml().create_resource(resource_name, input).await
            }
            "transcribe_streaming" => {
                self.transcribe_streaming().create_resource(resource_name, input).await
            }
            "aiops" => {
                self.aiops().create_resource(resource_name, input).await
            }
            "service_catalog" => {
                self.service_catalog().create_resource(resource_name, input).await
            }
            "databrew" => {
                self.databrew().create_resource(resource_name, input).await
            }
            "codecommit" => {
                self.codecommit().create_resource(resource_name, input).await
            }
            "resource_explorer_2" => {
                self.resource_explorer_2().create_resource(resource_name, input).await
            }
            "acm_pca" => {
                self.acm_pca().create_resource(resource_name, input).await
            }
            "payment_cryptography" => {
                self.payment_cryptography().create_resource(resource_name, input).await
            }
            "mq" => {
                self.mq().create_resource(resource_name, input).await
            }
            "api_gateway" => {
                self.api_gateway().create_resource(resource_name, input).await
            }
            "grafana" => {
                self.grafana().create_resource(resource_name, input).await
            }
            "glacier" => {
                self.glacier().create_resource(resource_name, input).await
            }
            "bedrock" => {
                self.bedrock().create_resource(resource_name, input).await
            }
            "s3tables" => {
                self.s3tables().create_resource(resource_name, input).await
            }
            "ivs_realtime" => {
                self.ivs_realtime().create_resource(resource_name, input).await
            }
            "medialive" => {
                self.medialive().create_resource(resource_name, input).await
            }
            "backupsearch" => {
                self.backupsearch().create_resource(resource_name, input).await
            }
            "networkflowmonitor" => {
                self.networkflowmonitor().create_resource(resource_name, input).await
            }
            "elasticache" => {
                self.elasticache().create_resource(resource_name, input).await
            }
            "fis" => {
                self.fis().create_resource(resource_name, input).await
            }
            "cloudhsm" => {
                self.cloudhsm().create_resource(resource_name, input).await
            }
            "cost_optimization_hub" => {
                self.cost_optimization_hub().create_resource(resource_name, input).await
            }
            "synthetics" => {
                self.synthetics().create_resource(resource_name, input).await
            }
            "rum" => {
                self.rum().create_resource(resource_name, input).await
            }
            "emr_containers" => {
                self.emr_containers().create_resource(resource_name, input).await
            }
            "sagemaker_a2i_runtime" => {
                self.sagemaker_a2i_runtime().create_resource(resource_name, input).await
            }
            "ssm_contacts" => {
                self.ssm_contacts().create_resource(resource_name, input).await
            }
            "bcm_data_exports" => {
                self.bcm_data_exports().create_resource(resource_name, input).await
            }
            "opensearch" => {
                self.opensearch().create_resource(resource_name, input).await
            }
            "dax" => {
                self.dax().create_resource(resource_name, input).await
            }
            "neptune" => {
                self.neptune().create_resource(resource_name, input).await
            }
            "pricing" => {
                self.pricing().create_resource(resource_name, input).await
            }
            "location" => {
                self.location().create_resource(resource_name, input).await
            }
            "route53profiles" => {
                self.route53profiles().create_resource(resource_name, input).await
            }
            "lambda" => {
                self.lambda().create_resource(resource_name, input).await
            }
            "ivschat" => {
                self.ivschat().create_resource(resource_name, input).await
            }
            "billing" => {
                self.billing().create_resource(resource_name, input).await
            }
            "wisdom" => {
                self.wisdom().create_resource(resource_name, input).await
            }
            "schemas" => {
                self.schemas().create_resource(resource_name, input).await
            }
            "bedrock_agentcore_control" => {
                self.bedrock_agentcore_control().create_resource(resource_name, input).await
            }
            "controlcatalog" => {
                self.controlcatalog().create_resource(resource_name, input).await
            }
            "cloudsearch" => {
                self.cloudsearch().create_resource(resource_name, input).await
            }
            "deadline" => {
                self.deadline().create_resource(resource_name, input).await
            }
            "managedblockchain" => {
                self.managedblockchain().create_resource(resource_name, input).await
            }
            "amplify" => {
                self.amplify().create_resource(resource_name, input).await
            }
            "iotsecuretunneling" => {
                self.iotsecuretunneling().create_resource(resource_name, input).await
            }
            "connectcampaigns" => {
                self.connectcampaigns().create_resource(resource_name, input).await
            }
            "kafkaconnect" => {
                self.kafkaconnect().create_resource(resource_name, input).await
            }
            "mediaconvert" => {
                self.mediaconvert().create_resource(resource_name, input).await
            }
            "data_pipeline" => {
                self.data_pipeline().create_resource(resource_name, input).await
            }
            "codepipeline" => {
                self.codepipeline().create_resource(resource_name, input).await
            }
            "clouddirectory" => {
                self.clouddirectory().create_resource(resource_name, input).await
            }
            "amplifyuibuilder" => {
                self.amplifyuibuilder().create_resource(resource_name, input).await
            }
            "rtbfabric" => {
                self.rtbfabric().create_resource(resource_name, input).await
            }
            "memorydb" => {
                self.memorydb().create_resource(resource_name, input).await
            }
            "iot" => {
                self.iot().create_resource(resource_name, input).await
            }
            "marketplace_commerce_analytics" => {
                self.marketplace_commerce_analytics().create_resource(resource_name, input).await
            }
            "frauddetector" => {
                self.frauddetector().create_resource(resource_name, input).await
            }
            "bedrock_data_automation" => {
                self.bedrock_data_automation().create_resource(resource_name, input).await
            }
            "elastic_load_balancing" => {
                self.elastic_load_balancing().create_resource(resource_name, input).await
            }
            "verifiedpermissions" => {
                self.verifiedpermissions().create_resource(resource_name, input).await
            }
            "networkmanager" => {
                self.networkmanager().create_resource(resource_name, input).await
            }
            "devops_guru" => {
                self.devops_guru().create_resource(resource_name, input).await
            }
            "taxsettings" => {
                self.taxsettings().create_resource(resource_name, input).await
            }
            "workspaces_instances" => {
                self.workspaces_instances().create_resource(resource_name, input).await
            }
            "arc_zonal_shift" => {
                self.arc_zonal_shift().create_resource(resource_name, input).await
            }
            "elastic_transcoder" => {
                self.elastic_transcoder().create_resource(resource_name, input).await
            }
            "fms" => {
                self.fms().create_resource(resource_name, input).await
            }
            "imagebuilder" => {
                self.imagebuilder().create_resource(resource_name, input).await
            }
            "chime_sdk" => {
                self.chime_sdk().create_resource(resource_name, input).await
            }
            "groundstation" => {
                self.groundstation().create_resource(resource_name, input).await
            }
            "forecast" => {
                self.forecast().create_resource(resource_name, input).await
            }
            "appstream" => {
                self.appstream().create_resource(resource_name, input).await
            }
            "chime_sdk_meetings" => {
                self.chime_sdk_meetings().create_resource(resource_name, input).await
            }
            "comprehend" => {
                self.comprehend().create_resource(resource_name, input).await
            }
            "redshift_serverless" => {
                self.redshift_serverless().create_resource(resource_name, input).await
            }
            "pinpoint" => {
                self.pinpoint().create_resource(resource_name, input).await
            }
            "pi" => {
                self.pi().create_resource(resource_name, input).await
            }
            "gameliftstreams" => {
                self.gameliftstreams().create_resource(resource_name, input).await
            }
            "customer_profiles" => {
                self.customer_profiles().create_resource(resource_name, input).await
            }
            "workspaces" => {
                self.workspaces().create_resource(resource_name, input).await
            }
            "auditmanager" => {
                self.auditmanager().create_resource(resource_name, input).await
            }
            "docdb" => {
                self.docdb().create_resource(resource_name, input).await
            }
            "mturk" => {
                self.mturk().create_resource(resource_name, input).await
            }
            "cognito_identity" => {
                self.cognito_identity().create_resource(resource_name, input).await
            }
            "dynamodb" => {
                self.dynamodb().create_resource(resource_name, input).await
            }
            "codeartifact" => {
                self.codeartifact().create_resource(resource_name, input).await
            }
            "organizations" => {
                self.organizations().create_resource(resource_name, input).await
            }
            "dlm" => {
                self.dlm().create_resource(resource_name, input).await
            }
            "sso" => {
                self.sso().create_resource(resource_name, input).await
            }
            "osis" => {
                self.osis().create_resource(resource_name, input).await
            }
            "migration_hub" => {
                self.migration_hub().create_resource(resource_name, input).await
            }
            "chatbot" => {
                self.chatbot().create_resource(resource_name, input).await
            }
            "docdb_elastic" => {
                self.docdb_elastic().create_resource(resource_name, input).await
            }
            "supplychain" => {
                self.supplychain().create_resource(resource_name, input).await
            }
            "ses" => {
                self.ses().create_resource(resource_name, input).await
            }
            "repostspace" => {
                self.repostspace().create_resource(resource_name, input).await
            }
            "mediastore_data" => {
                self.mediastore_data().create_resource(resource_name, input).await
            }
            "bedrock_agent" => {
                self.bedrock_agent().create_resource(resource_name, input).await
            }
            "wellarchitected" => {
                self.wellarchitected().create_resource(resource_name, input).await
            }
            "budgets" => {
                self.budgets().create_resource(resource_name, input).await
            }
            "mediatailor" => {
                self.mediatailor().create_resource(resource_name, input).await
            }
            "appsync" => {
                self.appsync().create_resource(resource_name, input).await
            }
            "ssm_guiconnect" => {
                self.ssm_guiconnect().create_resource(resource_name, input).await
            }
            "evs" => {
                self.evs().create_resource(resource_name, input).await
            }
            "eks_auth" => {
                self.eks_auth().create_resource(resource_name, input).await
            }
            "chime_sdk_messaging" => {
                self.chime_sdk_messaging().create_resource(resource_name, input).await
            }
            "mediaconnect" => {
                self.mediaconnect().create_resource(resource_name, input).await
            }
            "identitystore" => {
                self.identitystore().create_resource(resource_name, input).await
            }
            "bcm_pricing_calculator" => {
                self.bcm_pricing_calculator().create_resource(resource_name, input).await
            }
            "lakeformation" => {
                self.lakeformation().create_resource(resource_name, input).await
            }
            "xray" => {
                self.xray().create_resource(resource_name, input).await
            }
            "cloudfront" => {
                self.cloudfront().create_resource(resource_name, input).await
            }
            "sagemaker_metrics" => {
                self.sagemaker_metrics().create_resource(resource_name, input).await
            }
            "sso_oidc" => {
                self.sso_oidc().create_resource(resource_name, input).await
            }
            "sagemaker" => {
                self.sagemaker().create_resource(resource_name, input).await
            }
            "codestar_connections" => {
                self.codestar_connections().create_resource(resource_name, input).await
            }
            "device_farm" => {
                self.device_farm().create_resource(resource_name, input).await
            }
            "translate" => {
                self.translate().create_resource(resource_name, input).await
            }
            "sagemaker_runtime" => {
                self.sagemaker_runtime().create_resource(resource_name, input).await
            }
            "b2bi" => {
                self.b2bi().create_resource(resource_name, input).await
            }
            "savingsplans" => {
                self.savingsplans().create_resource(resource_name, input).await
            }
            "pipes" => {
                self.pipes().create_resource(resource_name, input).await
            }
            "config_service" => {
                self.config_service().create_resource(resource_name, input).await
            }
            "codeguruprofiler" => {
                self.codeguruprofiler().create_resource(resource_name, input).await
            }
            "s3" => {
                self.s3().create_resource(resource_name, input).await
            }
            "polly" => {
                self.polly().create_resource(resource_name, input).await
            }
            "cognito_sync" => {
                self.cognito_sync().create_resource(resource_name, input).await
            }
            "scheduler" => {
                self.scheduler().create_resource(resource_name, input).await
            }
            "pca_connector_ad" => {
                self.pca_connector_ad().create_resource(resource_name, input).await
            }
            "waf_regional" => {
                self.waf_regional().create_resource(resource_name, input).await
            }
            "apigatewaymanagementapi" => {
                self.apigatewaymanagementapi().create_resource(resource_name, input).await
            }
            "workspaces_web" => {
                self.workspaces_web().create_resource(resource_name, input).await
            }
            "pca_connector_scep" => {
                self.pca_connector_scep().create_resource(resource_name, input).await
            }
            "codestar_notifications" => {
                self.codestar_notifications().create_resource(resource_name, input).await
            }
            "direct_connect" => {
                self.direct_connect().create_resource(resource_name, input).await
            }
            "shield" => {
                self.shield().create_resource(resource_name, input).await
            }
            "application_signals" => {
                self.application_signals().create_resource(resource_name, input).await
            }
            "iot_managed_integrations" => {
                self.iot_managed_integrations().create_resource(resource_name, input).await
            }
            "iot_wireless" => {
                self.iot_wireless().create_resource(resource_name, input).await
            }
            "iot_events" => {
                self.iot_events().create_resource(resource_name, input).await
            }
            "backup_gateway" => {
                self.backup_gateway().create_resource(resource_name, input).await
            }
            "sso_admin" => {
                self.sso_admin().create_resource(resource_name, input).await
            }
            "elastic_beanstalk" => {
                self.elastic_beanstalk().create_resource(resource_name, input).await
            }
            "drs" => {
                self.drs().create_resource(resource_name, input).await
            }
            "personalize_runtime" => {
                self.personalize_runtime().create_resource(resource_name, input).await
            }
            "outposts" => {
                self.outposts().create_resource(resource_name, input).await
            }
            "license_manager_user_subscriptions" => {
                self.license_manager_user_subscriptions().create_resource(resource_name, input).await
            }
            "cloudtrail_data" => {
                self.cloudtrail_data().create_resource(resource_name, input).await
            }
            "lex_runtime_service" => {
                self.lex_runtime_service().create_resource(resource_name, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Read/refresh resource state
    async fn read(&self, resource_type: &str, id: &str) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "emr_serverless" => {
                self.emr_serverless().read_resource(resource_name, id).await
            }
            "cloudformation" => {
                self.cloudformation().read_resource(resource_name, id).await
            }
            "application_auto_scaling" => {
                self.application_auto_scaling().read_resource(resource_name, id).await
            }
            "personalize_events" => {
                self.personalize_events().read_resource(resource_name, id).await
            }
            "tnb" => {
                self.tnb().read_resource(resource_name, id).await
            }
            "rolesanywhere" => {
                self.rolesanywhere().read_resource(resource_name, id).await
            }
            "kms" => {
                self.kms().read_resource(resource_name, id).await
            }
            "datasync" => {
                self.datasync().read_resource(resource_name, id).await
            }
            "bedrock_agent_runtime" => {
                self.bedrock_agent_runtime().read_resource(resource_name, id).await
            }
            "pinpoint_email" => {
                self.pinpoint_email().read_resource(resource_name, id).await
            }
            "connect_contact_lens" => {
                self.connect_contact_lens().read_resource(resource_name, id).await
            }
            "athena" => {
                self.athena().read_resource(resource_name, id).await
            }
            "iotfleetwise" => {
                self.iotfleetwise().read_resource(resource_name, id).await
            }
            "iot_data_plane" => {
                self.iot_data_plane().read_resource(resource_name, id).await
            }
            "bedrock_data_automation_runtime" => {
                self.bedrock_data_automation_runtime().read_resource(resource_name, id).await
            }
            "entityresolution" => {
                self.entityresolution().read_resource(resource_name, id).await
            }
            "forecastquery" => {
                self.forecastquery().read_resource(resource_name, id).await
            }
            "detective" => {
                self.detective().read_resource(resource_name, id).await
            }
            "panorama" => {
                self.panorama().read_resource(resource_name, id).await
            }
            "backup" => {
                self.backup().read_resource(resource_name, id).await
            }
            "mwaa" => {
                self.mwaa().read_resource(resource_name, id).await
            }
            "iot_jobs_data_plane" => {
                self.iot_jobs_data_plane().read_resource(resource_name, id).await
            }
            "transcribe" => {
                self.transcribe().read_resource(resource_name, id).await
            }
            "cloudwatch" => {
                self.cloudwatch().read_resource(resource_name, id).await
            }
            "snow_device_management" => {
                self.snow_device_management().read_resource(resource_name, id).await
            }
            "workmailmessageflow" => {
                self.workmailmessageflow().read_resource(resource_name, id).await
            }
            "appconfig" => {
                self.appconfig().read_resource(resource_name, id).await
            }
            "lightsail" => {
                self.lightsail().read_resource(resource_name, id).await
            }
            "guardduty" => {
                self.guardduty().read_resource(resource_name, id).await
            }
            "apigatewayv2" => {
                self.apigatewayv2().read_resource(resource_name, id).await
            }
            "wafv2" => {
                self.wafv2().read_resource(resource_name, id).await
            }
            "iotsitewise" => {
                self.iotsitewise().read_resource(resource_name, id).await
            }
            "iotthingsgraph" => {
                self.iotthingsgraph().read_resource(resource_name, id).await
            }
            "batch" => {
                self.batch().read_resource(resource_name, id).await
            }
            "mailmanager" => {
                self.mailmanager().read_resource(resource_name, id).await
            }
            "marketplace_reporting" => {
                self.marketplace_reporting().read_resource(resource_name, id).await
            }
            "m2" => {
                self.m2().read_resource(resource_name, id).await
            }
            "codedeploy" => {
                self.codedeploy().read_resource(resource_name, id).await
            }
            "route53_recovery_control_config" => {
                self.route53_recovery_control_config().read_resource(resource_name, id).await
            }
            "simspaceweaver" => {
                self.simspaceweaver().read_resource(resource_name, id).await
            }
            "resiliencehub" => {
                self.resiliencehub().read_resource(resource_name, id).await
            }
            "oam" => {
                self.oam().read_resource(resource_name, id).await
            }
            "license_manager_linux_subscriptions" => {
                self.license_manager_linux_subscriptions().read_resource(resource_name, id).await
            }
            "voice_id" => {
                self.voice_id().read_resource(resource_name, id).await
            }
            "chime" => {
                self.chime().read_resource(resource_name, id).await
            }
            "efs" => {
                self.efs().read_resource(resource_name, id).await
            }
            "freetier" => {
                self.freetier().read_resource(resource_name, id).await
            }
            "storage_gateway" => {
                self.storage_gateway().read_resource(resource_name, id).await
            }
            "dynamodb_streams" => {
                self.dynamodb_streams().read_resource(resource_name, id).await
            }
            "gamelift" => {
                self.gamelift().read_resource(resource_name, id).await
            }
            "inspector2" => {
                self.inspector2().read_resource(resource_name, id).await
            }
            "keyspaces" => {
                self.keyspaces().read_resource(resource_name, id).await
            }
            "sqs" => {
                self.sqs().read_resource(resource_name, id).await
            }
            "ram" => {
                self.ram().read_resource(resource_name, id).await
            }
            "ssm_sap" => {
                self.ssm_sap().read_resource(resource_name, id).await
            }
            "directory_service_data" => {
                self.directory_service_data().read_resource(resource_name, id).await
            }
            "route_53_domains" => {
                self.route_53_domains().read_resource(resource_name, id).await
            }
            "bedrock_agentcore" => {
                self.bedrock_agentcore().read_resource(resource_name, id).await
            }
            "trustedadvisor" => {
                self.trustedadvisor().read_resource(resource_name, id).await
            }
            "migrationhubstrategy" => {
                self.migrationhubstrategy().read_resource(resource_name, id).await
            }
            "dataexchange" => {
                self.dataexchange().read_resource(resource_name, id).await
            }
            "braket" => {
                self.braket().read_resource(resource_name, id).await
            }
            "codebuild" => {
                self.codebuild().read_resource(resource_name, id).await
            }
            "acm" => {
                self.acm().read_resource(resource_name, id).await
            }
            "route53_recovery_cluster" => {
                self.route53_recovery_cluster().read_resource(resource_name, id).await
            }
            "lookoutequipment" => {
                self.lookoutequipment().read_resource(resource_name, id).await
            }
            "marketplace_catalog" => {
                self.marketplace_catalog().read_resource(resource_name, id).await
            }
            "payment_cryptography_data" => {
                self.payment_cryptography_data().read_resource(resource_name, id).await
            }
            "cloud9" => {
                self.cloud9().read_resource(resource_name, id).await
            }
            "workdocs" => {
                self.workdocs().read_resource(resource_name, id).await
            }
            "license_manager" => {
                self.license_manager().read_resource(resource_name, id).await
            }
            "sts" => {
                self.sts().read_resource(resource_name, id).await
            }
            "s3vectors" => {
                self.s3vectors().read_resource(resource_name, id).await
            }
            "chime_sdk_media_pipelines" => {
                self.chime_sdk_media_pipelines().read_resource(resource_name, id).await
            }
            "machine_learning" => {
                self.machine_learning().read_resource(resource_name, id).await
            }
            "timestream_query" => {
                self.timestream_query().read_resource(resource_name, id).await
            }
            "codeguru_reviewer" => {
                self.codeguru_reviewer().read_resource(resource_name, id).await
            }
            "mgn" => {
                self.mgn().read_resource(resource_name, id).await
            }
            "evidently" => {
                self.evidently().read_resource(resource_name, id).await
            }
            "qbusiness" => {
                self.qbusiness().read_resource(resource_name, id).await
            }
            "connectcases" => {
                self.connectcases().read_resource(resource_name, id).await
            }
            "fsx" => {
                self.fsx().read_resource(resource_name, id).await
            }
            "ecr" => {
                self.ecr().read_resource(resource_name, id).await
            }
            "connectcampaignsv2" => {
                self.connectcampaignsv2().read_resource(resource_name, id).await
            }
            "rds" => {
                self.rds().read_resource(resource_name, id).await
            }
            "qapps" => {
                self.qapps().read_resource(resource_name, id).await
            }
            "qconnect" => {
                self.qconnect().read_resource(resource_name, id).await
            }
            "omics" => {
                self.omics().read_resource(resource_name, id).await
            }
            "bcm_dashboards" => {
                self.bcm_dashboards().read_resource(resource_name, id).await
            }
            "geo_routes" => {
                self.geo_routes().read_resource(resource_name, id).await
            }
            "quicksight" => {
                self.quicksight().read_resource(resource_name, id).await
            }
            "amp" => {
                self.amp().read_resource(resource_name, id).await
            }
            "opensearchserverless" => {
                self.opensearchserverless().read_resource(resource_name, id).await
            }
            "emr" => {
                self.emr().read_resource(resource_name, id).await
            }
            "service_quotas" => {
                self.service_quotas().read_resource(resource_name, id).await
            }
            "service_catalog_appregistry" => {
                self.service_catalog_appregistry().read_resource(resource_name, id).await
            }
            "migrationhub_config" => {
                self.migrationhub_config().read_resource(resource_name, id).await
            }
            "iam" => {
                self.iam().read_resource(resource_name, id).await
            }
            "accessanalyzer" => {
                self.accessanalyzer().read_resource(resource_name, id).await
            }
            "appconfigdata" => {
                self.appconfigdata().read_resource(resource_name, id).await
            }
            "route53resolver" => {
                self.route53resolver().read_resource(resource_name, id).await
            }
            "s3outposts" => {
                self.s3outposts().read_resource(resource_name, id).await
            }
            "kendra_ranking" => {
                self.kendra_ranking().read_resource(resource_name, id).await
            }
            "controltower" => {
                self.controltower().read_resource(resource_name, id).await
            }
            "arc_region_switch" => {
                self.arc_region_switch().read_resource(resource_name, id).await
            }
            "neptune_graph" => {
                self.neptune_graph().read_resource(resource_name, id).await
            }
            "route53_recovery_readiness" => {
                self.route53_recovery_readiness().read_resource(resource_name, id).await
            }
            "greengrassv2" => {
                self.greengrassv2().read_resource(resource_name, id).await
            }
            "migration_hub_refactor_spaces" => {
                self.migration_hub_refactor_spaces().read_resource(resource_name, id).await
            }
            "cost_and_usage_report_service" => {
                self.cost_and_usage_report_service().read_resource(resource_name, id).await
            }
            "ebs" => {
                self.ebs().read_resource(resource_name, id).await
            }
            "appflow" => {
                self.appflow().read_resource(resource_name, id).await
            }
            "migrationhuborchestrator" => {
                self.migrationhuborchestrator().read_resource(resource_name, id).await
            }
            "chime_sdk_identity" => {
                self.chime_sdk_identity().read_resource(resource_name, id).await
            }
            "cloudfront_keyvaluestore" => {
                self.cloudfront_keyvaluestore().read_resource(resource_name, id).await
            }
            "waf" => {
                self.waf().read_resource(resource_name, id).await
            }
            "greengrass" => {
                self.greengrass().read_resource(resource_name, id).await
            }
            "sagemaker_featurestore_runtime" => {
                self.sagemaker_featurestore_runtime().read_resource(resource_name, id).await
            }
            "inspector" => {
                self.inspector().read_resource(resource_name, id).await
            }
            "appfabric" => {
                self.appfabric().read_resource(resource_name, id).await
            }
            "lex_model_building_service" => {
                self.lex_model_building_service().read_resource(resource_name, id).await
            }
            "serverlessapplicationrepository" => {
                self.serverlessapplicationrepository().read_resource(resource_name, id).await
            }
            "cloudsearch_domain" => {
                self.cloudsearch_domain().read_resource(resource_name, id).await
            }
            "codeguru_security" => {
                self.codeguru_security().read_resource(resource_name, id).await
            }
            "socialmessaging" => {
                self.socialmessaging().read_resource(resource_name, id).await
            }
            "geo_maps" => {
                self.geo_maps().read_resource(resource_name, id).await
            }
            "kinesis" => {
                self.kinesis().read_resource(resource_name, id).await
            }
            "dsql" => {
                self.dsql().read_resource(resource_name, id).await
            }
            "appintegrations" => {
                self.appintegrations().read_resource(resource_name, id).await
            }
            "personalize" => {
                self.personalize().read_resource(resource_name, id).await
            }
            "proton" => {
                self.proton().read_resource(resource_name, id).await
            }
            "cloudcontrol" => {
                self.cloudcontrol().read_resource(resource_name, id).await
            }
            "redshift" => {
                self.redshift().read_resource(resource_name, id).await
            }
            "geo_places" => {
                self.geo_places().read_resource(resource_name, id).await
            }
            "elasticsearch_service" => {
                self.elasticsearch_service().read_resource(resource_name, id).await
            }
            "bcm_recommended_actions" => {
                self.bcm_recommended_actions().read_resource(resource_name, id).await
            }
            "invoicing" => {
                self.invoicing().read_resource(resource_name, id).await
            }
            "apprunner" => {
                self.apprunner().read_resource(resource_name, id).await
            }
            "sns" => {
                self.sns().read_resource(resource_name, id).await
            }
            "textract" => {
                self.textract().read_resource(resource_name, id).await
            }
            "workmail" => {
                self.workmail().read_resource(resource_name, id).await
            }
            "datazone" => {
                self.datazone().read_resource(resource_name, id).await
            }
            "rekognition" => {
                self.rekognition().read_resource(resource_name, id).await
            }
            "ssm" => {
                self.ssm().read_resource(resource_name, id).await
            }
            "medical_imaging" => {
                self.medical_imaging().read_resource(resource_name, id).await
            }
            "lex_models" => {
                self.lex_models().read_resource(resource_name, id).await
            }
            "support" => {
                self.support().read_resource(resource_name, id).await
            }
            "signer" => {
                self.signer().read_resource(resource_name, id).await
            }
            "partnercentral_selling" => {
                self.partnercentral_selling().read_resource(resource_name, id).await
            }
            "comprehendmedical" => {
                self.comprehendmedical().read_resource(resource_name, id).await
            }
            "macie2" => {
                self.macie2().read_resource(resource_name, id).await
            }
            "redshift_data" => {
                self.redshift_data().read_resource(resource_name, id).await
            }
            "marketplace_agreement" => {
                self.marketplace_agreement().read_resource(resource_name, id).await
            }
            "health" => {
                self.health().read_resource(resource_name, id).await
            }
            "odb" => {
                self.odb().read_resource(resource_name, id).await
            }
            "resource_groups_tagging_api" => {
                self.resource_groups_tagging_api().read_resource(resource_name, id).await
            }
            "application_insights" => {
                self.application_insights().read_resource(resource_name, id).await
            }
            "timestream_write" => {
                self.timestream_write().read_resource(resource_name, id).await
            }
            "pinpoint_sms" => {
                self.pinpoint_sms().read_resource(resource_name, id).await
            }
            "mediapackagev2" => {
                self.mediapackagev2().read_resource(resource_name, id).await
            }
            "ec2" => {
                self.ec2().read_resource(resource_name, id).await
            }
            "cleanrooms" => {
                self.cleanrooms().read_resource(resource_name, id).await
            }
            "healthlake" => {
                self.healthlake().read_resource(resource_name, id).await
            }
            "sfn" => {
                self.sfn().read_resource(resource_name, id).await
            }
            "iottwinmaker" => {
                self.iottwinmaker().read_resource(resource_name, id).await
            }
            "cloudtrail" => {
                self.cloudtrail().read_resource(resource_name, id).await
            }
            "iotdeviceadvisor" => {
                self.iotdeviceadvisor().read_resource(resource_name, id).await
            }
            "ssm_incidents" => {
                self.ssm_incidents().read_resource(resource_name, id).await
            }
            "pcs" => {
                self.pcs().read_resource(resource_name, id).await
            }
            "support_app" => {
                self.support_app().read_resource(resource_name, id).await
            }
            "managedblockchain_query" => {
                self.managedblockchain_query().read_resource(resource_name, id).await
            }
            "iot_events_data" => {
                self.iot_events_data().read_resource(resource_name, id).await
            }
            "lex_runtime" => {
                self.lex_runtime().read_resource(resource_name, id).await
            }
            "observabilityadmin" => {
                self.observabilityadmin().read_resource(resource_name, id).await
            }
            "applicationcostprofiler" => {
                self.applicationcostprofiler().read_resource(resource_name, id).await
            }
            "billingconductor" => {
                self.billingconductor().read_resource(resource_name, id).await
            }
            "artifact" => {
                self.artifact().read_resource(resource_name, id).await
            }
            "ecr_public" => {
                self.ecr_public().read_resource(resource_name, id).await
            }
            "connectparticipant" => {
                self.connectparticipant().read_resource(resource_name, id).await
            }
            "rds_data" => {
                self.rds_data().read_resource(resource_name, id).await
            }
            "internetmonitor" => {
                self.internetmonitor().read_resource(resource_name, id).await
            }
            "route_53" => {
                self.route_53().read_resource(resource_name, id).await
            }
            "bedrock_runtime" => {
                self.bedrock_runtime().read_resource(resource_name, id).await
            }
            "amplifybackend" => {
                self.amplifybackend().read_resource(resource_name, id).await
            }
            "marketplace_deployment" => {
                self.marketplace_deployment().read_resource(resource_name, id).await
            }
            "account" => {
                self.account().read_resource(resource_name, id).await
            }
            "snowball" => {
                self.snowball().read_resource(resource_name, id).await
            }
            "eventbridge" => {
                self.eventbridge().read_resource(resource_name, id).await
            }
            "auto_scaling_plans" => {
                self.auto_scaling_plans().read_resource(resource_name, id).await
            }
            "directory_service" => {
                self.directory_service().read_resource(resource_name, id).await
            }
            "mediapackage" => {
                self.mediapackage().read_resource(resource_name, id).await
            }
            "ssm_quicksetup" => {
                self.ssm_quicksetup().read_resource(resource_name, id).await
            }
            "s3_control" => {
                self.s3_control().read_resource(resource_name, id).await
            }
            "codecatalyst" => {
                self.codecatalyst().read_resource(resource_name, id).await
            }
            "notificationscontacts" => {
                self.notificationscontacts().read_resource(resource_name, id).await
            }
            "mpa" => {
                self.mpa().read_resource(resource_name, id).await
            }
            "ec2_instance_connect" => {
                self.ec2_instance_connect().read_resource(resource_name, id).await
            }
            "sagemaker_geospatial" => {
                self.sagemaker_geospatial().read_resource(resource_name, id).await
            }
            "notifications" => {
                self.notifications().read_resource(resource_name, id).await
            }
            "securitylake" => {
                self.securitylake().read_resource(resource_name, id).await
            }
            "networkmonitor" => {
                self.networkmonitor().read_resource(resource_name, id).await
            }
            "codeconnections" => {
                self.codeconnections().read_resource(resource_name, id).await
            }
            "app_mesh" => {
                self.app_mesh().read_resource(resource_name, id).await
            }
            "workspaces_thin_client" => {
                self.workspaces_thin_client().read_resource(resource_name, id).await
            }
            "finspace_data" => {
                self.finspace_data().read_resource(resource_name, id).await
            }
            "compute_optimizer" => {
                self.compute_optimizer().read_resource(resource_name, id).await
            }
            "secrets_manager" => {
                self.secrets_manager().read_resource(resource_name, id).await
            }
            "mediastore" => {
                self.mediastore().read_resource(resource_name, id).await
            }
            "ecs" => {
                self.ecs().read_resource(resource_name, id).await
            }
            "vpc_lattice" => {
                self.vpc_lattice().read_resource(resource_name, id).await
            }
            "auto_scaling" => {
                self.auto_scaling().read_resource(resource_name, id).await
            }
            "resource_groups" => {
                self.resource_groups().read_resource(resource_name, id).await
            }
            "eks" => {
                self.eks().read_resource(resource_name, id).await
            }
            "marketplace_entitlement_service" => {
                self.marketplace_entitlement_service().read_resource(resource_name, id).await
            }
            "database_migration_service" => {
                self.database_migration_service().read_resource(resource_name, id).await
            }
            "security_ir" => {
                self.security_ir().read_resource(resource_name, id).await
            }
            "inspector_scan" => {
                self.inspector_scan().read_resource(resource_name, id).await
            }
            "global_accelerator" => {
                self.global_accelerator().read_resource(resource_name, id).await
            }
            "kinesis_analytics" => {
                self.kinesis_analytics().read_resource(resource_name, id).await
            }
            "neptunedata" => {
                self.neptunedata().read_resource(resource_name, id).await
            }
            "swf" => {
                self.swf().read_resource(resource_name, id).await
            }
            "cloudwatch_logs" => {
                self.cloudwatch_logs().read_resource(resource_name, id).await
            }
            "connect" => {
                self.connect().read_resource(resource_name, id).await
            }
            "glue" => {
                self.glue().read_resource(resource_name, id).await
            }
            "cognito_identity_provider" => {
                self.cognito_identity_provider().read_resource(resource_name, id).await
            }
            "cloudwatch_events" => {
                self.cloudwatch_events().read_resource(resource_name, id).await
            }
            "cost_explorer" => {
                self.cost_explorer().read_resource(resource_name, id).await
            }
            "network_firewall" => {
                self.network_firewall().read_resource(resource_name, id).await
            }
            "firehose" => {
                self.firehose().read_resource(resource_name, id).await
            }
            "transfer" => {
                self.transfer().read_resource(resource_name, id).await
            }
            "marketplace_metering" => {
                self.marketplace_metering().read_resource(resource_name, id).await
            }
            "rbin" => {
                self.rbin().read_resource(resource_name, id).await
            }
            "timestream_influxdb" => {
                self.timestream_influxdb().read_resource(resource_name, id).await
            }
            "iotanalytics" => {
                self.iotanalytics().read_resource(resource_name, id).await
            }
            "ivs" => {
                self.ivs().read_resource(resource_name, id).await
            }
            "kafka" => {
                self.kafka().read_resource(resource_name, id).await
            }
            "sesv2" => {
                self.sesv2().read_resource(resource_name, id).await
            }
            "kendra" => {
                self.kendra().read_resource(resource_name, id).await
            }
            "sagemaker_edge" => {
                self.sagemaker_edge().read_resource(resource_name, id).await
            }
            "launch_wizard" => {
                self.launch_wizard().read_resource(resource_name, id).await
            }
            "securityhub" => {
                self.securityhub().read_resource(resource_name, id).await
            }
            "finspace" => {
                self.finspace().read_resource(resource_name, id).await
            }
            "keyspacesstreams" => {
                self.keyspacesstreams().read_resource(resource_name, id).await
            }
            "cleanroomsml" => {
                self.cleanroomsml().read_resource(resource_name, id).await
            }
            "transcribe_streaming" => {
                self.transcribe_streaming().read_resource(resource_name, id).await
            }
            "aiops" => {
                self.aiops().read_resource(resource_name, id).await
            }
            "service_catalog" => {
                self.service_catalog().read_resource(resource_name, id).await
            }
            "databrew" => {
                self.databrew().read_resource(resource_name, id).await
            }
            "codecommit" => {
                self.codecommit().read_resource(resource_name, id).await
            }
            "resource_explorer_2" => {
                self.resource_explorer_2().read_resource(resource_name, id).await
            }
            "acm_pca" => {
                self.acm_pca().read_resource(resource_name, id).await
            }
            "payment_cryptography" => {
                self.payment_cryptography().read_resource(resource_name, id).await
            }
            "mq" => {
                self.mq().read_resource(resource_name, id).await
            }
            "api_gateway" => {
                self.api_gateway().read_resource(resource_name, id).await
            }
            "grafana" => {
                self.grafana().read_resource(resource_name, id).await
            }
            "glacier" => {
                self.glacier().read_resource(resource_name, id).await
            }
            "bedrock" => {
                self.bedrock().read_resource(resource_name, id).await
            }
            "s3tables" => {
                self.s3tables().read_resource(resource_name, id).await
            }
            "ivs_realtime" => {
                self.ivs_realtime().read_resource(resource_name, id).await
            }
            "medialive" => {
                self.medialive().read_resource(resource_name, id).await
            }
            "backupsearch" => {
                self.backupsearch().read_resource(resource_name, id).await
            }
            "networkflowmonitor" => {
                self.networkflowmonitor().read_resource(resource_name, id).await
            }
            "elasticache" => {
                self.elasticache().read_resource(resource_name, id).await
            }
            "fis" => {
                self.fis().read_resource(resource_name, id).await
            }
            "cloudhsm" => {
                self.cloudhsm().read_resource(resource_name, id).await
            }
            "cost_optimization_hub" => {
                self.cost_optimization_hub().read_resource(resource_name, id).await
            }
            "synthetics" => {
                self.synthetics().read_resource(resource_name, id).await
            }
            "rum" => {
                self.rum().read_resource(resource_name, id).await
            }
            "emr_containers" => {
                self.emr_containers().read_resource(resource_name, id).await
            }
            "sagemaker_a2i_runtime" => {
                self.sagemaker_a2i_runtime().read_resource(resource_name, id).await
            }
            "ssm_contacts" => {
                self.ssm_contacts().read_resource(resource_name, id).await
            }
            "bcm_data_exports" => {
                self.bcm_data_exports().read_resource(resource_name, id).await
            }
            "opensearch" => {
                self.opensearch().read_resource(resource_name, id).await
            }
            "dax" => {
                self.dax().read_resource(resource_name, id).await
            }
            "neptune" => {
                self.neptune().read_resource(resource_name, id).await
            }
            "pricing" => {
                self.pricing().read_resource(resource_name, id).await
            }
            "location" => {
                self.location().read_resource(resource_name, id).await
            }
            "route53profiles" => {
                self.route53profiles().read_resource(resource_name, id).await
            }
            "lambda" => {
                self.lambda().read_resource(resource_name, id).await
            }
            "ivschat" => {
                self.ivschat().read_resource(resource_name, id).await
            }
            "billing" => {
                self.billing().read_resource(resource_name, id).await
            }
            "wisdom" => {
                self.wisdom().read_resource(resource_name, id).await
            }
            "schemas" => {
                self.schemas().read_resource(resource_name, id).await
            }
            "bedrock_agentcore_control" => {
                self.bedrock_agentcore_control().read_resource(resource_name, id).await
            }
            "controlcatalog" => {
                self.controlcatalog().read_resource(resource_name, id).await
            }
            "cloudsearch" => {
                self.cloudsearch().read_resource(resource_name, id).await
            }
            "deadline" => {
                self.deadline().read_resource(resource_name, id).await
            }
            "managedblockchain" => {
                self.managedblockchain().read_resource(resource_name, id).await
            }
            "amplify" => {
                self.amplify().read_resource(resource_name, id).await
            }
            "iotsecuretunneling" => {
                self.iotsecuretunneling().read_resource(resource_name, id).await
            }
            "connectcampaigns" => {
                self.connectcampaigns().read_resource(resource_name, id).await
            }
            "kafkaconnect" => {
                self.kafkaconnect().read_resource(resource_name, id).await
            }
            "mediaconvert" => {
                self.mediaconvert().read_resource(resource_name, id).await
            }
            "data_pipeline" => {
                self.data_pipeline().read_resource(resource_name, id).await
            }
            "codepipeline" => {
                self.codepipeline().read_resource(resource_name, id).await
            }
            "clouddirectory" => {
                self.clouddirectory().read_resource(resource_name, id).await
            }
            "amplifyuibuilder" => {
                self.amplifyuibuilder().read_resource(resource_name, id).await
            }
            "rtbfabric" => {
                self.rtbfabric().read_resource(resource_name, id).await
            }
            "memorydb" => {
                self.memorydb().read_resource(resource_name, id).await
            }
            "iot" => {
                self.iot().read_resource(resource_name, id).await
            }
            "marketplace_commerce_analytics" => {
                self.marketplace_commerce_analytics().read_resource(resource_name, id).await
            }
            "frauddetector" => {
                self.frauddetector().read_resource(resource_name, id).await
            }
            "bedrock_data_automation" => {
                self.bedrock_data_automation().read_resource(resource_name, id).await
            }
            "elastic_load_balancing" => {
                self.elastic_load_balancing().read_resource(resource_name, id).await
            }
            "verifiedpermissions" => {
                self.verifiedpermissions().read_resource(resource_name, id).await
            }
            "networkmanager" => {
                self.networkmanager().read_resource(resource_name, id).await
            }
            "devops_guru" => {
                self.devops_guru().read_resource(resource_name, id).await
            }
            "taxsettings" => {
                self.taxsettings().read_resource(resource_name, id).await
            }
            "workspaces_instances" => {
                self.workspaces_instances().read_resource(resource_name, id).await
            }
            "arc_zonal_shift" => {
                self.arc_zonal_shift().read_resource(resource_name, id).await
            }
            "elastic_transcoder" => {
                self.elastic_transcoder().read_resource(resource_name, id).await
            }
            "fms" => {
                self.fms().read_resource(resource_name, id).await
            }
            "imagebuilder" => {
                self.imagebuilder().read_resource(resource_name, id).await
            }
            "chime_sdk" => {
                self.chime_sdk().read_resource(resource_name, id).await
            }
            "groundstation" => {
                self.groundstation().read_resource(resource_name, id).await
            }
            "forecast" => {
                self.forecast().read_resource(resource_name, id).await
            }
            "appstream" => {
                self.appstream().read_resource(resource_name, id).await
            }
            "chime_sdk_meetings" => {
                self.chime_sdk_meetings().read_resource(resource_name, id).await
            }
            "comprehend" => {
                self.comprehend().read_resource(resource_name, id).await
            }
            "redshift_serverless" => {
                self.redshift_serverless().read_resource(resource_name, id).await
            }
            "pinpoint" => {
                self.pinpoint().read_resource(resource_name, id).await
            }
            "pi" => {
                self.pi().read_resource(resource_name, id).await
            }
            "gameliftstreams" => {
                self.gameliftstreams().read_resource(resource_name, id).await
            }
            "customer_profiles" => {
                self.customer_profiles().read_resource(resource_name, id).await
            }
            "workspaces" => {
                self.workspaces().read_resource(resource_name, id).await
            }
            "auditmanager" => {
                self.auditmanager().read_resource(resource_name, id).await
            }
            "docdb" => {
                self.docdb().read_resource(resource_name, id).await
            }
            "mturk" => {
                self.mturk().read_resource(resource_name, id).await
            }
            "cognito_identity" => {
                self.cognito_identity().read_resource(resource_name, id).await
            }
            "dynamodb" => {
                self.dynamodb().read_resource(resource_name, id).await
            }
            "codeartifact" => {
                self.codeartifact().read_resource(resource_name, id).await
            }
            "organizations" => {
                self.organizations().read_resource(resource_name, id).await
            }
            "dlm" => {
                self.dlm().read_resource(resource_name, id).await
            }
            "sso" => {
                self.sso().read_resource(resource_name, id).await
            }
            "osis" => {
                self.osis().read_resource(resource_name, id).await
            }
            "migration_hub" => {
                self.migration_hub().read_resource(resource_name, id).await
            }
            "chatbot" => {
                self.chatbot().read_resource(resource_name, id).await
            }
            "docdb_elastic" => {
                self.docdb_elastic().read_resource(resource_name, id).await
            }
            "supplychain" => {
                self.supplychain().read_resource(resource_name, id).await
            }
            "ses" => {
                self.ses().read_resource(resource_name, id).await
            }
            "repostspace" => {
                self.repostspace().read_resource(resource_name, id).await
            }
            "mediastore_data" => {
                self.mediastore_data().read_resource(resource_name, id).await
            }
            "bedrock_agent" => {
                self.bedrock_agent().read_resource(resource_name, id).await
            }
            "wellarchitected" => {
                self.wellarchitected().read_resource(resource_name, id).await
            }
            "budgets" => {
                self.budgets().read_resource(resource_name, id).await
            }
            "mediatailor" => {
                self.mediatailor().read_resource(resource_name, id).await
            }
            "appsync" => {
                self.appsync().read_resource(resource_name, id).await
            }
            "ssm_guiconnect" => {
                self.ssm_guiconnect().read_resource(resource_name, id).await
            }
            "evs" => {
                self.evs().read_resource(resource_name, id).await
            }
            "eks_auth" => {
                self.eks_auth().read_resource(resource_name, id).await
            }
            "chime_sdk_messaging" => {
                self.chime_sdk_messaging().read_resource(resource_name, id).await
            }
            "mediaconnect" => {
                self.mediaconnect().read_resource(resource_name, id).await
            }
            "identitystore" => {
                self.identitystore().read_resource(resource_name, id).await
            }
            "bcm_pricing_calculator" => {
                self.bcm_pricing_calculator().read_resource(resource_name, id).await
            }
            "lakeformation" => {
                self.lakeformation().read_resource(resource_name, id).await
            }
            "xray" => {
                self.xray().read_resource(resource_name, id).await
            }
            "cloudfront" => {
                self.cloudfront().read_resource(resource_name, id).await
            }
            "sagemaker_metrics" => {
                self.sagemaker_metrics().read_resource(resource_name, id).await
            }
            "sso_oidc" => {
                self.sso_oidc().read_resource(resource_name, id).await
            }
            "sagemaker" => {
                self.sagemaker().read_resource(resource_name, id).await
            }
            "codestar_connections" => {
                self.codestar_connections().read_resource(resource_name, id).await
            }
            "device_farm" => {
                self.device_farm().read_resource(resource_name, id).await
            }
            "translate" => {
                self.translate().read_resource(resource_name, id).await
            }
            "sagemaker_runtime" => {
                self.sagemaker_runtime().read_resource(resource_name, id).await
            }
            "b2bi" => {
                self.b2bi().read_resource(resource_name, id).await
            }
            "savingsplans" => {
                self.savingsplans().read_resource(resource_name, id).await
            }
            "pipes" => {
                self.pipes().read_resource(resource_name, id).await
            }
            "config_service" => {
                self.config_service().read_resource(resource_name, id).await
            }
            "codeguruprofiler" => {
                self.codeguruprofiler().read_resource(resource_name, id).await
            }
            "s3" => {
                self.s3().read_resource(resource_name, id).await
            }
            "polly" => {
                self.polly().read_resource(resource_name, id).await
            }
            "cognito_sync" => {
                self.cognito_sync().read_resource(resource_name, id).await
            }
            "scheduler" => {
                self.scheduler().read_resource(resource_name, id).await
            }
            "pca_connector_ad" => {
                self.pca_connector_ad().read_resource(resource_name, id).await
            }
            "waf_regional" => {
                self.waf_regional().read_resource(resource_name, id).await
            }
            "apigatewaymanagementapi" => {
                self.apigatewaymanagementapi().read_resource(resource_name, id).await
            }
            "workspaces_web" => {
                self.workspaces_web().read_resource(resource_name, id).await
            }
            "pca_connector_scep" => {
                self.pca_connector_scep().read_resource(resource_name, id).await
            }
            "codestar_notifications" => {
                self.codestar_notifications().read_resource(resource_name, id).await
            }
            "direct_connect" => {
                self.direct_connect().read_resource(resource_name, id).await
            }
            "shield" => {
                self.shield().read_resource(resource_name, id).await
            }
            "application_signals" => {
                self.application_signals().read_resource(resource_name, id).await
            }
            "iot_managed_integrations" => {
                self.iot_managed_integrations().read_resource(resource_name, id).await
            }
            "iot_wireless" => {
                self.iot_wireless().read_resource(resource_name, id).await
            }
            "iot_events" => {
                self.iot_events().read_resource(resource_name, id).await
            }
            "backup_gateway" => {
                self.backup_gateway().read_resource(resource_name, id).await
            }
            "sso_admin" => {
                self.sso_admin().read_resource(resource_name, id).await
            }
            "elastic_beanstalk" => {
                self.elastic_beanstalk().read_resource(resource_name, id).await
            }
            "drs" => {
                self.drs().read_resource(resource_name, id).await
            }
            "personalize_runtime" => {
                self.personalize_runtime().read_resource(resource_name, id).await
            }
            "outposts" => {
                self.outposts().read_resource(resource_name, id).await
            }
            "license_manager_user_subscriptions" => {
                self.license_manager_user_subscriptions().read_resource(resource_name, id).await
            }
            "cloudtrail_data" => {
                self.cloudtrail_data().read_resource(resource_name, id).await
            }
            "lex_runtime_service" => {
                self.lex_runtime_service().read_resource(resource_name, id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Update an existing resource
    async fn update(
        &self,
        resource_type: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "emr_serverless" => {
                self.emr_serverless().update_resource(resource_name, id, input).await
            }
            "cloudformation" => {
                self.cloudformation().update_resource(resource_name, id, input).await
            }
            "application_auto_scaling" => {
                self.application_auto_scaling().update_resource(resource_name, id, input).await
            }
            "personalize_events" => {
                self.personalize_events().update_resource(resource_name, id, input).await
            }
            "tnb" => {
                self.tnb().update_resource(resource_name, id, input).await
            }
            "rolesanywhere" => {
                self.rolesanywhere().update_resource(resource_name, id, input).await
            }
            "kms" => {
                self.kms().update_resource(resource_name, id, input).await
            }
            "datasync" => {
                self.datasync().update_resource(resource_name, id, input).await
            }
            "bedrock_agent_runtime" => {
                self.bedrock_agent_runtime().update_resource(resource_name, id, input).await
            }
            "pinpoint_email" => {
                self.pinpoint_email().update_resource(resource_name, id, input).await
            }
            "connect_contact_lens" => {
                self.connect_contact_lens().update_resource(resource_name, id, input).await
            }
            "athena" => {
                self.athena().update_resource(resource_name, id, input).await
            }
            "iotfleetwise" => {
                self.iotfleetwise().update_resource(resource_name, id, input).await
            }
            "iot_data_plane" => {
                self.iot_data_plane().update_resource(resource_name, id, input).await
            }
            "bedrock_data_automation_runtime" => {
                self.bedrock_data_automation_runtime().update_resource(resource_name, id, input).await
            }
            "entityresolution" => {
                self.entityresolution().update_resource(resource_name, id, input).await
            }
            "forecastquery" => {
                self.forecastquery().update_resource(resource_name, id, input).await
            }
            "detective" => {
                self.detective().update_resource(resource_name, id, input).await
            }
            "panorama" => {
                self.panorama().update_resource(resource_name, id, input).await
            }
            "backup" => {
                self.backup().update_resource(resource_name, id, input).await
            }
            "mwaa" => {
                self.mwaa().update_resource(resource_name, id, input).await
            }
            "iot_jobs_data_plane" => {
                self.iot_jobs_data_plane().update_resource(resource_name, id, input).await
            }
            "transcribe" => {
                self.transcribe().update_resource(resource_name, id, input).await
            }
            "cloudwatch" => {
                self.cloudwatch().update_resource(resource_name, id, input).await
            }
            "snow_device_management" => {
                self.snow_device_management().update_resource(resource_name, id, input).await
            }
            "workmailmessageflow" => {
                self.workmailmessageflow().update_resource(resource_name, id, input).await
            }
            "appconfig" => {
                self.appconfig().update_resource(resource_name, id, input).await
            }
            "lightsail" => {
                self.lightsail().update_resource(resource_name, id, input).await
            }
            "guardduty" => {
                self.guardduty().update_resource(resource_name, id, input).await
            }
            "apigatewayv2" => {
                self.apigatewayv2().update_resource(resource_name, id, input).await
            }
            "wafv2" => {
                self.wafv2().update_resource(resource_name, id, input).await
            }
            "iotsitewise" => {
                self.iotsitewise().update_resource(resource_name, id, input).await
            }
            "iotthingsgraph" => {
                self.iotthingsgraph().update_resource(resource_name, id, input).await
            }
            "batch" => {
                self.batch().update_resource(resource_name, id, input).await
            }
            "mailmanager" => {
                self.mailmanager().update_resource(resource_name, id, input).await
            }
            "marketplace_reporting" => {
                self.marketplace_reporting().update_resource(resource_name, id, input).await
            }
            "m2" => {
                self.m2().update_resource(resource_name, id, input).await
            }
            "codedeploy" => {
                self.codedeploy().update_resource(resource_name, id, input).await
            }
            "route53_recovery_control_config" => {
                self.route53_recovery_control_config().update_resource(resource_name, id, input).await
            }
            "simspaceweaver" => {
                self.simspaceweaver().update_resource(resource_name, id, input).await
            }
            "resiliencehub" => {
                self.resiliencehub().update_resource(resource_name, id, input).await
            }
            "oam" => {
                self.oam().update_resource(resource_name, id, input).await
            }
            "license_manager_linux_subscriptions" => {
                self.license_manager_linux_subscriptions().update_resource(resource_name, id, input).await
            }
            "voice_id" => {
                self.voice_id().update_resource(resource_name, id, input).await
            }
            "chime" => {
                self.chime().update_resource(resource_name, id, input).await
            }
            "efs" => {
                self.efs().update_resource(resource_name, id, input).await
            }
            "freetier" => {
                self.freetier().update_resource(resource_name, id, input).await
            }
            "storage_gateway" => {
                self.storage_gateway().update_resource(resource_name, id, input).await
            }
            "dynamodb_streams" => {
                self.dynamodb_streams().update_resource(resource_name, id, input).await
            }
            "gamelift" => {
                self.gamelift().update_resource(resource_name, id, input).await
            }
            "inspector2" => {
                self.inspector2().update_resource(resource_name, id, input).await
            }
            "keyspaces" => {
                self.keyspaces().update_resource(resource_name, id, input).await
            }
            "sqs" => {
                self.sqs().update_resource(resource_name, id, input).await
            }
            "ram" => {
                self.ram().update_resource(resource_name, id, input).await
            }
            "ssm_sap" => {
                self.ssm_sap().update_resource(resource_name, id, input).await
            }
            "directory_service_data" => {
                self.directory_service_data().update_resource(resource_name, id, input).await
            }
            "route_53_domains" => {
                self.route_53_domains().update_resource(resource_name, id, input).await
            }
            "bedrock_agentcore" => {
                self.bedrock_agentcore().update_resource(resource_name, id, input).await
            }
            "trustedadvisor" => {
                self.trustedadvisor().update_resource(resource_name, id, input).await
            }
            "migrationhubstrategy" => {
                self.migrationhubstrategy().update_resource(resource_name, id, input).await
            }
            "dataexchange" => {
                self.dataexchange().update_resource(resource_name, id, input).await
            }
            "braket" => {
                self.braket().update_resource(resource_name, id, input).await
            }
            "codebuild" => {
                self.codebuild().update_resource(resource_name, id, input).await
            }
            "acm" => {
                self.acm().update_resource(resource_name, id, input).await
            }
            "route53_recovery_cluster" => {
                self.route53_recovery_cluster().update_resource(resource_name, id, input).await
            }
            "lookoutequipment" => {
                self.lookoutequipment().update_resource(resource_name, id, input).await
            }
            "marketplace_catalog" => {
                self.marketplace_catalog().update_resource(resource_name, id, input).await
            }
            "payment_cryptography_data" => {
                self.payment_cryptography_data().update_resource(resource_name, id, input).await
            }
            "cloud9" => {
                self.cloud9().update_resource(resource_name, id, input).await
            }
            "workdocs" => {
                self.workdocs().update_resource(resource_name, id, input).await
            }
            "license_manager" => {
                self.license_manager().update_resource(resource_name, id, input).await
            }
            "sts" => {
                self.sts().update_resource(resource_name, id, input).await
            }
            "s3vectors" => {
                self.s3vectors().update_resource(resource_name, id, input).await
            }
            "chime_sdk_media_pipelines" => {
                self.chime_sdk_media_pipelines().update_resource(resource_name, id, input).await
            }
            "machine_learning" => {
                self.machine_learning().update_resource(resource_name, id, input).await
            }
            "timestream_query" => {
                self.timestream_query().update_resource(resource_name, id, input).await
            }
            "codeguru_reviewer" => {
                self.codeguru_reviewer().update_resource(resource_name, id, input).await
            }
            "mgn" => {
                self.mgn().update_resource(resource_name, id, input).await
            }
            "evidently" => {
                self.evidently().update_resource(resource_name, id, input).await
            }
            "qbusiness" => {
                self.qbusiness().update_resource(resource_name, id, input).await
            }
            "connectcases" => {
                self.connectcases().update_resource(resource_name, id, input).await
            }
            "fsx" => {
                self.fsx().update_resource(resource_name, id, input).await
            }
            "ecr" => {
                self.ecr().update_resource(resource_name, id, input).await
            }
            "connectcampaignsv2" => {
                self.connectcampaignsv2().update_resource(resource_name, id, input).await
            }
            "rds" => {
                self.rds().update_resource(resource_name, id, input).await
            }
            "qapps" => {
                self.qapps().update_resource(resource_name, id, input).await
            }
            "qconnect" => {
                self.qconnect().update_resource(resource_name, id, input).await
            }
            "omics" => {
                self.omics().update_resource(resource_name, id, input).await
            }
            "bcm_dashboards" => {
                self.bcm_dashboards().update_resource(resource_name, id, input).await
            }
            "geo_routes" => {
                self.geo_routes().update_resource(resource_name, id, input).await
            }
            "quicksight" => {
                self.quicksight().update_resource(resource_name, id, input).await
            }
            "amp" => {
                self.amp().update_resource(resource_name, id, input).await
            }
            "opensearchserverless" => {
                self.opensearchserverless().update_resource(resource_name, id, input).await
            }
            "emr" => {
                self.emr().update_resource(resource_name, id, input).await
            }
            "service_quotas" => {
                self.service_quotas().update_resource(resource_name, id, input).await
            }
            "service_catalog_appregistry" => {
                self.service_catalog_appregistry().update_resource(resource_name, id, input).await
            }
            "migrationhub_config" => {
                self.migrationhub_config().update_resource(resource_name, id, input).await
            }
            "iam" => {
                self.iam().update_resource(resource_name, id, input).await
            }
            "accessanalyzer" => {
                self.accessanalyzer().update_resource(resource_name, id, input).await
            }
            "appconfigdata" => {
                self.appconfigdata().update_resource(resource_name, id, input).await
            }
            "route53resolver" => {
                self.route53resolver().update_resource(resource_name, id, input).await
            }
            "s3outposts" => {
                self.s3outposts().update_resource(resource_name, id, input).await
            }
            "kendra_ranking" => {
                self.kendra_ranking().update_resource(resource_name, id, input).await
            }
            "controltower" => {
                self.controltower().update_resource(resource_name, id, input).await
            }
            "arc_region_switch" => {
                self.arc_region_switch().update_resource(resource_name, id, input).await
            }
            "neptune_graph" => {
                self.neptune_graph().update_resource(resource_name, id, input).await
            }
            "route53_recovery_readiness" => {
                self.route53_recovery_readiness().update_resource(resource_name, id, input).await
            }
            "greengrassv2" => {
                self.greengrassv2().update_resource(resource_name, id, input).await
            }
            "migration_hub_refactor_spaces" => {
                self.migration_hub_refactor_spaces().update_resource(resource_name, id, input).await
            }
            "cost_and_usage_report_service" => {
                self.cost_and_usage_report_service().update_resource(resource_name, id, input).await
            }
            "ebs" => {
                self.ebs().update_resource(resource_name, id, input).await
            }
            "appflow" => {
                self.appflow().update_resource(resource_name, id, input).await
            }
            "migrationhuborchestrator" => {
                self.migrationhuborchestrator().update_resource(resource_name, id, input).await
            }
            "chime_sdk_identity" => {
                self.chime_sdk_identity().update_resource(resource_name, id, input).await
            }
            "cloudfront_keyvaluestore" => {
                self.cloudfront_keyvaluestore().update_resource(resource_name, id, input).await
            }
            "waf" => {
                self.waf().update_resource(resource_name, id, input).await
            }
            "greengrass" => {
                self.greengrass().update_resource(resource_name, id, input).await
            }
            "sagemaker_featurestore_runtime" => {
                self.sagemaker_featurestore_runtime().update_resource(resource_name, id, input).await
            }
            "inspector" => {
                self.inspector().update_resource(resource_name, id, input).await
            }
            "appfabric" => {
                self.appfabric().update_resource(resource_name, id, input).await
            }
            "lex_model_building_service" => {
                self.lex_model_building_service().update_resource(resource_name, id, input).await
            }
            "serverlessapplicationrepository" => {
                self.serverlessapplicationrepository().update_resource(resource_name, id, input).await
            }
            "cloudsearch_domain" => {
                self.cloudsearch_domain().update_resource(resource_name, id, input).await
            }
            "codeguru_security" => {
                self.codeguru_security().update_resource(resource_name, id, input).await
            }
            "socialmessaging" => {
                self.socialmessaging().update_resource(resource_name, id, input).await
            }
            "geo_maps" => {
                self.geo_maps().update_resource(resource_name, id, input).await
            }
            "kinesis" => {
                self.kinesis().update_resource(resource_name, id, input).await
            }
            "dsql" => {
                self.dsql().update_resource(resource_name, id, input).await
            }
            "appintegrations" => {
                self.appintegrations().update_resource(resource_name, id, input).await
            }
            "personalize" => {
                self.personalize().update_resource(resource_name, id, input).await
            }
            "proton" => {
                self.proton().update_resource(resource_name, id, input).await
            }
            "cloudcontrol" => {
                self.cloudcontrol().update_resource(resource_name, id, input).await
            }
            "redshift" => {
                self.redshift().update_resource(resource_name, id, input).await
            }
            "geo_places" => {
                self.geo_places().update_resource(resource_name, id, input).await
            }
            "elasticsearch_service" => {
                self.elasticsearch_service().update_resource(resource_name, id, input).await
            }
            "bcm_recommended_actions" => {
                self.bcm_recommended_actions().update_resource(resource_name, id, input).await
            }
            "invoicing" => {
                self.invoicing().update_resource(resource_name, id, input).await
            }
            "apprunner" => {
                self.apprunner().update_resource(resource_name, id, input).await
            }
            "sns" => {
                self.sns().update_resource(resource_name, id, input).await
            }
            "textract" => {
                self.textract().update_resource(resource_name, id, input).await
            }
            "workmail" => {
                self.workmail().update_resource(resource_name, id, input).await
            }
            "datazone" => {
                self.datazone().update_resource(resource_name, id, input).await
            }
            "rekognition" => {
                self.rekognition().update_resource(resource_name, id, input).await
            }
            "ssm" => {
                self.ssm().update_resource(resource_name, id, input).await
            }
            "medical_imaging" => {
                self.medical_imaging().update_resource(resource_name, id, input).await
            }
            "lex_models" => {
                self.lex_models().update_resource(resource_name, id, input).await
            }
            "support" => {
                self.support().update_resource(resource_name, id, input).await
            }
            "signer" => {
                self.signer().update_resource(resource_name, id, input).await
            }
            "partnercentral_selling" => {
                self.partnercentral_selling().update_resource(resource_name, id, input).await
            }
            "comprehendmedical" => {
                self.comprehendmedical().update_resource(resource_name, id, input).await
            }
            "macie2" => {
                self.macie2().update_resource(resource_name, id, input).await
            }
            "redshift_data" => {
                self.redshift_data().update_resource(resource_name, id, input).await
            }
            "marketplace_agreement" => {
                self.marketplace_agreement().update_resource(resource_name, id, input).await
            }
            "health" => {
                self.health().update_resource(resource_name, id, input).await
            }
            "odb" => {
                self.odb().update_resource(resource_name, id, input).await
            }
            "resource_groups_tagging_api" => {
                self.resource_groups_tagging_api().update_resource(resource_name, id, input).await
            }
            "application_insights" => {
                self.application_insights().update_resource(resource_name, id, input).await
            }
            "timestream_write" => {
                self.timestream_write().update_resource(resource_name, id, input).await
            }
            "pinpoint_sms" => {
                self.pinpoint_sms().update_resource(resource_name, id, input).await
            }
            "mediapackagev2" => {
                self.mediapackagev2().update_resource(resource_name, id, input).await
            }
            "ec2" => {
                self.ec2().update_resource(resource_name, id, input).await
            }
            "cleanrooms" => {
                self.cleanrooms().update_resource(resource_name, id, input).await
            }
            "healthlake" => {
                self.healthlake().update_resource(resource_name, id, input).await
            }
            "sfn" => {
                self.sfn().update_resource(resource_name, id, input).await
            }
            "iottwinmaker" => {
                self.iottwinmaker().update_resource(resource_name, id, input).await
            }
            "cloudtrail" => {
                self.cloudtrail().update_resource(resource_name, id, input).await
            }
            "iotdeviceadvisor" => {
                self.iotdeviceadvisor().update_resource(resource_name, id, input).await
            }
            "ssm_incidents" => {
                self.ssm_incidents().update_resource(resource_name, id, input).await
            }
            "pcs" => {
                self.pcs().update_resource(resource_name, id, input).await
            }
            "support_app" => {
                self.support_app().update_resource(resource_name, id, input).await
            }
            "managedblockchain_query" => {
                self.managedblockchain_query().update_resource(resource_name, id, input).await
            }
            "iot_events_data" => {
                self.iot_events_data().update_resource(resource_name, id, input).await
            }
            "lex_runtime" => {
                self.lex_runtime().update_resource(resource_name, id, input).await
            }
            "observabilityadmin" => {
                self.observabilityadmin().update_resource(resource_name, id, input).await
            }
            "applicationcostprofiler" => {
                self.applicationcostprofiler().update_resource(resource_name, id, input).await
            }
            "billingconductor" => {
                self.billingconductor().update_resource(resource_name, id, input).await
            }
            "artifact" => {
                self.artifact().update_resource(resource_name, id, input).await
            }
            "ecr_public" => {
                self.ecr_public().update_resource(resource_name, id, input).await
            }
            "connectparticipant" => {
                self.connectparticipant().update_resource(resource_name, id, input).await
            }
            "rds_data" => {
                self.rds_data().update_resource(resource_name, id, input).await
            }
            "internetmonitor" => {
                self.internetmonitor().update_resource(resource_name, id, input).await
            }
            "route_53" => {
                self.route_53().update_resource(resource_name, id, input).await
            }
            "bedrock_runtime" => {
                self.bedrock_runtime().update_resource(resource_name, id, input).await
            }
            "amplifybackend" => {
                self.amplifybackend().update_resource(resource_name, id, input).await
            }
            "marketplace_deployment" => {
                self.marketplace_deployment().update_resource(resource_name, id, input).await
            }
            "account" => {
                self.account().update_resource(resource_name, id, input).await
            }
            "snowball" => {
                self.snowball().update_resource(resource_name, id, input).await
            }
            "eventbridge" => {
                self.eventbridge().update_resource(resource_name, id, input).await
            }
            "auto_scaling_plans" => {
                self.auto_scaling_plans().update_resource(resource_name, id, input).await
            }
            "directory_service" => {
                self.directory_service().update_resource(resource_name, id, input).await
            }
            "mediapackage" => {
                self.mediapackage().update_resource(resource_name, id, input).await
            }
            "ssm_quicksetup" => {
                self.ssm_quicksetup().update_resource(resource_name, id, input).await
            }
            "s3_control" => {
                self.s3_control().update_resource(resource_name, id, input).await
            }
            "codecatalyst" => {
                self.codecatalyst().update_resource(resource_name, id, input).await
            }
            "notificationscontacts" => {
                self.notificationscontacts().update_resource(resource_name, id, input).await
            }
            "mpa" => {
                self.mpa().update_resource(resource_name, id, input).await
            }
            "ec2_instance_connect" => {
                self.ec2_instance_connect().update_resource(resource_name, id, input).await
            }
            "sagemaker_geospatial" => {
                self.sagemaker_geospatial().update_resource(resource_name, id, input).await
            }
            "notifications" => {
                self.notifications().update_resource(resource_name, id, input).await
            }
            "securitylake" => {
                self.securitylake().update_resource(resource_name, id, input).await
            }
            "networkmonitor" => {
                self.networkmonitor().update_resource(resource_name, id, input).await
            }
            "codeconnections" => {
                self.codeconnections().update_resource(resource_name, id, input).await
            }
            "app_mesh" => {
                self.app_mesh().update_resource(resource_name, id, input).await
            }
            "workspaces_thin_client" => {
                self.workspaces_thin_client().update_resource(resource_name, id, input).await
            }
            "finspace_data" => {
                self.finspace_data().update_resource(resource_name, id, input).await
            }
            "compute_optimizer" => {
                self.compute_optimizer().update_resource(resource_name, id, input).await
            }
            "secrets_manager" => {
                self.secrets_manager().update_resource(resource_name, id, input).await
            }
            "mediastore" => {
                self.mediastore().update_resource(resource_name, id, input).await
            }
            "ecs" => {
                self.ecs().update_resource(resource_name, id, input).await
            }
            "vpc_lattice" => {
                self.vpc_lattice().update_resource(resource_name, id, input).await
            }
            "auto_scaling" => {
                self.auto_scaling().update_resource(resource_name, id, input).await
            }
            "resource_groups" => {
                self.resource_groups().update_resource(resource_name, id, input).await
            }
            "eks" => {
                self.eks().update_resource(resource_name, id, input).await
            }
            "marketplace_entitlement_service" => {
                self.marketplace_entitlement_service().update_resource(resource_name, id, input).await
            }
            "database_migration_service" => {
                self.database_migration_service().update_resource(resource_name, id, input).await
            }
            "security_ir" => {
                self.security_ir().update_resource(resource_name, id, input).await
            }
            "inspector_scan" => {
                self.inspector_scan().update_resource(resource_name, id, input).await
            }
            "global_accelerator" => {
                self.global_accelerator().update_resource(resource_name, id, input).await
            }
            "kinesis_analytics" => {
                self.kinesis_analytics().update_resource(resource_name, id, input).await
            }
            "neptunedata" => {
                self.neptunedata().update_resource(resource_name, id, input).await
            }
            "swf" => {
                self.swf().update_resource(resource_name, id, input).await
            }
            "cloudwatch_logs" => {
                self.cloudwatch_logs().update_resource(resource_name, id, input).await
            }
            "connect" => {
                self.connect().update_resource(resource_name, id, input).await
            }
            "glue" => {
                self.glue().update_resource(resource_name, id, input).await
            }
            "cognito_identity_provider" => {
                self.cognito_identity_provider().update_resource(resource_name, id, input).await
            }
            "cloudwatch_events" => {
                self.cloudwatch_events().update_resource(resource_name, id, input).await
            }
            "cost_explorer" => {
                self.cost_explorer().update_resource(resource_name, id, input).await
            }
            "network_firewall" => {
                self.network_firewall().update_resource(resource_name, id, input).await
            }
            "firehose" => {
                self.firehose().update_resource(resource_name, id, input).await
            }
            "transfer" => {
                self.transfer().update_resource(resource_name, id, input).await
            }
            "marketplace_metering" => {
                self.marketplace_metering().update_resource(resource_name, id, input).await
            }
            "rbin" => {
                self.rbin().update_resource(resource_name, id, input).await
            }
            "timestream_influxdb" => {
                self.timestream_influxdb().update_resource(resource_name, id, input).await
            }
            "iotanalytics" => {
                self.iotanalytics().update_resource(resource_name, id, input).await
            }
            "ivs" => {
                self.ivs().update_resource(resource_name, id, input).await
            }
            "kafka" => {
                self.kafka().update_resource(resource_name, id, input).await
            }
            "sesv2" => {
                self.sesv2().update_resource(resource_name, id, input).await
            }
            "kendra" => {
                self.kendra().update_resource(resource_name, id, input).await
            }
            "sagemaker_edge" => {
                self.sagemaker_edge().update_resource(resource_name, id, input).await
            }
            "launch_wizard" => {
                self.launch_wizard().update_resource(resource_name, id, input).await
            }
            "securityhub" => {
                self.securityhub().update_resource(resource_name, id, input).await
            }
            "finspace" => {
                self.finspace().update_resource(resource_name, id, input).await
            }
            "keyspacesstreams" => {
                self.keyspacesstreams().update_resource(resource_name, id, input).await
            }
            "cleanroomsml" => {
                self.cleanroomsml().update_resource(resource_name, id, input).await
            }
            "transcribe_streaming" => {
                self.transcribe_streaming().update_resource(resource_name, id, input).await
            }
            "aiops" => {
                self.aiops().update_resource(resource_name, id, input).await
            }
            "service_catalog" => {
                self.service_catalog().update_resource(resource_name, id, input).await
            }
            "databrew" => {
                self.databrew().update_resource(resource_name, id, input).await
            }
            "codecommit" => {
                self.codecommit().update_resource(resource_name, id, input).await
            }
            "resource_explorer_2" => {
                self.resource_explorer_2().update_resource(resource_name, id, input).await
            }
            "acm_pca" => {
                self.acm_pca().update_resource(resource_name, id, input).await
            }
            "payment_cryptography" => {
                self.payment_cryptography().update_resource(resource_name, id, input).await
            }
            "mq" => {
                self.mq().update_resource(resource_name, id, input).await
            }
            "api_gateway" => {
                self.api_gateway().update_resource(resource_name, id, input).await
            }
            "grafana" => {
                self.grafana().update_resource(resource_name, id, input).await
            }
            "glacier" => {
                self.glacier().update_resource(resource_name, id, input).await
            }
            "bedrock" => {
                self.bedrock().update_resource(resource_name, id, input).await
            }
            "s3tables" => {
                self.s3tables().update_resource(resource_name, id, input).await
            }
            "ivs_realtime" => {
                self.ivs_realtime().update_resource(resource_name, id, input).await
            }
            "medialive" => {
                self.medialive().update_resource(resource_name, id, input).await
            }
            "backupsearch" => {
                self.backupsearch().update_resource(resource_name, id, input).await
            }
            "networkflowmonitor" => {
                self.networkflowmonitor().update_resource(resource_name, id, input).await
            }
            "elasticache" => {
                self.elasticache().update_resource(resource_name, id, input).await
            }
            "fis" => {
                self.fis().update_resource(resource_name, id, input).await
            }
            "cloudhsm" => {
                self.cloudhsm().update_resource(resource_name, id, input).await
            }
            "cost_optimization_hub" => {
                self.cost_optimization_hub().update_resource(resource_name, id, input).await
            }
            "synthetics" => {
                self.synthetics().update_resource(resource_name, id, input).await
            }
            "rum" => {
                self.rum().update_resource(resource_name, id, input).await
            }
            "emr_containers" => {
                self.emr_containers().update_resource(resource_name, id, input).await
            }
            "sagemaker_a2i_runtime" => {
                self.sagemaker_a2i_runtime().update_resource(resource_name, id, input).await
            }
            "ssm_contacts" => {
                self.ssm_contacts().update_resource(resource_name, id, input).await
            }
            "bcm_data_exports" => {
                self.bcm_data_exports().update_resource(resource_name, id, input).await
            }
            "opensearch" => {
                self.opensearch().update_resource(resource_name, id, input).await
            }
            "dax" => {
                self.dax().update_resource(resource_name, id, input).await
            }
            "neptune" => {
                self.neptune().update_resource(resource_name, id, input).await
            }
            "pricing" => {
                self.pricing().update_resource(resource_name, id, input).await
            }
            "location" => {
                self.location().update_resource(resource_name, id, input).await
            }
            "route53profiles" => {
                self.route53profiles().update_resource(resource_name, id, input).await
            }
            "lambda" => {
                self.lambda().update_resource(resource_name, id, input).await
            }
            "ivschat" => {
                self.ivschat().update_resource(resource_name, id, input).await
            }
            "billing" => {
                self.billing().update_resource(resource_name, id, input).await
            }
            "wisdom" => {
                self.wisdom().update_resource(resource_name, id, input).await
            }
            "schemas" => {
                self.schemas().update_resource(resource_name, id, input).await
            }
            "bedrock_agentcore_control" => {
                self.bedrock_agentcore_control().update_resource(resource_name, id, input).await
            }
            "controlcatalog" => {
                self.controlcatalog().update_resource(resource_name, id, input).await
            }
            "cloudsearch" => {
                self.cloudsearch().update_resource(resource_name, id, input).await
            }
            "deadline" => {
                self.deadline().update_resource(resource_name, id, input).await
            }
            "managedblockchain" => {
                self.managedblockchain().update_resource(resource_name, id, input).await
            }
            "amplify" => {
                self.amplify().update_resource(resource_name, id, input).await
            }
            "iotsecuretunneling" => {
                self.iotsecuretunneling().update_resource(resource_name, id, input).await
            }
            "connectcampaigns" => {
                self.connectcampaigns().update_resource(resource_name, id, input).await
            }
            "kafkaconnect" => {
                self.kafkaconnect().update_resource(resource_name, id, input).await
            }
            "mediaconvert" => {
                self.mediaconvert().update_resource(resource_name, id, input).await
            }
            "data_pipeline" => {
                self.data_pipeline().update_resource(resource_name, id, input).await
            }
            "codepipeline" => {
                self.codepipeline().update_resource(resource_name, id, input).await
            }
            "clouddirectory" => {
                self.clouddirectory().update_resource(resource_name, id, input).await
            }
            "amplifyuibuilder" => {
                self.amplifyuibuilder().update_resource(resource_name, id, input).await
            }
            "rtbfabric" => {
                self.rtbfabric().update_resource(resource_name, id, input).await
            }
            "memorydb" => {
                self.memorydb().update_resource(resource_name, id, input).await
            }
            "iot" => {
                self.iot().update_resource(resource_name, id, input).await
            }
            "marketplace_commerce_analytics" => {
                self.marketplace_commerce_analytics().update_resource(resource_name, id, input).await
            }
            "frauddetector" => {
                self.frauddetector().update_resource(resource_name, id, input).await
            }
            "bedrock_data_automation" => {
                self.bedrock_data_automation().update_resource(resource_name, id, input).await
            }
            "elastic_load_balancing" => {
                self.elastic_load_balancing().update_resource(resource_name, id, input).await
            }
            "verifiedpermissions" => {
                self.verifiedpermissions().update_resource(resource_name, id, input).await
            }
            "networkmanager" => {
                self.networkmanager().update_resource(resource_name, id, input).await
            }
            "devops_guru" => {
                self.devops_guru().update_resource(resource_name, id, input).await
            }
            "taxsettings" => {
                self.taxsettings().update_resource(resource_name, id, input).await
            }
            "workspaces_instances" => {
                self.workspaces_instances().update_resource(resource_name, id, input).await
            }
            "arc_zonal_shift" => {
                self.arc_zonal_shift().update_resource(resource_name, id, input).await
            }
            "elastic_transcoder" => {
                self.elastic_transcoder().update_resource(resource_name, id, input).await
            }
            "fms" => {
                self.fms().update_resource(resource_name, id, input).await
            }
            "imagebuilder" => {
                self.imagebuilder().update_resource(resource_name, id, input).await
            }
            "chime_sdk" => {
                self.chime_sdk().update_resource(resource_name, id, input).await
            }
            "groundstation" => {
                self.groundstation().update_resource(resource_name, id, input).await
            }
            "forecast" => {
                self.forecast().update_resource(resource_name, id, input).await
            }
            "appstream" => {
                self.appstream().update_resource(resource_name, id, input).await
            }
            "chime_sdk_meetings" => {
                self.chime_sdk_meetings().update_resource(resource_name, id, input).await
            }
            "comprehend" => {
                self.comprehend().update_resource(resource_name, id, input).await
            }
            "redshift_serverless" => {
                self.redshift_serverless().update_resource(resource_name, id, input).await
            }
            "pinpoint" => {
                self.pinpoint().update_resource(resource_name, id, input).await
            }
            "pi" => {
                self.pi().update_resource(resource_name, id, input).await
            }
            "gameliftstreams" => {
                self.gameliftstreams().update_resource(resource_name, id, input).await
            }
            "customer_profiles" => {
                self.customer_profiles().update_resource(resource_name, id, input).await
            }
            "workspaces" => {
                self.workspaces().update_resource(resource_name, id, input).await
            }
            "auditmanager" => {
                self.auditmanager().update_resource(resource_name, id, input).await
            }
            "docdb" => {
                self.docdb().update_resource(resource_name, id, input).await
            }
            "mturk" => {
                self.mturk().update_resource(resource_name, id, input).await
            }
            "cognito_identity" => {
                self.cognito_identity().update_resource(resource_name, id, input).await
            }
            "dynamodb" => {
                self.dynamodb().update_resource(resource_name, id, input).await
            }
            "codeartifact" => {
                self.codeartifact().update_resource(resource_name, id, input).await
            }
            "organizations" => {
                self.organizations().update_resource(resource_name, id, input).await
            }
            "dlm" => {
                self.dlm().update_resource(resource_name, id, input).await
            }
            "sso" => {
                self.sso().update_resource(resource_name, id, input).await
            }
            "osis" => {
                self.osis().update_resource(resource_name, id, input).await
            }
            "migration_hub" => {
                self.migration_hub().update_resource(resource_name, id, input).await
            }
            "chatbot" => {
                self.chatbot().update_resource(resource_name, id, input).await
            }
            "docdb_elastic" => {
                self.docdb_elastic().update_resource(resource_name, id, input).await
            }
            "supplychain" => {
                self.supplychain().update_resource(resource_name, id, input).await
            }
            "ses" => {
                self.ses().update_resource(resource_name, id, input).await
            }
            "repostspace" => {
                self.repostspace().update_resource(resource_name, id, input).await
            }
            "mediastore_data" => {
                self.mediastore_data().update_resource(resource_name, id, input).await
            }
            "bedrock_agent" => {
                self.bedrock_agent().update_resource(resource_name, id, input).await
            }
            "wellarchitected" => {
                self.wellarchitected().update_resource(resource_name, id, input).await
            }
            "budgets" => {
                self.budgets().update_resource(resource_name, id, input).await
            }
            "mediatailor" => {
                self.mediatailor().update_resource(resource_name, id, input).await
            }
            "appsync" => {
                self.appsync().update_resource(resource_name, id, input).await
            }
            "ssm_guiconnect" => {
                self.ssm_guiconnect().update_resource(resource_name, id, input).await
            }
            "evs" => {
                self.evs().update_resource(resource_name, id, input).await
            }
            "eks_auth" => {
                self.eks_auth().update_resource(resource_name, id, input).await
            }
            "chime_sdk_messaging" => {
                self.chime_sdk_messaging().update_resource(resource_name, id, input).await
            }
            "mediaconnect" => {
                self.mediaconnect().update_resource(resource_name, id, input).await
            }
            "identitystore" => {
                self.identitystore().update_resource(resource_name, id, input).await
            }
            "bcm_pricing_calculator" => {
                self.bcm_pricing_calculator().update_resource(resource_name, id, input).await
            }
            "lakeformation" => {
                self.lakeformation().update_resource(resource_name, id, input).await
            }
            "xray" => {
                self.xray().update_resource(resource_name, id, input).await
            }
            "cloudfront" => {
                self.cloudfront().update_resource(resource_name, id, input).await
            }
            "sagemaker_metrics" => {
                self.sagemaker_metrics().update_resource(resource_name, id, input).await
            }
            "sso_oidc" => {
                self.sso_oidc().update_resource(resource_name, id, input).await
            }
            "sagemaker" => {
                self.sagemaker().update_resource(resource_name, id, input).await
            }
            "codestar_connections" => {
                self.codestar_connections().update_resource(resource_name, id, input).await
            }
            "device_farm" => {
                self.device_farm().update_resource(resource_name, id, input).await
            }
            "translate" => {
                self.translate().update_resource(resource_name, id, input).await
            }
            "sagemaker_runtime" => {
                self.sagemaker_runtime().update_resource(resource_name, id, input).await
            }
            "b2bi" => {
                self.b2bi().update_resource(resource_name, id, input).await
            }
            "savingsplans" => {
                self.savingsplans().update_resource(resource_name, id, input).await
            }
            "pipes" => {
                self.pipes().update_resource(resource_name, id, input).await
            }
            "config_service" => {
                self.config_service().update_resource(resource_name, id, input).await
            }
            "codeguruprofiler" => {
                self.codeguruprofiler().update_resource(resource_name, id, input).await
            }
            "s3" => {
                self.s3().update_resource(resource_name, id, input).await
            }
            "polly" => {
                self.polly().update_resource(resource_name, id, input).await
            }
            "cognito_sync" => {
                self.cognito_sync().update_resource(resource_name, id, input).await
            }
            "scheduler" => {
                self.scheduler().update_resource(resource_name, id, input).await
            }
            "pca_connector_ad" => {
                self.pca_connector_ad().update_resource(resource_name, id, input).await
            }
            "waf_regional" => {
                self.waf_regional().update_resource(resource_name, id, input).await
            }
            "apigatewaymanagementapi" => {
                self.apigatewaymanagementapi().update_resource(resource_name, id, input).await
            }
            "workspaces_web" => {
                self.workspaces_web().update_resource(resource_name, id, input).await
            }
            "pca_connector_scep" => {
                self.pca_connector_scep().update_resource(resource_name, id, input).await
            }
            "codestar_notifications" => {
                self.codestar_notifications().update_resource(resource_name, id, input).await
            }
            "direct_connect" => {
                self.direct_connect().update_resource(resource_name, id, input).await
            }
            "shield" => {
                self.shield().update_resource(resource_name, id, input).await
            }
            "application_signals" => {
                self.application_signals().update_resource(resource_name, id, input).await
            }
            "iot_managed_integrations" => {
                self.iot_managed_integrations().update_resource(resource_name, id, input).await
            }
            "iot_wireless" => {
                self.iot_wireless().update_resource(resource_name, id, input).await
            }
            "iot_events" => {
                self.iot_events().update_resource(resource_name, id, input).await
            }
            "backup_gateway" => {
                self.backup_gateway().update_resource(resource_name, id, input).await
            }
            "sso_admin" => {
                self.sso_admin().update_resource(resource_name, id, input).await
            }
            "elastic_beanstalk" => {
                self.elastic_beanstalk().update_resource(resource_name, id, input).await
            }
            "drs" => {
                self.drs().update_resource(resource_name, id, input).await
            }
            "personalize_runtime" => {
                self.personalize_runtime().update_resource(resource_name, id, input).await
            }
            "outposts" => {
                self.outposts().update_resource(resource_name, id, input).await
            }
            "license_manager_user_subscriptions" => {
                self.license_manager_user_subscriptions().update_resource(resource_name, id, input).await
            }
            "cloudtrail_data" => {
                self.cloudtrail_data().update_resource(resource_name, id, input).await
            }
            "lex_runtime_service" => {
                self.lex_runtime_service().update_resource(resource_name, id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Delete a resource
    async fn delete(&self, resource_type: &str, id: &str) -> Result<()> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "emr_serverless" => {
                self.emr_serverless().delete_resource(resource_name, id).await
            }
            "cloudformation" => {
                self.cloudformation().delete_resource(resource_name, id).await
            }
            "application_auto_scaling" => {
                self.application_auto_scaling().delete_resource(resource_name, id).await
            }
            "personalize_events" => {
                self.personalize_events().delete_resource(resource_name, id).await
            }
            "tnb" => {
                self.tnb().delete_resource(resource_name, id).await
            }
            "rolesanywhere" => {
                self.rolesanywhere().delete_resource(resource_name, id).await
            }
            "kms" => {
                self.kms().delete_resource(resource_name, id).await
            }
            "datasync" => {
                self.datasync().delete_resource(resource_name, id).await
            }
            "bedrock_agent_runtime" => {
                self.bedrock_agent_runtime().delete_resource(resource_name, id).await
            }
            "pinpoint_email" => {
                self.pinpoint_email().delete_resource(resource_name, id).await
            }
            "connect_contact_lens" => {
                self.connect_contact_lens().delete_resource(resource_name, id).await
            }
            "athena" => {
                self.athena().delete_resource(resource_name, id).await
            }
            "iotfleetwise" => {
                self.iotfleetwise().delete_resource(resource_name, id).await
            }
            "iot_data_plane" => {
                self.iot_data_plane().delete_resource(resource_name, id).await
            }
            "bedrock_data_automation_runtime" => {
                self.bedrock_data_automation_runtime().delete_resource(resource_name, id).await
            }
            "entityresolution" => {
                self.entityresolution().delete_resource(resource_name, id).await
            }
            "forecastquery" => {
                self.forecastquery().delete_resource(resource_name, id).await
            }
            "detective" => {
                self.detective().delete_resource(resource_name, id).await
            }
            "panorama" => {
                self.panorama().delete_resource(resource_name, id).await
            }
            "backup" => {
                self.backup().delete_resource(resource_name, id).await
            }
            "mwaa" => {
                self.mwaa().delete_resource(resource_name, id).await
            }
            "iot_jobs_data_plane" => {
                self.iot_jobs_data_plane().delete_resource(resource_name, id).await
            }
            "transcribe" => {
                self.transcribe().delete_resource(resource_name, id).await
            }
            "cloudwatch" => {
                self.cloudwatch().delete_resource(resource_name, id).await
            }
            "snow_device_management" => {
                self.snow_device_management().delete_resource(resource_name, id).await
            }
            "workmailmessageflow" => {
                self.workmailmessageflow().delete_resource(resource_name, id).await
            }
            "appconfig" => {
                self.appconfig().delete_resource(resource_name, id).await
            }
            "lightsail" => {
                self.lightsail().delete_resource(resource_name, id).await
            }
            "guardduty" => {
                self.guardduty().delete_resource(resource_name, id).await
            }
            "apigatewayv2" => {
                self.apigatewayv2().delete_resource(resource_name, id).await
            }
            "wafv2" => {
                self.wafv2().delete_resource(resource_name, id).await
            }
            "iotsitewise" => {
                self.iotsitewise().delete_resource(resource_name, id).await
            }
            "iotthingsgraph" => {
                self.iotthingsgraph().delete_resource(resource_name, id).await
            }
            "batch" => {
                self.batch().delete_resource(resource_name, id).await
            }
            "mailmanager" => {
                self.mailmanager().delete_resource(resource_name, id).await
            }
            "marketplace_reporting" => {
                self.marketplace_reporting().delete_resource(resource_name, id).await
            }
            "m2" => {
                self.m2().delete_resource(resource_name, id).await
            }
            "codedeploy" => {
                self.codedeploy().delete_resource(resource_name, id).await
            }
            "route53_recovery_control_config" => {
                self.route53_recovery_control_config().delete_resource(resource_name, id).await
            }
            "simspaceweaver" => {
                self.simspaceweaver().delete_resource(resource_name, id).await
            }
            "resiliencehub" => {
                self.resiliencehub().delete_resource(resource_name, id).await
            }
            "oam" => {
                self.oam().delete_resource(resource_name, id).await
            }
            "license_manager_linux_subscriptions" => {
                self.license_manager_linux_subscriptions().delete_resource(resource_name, id).await
            }
            "voice_id" => {
                self.voice_id().delete_resource(resource_name, id).await
            }
            "chime" => {
                self.chime().delete_resource(resource_name, id).await
            }
            "efs" => {
                self.efs().delete_resource(resource_name, id).await
            }
            "freetier" => {
                self.freetier().delete_resource(resource_name, id).await
            }
            "storage_gateway" => {
                self.storage_gateway().delete_resource(resource_name, id).await
            }
            "dynamodb_streams" => {
                self.dynamodb_streams().delete_resource(resource_name, id).await
            }
            "gamelift" => {
                self.gamelift().delete_resource(resource_name, id).await
            }
            "inspector2" => {
                self.inspector2().delete_resource(resource_name, id).await
            }
            "keyspaces" => {
                self.keyspaces().delete_resource(resource_name, id).await
            }
            "sqs" => {
                self.sqs().delete_resource(resource_name, id).await
            }
            "ram" => {
                self.ram().delete_resource(resource_name, id).await
            }
            "ssm_sap" => {
                self.ssm_sap().delete_resource(resource_name, id).await
            }
            "directory_service_data" => {
                self.directory_service_data().delete_resource(resource_name, id).await
            }
            "route_53_domains" => {
                self.route_53_domains().delete_resource(resource_name, id).await
            }
            "bedrock_agentcore" => {
                self.bedrock_agentcore().delete_resource(resource_name, id).await
            }
            "trustedadvisor" => {
                self.trustedadvisor().delete_resource(resource_name, id).await
            }
            "migrationhubstrategy" => {
                self.migrationhubstrategy().delete_resource(resource_name, id).await
            }
            "dataexchange" => {
                self.dataexchange().delete_resource(resource_name, id).await
            }
            "braket" => {
                self.braket().delete_resource(resource_name, id).await
            }
            "codebuild" => {
                self.codebuild().delete_resource(resource_name, id).await
            }
            "acm" => {
                self.acm().delete_resource(resource_name, id).await
            }
            "route53_recovery_cluster" => {
                self.route53_recovery_cluster().delete_resource(resource_name, id).await
            }
            "lookoutequipment" => {
                self.lookoutequipment().delete_resource(resource_name, id).await
            }
            "marketplace_catalog" => {
                self.marketplace_catalog().delete_resource(resource_name, id).await
            }
            "payment_cryptography_data" => {
                self.payment_cryptography_data().delete_resource(resource_name, id).await
            }
            "cloud9" => {
                self.cloud9().delete_resource(resource_name, id).await
            }
            "workdocs" => {
                self.workdocs().delete_resource(resource_name, id).await
            }
            "license_manager" => {
                self.license_manager().delete_resource(resource_name, id).await
            }
            "sts" => {
                self.sts().delete_resource(resource_name, id).await
            }
            "s3vectors" => {
                self.s3vectors().delete_resource(resource_name, id).await
            }
            "chime_sdk_media_pipelines" => {
                self.chime_sdk_media_pipelines().delete_resource(resource_name, id).await
            }
            "machine_learning" => {
                self.machine_learning().delete_resource(resource_name, id).await
            }
            "timestream_query" => {
                self.timestream_query().delete_resource(resource_name, id).await
            }
            "codeguru_reviewer" => {
                self.codeguru_reviewer().delete_resource(resource_name, id).await
            }
            "mgn" => {
                self.mgn().delete_resource(resource_name, id).await
            }
            "evidently" => {
                self.evidently().delete_resource(resource_name, id).await
            }
            "qbusiness" => {
                self.qbusiness().delete_resource(resource_name, id).await
            }
            "connectcases" => {
                self.connectcases().delete_resource(resource_name, id).await
            }
            "fsx" => {
                self.fsx().delete_resource(resource_name, id).await
            }
            "ecr" => {
                self.ecr().delete_resource(resource_name, id).await
            }
            "connectcampaignsv2" => {
                self.connectcampaignsv2().delete_resource(resource_name, id).await
            }
            "rds" => {
                self.rds().delete_resource(resource_name, id).await
            }
            "qapps" => {
                self.qapps().delete_resource(resource_name, id).await
            }
            "qconnect" => {
                self.qconnect().delete_resource(resource_name, id).await
            }
            "omics" => {
                self.omics().delete_resource(resource_name, id).await
            }
            "bcm_dashboards" => {
                self.bcm_dashboards().delete_resource(resource_name, id).await
            }
            "geo_routes" => {
                self.geo_routes().delete_resource(resource_name, id).await
            }
            "quicksight" => {
                self.quicksight().delete_resource(resource_name, id).await
            }
            "amp" => {
                self.amp().delete_resource(resource_name, id).await
            }
            "opensearchserverless" => {
                self.opensearchserverless().delete_resource(resource_name, id).await
            }
            "emr" => {
                self.emr().delete_resource(resource_name, id).await
            }
            "service_quotas" => {
                self.service_quotas().delete_resource(resource_name, id).await
            }
            "service_catalog_appregistry" => {
                self.service_catalog_appregistry().delete_resource(resource_name, id).await
            }
            "migrationhub_config" => {
                self.migrationhub_config().delete_resource(resource_name, id).await
            }
            "iam" => {
                self.iam().delete_resource(resource_name, id).await
            }
            "accessanalyzer" => {
                self.accessanalyzer().delete_resource(resource_name, id).await
            }
            "appconfigdata" => {
                self.appconfigdata().delete_resource(resource_name, id).await
            }
            "route53resolver" => {
                self.route53resolver().delete_resource(resource_name, id).await
            }
            "s3outposts" => {
                self.s3outposts().delete_resource(resource_name, id).await
            }
            "kendra_ranking" => {
                self.kendra_ranking().delete_resource(resource_name, id).await
            }
            "controltower" => {
                self.controltower().delete_resource(resource_name, id).await
            }
            "arc_region_switch" => {
                self.arc_region_switch().delete_resource(resource_name, id).await
            }
            "neptune_graph" => {
                self.neptune_graph().delete_resource(resource_name, id).await
            }
            "route53_recovery_readiness" => {
                self.route53_recovery_readiness().delete_resource(resource_name, id).await
            }
            "greengrassv2" => {
                self.greengrassv2().delete_resource(resource_name, id).await
            }
            "migration_hub_refactor_spaces" => {
                self.migration_hub_refactor_spaces().delete_resource(resource_name, id).await
            }
            "cost_and_usage_report_service" => {
                self.cost_and_usage_report_service().delete_resource(resource_name, id).await
            }
            "ebs" => {
                self.ebs().delete_resource(resource_name, id).await
            }
            "appflow" => {
                self.appflow().delete_resource(resource_name, id).await
            }
            "migrationhuborchestrator" => {
                self.migrationhuborchestrator().delete_resource(resource_name, id).await
            }
            "chime_sdk_identity" => {
                self.chime_sdk_identity().delete_resource(resource_name, id).await
            }
            "cloudfront_keyvaluestore" => {
                self.cloudfront_keyvaluestore().delete_resource(resource_name, id).await
            }
            "waf" => {
                self.waf().delete_resource(resource_name, id).await
            }
            "greengrass" => {
                self.greengrass().delete_resource(resource_name, id).await
            }
            "sagemaker_featurestore_runtime" => {
                self.sagemaker_featurestore_runtime().delete_resource(resource_name, id).await
            }
            "inspector" => {
                self.inspector().delete_resource(resource_name, id).await
            }
            "appfabric" => {
                self.appfabric().delete_resource(resource_name, id).await
            }
            "lex_model_building_service" => {
                self.lex_model_building_service().delete_resource(resource_name, id).await
            }
            "serverlessapplicationrepository" => {
                self.serverlessapplicationrepository().delete_resource(resource_name, id).await
            }
            "cloudsearch_domain" => {
                self.cloudsearch_domain().delete_resource(resource_name, id).await
            }
            "codeguru_security" => {
                self.codeguru_security().delete_resource(resource_name, id).await
            }
            "socialmessaging" => {
                self.socialmessaging().delete_resource(resource_name, id).await
            }
            "geo_maps" => {
                self.geo_maps().delete_resource(resource_name, id).await
            }
            "kinesis" => {
                self.kinesis().delete_resource(resource_name, id).await
            }
            "dsql" => {
                self.dsql().delete_resource(resource_name, id).await
            }
            "appintegrations" => {
                self.appintegrations().delete_resource(resource_name, id).await
            }
            "personalize" => {
                self.personalize().delete_resource(resource_name, id).await
            }
            "proton" => {
                self.proton().delete_resource(resource_name, id).await
            }
            "cloudcontrol" => {
                self.cloudcontrol().delete_resource(resource_name, id).await
            }
            "redshift" => {
                self.redshift().delete_resource(resource_name, id).await
            }
            "geo_places" => {
                self.geo_places().delete_resource(resource_name, id).await
            }
            "elasticsearch_service" => {
                self.elasticsearch_service().delete_resource(resource_name, id).await
            }
            "bcm_recommended_actions" => {
                self.bcm_recommended_actions().delete_resource(resource_name, id).await
            }
            "invoicing" => {
                self.invoicing().delete_resource(resource_name, id).await
            }
            "apprunner" => {
                self.apprunner().delete_resource(resource_name, id).await
            }
            "sns" => {
                self.sns().delete_resource(resource_name, id).await
            }
            "textract" => {
                self.textract().delete_resource(resource_name, id).await
            }
            "workmail" => {
                self.workmail().delete_resource(resource_name, id).await
            }
            "datazone" => {
                self.datazone().delete_resource(resource_name, id).await
            }
            "rekognition" => {
                self.rekognition().delete_resource(resource_name, id).await
            }
            "ssm" => {
                self.ssm().delete_resource(resource_name, id).await
            }
            "medical_imaging" => {
                self.medical_imaging().delete_resource(resource_name, id).await
            }
            "lex_models" => {
                self.lex_models().delete_resource(resource_name, id).await
            }
            "support" => {
                self.support().delete_resource(resource_name, id).await
            }
            "signer" => {
                self.signer().delete_resource(resource_name, id).await
            }
            "partnercentral_selling" => {
                self.partnercentral_selling().delete_resource(resource_name, id).await
            }
            "comprehendmedical" => {
                self.comprehendmedical().delete_resource(resource_name, id).await
            }
            "macie2" => {
                self.macie2().delete_resource(resource_name, id).await
            }
            "redshift_data" => {
                self.redshift_data().delete_resource(resource_name, id).await
            }
            "marketplace_agreement" => {
                self.marketplace_agreement().delete_resource(resource_name, id).await
            }
            "health" => {
                self.health().delete_resource(resource_name, id).await
            }
            "odb" => {
                self.odb().delete_resource(resource_name, id).await
            }
            "resource_groups_tagging_api" => {
                self.resource_groups_tagging_api().delete_resource(resource_name, id).await
            }
            "application_insights" => {
                self.application_insights().delete_resource(resource_name, id).await
            }
            "timestream_write" => {
                self.timestream_write().delete_resource(resource_name, id).await
            }
            "pinpoint_sms" => {
                self.pinpoint_sms().delete_resource(resource_name, id).await
            }
            "mediapackagev2" => {
                self.mediapackagev2().delete_resource(resource_name, id).await
            }
            "ec2" => {
                self.ec2().delete_resource(resource_name, id).await
            }
            "cleanrooms" => {
                self.cleanrooms().delete_resource(resource_name, id).await
            }
            "healthlake" => {
                self.healthlake().delete_resource(resource_name, id).await
            }
            "sfn" => {
                self.sfn().delete_resource(resource_name, id).await
            }
            "iottwinmaker" => {
                self.iottwinmaker().delete_resource(resource_name, id).await
            }
            "cloudtrail" => {
                self.cloudtrail().delete_resource(resource_name, id).await
            }
            "iotdeviceadvisor" => {
                self.iotdeviceadvisor().delete_resource(resource_name, id).await
            }
            "ssm_incidents" => {
                self.ssm_incidents().delete_resource(resource_name, id).await
            }
            "pcs" => {
                self.pcs().delete_resource(resource_name, id).await
            }
            "support_app" => {
                self.support_app().delete_resource(resource_name, id).await
            }
            "managedblockchain_query" => {
                self.managedblockchain_query().delete_resource(resource_name, id).await
            }
            "iot_events_data" => {
                self.iot_events_data().delete_resource(resource_name, id).await
            }
            "lex_runtime" => {
                self.lex_runtime().delete_resource(resource_name, id).await
            }
            "observabilityadmin" => {
                self.observabilityadmin().delete_resource(resource_name, id).await
            }
            "applicationcostprofiler" => {
                self.applicationcostprofiler().delete_resource(resource_name, id).await
            }
            "billingconductor" => {
                self.billingconductor().delete_resource(resource_name, id).await
            }
            "artifact" => {
                self.artifact().delete_resource(resource_name, id).await
            }
            "ecr_public" => {
                self.ecr_public().delete_resource(resource_name, id).await
            }
            "connectparticipant" => {
                self.connectparticipant().delete_resource(resource_name, id).await
            }
            "rds_data" => {
                self.rds_data().delete_resource(resource_name, id).await
            }
            "internetmonitor" => {
                self.internetmonitor().delete_resource(resource_name, id).await
            }
            "route_53" => {
                self.route_53().delete_resource(resource_name, id).await
            }
            "bedrock_runtime" => {
                self.bedrock_runtime().delete_resource(resource_name, id).await
            }
            "amplifybackend" => {
                self.amplifybackend().delete_resource(resource_name, id).await
            }
            "marketplace_deployment" => {
                self.marketplace_deployment().delete_resource(resource_name, id).await
            }
            "account" => {
                self.account().delete_resource(resource_name, id).await
            }
            "snowball" => {
                self.snowball().delete_resource(resource_name, id).await
            }
            "eventbridge" => {
                self.eventbridge().delete_resource(resource_name, id).await
            }
            "auto_scaling_plans" => {
                self.auto_scaling_plans().delete_resource(resource_name, id).await
            }
            "directory_service" => {
                self.directory_service().delete_resource(resource_name, id).await
            }
            "mediapackage" => {
                self.mediapackage().delete_resource(resource_name, id).await
            }
            "ssm_quicksetup" => {
                self.ssm_quicksetup().delete_resource(resource_name, id).await
            }
            "s3_control" => {
                self.s3_control().delete_resource(resource_name, id).await
            }
            "codecatalyst" => {
                self.codecatalyst().delete_resource(resource_name, id).await
            }
            "notificationscontacts" => {
                self.notificationscontacts().delete_resource(resource_name, id).await
            }
            "mpa" => {
                self.mpa().delete_resource(resource_name, id).await
            }
            "ec2_instance_connect" => {
                self.ec2_instance_connect().delete_resource(resource_name, id).await
            }
            "sagemaker_geospatial" => {
                self.sagemaker_geospatial().delete_resource(resource_name, id).await
            }
            "notifications" => {
                self.notifications().delete_resource(resource_name, id).await
            }
            "securitylake" => {
                self.securitylake().delete_resource(resource_name, id).await
            }
            "networkmonitor" => {
                self.networkmonitor().delete_resource(resource_name, id).await
            }
            "codeconnections" => {
                self.codeconnections().delete_resource(resource_name, id).await
            }
            "app_mesh" => {
                self.app_mesh().delete_resource(resource_name, id).await
            }
            "workspaces_thin_client" => {
                self.workspaces_thin_client().delete_resource(resource_name, id).await
            }
            "finspace_data" => {
                self.finspace_data().delete_resource(resource_name, id).await
            }
            "compute_optimizer" => {
                self.compute_optimizer().delete_resource(resource_name, id).await
            }
            "secrets_manager" => {
                self.secrets_manager().delete_resource(resource_name, id).await
            }
            "mediastore" => {
                self.mediastore().delete_resource(resource_name, id).await
            }
            "ecs" => {
                self.ecs().delete_resource(resource_name, id).await
            }
            "vpc_lattice" => {
                self.vpc_lattice().delete_resource(resource_name, id).await
            }
            "auto_scaling" => {
                self.auto_scaling().delete_resource(resource_name, id).await
            }
            "resource_groups" => {
                self.resource_groups().delete_resource(resource_name, id).await
            }
            "eks" => {
                self.eks().delete_resource(resource_name, id).await
            }
            "marketplace_entitlement_service" => {
                self.marketplace_entitlement_service().delete_resource(resource_name, id).await
            }
            "database_migration_service" => {
                self.database_migration_service().delete_resource(resource_name, id).await
            }
            "security_ir" => {
                self.security_ir().delete_resource(resource_name, id).await
            }
            "inspector_scan" => {
                self.inspector_scan().delete_resource(resource_name, id).await
            }
            "global_accelerator" => {
                self.global_accelerator().delete_resource(resource_name, id).await
            }
            "kinesis_analytics" => {
                self.kinesis_analytics().delete_resource(resource_name, id).await
            }
            "neptunedata" => {
                self.neptunedata().delete_resource(resource_name, id).await
            }
            "swf" => {
                self.swf().delete_resource(resource_name, id).await
            }
            "cloudwatch_logs" => {
                self.cloudwatch_logs().delete_resource(resource_name, id).await
            }
            "connect" => {
                self.connect().delete_resource(resource_name, id).await
            }
            "glue" => {
                self.glue().delete_resource(resource_name, id).await
            }
            "cognito_identity_provider" => {
                self.cognito_identity_provider().delete_resource(resource_name, id).await
            }
            "cloudwatch_events" => {
                self.cloudwatch_events().delete_resource(resource_name, id).await
            }
            "cost_explorer" => {
                self.cost_explorer().delete_resource(resource_name, id).await
            }
            "network_firewall" => {
                self.network_firewall().delete_resource(resource_name, id).await
            }
            "firehose" => {
                self.firehose().delete_resource(resource_name, id).await
            }
            "transfer" => {
                self.transfer().delete_resource(resource_name, id).await
            }
            "marketplace_metering" => {
                self.marketplace_metering().delete_resource(resource_name, id).await
            }
            "rbin" => {
                self.rbin().delete_resource(resource_name, id).await
            }
            "timestream_influxdb" => {
                self.timestream_influxdb().delete_resource(resource_name, id).await
            }
            "iotanalytics" => {
                self.iotanalytics().delete_resource(resource_name, id).await
            }
            "ivs" => {
                self.ivs().delete_resource(resource_name, id).await
            }
            "kafka" => {
                self.kafka().delete_resource(resource_name, id).await
            }
            "sesv2" => {
                self.sesv2().delete_resource(resource_name, id).await
            }
            "kendra" => {
                self.kendra().delete_resource(resource_name, id).await
            }
            "sagemaker_edge" => {
                self.sagemaker_edge().delete_resource(resource_name, id).await
            }
            "launch_wizard" => {
                self.launch_wizard().delete_resource(resource_name, id).await
            }
            "securityhub" => {
                self.securityhub().delete_resource(resource_name, id).await
            }
            "finspace" => {
                self.finspace().delete_resource(resource_name, id).await
            }
            "keyspacesstreams" => {
                self.keyspacesstreams().delete_resource(resource_name, id).await
            }
            "cleanroomsml" => {
                self.cleanroomsml().delete_resource(resource_name, id).await
            }
            "transcribe_streaming" => {
                self.transcribe_streaming().delete_resource(resource_name, id).await
            }
            "aiops" => {
                self.aiops().delete_resource(resource_name, id).await
            }
            "service_catalog" => {
                self.service_catalog().delete_resource(resource_name, id).await
            }
            "databrew" => {
                self.databrew().delete_resource(resource_name, id).await
            }
            "codecommit" => {
                self.codecommit().delete_resource(resource_name, id).await
            }
            "resource_explorer_2" => {
                self.resource_explorer_2().delete_resource(resource_name, id).await
            }
            "acm_pca" => {
                self.acm_pca().delete_resource(resource_name, id).await
            }
            "payment_cryptography" => {
                self.payment_cryptography().delete_resource(resource_name, id).await
            }
            "mq" => {
                self.mq().delete_resource(resource_name, id).await
            }
            "api_gateway" => {
                self.api_gateway().delete_resource(resource_name, id).await
            }
            "grafana" => {
                self.grafana().delete_resource(resource_name, id).await
            }
            "glacier" => {
                self.glacier().delete_resource(resource_name, id).await
            }
            "bedrock" => {
                self.bedrock().delete_resource(resource_name, id).await
            }
            "s3tables" => {
                self.s3tables().delete_resource(resource_name, id).await
            }
            "ivs_realtime" => {
                self.ivs_realtime().delete_resource(resource_name, id).await
            }
            "medialive" => {
                self.medialive().delete_resource(resource_name, id).await
            }
            "backupsearch" => {
                self.backupsearch().delete_resource(resource_name, id).await
            }
            "networkflowmonitor" => {
                self.networkflowmonitor().delete_resource(resource_name, id).await
            }
            "elasticache" => {
                self.elasticache().delete_resource(resource_name, id).await
            }
            "fis" => {
                self.fis().delete_resource(resource_name, id).await
            }
            "cloudhsm" => {
                self.cloudhsm().delete_resource(resource_name, id).await
            }
            "cost_optimization_hub" => {
                self.cost_optimization_hub().delete_resource(resource_name, id).await
            }
            "synthetics" => {
                self.synthetics().delete_resource(resource_name, id).await
            }
            "rum" => {
                self.rum().delete_resource(resource_name, id).await
            }
            "emr_containers" => {
                self.emr_containers().delete_resource(resource_name, id).await
            }
            "sagemaker_a2i_runtime" => {
                self.sagemaker_a2i_runtime().delete_resource(resource_name, id).await
            }
            "ssm_contacts" => {
                self.ssm_contacts().delete_resource(resource_name, id).await
            }
            "bcm_data_exports" => {
                self.bcm_data_exports().delete_resource(resource_name, id).await
            }
            "opensearch" => {
                self.opensearch().delete_resource(resource_name, id).await
            }
            "dax" => {
                self.dax().delete_resource(resource_name, id).await
            }
            "neptune" => {
                self.neptune().delete_resource(resource_name, id).await
            }
            "pricing" => {
                self.pricing().delete_resource(resource_name, id).await
            }
            "location" => {
                self.location().delete_resource(resource_name, id).await
            }
            "route53profiles" => {
                self.route53profiles().delete_resource(resource_name, id).await
            }
            "lambda" => {
                self.lambda().delete_resource(resource_name, id).await
            }
            "ivschat" => {
                self.ivschat().delete_resource(resource_name, id).await
            }
            "billing" => {
                self.billing().delete_resource(resource_name, id).await
            }
            "wisdom" => {
                self.wisdom().delete_resource(resource_name, id).await
            }
            "schemas" => {
                self.schemas().delete_resource(resource_name, id).await
            }
            "bedrock_agentcore_control" => {
                self.bedrock_agentcore_control().delete_resource(resource_name, id).await
            }
            "controlcatalog" => {
                self.controlcatalog().delete_resource(resource_name, id).await
            }
            "cloudsearch" => {
                self.cloudsearch().delete_resource(resource_name, id).await
            }
            "deadline" => {
                self.deadline().delete_resource(resource_name, id).await
            }
            "managedblockchain" => {
                self.managedblockchain().delete_resource(resource_name, id).await
            }
            "amplify" => {
                self.amplify().delete_resource(resource_name, id).await
            }
            "iotsecuretunneling" => {
                self.iotsecuretunneling().delete_resource(resource_name, id).await
            }
            "connectcampaigns" => {
                self.connectcampaigns().delete_resource(resource_name, id).await
            }
            "kafkaconnect" => {
                self.kafkaconnect().delete_resource(resource_name, id).await
            }
            "mediaconvert" => {
                self.mediaconvert().delete_resource(resource_name, id).await
            }
            "data_pipeline" => {
                self.data_pipeline().delete_resource(resource_name, id).await
            }
            "codepipeline" => {
                self.codepipeline().delete_resource(resource_name, id).await
            }
            "clouddirectory" => {
                self.clouddirectory().delete_resource(resource_name, id).await
            }
            "amplifyuibuilder" => {
                self.amplifyuibuilder().delete_resource(resource_name, id).await
            }
            "rtbfabric" => {
                self.rtbfabric().delete_resource(resource_name, id).await
            }
            "memorydb" => {
                self.memorydb().delete_resource(resource_name, id).await
            }
            "iot" => {
                self.iot().delete_resource(resource_name, id).await
            }
            "marketplace_commerce_analytics" => {
                self.marketplace_commerce_analytics().delete_resource(resource_name, id).await
            }
            "frauddetector" => {
                self.frauddetector().delete_resource(resource_name, id).await
            }
            "bedrock_data_automation" => {
                self.bedrock_data_automation().delete_resource(resource_name, id).await
            }
            "elastic_load_balancing" => {
                self.elastic_load_balancing().delete_resource(resource_name, id).await
            }
            "verifiedpermissions" => {
                self.verifiedpermissions().delete_resource(resource_name, id).await
            }
            "networkmanager" => {
                self.networkmanager().delete_resource(resource_name, id).await
            }
            "devops_guru" => {
                self.devops_guru().delete_resource(resource_name, id).await
            }
            "taxsettings" => {
                self.taxsettings().delete_resource(resource_name, id).await
            }
            "workspaces_instances" => {
                self.workspaces_instances().delete_resource(resource_name, id).await
            }
            "arc_zonal_shift" => {
                self.arc_zonal_shift().delete_resource(resource_name, id).await
            }
            "elastic_transcoder" => {
                self.elastic_transcoder().delete_resource(resource_name, id).await
            }
            "fms" => {
                self.fms().delete_resource(resource_name, id).await
            }
            "imagebuilder" => {
                self.imagebuilder().delete_resource(resource_name, id).await
            }
            "chime_sdk" => {
                self.chime_sdk().delete_resource(resource_name, id).await
            }
            "groundstation" => {
                self.groundstation().delete_resource(resource_name, id).await
            }
            "forecast" => {
                self.forecast().delete_resource(resource_name, id).await
            }
            "appstream" => {
                self.appstream().delete_resource(resource_name, id).await
            }
            "chime_sdk_meetings" => {
                self.chime_sdk_meetings().delete_resource(resource_name, id).await
            }
            "comprehend" => {
                self.comprehend().delete_resource(resource_name, id).await
            }
            "redshift_serverless" => {
                self.redshift_serverless().delete_resource(resource_name, id).await
            }
            "pinpoint" => {
                self.pinpoint().delete_resource(resource_name, id).await
            }
            "pi" => {
                self.pi().delete_resource(resource_name, id).await
            }
            "gameliftstreams" => {
                self.gameliftstreams().delete_resource(resource_name, id).await
            }
            "customer_profiles" => {
                self.customer_profiles().delete_resource(resource_name, id).await
            }
            "workspaces" => {
                self.workspaces().delete_resource(resource_name, id).await
            }
            "auditmanager" => {
                self.auditmanager().delete_resource(resource_name, id).await
            }
            "docdb" => {
                self.docdb().delete_resource(resource_name, id).await
            }
            "mturk" => {
                self.mturk().delete_resource(resource_name, id).await
            }
            "cognito_identity" => {
                self.cognito_identity().delete_resource(resource_name, id).await
            }
            "dynamodb" => {
                self.dynamodb().delete_resource(resource_name, id).await
            }
            "codeartifact" => {
                self.codeartifact().delete_resource(resource_name, id).await
            }
            "organizations" => {
                self.organizations().delete_resource(resource_name, id).await
            }
            "dlm" => {
                self.dlm().delete_resource(resource_name, id).await
            }
            "sso" => {
                self.sso().delete_resource(resource_name, id).await
            }
            "osis" => {
                self.osis().delete_resource(resource_name, id).await
            }
            "migration_hub" => {
                self.migration_hub().delete_resource(resource_name, id).await
            }
            "chatbot" => {
                self.chatbot().delete_resource(resource_name, id).await
            }
            "docdb_elastic" => {
                self.docdb_elastic().delete_resource(resource_name, id).await
            }
            "supplychain" => {
                self.supplychain().delete_resource(resource_name, id).await
            }
            "ses" => {
                self.ses().delete_resource(resource_name, id).await
            }
            "repostspace" => {
                self.repostspace().delete_resource(resource_name, id).await
            }
            "mediastore_data" => {
                self.mediastore_data().delete_resource(resource_name, id).await
            }
            "bedrock_agent" => {
                self.bedrock_agent().delete_resource(resource_name, id).await
            }
            "wellarchitected" => {
                self.wellarchitected().delete_resource(resource_name, id).await
            }
            "budgets" => {
                self.budgets().delete_resource(resource_name, id).await
            }
            "mediatailor" => {
                self.mediatailor().delete_resource(resource_name, id).await
            }
            "appsync" => {
                self.appsync().delete_resource(resource_name, id).await
            }
            "ssm_guiconnect" => {
                self.ssm_guiconnect().delete_resource(resource_name, id).await
            }
            "evs" => {
                self.evs().delete_resource(resource_name, id).await
            }
            "eks_auth" => {
                self.eks_auth().delete_resource(resource_name, id).await
            }
            "chime_sdk_messaging" => {
                self.chime_sdk_messaging().delete_resource(resource_name, id).await
            }
            "mediaconnect" => {
                self.mediaconnect().delete_resource(resource_name, id).await
            }
            "identitystore" => {
                self.identitystore().delete_resource(resource_name, id).await
            }
            "bcm_pricing_calculator" => {
                self.bcm_pricing_calculator().delete_resource(resource_name, id).await
            }
            "lakeformation" => {
                self.lakeformation().delete_resource(resource_name, id).await
            }
            "xray" => {
                self.xray().delete_resource(resource_name, id).await
            }
            "cloudfront" => {
                self.cloudfront().delete_resource(resource_name, id).await
            }
            "sagemaker_metrics" => {
                self.sagemaker_metrics().delete_resource(resource_name, id).await
            }
            "sso_oidc" => {
                self.sso_oidc().delete_resource(resource_name, id).await
            }
            "sagemaker" => {
                self.sagemaker().delete_resource(resource_name, id).await
            }
            "codestar_connections" => {
                self.codestar_connections().delete_resource(resource_name, id).await
            }
            "device_farm" => {
                self.device_farm().delete_resource(resource_name, id).await
            }
            "translate" => {
                self.translate().delete_resource(resource_name, id).await
            }
            "sagemaker_runtime" => {
                self.sagemaker_runtime().delete_resource(resource_name, id).await
            }
            "b2bi" => {
                self.b2bi().delete_resource(resource_name, id).await
            }
            "savingsplans" => {
                self.savingsplans().delete_resource(resource_name, id).await
            }
            "pipes" => {
                self.pipes().delete_resource(resource_name, id).await
            }
            "config_service" => {
                self.config_service().delete_resource(resource_name, id).await
            }
            "codeguruprofiler" => {
                self.codeguruprofiler().delete_resource(resource_name, id).await
            }
            "s3" => {
                self.s3().delete_resource(resource_name, id).await
            }
            "polly" => {
                self.polly().delete_resource(resource_name, id).await
            }
            "cognito_sync" => {
                self.cognito_sync().delete_resource(resource_name, id).await
            }
            "scheduler" => {
                self.scheduler().delete_resource(resource_name, id).await
            }
            "pca_connector_ad" => {
                self.pca_connector_ad().delete_resource(resource_name, id).await
            }
            "waf_regional" => {
                self.waf_regional().delete_resource(resource_name, id).await
            }
            "apigatewaymanagementapi" => {
                self.apigatewaymanagementapi().delete_resource(resource_name, id).await
            }
            "workspaces_web" => {
                self.workspaces_web().delete_resource(resource_name, id).await
            }
            "pca_connector_scep" => {
                self.pca_connector_scep().delete_resource(resource_name, id).await
            }
            "codestar_notifications" => {
                self.codestar_notifications().delete_resource(resource_name, id).await
            }
            "direct_connect" => {
                self.direct_connect().delete_resource(resource_name, id).await
            }
            "shield" => {
                self.shield().delete_resource(resource_name, id).await
            }
            "application_signals" => {
                self.application_signals().delete_resource(resource_name, id).await
            }
            "iot_managed_integrations" => {
                self.iot_managed_integrations().delete_resource(resource_name, id).await
            }
            "iot_wireless" => {
                self.iot_wireless().delete_resource(resource_name, id).await
            }
            "iot_events" => {
                self.iot_events().delete_resource(resource_name, id).await
            }
            "backup_gateway" => {
                self.backup_gateway().delete_resource(resource_name, id).await
            }
            "sso_admin" => {
                self.sso_admin().delete_resource(resource_name, id).await
            }
            "elastic_beanstalk" => {
                self.elastic_beanstalk().delete_resource(resource_name, id).await
            }
            "drs" => {
                self.drs().delete_resource(resource_name, id).await
            }
            "personalize_runtime" => {
                self.personalize_runtime().delete_resource(resource_name, id).await
            }
            "outposts" => {
                self.outposts().delete_resource(resource_name, id).await
            }
            "license_manager_user_subscriptions" => {
                self.license_manager_user_subscriptions().delete_resource(resource_name, id).await
            }
            "cloudtrail_data" => {
                self.cloudtrail_data().delete_resource(resource_name, id).await
            }
            "lex_runtime_service" => {
                self.lex_runtime_service().delete_resource(resource_name, id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }
}

/// Factory function to create a provider instance
///
/// This is the entry point called by Hemmer when loading the provider as a dynamic library.
#[no_mangle]
pub extern "C" fn create_provider() -> *mut dyn ProviderExecutor {
    match AwsProvider::new() {
        Ok(provider) => Box::into_raw(Box::new(provider)) as *mut dyn ProviderExecutor,
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        // Provider creation test
        // Note: This will fail without proper credentials
        // let provider = AwsProvider::new();
        // assert!(provider.is_ok());
    }
}
