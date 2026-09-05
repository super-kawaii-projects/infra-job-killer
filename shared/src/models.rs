use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildType { Vpc, Ec2, Ebs, Eks }

impl std::fmt::Display for BuildType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildType::Vpc => write!(f, "VPC"),
            BuildType::Ec2 => write!(f, "EC2"),
            BuildType::Ebs => write!(f, "EBS"),
            BuildType::Eks => write!(f, "EKS"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub build_type: BuildType,
    pub environment: Environment,
    pub region: String,
    pub production_ready: bool,
    pub config: BuildConfig,
    pub status: BuildStatus,
    pub cost_estimate: Option<CostEstimate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Environment { Dev, Staging, Production }

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Dev => write!(f, "dev"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuildStatus { Draft, Planning, Planned, Building, Built, Failed, Destroying, Destroyed }

impl std::fmt::Display for BuildStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildStatus::Draft => write!(f, "Draft"),
            BuildStatus::Planning => write!(f, "Planning..."),
            BuildStatus::Planned => write!(f, "Plan Ready"),
            BuildStatus::Building => write!(f, "Building..."),
            BuildStatus::Built => write!(f, "Built"),
            BuildStatus::Failed => write!(f, "Failed"),
            BuildStatus::Destroying => write!(f, "Destroying..."),
            BuildStatus::Destroyed => write!(f, "Destroyed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildConfig {
    Vpc(VpcConfig),
    Ec2(Ec2Config),
    Ebs(EbsConfig),
    Eks(EksConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcConfig {
    pub vpc_cidr: String,
    pub az_count: u8,
    pub enable_nat_gateway: bool,
    pub single_nat_gateway: bool,
    pub enable_vpn_gateway: bool,
    pub enable_flow_logs: bool,
    pub enable_vpc_endpoints: bool,
    pub private_subnets: Vec<String>,
    pub public_subnets: Vec<String>,
    pub tags: HashMap<String, String>,
}

impl Default for VpcConfig {
    fn default() -> Self {
        Self {
            vpc_cidr: "10.0.0.0/16".to_string(),
            az_count: 3,
            enable_nat_gateway: true,
            single_nat_gateway: false,
            enable_vpn_gateway: false,
            enable_flow_logs: true,
            enable_vpc_endpoints: false,
            private_subnets: vec!["10.0.1.0/24".into(), "10.0.2.0/24".into(), "10.0.3.0/24".into()],
            public_subnets: vec!["10.0.101.0/24".into(), "10.0.102.0/24".into(), "10.0.103.0/24".into()],
            tags: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ec2Config {
    pub instance_type: String,
    pub instance_count: u32,
    pub volume_size_gb: u32,
    pub volume_type: String,
    pub associate_public_ip: bool,
    pub enable_monitoring: bool,
    pub enable_termination_protection: bool,
    pub key_pair_name: String,
    pub subnet_placement: SubnetPlacement,
    pub tags: HashMap<String, String>,
}

impl Default for Ec2Config {
    fn default() -> Self {
        Self {
            instance_type: "t3.medium".to_string(),
            instance_count: 1, volume_size_gb: 30,
            volume_type: "gp3".to_string(),
            associate_public_ip: false,
            enable_monitoring: true,
            enable_termination_protection: false,
            key_pair_name: String::new(),
            subnet_placement: SubnetPlacement::Private,
            tags: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubnetPlacement { Public, Private }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbsConfig {
    pub volume_size_gb: u32,
    pub volume_type: String,
    pub iops: Option<u32>,
    pub throughput: Option<u32>,
    pub encrypted: bool,
    pub multi_attach: bool,
    pub tags: HashMap<String, String>,
}

impl Default for EbsConfig {
    fn default() -> Self {
        Self {
            volume_size_gb: 100, volume_type: "gp3".to_string(),
            iops: Some(3000), throughput: Some(125),
            encrypted: true, multi_attach: false, tags: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EksConfig {
    pub cluster_name: String,
    pub cluster_version: String,
    pub multi_az: bool,
    pub private_nodes: bool,
    pub enable_nat: bool,
    pub enable_vpc_endpoints: bool,
    pub enable_alb_controller: bool,
    pub vpc_cidr: String,
    pub compute_type: EksComputeType,
    pub instance_type: String,
    pub min_nodes: u32,
    pub max_nodes: u32,
    pub desired_nodes: u32,
    pub enable_argocd: bool,
    pub enable_prometheus: bool,
    pub enable_grafana: bool,
    pub enable_external_secrets: bool,
    pub enable_cert_manager: bool,
    pub enable_external_dns: bool,
    pub enable_istio: bool,
    pub enable_pod_identity: bool,
    pub enable_network_policies: bool,
    pub private_api_endpoint: bool,
    pub enable_secrets_encryption: bool,
    pub enable_keda: bool,
    pub enable_descheduler: bool,
    pub enable_crane: bool,
    pub enable_nth: bool,
    pub enable_pdb: bool,
    pub enable_topology_spread: bool,
    pub tags: HashMap<String, String>,
}

impl Default for EksConfig {
    fn default() -> Self {
        Self {
            cluster_name: String::new(), cluster_version: "1.30".to_string(),
            multi_az: true, private_nodes: true, enable_nat: true,
            enable_vpc_endpoints: false, enable_alb_controller: true,
            vpc_cidr: "10.0.0.0/16".to_string(),
            compute_type: EksComputeType::Karpenter,
            instance_type: "m5.xlarge".to_string(),
            min_nodes: 2, max_nodes: 10, desired_nodes: 3,
            enable_argocd: false, enable_prometheus: true, enable_grafana: true,
            enable_external_secrets: true, enable_cert_manager: true,
            enable_external_dns: false, enable_istio: false,
            enable_pod_identity: true, enable_network_policies: true,
            private_api_endpoint: false, enable_secrets_encryption: true,
            enable_keda: false, enable_descheduler: false,
            enable_crane: false, enable_nth: true,
            enable_pdb: true, enable_topology_spread: true,
            tags: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EksComputeType { ManagedNodeGroups, Karpenter }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostEstimate {
    pub monthly_total: f64,
    pub line_items: Vec<CostLineItem>,
    pub production_addon_cost: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostLineItem {
    pub service: String,
    pub resource: String,
    pub description: String,
    pub monthly_cost: f64,
    pub is_production_addon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub plan_summary: Option<PlanSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSummary {
    pub additions: u32,
    pub changes: u32,
    pub destructions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerraformAction { Plan, Apply, Destroy }
