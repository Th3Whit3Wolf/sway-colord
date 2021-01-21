use anyhow::{anyhow, Result};
use json5;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty, Value};

use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VSCode {
    pub dark_theme: Option<String>,
    pub light_theme: Option<String>,
}

impl VSCode {
    pub fn dark_mode(&self) -> Result<()> {
        if let Some(theme) = &self.dark_theme {
            VSCode::change_theme(theme.as_str())?;
        }
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        if let Some(theme) = &self.light_theme {
            VSCode::change_theme(theme.as_str())?;
        }
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        self.dark_theme.is_some()
            && self.light_theme.is_some()
            && (dirs_next::home_dir()
                .expect("Error: unable to find home directory")
                .join(".config/Code - OSS/User/settings.json")
                .is_file()
                || dirs_next::home_dir()
                    .expect("Error: unable to find home directory")
                    .join(".config/Code/User/settings.json")
                    .is_file())
    }
    pub fn change_theme(theme: &str) -> Result<()> {
        let setting = if dirs_next::home_dir()
            .expect("Error: unable to find home directory")
            .join(".config/Code - OSS/User/settings.json")
            .is_file()
        {
            Ok(dirs_next::home_dir()
                .expect("Error: unable to find home directory")
                .join(".config/Code - OSS/User/settings.json"))
        } else if dirs_next::home_dir()
            .expect("Error: unable to find home directory")
            .join(".config/Code/User/settings.json")
            .is_file()
        {
            Ok(dirs_next::home_dir()
                .expect("Error: unable to find home directory")
                .join(".config/Code/User/settings.json"))
        } else {
            Err(anyhow!("Could not find the vscode user directory"))
        };
        if let Ok(file) = &setting {
            let data = fs::read_to_string(file.clone()).expect("Unable to read file");
            // Parse the string of data into serde_json::Value.
            let v: Value = json5::from_str(&data).expect("failure to convert");

            let mut v: Value = serde_json::from_str(v.to_string().as_str())?;

            match v.get_mut("workbench.colorTheme") {
                Some(val) => *val = json!(theme),
                None => println!("Theme not set"),
            }

            let mut f = File::create(setting?).expect("Unable to create file");
            f.write_all(to_string_pretty(&v)?.as_bytes())
                .expect("Unable to write data");
            f.flush().expect("Error: Flushing  VSCodes settings.json");
        };
        Ok(())
    }
}
