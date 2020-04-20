#[derive(Serialize, Deserialize, Debug)]
pub struct WeFoConfig {
    pub system_config: SystemConfig,
    pub city_id: u32,
    pub openweathermap_api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemConfig {
    pub log_level: String,
}

impl ::std::default::Default for WeFoConfig {
    fn default() -> Self {
        WeFoConfig {
            system_config: SystemConfig {
                log_level: "info".into(),
            },
            city_id: 0,
            openweathermap_api_key: "".into(),
        }
    }
}

pub fn load_config(path: Option<&str>) -> Result<WeFoConfig, confy::ConfyError> {
    let cfg: WeFoConfig;
    if path.is_some() {
        cfg = confy::load(path.unwrap())?;
    } else {
        cfg = confy::load("wefo")?;
    }
    Ok(cfg)
}
