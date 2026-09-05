use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhyExplanation {
    pub decision: String,
    pub reason: String,
    pub additional_cost: f64,
    pub risk_level: String,
}

pub fn get_explanations_for_build_type(build_type: &str) -> Vec<WhyExplanation> {
    match build_type {
        "eks" => vec![
            why("Multi-AZ deployment", "Survive AZ failure. AWS AZs fail independently.", 0.0, "Critical"),
            why("Private networking", "Nodes unreachable from internet. Only LB is public.", 32.0, "High"),
            why("Encryption at rest", "All data encrypted. Required for SOC2/HIPAA/PCI.", 0.0, "High"),
            why("Pod Disruption Budgets", "Prevent evicting too many pods during rollouts.", 0.0, "High"),
            why("Topology spread", "Pods spread across AZs and nodes.", 0.0, "High"),
            why("Monitoring (Prometheus)", "Full observability into cluster health.", 45.0, "High"),
            why("Least-privilege IAM (IRSA)", "Each pod gets only the AWS perms it needs.", 0.0, "Critical"),
            why("Node Termination Handler", "Graceful drain on spot interruption.", 0.0, "Medium"),
            why("Secrets encryption (KMS)", "K8s secrets encrypted at rest in etcd.", 0.0, "High"),
        ],
        "vpc" => vec![
            why("Multi-AZ (3 AZs)", "Redundancy across failure domains.", 0.0, "Critical"),
            why("HA NAT (per-AZ)", "Each AZ has its own NAT. One AZ down doesn't kill outbound.", 65.0, "High"),
            why("VPC Flow Logs", "Audit trail for network traffic. Required for compliance.", 5.0, "Medium"),
            why("VPC Endpoints", "Private connectivity to AWS services. No internet transit.", 87.0, "Medium"),
        ],
        "ec2" => vec![
            why("Termination protection", "Prevents accidental deletion.", 0.0, "Critical"),
            why("Detailed monitoring", "1-min CloudWatch metrics for faster alerting.", 3.50, "Medium"),
            why("Private subnet", "Not directly reachable from internet.", 0.0, "High"),
            why("Encrypted volumes", "EBS encrypted at rest. No perf impact.", 0.0, "High"),
        ],
        "ebs" => vec![
            why("Encryption at rest", "AES-256 encryption. Zero perf impact on modern instances.", 0.0, "High"),
        ],
        _ => vec![],
    }
}

fn why(decision: &str, reason: &str, cost: f64, risk: &str) -> WhyExplanation {
    WhyExplanation {
        decision: decision.to_string(),
        reason: reason.to_string(),
        additional_cost: cost,
        risk_level: risk.to_string(),
    }
}
