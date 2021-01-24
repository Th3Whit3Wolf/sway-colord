use std::fs;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use zbus::Connection;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Monitor {
    pub device: String,
    pub dark_perc: u64,
    pub light_perc: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Keyboard {
    pub device: String,
    pub dark_perc: u64,
    pub light_perc: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lighting {
    pub monitor: Option<Monitor>,
    pub keyboard: Option<Keyboard>,
}

impl Lighting {
    pub fn dark_mode(&self) -> Result<()> {
        let sd_bus = Connection::new_system().unwrap();

        if let Some(mon) = &self.monitor {
            let dev = &mon.device;
            let dark_perc = mon.dark_perc;
            let max_b = get_mon_max_brightness(dev.as_str())?;
            let cur_b = get_mon_current_brightness(dev.as_str())?;
            let cur_p = (cur_b as f64 / max_b as f64).round();

            if dark_perc < 100u64 && dark_perc != cur_p as u64 {
                match sd_bus.call_method(
                    Some("org.freedesktop.login1"),
                    "/org/freedesktop/login1/session/auto",
                    Some("org.freedesktop.login1.Session"),
                    "SetBrightness",
                    &(
                        "backlight",
                        dev,
                        (dark_perc as f64 / 100.0 * max_b as f64).round() as u32,
                    ),
                ) {
                    Ok(_msg) => {}
                    Err(e) => println!("Failed to set brightness: {}", e),
                }
            }
        }
        if let Some(kbd) = &self.keyboard {
            let dev = &kbd.device;
            let dark_perc = kbd.dark_perc;
            let max_b = get_kbd_max_brightness(dev.as_str())?;
            let cur_b = get_kbd_current_brightness(dev.as_str())?;
            let cur_p = (cur_b as f64 / max_b as f64).round();

            if dark_perc < 100u64 && dark_perc != cur_p as u64 {
                match sd_bus.call_method(
                    Some("org.freedesktop.login1"),
                    "/org/freedesktop/login1/session/auto",
                    Some("org.freedesktop.login1.Session"),
                    "SetBrightness",
                    &(
                        "leds",
                        dev,
                        (dark_perc as f64 / 100.0 * max_b as f64).round() as u32,
                    ),
                ) {
                    Ok(_msg) => {}
                    Err(e) => println!("Failed to set brightness: {}", e),
                }
            }
        }
        Ok(())
    }
    pub fn light_mode(&self) -> Result<()> {
        let sd_bus = Connection::new_system().unwrap();

        if let Some(mon) = &self.monitor {
            let dev = &mon.device;
            let light_perc = mon.light_perc;
            let max_b = get_mon_max_brightness(dev.as_str())?;
            let cur_b = get_mon_current_brightness(dev.as_str())?;
            let cur_p = (cur_b as f64 / max_b as f64 * 100.0).round();

            if light_perc < 100u64 && light_perc != cur_p as u64 {
                match sd_bus.call_method(
                    Some("org.freedesktop.login1"),
                    "/org/freedesktop/login1/session/auto",
                    Some("org.freedesktop.login1.Session"),
                    "SetBrightness",
                    &(
                        "backlight",
                        dev,
                        (light_perc as f64 / 100.0 * max_b as f64).round() as u32,
                    ),
                ) {
                    Ok(_msg) => {}
                    Err(e) => println!("Failed to set brightness: {}", e),
                }
            }
        }
        if let Some(kbd) = &self.keyboard {
            let dev = &kbd.device;
            let light_perc = kbd.light_perc;
            let max_b = get_kbd_max_brightness(dev.as_str())?;
            let cur_b = get_kbd_current_brightness(dev.as_str())?;
            let cur_p = (cur_b as f64 / max_b as f64 * 100.0).round();

            if light_perc < 100u64 && light_perc != cur_p as u64 {
                match sd_bus.call_method(
                    Some("org.freedesktop.login1"),
                    "/org/freedesktop/login1/session/auto",
                    Some("org.freedesktop.login1.Session"),
                    "SetBrightness",
                    &(
                        "leds",
                        dev,
                        (light_perc as f64 / 100.0 * max_b as f64).round() as u32,
                    ),
                ) {
                    Ok(_msg) => {}
                    Err(e) => println!("Failed to set brightness: {}", e),
                }
            }
        }
        Ok(())
    }
    pub fn is_some(&self) -> bool {
        self.monitor.is_some() || self.keyboard.is_some()
    }
}
fn get_mon_max_brightness(device: &str) -> Result<u64> {
    if let Ok(file) = fs::read_to_string(format!("/sys/class/backlight/{}/max_brightness", device))
    {
        if let Ok(num) = file.trim().parse::<u64>() {
            Ok(num)
        } else {
            Err(anyhow!("unable to parse brightness to number"))
        }
    } else {
        Err(anyhow!("unable ot parse string into path"))
    }
}

fn get_mon_current_brightness(device: &str) -> Result<u64> {
    if let Ok(file) = fs::read_to_string(format!("/sys/class/backlight/{}/brightness", device)) {
        if let Ok(num) = file.trim().parse::<u64>() {
            Ok(num)
        } else {
            Err(anyhow!("unable to parse brightness to number"))
        }
    } else {
        Err(anyhow!("unable ot parse string into path"))
    }
}
fn get_kbd_max_brightness(device: &str) -> Result<u64> {
    if let Ok(file) = fs::read_to_string(format!("/sys/class/leds/{}/max_brightness", device)) {
        if let Ok(num) = file.trim().parse::<u64>() {
            Ok(num)
        } else {
            Err(anyhow!("unable to parse max brightness to number"))
        }
    } else {
        Err(anyhow!("unable ot parse string into path"))
    }
}

fn get_kbd_current_brightness(device: &str) -> Result<u64> {
    if let Ok(file) = fs::read_to_string(format!("/sys/class/leds/{}/brightness", device)) {
        if let Ok(num) = file.trim().parse::<u64>() {
            Ok(num)
        } else {
            Err(anyhow!("unable to parse current brightness to number"))
        }
    } else {
        Err(anyhow!("unable ot parse string into path"))
    }
}
