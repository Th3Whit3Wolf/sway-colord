mod config;

use anyhow::Result;
use async_recursion::async_recursion;
use async_std::task;
use chrono::{prelude::*, NaiveTime, Utc};
use std::time::Duration;
use config::Config;

const DAY: i64 = 24 * 3600;

#[async_recursion]
async fn auto_change_rigid(conf: Config, dawn: NaiveTime, dusk: NaiveTime) -> Result<()> {
    let now: NaiveTime = Utc::now().time();
    if  now < dawn {
        conf.set_dark_mode()?;
        task::sleep((dawn - now).to_std()?).await;
    } else if now > dusk {
        conf.set_dark_mode()?;
        task::sleep(Duration::from_secs(((dawn - now).num_seconds() + DAY) as u64)).await;
    } else {
        conf.set_light_mode()?;
        task::sleep((dusk - now).to_std()?).await;
    }
    auto_change_rigid(conf, dawn, dusk).await?;
    Ok(())
}

#[async_recursion]
async fn auto_change_solar(conf: Config, lattitude: f64, longitude: f64) -> Result<()> {
    let utc: DateTime<Utc> = Utc::now();
    let (sunrise, sunset) = sunrise::sunrise_sunset(lattitude, longitude, utc.year(), utc.month(), utc.day());
    println!("Sunrise: {}", NaiveDateTime::from_timestamp(sunrise, 0).time().format("%H:%M"));
    println!("Sunset: {}", NaiveDateTime::from_timestamp(sunset, 0).time().format("%H:%M"));
    println!("Now: {}", utc.time().format("%H:%M"));

    let now = utc.timestamp();
    if now < sunrise {
        println!("{}", sunrise-now);
        conf.set_dark_mode()?;
        task::sleep(Duration::from_secs((sunrise-now) as u64)).await;
    } else if now > sunset {
        let tomorrow_morning = sunrise::sunrise_sunset(lattitude, longitude, utc.year(), utc.month(), utc.day() + 1).0;
        println!("Tomorrow Morning: {}", NaiveDateTime::from_timestamp(tomorrow_morning, 0).time().format("%H:%M"));
        conf.set_dark_mode()?;
        task::sleep(Duration::from_secs(
            (tomorrow_morning-now) as u64
        ))
        .await;
    } else {
        conf.set_light_mode()?;
        task::sleep(Duration::from_secs(
            (sunset - now) as u64
        )).await;
    }

    auto_change_solar(conf,lattitude,longitude).await?;
    Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
    let conf = Config::load();
    match conf.timechange.clone() {
        config::TimeChange::Rigid(morning, night) => {
            let dawn = NaiveTime::parse_from_str(&morning, "%H:%M:%S").unwrap_or(NaiveTime::from_hms(7, 0, 0));
            let dusk = NaiveTime::parse_from_str(&night, "%H:%M:%S").unwrap_or(NaiveTime::from_hms(19, 0, 0));
            auto_change_rigid(conf,dawn,dusk).await?;
        },
        config::TimeChange::Solar(lattitude, longitude) => {
            auto_change_solar(conf, lattitude, longitude).await?;
        }
    }
    Ok(())
}
