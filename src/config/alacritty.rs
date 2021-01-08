use super::utils::theme_file;
use serde::Deserialize;
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;

use std::path::PathBuf;
#[derive(Debug, Deserialize)]
pub struct Alacritty {
    pub dark_theme: Option<String>,
    pub light_theme: Option<String>,
}

fn alacritty_yml() -> &'static PathBuf {
    static ALACRITTY_YML: OnceCell<PathBuf> = OnceCell::new();
    ALACRITTY_YML.get_or_init(|| {
        xdg::BaseDirectories::with_prefix("alacritty")
        .expect("~/.config/alacritty doesn't exist")
        .find_config_file("alacritty.yml")
        .expect("Unable to read alacritty.yml")
    })
}

const ALACRITTY_SETTINGS_STARTS_WITH: &str = "colors";

impl Alacritty {
    pub fn dark_mode(&self) -> Result<()> {
        let dark_theme_insert = format!(
            "{}: *{}",
            ALACRITTY_SETTINGS_STARTS_WITH,
            self.dark_theme
                .as_deref()
                .ok_or(anyhow!("No dark mode for Alacritty"))?
        );
        theme_file(
            alacritty_yml().to_owned(),
            ALACRITTY_SETTINGS_STARTS_WITH,
            dark_theme_insert,
        );
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        let light_theme_insert = format!(
            "{}: *{}",
            ALACRITTY_SETTINGS_STARTS_WITH,
            self.light_theme
                .as_deref()
                .expect("Error Alacritty: light mode theme does not exist")
        );
        theme_file(
            alacritty_yml().to_owned(),
            ALACRITTY_SETTINGS_STARTS_WITH,
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
