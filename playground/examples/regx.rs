use anyhow;
use chrono::{DateTime, Utc};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let log_str = "[2024-08-01T06:40:12.921Z] [info]  (AssetCache)    App.js update download complete { version: '23.13.0.340' }";
    let log = parser(log_str)?;
    println!("{:?}", log);
    
    Ok(())
}

// parse
// [2024-08-01T06:40:12.921Z] [info]  (AssetCache)    App.js update download complete { version: '23.13.0.340' }
// to Log struct
#[derive(Debug)]
pub struct Log {
    timestamp: DateTime<Utc>,
    level: String,
    module: String,
    message: String,
}

fn parser(str: &str) -> anyhow::Result<Log> {
    let reg = Regex::new(
        r"^\[(?<datetime>[^\]]+)\]\s+\[(?<log_type>\w+)\]\s+\((?<info_title>\w+)\)\s+(?<info_detail>[\s+\S+]+)$",
    )?;

    let Some(captures) = reg.captures(str) else {
        return Err(anyhow::anyhow!("Capture not found"));
    };

    let datetime = DateTime::parse_from_rfc3339(&captures["datetime"])?.into();
    let level = captures["log_type"].to_string();
    let module = captures["info_title"].to_string();
    let message = captures["info_detail"].to_string();

    Ok(Log {
        timestamp: datetime,
        level,
        module,
        message,
    })
}
