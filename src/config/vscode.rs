use super::utils::theme_file;
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use serde::Deserialize;

use std::path::PathBuf;

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

const VSCODE_SETTINGS_STARTS_WITH: &str = "    \"workbench.colorTheme\"";

impl VSCode {
    pub fn dark_mode(&self) -> Result<()> {
        let dark_theme_insert = format!(
            "{}: \"{}\",",
            VSCODE_SETTINGS_STARTS_WITH,
            self.dark_theme
            .as_deref()
            .ok_or(anyhow!("No dark mode for VSCode"))?
        );
        theme_file(
            vscode_settings().to_owned(),
            VSCODE_SETTINGS_STARTS_WITH,
            dark_theme_insert,
        );
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        let light_theme_insert = format!(
            "{}: \"{}\",",
            VSCODE_SETTINGS_STARTS_WITH,
            self.light_theme
                .as_deref()
                .ok_or(anyhow!("No light mode for VSCode"))?
            );
        theme_file(
            vscode_settings().to_owned(),
            VSCODE_SETTINGS_STARTS_WITH,
            light_theme_insert,
        );
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        if self.dark_theme.is_some() && self.light_theme.is_some() {
            true
        } else {
            false
        }
    }
}
