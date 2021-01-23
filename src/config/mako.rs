use anyhow::Result;
use libc::kill;
use procfs::process;
use serde::{Deserialize, Serialize};

use std::{
    fs::File,
    process::{Command, Stdio},
};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mako {
    pub dark_theme: Option<String>,
    pub light_theme: Option<String>,
}

impl Mako {
    pub fn light_mode(&self) -> Result<()> {
        if let Some(theme_name) = &self.light_theme {
            let theme = format!("~/.config/mako/{}", &theme_name);
            change_theme(&theme)?
        }
        Ok(())
    }
    pub fn dark_mode(&self) -> Result<()> {
        if let Some(theme_name) = &self.dark_theme {
            let theme = format!("~/.config/mako/{}", &theme_name);
            change_theme(&theme)?
        }
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        let mako_dir = dirs_next::home_dir().expect("Error: unable to find home directory").join(".config/mako");
        if let Some(light_path) = &self.light_theme {
            if mako_dir.join(light_path).is_file() {
                if let Some(dark_path) = &self.dark_theme {
                    if mako_dir.join(dark_path).is_file() {
                        true
                    } else {
                        println!("File: {:?} - not found", mako_dir.join(dark_path));
                        false
                    }
                } else {
                    false
                }
            } else {
                println!("File: {:?} - not found", mako_dir.join(light_path));
                false
            }
        } else {
            false
        }
    }
}

fn change_theme(theme: &str) -> Result<()> {
    let mut pids = Vec::with_capacity(2);
    for prc in process::all_processes()? {
        if prc.stat.comm.starts_with("mako") || prc.stat.comm.starts_with("/bin/sh -c mako") {
            pids.push(prc.stat.pid)
        }
    }
    if pids.is_empty() == false {
        for pid in &pids {
            println!("PID: {:?}", pid);
            unsafe { kill(*pid, 9) };
        }
    }
    pids.clear();

    println!("Mako theme path: {}",&theme);
    let outputs = File::create("/tmp/mako.log")?;
    let errors = outputs.try_clone()?;

    let mako = format!("mako -c {}", &theme);
    Command::new("sh")
        .args(&["-c", &mako])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?;

    println!("Mako theme changed");

    Ok(())
}
