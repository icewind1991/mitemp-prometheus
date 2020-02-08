use main_error::MainError;
use mitemp::{adapter_by_mac, BDAddr, Sensor};
use std::collections::HashMap;
use std::str::FromStr;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let mut env: HashMap<String, String> = dotenv::vars().collect();
    let adapter = BDAddr::from_str(&env.remove("ADAPTER").ok_or("No ADDR set")?)
        .map_err(|_| "Invalid adapter address")?;
    let device = BDAddr::from_str(&env.remove("DEVICE").ok_or("No KEYFILE set")?)
        .map_err(|_| "Invalid device address")?;
    let port = env
        .get("PORT")
        .and_then(|s| u16::from_str(s).ok())
        .unwrap_or(80);

    let adapter = adapter_by_mac(adapter).map_err(|_| "Adapter not found")?;

    let sensor = Sensor::new(adapter, device).start();

    let metrics = warp::path!("metrics").map(move || {
        let data = sensor.get_data();
        if data.battery > 0 {
            format!(
                "sensor_temperature {}\nsensor_humidity {}\nsensor_battery {}\n",
                data.temperature, data.humidity, data.battery
            )
        } else {
            format!(
                "sensor_temperature {}\nsensor_humidity {}\n",
                data.temperature, data.humidity
            )
        }
    });

    warp::serve(metrics).run(([0, 0, 0, 0], port)).await;

    Ok(())
}