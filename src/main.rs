use anyhow::Result;
use async_recursion::async_recursion;
use async_std::task;
use chrono::{NaiveTime, Timelike, Utc};

use std::time::Duration;

mod config;

pub use config::Config;

const DAWN: u64 = 7 * 3600;
const DUSK: u64 = 19 * 3600;
const DAY: u64 = 24 * 3600;

#[async_std::main]
async fn main() -> Result<()> {
    let time: u64 = time_as_secs();
    let conf = Config::get()?;
    auto_change(time, conf).await?;
    Ok(())
}

fn time_as_secs() -> u64 {
    let now: NaiveTime = Utc::now().time();
    ((now.hour() * 3600) + (now.minute() * 60) + now.second()) as u64
}

#[async_recursion]
async fn auto_change(time: u64, conf: Config) -> Result<()> {
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
    auto_change(time, conf).await?;
    Ok(())
}
