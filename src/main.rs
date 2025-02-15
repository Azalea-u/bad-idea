mod config;
use crate::config::AutomationConfig;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml_content = fs::read_to_string("automation.yaml")?;
    let config: AutomationConfig = serde_yaml::from_str(&yaml_content)?;

    println!("{:#?}", config);

    Ok(())
}