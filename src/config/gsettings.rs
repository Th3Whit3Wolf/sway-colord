use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use swayipc::Connection;

const GSETTINGS: &str = "exec gsettings set org.gnome.desktop.interface";

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub fn dark_mode(&self) -> Result<()> {
        let mut v: Vec<String> = Vec::with_capacity(4);
        if self.has_gtk_theme() {
            v.push(format!(
                "{} gtk-theme {}",
                GSETTINGS,
                self.dark_gtk_theme
                    .as_deref()
                    .ok_or(anyhow!("No dark gtk theme"))?
            ))
        }
        if self.has_icon_theme() {
            v.push(format!(
                "{} icon-theme {}",
                GSETTINGS,
                self.dark_icon_theme
                    .as_deref()
                    .ok_or(anyhow!("No dark icon theme"))?
            ))
        }
        if self.has_cursor_theme() {
            v.push(format!(
                "{} cursor-theme {}",
                GSETTINGS,
                self.dark_cursor_theme
                    .as_deref()
                    .ok_or(anyhow!("No dark cursor theme"))?
            ))
        }
        if self.has_font_name() {
            v.push(format!(
                "{} font-name {}",
                GSETTINGS,
                self.dark_font_name
                    .as_deref()
                    .ok_or(anyhow!("No dark font name"))?
            ))
        }
        sway_exec(v)?;
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        let mut v: Vec<String> = Vec::with_capacity(4);
        if self.has_gtk_theme() {
            v.push(format!(
                "{} gtk-theme {}",
                GSETTINGS,
                self.light_gtk_theme
                    .as_deref()
                    .ok_or(anyhow!("No light gtk theme"))?
            ))
        }
        if self.has_icon_theme() {
            v.push(format!(
                "{} icon-theme {}",
                GSETTINGS,
                self.light_icon_theme
                    .as_deref()
                    .ok_or(anyhow!("No light icon theme"))?
            ))
        }
        if self.has_cursor_theme() {
            v.push(format!(
                "{} cursor-theme {}",
                GSETTINGS,
                self.light_cursor_theme
                    .as_deref()
                    .ok_or(anyhow!("No light cursor theme"))?
            ))
        }
        if self.has_font_name() {
            v.push(format!(
                "{} font-name {}",
                GSETTINGS,
                self.light_font_name
                    .as_deref()
                    .ok_or(anyhow!("No light font name"))?
            ))
        }
        sway_exec(v)?;
        Ok(())
    }

    pub fn has_gtk_theme(&self) -> bool {
        self.dark_gtk_theme.is_some() && self.light_gtk_theme.is_some()
    }

    pub fn has_icon_theme(&self) -> bool {
        self.dark_icon_theme.is_some() && self.light_icon_theme.is_some()
    }

    pub fn has_cursor_theme(&self) -> bool {
        self.dark_cursor_theme.is_some() && self.light_cursor_theme.is_some()
    }

    pub fn has_font_name(&self) -> bool {
        self.dark_font_name.is_some() && self.light_font_name.is_some()
    }

    pub fn is_some(&self) -> bool {
        self.has_gtk_theme()
            || self.has_icon_theme()
            || self.has_cursor_theme()
            || self.has_font_name()
    }
}

fn sway_exec(v: Vec<String>) -> Result<()> {
    let mut connection = Connection::new()?;
    for cmd in v {
        connection.run_command(&cmd)?;
        dbg!("swaymsg {}", &cmd);
    }
    Ok(())
}