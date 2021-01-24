mod config;

use anyhow::Result;
use async_recursion::async_recursion;
use async_std::task;
use chrono::{prelude::*, NaiveTime, Utc};
use config::Config;
use std::{fs::File, io::prelude::*, time::Duration};

const DAY: i64 = 24 * 3600;

#[async_recursion]
async fn auto_change_rigid(conf: Config, dawn: NaiveTime, dusk: NaiveTime) -> Result<()> {
    let now: NaiveTime = Utc::now().time();
    write_tmp(
        format!("{}\n", dawn.format("%H:%M:%S")),
        format!("{}\n", dusk.format("%H:%M:%S")),
    )?;
    if now < dawn {
        conf.set_dark_mode()?;
        task::sleep((dawn - now).to_std()?).await;
    } else if now > dusk {
        conf.set_dark_mode()?;
        task::sleep(Duration::from_secs(
            ((dawn - now).num_seconds() + DAY) as u64,
        ))
        .await;
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
    let (sunrise, sunset) =
        sunrise::sunrise_sunset(lattitude, longitude, utc.year(), utc.month(), utc.day());
    let sunrise = NaiveDateTime::from_timestamp(sunrise, 0);
    let sunset = NaiveDateTime::from_timestamp(sunset, 0);
    let now = utc.naive_local();
    write_tmp(
        format!("{}\n", sunrise.time().format("%H:%M:%S")),
        format!("{}\n", sunset.time().format("%H:%M:%S")),
    )?;
    if now < sunrise {
        dbg!("{}", sunrise - now);
        conf.set_dark_mode()?;
        task::sleep((sunrise - now).to_std()?).await;
    } else if now > sunset {
        let tomorrow_morning = NaiveDateTime::from_timestamp(
            sunrise::sunrise_sunset(lattitude, longitude, utc.year(), utc.month(), utc.day() + 1).0,
            0,
        );
        dbg!("{}", tomorrow_morning - now);
        conf.set_dark_mode()?;
        task::sleep((tomorrow_morning - now).to_std()?).await;
    } else {
        dbg!("{}", sunset - now);
        conf.set_light_mode()?;
        task::sleep((sunset - now).to_std()?).await;
    }

    auto_change_solar(conf, lattitude, longitude).await?;
    Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
    let conf = Config::load()?;
    match conf.timechange.clone() {
        config::TimeChange::Rigid(morning, night) => {
            let dawn = NaiveTime::parse_from_str(&morning, "%H:%M:%S")
                .unwrap_or_else(|_| NaiveTime::from_hms(7, 0, 0));
            let dusk = NaiveTime::parse_from_str(&night, "%H:%M:%S")
                .unwrap_or_else(|_| NaiveTime::from_hms(19, 0, 0));
            auto_change_rigid(conf, dawn, dusk).await?;
        }
        config::TimeChange::Solar(lattitude, longitude) => {
            auto_change_solar(conf, lattitude, longitude).await?;
        }
    }
    Ok(())
}

fn write_tmp(dawn: String, dusk: String) -> Result<()> {
    let dir = Config::get_tmp();
    let dawn_file = &dir.join("dawn");
    let mut dawn_file = File::create(dawn_file)?;
    let dusk_file = &dir.join("dusk");
    let mut dusk_file = File::create(dusk_file)?;
    dbg!(format!("Dawn: {}", &dawn));
    dbg!(format!("Dawn: {}", &dusk));
    dawn_file.write_all(dawn.as_bytes())?;
    dusk_file.write_all(dusk.as_bytes())?;
    Ok(())
}
