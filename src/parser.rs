use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Player {
    pub role: String,
    #[serde(default)]
    pub role_data: serde_yaml::Value,
    #[serde(default)]
    pub tokens: Vec<serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub claim: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    pub step: u32,
    pub events: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct World {
    pub script: String,
    pub players: HashMap<String, Player>,
    #[serde(default)]
    pub steps: Vec<Step>,
}
