# mitemp-prometheus

Expose Xiaomi MI Temperature and Humidity Sensor to prometheus

## Usage

Run the binary with the following environment variables

```dotenv
ADAPTER=00:1A:7D:DA:71:01
PORT=3030
NAMES="58:2d:34:39:1a:01=Sensor 1,58:2d:34:39:1a:02=Sensor 2"
```

The prometheus metrics will be available at `localhost:3030/metrics`

```
sensor_battery{name="Sensor 1", mac="58:2d:34:39:1a:01"} 100
sensor_temperature{name="Sensor 1", mac="58:2d:34:39:1a:01"} 15.8
sensor_humidity{name="Sensor 1", mac="58:2d:34:39:1a:01"} 59.2
sensor_battery{name="Sensor 2", mac="58:2d:34:39:1a:02"} 100
sensor_temperature{name="Sensor 2", mac="58:2d:34:39:1a:02"} 16
sensor_humidity{name="Sensor 2", mac="58:2d:34:39:1a:02"} 55.9
sensor_battery{mac="58:2d:34:39:1a:03"} 100
sensor_temperature{mac="58:2d:34:39:1a:03"} 16.1
sensor_humidity{mac="58:2d:34:39:1a:03"} 55.3
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.