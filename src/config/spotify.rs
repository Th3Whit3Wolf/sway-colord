use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::process::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Spotify {
    pub dark_theme: Option<String>,
    pub light_theme: Option<String>,
}

impl Spotify {
    pub fn light_mode(&self) -> Result<()> {
        if let Some(theme) = &self.light_theme {
            change_theme(&theme)?
        }
        Ok(())
    }
    pub fn dark_mode(&self) -> Result<()> {
        if let Some(theme) = &self.dark_theme {
            change_theme(&theme)?
        }
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        let spicetify = dirs_next::home_dir()
            .expect("Error: unable to find home directory")
            .join(".config/spicetify/config.ini");
        self.dark_theme.is_some() && self.light_theme.is_some() && spicetify.is_file()
    }
}

fn change_theme(theme: &str) -> Result<()> {
    Command::new("spicetify")
        .args(&["config", "color_scheme", theme])
        .spawn()?
        .wait_with_output()?;

    Command::new("spicetify")
        .args(&["-n", "apply"])
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
