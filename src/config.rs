use btleplug::api::BDAddr;
use main_error::MainError;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs::read_to_string;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listen: ListenConfig,
    pub names: BTreeMap<BDAddr, String>,
    #[allow(dead_code)]
    pub mqtt: Option<MqttConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListenConfig {
    Ip {
        #[serde(default = "default_address")]
        address: IpAddr,
        port: u16,
    },
    Unix {
        socket: String,
    },
}

fn default_address() -> IpAddr {
    Ipv4Addr::UNSPECIFIED.into()
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    #[serde(rename = "hostname")]
    host: String,
    #[serde(default = "default_mqtt_port")]
    port: u16,
    #[serde(flatten)]
    credentials: Option<Credentials>,
}

fn default_mqtt_port() -> u16 {
    1883
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Credentials {
    Raw {
        username: String,
        password: String,
    },
    File {
        username: String,
        password_file: String,
    },
}

#[allow(dead_code)]
impl Credentials {
    pub fn username(&self) -> String {
        match self {
            Credentials::Raw { username, .. } => username.clone(),
            Credentials::File { username, .. } => username.clone(),
        }
    }
    pub fn password(&self) -> String {
        match self {
            Credentials::Raw { password, .. } => password.clone(),
            Credentials::File { password_file, .. } => secretfile::load(password_file).unwrap(),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Config, MainError> {
        let mut env: HashMap<String, String> = dotenvy::vars().collect();
        let port = env
            .get("PORT")
            .and_then(|s| u16::from_str(s).ok())
            .unwrap_or(80);

        let names = env.remove("NAMES").unwrap_or_default();
        let names = names
            .split(',')
            .map(|pair| {
                let mut parts = pair.split('=');
                if let (Some(Ok(mac)), Some(name)) =
                    (parts.next().map(BDAddr::from_str), parts.next())
                {
                    Ok((mac, name.to_string()))
                } else {
                    Err(MainError::from("Invalid NAMES"))
                }
            })
            .collect::<Result<BTreeMap<BDAddr, String>, MainError>>()?;

        Ok(Config {
            listen: ListenConfig::Ip {
                port,
                address: default_address(),
            },
            names,
            mqtt: None,
        })
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, MainError> {
        let raw = read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}
