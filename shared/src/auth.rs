use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsAccount {
    pub id: Uuid,
    pub name: String,
    pub account_id: String,
    pub default_region: String,
    pub auth_method: AwsAuthMethod,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AwsAuthMethod {
    IamKeys {
        access_key_id: String,
        secret_access_key: String,
    },
    AssumeRole {
        role_arn: String,
        external_id: Option<String>,
        source_access_key_id: Option<String>,
        source_secret_access_key: Option<String>,
    },
}

impl std::fmt::Display for AwsAuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AwsAuthMethod::IamKeys { .. } => write!(f, "IAM Access Keys"),
            AwsAuthMethod::AssumeRole { .. } => write!(f, "Assume Role"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAccountRequest {
    pub name: String,
    pub account_id: String,
    pub default_region: String,
    pub auth_method: AwsAuthMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthValidation {
    pub valid: bool,
    pub account_id: String,
    pub arn: String,
    pub user_id: String,
    pub error: Option<String>,
}

impl AwsAccount {
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        let mut vars = vec![
            ("AWS_REGION".to_string(), self.default_region.clone()),
            ("AWS_DEFAULT_REGION".to_string(), self.default_region.clone()),
        ];
        match &self.auth_method {
            AwsAuthMethod::IamKeys { access_key_id, secret_access_key } => {
                vars.push(("AWS_ACCESS_KEY_ID".to_string(), access_key_id.clone()));
                vars.push(("AWS_SECRET_ACCESS_KEY".to_string(), secret_access_key.clone()));
            }
            AwsAuthMethod::AssumeRole { role_arn, external_id, source_access_key_id, source_secret_access_key } => {
                if let (Some(ak), Some(sk)) = (source_access_key_id, source_secret_access_key) {
                    vars.push(("AWS_ACCESS_KEY_ID".to_string(), ak.clone()));
                    vars.push(("AWS_SECRET_ACCESS_KEY".to_string(), sk.clone()));
                }
                vars.push(("TF_VAR_assume_role_arn".to_string(), role_arn.clone()));
                if let Some(eid) = external_id {
                    vars.push(("TF_VAR_assume_role_external_id".to_string(), eid.clone()));
                }
            }
        }
        vars
    }

    pub fn provider_override(&self) -> Option<String> {
        match &self.auth_method {
            AwsAuthMethod::AssumeRole { role_arn, external_id, .. } => {
                let ext = external_id.as_ref()
                    .map(|e| format!("    external_id = \"{}\"\n", e))
                    .unwrap_or_default();
                Some(format!(
                    "provider \"aws\" {{\n  region = \"{}\"\n\n  assume_role {{\n    role_arn = \"{}\"\n{}  }}\n}}\n",
                    self.default_region, role_arn, ext
                ))
            }
            _ => None,
        }
    }

    pub fn masked_credentials(&self) -> String {
        match &self.auth_method {
            AwsAuthMethod::IamKeys { access_key_id, .. } => {
                if access_key_id.len() > 8 {
                    format!("{}...{}", &access_key_id[..4], &access_key_id[access_key_id.len()-4..])
                } else { "****".to_string() }
            }
            AwsAuthMethod::AssumeRole { role_arn, .. } => role_arn.clone(),
        }
    }
}
