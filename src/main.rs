use main_error::MainError;
use mitemp::{adapter_by_mac, listen, BDAddr, Sensor};
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use warp::Filter;

type Cache = Arc<Mutex<HashMap<BDAddr, Sensor>>>;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let cache: Cache = Arc::default();

    let mut env: HashMap<String, String> = dotenv::vars().collect();
    let adapter = BDAddr::from_str(&env.remove("ADAPTER").ok_or("No ADDR set")?)
        .map_err(|_| "Invalid adapter address")?;
    let port = env
        .get("PORT")
        .and_then(|s| u16::from_str(s).ok())
        .unwrap_or(80);
    let names = env.remove("NAMES").unwrap_or_default();
    let names = names
        .split(',')
        .map(|pair| {
            let mut parts = pair.split('=');
            if let (Some(Ok(mac)), Some(name)) = (parts.next().map(BDAddr::from_str), parts.next())
            {
                Ok((mac, name.to_string()))
            } else {
                Err(MainError::from("Invalid NAMES"))
            }
        })
        .collect::<Result<HashMap<BDAddr, String>, MainError>>()?;

    let adapter = adapter_by_mac(adapter).map_err(|_| "Adapter not found")?;

    let iter = listen(adapter).map_err(|e| format!("Failed to start btle listen: {}", e))?;

    let rx_cache = cache.clone();
    spawn(move || {
        for sensor in iter {
            rx_cache.lock().unwrap().insert(sensor.mac, sensor);
        }
    });

    let metrics = warp::path!("metrics").map(move || {
        let mut result = String::new();

        for sensor in cache.lock().unwrap().values() {
            format(&mut result, sensor, &names).unwrap();
        }

        result
    });

    warp::serve(metrics).run(([0, 0, 0, 0], port)).await;

    Ok(())
}

fn format<W: Write>(
    mut writer: W,
    sensor: &Sensor,
    names: &HashMap<BDAddr, String>,
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
            writeln!(
                writer,
                "sensor_battery{{mac=\"{}\"}} {}",
                sensor.mac, sensor.data.battery
            )?;
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
        writeln!(
            writer,
            "sensor_temperature{{mac=\"{}\"}} {}",
            sensor.mac, sensor.data.temperature
        )?;
        writeln!(
            writer,
            "sensor_humidity{{mac=\"{}\"}} {}",
            sensor.mac, sensor.data.humidity
        )?;
    }

    Ok(())
}
