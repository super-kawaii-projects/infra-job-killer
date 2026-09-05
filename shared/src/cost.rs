use crate::models::*;

/// Format a dollar amount with comma thousands separators, no decimals.
/// e.g. 1234567.0 -> "1,234,567"
pub fn format_money(amount: f64) -> String {
    let rounded = amount.round() as i64;
    let negative = rounded < 0;
    let digits = rounded.abs().to_string();
    let mut out = String::new();
    let len = digits.len();
    for (idx, ch) in digits.chars().enumerate() {
        if idx > 0 && (len - idx) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    if negative {
        format!("-{}", out)
    } else {
        out
    }
}

pub fn estimate_vpc_cost(config: &VpcConfig, _region: &str) -> CostEstimate {
    let mut items = Vec::new();
    if config.enable_nat_gateway {
        let nat_count = if config.single_nat_gateway { 1 } else { config.az_count as u32 };
        items.push(CostLineItem {
            service: "VPC".into(), resource: "NAT Gateway".into(),
            description: format!("{} NAT(s) × $0.045/hr", nat_count),
            monthly_cost: 0.045 * 730.0 * nat_count as f64,
            is_production_addon: !config.single_nat_gateway && nat_count > 1,
        });
    }
    if config.enable_flow_logs {
        items.push(CostLineItem {
            service: "CloudWatch".into(), resource: "Flow Logs".into(),
            description: "~10GB/mo ingestion".into(), monthly_cost: 5.0, is_production_addon: true,
        });
    }
    if config.enable_vpc_endpoints {
        let cost = 0.01 * 730.0 * 4.0 * config.az_count as f64;
        items.push(CostLineItem {
            service: "VPC".into(), resource: "Endpoints".into(),
            description: format!("4 endpoints × {} AZs", config.az_count),
            monthly_cost: cost, is_production_addon: true,
        });
    }
    let total: f64 = items.iter().map(|i| i.monthly_cost).sum();
    let prod: f64 = items.iter().filter(|i| i.is_production_addon).map(|i| i.monthly_cost).sum();
    CostEstimate { monthly_total: total, line_items: items, production_addon_cost: prod, currency: "USD".into() }
}

pub fn estimate_ec2_cost(config: &Ec2Config, _region: &str) -> CostEstimate {
    let mut items = Vec::new();
    let hourly = match config.instance_type.as_str() {
        "t3.micro" => 0.0104, "t3.small" => 0.0208, "t3.medium" => 0.0416,
        "t3.large" => 0.0832, "m5.large" => 0.096, "m5.xlarge" => 0.192,
        "c5.large" => 0.085, "c5.xlarge" => 0.170, "r5.large" => 0.126, _ => 0.10,
    };
    items.push(CostLineItem {
        service: "EC2".into(), resource: "Instance".into(),
        description: format!("{} × {} @ ${:.4}/hr", config.instance_count, config.instance_type, hourly),
        monthly_cost: hourly * 730.0 * config.instance_count as f64, is_production_addon: false,
    });
    let ebs_rate = match config.volume_type.as_str() { "gp3" => 0.08, "gp2" => 0.10, "io2" => 0.125, _ => 0.08 };
    items.push(CostLineItem {
        service: "EBS".into(), resource: "Volume".into(),
        description: format!("{}GB {}", config.volume_size_gb, config.volume_type),
        monthly_cost: ebs_rate * config.volume_size_gb as f64 * config.instance_count as f64, is_production_addon: false,
    });
    if config.enable_monitoring {
        items.push(CostLineItem {
            service: "CloudWatch".into(), resource: "Monitoring".into(),
            description: "Detailed 1-min metrics".into(),
            monthly_cost: 3.50 * config.instance_count as f64, is_production_addon: true,
        });
    }
    let total: f64 = items.iter().map(|i| i.monthly_cost).sum();
    let prod: f64 = items.iter().filter(|i| i.is_production_addon).map(|i| i.monthly_cost).sum();
    CostEstimate { monthly_total: total, line_items: items, production_addon_cost: prod, currency: "USD".into() }
}

pub fn estimate_ebs_cost(config: &EbsConfig, _region: &str) -> CostEstimate {
    let mut items = Vec::new();
    let rate = match config.volume_type.as_str() { "gp3" => 0.08, "gp2" => 0.10, "io2" => 0.125, "st1" => 0.045, _ => 0.08 };
    items.push(CostLineItem {
        service: "EBS".into(), resource: "Storage".into(),
        description: format!("{}GB {} @ ${}/GB", config.volume_size_gb, config.volume_type, rate),
        monthly_cost: rate * config.volume_size_gb as f64, is_production_addon: false,
    });
    if let Some(iops) = config.iops {
        if config.volume_type == "gp3" && iops > 3000 {
            items.push(CostLineItem {
                service: "EBS".into(), resource: "IOPS".into(),
                description: format!("{} extra IOPS", iops - 3000),
                monthly_cost: (iops - 3000) as f64 * 0.005, is_production_addon: false,
            });
        }
    }
    let total: f64 = items.iter().map(|i| i.monthly_cost).sum();
    CostEstimate { monthly_total: total, line_items: items, production_addon_cost: 0.0, currency: "USD".into() }
}

pub fn estimate_eks_cost(config: &EksConfig, _region: &str) -> CostEstimate {
    let mut items = Vec::new();
    items.push(CostLineItem {
        service: "EKS".into(), resource: "Control Plane".into(),
        description: "$0.10/hr".into(), monthly_cost: 73.0, is_production_addon: false,
    });
    let hourly = match config.instance_type.as_str() {
        "t3.large" => 0.0832, "m5.large" => 0.096, "m5.xlarge" => 0.192,
        "m5.2xlarge" => 0.384, "c5.xlarge" => 0.170, _ => 0.192,
    };
    items.push(CostLineItem {
        service: "EC2".into(), resource: "Worker Nodes".into(),
        description: format!("{} × {}", config.desired_nodes, config.instance_type),
        monthly_cost: hourly * 730.0 * config.desired_nodes as f64, is_production_addon: false,
    });
    if config.enable_nat {
        let nat_count = if config.multi_az { 3 } else { 1 };
        items.push(CostLineItem {
            service: "VPC".into(), resource: "NAT Gateway".into(),
            description: format!("{} NATs", nat_count),
            monthly_cost: 0.045 * 730.0 * nat_count as f64, is_production_addon: config.multi_az,
        });
    }
    if config.enable_vpc_endpoints {
        items.push(CostLineItem {
            service: "VPC".into(), resource: "Endpoints".into(),
            description: "4 endpoints × 3 AZs".into(), monthly_cost: 87.60, is_production_addon: true,
        });
    }
    if config.enable_alb_controller {
        items.push(CostLineItem {
            service: "ELB".into(), resource: "ALB".into(),
            description: "Base + LCU".into(), monthly_cost: 22.0, is_production_addon: false,
        });
    }
    let total: f64 = items.iter().map(|i| i.monthly_cost).sum();
    let prod: f64 = items.iter().filter(|i| i.is_production_addon).map(|i| i.monthly_cost).sum();
    CostEstimate { monthly_total: total, line_items: items, production_addon_cost: prod, currency: "USD".into() }
}
