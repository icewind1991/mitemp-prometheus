mod config;

use btleplug::api::{Central, Manager as _};
use btleplug::platform::Manager;
use tracing::info;
use main_error::MainError;
use mitemp::{listen, BDAddr, Sensor};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use tokio::{pin, spawn};
use tokio_stream::StreamExt;
use warp::Filter;
use clap::Parser;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use crate::config::{Config, ListenConfig};

type Cache = Arc<Mutex<HashMap<BDAddr, Sensor>>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file to use, if omitted the config will be loaded from environment variables
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    tracing_subscriber::fmt::init();
    let cache: Cache = Arc::default();

    let args = Args::parse();
    let config = match args.config {
        Some(path) => Config::from_file(path)?,
        _ => Config::from_env()?,
    };
    info!("{} devices configured", config.names.len());

    let manager = Manager::new().await?;
    for adapter in manager.adapters().await? {
        let rx_cache = cache.clone();
        spawn(async move {
            if let Ok(info) = adapter.adapter_info().await {
                info!("Listening on {}", info);
            }
            let stream = match listen(&adapter).await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to listen to adapter: {:#}", e);
                    return;
                }
            };
            pin!(stream);

            while let Some(sensor) = stream.next().await {
                info!("Got update for {}: {:?}", sensor.mac, sensor.data);
                rx_cache.lock().unwrap().insert(sensor.mac, sensor);
            }
        });
    }

    let names = config.names;
    let metrics = warp::path!("metrics").map(move || {
        let mut result = String::new();

        for sensor in cache.lock().unwrap().values() {
            format(&mut result, sensor, &names).unwrap();
        }

        result
    });

    match config.listen {
        ListenConfig::Ip { address, port } => {
            warp::serve(metrics).run((address, port)).await;
        }
        ListenConfig::Unix { socket: path } => {
            let listener = UnixListener::bind(path).unwrap();
            let incoming = UnixListenerStream::new(listener);
            warp::serve(metrics).run_incoming(incoming).await;
        }
    }

    Ok(())
}

fn format<W: Write>(
    mut writer: W,
    sensor: &Sensor,
    names: &BTreeMap<BDAddr, String>,
) -> std::fmt::Result {
    if sensor.data.temperature == 0.0 || sensor.data.humidity == 0.0 {
        return Ok(());
    }
    let name = names.get(&sensor.mac);
    if sensor.data.battery > 0 {
        if let Some(name) = name {
            writeln!(
                writer,
                "sensor_battery{{name=\"{}\", mac=\"{}\"}} {}",
                name, sensor.mac, sensor.data.battery
            )?;
        } else {
            info!("Skipping unnamed sensor {}", sensor.mac);
        }
    }
    if let Some(name) = name {
        writeln!(
            writer,
            "sensor_temperature{{name=\"{}\", mac=\"{}\"}} {}",
            name, sensor.mac, sensor.data.temperature
        )?;
        writeln!(
            writer,
            "sensor_humidity{{name=\"{}\", mac=\"{}\"}} {}",
            name, sensor.mac, sensor.data.humidity
        )?;
    } else {
        info!("Skipping unnamed sensor {}", sensor.mac);
    }

    Ok(())
}
