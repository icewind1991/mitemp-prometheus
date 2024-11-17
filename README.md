# mitemp-prometheus

Expose Xiaomi MI Temperature and Humidity Sensor to prometheus

## Usage

Configuration can be done by either a config file or environment variables.

### Usage with a config file

Create a `config.toml` like

```toml
[listen]
port = 3030

[names]
"58:2D:34:39:1A:01" = "Sensor 1"
"58:2D:34:39:1A:02" = "Sensor 2"
```

And tun the binary like

```
mitemp-prometheus config.toml
```

### Usage with environment variables

Run the binary with the following environment variables

```dotenv
PORT=3030
NAMES="58:2d:34:39:1a:01=Sensor 1,58:2d:34:39:1a:02=Sensor 2"
```

### Querying metrics

The prometheus metrics will be available at `localhost:3030/metrics`

```
sensor_battery{name="Sensor 1", mac="58:2d:34:39:1a:01"} 100
sensor_temperature{name="Sensor 1", mac="58:2d:34:39:1a:01"} 15.8
sensor_humidity{name="Sensor 1", mac="58:2d:34:39:1a:01"} 59.2
sensor_battery{name="Sensor 2", mac="58:2d:34:39:1a:02"} 100
sensor_temperature{name="Sensor 2", mac="58:2d:34:39:1a:02"} 16
sensor_humidity{name="Sensor 2", mac="58:2d:34:39:1a:02"} 55.9
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
