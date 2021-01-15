use anyhow::Result;
use json5;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_json::{json, to_string_pretty, Value};

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Deserialize)]
pub struct VSCode {
    pub dark_theme: Option<String>,
    pub light_theme: Option<String>,
}

fn vscode_settings() -> &'static PathBuf {
    static VSCODE_SETTINGS: OnceCell<PathBuf> = OnceCell::new();
    VSCODE_SETTINGS.get_or_init(|| {
        xdg::BaseDirectories::with_profile("Code - OSS", "User")
            .expect("~/.config/Code - OSS/User doesn't exist")
            .find_config_file("settings.json")
            .expect("Unable to read settings.json")
    })
}

impl VSCode {
    pub fn dark_mode(&self) -> Result<()> {
        if let Some(theme) = &self.dark_theme {
            change_theme(&theme)?;
        }
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        if let Some(theme) = &self.light_theme {
            change_theme(&theme)?;
        }
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        self.dark_theme.is_some() && self.light_theme.is_some()
    }
}

fn change_theme(theme: &str) -> Result<()> {
    let data = fs::read_to_string(vscode_settings()).expect("Unable to read file");

    // Parse the string of data into serde_json::Value.
    let v: Value = json5::from_str(&data).expect("failure to convert");

    let mut v: Value = serde_json::from_str(v.to_string().as_str())?;

    match v.get_mut("workbench.colorTheme") {
        Some(val) => *val = json!(theme),
        None => println!("Theme not set"),
    }

    let mut f = File::create(vscode_settings()).expect("Unable to create file");
    f.write_all(to_string_pretty(&v)?.as_bytes())
        .expect("Unable to write data");
    f.flush().expect("Error: Flushing  VSCodes settings.json");
    Ok(())
}
