use super::utils::theme_file;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArbitraryList {
    pub arbitraries: Vec<Arbitrary>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Arbitrary {
    pub config_file: String,
    pub dark_line: String,
    pub light_line: String,
    pub post_hook: Option<String>,
}

impl ArbitraryList {
    pub fn is_some(&self) -> bool {
        if self.arbitraries.is_empty() {
            false
        } else {
            let mut status: Box<bool> = Box::new(true);
            for arbitrary in self.arbitraries.iter() {
                if !arbitrary.is_some() {
                    *status = false
                }
            }
            *status
        }
    }
    pub fn dark_mode(&self) -> Result<()> {
        for arbitrary in self.arbitraries.iter() {
            arbitrary.dark_mode()?;
        }
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        for arbitrary in self.arbitraries.iter() {
            arbitrary.light_mode()?;
        }
        Ok(())
    }
}

impl Arbitrary {
    pub fn is_some(&self) -> bool {
        if let Some(path) = expand_shell_path(self.config_file.to_owned()) {
            path.exists()
        } else {
            false
        }
    }
    pub fn dark_mode(&self) -> Result<()> {
        if let Some(path) = expand_shell_path(self.config_file.to_owned()) {
            if theme_file(path, &self.light_line, self.dark_line.to_owned()).is_ok() {
                if let Some(cmd) = &self.post_hook {
                    Command::new("sh").arg("-c").arg(cmd).output()?;
                };
            }
        };
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        if let Some(path) = expand_shell_path(self.config_file.to_owned()) {
            if theme_file(path, &self.dark_line, self.light_line.to_owned()).is_ok() {
                if let Some(cmd) = &self.post_hook {
                    Command::new("sh").arg("-c").arg(cmd).output()?;
                };
            }
        }
        Ok(())
    }
}

fn expand_shell_path<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") || p == Path::new("$HOME") {
        return dirs_next::home_dir();
    }
    dirs_next::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}
