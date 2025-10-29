use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Player {
    pub role: String,
    #[serde(default)]
    pub tokens: Vec<serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct DayPlayer {
    pub claim: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Day {
    pub players: HashMap<String, DayPlayer>,
}

#[derive(Debug, Deserialize)]
pub struct World {
    pub script: String,
    pub players: HashMap<String, Player>,
    #[serde(default)]
    pub days: Vec<Day>,
}

pub enum Error {
    ParseError(Box<dyn std::error::Error>),
}
