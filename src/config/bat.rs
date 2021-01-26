use super::utils::theme_file;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bat {
    pub dark_theme: Option<String>,
    pub light_theme: Option<String>,
}

const BAT_SETTINGS_STARTS_WITH: &str = "--theme=";

impl Bat {
    pub fn dark_mode(&self) -> Result<()> {
        if let Some(theme) = &self.dark_theme {
            Bat::change_theme(theme.as_str())?;
        }
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        if let Some(theme) = &self.light_theme {
            Bat::change_theme(theme.as_str())?;
        }
        Ok(())
    }
    pub fn change_theme(theme: &str) -> Result<()> {
        let setting = dirs_next::home_dir()
            .expect("Error: unable to find home directory")
            .join(".config/bat/config");
        let theme_insert = format!("{}{}", BAT_SETTINGS_STARTS_WITH, theme);
        theme_file(setting, BAT_SETTINGS_STARTS_WITH, theme_insert)?;
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        self.dark_theme.is_some()
            && self.light_theme.is_some()
            && dirs_next::home_dir()
                .expect("Error: unable to find home directory")
                .join(".config/Bat/config")
                .is_file()
    }
}
