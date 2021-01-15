use anyhow::Result;
use async_recursion::async_recursion;
use async_std::task;
use chrono::{prelude::*, NaiveTime, Timelike, Utc};
use std::time::Duration;
mod config;

pub use config::Config;

const DAWN_H: u64 = 7;
const DUSK_H: u64 = DAWN_H + 12;
const DAWN: u64 = DAWN_H * 3600;
const DUSK: u64 = DUSK_H * 3600;
const DAY: u64 = 24 * 3600;

const LAT: f64 = 52.4045;
const LON: f64 = 0.5613;

fn time_as_secs() -> u64 {
    let now: NaiveTime = Utc::now().time();
    ((now.hour() * 3600) + (now.minute() * 60) + now.second()) as u64
}

#[async_recursion]
async fn auto_change_rigid(conf: Config) -> Result<()> {
    let time: u64 = time_as_secs();
    if time < DAWN {
        conf.set_dark_mode().await?;
        task::sleep(Duration::from_secs(DAWN - time)).await;
    } else if time > DUSK {
        conf.set_dark_mode().await?;
        task::sleep(Duration::from_secs(DAWN + DAY - time)).await;
    } else {
        conf.set_light_mode().await?;
        task::sleep(Duration::from_secs(DUSK - time)).await;
    }
    auto_change_rigid(conf).await?;
    Ok(())
}

#[async_recursion]
async fn auto_change_solar(conf: Config) -> Result<()> {
    let utc: DateTime<Utc> = Utc::now();
    // Calculate times for January 1, 2016 in Toronto
    let (sunrise, sunset) = sunrise::sunrise_sunset(LAT, LON, utc.year(), utc.month(), utc.day());

    let now = utc.timestamp();
    if now < sunrise {
        conf.set_dark_mode().await?;
        task::sleep(Duration::from_secs((sunrise-now) as u64)).await;
    } else if now > sunset {
        let tomorrow_morning = sunrise::sunrise_sunset(LAT, LON, utc.year(), utc.month(), utc.day() + 1).0;
        
        conf.set_dark_mode().await?;
        task::sleep(Duration::from_secs(
            (tomorrow_morning-now) as u64
        ))
        .await;
    } else {
        conf.set_light_mode().await?;
        task::sleep(Duration::from_secs(
            (sunset - now) as u64
        )).await;
    }

    auto_change_solar(conf).await?;
    Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
    let conf = Config::get()?;
    //auto_change_rigid(conf).await?;
    auto_change_solar(conf).await?;
    Ok(())
}
