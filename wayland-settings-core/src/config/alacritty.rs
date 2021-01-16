use super::utils::theme_file;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Alacritty {
    pub dark_theme: Option<String>,
    pub light_theme: Option<String>,
}

const ALACRITTY_SETTINGS_STARTS_WITH: &str = "colors";

impl Alacritty {
    pub fn dark_mode(&self) -> Result<()> {
        let setting = dirs_next::home_dir().expect("Error: unable to find home directory").join(".config/alacritty/alacritty.yml");
        let dark_theme_insert = format!(
            "{}: *{}",
            ALACRITTY_SETTINGS_STARTS_WITH,
            self.dark_theme
                .as_deref()
                .ok_or(anyhow!("No dark mode for Alacritty"))?
        );
        theme_file(
            setting,
            ALACRITTY_SETTINGS_STARTS_WITH,
            dark_theme_insert,
        );
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        let setting = dirs_next::home_dir().expect("Error: unable to find home directory").join(".config/alacritty/alacritty.json");
        let light_theme_insert = format!(
            "{}: *{}",
            ALACRITTY_SETTINGS_STARTS_WITH,
            self.light_theme
                .as_deref()
                .expect("Error Alacritty: light mode theme does not exist")
        );
        theme_file(
            setting,
            ALACRITTY_SETTINGS_STARTS_WITH,
            light_theme_insert,
        );
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        self.dark_theme.is_some() && self.light_theme.is_some()
    }
}
