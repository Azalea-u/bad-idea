// config.rs
#![allow(dead_code)] // Temporarily suppress unused warnings
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct AutomationConfig {
    pub automations: Vec<Automation>,
}

#[derive(Debug, Deserialize)]
pub struct Automation {
    pub id: String,
    pub name: String,
    pub trigger: Trigger,
    #[serde(default)]
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Trigger {
    #[serde(rename = "time")]
    Time { at: String },
    #[serde(rename = "event")]
    Event { on: String },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    #[serde(rename = "wifi")]
    Wifi { status: String },
    #[serde(rename = "file_exists")]
    FileExists { path: PathBuf },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "run_app")]
    RunApp { app: String },
    #[serde(rename = "wait")]
    Wait { duration: u64 },
    #[serde(rename = "send_email")]
    SendEmail {
        to: String,
        subject: String,
        body: String,
    },
}