use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FebiFlowConfigAws {
  pub region: String,
  pub iam_role: String,
}

#[derive(Debug, Deserialize)]
pub struct FebiFlowConfigProfile {
  pub env_path: String,
}

#[derive(Debug, Deserialize)]
pub struct FebiFlowConfig {
  pub aws: FebiFlowConfigAws,
  pub profiles: HashMap<String, FebiFlowConfigProfile>
}
