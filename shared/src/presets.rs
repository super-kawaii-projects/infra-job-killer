use crate::models::*;

pub fn make_production_ready_eks(config: &mut EksConfig) {
    config.multi_az = true;
    config.private_nodes = true;
    config.enable_nat = true;
    config.enable_vpc_endpoints = true;
    config.enable_prometheus = true;
    config.enable_grafana = true;
    config.enable_external_secrets = true;
    config.enable_cert_manager = true;
    config.enable_pod_identity = true;
    config.enable_network_policies = true;
    config.private_api_endpoint = true;
    config.enable_secrets_encryption = true;
    config.enable_nth = true;
    config.enable_pdb = true;
    config.enable_topology_spread = true;
    config.enable_descheduler = true;
    if config.min_nodes < 3 { config.min_nodes = 3; }
}

pub fn make_production_ready_vpc(config: &mut VpcConfig) {
    config.az_count = 3;
    config.single_nat_gateway = false;
    config.enable_flow_logs = true;
    config.enable_vpc_endpoints = true;
}

pub fn make_production_ready_ec2(config: &mut Ec2Config) {
    config.enable_monitoring = true;
    config.enable_termination_protection = true;
    config.associate_public_ip = false;
    config.subnet_placement = SubnetPlacement::Private;
}
