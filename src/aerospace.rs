use serde::Deserialize;
use std::{error::Error, fs, process::Command};

#[derive(Debug, Deserialize)]
pub struct AerospaceConfig {
    // we only care about gaps for now
    gaps: Gaps,
}

#[derive(Debug, Deserialize)]
pub struct Gaps {
    pub inner: Inner,
    pub outer: Outer,
}

#[derive(Debug, Deserialize)]
pub struct Inner {
    pub horizontal: u32,
    pub vertical: u32,
}

#[derive(Debug, Deserialize)]
pub struct Outer {
    pub left: u32,
    pub bottom: u32,
    pub top: u32,
    pub right: u32,
}

// this returns a result since its assumed that a user has a valid config
pub fn get_padding() -> Result<Gaps, Box<dyn Error>> {
    let config_path = get_config_path().expect("Aerospace config path not found");
    let config = read_config(&config_path).expect(
        format!(
            "Unable to read from Aerospace config path at {}",
            config_path
        )
        .as_str(),
    );
    Ok(config.gaps)
}

fn get_config_path() -> Option<String> {
    let output = Command::new("aerospace")
        .args(&["config", "--config-path"])
        .output()
        .ok()?;
    let config_path = String::from_utf8(output.stdout).ok()?;
    Some(config_path.trim().to_owned())
}

fn read_config(config_path: &str) -> Result<AerospaceConfig, Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(config_path)?;
    let config: AerospaceConfig = toml::from_str(&toml_str)?;
    Ok(config)
}

pub fn get_focused_monitor_id() -> Option<u32> {
    let output = Command::new("aerospace")
        .args(&[
            "list-monitors",
            "--focused",
            "--format",
            "%{monitor-appkit-nsscreen-screens-id}",
        ])
        .output()
        .ok()?;

    let id_str = String::from_utf8(output.stdout).ok()?;
    let id_str = id_str.trim();
    id_str.parse::<u32>().ok()
}

pub fn get_focused_window_id() -> Option<i32> {
    let output = Command::new("aerospace")
        .args(&["list-windows", "--focused", "--format", "%{window-id}"])
        .output()
        .ok()?;

    let id_str = String::from_utf8(output.stdout).ok()?;
    let id_str = id_str.trim();

    id_str.parse::<i32>().ok()
}
