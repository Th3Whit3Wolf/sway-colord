use anyhow::{anyhow, Result};
use serde::Deserialize;
use swayipc_async::Connection;

const GSETTINGS: &str = "gsettings set org.gnome.desktop.interface";

#[derive(Debug, Deserialize)]
pub struct GSettings {
    pub dark_gtk_theme: Option<String>,
    pub dark_icon_theme: Option<String>,
    pub dark_cursor_theme: Option<String>,
    pub dark_font_name: Option<String>,
    pub light_gtk_theme: Option<String>,
    pub light_icon_theme: Option<String>,
    pub light_cursor_theme: Option<String>,
    pub light_font_name: Option<String>,
}

impl GSettings {
    pub async fn dark_mode(&self) -> Result<()> {
        let mut v: Vec<String> = Vec::with_capacity(4);
        let mut connection = Connection::new().await?;
        if self.has_gtk_theme() {
            v.push(format!(
                "\t{} gtk-theme {}",
                GSETTINGS,
                self.dark_gtk_theme
                    .as_deref()
                    .ok_or(anyhow!("No dark gtk theme"))?
            ))
        }
        if self.has_icon_theme() {
            v.push(format!(
                "\t{} icon-theme {}",
                GSETTINGS,
                self.dark_icon_theme
                    .as_deref()
                    .ok_or(anyhow!("No dark icon theme"))?
            ))
        }
        if self.has_cursor_theme() {
            v.push(format!(
                "\t{} cursor-theme {}",
                GSETTINGS,
                self.dark_cursor_theme
                    .as_deref()
                    .ok_or(anyhow!("No dark cursor theme"))?
            ))
        }
        if self.has_font_name() {
            v.push(format!(
                "\t{} font-name {}",
                GSETTINGS,
                self.dark_font_name
                    .as_deref()
                    .ok_or(anyhow!("No dark font name"))?
            ))
        }
        for cmd in v {
            connection.run_command(format!("exec {}", &cmd)).await?;
            println!("{}", cmd)
        }
        Ok(())
    }
    pub async fn light_mode(&self) -> Result<()> {
        let mut v: Vec<String> = Vec::with_capacity(4);
        let mut connection = Connection::new().await?;
        if self.has_gtk_theme() {
            v.push(format!(
                "\t{} gtk-theme {}",
                GSETTINGS,
                self.light_gtk_theme
                    .as_deref()
                    .ok_or(anyhow!("No light gtk theme"))?
            ))
        }
        if self.has_icon_theme() {
            v.push(format!(
                "\t{} icon-theme {}",
                GSETTINGS,
                self.light_icon_theme
                    .as_deref()
                    .ok_or(anyhow!("No light icon theme"))?
            ))
        }
        if self.has_cursor_theme() {
            v.push(format!(
                "\t{} cursor-theme {}",
                GSETTINGS,
                self.light_cursor_theme
                    .as_deref()
                    .ok_or(anyhow!("No light cursor theme"))?
            ))
        }
        if self.has_font_name() {
            v.push(format!(
                "\t{} font-name {}",
                GSETTINGS,
                self.light_font_name
                    .as_deref()
                    .ok_or(anyhow!("No light font name"))?
            ))
        }
        for cmd in v {
            connection.run_command(format!("exec {}", &cmd)).await?;
            println!("{}", cmd)
        }
        Ok(())
    }

    pub fn has_gtk_theme(&self) -> bool {
        if self.dark_gtk_theme.is_some() && self.light_gtk_theme.is_some() {
            true
        } else {
            false
        }
    }

    pub fn has_icon_theme(&self) -> bool {
        if self.dark_icon_theme.is_some() && self.light_icon_theme.is_some() {
            true
        } else {
            false
        }
    }

    pub fn has_cursor_theme(&self) -> bool {
        if self.dark_cursor_theme.is_some() && self.light_cursor_theme.is_some() {
            true
        } else {
            false
        }
    }

    pub fn has_font_name(&self) -> bool {
        if self.dark_font_name.is_some() && self.light_font_name.is_some() {
            true
        } else {
            false
        }
    }

    pub fn is_some(&self) -> bool {
        if self.has_gtk_theme() {
            true
        } else if self.has_icon_theme() {
            true
        } else if self.has_cursor_theme() {
            true
        } else if self.has_font_name() {
            true
        } else {
            false
        }
    }
}
