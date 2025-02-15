// basic YAML configuration file(will be changed probably)
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
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Trigger {
    Time { time: String },
    Event { event: String },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    Wifi { status: String },
    FileExists { path: PathBuf },
    // will add more conditions later I guess :)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    RunApp { app: String },
    RunCommand { command: String },
    CloseApp { app: String },
    Wait { duration: u64 },
    SendEmail {
        to: String,
        subject: String,
        body: String,
    },
    ClickButton { button: String },
    // will add more actions later I guess :)
}
